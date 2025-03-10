use crate::{
    audio_session_events::{AudioSessionEvents, AudioSessionEventsWrapper},
    bits::AudioSessionState,
    string::{WinStr, WinString},
    util::as_raw_or_null,
    SimpleAudioVolume,
};
use std::ops::Deref;

use windows::core::ComInterface;
use windows::{
    core::GUID,
    Win32::Media::Audio::{IAudioSessionControl, IAudioSessionControl2, IAudioSessionEvents},
};

/// See also: [`IAudioSessionControl`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiosessioncontrol)
#[derive(Debug, Clone)]
pub struct AudioSessionControl {
    inner: IAudioSessionControl,
}

impl AudioSessionControl {
    pub(crate) fn new(inner: IAudioSessionControl) -> Self {
        Self { inner }
    }

    /// See also: [`IAudioSessionControl::GetDisplayName`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol-getdisplayname)
    pub fn get_display_name(&self) -> windows::core::Result<WinString> {
        unsafe {
            self.inner
                .GetDisplayName()
                .map(|x| WinString::from_com_pwstr(x))
        }
    }

    /// See also: [`IAudioSessionControl::GetGroupingParam`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol-getgroupingparam)
    pub fn get_grouping_param(&self) -> windows::core::Result<GUID> {
        unsafe { self.inner.GetGroupingParam() }
    }

    /// See also: [`IAudioSessionControl::GetIconPath`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol-geticonpath)
    pub fn get_icon_path(&self) -> windows::core::Result<WinString> {
        unsafe {
            self.inner
                .GetIconPath()
                .map(|x| WinString::from_com_pwstr(x))
        }
    }

    /// See also: [`IAudioSessionControl::GetState`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol-getstate)
    pub fn get_state(&self) -> windows::core::Result<AudioSessionState> {
        unsafe { self.inner.GetState().map(AudioSessionState::from_raw) }
    }

    /// See also: [`IAudioSessionControl::RegisterAudioSessionNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol-registeraudiosessionnotification)
    pub fn register_audio_session_notification<T>(
        &self,
        session_notification: T,
    ) -> windows::core::Result<AudioSessionEventsHandle>
    where
        T: AudioSessionEvents,
    {
        let session_notification =
            IAudioSessionEvents::from(AudioSessionEventsWrapper::new(session_notification));
        unsafe {
            self.inner
                .RegisterAudioSessionNotification(&session_notification)?
        };
        Ok(AudioSessionEventsHandle {
            inner: session_notification,
            parent: self.inner.clone(),
        })
    }

    /// See also: [`IAudioSessionControl::SetDisplayName`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol-setdisplayname)
    pub fn set_display_name(
        &self,
        value: &WinStr,
        event_context: Option<&GUID>,
    ) -> windows::core::Result<()> {
        unsafe {
            self.inner
                .SetDisplayName(value.as_pcwstr(), as_raw_or_null(event_context))
        }
    }

    /// See also: [`IAudioSessionControl::SetGroupingParam`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol-setgroupingparam)
    pub fn set_grouping_param(
        &self,
        value: &GUID,
        event_context: Option<&GUID>,
    ) -> windows::core::Result<()> {
        unsafe {
            self.inner
                .SetGroupingParam(value, as_raw_or_null(event_context))
        }
    }

    /// See also: [`IAudioSessionControl::SetIconPath`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol-seticonpath)
    pub fn set_icon_path(
        &self,
        value: &WinStr,
        event_context: Option<&GUID>,
    ) -> windows::core::Result<()> {
        unsafe {
            self.inner
                .SetIconPath(value.as_pcwstr(), as_raw_or_null(event_context))
        }
    }

    pub fn upgrade(&self) -> windows::core::Result<AudioSessionControl2> {
        self.inner.cast().map(AudioSessionControl2::new)
    }

    pub fn get_simple_audio_volume(&self) -> windows::core::Result<SimpleAudioVolume> {
        self.inner.cast().map(SimpleAudioVolume::new)
    }
}

/// See also: [`IAudioSessionControl2`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiosessioncontrol2)
#[derive(Debug, Clone)]
pub struct AudioSessionControl2 {
    inner: IAudioSessionControl2,
    downgrade: AudioSessionControl,
}

impl AudioSessionControl2 {
    pub(crate) fn new(inner: IAudioSessionControl2) -> Self {
        let downgrade = AudioSessionControl::new(inner.cast().unwrap());
        Self { inner, downgrade }
    }

    /// See also: [`IAudioSessionControl2::GetProcessId`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol2-getprocessid)
    pub fn get_process_id(&self) -> windows::core::Result<u32> {
        unsafe { self.inner.GetProcessId() }
    }

    /// See also: [`IAudioSessionControl2::GetSessionIdentifier`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol2-getsessionidentifier)
    pub fn get_session_identifier(&self) -> windows::core::Result<WinString> {
        unsafe {
            self.inner
                .GetSessionIdentifier()
                .map(|x| WinString::from_com_pwstr(x))
        }
    }

    /// See also: [`IAudioSessionControl2::GetSessionInstanceIdentifier`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol2-getsessioninstanceidentifier)
    pub fn get_session_instance_identifier(&self) -> windows::core::Result<WinString> {
        unsafe {
            self.inner
                .GetSessionInstanceIdentifier()
                .map(|x| WinString::from_com_pwstr(x))
        }
    }

    /// See also: [`IAudioSessionControl2::IsSystemSoundsSession`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol2-issystemsoundssession)
    pub fn is_system_sounds_session(&self) -> bool {
        todo!("cannot query IsSystemSoundsSession yet, see https://github.com/microsoft/windows-rs/issues/1065")
    }

    /// See also: [`IAudioSessionControl2::SetDuckingPreference`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessioncontrol2-setduckingpreference)
    pub fn set_ducking_preference(&self, opt_out: bool) -> windows::core::Result<()> {
        unsafe { self.inner.SetDuckingPreference(opt_out) }
    }
}

impl Deref for AudioSessionControl2 {
    type Target = AudioSessionControl;

    fn deref(&self) -> &Self::Target {
        &self.downgrade
    }
}

#[derive(Debug, Clone)]
#[must_use = "callback will be unregistered when the handle is dropped"]
pub struct AudioSessionEventsHandle {
    inner: IAudioSessionEvents,
    parent: IAudioSessionControl,
}

impl AudioSessionEventsHandle {
    pub fn unregister(self) {
        // Handled by the Drop impl
    }
}

impl Drop for AudioSessionEventsHandle {
    fn drop(&mut self) {
        unsafe {
            self.parent
                .UnregisterAudioSessionNotification(&self.inner)
                .ok()
        };
    }
}
