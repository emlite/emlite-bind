use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutConstraintsOptions {
    inner: emlite::Val,
}
impl FromVal for LayoutConstraintsOptions {
    fn from_val(v: &emlite::Val) -> Self {
        LayoutConstraintsOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LayoutConstraintsOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutConstraintsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LayoutConstraintsOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LayoutConstraintsOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LayoutConstraintsOptions> for emlite::Val {
    fn from(s: LayoutConstraintsOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&LayoutConstraintsOptions> for emlite::Val {
    fn from(s: &LayoutConstraintsOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl LayoutConstraintsOptions {
    pub fn available_inline_size(&self) -> f64 {
        self.inner.get("availableInlineSize").as_::<f64>()
    }

    pub fn set_available_inline_size(&mut self, value: f64) {
        self.inner.set("availableInlineSize", value);
    }
}
impl LayoutConstraintsOptions {
    pub fn available_block_size(&self) -> f64 {
        self.inner.get("availableBlockSize").as_::<f64>()
    }

    pub fn set_available_block_size(&mut self, value: f64) {
        self.inner.set("availableBlockSize", value);
    }
}
impl LayoutConstraintsOptions {
    pub fn fixed_inline_size(&self) -> f64 {
        self.inner.get("fixedInlineSize").as_::<f64>()
    }

    pub fn set_fixed_inline_size(&mut self, value: f64) {
        self.inner.set("fixedInlineSize", value);
    }
}
impl LayoutConstraintsOptions {
    pub fn fixed_block_size(&self) -> f64 {
        self.inner.get("fixedBlockSize").as_::<f64>()
    }

    pub fn set_fixed_block_size(&mut self, value: f64) {
        self.inner.set("fixedBlockSize", value);
    }
}
impl LayoutConstraintsOptions {
    pub fn percentage_inline_size(&self) -> f64 {
        self.inner.get("percentageInlineSize").as_::<f64>()
    }

    pub fn set_percentage_inline_size(&mut self, value: f64) {
        self.inner.set("percentageInlineSize", value);
    }
}
impl LayoutConstraintsOptions {
    pub fn percentage_block_size(&self) -> f64 {
        self.inner.get("percentageBlockSize").as_::<f64>()
    }

    pub fn set_percentage_block_size(&mut self, value: f64) {
        self.inner.set("percentageBlockSize", value);
    }
}
impl LayoutConstraintsOptions {
    pub fn block_fragmentation_offset(&self) -> f64 {
        self.inner.get("blockFragmentationOffset").as_::<f64>()
    }

    pub fn set_block_fragmentation_offset(&mut self, value: f64) {
        self.inner.set("blockFragmentationOffset", value);
    }
}
impl LayoutConstraintsOptions {
    pub fn block_fragmentation_type(&self) -> BlockFragmentationType {
        self.inner
            .get("blockFragmentationType")
            .as_::<BlockFragmentationType>()
    }

    pub fn set_block_fragmentation_type(&mut self, value: &BlockFragmentationType) {
        self.inner.set("blockFragmentationType", value);
    }
}
impl LayoutConstraintsOptions {
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutChild {
    inner: emlite::Val,
}
impl FromVal for LayoutChild {
    fn from_val(v: &emlite::Val) -> Self {
        LayoutChild {
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
impl core::ops::Deref for LayoutChild {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutChild {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LayoutChild {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LayoutChild {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LayoutChild> for emlite::Val {
    fn from(s: LayoutChild) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&LayoutChild> for emlite::Val {
    fn from(s: &LayoutChild) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LayoutChild);

impl LayoutChild {
    pub fn style_map(&self) -> StylePropertyMapReadOnly {
        self.inner.get("styleMap").as_::<StylePropertyMapReadOnly>()
    }
}
impl LayoutChild {
    pub fn intrinsic_sizes(&self) -> Promise {
        self.inner.call("intrinsicSizes", &[]).as_::<Promise>()
    }
}
impl LayoutChild {
    pub fn layout_next_fragment(
        &self,
        constraints: &LayoutConstraintsOptions,
        break_token: &ChildBreakToken,
    ) -> Promise {
        self.inner
            .call(
                "layoutNextFragment",
                &[constraints.into(), break_token.into()],
            )
            .as_::<Promise>()
    }
}
