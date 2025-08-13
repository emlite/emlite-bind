use super::*;




/// The ImageDecoder class.
/// [`ImageDecoder`](https://developer.mozilla.org/en-US/docs/Web/API/ImageDecoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageDecoder {
    inner: Any,
}

impl FromVal for ImageDecoder {
    fn from_val(v: &Any) -> Self {
        ImageDecoder { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageDecoder {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageDecoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageDecoder {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ImageDecoder> for Any {
    fn from(s: ImageDecoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageDecoder> for Any {
    fn from(s: &ImageDecoder) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ImageDecoder);



impl ImageDecoder {
    /// The `new ImageDecoder(..)` constructor, creating a new ImageDecoder instance
    pub fn new(init: &ImageDecoderInit) -> ImageDecoder {
        Self {
            inner: Any::global("ImageDecoder").new(&[init.into()]).as_::<Any>(),
        }
    }

}
impl ImageDecoder {
    /// Getter of the `type` attribute.
    /// [`ImageDecoder.type`](https://developer.mozilla.org/en-US/docs/Web/API/ImageDecoder/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

}
impl ImageDecoder {
    /// Getter of the `complete` attribute.
    /// [`ImageDecoder.complete`](https://developer.mozilla.org/en-US/docs/Web/API/ImageDecoder/complete)
    pub fn complete(&self) -> bool {
        self.inner.get("complete").as_::<bool>()
    }

}
impl ImageDecoder {
    /// Getter of the `completed` attribute.
    /// [`ImageDecoder.completed`](https://developer.mozilla.org/en-US/docs/Web/API/ImageDecoder/completed)
    pub fn completed(&self) -> Promise<Undefined> {
        self.inner.get("completed").as_::<Promise<Undefined>>()
    }

}
impl ImageDecoder {
    /// Getter of the `tracks` attribute.
    /// [`ImageDecoder.tracks`](https://developer.mozilla.org/en-US/docs/Web/API/ImageDecoder/tracks)
    pub fn tracks(&self) -> ImageTrackList {
        self.inner.get("tracks").as_::<ImageTrackList>()
    }

}
impl ImageDecoder {
    /// The decode method.
    /// [`ImageDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/ImageDecoder/decode)
    pub fn decode0(&self, ) -> Promise<ImageDecodeResult> {
        self.inner.call("decode", &[]).as_::<Promise<ImageDecodeResult>>()
    }
    /// The decode method.
    /// [`ImageDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/ImageDecoder/decode)
    pub fn decode1(&self, options: &ImageDecodeOptions) -> Promise<ImageDecodeResult> {
        self.inner.call("decode", &[options.into(), ]).as_::<Promise<ImageDecodeResult>>()
    }
}
impl ImageDecoder {
    /// The reset method.
    /// [`ImageDecoder.reset`](https://developer.mozilla.org/en-US/docs/Web/API/ImageDecoder/reset)
    pub fn reset(&self, ) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl ImageDecoder {
    /// The close method.
    /// [`ImageDecoder.close`](https://developer.mozilla.org/en-US/docs/Web/API/ImageDecoder/close)
    pub fn close(&self, ) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl ImageDecoder {
    /// The isTypeSupported method.
    /// [`ImageDecoder.isTypeSupported`](https://developer.mozilla.org/en-US/docs/Web/API/ImageDecoder/isTypeSupported)
    pub fn is_type_supported(type_: &JsString) -> Promise<bool> {
        Any::global("ImageDecoder").call("isTypeSupported", &[type_.into(), ]).as_::<Promise<bool>>()
    }
}
