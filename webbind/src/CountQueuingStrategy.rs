use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CountQueuingStrategy {
    inner: emlite::Val,
}
impl FromVal for CountQueuingStrategy {
    fn from_val(v: &emlite::Val) -> Self {
        CountQueuingStrategy {
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
impl core::ops::Deref for CountQueuingStrategy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CountQueuingStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CountQueuingStrategy> for emlite::Val {
    fn from(s: CountQueuingStrategy) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CountQueuingStrategy {
    pub fn new(init: jsbind::Any) -> CountQueuingStrategy {
        Self {
            inner: emlite::Val::global("CountQueuingStrategy")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl CountQueuingStrategy {
    pub fn high_water_mark(&self) -> f64 {
        self.inner.get("highWaterMark").as_::<f64>()
    }
}
impl CountQueuingStrategy {
    pub fn size(&self) -> jsbind::Function {
        self.inner.get("size").as_::<jsbind::Function>()
    }
}
