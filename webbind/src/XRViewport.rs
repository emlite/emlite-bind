use super::*;

/// The XRViewport class.
/// [`XRViewport`](https://developer.mozilla.org/en-US/docs/Web/API/XRViewport)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRViewport {
    inner: Any,
}

impl FromVal for XRViewport {
    fn from_val(v: &Any) -> Self {
        XRViewport {
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

impl core::ops::Deref for XRViewport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRViewport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRViewport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRViewport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRViewport> for Any {
    fn from(s: XRViewport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRViewport> for Any {
    fn from(s: &XRViewport) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRViewport);

impl XRViewport {
    /// Getter of the `x` attribute.
    /// [`XRViewport.x`](https://developer.mozilla.org/en-US/docs/Web/API/XRViewport/x)
    pub fn x(&self) -> i32 {
        self.inner.get("x").as_::<i32>()
    }
}
impl XRViewport {
    /// Getter of the `y` attribute.
    /// [`XRViewport.y`](https://developer.mozilla.org/en-US/docs/Web/API/XRViewport/y)
    pub fn y(&self) -> i32 {
        self.inner.get("y").as_::<i32>()
    }
}
impl XRViewport {
    /// Getter of the `width` attribute.
    /// [`XRViewport.width`](https://developer.mozilla.org/en-US/docs/Web/API/XRViewport/width)
    pub fn width(&self) -> i32 {
        self.inner.get("width").as_::<i32>()
    }
}
impl XRViewport {
    /// Getter of the `height` attribute.
    /// [`XRViewport.height`](https://developer.mozilla.org/en-US/docs/Web/API/XRViewport/height)
    pub fn height(&self) -> i32 {
        self.inner.get("height").as_::<i32>()
    }
}
