use super::*;

/// The RTCDTMFSender class.
/// [`RTCDTMFSender`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDTMFSender {
    inner: EventTarget,
}
impl FromVal for RTCDTMFSender {
    fn from_val(v: &Any) -> Self {
        RTCDTMFSender {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCDTMFSender {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDTMFSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCDTMFSender {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCDTMFSender {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCDTMFSender> for Any {
    fn from(s: RTCDTMFSender) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCDTMFSender> for Any {
    fn from(s: &RTCDTMFSender) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCDTMFSender);

impl RTCDTMFSender {
    /// The insertDTMF method.
    /// [`RTCDTMFSender.insertDTMF`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)
    pub fn insert_dtmf0(&self, tones: &str) -> Undefined {
        self.inner
            .call("insertDTMF", &[tones.into()])
            .as_::<Undefined>()
    }
    /// The insertDTMF method.
    /// [`RTCDTMFSender.insertDTMF`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)
    pub fn insert_dtmf1(&self, tones: &str, duration: u32) -> Undefined {
        self.inner
            .call("insertDTMF", &[tones.into(), duration.into()])
            .as_::<Undefined>()
    }
    /// The insertDTMF method.
    /// [`RTCDTMFSender.insertDTMF`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)
    pub fn insert_dtmf2(&self, tones: &str, duration: u32, inter_tone_gap: u32) -> Undefined {
        self.inner
            .call(
                "insertDTMF",
                &[tones.into(), duration.into(), inter_tone_gap.into()],
            )
            .as_::<Undefined>()
    }
}
impl RTCDTMFSender {
    /// Getter of the `ontonechange` attribute.
    /// [`RTCDTMFSender.ontonechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/ontonechange)
    pub fn ontonechange(&self) -> Any {
        self.inner.get("ontonechange").as_::<Any>()
    }

    /// Setter of the `ontonechange` attribute.
    /// [`RTCDTMFSender.ontonechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/ontonechange)
    pub fn set_ontonechange(&mut self, value: &Any) {
        self.inner.set("ontonechange", value);
    }
}
impl RTCDTMFSender {
    /// Getter of the `canInsertDTMF` attribute.
    /// [`RTCDTMFSender.canInsertDTMF`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/canInsertDTMF)
    pub fn can_insert_dtmf(&self) -> bool {
        self.inner.get("canInsertDTMF").as_::<bool>()
    }
}
impl RTCDTMFSender {
    /// Getter of the `toneBuffer` attribute.
    /// [`RTCDTMFSender.toneBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/toneBuffer)
    pub fn tone_buffer(&self) -> String {
        self.inner.get("toneBuffer").as_::<String>()
    }
}
