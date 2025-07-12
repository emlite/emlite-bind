use super::*;

#[derive(Clone, Debug)]
pub struct PermissionsPolicy {
    inner: emlite::Val,
}
impl FromVal for PermissionsPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        PermissionsPolicy {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PermissionsPolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PermissionsPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PermissionsPolicy> for emlite::Val {
    fn from(s: PermissionsPolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PermissionsPolicy {
    pub fn allows_feature0(&self, feature: jsbind::DOMString) -> bool {
        self.inner
            .call("allowsFeature", &[feature.into()])
            .as_::<bool>()
    }

    pub fn allows_feature1(&self, feature: jsbind::DOMString, origin: jsbind::DOMString) -> bool {
        self.inner
            .call("allowsFeature", &[feature.into(), origin.into()])
            .as_::<bool>()
    }
}
impl PermissionsPolicy {
    pub fn features(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .call("features", &[])
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }
}
impl PermissionsPolicy {
    pub fn allowed_features(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .call("allowedFeatures", &[])
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }
}
impl PermissionsPolicy {
    pub fn get_allowlist_for_feature(
        &self,
        feature: jsbind::DOMString,
    ) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .call("getAllowlistForFeature", &[feature.into()])
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }
}
