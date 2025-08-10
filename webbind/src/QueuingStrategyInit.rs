use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct QueuingStrategyInit {
    inner: Any,
}
impl FromVal for QueuingStrategyInit {
    fn from_val(v: &Any) -> Self {
        QueuingStrategyInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for QueuingStrategyInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for QueuingStrategyInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for QueuingStrategyInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for QueuingStrategyInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<QueuingStrategyInit> for Any {
    fn from(s: QueuingStrategyInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&QueuingStrategyInit> for Any {
    fn from(s: &QueuingStrategyInit) -> Any {
        s.inner.clone()
    }
}

impl QueuingStrategyInit {
    pub fn high_water_mark(&self) -> f64 {
        self.inner.get("highWaterMark").as_::<f64>()
    }

    pub fn set_high_water_mark(&mut self, value: f64) {
        self.inner.set("highWaterMark", value);
    }
}
