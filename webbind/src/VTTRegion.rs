use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VTTRegion {
    inner: emlite::Val,
}
impl FromVal for VTTRegion {
    fn from_val(v: &emlite::Val) -> Self {
        VTTRegion {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VTTRegion {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VTTRegion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VTTRegion> for emlite::Val {
    fn from(s: VTTRegion) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VTTRegion {
    pub fn new() -> VTTRegion {
        Self {
            inner: emlite::Val::global("VTTRegion")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl VTTRegion {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }

    pub fn set_id(&mut self, value: jsbind::DOMString) {
        self.inner.set("id", value);
    }
}
impl VTTRegion {
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }

    pub fn set_width(&mut self, value: f64) {
        self.inner.set("width", value);
    }
}
impl VTTRegion {
    pub fn lines(&self) -> u32 {
        self.inner.get("lines").as_::<u32>()
    }

    pub fn set_lines(&mut self, value: u32) {
        self.inner.set("lines", value);
    }
}
impl VTTRegion {
    pub fn region_anchor_x(&self) -> f64 {
        self.inner.get("regionAnchorX").as_::<f64>()
    }

    pub fn set_region_anchor_x(&mut self, value: f64) {
        self.inner.set("regionAnchorX", value);
    }
}
impl VTTRegion {
    pub fn region_anchor_y(&self) -> f64 {
        self.inner.get("regionAnchorY").as_::<f64>()
    }

    pub fn set_region_anchor_y(&mut self, value: f64) {
        self.inner.set("regionAnchorY", value);
    }
}
impl VTTRegion {
    pub fn viewport_anchor_x(&self) -> f64 {
        self.inner.get("viewportAnchorX").as_::<f64>()
    }

    pub fn set_viewport_anchor_x(&mut self, value: f64) {
        self.inner.set("viewportAnchorX", value);
    }
}
impl VTTRegion {
    pub fn viewport_anchor_y(&self) -> f64 {
        self.inner.get("viewportAnchorY").as_::<f64>()
    }

    pub fn set_viewport_anchor_y(&mut self, value: f64) {
        self.inner.set("viewportAnchorY", value);
    }
}
impl VTTRegion {
    pub fn scroll(&self) -> ScrollSetting {
        self.inner.get("scroll").as_::<ScrollSetting>()
    }

    pub fn set_scroll(&mut self, value: ScrollSetting) {
        self.inner.set("scroll", value);
    }
}
