use super::*;

/// The RTCEncodedVideoFrame class.
/// [`RTCEncodedVideoFrame`](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedVideoFrame {
    inner: Any,
}

impl FromVal for RTCEncodedVideoFrame {
    fn from_val(v: &Any) -> Self {
        RTCEncodedVideoFrame {
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

impl core::ops::Deref for RTCEncodedVideoFrame {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCEncodedVideoFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCEncodedVideoFrame {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCEncodedVideoFrame {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCEncodedVideoFrame> for Any {
    fn from(s: RTCEncodedVideoFrame) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCEncodedVideoFrame> for Any {
    fn from(s: &RTCEncodedVideoFrame) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCEncodedVideoFrame);

impl RTCEncodedVideoFrame {
    /// Getter of the `type` attribute.
    /// [`RTCEncodedVideoFrame.type`](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame/type)
    pub fn type_(&self) -> RTCEncodedVideoFrameType {
        self.inner.get("type").as_::<RTCEncodedVideoFrameType>()
    }
}
impl RTCEncodedVideoFrame {
    /// Getter of the `data` attribute.
    /// [`RTCEncodedVideoFrame.data`](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame/data)
    pub fn data(&self) -> ArrayBuffer {
        self.inner.get("data").as_::<ArrayBuffer>()
    }

    /// Setter of the `data` attribute.
    /// [`RTCEncodedVideoFrame.data`](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame/data)
    pub fn set_data(&mut self, value: &ArrayBuffer) {
        self.inner.set("data", value);
    }
}

impl RTCEncodedVideoFrame {
    /// The `new RTCEncodedVideoFrame(..)` constructor, creating a new RTCEncodedVideoFrame instance
    pub fn new(original_frame: &RTCEncodedVideoFrame) -> RTCEncodedVideoFrame {
        Self {
            inner: Any::global("RTCEncodedVideoFrame")
                .new(&[original_frame.into()])
                .as_::<Any>(),
        }
    }
}

impl RTCEncodedVideoFrame {
    /// The `new RTCEncodedVideoFrame(..)` constructor, creating a new RTCEncodedVideoFrame instance
    pub fn new_with_options(
        original_frame: &RTCEncodedVideoFrame,
        options: &RTCEncodedVideoFrameOptions,
    ) -> RTCEncodedVideoFrame {
        Self {
            inner: Any::global("RTCEncodedVideoFrame")
                .new(&[original_frame.into(), options.into()])
                .as_::<Any>(),
        }
    }
}

impl RTCEncodedVideoFrame {
    /// The getMetadata method.
    /// [`RTCEncodedVideoFrame.getMetadata`](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame/getMetadata)
    pub fn get_metadata(&self) -> RTCEncodedVideoFrameMetadata {
        self.inner
            .call("getMetadata", &[])
            .as_::<RTCEncodedVideoFrameMetadata>()
    }
}
