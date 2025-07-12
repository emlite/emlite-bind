use super::*;

#[derive(Clone, Debug)]
pub struct AttributionAggregationServices {
    inner: emlite::Val,
}
impl FromVal for AttributionAggregationServices {
    fn from_val(v: &emlite::Val) -> Self {
        AttributionAggregationServices {
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
impl std::ops::Deref for AttributionAggregationServices {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AttributionAggregationServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AttributionAggregationServices> for emlite::Val {
    fn from(s: AttributionAggregationServices) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
