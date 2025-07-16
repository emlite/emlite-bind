use super::*;

/// The XRCompositionLayer class.
/// [`XRCompositionLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRCompositionLayer {
    inner: XRLayer,
}
impl FromVal for XRCompositionLayer {
    fn from_val(v: &Any) -> Self {
        XRCompositionLayer {
            inner: XRLayer::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for XRCompositionLayer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRCompositionLayer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRCompositionLayer> for Any {
    fn from(s: XRCompositionLayer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRCompositionLayer> for Any {
    fn from(s: &XRCompositionLayer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRCompositionLayer);

impl XRCompositionLayer {
    /// Getter of the `layout` attribute.
    /// [`XRCompositionLayer.layout`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/layout)
    pub fn layout(&self) -> XRLayerLayout {
        self.inner.get("layout").as_::<XRLayerLayout>()
    }
}
impl XRCompositionLayer {
    /// Getter of the `blendTextureSourceAlpha` attribute.
    /// [`XRCompositionLayer.blendTextureSourceAlpha`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/blendTextureSourceAlpha)
    pub fn blend_texture_source_alpha(&self) -> bool {
        self.inner.get("blendTextureSourceAlpha").as_::<bool>()
    }

    /// Setter of the `blendTextureSourceAlpha` attribute.
    /// [`XRCompositionLayer.blendTextureSourceAlpha`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/blendTextureSourceAlpha)
    pub fn set_blend_texture_source_alpha(&mut self, value: bool) {
        self.inner.set("blendTextureSourceAlpha", value);
    }
}
impl XRCompositionLayer {
    /// Getter of the `forceMonoPresentation` attribute.
    /// [`XRCompositionLayer.forceMonoPresentation`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/forceMonoPresentation)
    pub fn force_mono_presentation(&self) -> bool {
        self.inner.get("forceMonoPresentation").as_::<bool>()
    }

    /// Setter of the `forceMonoPresentation` attribute.
    /// [`XRCompositionLayer.forceMonoPresentation`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/forceMonoPresentation)
    pub fn set_force_mono_presentation(&mut self, value: bool) {
        self.inner.set("forceMonoPresentation", value);
    }
}
impl XRCompositionLayer {
    /// Getter of the `opacity` attribute.
    /// [`XRCompositionLayer.opacity`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/opacity)
    pub fn opacity(&self) -> f32 {
        self.inner.get("opacity").as_::<f32>()
    }

    /// Setter of the `opacity` attribute.
    /// [`XRCompositionLayer.opacity`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/opacity)
    pub fn set_opacity(&mut self, value: f32) {
        self.inner.set("opacity", value);
    }
}
impl XRCompositionLayer {
    /// Getter of the `mipLevels` attribute.
    /// [`XRCompositionLayer.mipLevels`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/mipLevels)
    pub fn mip_levels(&self) -> u32 {
        self.inner.get("mipLevels").as_::<u32>()
    }
}
impl XRCompositionLayer {
    /// Getter of the `quality` attribute.
    /// [`XRCompositionLayer.quality`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/quality)
    pub fn quality(&self) -> XRLayerQuality {
        self.inner.get("quality").as_::<XRLayerQuality>()
    }

    /// Setter of the `quality` attribute.
    /// [`XRCompositionLayer.quality`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/quality)
    pub fn set_quality(&mut self, value: &XRLayerQuality) {
        self.inner.set("quality", value);
    }
}
impl XRCompositionLayer {
    /// Getter of the `needsRedraw` attribute.
    /// [`XRCompositionLayer.needsRedraw`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/needsRedraw)
    pub fn needs_redraw(&self) -> bool {
        self.inner.get("needsRedraw").as_::<bool>()
    }
}
impl XRCompositionLayer {
    /// The destroy method.
    /// [`XRCompositionLayer.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/XRCompositionLayer/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
