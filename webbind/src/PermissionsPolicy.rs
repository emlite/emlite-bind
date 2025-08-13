use super::*;




/// The PermissionsPolicy class.
/// [`PermissionsPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicy)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PermissionsPolicy {
    inner: Any,
}

impl FromVal for PermissionsPolicy {
    fn from_val(v: &Any) -> Self {
        PermissionsPolicy { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PermissionsPolicy {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PermissionsPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PermissionsPolicy {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PermissionsPolicy {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PermissionsPolicy> for Any {
    fn from(s: PermissionsPolicy) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PermissionsPolicy> for Any {
    fn from(s: &PermissionsPolicy) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PermissionsPolicy);


impl PermissionsPolicy {
    /// The allowsFeature method.
    /// [`PermissionsPolicy.allowsFeature`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicy/allowsFeature)
    pub fn allows_feature0(&self, feature: &JsString) -> bool {
        self.inner.call("allowsFeature", &[feature.into(), ]).as_::<bool>()
    }
    /// The allowsFeature method.
    /// [`PermissionsPolicy.allowsFeature`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicy/allowsFeature)
    pub fn allows_feature1(&self, feature: &JsString, origin: &JsString) -> bool {
        self.inner.call("allowsFeature", &[feature.into(), origin.into(), ]).as_::<bool>()
    }
}
impl PermissionsPolicy {
    /// The features method.
    /// [`PermissionsPolicy.features`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicy/features)
    pub fn features(&self, ) -> TypedArray<JsString> {
        self.inner.call("features", &[]).as_::<TypedArray<JsString>>()
    }
}
impl PermissionsPolicy {
    /// The allowedFeatures method.
    /// [`PermissionsPolicy.allowedFeatures`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicy/allowedFeatures)
    pub fn allowed_features(&self, ) -> TypedArray<JsString> {
        self.inner.call("allowedFeatures", &[]).as_::<TypedArray<JsString>>()
    }
}
impl PermissionsPolicy {
    /// The getAllowlistForFeature method.
    /// [`PermissionsPolicy.getAllowlistForFeature`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicy/getAllowlistForFeature)
    pub fn get_allowlist_for_feature(&self, feature: &JsString) -> TypedArray<JsString> {
        self.inner.call("getAllowlistForFeature", &[feature.into(), ]).as_::<TypedArray<JsString>>()
    }
}
