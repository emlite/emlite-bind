use super::*;

#[derive(Clone, Debug)]
pub struct ByteLengthQueuingStrategy {
    inner: emlite::Val,
}
impl FromVal for ByteLengthQueuingStrategy {
    fn from_val(v: &emlite::Val) -> Self {
        ByteLengthQueuingStrategy {
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
impl std::ops::Deref for ByteLengthQueuingStrategy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ByteLengthQueuingStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ByteLengthQueuingStrategy> for emlite::Val {
    fn from(s: ByteLengthQueuingStrategy) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ByteLengthQueuingStrategy {
    pub fn new(init: jsbind::Any) -> ByteLengthQueuingStrategy {
        Self {
            inner: emlite::Val::global("ByteLengthQueuingStrategy")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ByteLengthQueuingStrategy {
    pub fn high_water_mark(&self) -> f64 {
        self.inner.get("highWaterMark").as_::<f64>()
    }
}
impl ByteLengthQueuingStrategy {
    pub fn size(&self) -> jsbind::Function {
        self.inner.get("size").as_::<jsbind::Function>()
    }
}
