use super::*;

/// The Viewport class.
/// [`Viewport`](https://developer.mozilla.org/en-US/docs/Web/API/Viewport)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Viewport {
    inner: Any,
}

impl FromVal for Viewport {
    fn from_val(v: &Any) -> Self {
        Viewport {
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

impl core::ops::Deref for Viewport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Viewport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Viewport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Viewport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Viewport> for Any {
    fn from(s: Viewport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Viewport> for Any {
    fn from(s: &Viewport) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Viewport);

impl Viewport {
    /// Getter of the `segments` attribute.
    /// [`Viewport.segments`](https://developer.mozilla.org/en-US/docs/Web/API/Viewport/segments)
    pub fn segments(&self) -> TypedArray<DOMRect> {
        self.inner.get("segments").as_::<TypedArray<DOMRect>>()
    }
}
