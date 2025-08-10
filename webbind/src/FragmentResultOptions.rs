use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FragmentResultOptions {
    inner: Any,
}
impl FromVal for FragmentResultOptions {
    fn from_val(v: &Any) -> Self {
        FragmentResultOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FragmentResultOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FragmentResultOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FragmentResultOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FragmentResultOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FragmentResultOptions> for Any {
    fn from(s: FragmentResultOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FragmentResultOptions> for Any {
    fn from(s: &FragmentResultOptions) -> Any {
        s.inner.clone()
    }
}

impl FragmentResultOptions {
    pub fn inline_size(&self) -> f64 {
        self.inner.get("inlineSize").as_::<f64>()
    }

    pub fn set_inline_size(&mut self, value: f64) {
        self.inner.set("inlineSize", value);
    }
}
impl FragmentResultOptions {
    pub fn block_size(&self) -> f64 {
        self.inner.get("blockSize").as_::<f64>()
    }

    pub fn set_block_size(&mut self, value: f64) {
        self.inner.set("blockSize", value);
    }
}
impl FragmentResultOptions {
    pub fn auto_block_size(&self) -> f64 {
        self.inner.get("autoBlockSize").as_::<f64>()
    }

    pub fn set_auto_block_size(&mut self, value: f64) {
        self.inner.set("autoBlockSize", value);
    }
}
impl FragmentResultOptions {
    pub fn child_fragments(&self) -> TypedArray<LayoutFragment> {
        self.inner
            .get("childFragments")
            .as_::<TypedArray<LayoutFragment>>()
    }

    pub fn set_child_fragments(&mut self, value: &TypedArray<LayoutFragment>) {
        self.inner.set("childFragments", value);
    }
}
impl FragmentResultOptions {
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
impl FragmentResultOptions {
    pub fn break_token(&self) -> BreakTokenOptions {
        self.inner.get("breakToken").as_::<BreakTokenOptions>()
    }

    pub fn set_break_token(&mut self, value: &BreakTokenOptions) {
        self.inner.set("breakToken", value);
    }
}
