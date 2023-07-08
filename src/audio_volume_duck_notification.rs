use crate::string::WinStr;
use windows::core::{implement, PCWSTR, PWSTR};
use windows::Win32::Media::Audio::{
    IAudioVolumeDuckNotification, IAudioVolumeDuckNotification_Impl,
};

/// See also: [`IAudioVolumeDuckNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiovolumeducknotification)
pub trait AudioVolumeDuckNotification: 'static {
    /// See also: [`IAudioVolumeDuckNotification::OnVolumeDuckNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiovolumeducknotification-onvolumeducknotification)
    fn on_volume_duck_notification(
        &self,
        session_id: &WinStr,
        num_communication_sessions: u32,
    ) -> windows::core::Result<()> {
        let _ = (session_id, num_communication_sessions);
        Ok(())
    }

    /// See also: [`IAudioVolumeDuckNotification::OnVolumeUnduckNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiovolumeducknotification-onvolumeunducknotification)
    fn on_volume_unduck_notification(&self, session_id: &WinStr) -> windows::core::Result<()> {
        let _ = session_id;
        Ok(())
    }
}

#[implement(IAudioVolumeDuckNotification)]
pub(crate) struct AudioVolumeDuckNotificationWrapper {
    inner: Box<dyn AudioVolumeDuckNotification>,
}

impl AudioVolumeDuckNotificationWrapper {
    pub(crate) fn new<T>(inner: T) -> Self
    where
        T: AudioVolumeDuckNotification,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

// impl IAudioVolumeDuckNotification
#[allow(non_snake_case)]
impl IAudioVolumeDuckNotification_Impl for AudioVolumeDuckNotificationWrapper {
    fn OnVolumeDuckNotification(
        &self,
        session_id: &PCWSTR,
        num_communication_sessions: u32,
    ) -> windows::core::Result<()> {
        self.inner.on_volume_duck_notification(
            unsafe { WinStr::from_pcwstr(&session_id) },
            num_communication_sessions,
        )
    }

    fn OnVolumeUnduckNotification(&self, session_id: &PCWSTR) -> windows::core::Result<()> {
        self.inner
            .on_volume_unduck_notification(unsafe { WinStr::from_pcwstr(&session_id) })
    }
}
