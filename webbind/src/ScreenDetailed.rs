use super::*;

/// The ScreenDetailed class.
/// [`ScreenDetailed`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScreenDetailed {
    inner: Screen,
}
impl FromVal for ScreenDetailed {
    fn from_val(v: &Any) -> Self {
        ScreenDetailed {
            inner: Screen::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ScreenDetailed {
    type Target = Screen;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ScreenDetailed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ScreenDetailed {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ScreenDetailed {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ScreenDetailed> for Any {
    fn from(s: ScreenDetailed) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ScreenDetailed> for Any {
    fn from(s: &ScreenDetailed) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ScreenDetailed);

impl ScreenDetailed {
    /// Getter of the `availLeft` attribute.
    /// [`ScreenDetailed.availLeft`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/availLeft)
    pub fn avail_left(&self) -> i32 {
        self.inner.get("availLeft").as_::<i32>()
    }
}
impl ScreenDetailed {
    /// Getter of the `availTop` attribute.
    /// [`ScreenDetailed.availTop`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/availTop)
    pub fn avail_top(&self) -> i32 {
        self.inner.get("availTop").as_::<i32>()
    }
}
impl ScreenDetailed {
    /// Getter of the `left` attribute.
    /// [`ScreenDetailed.left`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/left)
    pub fn left(&self) -> i32 {
        self.inner.get("left").as_::<i32>()
    }
}
impl ScreenDetailed {
    /// Getter of the `top` attribute.
    /// [`ScreenDetailed.top`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/top)
    pub fn top(&self) -> i32 {
        self.inner.get("top").as_::<i32>()
    }
}
impl ScreenDetailed {
    /// Getter of the `isPrimary` attribute.
    /// [`ScreenDetailed.isPrimary`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/isPrimary)
    pub fn is_primary(&self) -> bool {
        self.inner.get("isPrimary").as_::<bool>()
    }
}
impl ScreenDetailed {
    /// Getter of the `isInternal` attribute.
    /// [`ScreenDetailed.isInternal`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/isInternal)
    pub fn is_internal(&self) -> bool {
        self.inner.get("isInternal").as_::<bool>()
    }
}
impl ScreenDetailed {
    /// Getter of the `devicePixelRatio` attribute.
    /// [`ScreenDetailed.devicePixelRatio`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/devicePixelRatio)
    pub fn device_pixel_ratio(&self) -> f32 {
        self.inner.get("devicePixelRatio").as_::<f32>()
    }
}
impl ScreenDetailed {
    /// Getter of the `label` attribute.
    /// [`ScreenDetailed.label`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/label)
    pub fn label(&self) -> DOMString {
        self.inner.get("label").as_::<DOMString>()
    }
}
