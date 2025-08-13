use super::*;




/// The TrustedHTML class.
/// [`TrustedHTML`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedHTML)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrustedHTML {
    inner: Any,
}

impl FromVal for TrustedHTML {
    fn from_val(v: &Any) -> Self {
        TrustedHTML { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TrustedHTML {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TrustedHTML {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TrustedHTML {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TrustedHTML {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<TrustedHTML> for Any {
    fn from(s: TrustedHTML) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TrustedHTML> for Any {
    fn from(s: &TrustedHTML) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TrustedHTML);


impl TrustedHTML {
    /// The toJSON method.
    /// [`TrustedHTML.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedHTML/toJSON)
    pub fn to_json(&self, ) -> JsString {
        self.inner.call("toJSON", &[]).as_::<JsString>()
    }
}
