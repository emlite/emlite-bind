use super::*;

/// The CSSFunctionDescriptors class.
/// [`CSSFunctionDescriptors`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionDescriptors)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFunctionDescriptors {
    inner: CSSStyleDeclaration,
}
impl FromVal for CSSFunctionDescriptors {
    fn from_val(v: &Any) -> Self {
        CSSFunctionDescriptors {
            inner: CSSStyleDeclaration::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSFunctionDescriptors {
    type Target = CSSStyleDeclaration;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSFunctionDescriptors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSFunctionDescriptors {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSFunctionDescriptors {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSFunctionDescriptors> for Any {
    fn from(s: CSSFunctionDescriptors) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSFunctionDescriptors> for Any {
    fn from(s: &CSSFunctionDescriptors) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSFunctionDescriptors);

impl CSSFunctionDescriptors {
    /// Getter of the `result` attribute.
    /// [`CSSFunctionDescriptors.result`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionDescriptors/result)
    pub fn result(&self) -> CSSOMString {
        self.inner.get("result").as_::<CSSOMString>()
    }

    /// Setter of the `result` attribute.
    /// [`CSSFunctionDescriptors.result`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionDescriptors/result)
    pub fn set_result(&mut self, value: &CSSOMString) {
        self.inner.set("result", value);
    }
}
