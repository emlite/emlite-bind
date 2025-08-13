use super::*;




/// The ResizeObserverSize class.
/// [`ResizeObserverSize`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverSize)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ResizeObserverSize {
    inner: Any,
}

impl FromVal for ResizeObserverSize {
    fn from_val(v: &Any) -> Self {
        ResizeObserverSize { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ResizeObserverSize {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ResizeObserverSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ResizeObserverSize {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ResizeObserverSize {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ResizeObserverSize> for Any {
    fn from(s: ResizeObserverSize) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ResizeObserverSize> for Any {
    fn from(s: &ResizeObserverSize) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ResizeObserverSize);


impl ResizeObserverSize {
    /// Getter of the `inlineSize` attribute.
    /// [`ResizeObserverSize.inlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverSize/inlineSize)
    pub fn inline_size(&self) -> f64 {
        self.inner.get("inlineSize").as_::<f64>()
    }

}
impl ResizeObserverSize {
    /// Getter of the `blockSize` attribute.
    /// [`ResizeObserverSize.blockSize`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverSize/blockSize)
    pub fn block_size(&self) -> f64 {
        self.inner.get("blockSize").as_::<f64>()
    }

}
