//! Generic WinRT RFCOMM service inspection, not a Bose protocol implementation.
//!
//! The host supplies an explicitly selected BluetoothDevice that it obtained in
//! the appropriate UI/consent context. This module never creates that object,
//! requests access, pairs, opens a socket, or changes a connection or setting.
use crate::runtime::{BoundedOperation, ReadOnlyOperation};
use crate::{CacheMode, DiscoveredService, DiscoveryOutcome, DiscoveryRequest, NoWriteDiscovery};
use speakerlink_core::model::DeviceRef;
use speakerlink_core::{ErrorCode, Result};
use std::task::Poll;
use std::time::Instant;
use windows::core::{Error, HSTRING};
use windows::Devices::Bluetooth::Rfcomm::{RfcommDeviceService, RfcommDeviceServicesResult};
use windows::Devices::Bluetooth::{BluetoothCacheMode, BluetoothDevice, BluetoothError};
use windows::Devices::Enumeration::DeviceAccessStatus;
use windows_future::{AsyncStatus, IAsyncOperation};

pub struct WindowsDiscovery {
    device: BluetoothDevice,
}

impl WindowsDiscovery {
    /// Accepts a caller-selected, already accessible device. No discovery or
    /// consent prompt is started here. The host retains responsibility for its
    /// UI thread, COM initialization, packaging and explicit user selection.
    pub fn from_authorized_device(device: BluetoothDevice) -> Result<Self> {
        require_access(&device)?;
        Ok(Self { device })
    }
}

impl NoWriteDiscovery for WindowsDiscovery {
    async fn discover(&mut self, request: DiscoveryRequest) -> Result<DiscoveryOutcome> {
        let deadline = Instant::now()
            .checked_add(request.timeout())
            .ok_or(ErrorCode::InvalidArgument)?;
        // Recheck on every query so a prior grant cannot override revocation.
        require_access(&self.device)?;
        let device_ref = local_reference(self.device.DeviceId().map_err(normalize_error)?)?;
        let mode = match request.cache_mode {
            CacheMode::Cached => BluetoothCacheMode::Cached,
            CacheMode::Uncached => BluetoothCacheMode::Uncached,
        };
        let operation = self
            .device
            .GetRfcommServicesWithCacheModeAsync(mode)
            .map_err(normalize_error)?;
        BoundedOperation::new(
            ServiceOperation {
                operation,
                request,
                device_ref,
            },
            deadline,
        )
        .await
    }
}

fn require_access(device: &BluetoothDevice) -> Result<()> {
    let status = device
        .DeviceAccessInformation()
        .and_then(|information| information.CurrentStatus())
        .map_err(normalize_error)?;
    if status == DeviceAccessStatus::Allowed {
        Ok(())
    } else {
        Err(ErrorCode::AccessDenied)
    }
}

fn local_reference(value: HSTRING) -> Result<DeviceRef> {
    // Bound conversion before allocating a Rust string. DeviceRef then checks
    // the UTF-8 byte limit and control characters. Values never enter errors.
    if value.len() > 4096 {
        return Err(ErrorCode::ProtocolError);
    }
    DeviceRef::new(value.to_string()).map_err(|_| ErrorCode::ProtocolError)
}

fn normalize_error(error: Error) -> ErrorCode {
    match error.code().0 as u32 {
        0x8007_0005 => ErrorCode::AccessDenied,
        0x8007_048f | 0x8007_0490 => ErrorCode::Disconnected,
        0x8007_05b4 | 0x8007_0102 => ErrorCode::Timeout,
        0x8000_4001 | 0x8000_4002 | 0x8004_0154 => ErrorCode::Unsupported,
        _ => ErrorCode::InternalError,
    }
}

fn require_bluetooth_success(error: BluetoothError) -> Result<()> {
    match error {
        BluetoothError::Success => Ok(()),
        BluetoothError::RadioNotAvailable
        | BluetoothError::DeviceNotConnected
        | BluetoothError::DisabledByUser => Err(ErrorCode::Disconnected),
        BluetoothError::ResourceInUse => Err(ErrorCode::Busy),
        BluetoothError::DisabledByPolicy | BluetoothError::ConsentRequired => {
            Err(ErrorCode::AccessDenied)
        }
        BluetoothError::NotSupported | BluetoothError::TransportNotSupported => {
            Err(ErrorCode::Unsupported)
        }
        _ => Err(ErrorCode::InternalError),
    }
}

struct ServiceOperation {
    operation: IAsyncOperation<RfcommDeviceServicesResult>,
    request: DiscoveryRequest,
    device_ref: DeviceRef,
}

impl ReadOnlyOperation for ServiceOperation {
    fn poll_result(&mut self) -> Poll<Result<DiscoveryOutcome>> {
        let status = match self.operation.Status() {
            Ok(status) => status,
            Err(error) => return Poll::Ready(Err(normalize_error(error))),
        };
        match status {
            AsyncStatus::Started => Poll::Pending,
            AsyncStatus::Canceled => Poll::Ready(Ok(DiscoveryOutcome::Cancelled)),
            AsyncStatus::Completed => Poll::Ready(
                self.operation
                    .GetResults()
                    .map_err(normalize_error)
                    .and_then(|result| collect_services(result, self.request, &self.device_ref)),
            ),
            _ => Poll::Ready(Err(self
                .operation
                .ErrorCode()
                .map(|code| normalize_error(Error::from_hresult(code)))
                .unwrap_or(ErrorCode::InternalError))),
        }
    }

    fn cancel(&mut self) {
        // WinRT cancellation is cooperative. Drop releases our reference even
        // when the OS has not yet acknowledged cancellation.
        let _ = self.operation.Cancel();
    }

    fn close(&mut self) {
        // Close is not legal while an operation is still Started. Do not wait
        // indefinitely for an OS cancellation acknowledgment.
        if matches!(self.operation.Status(), Ok(status) if status != AsyncStatus::Started) {
            let _ = self.operation.Close();
        }
    }
}

fn collect_services(
    result: RfcommDeviceServicesResult,
    request: DiscoveryRequest,
    device_ref: &DeviceRef,
) -> Result<DiscoveryOutcome> {
    require_bluetooth_success(result.Error().map_err(normalize_error)?)?;
    let services = result.Services().map_err(normalize_error)?;
    let count = services.Size().map_err(normalize_error)?;
    if count as usize > request.max_services() {
        return Err(ErrorCode::ProtocolError);
    }
    let mut output = Vec::with_capacity(count as usize);
    for index in 0..count {
        let service = services.GetAt(index).map_err(normalize_error)?;
        let item = read_service(&service, request.cache_mode, device_ref);
        let closed = service.Close().map_err(normalize_error);
        output.push(item?);
        closed?;
    }
    Ok(DiscoveryOutcome::Services(output))
}

fn read_service(
    service: &RfcommDeviceService,
    cache_mode: CacheMode,
    device_ref: &DeviceRef,
) -> Result<DiscoveredService> {
    let uuid = service
        .ServiceId()
        .and_then(|id| id.Uuid())
        .map_err(normalize_error)?;
    let host = service
        .ConnectionHostName()
        .and_then(|name| name.RawName())
        .map_err(normalize_error)?;
    let service_name = service.ConnectionServiceName().map_err(normalize_error)?;
    Ok(DiscoveredService {
        device_ref: device_ref.clone(),
        service_uuid: uuid.to_u128().to_be_bytes(),
        connection_host: local_reference(host)?,
        connection_service_name: local_reference(service_name)?,
        cache_mode,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows::core::HRESULT;

    #[test]
    fn hresult_mapping_returns_only_normalized_codes() {
        for (raw, expected) in [
            (0x8007_0005_u32, ErrorCode::AccessDenied),
            (0x8007_048f, ErrorCode::Disconnected),
            (0x8007_0490, ErrorCode::Disconnected),
            (0x8007_05b4, ErrorCode::Timeout),
            (0x8007_0102, ErrorCode::Timeout),
            (0x8000_4001, ErrorCode::Unsupported),
            (0x8000_4002, ErrorCode::Unsupported),
            (0x8004_0154, ErrorCode::Unsupported),
            (0x8000_ffff, ErrorCode::InternalError),
        ] {
            assert_eq!(
                normalize_error(Error::from_hresult(HRESULT(raw as i32))),
                expected
            );
        }
    }

    #[test]
    fn bluetooth_failures_do_not_trigger_permission_or_transport_fallbacks() {
        assert_eq!(require_bluetooth_success(BluetoothError::Success), Ok(()));
        for (error, expected) in [
            (BluetoothError::ConsentRequired, ErrorCode::AccessDenied),
            (BluetoothError::DisabledByPolicy, ErrorCode::AccessDenied),
            (BluetoothError::RadioNotAvailable, ErrorCode::Disconnected),
            (BluetoothError::DeviceNotConnected, ErrorCode::Disconnected),
            (BluetoothError::DisabledByUser, ErrorCode::Disconnected),
            (BluetoothError::ResourceInUse, ErrorCode::Busy),
            (BluetoothError::NotSupported, ErrorCode::Unsupported),
            (
                BluetoothError::TransportNotSupported,
                ErrorCode::Unsupported,
            ),
            (BluetoothError(i32::MAX), ErrorCode::InternalError),
        ] {
            assert_eq!(require_bluetooth_success(error), Err(expected));
        }
    }

    #[test]
    fn local_strings_are_bounded_and_redacted_without_hardware() {
        let reference = local_reference(HSTRING::from("synthetic:私人裝置")).unwrap();
        assert!(!format!("{reference:?}").contains("私人"));
        for invalid in [String::new(), "x".repeat(4097), "synthetic\nvalue".into()] {
            assert_eq!(
                local_reference(HSTRING::from(invalid)),
                Err(ErrorCode::ProtocolError)
            );
        }
    }

    #[test]
    fn synthetic_winrt_error_operation_uses_the_real_projection_without_a_device() {
        let mut operation = ServiceOperation {
            operation: IAsyncOperation::ready(Err(Error::from_hresult(HRESULT(
                0x8007_0005_u32 as i32,
            )))),
            request: DiscoveryRequest::new(std::time::Duration::from_secs(1), 4).unwrap(),
            device_ref: DeviceRef::new("synthetic-only").unwrap(),
        };
        assert_eq!(
            operation.poll_result(),
            Poll::Ready(Err(ErrorCode::AccessDenied))
        );
        operation.close();
    }
}
