use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for SVGPathSegment {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGPathSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGPathSegment> for emlite::Val {
    fn from(s: SVGPathSegment) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGPathSegment {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
impl SVGPathSegment {
    pub fn values(&self) -> jsbind::FrozenArray<f32> {
        self.inner.get("values").as_::<jsbind::FrozenArray<f32>>()
    }

    pub fn set_values(&mut self, value: jsbind::FrozenArray<f32>) {
        self.inner.set("values", value);
    }
}
