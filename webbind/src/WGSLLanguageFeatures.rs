use super::*;

/// The WGSLLanguageFeatures class.
/// [`WGSLLanguageFeatures`](https://developer.mozilla.org/en-US/docs/Web/API/WGSLLanguageFeatures)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WGSLLanguageFeatures {
    inner: Any,
}

impl FromVal for WGSLLanguageFeatures {
    fn from_val(v: &Any) -> Self {
        WGSLLanguageFeatures {
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

impl core::ops::Deref for WGSLLanguageFeatures {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WGSLLanguageFeatures {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WGSLLanguageFeatures {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WGSLLanguageFeatures {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WGSLLanguageFeatures> for Any {
    fn from(s: WGSLLanguageFeatures) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WGSLLanguageFeatures> for Any {
    fn from(s: &WGSLLanguageFeatures) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WGSLLanguageFeatures);
