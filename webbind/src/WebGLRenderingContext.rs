use super::*;

/// The WebGLRenderingContext class.
/// [`WebGLRenderingContext`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLRenderingContext {
    inner: Any,
}

impl FromVal for WebGLRenderingContext {
    fn from_val(v: &Any) -> Self {
        WebGLRenderingContext {
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

impl core::ops::Deref for WebGLRenderingContext {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLRenderingContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLRenderingContext {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLRenderingContext {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebGLRenderingContext> for Any {
    fn from(s: WebGLRenderingContext) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLRenderingContext> for Any {
    fn from(s: &WebGLRenderingContext) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLRenderingContext);

impl WebGLRenderingContext {
    /// The makeXRCompatible method.
    /// [`WebGLRenderingContext.makeXRCompatible`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/makeXRCompatible)
    pub fn make_xr_compatible(&self) -> Promise<Undefined> {
        self.inner
            .call("makeXRCompatible", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl WebGLRenderingContext {
    /// The bufferData method.
    /// [`WebGLRenderingContext.bufferData`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)
    pub fn buffer_data(&self, target: &Any, size: &Any, usage: &Any) -> Undefined {
        self.inner
            .call("bufferData", &[target.into(), size.into(), usage.into()])
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The bufferData method.
    /// [`WebGLRenderingContext.bufferData`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)
    pub fn buffer_data_with_target_and_data_and_usage(
        &self,
        target: &Any,
        data: &Any,
        usage: &Any,
    ) -> Undefined {
        self.inner
            .call("bufferData", &[target.into(), data.into(), usage.into()])
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The bufferSubData method.
    /// [`WebGLRenderingContext.bufferSubData`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)
    pub fn buffer_sub_data(&self, target: &Any, offset: &Any, data: &Any) -> Undefined {
        self.inner
            .call(
                "bufferSubData",
                &[target.into(), offset.into(), data.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The compressedTexImage2D method.
    /// [`WebGLRenderingContext.compressedTexImage2D`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexImage2D)
    pub fn compressed_tex_image2_d(
        &self,
        target: &Any,
        level: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
        border: &Any,
        data: &Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The compressedTexSubImage2D method.
    /// [`WebGLRenderingContext.compressedTexSubImage2D`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexSubImage2D)
    pub fn compressed_tex_sub_image2_d(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        width: &Any,
        height: &Any,
        format: &Any,
        data: &Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The readPixels method.
    /// [`WebGLRenderingContext.readPixels`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels)
    pub fn read_pixels(
        &self,
        x: &Any,
        y: &Any,
        width: &Any,
        height: &Any,
        format: &Any,
        type_: &Any,
        pixels: &Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The texImage2D method.
    /// [`WebGLRenderingContext.texImage2D`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)
    pub fn tex_image2_d(
        &self,
        target: &Any,
        level: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
        border: &Any,
        format: &Any,
        type_: &Any,
        pixels: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "texImage2D",
                &[
                    target.into(),
                    level.into(),
                    internalformat.into(),
                    width.into(),
                    height.into(),
                    border.into(),
                    format.into(),
                    type_.into(),
                    pixels.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The texImage2D method.
    /// [`WebGLRenderingContext.texImage2D`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)
    pub fn tex_image2_d_with_target_and_level_and_internalformat_and_format_and_type_and_source(
        &self,
        target: &Any,
        level: &Any,
        internalformat: &Any,
        format: &Any,
        type_: &Any,
        source: &Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The texSubImage2D method.
    /// [`WebGLRenderingContext.texSubImage2D`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)
    pub fn tex_sub_image2_d(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        width: &Any,
        height: &Any,
        format: &Any,
        type_: &Any,
        pixels: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "texSubImage2D",
                &[
                    target.into(),
                    level.into(),
                    xoffset.into(),
                    yoffset.into(),
                    width.into(),
                    height.into(),
                    format.into(),
                    type_.into(),
                    pixels.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The texSubImage2D method.
    /// [`WebGLRenderingContext.texSubImage2D`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)
    pub fn tex_sub_image2_d_with_target_and_level_and_xoffset_and_yoffset_and_format_and_type_and_source(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        format: &Any,
        type_: &Any,
        source: &Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniform1fv method.
    /// [`WebGLRenderingContext.uniform1fv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1fv)
    pub fn uniform1fv(&self, location: &WebGLUniformLocation, v: &Any) -> Undefined {
        self.inner
            .call("uniform1fv", &[location.into(), v.into()])
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniform2fv method.
    /// [`WebGLRenderingContext.uniform2fv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2fv)
    pub fn uniform2fv(&self, location: &WebGLUniformLocation, v: &Any) -> Undefined {
        self.inner
            .call("uniform2fv", &[location.into(), v.into()])
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniform3fv method.
    /// [`WebGLRenderingContext.uniform3fv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3fv)
    pub fn uniform3fv(&self, location: &WebGLUniformLocation, v: &Any) -> Undefined {
        self.inner
            .call("uniform3fv", &[location.into(), v.into()])
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniform4fv method.
    /// [`WebGLRenderingContext.uniform4fv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4fv)
    pub fn uniform4fv(&self, location: &WebGLUniformLocation, v: &Any) -> Undefined {
        self.inner
            .call("uniform4fv", &[location.into(), v.into()])
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniform1iv method.
    /// [`WebGLRenderingContext.uniform1iv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1iv)
    pub fn uniform1iv(&self, location: &WebGLUniformLocation, v: &Any) -> Undefined {
        self.inner
            .call("uniform1iv", &[location.into(), v.into()])
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniform2iv method.
    /// [`WebGLRenderingContext.uniform2iv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2iv)
    pub fn uniform2iv(&self, location: &WebGLUniformLocation, v: &Any) -> Undefined {
        self.inner
            .call("uniform2iv", &[location.into(), v.into()])
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniform3iv method.
    /// [`WebGLRenderingContext.uniform3iv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3iv)
    pub fn uniform3iv(&self, location: &WebGLUniformLocation, v: &Any) -> Undefined {
        self.inner
            .call("uniform3iv", &[location.into(), v.into()])
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniform4iv method.
    /// [`WebGLRenderingContext.uniform4iv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4iv)
    pub fn uniform4iv(&self, location: &WebGLUniformLocation, v: &Any) -> Undefined {
        self.inner
            .call("uniform4iv", &[location.into(), v.into()])
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniformMatrix2fv method.
    /// [`WebGLRenderingContext.uniformMatrix2fv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix2fv)
    pub fn uniform_matrix2fv(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        value: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix2fv",
                &[location.into(), transpose.into(), value.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniformMatrix3fv method.
    /// [`WebGLRenderingContext.uniformMatrix3fv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix3fv)
    pub fn uniform_matrix3fv(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        value: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix3fv",
                &[location.into(), transpose.into(), value.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGLRenderingContext {
    /// The uniformMatrix4fv method.
    /// [`WebGLRenderingContext.uniformMatrix4fv`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix4fv)
    pub fn uniform_matrix4fv(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        value: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix4fv",
                &[location.into(), transpose.into(), value.into()],
            )
            .as_::<Undefined>()
    }
}
