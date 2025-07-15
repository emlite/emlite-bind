use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCanvasConfiguration {
    inner: emlite::Val,
}
impl FromVal for GPUCanvasConfiguration {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCanvasConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCanvasConfiguration {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCanvasConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUCanvasConfiguration {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUCanvasConfiguration {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<GPUCanvasConfiguration> for emlite::Val {
    fn from(s: GPUCanvasConfiguration) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUCanvasConfiguration {
    pub fn device(&self) -> GPUDevice {
        self.inner.get("device").as_::<GPUDevice>()
    }

    pub fn set_device(&mut self, value: GPUDevice) {
        self.inner.set("device", value);
    }

}
impl GPUCanvasConfiguration {
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    pub fn set_format(&mut self, value: GPUTextureFormat) {
        self.inner.set("format", value);
    }

}
impl GPUCanvasConfiguration {
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }

    pub fn set_usage(&mut self, value: Any) {
        self.inner.set("usage", value);
    }

}
impl GPUCanvasConfiguration {
    pub fn view_formats(&self) -> Sequence<GPUTextureFormat> {
        self.inner.get("viewFormats").as_::<Sequence<GPUTextureFormat>>()
    }

    pub fn set_view_formats(&mut self, value: Sequence<GPUTextureFormat>) {
        self.inner.set("viewFormats", value);
    }

}
impl GPUCanvasConfiguration {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    pub fn set_color_space(&mut self, value: PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }

}
impl GPUCanvasConfiguration {
    pub fn tone_mapping(&self) -> Any {
        self.inner.get("toneMapping").as_::<Any>()
    }

    pub fn set_tone_mapping(&mut self, value: Any) {
        self.inner.set("toneMapping", value);
    }

}
impl GPUCanvasConfiguration {
    pub fn alpha_mode(&self) -> GPUCanvasAlphaMode {
        self.inner.get("alphaMode").as_::<GPUCanvasAlphaMode>()
    }

    pub fn set_alpha_mode(&mut self, value: GPUCanvasAlphaMode) {
        self.inner.set("alphaMode", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCanvasContext {
    inner: emlite::Val,
}
impl FromVal for GPUCanvasContext {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCanvasContext { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCanvasContext {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCanvasContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUCanvasContext {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUCanvasContext {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<GPUCanvasContext> for emlite::Val {
    fn from(s: GPUCanvasContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GPUCanvasContext);


impl GPUCanvasContext {
    pub fn canvas(&self) -> Any {
        self.inner.get("canvas").as_::<Any>()
    }

}
impl GPUCanvasContext {
    pub fn configure(&self, configuration: GPUCanvasConfiguration) -> Undefined {
        self.inner.call("configure", &[configuration.into(), ]).as_::<Undefined>()
    }

}
impl GPUCanvasContext {
    pub fn unconfigure(&self, ) -> Undefined {
        self.inner.call("unconfigure", &[]).as_::<Undefined>()
    }

}
impl GPUCanvasContext {
    pub fn get_configuration(&self, ) -> GPUCanvasConfiguration {
        self.inner.call("getConfiguration", &[]).as_::<GPUCanvasConfiguration>()
    }

}
impl GPUCanvasContext {
    pub fn get_current_texture(&self, ) -> GPUTexture {
        self.inner.call("getCurrentTexture", &[]).as_::<GPUTexture>()
    }

}
