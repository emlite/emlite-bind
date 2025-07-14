use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Viewport {
    inner: emlite::Val,
}
impl FromVal for Viewport {
    fn from_val(v: &emlite::Val) -> Self {
        Viewport {
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
impl core::ops::Deref for Viewport {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Viewport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Viewport> for emlite::Val {
    fn from(s: Viewport) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Viewport {
    pub fn segments(&self) -> jsbind::FrozenArray<DOMRect> {
        self.inner
            .get("segments")
            .as_::<jsbind::FrozenArray<DOMRect>>()
    }
}
