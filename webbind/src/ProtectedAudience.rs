use super::*;

/// The ProtectedAudience class.
/// [`ProtectedAudience`](https://developer.mozilla.org/en-US/docs/Web/API/ProtectedAudience)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProtectedAudience {
    inner: Any,
}
impl FromVal for ProtectedAudience {
    fn from_val(v: &Any) -> Self {
        ProtectedAudience {
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
impl core::ops::Deref for ProtectedAudience {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ProtectedAudience {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ProtectedAudience {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ProtectedAudience {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ProtectedAudience> for Any {
    fn from(s: ProtectedAudience) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ProtectedAudience> for Any {
    fn from(s: &ProtectedAudience) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ProtectedAudience);

impl ProtectedAudience {
    /// The queryFeatureSupport method.
    /// [`ProtectedAudience.queryFeatureSupport`](https://developer.mozilla.org/en-US/docs/Web/API/ProtectedAudience/queryFeatureSupport)
    pub fn query_feature_support(&self, feature: &JsString) -> Any {
        self.inner
            .call("queryFeatureSupport", &[feature.into()])
            .as_::<Any>()
    }
}
