use super::*;




/// The VTTRegion class.
/// [`VTTRegion`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VTTRegion {
    inner: Any,
}

impl FromVal for VTTRegion {
    fn from_val(v: &Any) -> Self {
        VTTRegion { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VTTRegion {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VTTRegion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VTTRegion {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VTTRegion {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<VTTRegion> for Any {
    fn from(s: VTTRegion) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VTTRegion> for Any {
    fn from(s: &VTTRegion) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(VTTRegion);



impl VTTRegion {
    /// The `new VTTRegion(..)` constructor, creating a new VTTRegion instance
    pub fn new() -> VTTRegion {
        Self {
            inner: Any::global("VTTRegion").new(&[]).as_::<Any>(),
        }
    }

}
impl VTTRegion {
    /// Getter of the `id` attribute.
    /// [`VTTRegion.id`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    /// [`VTTRegion.id`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/id)
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl VTTRegion {
    /// Getter of the `width` attribute.
    /// [`VTTRegion.width`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/width)
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }

    /// Setter of the `width` attribute.
    /// [`VTTRegion.width`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/width)
    pub fn set_width(&mut self, value: f64) {
        self.inner.set("width", value);
    }
}
impl VTTRegion {
    /// Getter of the `lines` attribute.
    /// [`VTTRegion.lines`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/lines)
    pub fn lines(&self) -> u32 {
        self.inner.get("lines").as_::<u32>()
    }

    /// Setter of the `lines` attribute.
    /// [`VTTRegion.lines`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/lines)
    pub fn set_lines(&mut self, value: u32) {
        self.inner.set("lines", value);
    }
}
impl VTTRegion {
    /// Getter of the `regionAnchorX` attribute.
    /// [`VTTRegion.regionAnchorX`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorX)
    pub fn region_anchor_x(&self) -> f64 {
        self.inner.get("regionAnchorX").as_::<f64>()
    }

    /// Setter of the `regionAnchorX` attribute.
    /// [`VTTRegion.regionAnchorX`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorX)
    pub fn set_region_anchor_x(&mut self, value: f64) {
        self.inner.set("regionAnchorX", value);
    }
}
impl VTTRegion {
    /// Getter of the `regionAnchorY` attribute.
    /// [`VTTRegion.regionAnchorY`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorY)
    pub fn region_anchor_y(&self) -> f64 {
        self.inner.get("regionAnchorY").as_::<f64>()
    }

    /// Setter of the `regionAnchorY` attribute.
    /// [`VTTRegion.regionAnchorY`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorY)
    pub fn set_region_anchor_y(&mut self, value: f64) {
        self.inner.set("regionAnchorY", value);
    }
}
impl VTTRegion {
    /// Getter of the `viewportAnchorX` attribute.
    /// [`VTTRegion.viewportAnchorX`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorX)
    pub fn viewport_anchor_x(&self) -> f64 {
        self.inner.get("viewportAnchorX").as_::<f64>()
    }

    /// Setter of the `viewportAnchorX` attribute.
    /// [`VTTRegion.viewportAnchorX`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorX)
    pub fn set_viewport_anchor_x(&mut self, value: f64) {
        self.inner.set("viewportAnchorX", value);
    }
}
impl VTTRegion {
    /// Getter of the `viewportAnchorY` attribute.
    /// [`VTTRegion.viewportAnchorY`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorY)
    pub fn viewport_anchor_y(&self) -> f64 {
        self.inner.get("viewportAnchorY").as_::<f64>()
    }

    /// Setter of the `viewportAnchorY` attribute.
    /// [`VTTRegion.viewportAnchorY`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorY)
    pub fn set_viewport_anchor_y(&mut self, value: f64) {
        self.inner.set("viewportAnchorY", value);
    }
}
impl VTTRegion {
    /// Getter of the `scroll` attribute.
    /// [`VTTRegion.scroll`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/scroll)
    pub fn scroll(&self) -> ScrollSetting {
        self.inner.get("scroll").as_::<ScrollSetting>()
    }

    /// Setter of the `scroll` attribute.
    /// [`VTTRegion.scroll`](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/scroll)
    pub fn set_scroll(&mut self, value: &ScrollSetting) {
        self.inner.set("scroll", value);
    }
}
