use super::*;

#[derive(Clone, Debug)]
pub struct ProtectedAudience {
    inner: emlite::Val,
}
impl FromVal for ProtectedAudience {
    fn from_val(v: &emlite::Val) -> Self {
        ProtectedAudience {
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
impl std::ops::Deref for ProtectedAudience {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ProtectedAudience {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ProtectedAudience> for emlite::Val {
    fn from(s: ProtectedAudience) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ProtectedAudience {
    pub fn query_feature_support(&self, feature: jsbind::DOMString) -> jsbind::Any {
        self.inner
            .call("queryFeatureSupport", &[feature.into()])
            .as_::<jsbind::Any>()
    }
}
