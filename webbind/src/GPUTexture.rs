use super::*;

/// The GPUTexture class.
/// [`GPUTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTexture {
    inner: Any,
}

impl FromVal for GPUTexture {
    fn from_val(v: &Any) -> Self {
        GPUTexture {
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

impl core::ops::Deref for GPUTexture {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUTexture {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUTexture {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUTexture> for Any {
    fn from(s: GPUTexture) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUTexture> for Any {
    fn from(s: &GPUTexture) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUTexture);

impl GPUTexture {
    /// The createView method.
    /// [`GPUTexture.createView`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/createView)
    pub fn create_view0(&self) -> GPUTextureView {
        self.inner.call("createView", &[]).as_::<GPUTextureView>()
    }
    /// The createView method.
    /// [`GPUTexture.createView`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/createView)
    pub fn create_view1(&self, descriptor: &GPUTextureViewDescriptor) -> GPUTextureView {
        self.inner
            .call("createView", &[descriptor.into()])
            .as_::<GPUTextureView>()
    }
}
impl GPUTexture {
    /// The destroy method.
    /// [`GPUTexture.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
impl GPUTexture {
    /// Getter of the `width` attribute.
    /// [`GPUTexture.width`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/width)
    pub fn width(&self) -> Any {
        self.inner.get("width").as_::<Any>()
    }
}
impl GPUTexture {
    /// Getter of the `height` attribute.
    /// [`GPUTexture.height`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/height)
    pub fn height(&self) -> Any {
        self.inner.get("height").as_::<Any>()
    }
}
impl GPUTexture {
    /// Getter of the `depthOrArrayLayers` attribute.
    /// [`GPUTexture.depthOrArrayLayers`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/depthOrArrayLayers)
    pub fn depth_or_array_layers(&self) -> Any {
        self.inner.get("depthOrArrayLayers").as_::<Any>()
    }
}
impl GPUTexture {
    /// Getter of the `mipLevelCount` attribute.
    /// [`GPUTexture.mipLevelCount`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/mipLevelCount)
    pub fn mip_level_count(&self) -> Any {
        self.inner.get("mipLevelCount").as_::<Any>()
    }
}
impl GPUTexture {
    /// Getter of the `sampleCount` attribute.
    /// [`GPUTexture.sampleCount`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/sampleCount)
    pub fn sample_count(&self) -> Any {
        self.inner.get("sampleCount").as_::<Any>()
    }
}
impl GPUTexture {
    /// Getter of the `dimension` attribute.
    /// [`GPUTexture.dimension`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/dimension)
    pub fn dimension(&self) -> GPUTextureDimension {
        self.inner.get("dimension").as_::<GPUTextureDimension>()
    }
}
impl GPUTexture {
    /// Getter of the `format` attribute.
    /// [`GPUTexture.format`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/format)
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }
}
impl GPUTexture {
    /// Getter of the `usage` attribute.
    /// [`GPUTexture.usage`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/usage)
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }
}
impl GPUTexture {
    /// Getter of the `label` attribute.
    /// [`GPUTexture.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUTexture.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
