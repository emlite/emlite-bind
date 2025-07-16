use super::*;

/// The CSSUnparsedValue class.
/// [`CSSUnparsedValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSUnparsedValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSUnparsedValue {
    inner: CSSStyleValue,
}
impl FromVal for CSSUnparsedValue {
    fn from_val(v: &Any) -> Self {
        CSSUnparsedValue {
            inner: CSSStyleValue::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSUnparsedValue {
    type Target = CSSStyleValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSUnparsedValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSUnparsedValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSUnparsedValue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSUnparsedValue> for Any {
    fn from(s: CSSUnparsedValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSUnparsedValue> for Any {
    fn from(s: &CSSUnparsedValue) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSUnparsedValue);

impl CSSUnparsedValue {
    /// The `new CSSUnparsedValue(..)` constructor, creating a new CSSUnparsedValue instance
    pub fn new(members: &Sequence<Any>) -> CSSUnparsedValue {
        Self {
            inner: Any::global("CSSUnparsedValue")
                .new(&[members.into()])
                .as_::<CSSStyleValue>(),
        }
    }
}
impl CSSUnparsedValue {
    /// Getter of the `length` attribute.
    /// [`CSSUnparsedValue.length`](https://developer.mozilla.org/en-US/docs/Web/API/CSSUnparsedValue/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
