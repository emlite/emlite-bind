use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLRenderingContext {
    inner: emlite::Val,
}
impl FromVal for WebGLRenderingContext {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLRenderingContext {
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
impl core::ops::Deref for WebGLRenderingContext {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLRenderingContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebGLRenderingContext {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebGLRenderingContext {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebGLRenderingContext> for emlite::Val {
    fn from(s: WebGLRenderingContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WebGLRenderingContext);

impl WebGLRenderingContext {
    pub fn make_xr_compatible(&self) -> jsbind::Promise {
        self.inner
            .call("makeXRCompatible", &[])
            .as_::<jsbind::Promise>()
    }
}
impl WebGLRenderingContext {
    pub fn buffer_data(
        &self,
        target: jsbind::Any,
        data: jsbind::Any,
        usage: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("bufferData", &[target.into(), data.into(), usage.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn buffer_sub_data(
        &self,
        target: jsbind::Any,
        offset: jsbind::Any,
        data: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "bufferSubData",
                &[target.into(), offset.into(), data.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn compressed_tex_image2_d(
        &self,
        target: jsbind::Any,
        level: jsbind::Any,
        internalformat: jsbind::Any,
        width: jsbind::Any,
        height: jsbind::Any,
        border: jsbind::Any,
        data: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "compressedTexImage2D",
                &[
                    target.into(),
                    level.into(),
                    internalformat.into(),
                    width.into(),
                    height.into(),
                    border.into(),
                    data.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn compressed_tex_sub_image2_d(
        &self,
        target: jsbind::Any,
        level: jsbind::Any,
        xoffset: jsbind::Any,
        yoffset: jsbind::Any,
        width: jsbind::Any,
        height: jsbind::Any,
        format: jsbind::Any,
        data: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "compressedTexSubImage2D",
                &[
                    target.into(),
                    level.into(),
                    xoffset.into(),
                    yoffset.into(),
                    width.into(),
                    height.into(),
                    format.into(),
                    data.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn read_pixels(
        &self,
        x: jsbind::Any,
        y: jsbind::Any,
        width: jsbind::Any,
        height: jsbind::Any,
        format: jsbind::Any,
        type_: jsbind::Any,
        pixels: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "readPixels",
                &[
                    x.into(),
                    y.into(),
                    width.into(),
                    height.into(),
                    format.into(),
                    type_.into(),
                    pixels.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn tex_image2_d(
        &self,
        target: jsbind::Any,
        level: jsbind::Any,
        internalformat: jsbind::Any,
        format: jsbind::Any,
        type_: jsbind::Any,
        source: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "texImage2D",
                &[
                    target.into(),
                    level.into(),
                    internalformat.into(),
                    format.into(),
                    type_.into(),
                    source.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn tex_sub_image2_d(
        &self,
        target: jsbind::Any,
        level: jsbind::Any,
        xoffset: jsbind::Any,
        yoffset: jsbind::Any,
        format: jsbind::Any,
        type_: jsbind::Any,
        source: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "texSubImage2D",
                &[
                    target.into(),
                    level.into(),
                    xoffset.into(),
                    yoffset.into(),
                    format.into(),
                    type_.into(),
                    source.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform1fv(&self, location: WebGLUniformLocation, v: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("uniform1fv", &[location.into(), v.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform2fv(&self, location: WebGLUniformLocation, v: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("uniform2fv", &[location.into(), v.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform3fv(&self, location: WebGLUniformLocation, v: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("uniform3fv", &[location.into(), v.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform4fv(&self, location: WebGLUniformLocation, v: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("uniform4fv", &[location.into(), v.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform1iv(&self, location: WebGLUniformLocation, v: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("uniform1iv", &[location.into(), v.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform2iv(&self, location: WebGLUniformLocation, v: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("uniform2iv", &[location.into(), v.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform3iv(&self, location: WebGLUniformLocation, v: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("uniform3iv", &[location.into(), v.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform4iv(&self, location: WebGLUniformLocation, v: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("uniform4iv", &[location.into(), v.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform_matrix2fv(
        &self,
        location: WebGLUniformLocation,
        transpose: jsbind::Any,
        value: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "uniformMatrix2fv",
                &[location.into(), transpose.into(), value.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform_matrix3fv(
        &self,
        location: WebGLUniformLocation,
        transpose: jsbind::Any,
        value: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "uniformMatrix3fv",
                &[location.into(), transpose.into(), value.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl WebGLRenderingContext {
    pub fn uniform_matrix4fv(
        &self,
        location: WebGLUniformLocation,
        transpose: jsbind::Any,
        value: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "uniformMatrix4fv",
                &[location.into(), transpose.into(), value.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
