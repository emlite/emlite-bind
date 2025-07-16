use super::*;

/// The WebGLShaderPrecisionFormat class.
/// [`WebGLShaderPrecisionFormat`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLShaderPrecisionFormat {
    inner: Any,
}
impl FromVal for WebGLShaderPrecisionFormat {
    fn from_val(v: &Any) -> Self {
        WebGLShaderPrecisionFormat {
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
impl core::ops::Deref for WebGLShaderPrecisionFormat {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLShaderPrecisionFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebGLShaderPrecisionFormat {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebGLShaderPrecisionFormat {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebGLShaderPrecisionFormat> for Any {
    fn from(s: WebGLShaderPrecisionFormat) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebGLShaderPrecisionFormat> for Any {
    fn from(s: &WebGLShaderPrecisionFormat) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebGLShaderPrecisionFormat);

impl WebGLShaderPrecisionFormat {
    /// Getter of the `rangeMin` attribute.
    /// [`WebGLShaderPrecisionFormat.rangeMin`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMin)
    pub fn range_min(&self) -> Any {
        self.inner.get("rangeMin").as_::<Any>()
    }
}
impl WebGLShaderPrecisionFormat {
    /// Getter of the `rangeMax` attribute.
    /// [`WebGLShaderPrecisionFormat.rangeMax`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMax)
    pub fn range_max(&self) -> Any {
        self.inner.get("rangeMax").as_::<Any>()
    }
}
impl WebGLShaderPrecisionFormat {
    /// Getter of the `precision` attribute.
    /// [`WebGLShaderPrecisionFormat.precision`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/precision)
    pub fn precision(&self) -> Any {
        self.inner.get("precision").as_::<Any>()
    }
}
