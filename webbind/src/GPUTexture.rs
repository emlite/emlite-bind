use super::*;

#[derive(Clone, Debug)]
pub struct GPUTextureViewDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUTextureViewDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTextureViewDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUTextureViewDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUTextureViewDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUTextureViewDescriptor> for emlite::Val {
    fn from(s: GPUTextureViewDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUTextureViewDescriptor {
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    pub fn set_format(&mut self, value: GPUTextureFormat) {
        self.inner.set("format", value);
    }
}
impl GPUTextureViewDescriptor {
    pub fn dimension(&self) -> GPUTextureViewDimension {
        self.inner.get("dimension").as_::<GPUTextureViewDimension>()
    }

    pub fn set_dimension(&mut self, value: GPUTextureViewDimension) {
        self.inner.set("dimension", value);
    }
}
impl GPUTextureViewDescriptor {
    pub fn usage(&self) -> jsbind::Any {
        self.inner.get("usage").as_::<jsbind::Any>()
    }

    pub fn set_usage(&mut self, value: jsbind::Any) {
        self.inner.set("usage", value);
    }
}
impl GPUTextureViewDescriptor {
    pub fn aspect(&self) -> GPUTextureAspect {
        self.inner.get("aspect").as_::<GPUTextureAspect>()
    }

    pub fn set_aspect(&mut self, value: GPUTextureAspect) {
        self.inner.set("aspect", value);
    }
}
impl GPUTextureViewDescriptor {
    pub fn base_mip_level(&self) -> jsbind::Any {
        self.inner.get("baseMipLevel").as_::<jsbind::Any>()
    }

    pub fn set_base_mip_level(&mut self, value: jsbind::Any) {
        self.inner.set("baseMipLevel", value);
    }
}
impl GPUTextureViewDescriptor {
    pub fn mip_level_count(&self) -> jsbind::Any {
        self.inner.get("mipLevelCount").as_::<jsbind::Any>()
    }

    pub fn set_mip_level_count(&mut self, value: jsbind::Any) {
        self.inner.set("mipLevelCount", value);
    }
}
impl GPUTextureViewDescriptor {
    pub fn base_array_layer(&self) -> jsbind::Any {
        self.inner.get("baseArrayLayer").as_::<jsbind::Any>()
    }

    pub fn set_base_array_layer(&mut self, value: jsbind::Any) {
        self.inner.set("baseArrayLayer", value);
    }
}
impl GPUTextureViewDescriptor {
    pub fn array_layer_count(&self) -> jsbind::Any {
        self.inner.get("arrayLayerCount").as_::<jsbind::Any>()
    }

    pub fn set_array_layer_count(&mut self, value: jsbind::Any) {
        self.inner.set("arrayLayerCount", value);
    }
}
#[derive(Clone, Debug)]
pub struct GPUTexture {
    inner: emlite::Val,
}
impl FromVal for GPUTexture {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTexture {
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
impl std::ops::Deref for GPUTexture {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUTexture> for emlite::Val {
    fn from(s: GPUTexture) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUTexture {
    pub fn create_view0(&self) -> GPUTextureView {
        self.inner.call("createView", &[]).as_::<GPUTextureView>()
    }

    pub fn create_view1(&self, descriptor: GPUTextureViewDescriptor) -> GPUTextureView {
        self.inner
            .call("createView", &[descriptor.into()])
            .as_::<GPUTextureView>()
    }
}
impl GPUTexture {
    pub fn destroy(&self) -> jsbind::Undefined {
        self.inner.call("destroy", &[]).as_::<jsbind::Undefined>()
    }
}
impl GPUTexture {
    pub fn width(&self) -> jsbind::Any {
        self.inner.get("width").as_::<jsbind::Any>()
    }
}
impl GPUTexture {
    pub fn height(&self) -> jsbind::Any {
        self.inner.get("height").as_::<jsbind::Any>()
    }
}
impl GPUTexture {
    pub fn depth_or_array_layers(&self) -> jsbind::Any {
        self.inner.get("depthOrArrayLayers").as_::<jsbind::Any>()
    }
}
impl GPUTexture {
    pub fn mip_level_count(&self) -> jsbind::Any {
        self.inner.get("mipLevelCount").as_::<jsbind::Any>()
    }
}
impl GPUTexture {
    pub fn sample_count(&self) -> jsbind::Any {
        self.inner.get("sampleCount").as_::<jsbind::Any>()
    }
}
impl GPUTexture {
    pub fn dimension(&self) -> GPUTextureDimension {
        self.inner.get("dimension").as_::<GPUTextureDimension>()
    }
}
impl GPUTexture {
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }
}
impl GPUTexture {
    pub fn usage(&self) -> jsbind::Any {
        self.inner.get("usage").as_::<jsbind::Any>()
    }
}
impl GPUTexture {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
