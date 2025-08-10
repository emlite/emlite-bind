use super::*;

/// The CanvasPattern class.
/// [`CanvasPattern`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CanvasPattern {
    inner: Any,
}

impl FromVal for CanvasPattern {
    fn from_val(v: &Any) -> Self {
        CanvasPattern {
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

impl core::ops::Deref for CanvasPattern {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CanvasPattern {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CanvasPattern {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CanvasPattern {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CanvasPattern> for Any {
    fn from(s: CanvasPattern) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CanvasPattern> for Any {
    fn from(s: &CanvasPattern) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CanvasPattern);

impl CanvasPattern {
    /// The setTransform method.
    /// [`CanvasPattern.setTransform`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern/setTransform)
    pub fn set_transform0(&self) -> Undefined {
        self.inner.call("setTransform", &[]).as_::<Undefined>()
    }
    /// The setTransform method.
    /// [`CanvasPattern.setTransform`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern/setTransform)
    pub fn set_transform1(&self, transform: &DOMMatrix2DInit) -> Undefined {
        self.inner
            .call("setTransform", &[transform.into()])
            .as_::<Undefined>()
    }
}
