use crate::audio_session_control::AudioSessionControl;
use windows::core::implement;
use windows::Win32::Media::Audio::IAudioSessionControl;
use windows::Win32::Media::Audio::IAudioSessionNotification;
use windows::Win32::Media::Audio::IAudioSessionNotification_Impl;

/// See also: [`IAudioSessionNotification`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nn-audiopolicy-iaudiosessionnotification)
pub trait AudioSessionNotification: 'static {
    /// See also: [`IAudioSessionNotification::OnSessionCreated`](https://docs.microsoft.com/en-us/windows/win32/api/audiopolicy/nf-audiopolicy-iaudiosessionnotification-onsessioncreated)
    fn on_session_created(&self, new_session: AudioSessionControl) -> windows::core::Result<()> {
        let _ = new_session;
        Ok(())
    }
}

#[implement(IAudioSessionNotification)]
pub(crate) struct AudioSessionNotificationWrapper {
    inner: Box<dyn AudioSessionNotification>,
}

impl AudioSessionNotificationWrapper {
    pub(crate) fn new<T>(inner: T) -> Self
    where
        T: AudioSessionNotification,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

// impl IAudioSessionNotification
#[allow(non_snake_case)]
impl IAudioSessionNotification_Impl for AudioSessionNotificationWrapper {
    fn OnSessionCreated(
        &self,
        new_session: Option<&IAudioSessionControl>,
    ) -> windows::core::Result<()> {
        self.inner
            .on_session_created(AudioSessionControl::new(new_session.unwrap().to_owned()))
    }
}
