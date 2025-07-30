use super::*;

/// The TrustedScript class.
/// [`TrustedScript`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedScript)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrustedScript {
    inner: Any,
}
impl FromVal for TrustedScript {
    fn from_val(v: &Any) -> Self {
        TrustedScript {
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
impl core::ops::Deref for TrustedScript {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TrustedScript {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TrustedScript {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TrustedScript {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TrustedScript> for Any {
    fn from(s: TrustedScript) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TrustedScript> for Any {
    fn from(s: &TrustedScript) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TrustedScript);

impl TrustedScript {
    /// The toJSON method.
    /// [`TrustedScript.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedScript/toJSON)
    pub fn to_json(&self) -> JsString {
        self.inner.call("toJSON", &[]).as_::<JsString>()
    }
}
