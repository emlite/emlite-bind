use super::*;




/// The LayoutConstraintsOptions dictionary.
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
    /// Getter of the `availableInlineSize` attribute.
    pub fn available_inline_size(&self) -> f64 {
        self.inner.get("availableInlineSize").as_::<f64>()
    }

    /// Setter of the `availableInlineSize` attribute.
    pub fn set_available_inline_size(&mut self, value: f64) {
        self.inner.set("availableInlineSize", value);
    }
}
impl LayoutConstraintsOptions {
    /// Getter of the `availableBlockSize` attribute.
    pub fn available_block_size(&self) -> f64 {
        self.inner.get("availableBlockSize").as_::<f64>()
    }

    /// Setter of the `availableBlockSize` attribute.
    pub fn set_available_block_size(&mut self, value: f64) {
        self.inner.set("availableBlockSize", value);
    }
}
impl LayoutConstraintsOptions {
    /// Getter of the `fixedInlineSize` attribute.
    pub fn fixed_inline_size(&self) -> f64 {
        self.inner.get("fixedInlineSize").as_::<f64>()
    }

    /// Setter of the `fixedInlineSize` attribute.
    pub fn set_fixed_inline_size(&mut self, value: f64) {
        self.inner.set("fixedInlineSize", value);
    }
}
impl LayoutConstraintsOptions {
    /// Getter of the `fixedBlockSize` attribute.
    pub fn fixed_block_size(&self) -> f64 {
        self.inner.get("fixedBlockSize").as_::<f64>()
    }

    /// Setter of the `fixedBlockSize` attribute.
    pub fn set_fixed_block_size(&mut self, value: f64) {
        self.inner.set("fixedBlockSize", value);
    }
}
impl LayoutConstraintsOptions {
    /// Getter of the `percentageInlineSize` attribute.
    pub fn percentage_inline_size(&self) -> f64 {
        self.inner.get("percentageInlineSize").as_::<f64>()
    }

    /// Setter of the `percentageInlineSize` attribute.
    pub fn set_percentage_inline_size(&mut self, value: f64) {
        self.inner.set("percentageInlineSize", value);
    }
}
impl LayoutConstraintsOptions {
    /// Getter of the `percentageBlockSize` attribute.
    pub fn percentage_block_size(&self) -> f64 {
        self.inner.get("percentageBlockSize").as_::<f64>()
    }

    /// Setter of the `percentageBlockSize` attribute.
    pub fn set_percentage_block_size(&mut self, value: f64) {
        self.inner.set("percentageBlockSize", value);
    }
}
impl LayoutConstraintsOptions {
    /// Getter of the `blockFragmentationOffset` attribute.
    pub fn block_fragmentation_offset(&self) -> f64 {
        self.inner.get("blockFragmentationOffset").as_::<f64>()
    }

    /// Setter of the `blockFragmentationOffset` attribute.
    pub fn set_block_fragmentation_offset(&mut self, value: f64) {
        self.inner.set("blockFragmentationOffset", value);
    }
}
impl LayoutConstraintsOptions {
    /// Getter of the `blockFragmentationType` attribute.
    pub fn block_fragmentation_type(&self) -> BlockFragmentationType {
        self.inner.get("blockFragmentationType").as_::<BlockFragmentationType>()
    }

    /// Setter of the `blockFragmentationType` attribute.
    pub fn set_block_fragmentation_type(&mut self, value: &BlockFragmentationType) {
        self.inner.set("blockFragmentationType", value);
    }
}
impl LayoutConstraintsOptions {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
