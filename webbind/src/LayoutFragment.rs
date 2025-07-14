use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutFragment {
    inner: emlite::Val,
}
impl FromVal for LayoutFragment {
    fn from_val(v: &emlite::Val) -> Self {
        LayoutFragment {
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
impl core::ops::Deref for LayoutFragment {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutFragment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LayoutFragment {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LayoutFragment {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LayoutFragment> for emlite::Val {
    fn from(s: LayoutFragment) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(LayoutFragment);

impl LayoutFragment {
    pub fn inline_size(&self) -> f64 {
        self.inner.get("inlineSize").as_::<f64>()
    }
}
impl LayoutFragment {
    pub fn block_size(&self) -> f64 {
        self.inner.get("blockSize").as_::<f64>()
    }
}
impl LayoutFragment {
    pub fn inline_offset(&self) -> f64 {
        self.inner.get("inlineOffset").as_::<f64>()
    }

    pub fn set_inline_offset(&mut self, value: f64) {
        self.inner.set("inlineOffset", value);
    }
}
impl LayoutFragment {
    pub fn block_offset(&self) -> f64 {
        self.inner.get("blockOffset").as_::<f64>()
    }

    pub fn set_block_offset(&mut self, value: f64) {
        self.inner.set("blockOffset", value);
    }
}
impl LayoutFragment {
    pub fn data(&self) -> jsbind::Any {
        self.inner.get("data").as_::<jsbind::Any>()
    }
}
impl LayoutFragment {
    pub fn break_token(&self) -> ChildBreakToken {
        self.inner.get("breakToken").as_::<ChildBreakToken>()
    }
}
