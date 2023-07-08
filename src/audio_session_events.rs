use std::convert::TryInto;

use crate::{
    bits::{AudioSessionDisconnectReason, AudioSessionState},
    string::WinStr,
};
use windows::{
    core::PCWSTR,
    Win32::Media::Audio::{IAudioSessionEvents, IAudioSessionEvents_Impl},
};
use windows::{
    core::{implement, GUID},
    Win32::Foundation::BOOL,
};

/// See also: [`IAudioSessionEvents`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiosessionevents)
pub trait AudioSessionEvents: 'static {
    /// See also: [`IAudioSessionEvents::OnChannelVolumeChanged`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionevents-onchannelvolumechanged)
    fn on_channel_volume_changed(
        &self,
        new_volume_levels: &[f32],
        changed_channel: usize,
        event_context: Option<&GUID>,
    ) -> windows::core::Result<()> {
        let _ = (new_volume_levels, changed_channel, event_context);
        Ok(())
    }

    /// See also: [`IAudioSessionEvents::OnDisplayNameChanged`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionevents-ondisplaynamechanged)
    fn on_display_name_changed(
        &self,
        new_display_name: &WinStr,
        event_context: Option<&GUID>,
    ) -> windows::core::Result<()> {
        let _ = (new_display_name, event_context);
        Ok(())
    }

    /// See also: [`IAudioSessionEvents::OnGroupingParamChanged`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionevents-ongroupingparamchanged)
    fn on_grouping_param_changed(
        &self,
        new_grouping_param: Option<&GUID>,
        event_context: Option<&GUID>,
    ) -> windows::core::Result<()> {
        let _ = (new_grouping_param, event_context);
        Ok(())
    }

    /// See also: [`IAudioSessionEvents::OnIconPathChanged`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionevents-oniconpathchanged)
    fn on_icon_path_changed(
        &self,
        new_icon_path: &WinStr,
        event_context: Option<&GUID>,
    ) -> windows::core::Result<()> {
        let _ = (new_icon_path, event_context);
        Ok(())
    }

    /// See also: [`IAudioSessionEvents::OnSessionDisconnected`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionevents-onsessiondisconnected)
    fn on_session_disconnected(
        &self,
        disconnect_reason: AudioSessionDisconnectReason,
    ) -> windows::core::Result<()> {
        let _ = disconnect_reason;
        Ok(())
    }

    /// See also: [`IAudioSessionEvents::OnSimpleVolumeChanged`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionevents-onsimplevolumechanged)
    fn on_simple_volume_changed(
        &self,
        new_volume: f32,
        new_mute: bool,
        event_context: Option<&GUID>,
    ) -> windows::core::Result<()> {
        let _ = (new_volume, new_mute, event_context);
        Ok(())
    }

    /// See also: [`IAudioSessionEvents::OnStateChanged`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionevents-onstatechanged)
    fn on_state_changed(&self, new_state: AudioSessionState) -> windows::core::Result<()> {
        let _ = new_state;
        Ok(())
    }
}

#[implement(IAudioSessionEvents)]
pub(crate) struct AudioSessionEventsWrapper {
    inner: Box<dyn AudioSessionEvents>,
}

impl AudioSessionEventsWrapper {
    pub(crate) fn new<T>(inner: T) -> Self
    where
        T: AudioSessionEvents,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

// impl IAudioSessionEvents
#[allow(non_snake_case)]
impl IAudioSessionEvents_Impl for AudioSessionEventsWrapper {
    fn OnChannelVolumeChanged(
        &self,
        channel_count: u32,
        channel_volume_array: *const f32,
        changed_channel: u32,
        event_context: *const GUID,
    ) -> windows::core::Result<()> {
        self.inner.on_channel_volume_changed(
            unsafe {
                std::slice::from_raw_parts(
                    channel_volume_array as *const f32,
                    channel_count.try_into().unwrap(),
                )
            },
            changed_channel.try_into().unwrap(),
            unsafe { event_context.as_ref() },
        )
    }

    fn OnDisplayNameChanged(
        &self,
        new_display_name: &PCWSTR,
        event_context: *const GUID,
    ) -> windows::core::Result<()> {
        self.inner
            .on_display_name_changed(unsafe { WinStr::from_pcwstr(new_display_name) }, unsafe {
                event_context.as_ref()
            })
    }

    fn OnGroupingParamChanged(
        &self,
        new_grouping_param: *const GUID,
        event_context: *const GUID,
    ) -> windows::core::Result<()> {
        self.inner
            .on_grouping_param_changed(unsafe { new_grouping_param.as_ref() }, unsafe {
                event_context.as_ref()
            })
    }

    fn OnIconPathChanged(
        &self,
        new_icon_path: &PCWSTR,
        event_context: *const GUID,
    ) -> windows::core::Result<()> {
        self.inner
            .on_icon_path_changed(unsafe { WinStr::from_pcwstr(new_icon_path) }, unsafe {
                event_context.as_ref()
            })
    }

    fn OnSessionDisconnected(
        &self,
        disconnect_reason: windows::Win32::Media::Audio::AudioSessionDisconnectReason,
    ) -> windows::core::Result<()> {
        self.inner
            .on_session_disconnected(AudioSessionDisconnectReason::from_raw(disconnect_reason))
    }

    fn OnSimpleVolumeChanged(
        &self,
        new_volume: f32,
        new_mute: BOOL,
        event_context: *const GUID,
    ) -> windows::core::Result<()> {
        self.inner
            .on_simple_volume_changed(new_volume, new_mute.into(), unsafe {
                event_context.as_ref()
            })
    }

    fn OnStateChanged(
        &self,
        new_state: windows::Win32::Media::Audio::AudioSessionState,
    ) -> windows::core::Result<()> {
        self.inner
            .on_state_changed(AudioSessionState::from_raw(new_state))
    }
}
