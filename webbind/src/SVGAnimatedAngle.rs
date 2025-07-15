use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedAngle {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedAngle {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedAngle {
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
impl core::ops::Deref for SVGAnimatedAngle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedAngle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGAnimatedAngle {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGAnimatedAngle {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGAnimatedAngle> for emlite::Val {
    fn from(s: SVGAnimatedAngle) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SVGAnimatedAngle> for emlite::Val {
    fn from(s: &SVGAnimatedAngle) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimatedAngle);

impl SVGAnimatedAngle {
    pub fn base_val(&self) -> SVGAngle {
        self.inner.get("baseVal").as_::<SVGAngle>()
    }
}
impl SVGAnimatedAngle {
    pub fn anim_val(&self) -> SVGAngle {
        self.inner.get("animVal").as_::<SVGAngle>()
    }
}
