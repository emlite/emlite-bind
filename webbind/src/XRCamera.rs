use super::*;

/// The XRCamera class.
/// [`XRCamera`](https://developer.mozilla.org/en-US/docs/Web/API/XRCamera)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRCamera {
    inner: Any,
}

impl FromVal for XRCamera {
    fn from_val(v: &Any) -> Self {
        XRCamera {
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

impl core::ops::Deref for XRCamera {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRCamera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRCamera {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRCamera {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRCamera> for Any {
    fn from(s: XRCamera) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRCamera> for Any {
    fn from(s: &XRCamera) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRCamera);

impl XRCamera {
    /// Getter of the `width` attribute.
    /// [`XRCamera.width`](https://developer.mozilla.org/en-US/docs/Web/API/XRCamera/width)
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }
}
impl XRCamera {
    /// Getter of the `height` attribute.
    /// [`XRCamera.height`](https://developer.mozilla.org/en-US/docs/Web/API/XRCamera/height)
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }
}
