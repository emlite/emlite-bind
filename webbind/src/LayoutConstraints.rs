use super::*;

/// The LayoutConstraints class.
/// [`LayoutConstraints`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutConstraints)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutConstraints {
    inner: Any,
}
impl FromVal for LayoutConstraints {
    fn from_val(v: &Any) -> Self {
        LayoutConstraints {
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
impl core::ops::Deref for LayoutConstraints {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LayoutConstraints {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LayoutConstraints {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LayoutConstraints> for Any {
    fn from(s: LayoutConstraints) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LayoutConstraints> for Any {
    fn from(s: &LayoutConstraints) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LayoutConstraints);

impl LayoutConstraints {
    /// Getter of the `availableInlineSize` attribute.
    /// [`LayoutConstraints.availableInlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutConstraints/availableInlineSize)
    pub fn available_inline_size(&self) -> f64 {
        self.inner.get("availableInlineSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    /// Getter of the `availableBlockSize` attribute.
    /// [`LayoutConstraints.availableBlockSize`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutConstraints/availableBlockSize)
    pub fn available_block_size(&self) -> f64 {
        self.inner.get("availableBlockSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    /// Getter of the `fixedInlineSize` attribute.
    /// [`LayoutConstraints.fixedInlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutConstraints/fixedInlineSize)
    pub fn fixed_inline_size(&self) -> f64 {
        self.inner.get("fixedInlineSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    /// Getter of the `fixedBlockSize` attribute.
    /// [`LayoutConstraints.fixedBlockSize`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutConstraints/fixedBlockSize)
    pub fn fixed_block_size(&self) -> f64 {
        self.inner.get("fixedBlockSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    /// Getter of the `percentageInlineSize` attribute.
    /// [`LayoutConstraints.percentageInlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutConstraints/percentageInlineSize)
    pub fn percentage_inline_size(&self) -> f64 {
        self.inner.get("percentageInlineSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    /// Getter of the `percentageBlockSize` attribute.
    /// [`LayoutConstraints.percentageBlockSize`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutConstraints/percentageBlockSize)
    pub fn percentage_block_size(&self) -> f64 {
        self.inner.get("percentageBlockSize").as_::<f64>()
    }
}
impl LayoutConstraints {
    /// Getter of the `blockFragmentationOffset` attribute.
    /// [`LayoutConstraints.blockFragmentationOffset`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutConstraints/blockFragmentationOffset)
    pub fn block_fragmentation_offset(&self) -> f64 {
        self.inner.get("blockFragmentationOffset").as_::<f64>()
    }
}
impl LayoutConstraints {
    /// Getter of the `blockFragmentationType` attribute.
    /// [`LayoutConstraints.blockFragmentationType`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutConstraints/blockFragmentationType)
    pub fn block_fragmentation_type(&self) -> BlockFragmentationType {
        self.inner
            .get("blockFragmentationType")
            .as_::<BlockFragmentationType>()
    }
}
impl LayoutConstraints {
    /// Getter of the `data` attribute.
    /// [`LayoutConstraints.data`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutConstraints/data)
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }
}
