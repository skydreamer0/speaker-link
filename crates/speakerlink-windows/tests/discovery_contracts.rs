//! Synthetic fixtures only. These tests do not use a Bluetooth adapter.
use speakerlink_core::model::{Capabilities, DeviceRef, Freshness, Observation, UnknownReason};
use speakerlink_core::{ErrorCode, Result};
use speakerlink_windows::{
    AccessStatus, CacheMode, DiscoveredService, DiscoveryOutcome, DiscoveryRequest,
    NoWriteDiscovery, UnimplementedWindowsDiscovery,
};
use std::future::{ready, Future};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

// Only immediately-ready synthetic futures are accepted. This is deliberately
// not a hardware runtime or a polling loop that could hang on device I/O.
fn ready_result<F: Future>(future: F) -> F::Output {
    let mut context = Context::from_waker(Waker::noop());
    match Box::pin(future).as_mut().poll(&mut context) {
        Poll::Ready(output) => output,
        Poll::Pending => panic!("synthetic fixture unexpectedly pending"),
    }
}

fn request() -> DiscoveryRequest {
    DiscoveryRequest::new(Duration::from_secs(1), 4).unwrap()
}

fn synthetic_service(cache_mode: CacheMode) -> DiscoveredService {
    DiscoveredService {
        device_ref: DeviceRef::new("synthetic-speaker").unwrap(),
        service_uuid: [0; 16],
        connection_host: DeviceRef::new("synthetic-host").unwrap(),
        connection_service_name: DeviceRef::new("synthetic-discovered-service").unwrap(),
        cache_mode,
    }
}

#[test]
fn requests_default_to_uncached_with_explicit_nonzero_limits() {
    let request = request();
    assert_eq!(request.cache_mode, CacheMode::Uncached);
    assert_eq!(request.timeout(), Duration::from_secs(1));
    assert_eq!(request.max_services(), 4);
    assert_eq!(
        DiscoveryRequest::new(Duration::ZERO, 4),
        Err(ErrorCode::InvalidArgument)
    );
    assert_eq!(
        DiscoveryRequest::new(Duration::from_secs(1), 0),
        Err(ErrorCode::InvalidArgument)
    );
    assert!(DiscoveryRequest::new(Duration::from_secs(60), 256).is_ok());
    assert_eq!(
        DiscoveryRequest::new(Duration::from_secs(61), 4),
        Err(ErrorCode::InvalidArgument)
    );
    assert_eq!(
        DiscoveryRequest::new(Duration::from_secs(1), 257),
        Err(ErrorCode::InvalidArgument)
    );
}

#[test]
fn oversized_discovery_results_fail_instead_of_silently_truncating() {
    let services: Vec<_> = (0..5)
        .map(|_| synthetic_service(CacheMode::Uncached))
        .collect();
    assert_eq!(request().validate_result(&services[..4]), Ok(()));
    assert_eq!(
        request().validate_result(&services),
        Err(ErrorCode::ProtocolError)
    );
}

#[test]
fn only_explicitly_allowed_access_passes_without_requesting_permissions() {
    assert_eq!(AccessStatus::Allowed.require_allowed(), Ok(()));
    for status in [
        AccessStatus::DeniedByUser,
        AccessStatus::DeniedBySystem,
        AccessStatus::Unspecified,
    ] {
        assert_eq!(status.require_allowed(), Err(ErrorCode::AccessDenied));
    }
}

#[test]
fn production_sentinel_is_unsupported_on_every_os_not_a_fake_discovery() {
    let mut adapter = UnimplementedWindowsDiscovery;
    assert_eq!(
        ready_result(adapter.discover(request())),
        Err(ErrorCode::Unsupported)
    );
}

#[test]
fn both_cached_and_uncached_discovery_leave_speaker_state_unknown() {
    for cache in [CacheMode::Cached, CacheMode::Uncached] {
        let service = synthetic_service(cache);
        assert_eq!(service.cache_mode, cache);
        let snapshot = service.unobserved_snapshot();
        assert_eq!(snapshot.observed_at, None);
        assert_eq!(snapshot.freshness, Freshness::Unknown);
        assert_eq!(snapshot.capabilities, Capabilities::default());
        assert_eq!(
            snapshot.speaker_devices,
            Observation::Unknown(UnknownReason::NotObserved)
        );
    }
}

#[test]
fn service_uuid_and_friendly_looking_local_id_do_not_enable_capabilities() {
    let mut service = synthetic_service(CacheMode::Uncached);
    // Standard AVRCP UUID is only a synthetic test input, not device evidence.
    service.service_uuid = [
        0, 0, 0x11, 0x0e, 0, 0, 0x10, 0, 0x80, 0, 0, 0x80, 0x5f, 0x9b, 0x34, 0xfb,
    ];
    service.device_ref = DeviceRef::new("synthetic Bose SoundLink Flex BC60").unwrap();
    assert_eq!(
        service.unobserved_snapshot().capabilities,
        Capabilities::default()
    );
}

struct SyntheticDiscovery {
    result: Result<DiscoveryOutcome>,
    calls: usize,
}

impl NoWriteDiscovery for SyntheticDiscovery {
    fn discover(
        &mut self,
        options: DiscoveryRequest,
    ) -> impl Future<Output = Result<DiscoveryOutcome>> {
        self.calls += 1;
        let result = self.result.clone().and_then(|outcome| {
            if let DiscoveryOutcome::Services(services) = &outcome {
                options.validate_result(services)?;
            }
            Ok(outcome)
        });
        ready(result)
    }
}

#[test]
fn failure_and_cancel_outcomes_survive_the_same_no_write_contract() {
    for result in [
        Err(ErrorCode::AccessDenied),
        Err(ErrorCode::Disconnected),
        Err(ErrorCode::Timeout),
        Err(ErrorCode::Busy),
        Ok(DiscoveryOutcome::Cancelled),
    ] {
        let mut adapter = SyntheticDiscovery {
            result: result.clone(),
            calls: 0,
        };
        assert_eq!(ready_result(adapter.discover(request())), result);
        assert_eq!(adapter.calls, 1);
    }
}

#[test]
fn observed_empty_services_do_not_become_an_observed_empty_speaker_list() {
    let mut adapter = SyntheticDiscovery {
        result: Ok(DiscoveryOutcome::Services(Vec::new())),
        calls: 0,
    };
    assert_eq!(
        ready_result(adapter.discover(request())),
        Ok(DiscoveryOutcome::Services(Vec::new()))
    );
    // Service discovery's empty vector concerns services only; it supplies
    // no speaker-side paired/connected/active-audio data.
}
