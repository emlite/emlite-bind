use super::*;




/// The LayoutFragment class.
/// [`LayoutFragment`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutFragment)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutFragment {
    inner: Any,
}

impl FromVal for LayoutFragment {
    fn from_val(v: &Any) -> Self {
        LayoutFragment { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for LayoutFragment {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LayoutFragment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LayoutFragment {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LayoutFragment {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<LayoutFragment> for Any {
    fn from(s: LayoutFragment) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LayoutFragment> for Any {
    fn from(s: &LayoutFragment) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(LayoutFragment);


impl LayoutFragment {
    /// Getter of the `inlineSize` attribute.
    /// [`LayoutFragment.inlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutFragment/inlineSize)
    pub fn inline_size(&self) -> f64 {
        self.inner.get("inlineSize").as_::<f64>()
    }

}
impl LayoutFragment {
    /// Getter of the `blockSize` attribute.
    /// [`LayoutFragment.blockSize`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutFragment/blockSize)
    pub fn block_size(&self) -> f64 {
        self.inner.get("blockSize").as_::<f64>()
    }

}
impl LayoutFragment {
    /// Getter of the `inlineOffset` attribute.
    /// [`LayoutFragment.inlineOffset`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutFragment/inlineOffset)
    pub fn inline_offset(&self) -> f64 {
        self.inner.get("inlineOffset").as_::<f64>()
    }

    /// Setter of the `inlineOffset` attribute.
    /// [`LayoutFragment.inlineOffset`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutFragment/inlineOffset)
    pub fn set_inline_offset(&mut self, value: f64) {
        self.inner.set("inlineOffset", value);
    }
}
impl LayoutFragment {
    /// Getter of the `blockOffset` attribute.
    /// [`LayoutFragment.blockOffset`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutFragment/blockOffset)
    pub fn block_offset(&self) -> f64 {
        self.inner.get("blockOffset").as_::<f64>()
    }

    /// Setter of the `blockOffset` attribute.
    /// [`LayoutFragment.blockOffset`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutFragment/blockOffset)
    pub fn set_block_offset(&mut self, value: f64) {
        self.inner.set("blockOffset", value);
    }
}
impl LayoutFragment {
    /// Getter of the `data` attribute.
    /// [`LayoutFragment.data`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutFragment/data)
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

}
impl LayoutFragment {
    /// Getter of the `breakToken` attribute.
    /// [`LayoutFragment.breakToken`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutFragment/breakToken)
    pub fn break_token(&self) -> ChildBreakToken {
        self.inner.get("breakToken").as_::<ChildBreakToken>()
    }

}
