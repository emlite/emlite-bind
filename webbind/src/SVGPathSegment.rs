use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPathSegment {
    inner: emlite::Val,
}
impl FromVal for SVGPathSegment {
    fn from_val(v: &emlite::Val) -> Self {
        SVGPathSegment {
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
impl core::ops::Deref for SVGPathSegment {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGPathSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGPathSegment {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGPathSegment {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGPathSegment> for emlite::Val {
    fn from(s: SVGPathSegment) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SVGPathSegment> for emlite::Val {
    fn from(s: &SVGPathSegment) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGPathSegment);

impl SVGPathSegment {
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }

    pub fn set_type_(&mut self, value: &str) {
        self.inner.set("type", value);
    }
}
impl SVGPathSegment {
    pub fn values(&self) -> FrozenArray<f32> {
        self.inner.get("values").as_::<FrozenArray<f32>>()
    }

    pub fn set_values(&mut self, value: FrozenArray<f32>) {
        self.inner.set("values", value);
    }
}
