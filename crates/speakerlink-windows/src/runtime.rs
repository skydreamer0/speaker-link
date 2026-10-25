//! Executor-neutral waiting for one read-only operation. The timer only wakes
//! the caller; it never opens a device or calls a platform API from its thread.
use speakerlink_core::discovery::DiscoveryOutcome;
use speakerlink_core::{ErrorCode, Result};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

const POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(crate) trait ReadOnlyOperation: Unpin {
    fn poll_result(&mut self) -> Poll<Result<DiscoveryOutcome>>;
    fn cancel(&mut self);
    fn close(&mut self);
}

struct TimerState {
    stopped: bool,
    waker: Option<Arc<Waker>>,
}

struct WakeTimer(Arc<(Mutex<TimerState>, Condvar)>);

impl WakeTimer {
    fn start(deadline: Instant, waker: &Waker) -> Result<Self> {
        let shared = Arc::new((
            Mutex::new(TimerState {
                stopped: false,
                waker: Some(Arc::new(waker.clone())),
            }),
            Condvar::new(),
        ));
        let worker = shared.clone();
        std::thread::Builder::new()
            .name("speakerlink-read-only-deadline".into())
            .spawn(move || loop {
                let (mutex, condition) = &*worker;
                let state = mutex.lock().unwrap_or_else(|error| error.into_inner());
                if state.stopped {
                    return;
                }
                let remaining = deadline.saturating_duration_since(Instant::now());
                let (mut state, _) = condition
                    .wait_timeout(state, remaining.min(POLL_INTERVAL))
                    .unwrap_or_else(|error| error.into_inner());
                if state.stopped {
                    return;
                }
                let expired = Instant::now() >= deadline;
                let waker = if expired {
                    state.stopped = true;
                    state.waker.take()
                } else {
                    state.waker.clone()
                };
                drop(state);
                if let Some(waker) = waker {
                    waker.wake_by_ref();
                }
                if expired {
                    return;
                }
            })
            .map_err(|_| ErrorCode::InternalError)?;
        Ok(Self(shared))
    }

    fn register(&self, waker: &Waker) {
        let incoming = Arc::new(waker.clone());
        let (mutex, _) = &*self.0;
        let mut state = mutex.lock().unwrap_or_else(|error| error.into_inner());
        let stopped = state.stopped;
        let previous = if stopped {
            None
        } else {
            state.waker.replace(incoming.clone())
        };
        drop(state);
        drop(previous);
        if stopped {
            incoming.wake_by_ref();
        }
    }
}

impl Drop for WakeTimer {
    fn drop(&mut self) {
        let (mutex, condition) = &*self.0;
        let mut state = mutex.lock().unwrap_or_else(|error| error.into_inner());
        state.stopped = true;
        let waker = state.waker.take();
        drop(state);
        drop(waker);
        condition.notify_one();
        // Do not join here: a custom Waker may run cancellation on this thread.
    }
}

pub(crate) struct BoundedOperation<O: ReadOnlyOperation> {
    operation: Option<O>,
    deadline: Instant,
    timer: Option<WakeTimer>,
}

impl<O: ReadOnlyOperation> BoundedOperation<O> {
    pub(crate) fn new(operation: O, deadline: Instant) -> Self {
        Self {
            operation: Some(operation),
            deadline,
            timer: None,
        }
    }

    fn finish(&mut self, cancel: bool) {
        self.timer.take();
        if let Some(mut operation) = self.operation.take() {
            if cancel {
                operation.cancel();
            }
            operation.close();
        }
    }
}

impl<O: ReadOnlyOperation> Future for BoundedOperation<O> {
    type Output = Result<DiscoveryOutcome>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.operation.is_none() {
            return Poll::Ready(Err(ErrorCode::InternalError));
        }
        if Instant::now() >= this.deadline {
            this.finish(true);
            return Poll::Ready(Err(ErrorCode::Timeout));
        }
        let result = this.operation.as_mut().unwrap().poll_result();
        if Instant::now() >= this.deadline {
            this.finish(true);
            return Poll::Ready(Err(ErrorCode::Timeout));
        }
        if let Poll::Ready(result) = result {
            this.finish(result.is_err());
            return Poll::Ready(result);
        }
        if let Some(timer) = &this.timer {
            timer.register(context.waker());
        } else {
            match WakeTimer::start(this.deadline, context.waker()) {
                Ok(timer) => this.timer = Some(timer),
                Err(error) => {
                    this.finish(true);
                    return Poll::Ready(Err(error));
                }
            }
        }
        Poll::Pending
    }
}

impl<O: ReadOnlyOperation> Drop for BoundedOperation<O> {
    fn drop(&mut self) {
        self.finish(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::mpsc::{channel, Sender};
    use std::task::Wake;

    #[derive(Default)]
    struct Counts {
        polls: AtomicUsize,
        cancels: AtomicUsize,
        closes: AtomicUsize,
    }

    struct SyntheticOperation {
        outcome: Option<Result<DiscoveryOutcome>>,
        counts: Arc<Counts>,
    }

    impl ReadOnlyOperation for SyntheticOperation {
        fn poll_result(&mut self) -> Poll<Result<DiscoveryOutcome>> {
            self.counts.polls.fetch_add(1, Ordering::SeqCst);
            self.outcome.take().map_or(Poll::Pending, Poll::Ready)
        }

        fn cancel(&mut self) {
            self.counts.cancels.fetch_add(1, Ordering::SeqCst);
        }

        fn close(&mut self) {
            self.counts.closes.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn operation(outcome: Option<Result<DiscoveryOutcome>>) -> (SyntheticOperation, Arc<Counts>) {
        let counts = Arc::new(Counts::default());
        (
            SyntheticOperation {
                outcome,
                counts: counts.clone(),
            },
            counts,
        )
    }

    #[test]
    fn ready_read_closes_once_without_cancelling() {
        let expected = Ok(DiscoveryOutcome::Services(Vec::new()));
        let (operation, counts) = operation(Some(expected.clone()));
        let mut task = Box::pin(BoundedOperation::new(
            operation,
            Instant::now() + Duration::from_secs(1),
        ));
        let result = task.as_mut().poll(&mut Context::from_waker(Waker::noop()));
        assert_eq!(result, Poll::Ready(expected));
        drop(task);
        assert_eq!(counts.cancels.load(Ordering::SeqCst), 0);
        assert_eq!(counts.closes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn normalized_failures_are_preserved_with_cleanup() {
        for error in [
            ErrorCode::AccessDenied,
            ErrorCode::Disconnected,
            ErrorCode::ProtocolError,
            ErrorCode::Busy,
            ErrorCode::Unsupported,
        ] {
            let (operation, counts) = operation(Some(Err(error)));
            let mut task = Box::pin(BoundedOperation::new(
                operation,
                Instant::now() + Duration::from_secs(1),
            ));
            assert_eq!(
                task.as_mut().poll(&mut Context::from_waker(Waker::noop())),
                Poll::Ready(Err(error))
            );
            drop(task);
            assert_eq!(counts.cancels.load(Ordering::SeqCst), 1);
            assert_eq!(counts.closes.load(Ordering::SeqCst), 1);
        }
    }

    #[test]
    fn expired_deadline_does_not_poll_or_accept_a_late_success() {
        let (operation, counts) = operation(Some(Ok(DiscoveryOutcome::Services(Vec::new()))));
        let mut task = Box::pin(BoundedOperation::new(operation, Instant::now()));
        assert_eq!(
            task.as_mut().poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Err(ErrorCode::Timeout))
        );
        assert_eq!(counts.polls.load(Ordering::SeqCst), 0);
        assert_eq!(counts.cancels.load(Ordering::SeqCst), 1);
        assert_eq!(counts.closes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn dropping_pending_read_cancels_and_releases_owned_operation() {
        let (operation, counts) = operation(None);
        let mut task = Box::pin(BoundedOperation::new(
            operation,
            Instant::now() + Duration::from_secs(1),
        ));
        assert!(task
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending());
        drop(task);
        assert_eq!(counts.cancels.load(Ordering::SeqCst), 1);
        assert_eq!(counts.closes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn dropping_before_first_poll_still_releases_an_owned_operation() {
        let (operation, counts) = operation(None);
        drop(BoundedOperation::new(
            operation,
            Instant::now() + Duration::from_secs(1),
        ));
        assert_eq!(counts.polls.load(Ordering::SeqCst), 0);
        assert_eq!(counts.cancels.load(Ordering::SeqCst), 1);
        assert_eq!(counts.closes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn platform_cancellation_is_a_control_outcome() {
        let (operation, counts) = operation(Some(Ok(DiscoveryOutcome::Cancelled)));
        let mut task = Box::pin(BoundedOperation::new(
            operation,
            Instant::now() + Duration::from_secs(1),
        ));
        assert_eq!(
            task.as_mut().poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Ok(DiscoveryOutcome::Cancelled))
        );
        assert_eq!(counts.cancels.load(Ordering::SeqCst), 0);
        assert_eq!(counts.closes.load(Ordering::SeqCst), 1);
    }

    struct LockCheckingWake {
        shared: std::sync::Weak<(Mutex<TimerState>, Condvar)>,
        wakes: AtomicUsize,
        blocked_callbacks: AtomicUsize,
    }

    impl Wake for LockCheckingWake {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            if let Some(shared) = self.shared.upgrade() {
                if shared.0.try_lock().is_err() {
                    self.blocked_callbacks.fetch_add(1, Ordering::SeqCst);
                }
            }
            self.wakes.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn manual_timer(stopped: bool) -> WakeTimer {
        WakeTimer(Arc::new((
            Mutex::new(TimerState {
                stopped,
                waker: None,
            }),
            Condvar::new(),
        )))
    }

    fn checking_waker(timer: &WakeTimer) -> (Waker, Arc<LockCheckingWake>) {
        let wake = Arc::new(LockCheckingWake {
            shared: Arc::downgrade(&timer.0),
            wakes: AtomicUsize::new(0),
            blocked_callbacks: AtomicUsize::new(0),
        });
        (Waker::from(wake.clone()), wake)
    }

    #[test]
    fn stopped_timer_wakes_replacement_after_previous_waker_was_taken() {
        let timer = manual_timer(false);
        let (first, first_counts) = checking_waker(&timer);
        timer.register(&first);
        let taken = {
            let mut state = timer.0 .0.lock().unwrap();
            state.stopped = true;
            state.waker.take().unwrap()
        };
        let (replacement, replacement_counts) = checking_waker(&timer);
        timer.register(&replacement);
        taken.wake_by_ref();
        assert_eq!(first_counts.wakes.load(Ordering::SeqCst), 1);
        assert_eq!(replacement_counts.wakes.load(Ordering::SeqCst), 1);
        assert_eq!(
            replacement_counts.blocked_callbacks.load(Ordering::SeqCst),
            0
        );
    }

    #[test]
    fn pending_operation_replaces_waker_and_timeout_cleans_up_once() {
        let (operation, counts) = operation(None);
        let timer = manual_timer(false);
        let (first, first_counts) = checking_waker(&timer);
        let (replacement, replacement_counts) = checking_waker(&timer);
        let mut task = Box::pin(BoundedOperation::new(
            operation,
            Instant::now() + Duration::from_secs(60),
        ));
        task.timer = Some(timer);
        assert!(task
            .as_mut()
            .poll(&mut Context::from_waker(&first))
            .is_pending());
        assert!(task
            .as_mut()
            .poll(&mut Context::from_waker(&replacement))
            .is_pending());
        let registered = task
            .timer
            .as_ref()
            .unwrap()
            .0
             .0
            .lock()
            .unwrap()
            .waker
            .take()
            .unwrap();
        registered.wake_by_ref();
        assert_eq!(first_counts.wakes.load(Ordering::SeqCst), 0);
        assert_eq!(replacement_counts.wakes.load(Ordering::SeqCst), 1);
        assert_eq!(
            replacement_counts.blocked_callbacks.load(Ordering::SeqCst),
            0
        );
        task.deadline = Instant::now();
        assert_eq!(
            task.as_mut().poll(&mut Context::from_waker(&replacement)),
            Poll::Ready(Err(ErrorCode::Timeout))
        );
        drop(task);
        assert_eq!(counts.cancels.load(Ordering::SeqCst), 1);
        assert_eq!(counts.closes.load(Ordering::SeqCst), 1);
    }
    struct DropCheckingWake {
        shared: std::sync::Weak<(Mutex<TimerState>, Condvar)>,
        drops: Arc<AtomicUsize>,
        blocked: Arc<AtomicUsize>,
    }

    impl Wake for DropCheckingWake {
        fn wake(self: Arc<Self>) {
            if let Some(shared) = self.shared.upgrade() {
                if shared.0.try_lock().is_err() {
                    self.blocked.fetch_add(1, Ordering::SeqCst);
                }
            }
        }
    }

    impl Drop for DropCheckingWake {
        fn drop(&mut self) {
            if let Some(shared) = self.shared.upgrade() {
                if shared.0.try_lock().is_err() {
                    self.blocked.fetch_add(1, Ordering::SeqCst);
                }
            }
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn replaced_and_cancelled_wakers_drop_without_timer_lock() {
        for replace in [true, false] {
            let timer = manual_timer(false);
            let drops = Arc::new(AtomicUsize::new(0));
            let blocked = Arc::new(AtomicUsize::new(0));
            let waker = Waker::from(Arc::new(DropCheckingWake {
                shared: Arc::downgrade(&timer.0),
                drops: drops.clone(),
                blocked: blocked.clone(),
            }));
            timer.register(&waker);
            drop(waker);
            if replace {
                timer.register(Waker::noop());
            }
            drop(timer);
            assert_eq!(drops.load(Ordering::SeqCst), 1);
            assert_eq!(blocked.load(Ordering::SeqCst), 0);
        }
    }
    struct SignalWake(Sender<()>);
    impl Wake for SignalWake {
        fn wake(self: Arc<Self>) {
            let _ = self.0.send(());
        }
    }

    #[test]
    fn pending_operation_wakes_executor_and_eventually_times_out() {
        let (operation, counts) = operation(None);
        let (sender, receiver) = channel();
        let waker = Waker::from(Arc::new(SignalWake(sender)));
        let mut context = Context::from_waker(&waker);
        let mut task = Box::pin(BoundedOperation::new(
            operation,
            Instant::now() + Duration::from_millis(30),
        ));
        loop {
            match task.as_mut().poll(&mut context) {
                Poll::Ready(result) => {
                    assert_eq!(result, Err(ErrorCode::Timeout));
                    break;
                }
                Poll::Pending => receiver.recv_timeout(Duration::from_secs(2)).unwrap(),
            }
        }
        assert_eq!(counts.cancels.load(Ordering::SeqCst), 1);
        assert_eq!(counts.closes.load(Ordering::SeqCst), 1);
    }
}
