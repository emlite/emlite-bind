use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Baseline {
    inner: emlite::Val,
}
impl FromVal for Baseline {
    fn from_val(v: &emlite::Val) -> Self {
        Baseline {
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
impl core::ops::Deref for Baseline {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Baseline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Baseline> for emlite::Val {
    fn from(s: Baseline) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Baseline {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl Baseline {
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }
}
