use super::*;




/// The TrustedScriptURL class.
/// [`TrustedScriptURL`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedScriptURL)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrustedScriptURL {
    inner: Any,
}

impl FromVal for TrustedScriptURL {
    fn from_val(v: &Any) -> Self {
        TrustedScriptURL { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TrustedScriptURL {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TrustedScriptURL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TrustedScriptURL {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TrustedScriptURL {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<TrustedScriptURL> for Any {
    fn from(s: TrustedScriptURL) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TrustedScriptURL> for Any {
    fn from(s: &TrustedScriptURL) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TrustedScriptURL);


impl TrustedScriptURL {
    /// The toJSON method.
    /// [`TrustedScriptURL.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedScriptURL/toJSON)
    pub fn to_json(&self, ) -> JsString {
        self.inner.call("toJSON", &[]).as_::<JsString>()
    }
}
