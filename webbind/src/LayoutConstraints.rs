use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LayoutConstraints {
    inner: emlite::Val,
}
impl FromVal for LayoutConstraints {
    fn from_val(v: &emlite::Val) -> Self {
        LayoutConstraints {
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
impl core::ops::Deref for LayoutConstraints {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LayoutConstraints> for emlite::Val {
    fn from(s: LayoutConstraints) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LayoutConstraints {
    pub fn available_inline_size(&self) -> f64 {
        self.inner.get("availableInlineSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    pub fn available_block_size(&self) -> f64 {
        self.inner.get("availableBlockSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    pub fn fixed_inline_size(&self) -> f64 {
        self.inner.get("fixedInlineSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    pub fn fixed_block_size(&self) -> f64 {
        self.inner.get("fixedBlockSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    pub fn percentage_inline_size(&self) -> f64 {
        self.inner.get("percentageInlineSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    pub fn percentage_block_size(&self) -> f64 {
        self.inner.get("percentageBlockSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    pub fn block_fragmentation_offset(&self) -> f64 {
        self.inner.get("blockFragmentationOffset").as_::<f64>()
    }
}
impl LayoutConstraints {
    pub fn block_fragmentation_type(&self) -> BlockFragmentationType {
        self.inner
            .get("blockFragmentationType")
            .as_::<BlockFragmentationType>()
    }
}
impl LayoutConstraints {
    pub fn data(&self) -> jsbind::Any {
        self.inner.get("data").as_::<jsbind::Any>()
    }
}
