use super::*;

/// The SFrameTransform class.
/// [`SFrameTransform`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SFrameTransform {
    inner: EventTarget,
}

impl FromVal for SFrameTransform {
    fn from_val(v: &Any) -> Self {
        SFrameTransform {
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

impl core::ops::Deref for SFrameTransform {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SFrameTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SFrameTransform {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SFrameTransform {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SFrameTransform> for Any {
    fn from(s: SFrameTransform) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SFrameTransform> for Any {
    fn from(s: &SFrameTransform) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SFrameTransform);

impl SFrameTransform {
    /// Getter of the `onerror` attribute.
    /// [`SFrameTransform.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`SFrameTransform.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl SFrameTransform {
    /// Getter of the `readable` attribute.
    /// [`SFrameTransform.readable`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl SFrameTransform {
    /// Getter of the `writable` attribute.
    /// [`SFrameTransform.writable`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/writable)
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}

impl SFrameTransform {
    /// The `new SFrameTransform(..)` constructor, creating a new SFrameTransform instance
    pub fn new0() -> SFrameTransform {
        Self {
            inner: Any::global("SFrameTransform").new(&[]).as_::<EventTarget>(),
        }
    }

    /// The `new SFrameTransform(..)` constructor, creating a new SFrameTransform instance
    pub fn new1(options: &SFrameTransformOptions) -> SFrameTransform {
        Self {
            inner: Any::global("SFrameTransform")
                .new(&[options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl SFrameTransform {
    /// The setEncryptionKey method.
    /// [`SFrameTransform.setEncryptionKey`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/setEncryptionKey)
    pub fn set_encryption_key0(&self, key: &CryptoKey) -> Promise<Undefined> {
        self.inner
            .call("setEncryptionKey", &[key.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The setEncryptionKey method.
    /// [`SFrameTransform.setEncryptionKey`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/setEncryptionKey)
    pub fn set_encryption_key1(&self, key: &CryptoKey, key_id: &Any) -> Promise<Undefined> {
        self.inner
            .call("setEncryptionKey", &[key.into(), key_id.into()])
            .as_::<Promise<Undefined>>()
    }
}
