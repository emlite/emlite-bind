use super::*;

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
    pub fn new1(
        original_frame: &RTCEncodedAudioFrame,
        options: &RTCEncodedAudioFrameOptions,
    ) -> RTCEncodedAudioFrame {
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
