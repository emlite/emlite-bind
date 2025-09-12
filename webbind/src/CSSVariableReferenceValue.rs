use super::*;

/// The CSSVariableReferenceValue class.
/// [`CSSVariableReferenceValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSVariableReferenceValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSVariableReferenceValue {
    inner: Any,
}

impl FromVal for CSSVariableReferenceValue {
    fn from_val(v: &Any) -> Self {
        CSSVariableReferenceValue {
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

impl core::ops::Deref for CSSVariableReferenceValue {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSVariableReferenceValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSVariableReferenceValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSVariableReferenceValue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSVariableReferenceValue> for Any {
    fn from(s: CSSVariableReferenceValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSVariableReferenceValue> for Any {
    fn from(s: &CSSVariableReferenceValue) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSVariableReferenceValue);

impl CSSVariableReferenceValue {
    /// Getter of the `variable` attribute.
    /// [`CSSVariableReferenceValue.variable`](https://developer.mozilla.org/en-US/docs/Web/API/CSSVariableReferenceValue/variable)
    pub fn variable(&self) -> JsString {
        self.inner.get("variable").as_::<JsString>()
    }

    /// Setter of the `variable` attribute.
    /// [`CSSVariableReferenceValue.variable`](https://developer.mozilla.org/en-US/docs/Web/API/CSSVariableReferenceValue/variable)
    pub fn set_variable(&mut self, value: &JsString) {
        self.inner.set("variable", value);
    }
}
impl CSSVariableReferenceValue {
    /// Getter of the `fallback` attribute.
    /// [`CSSVariableReferenceValue.fallback`](https://developer.mozilla.org/en-US/docs/Web/API/CSSVariableReferenceValue/fallback)
    pub fn fallback(&self) -> CSSUnparsedValue {
        self.inner.get("fallback").as_::<CSSUnparsedValue>()
    }
}

impl CSSVariableReferenceValue {
    /// The `new CSSVariableReferenceValue(..)` constructor, creating a new CSSVariableReferenceValue instance
    pub fn new(variable: &JsString) -> CSSVariableReferenceValue {
        Self {
            inner: Any::global("CSSVariableReferenceValue")
                .new(&[variable.into()])
                .as_::<Any>(),
        }
    }
}

impl CSSVariableReferenceValue {
    /// The `new CSSVariableReferenceValue(..)` constructor, creating a new CSSVariableReferenceValue instance
    pub fn new_with_fallback(
        variable: &JsString,
        fallback: &CSSUnparsedValue,
    ) -> CSSVariableReferenceValue {
        Self {
            inner: Any::global("CSSVariableReferenceValue")
                .new(&[variable.into(), fallback.into()])
                .as_::<Any>(),
        }
    }
}
