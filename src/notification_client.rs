use crate::bits::{DataFlow, DeviceRole, DeviceState};
use crate::string::WinStr;
use crate::PropertyKey;

use windows::core::{implement, PCWSTR};
use windows::Win32::Media::Audio::{EDataFlow, ERole};
use windows::Win32::Media::Audio::{IMMNotificationClient, IMMNotificationClient_Impl};
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;

/// See also: [`IMMNotificationClient`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nn-mmdeviceapi-immnotificationclient)
pub trait NotificationClient: 'static {
    /// See also: [`IMMNotificationClient::OnDefaultDeviceChanged`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immnotificationclient-ondefaultdevicechanged)
    fn on_default_device_changed(
        &self,
        data_flow: DataFlow,
        role: DeviceRole,
        device_id: &WinStr,
    ) -> windows::core::Result<()> {
        let _ = (data_flow, role, device_id);
        Ok(())
    }

    /// See also: [`IMMNotificationClient::OnDeviceAdded`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immnotificationclient-ondeviceadded)
    fn on_device_added(&self, device_id: &WinStr) -> windows::core::Result<()> {
        let _ = device_id;
        Ok(())
    }

    /// See also: [`IMMNotificationClient::OnDeviceRemoved`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immnotificationclient-ondeviceremoved)
    fn on_device_removed(&self, device_id: &WinStr) -> windows::core::Result<()> {
        let _ = device_id;
        Ok(())
    }

    /// See also: [`IMMNotificationClient::OnDeviceStateChanged`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immnotificationclient-ondevicestatechanged)
    fn on_device_state_changed(
        &self,
        device_id: &WinStr,
        state: DeviceState,
    ) -> windows::core::Result<()> {
        let _ = (device_id, state);
        Ok(())
    }

    /// See also: [`IMMNotificationClient::OnPropertyValueChanged`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immnotificationclient-onpropertyvaluechanged)
    fn on_property_value_changed(
        &self,
        device_id: &WinStr,
        property_key: PropertyKey,
    ) -> windows::core::Result<()> {
        let _ = (device_id, property_key);
        Ok(())
    }
}

#[implement(IMMNotificationClient)]
pub(crate) struct NotificationClientWrapper {
    inner: Box<dyn NotificationClient>,
}

impl NotificationClientWrapper {
    pub(crate) fn new<T>(inner: T) -> Self
    where
        T: NotificationClient,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

// Impl IMMNotificationClient
#[allow(non_snake_case)]
impl IMMNotificationClient_Impl for NotificationClientWrapper {
    fn OnDefaultDeviceChanged(
        &self,
        flow: EDataFlow,
        role: ERole,
        device_id: &PCWSTR,
    ) -> windows::core::Result<()> {
        self.inner.on_default_device_changed(
            DataFlow::from_raw(flow),
            DeviceRole::from_raw(role),
            unsafe { WinStr::from_pcwstr(device_id) },
        )
    }

    fn OnDeviceAdded(&self, device_id: &PCWSTR) -> windows::core::Result<()> {
        self.inner
            .on_device_added(unsafe { WinStr::from_pcwstr(&device_id) })
    }

    fn OnDeviceRemoved(&self, device_id: &PCWSTR) -> windows::core::Result<()> {
        self.inner
            .on_device_removed(unsafe { WinStr::from_pcwstr(device_id) })
    }

    fn OnDeviceStateChanged(
        &self,
        device_id: &PCWSTR,
        new_state: u32,
    ) -> windows::core::Result<()> {
        self.inner.on_device_state_changed(
            unsafe { WinStr::from_pcwstr(&device_id) },
            DeviceState::from_raw(new_state),
        )
    }

    fn OnPropertyValueChanged(
        &self,
        device_id: &PCWSTR,
        key: &PROPERTYKEY,
    ) -> windows::core::Result<()> {
        self.inner.on_property_value_changed(
            unsafe { WinStr::from_pcwstr(&device_id) },
            PropertyKey::from_raw(key.to_owned()),
        )
    }
}
