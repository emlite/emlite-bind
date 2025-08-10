use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLayerInit {
    inner: Any,
}
impl FromVal for XRLayerInit {
    fn from_val(v: &Any) -> Self {
        XRLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRLayerInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRLayerInit> for Any {
    fn from(s: XRLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRLayerInit> for Any {
    fn from(s: &XRLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRLayerInit {
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    pub fn set_space(&mut self, value: &XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRLayerInit {
    pub fn color_format(&self) -> Any {
        self.inner.get("colorFormat").as_::<Any>()
    }

    pub fn set_color_format(&mut self, value: &Any) {
        self.inner.set("colorFormat", value);
    }
}
impl XRLayerInit {
    pub fn depth_format(&self) -> Any {
        self.inner.get("depthFormat").as_::<Any>()
    }

    pub fn set_depth_format(&mut self, value: &Any) {
        self.inner.set("depthFormat", value);
    }
}
impl XRLayerInit {
    pub fn mip_levels(&self) -> u32 {
        self.inner.get("mipLevels").as_::<u32>()
    }

    pub fn set_mip_levels(&mut self, value: u32) {
        self.inner.set("mipLevels", value);
    }
}
impl XRLayerInit {
    pub fn view_pixel_width(&self) -> u32 {
        self.inner.get("viewPixelWidth").as_::<u32>()
    }

    pub fn set_view_pixel_width(&mut self, value: u32) {
        self.inner.set("viewPixelWidth", value);
    }
}
impl XRLayerInit {
    pub fn view_pixel_height(&self) -> u32 {
        self.inner.get("viewPixelHeight").as_::<u32>()
    }

    pub fn set_view_pixel_height(&mut self, value: u32) {
        self.inner.set("viewPixelHeight", value);
    }
}
impl XRLayerInit {
    pub fn layout(&self) -> XRLayerLayout {
        self.inner.get("layout").as_::<XRLayerLayout>()
    }

    pub fn set_layout(&mut self, value: &XRLayerLayout) {
        self.inner.set("layout", value);
    }
}
impl XRLayerInit {
    pub fn is_static(&self) -> bool {
        self.inner.get("isStatic").as_::<bool>()
    }

    pub fn set_is_static(&mut self, value: bool) {
        self.inner.set("isStatic", value);
    }
}
impl XRLayerInit {
    pub fn clear_on_access(&self) -> bool {
        self.inner.get("clearOnAccess").as_::<bool>()
    }

    pub fn set_clear_on_access(&mut self, value: bool) {
        self.inner.set("clearOnAccess", value);
    }
}
