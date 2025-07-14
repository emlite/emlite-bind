use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ShadowAnimation {
    inner: Animation,
}
impl FromVal for ShadowAnimation {
    fn from_val(v: &emlite::Val) -> Self {
        ShadowAnimation {
            inner: Animation::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ShadowAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ShadowAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ShadowAnimation> for emlite::Val {
    fn from(s: ShadowAnimation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ShadowAnimation {
    pub fn new(source: Animation, new_target: jsbind::Any) -> ShadowAnimation {
        Self {
            inner: emlite::Val::global("ShadowAnimation")
                .new(&[source.into(), new_target.into()])
                .as_::<Animation>(),
        }
    }
}
impl ShadowAnimation {
    pub fn source_animation(&self) -> Animation {
        self.inner.get("sourceAnimation").as_::<Animation>()
    }
}
