use super::*;

#[derive(Clone, Debug)]
pub struct IntrinsicSizes {
    inner: emlite::Val,
}
impl FromVal for IntrinsicSizes {
    fn from_val(v: &emlite::Val) -> Self {
        IntrinsicSizes {
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
impl std::ops::Deref for IntrinsicSizes {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IntrinsicSizes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IntrinsicSizes> for emlite::Val {
    fn from(s: IntrinsicSizes) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IntrinsicSizes {
    pub fn min_content_size(&self) -> f64 {
        self.inner.get("minContentSize").as_::<f64>()
    }
}
impl IntrinsicSizes {
    pub fn max_content_size(&self) -> f64 {
        self.inner.get("maxContentSize").as_::<f64>()
    }
}
