use super::*;

/// The DOMException class.
/// [`DOMException`](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMException {
    inner: Any,
}
impl FromVal for DOMException {
    fn from_val(v: &Any) -> Self {
        DOMException {
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
impl core::ops::Deref for DOMException {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DOMException {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DOMException {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DOMException> for Any {
    fn from(s: DOMException) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DOMException> for Any {
    fn from(s: &DOMException) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DOMException);

impl DOMException {
    /// The `new DOMException(..)` constructor, creating a new DOMException instance
    pub fn new0() -> DOMException {
        Self {
            inner: Any::global("DOMException").new(&[]).as_::<Any>(),
        }
    }

    /// The `new DOMException(..)` constructor, creating a new DOMException instance
    pub fn new1(message: &JsString) -> DOMException {
        Self {
            inner: Any::global("DOMException")
                .new(&[message.into()])
                .as_::<Any>(),
        }
    }

    /// The `new DOMException(..)` constructor, creating a new DOMException instance
    pub fn new2(message: &JsString, name: &JsString) -> DOMException {
        Self {
            inner: Any::global("DOMException")
                .new(&[message.into(), name.into()])
                .as_::<Any>(),
        }
    }
}
impl DOMException {
    /// Getter of the `name` attribute.
    /// [`DOMException.name`](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl DOMException {
    /// Getter of the `message` attribute.
    /// [`DOMException.message`](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/message)
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }
}
impl DOMException {
    /// Getter of the `code` attribute.
    /// [`DOMException.code`](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/code)
    pub fn code(&self) -> u16 {
        self.inner.get("code").as_::<u16>()
    }
}
