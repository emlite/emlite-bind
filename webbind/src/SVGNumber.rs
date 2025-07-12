use super::*;

#[derive(Clone, Debug)]
pub struct SVGNumber {
    inner: emlite::Val,
}
impl FromVal for SVGNumber {
    fn from_val(v: &emlite::Val) -> Self {
        SVGNumber {
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
impl std::ops::Deref for SVGNumber {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGNumber> for emlite::Val {
    fn from(s: SVGNumber) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGNumber {
    pub fn value(&self) -> f32 {
        self.inner.get("value").as_::<f32>()
    }

    pub fn set_value(&mut self, value: f32) {
        self.inner.set("value", value);
    }
}
