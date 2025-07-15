use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedBoolean {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedBoolean {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedBoolean {
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
impl core::ops::Deref for SVGAnimatedBoolean {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedBoolean {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGAnimatedBoolean {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGAnimatedBoolean {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGAnimatedBoolean> for emlite::Val {
    fn from(s: SVGAnimatedBoolean) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SVGAnimatedBoolean> for emlite::Val {
    fn from(s: &SVGAnimatedBoolean) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimatedBoolean);

impl SVGAnimatedBoolean {
    pub fn base_val(&self) -> bool {
        self.inner.get("baseVal").as_::<bool>()
    }

    pub fn set_base_val(&mut self, value: bool) {
        self.inner.set("baseVal", value);
    }
}
impl SVGAnimatedBoolean {
    pub fn anim_val(&self) -> bool {
        self.inner.get("animVal").as_::<bool>()
    }
}
