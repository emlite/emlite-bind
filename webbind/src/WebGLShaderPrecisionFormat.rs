use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLShaderPrecisionFormat {
    inner: emlite::Val,
}
impl FromVal for WebGLShaderPrecisionFormat {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLShaderPrecisionFormat { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebGLShaderPrecisionFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLShaderPrecisionFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebGLShaderPrecisionFormat {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebGLShaderPrecisionFormat {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<WebGLShaderPrecisionFormat> for emlite::Val {
    fn from(s: WebGLShaderPrecisionFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WebGLShaderPrecisionFormat);


impl WebGLShaderPrecisionFormat {
    pub fn range_min(&self) -> Any {
        self.inner.get("rangeMin").as_::<Any>()
    }

}
impl WebGLShaderPrecisionFormat {
    pub fn range_max(&self) -> Any {
        self.inner.get("rangeMax").as_::<Any>()
    }

}
impl WebGLShaderPrecisionFormat {
    pub fn precision(&self) -> Any {
        self.inner.get("precision").as_::<Any>()
    }

}
