use super::*;




/// The PaintSize class.
/// [`PaintSize`](https://developer.mozilla.org/en-US/docs/Web/API/PaintSize)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaintSize {
    inner: Any,
}

impl FromVal for PaintSize {
    fn from_val(v: &Any) -> Self {
        PaintSize { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PaintSize {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PaintSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PaintSize {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PaintSize {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PaintSize> for Any {
    fn from(s: PaintSize) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PaintSize> for Any {
    fn from(s: &PaintSize) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PaintSize);


impl PaintSize {
    /// Getter of the `width` attribute.
    /// [`PaintSize.width`](https://developer.mozilla.org/en-US/docs/Web/API/PaintSize/width)
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }

}
impl PaintSize {
    /// Getter of the `height` attribute.
    /// [`PaintSize.height`](https://developer.mozilla.org/en-US/docs/Web/API/PaintSize/height)
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }

}
