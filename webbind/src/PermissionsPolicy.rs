use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PermissionsPolicy {
    inner: emlite::Val,
}
impl FromVal for PermissionsPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        PermissionsPolicy { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PermissionsPolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PermissionsPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PermissionsPolicy {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PermissionsPolicy {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PermissionsPolicy> for emlite::Val {
    fn from(s: PermissionsPolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PermissionsPolicy);


impl PermissionsPolicy {
    pub fn allows_feature0(&self, feature: DOMString) -> bool {
        self.inner.call("allowsFeature", &[feature.into(), ]).as_::<bool>()
    }

    pub fn allows_feature1(&self, feature: DOMString, origin: DOMString) -> bool {
        self.inner.call("allowsFeature", &[feature.into(), origin.into(), ]).as_::<bool>()
    }

}
impl PermissionsPolicy {
    pub fn features(&self, ) -> Sequence<DOMString> {
        self.inner.call("features", &[]).as_::<Sequence<DOMString>>()
    }

}
impl PermissionsPolicy {
    pub fn allowed_features(&self, ) -> Sequence<DOMString> {
        self.inner.call("allowedFeatures", &[]).as_::<Sequence<DOMString>>()
    }

}
impl PermissionsPolicy {
    pub fn get_allowlist_for_feature(&self, feature: DOMString) -> Sequence<DOMString> {
        self.inner.call("getAllowlistForFeature", &[feature.into(), ]).as_::<Sequence<DOMString>>()
    }

}
