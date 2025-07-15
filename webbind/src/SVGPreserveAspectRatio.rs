use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPreserveAspectRatio {
    inner: emlite::Val,
}
impl FromVal for SVGPreserveAspectRatio {
    fn from_val(v: &emlite::Val) -> Self {
        SVGPreserveAspectRatio {
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
impl core::ops::Deref for SVGPreserveAspectRatio {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGPreserveAspectRatio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGPreserveAspectRatio {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGPreserveAspectRatio {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGPreserveAspectRatio> for emlite::Val {
    fn from(s: SVGPreserveAspectRatio) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SVGPreserveAspectRatio> for emlite::Val {
    fn from(s: &SVGPreserveAspectRatio) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGPreserveAspectRatio);

impl SVGPreserveAspectRatio {
    pub fn align(&self) -> u16 {
        self.inner.get("align").as_::<u16>()
    }

    pub fn set_align(&mut self, value: u16) {
        self.inner.set("align", value);
    }
}
impl SVGPreserveAspectRatio {
    pub fn meet_or_slice(&self) -> u16 {
        self.inner.get("meetOrSlice").as_::<u16>()
    }

    pub fn set_meet_or_slice(&mut self, value: u16) {
        self.inner.set("meetOrSlice", value);
    }
}
