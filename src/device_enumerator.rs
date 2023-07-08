use windows::Win32::{System::Com::{CoCreateInstance, CLSCTX_ALL}, Media::Audio::{MMDeviceEnumerator, IMMDeviceEnumerator, IMMNotificationClient}};

use crate::{
    bits::{DataFlow, DataFlowMask, DeviceRole, DeviceStateMask},
    device::Device,
    device_collection::DeviceCollection,
    notification_client::{NotificationClient, NotificationClientWrapper},
    string::WinStr,
};

/// See also: [`IMMDeviceEnumerator`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nn-mmdeviceapi-immdeviceenumerator)
#[derive(Debug, Clone)]
pub struct DeviceEnumerator {
    inner: IMMDeviceEnumerator,
}

impl DeviceEnumerator {
    pub fn new() -> windows::core::Result<Self> {
        // Static entrypoint:
        crate::ensure_thread_init();

        let inner = unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)? };
        Ok(Self { inner })
    }

    /// See also: [`IMMDeviceEnumerator::EnumAudioEndpoints`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdeviceenumerator-enumaudioendpoints)
    pub fn enum_audio_endpoints(
        &self,
        data_flow_mask: DataFlowMask,
        state_mask: DeviceStateMask,
    ) -> windows::core::Result<DeviceCollection> {
        let inner = unsafe {
            self.inner
                .EnumAudioEndpoints(data_flow_mask.to_raw(), state_mask.bits())?
        };
        Ok(DeviceCollection::new(inner))
    }

    /// See also: [`IMMDeviceEnumerator::GetDefaultAudioEndpoint`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdeviceenumerator-getdefaultaudioendpoint)
    pub fn get_default_audio_endpoint(
        &self,
        data_flow: DataFlow,
        role: DeviceRole,
    ) -> windows::core::Result<Device> {
        unsafe {
            self.inner
                .GetDefaultAudioEndpoint(data_flow.to_raw(), role.to_raw())
                .map(Device::new)
        }
    }

    /// See also: [`IMMDeviceEnumerator::GetDevice`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdeviceenumerator-getdevice)
    pub fn get_device(&self, device_id: &WinStr) -> windows::core::Result<Device> {
        unsafe { self.inner.GetDevice(device_id.as_pcwstr()).map(Device::new) }
    }

    /// See also: [`IMMDeviceEnumerator::RegisterEndpointNotificationCallback`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdeviceenumerator-registerendpointnotificationcallback)
    pub fn register_endpoint_notification<T>(
        &self,
        notification_client: T,
    ) -> windows::core::Result<NotificationClientHandle>
    where
        T: NotificationClient,
    {
        let wrapper =
            IMMNotificationClient::from(NotificationClientWrapper::new(notification_client));
        unsafe { self.inner.RegisterEndpointNotificationCallback(&wrapper)? };

        Ok(NotificationClientHandle {
            inner: wrapper,
            parent: self.inner.clone(),
        })
    }
}

#[derive(Debug, Clone)]
#[must_use = "callback will be unregistered when the handle is dropped"]
pub struct NotificationClientHandle {
    inner: IMMNotificationClient,
    parent: IMMDeviceEnumerator,
}

impl NotificationClientHandle {
    pub fn unregister(self) {
        // Handled by Drop impl
    }
}

impl Drop for NotificationClientHandle {
    fn drop(&mut self) {
        unsafe {
            self.parent
                .UnregisterEndpointNotificationCallback(&self.inner)
                .ok()
        };
    }
}
