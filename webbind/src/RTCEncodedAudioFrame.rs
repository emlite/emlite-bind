use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedAudioFrameMetadata {
    inner: Any,
}
impl FromVal for RTCEncodedAudioFrameMetadata {
    fn from_val(v: &Any) -> Self {
        RTCEncodedAudioFrameMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCEncodedAudioFrameMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCEncodedAudioFrameMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCEncodedAudioFrameMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCEncodedAudioFrameMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCEncodedAudioFrameMetadata> for Any {
    fn from(s: RTCEncodedAudioFrameMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCEncodedAudioFrameMetadata> for Any {
    fn from(s: &RTCEncodedAudioFrameMetadata) -> Any {
        s.inner.clone()
    }
}

impl RTCEncodedAudioFrameMetadata {
    pub fn sequence_number(&self) -> i16 {
        self.inner.get("sequenceNumber").as_::<i16>()
    }

    pub fn set_sequence_number(&mut self, value: i16) {
        self.inner.set("sequenceNumber", value);
    }
}
impl RTCEncodedAudioFrameMetadata {
    pub fn audio_level(&self) -> f64 {
        self.inner.get("audioLevel").as_::<f64>()
    }

    pub fn set_audio_level(&mut self, value: f64) {
        self.inner.set("audioLevel", value);
    }
}
/// The RTCEncodedAudioFrame class.
/// [`RTCEncodedAudioFrame`](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedAudioFrame {
    inner: Any,
}
impl FromVal for RTCEncodedAudioFrame {
    fn from_val(v: &Any) -> Self {
        RTCEncodedAudioFrame {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCEncodedAudioFrame {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCEncodedAudioFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCEncodedAudioFrame {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCEncodedAudioFrame {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCEncodedAudioFrame> for Any {
    fn from(s: RTCEncodedAudioFrame) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCEncodedAudioFrame> for Any {
    fn from(s: &RTCEncodedAudioFrame) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCEncodedAudioFrame);

impl RTCEncodedAudioFrame {
    /// The `new RTCEncodedAudioFrame(..)` constructor, creating a new RTCEncodedAudioFrame instance
    pub fn new0(original_frame: &RTCEncodedAudioFrame) -> RTCEncodedAudioFrame {
        Self {
            inner: Any::global("RTCEncodedAudioFrame")
                .new(&[original_frame.into()])
                .as_::<Any>(),
        }
    }

    /// The `new RTCEncodedAudioFrame(..)` constructor, creating a new RTCEncodedAudioFrame instance
    pub fn new1(original_frame: &RTCEncodedAudioFrame, options: &Any) -> RTCEncodedAudioFrame {
        Self {
            inner: Any::global("RTCEncodedAudioFrame")
                .new(&[original_frame.into(), options.into()])
                .as_::<Any>(),
        }
    }
}
impl RTCEncodedAudioFrame {
    /// Getter of the `data` attribute.
    /// [`RTCEncodedAudioFrame.data`](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/data)
    pub fn data(&self) -> ArrayBuffer {
        self.inner.get("data").as_::<ArrayBuffer>()
    }

    /// Setter of the `data` attribute.
    /// [`RTCEncodedAudioFrame.data`](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/data)
    pub fn set_data(&mut self, value: &ArrayBuffer) {
        self.inner.set("data", value);
    }
}
impl RTCEncodedAudioFrame {
    /// The getMetadata method.
    /// [`RTCEncodedAudioFrame.getMetadata`](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/getMetadata)
    pub fn get_metadata(&self) -> RTCEncodedAudioFrameMetadata {
        self.inner
            .call("getMetadata", &[])
            .as_::<RTCEncodedAudioFrameMetadata>()
    }
}
