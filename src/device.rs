use crate::{
    audio_endpoint_volume::AudioEndpointVolume,
    audio_session_manager::AudioSessionManager,
    bits::{DeviceState, StorageAccessMode},
    property_store::{PropertyKey, PropertyStore},
    string::WinString,
    AudioSessionManager2,
};
use windows::Win32::{
    Devices::{Properties::{
        DEVPKEY_DeviceInterface_FriendlyName, DEVPKEY_Device_DeviceDesc,
        DEVPKEY_Device_FriendlyName,
    }, FunctionDiscovery::{PKEY_DeviceInterface_FriendlyName, PKEY_Device_DeviceDesc, PKEY_Device_FriendlyName}},
    Media::Audio::IMMDevice,
    System::Com::{StructuredStorage::PROPVARIANT, CLSCTX_ALL, STGM},
};

/// See also: [`IMMDevice`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nn-mmdeviceapi-immdevice)
#[derive(Debug, Clone)]
pub struct Device {
    inner: IMMDevice,
}

impl Device {
    pub(crate) fn new(inner: IMMDevice) -> Self {
        Self { inner }
    }

    pub(crate) unsafe fn activate<T>(&self, params: *mut PROPVARIANT) -> windows::core::Result<T>
    where
        T: Activate,
    {
        let mut raw = None;
        unsafe {
            self.inner.Activate::<T::Raw>(CLSCTX_ALL, Some(params))?;
        }
        Ok(<T as Activate>::from_raw(raw.unwrap()))
    }

    pub fn activate_audio_endpoint_volume(&self) -> windows::core::Result<AudioEndpointVolume> {
        unsafe { self.activate(std::ptr::null_mut()) }
    }

    pub fn activate_audio_session_manager(&self) -> windows::core::Result<AudioSessionManager> {
        unsafe { self.activate(std::ptr::null_mut()) }
    }

    pub fn activate_audio_session_manager2(&self) -> windows::core::Result<AudioSessionManager2> {
        unsafe { self.activate(std::ptr::null_mut()) }
    }

    /// See also: [`IMMDevice::GetId`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdevice-getid)
    pub fn get_id(&self) -> windows::core::Result<WinString> {
        Ok(unsafe { WinString::from_com_pwstr(self.inner.GetId()?) })
    }

    /// See also: [`IMMDevice::GetState`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdevice-getstate)
    pub fn get_state(&self) -> windows::core::Result<DeviceState> {
        Ok(DeviceState::from_raw(unsafe { self.inner.GetState()? }))
    }

    /// See also: [`IMMDevice::OpenPropertyStore`](https://docs.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdevice-openpropertystore)
    pub fn open_property_store(
        &self,
        storage_access_mode: StorageAccessMode,
    ) -> windows::core::Result<PropertyStore> {
        unsafe {
            self.inner
                .OpenPropertyStore(STGM(storage_access_mode.to_raw() as _))
                .map(PropertyStore::new)
        }
    }
}

pub(crate) trait Activate {
    type Raw: windows::core::ComInterface;

    fn from_raw(raw: Self::Raw) -> Self;
}

pub const DEVICE_INTERFACE_FRIENDLY_NAME: PropertyKey =
    PropertyKey::from_raw(PKEY_DeviceInterface_FriendlyName);
pub const DEVICE_DESCRIPTION: PropertyKey = PropertyKey::from_raw(PKEY_Device_DeviceDesc);
pub const DEVICE_FRIENDLY_NAME: PropertyKey = PropertyKey::from_raw(PKEY_Device_FriendlyName);
