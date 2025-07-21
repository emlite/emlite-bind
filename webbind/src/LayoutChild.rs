use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutConstraintsOptions {
    inner: Any,
}
impl FromVal for LayoutConstraintsOptions {
    fn from_val(v: &Any) -> Self {
        LayoutConstraintsOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LayoutConstraintsOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutConstraintsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LayoutConstraintsOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LayoutConstraintsOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LayoutConstraintsOptions> for Any {
    fn from(s: LayoutConstraintsOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LayoutConstraintsOptions> for Any {
    fn from(s: &LayoutConstraintsOptions) -> Any {
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
/// The LayoutChild class.
/// [`LayoutChild`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutChild)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutChild {
    inner: Any,
}
impl FromVal for LayoutChild {
    fn from_val(v: &Any) -> Self {
        LayoutChild {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LayoutChild {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutChild {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LayoutChild {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LayoutChild {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LayoutChild> for Any {
    fn from(s: LayoutChild) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LayoutChild> for Any {
    fn from(s: &LayoutChild) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LayoutChild);

impl LayoutChild {
    /// Getter of the `styleMap` attribute.
    /// [`LayoutChild.styleMap`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutChild/styleMap)
    pub fn style_map(&self) -> StylePropertyMapReadOnly {
        self.inner.get("styleMap").as_::<StylePropertyMapReadOnly>()
    }
}
impl LayoutChild {
    /// The intrinsicSizes method.
    /// [`LayoutChild.intrinsicSizes`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutChild/intrinsicSizes)
    pub fn intrinsic_sizes(&self) -> Promise<IntrinsicSizes> {
        self.inner
            .call("intrinsicSizes", &[])
            .as_::<Promise<IntrinsicSizes>>()
    }
}
impl LayoutChild {
    /// The layoutNextFragment method.
    /// [`LayoutChild.layoutNextFragment`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutChild/layoutNextFragment)
    pub fn layout_next_fragment(
        &self,
        constraints: &LayoutConstraintsOptions,
        break_token: &ChildBreakToken,
    ) -> Promise<LayoutFragment> {
        self.inner
            .call(
                "layoutNextFragment",
                &[constraints.into(), break_token.into()],
            )
            .as_::<Promise<LayoutFragment>>()
    }
}
