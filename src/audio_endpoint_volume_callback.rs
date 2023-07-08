use std::convert::TryInto;
use std::slice;

use windows::core::implement;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolumeCallback;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolumeCallback_Impl;
use windows::Win32::Media::Audio::AUDIO_VOLUME_NOTIFICATION_DATA;
// use crate::bindings::*;

/// See also: [`IAudioEndpointVolumeCallback`](https://docs.microsoft.com/en-us/windows/win32/api/endpointvolume/nn-endpointvolume-iaudioendpointvolumecallback)
pub trait AudioEndpointVolumeCallback: 'static {
    fn on_notify(&self, data: &NotificationData) -> Result<()> {
        let _ = data;
        Ok(())
    }
}

#[non_exhaustive]
pub struct NotificationData<'a> {
    pub event_context: GUID,
    pub muted: bool,
    pub master_volume: f32,
    pub channel_volumes: &'a [f32],
}

impl<'a> NotificationData<'a> {
    pub(crate) unsafe fn from_raw(raw: &'a AUDIO_VOLUME_NOTIFICATION_DATA) -> Self {
        Self {
            event_context: raw.guidEventContext,
            muted: raw.bMuted.into(),
            master_volume: raw.fMasterVolume,
            channel_volumes: unsafe {
                slice::from_raw_parts(
                    raw.afChannelVolumes.as_ptr(),
                    raw.nChannels.try_into().unwrap(),
                )
            },
        }
    }
}

#[implement(IAudioEndpointVolumeCallback)]
pub(crate) struct AudioEndpointVolumeCallbackWrapper {
    inner: Box<dyn AudioEndpointVolumeCallback>,
}

impl AudioEndpointVolumeCallbackWrapper {
    pub(crate) fn new<T>(inner: T) -> Self
    where
        T: AudioEndpointVolumeCallback,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

// impl IAudioEndpointVolumeCallback_Impl for AudioEndpointVolumeCallbackWrapper {
//     fn OnNotify(
//         &self,
//         pnotify: *mut AUDIO_VOLUME_NOTIFICATION_DATA,
//     ) -> ::windows::core::Result<()> {
//         todo!()
//     }
// }

// Impl IMMNotificationClient
#[allow(non_snake_case)]
impl IAudioEndpointVolumeCallback_Impl for AudioEndpointVolumeCallbackWrapper {
    fn OnNotify(&self, data: *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> Result<()> {
        self.inner
            .on_notify(unsafe { &NotificationData::from_raw(&*data) })
    }
}
