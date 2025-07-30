use super::*;

/// The CSSTransformValue class.
/// [`CSSTransformValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransformValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSTransformValue {
    inner: CSSStyleValue,
}
impl FromVal for CSSTransformValue {
    fn from_val(v: &Any) -> Self {
        CSSTransformValue {
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
impl core::ops::Deref for CSSTransformValue {
    type Target = CSSStyleValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSTransformValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSTransformValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSTransformValue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSTransformValue> for Any {
    fn from(s: CSSTransformValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSTransformValue> for Any {
    fn from(s: &CSSTransformValue) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSTransformValue);

impl CSSTransformValue {
    /// The `new CSSTransformValue(..)` constructor, creating a new CSSTransformValue instance
    pub fn new(transforms: &TypedArray<CSSTransformComponent>) -> CSSTransformValue {
        Self {
            inner: Any::global("CSSTransformValue")
                .new(&[transforms.into()])
                .as_::<CSSStyleValue>(),
        }
    }
}
impl CSSTransformValue {
    /// Getter of the `length` attribute.
    /// [`CSSTransformValue.length`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransformValue/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl CSSTransformValue {
    /// Getter of the `is2D` attribute.
    /// [`CSSTransformValue.is2D`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransformValue/is2D)
    pub fn is2_d(&self) -> bool {
        self.inner.get("is2D").as_::<bool>()
    }
}
impl CSSTransformValue {
    /// The toMatrix method.
    /// [`CSSTransformValue.toMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransformValue/toMatrix)
    pub fn to_matrix(&self) -> DOMMatrix {
        self.inner.call("toMatrix", &[]).as_::<DOMMatrix>()
    }
}
