use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRCompositionLayer {
    inner: XRLayer,
}
impl FromVal for XRCompositionLayer {
    fn from_val(v: &emlite::Val) -> Self {
        XRCompositionLayer {
            inner: XRLayer::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRCompositionLayer {
    type Target = XRLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRCompositionLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRCompositionLayer> for emlite::Val {
    fn from(s: XRCompositionLayer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRCompositionLayer {
    pub fn layout(&self) -> XRLayerLayout {
        self.inner.get("layout").as_::<XRLayerLayout>()
    }
}
impl XRCompositionLayer {
    pub fn blend_texture_source_alpha(&self) -> bool {
        self.inner.get("blendTextureSourceAlpha").as_::<bool>()
    }

    pub fn set_blend_texture_source_alpha(&mut self, value: bool) {
        self.inner.set("blendTextureSourceAlpha", value);
    }
}
impl XRCompositionLayer {
    pub fn force_mono_presentation(&self) -> bool {
        self.inner.get("forceMonoPresentation").as_::<bool>()
    }

    pub fn set_force_mono_presentation(&mut self, value: bool) {
        self.inner.set("forceMonoPresentation", value);
    }
}
impl XRCompositionLayer {
    pub fn opacity(&self) -> f32 {
        self.inner.get("opacity").as_::<f32>()
    }

    pub fn set_opacity(&mut self, value: f32) {
        self.inner.set("opacity", value);
    }
}
impl XRCompositionLayer {
    pub fn mip_levels(&self) -> u32 {
        self.inner.get("mipLevels").as_::<u32>()
    }
}
impl XRCompositionLayer {
    pub fn quality(&self) -> XRLayerQuality {
        self.inner.get("quality").as_::<XRLayerQuality>()
    }

    pub fn set_quality(&mut self, value: XRLayerQuality) {
        self.inner.set("quality", value);
    }
}
impl XRCompositionLayer {
    pub fn needs_redraw(&self) -> bool {
        self.inner.get("needsRedraw").as_::<bool>()
    }
}
impl XRCompositionLayer {
    pub fn destroy(&self) -> jsbind::Undefined {
        self.inner.call("destroy", &[]).as_::<jsbind::Undefined>()
    }
}
