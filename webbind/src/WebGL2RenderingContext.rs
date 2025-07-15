use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGL2RenderingContext {
    inner: emlite::Val,
}
impl FromVal for WebGL2RenderingContext {
    fn from_val(v: &emlite::Val) -> Self {
        WebGL2RenderingContext {
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
impl core::ops::Deref for WebGL2RenderingContext {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGL2RenderingContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebGL2RenderingContext {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebGL2RenderingContext {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebGL2RenderingContext> for emlite::Val {
    fn from(s: WebGL2RenderingContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WebGL2RenderingContext> for emlite::Val {
    fn from(s: &WebGL2RenderingContext) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebGL2RenderingContext);

impl WebGL2RenderingContext {
    pub fn make_xr_compatible(&self) -> Promise {
        self.inner.call("makeXRCompatible", &[]).as_::<Promise>()
    }
}
impl WebGL2RenderingContext {
    pub fn copy_buffer_sub_data(
        &self,
        read_target: &Any,
        write_target: &Any,
        read_offset: &Any,
        write_offset: &Any,
        size: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "copyBufferSubData",
                &[
                    read_target.into(),
                    write_target.into(),
                    read_offset.into(),
                    write_offset.into(),
                    size.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_buffer_sub_data0(
        &self,
        target: &Any,
        src_byte_offset: &Any,
        dst_buffer: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "getBufferSubData",
                &[target.into(), src_byte_offset.into(), dst_buffer.into()],
            )
            .as_::<Undefined>()
    }

    pub fn get_buffer_sub_data1(
        &self,
        target: &Any,
        src_byte_offset: &Any,
        dst_buffer: &Any,
        dst_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "getBufferSubData",
                &[
                    target.into(),
                    src_byte_offset.into(),
                    dst_buffer.into(),
                    dst_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn get_buffer_sub_data2(
        &self,
        target: &Any,
        src_byte_offset: &Any,
        dst_buffer: &Any,
        dst_offset: u64,
        length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "getBufferSubData",
                &[
                    target.into(),
                    src_byte_offset.into(),
                    dst_buffer.into(),
                    dst_offset.into(),
                    length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn blit_framebuffer(
        &self,
        src_x0: &Any,
        src_y0: &Any,
        src_x1: &Any,
        src_y1: &Any,
        dst_x0: &Any,
        dst_y0: &Any,
        dst_x1: &Any,
        dst_y1: &Any,
        mask: &Any,
        filter: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "blitFramebuffer",
                &[
                    src_x0.into(),
                    src_y0.into(),
                    src_x1.into(),
                    src_y1.into(),
                    dst_x0.into(),
                    dst_y0.into(),
                    dst_x1.into(),
                    dst_y1.into(),
                    mask.into(),
                    filter.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn framebuffer_texture_layer(
        &self,
        target: &Any,
        attachment: &Any,
        texture: &WebGLTexture,
        level: &Any,
        layer: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "framebufferTextureLayer",
                &[
                    target.into(),
                    attachment.into(),
                    texture.into(),
                    level.into(),
                    layer.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn invalidate_framebuffer(&self, target: &Any, attachments: &Sequence<Any>) -> Undefined {
        self.inner
            .call(
                "invalidateFramebuffer",
                &[target.into(), attachments.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn invalidate_sub_framebuffer(
        &self,
        target: &Any,
        attachments: &Sequence<Any>,
        x: &Any,
        y: &Any,
        width: &Any,
        height: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "invalidateSubFramebuffer",
                &[
                    target.into(),
                    attachments.into(),
                    x.into(),
                    y.into(),
                    width.into(),
                    height.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn read_buffer(&self, src: &Any) -> Undefined {
        self.inner
            .call("readBuffer", &[src.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_internalformat_parameter(
        &self,
        target: &Any,
        internalformat: &Any,
        pname: &Any,
    ) -> Any {
        self.inner
            .call(
                "getInternalformatParameter",
                &[target.into(), internalformat.into(), pname.into()],
            )
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn renderbuffer_storage_multisample(
        &self,
        target: &Any,
        samples: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "renderbufferStorageMultisample",
                &[
                    target.into(),
                    samples.into(),
                    internalformat.into(),
                    width.into(),
                    height.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn tex_storage2_d(
        &self,
        target: &Any,
        levels: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "texStorage2D",
                &[
                    target.into(),
                    levels.into(),
                    internalformat.into(),
                    width.into(),
                    height.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn tex_storage3_d(
        &self,
        target: &Any,
        levels: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
        depth: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "texStorage3D",
                &[
                    target.into(),
                    levels.into(),
                    internalformat.into(),
                    width.into(),
                    height.into(),
                    depth.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn tex_image3_d(
        &self,
        target: &Any,
        level: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
        depth: &Any,
        border: &Any,
        format: &Any,
        type_: &Any,
        src_data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "texImage3D",
                &[
                    target.into(),
                    level.into(),
                    internalformat.into(),
                    width.into(),
                    height.into(),
                    depth.into(),
                    border.into(),
                    format.into(),
                    type_.into(),
                    src_data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn tex_sub_image3_d0(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        zoffset: &Any,
        width: &Any,
        height: &Any,
        depth: &Any,
        format: &Any,
        type_: &Any,
        src_data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "texSubImage3D",
                &[
                    target.into(),
                    level.into(),
                    xoffset.into(),
                    yoffset.into(),
                    zoffset.into(),
                    width.into(),
                    height.into(),
                    depth.into(),
                    format.into(),
                    type_.into(),
                    src_data.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn tex_sub_image3_d1(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        zoffset: &Any,
        width: &Any,
        height: &Any,
        depth: &Any,
        format: &Any,
        type_: &Any,
        src_data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "texSubImage3D",
                &[
                    target.into(),
                    level.into(),
                    xoffset.into(),
                    yoffset.into(),
                    zoffset.into(),
                    width.into(),
                    height.into(),
                    depth.into(),
                    format.into(),
                    type_.into(),
                    src_data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn copy_tex_sub_image3_d(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        zoffset: &Any,
        x: &Any,
        y: &Any,
        width: &Any,
        height: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "copyTexSubImage3D",
                &[
                    target.into(),
                    level.into(),
                    xoffset.into(),
                    yoffset.into(),
                    zoffset.into(),
                    x.into(),
                    y.into(),
                    width.into(),
                    height.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn compressed_tex_image3_d0(
        &self,
        target: &Any,
        level: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
        depth: &Any,
        border: &Any,
        src_data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "compressedTexImage3D",
                &[
                    target.into(),
                    level.into(),
                    internalformat.into(),
                    width.into(),
                    height.into(),
                    depth.into(),
                    border.into(),
                    src_data.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn compressed_tex_image3_d1(
        &self,
        target: &Any,
        level: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
        depth: &Any,
        border: &Any,
        src_data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "compressedTexImage3D",
                &[
                    target.into(),
                    level.into(),
                    internalformat.into(),
                    width.into(),
                    height.into(),
                    depth.into(),
                    border.into(),
                    src_data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn compressed_tex_image3_d2(
        &self,
        target: &Any,
        level: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
        depth: &Any,
        border: &Any,
        src_data: &Any,
        src_offset: u64,
        src_length_override: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "compressedTexImage3D",
                &[
                    target.into(),
                    level.into(),
                    internalformat.into(),
                    width.into(),
                    height.into(),
                    depth.into(),
                    border.into(),
                    src_data.into(),
                    src_offset.into(),
                    src_length_override.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn compressed_tex_sub_image3_d0(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        zoffset: &Any,
        width: &Any,
        height: &Any,
        depth: &Any,
        format: &Any,
        src_data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "compressedTexSubImage3D",
                &[
                    target.into(),
                    level.into(),
                    xoffset.into(),
                    yoffset.into(),
                    zoffset.into(),
                    width.into(),
                    height.into(),
                    depth.into(),
                    format.into(),
                    src_data.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn compressed_tex_sub_image3_d1(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        zoffset: &Any,
        width: &Any,
        height: &Any,
        depth: &Any,
        format: &Any,
        src_data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "compressedTexSubImage3D",
                &[
                    target.into(),
                    level.into(),
                    xoffset.into(),
                    yoffset.into(),
                    zoffset.into(),
                    width.into(),
                    height.into(),
                    depth.into(),
                    format.into(),
                    src_data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn compressed_tex_sub_image3_d2(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        zoffset: &Any,
        width: &Any,
        height: &Any,
        depth: &Any,
        format: &Any,
        src_data: &Any,
        src_offset: u64,
        src_length_override: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "compressedTexSubImage3D",
                &[
                    target.into(),
                    level.into(),
                    xoffset.into(),
                    yoffset.into(),
                    zoffset.into(),
                    width.into(),
                    height.into(),
                    depth.into(),
                    format.into(),
                    src_data.into(),
                    src_offset.into(),
                    src_length_override.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_frag_data_location(&self, program: &WebGLProgram, name: &str) -> Any {
        self.inner
            .call("getFragDataLocation", &[program.into(), name.into()])
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform1ui(&self, location: &WebGLUniformLocation, v0: &Any) -> Undefined {
        self.inner
            .call("uniform1ui", &[location.into(), v0.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform2ui(&self, location: &WebGLUniformLocation, v0: &Any, v1: &Any) -> Undefined {
        self.inner
            .call("uniform2ui", &[location.into(), v0.into(), v1.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform3ui(
        &self,
        location: &WebGLUniformLocation,
        v0: &Any,
        v1: &Any,
        v2: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform3ui",
                &[location.into(), v0.into(), v1.into(), v2.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform4ui(
        &self,
        location: &WebGLUniformLocation,
        v0: &Any,
        v1: &Any,
        v2: &Any,
        v3: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform4ui",
                &[location.into(), v0.into(), v1.into(), v2.into(), v3.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform1uiv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform1uiv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform1uiv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform1uiv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform1uiv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform1uiv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform2uiv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform2uiv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform2uiv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform2uiv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform2uiv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform2uiv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform3uiv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform3uiv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform3uiv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform3uiv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform3uiv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform3uiv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform4uiv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform4uiv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform4uiv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform4uiv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform4uiv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform4uiv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform_matrix3x2fv0(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix3x2fv",
                &[location.into(), transpose.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix3x2fv1(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix3x2fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix3x2fv2(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix3x2fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform_matrix4x2fv0(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix4x2fv",
                &[location.into(), transpose.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix4x2fv1(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix4x2fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix4x2fv2(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix4x2fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform_matrix2x3fv0(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix2x3fv",
                &[location.into(), transpose.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix2x3fv1(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix2x3fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix2x3fv2(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix2x3fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform_matrix4x3fv0(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix4x3fv",
                &[location.into(), transpose.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix4x3fv1(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix4x3fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix4x3fv2(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix4x3fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform_matrix2x4fv0(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix2x4fv",
                &[location.into(), transpose.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix2x4fv1(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix2x4fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix2x4fv2(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix2x4fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform_matrix3x4fv0(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix3x4fv",
                &[location.into(), transpose.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix3x4fv1(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix3x4fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix3x4fv2(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix3x4fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn vertex_attrib_i4i(&self, index: &Any, x: &Any, y: &Any, z: &Any, w: &Any) -> Undefined {
        self.inner
            .call(
                "vertexAttribI4i",
                &[index.into(), x.into(), y.into(), z.into(), w.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn vertex_attrib_i4iv(&self, index: &Any, values: &Any) -> Undefined {
        self.inner
            .call("vertexAttribI4iv", &[index.into(), values.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn vertex_attrib_i4ui(&self, index: &Any, x: &Any, y: &Any, z: &Any, w: &Any) -> Undefined {
        self.inner
            .call(
                "vertexAttribI4ui",
                &[index.into(), x.into(), y.into(), z.into(), w.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn vertex_attrib_i4uiv(&self, index: &Any, values: &Any) -> Undefined {
        self.inner
            .call("vertexAttribI4uiv", &[index.into(), values.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn vertex_attrib_i_pointer(
        &self,
        index: &Any,
        size: &Any,
        type_: &Any,
        stride: &Any,
        offset: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "vertexAttribIPointer",
                &[
                    index.into(),
                    size.into(),
                    type_.into(),
                    stride.into(),
                    offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn vertex_attrib_divisor(&self, index: &Any, divisor: &Any) -> Undefined {
        self.inner
            .call("vertexAttribDivisor", &[index.into(), divisor.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn draw_arrays_instanced(
        &self,
        mode: &Any,
        first: &Any,
        count: &Any,
        instance_count: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "drawArraysInstanced",
                &[
                    mode.into(),
                    first.into(),
                    count.into(),
                    instance_count.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn draw_elements_instanced(
        &self,
        mode: &Any,
        count: &Any,
        type_: &Any,
        offset: &Any,
        instance_count: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "drawElementsInstanced",
                &[
                    mode.into(),
                    count.into(),
                    type_.into(),
                    offset.into(),
                    instance_count.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn draw_range_elements(
        &self,
        mode: &Any,
        start: &Any,
        end: &Any,
        count: &Any,
        type_: &Any,
        offset: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "drawRangeElements",
                &[
                    mode.into(),
                    start.into(),
                    end.into(),
                    count.into(),
                    type_.into(),
                    offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn draw_buffers(&self, buffers: &Sequence<Any>) -> Undefined {
        self.inner
            .call("drawBuffers", &[buffers.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn clear_bufferfv0(&self, buffer: &Any, drawbuffer: &Any, values: &Any) -> Undefined {
        self.inner
            .call(
                "clearBufferfv",
                &[buffer.into(), drawbuffer.into(), values.into()],
            )
            .as_::<Undefined>()
    }

    pub fn clear_bufferfv1(
        &self,
        buffer: &Any,
        drawbuffer: &Any,
        values: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "clearBufferfv",
                &[
                    buffer.into(),
                    drawbuffer.into(),
                    values.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn clear_bufferiv0(&self, buffer: &Any, drawbuffer: &Any, values: &Any) -> Undefined {
        self.inner
            .call(
                "clearBufferiv",
                &[buffer.into(), drawbuffer.into(), values.into()],
            )
            .as_::<Undefined>()
    }

    pub fn clear_bufferiv1(
        &self,
        buffer: &Any,
        drawbuffer: &Any,
        values: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "clearBufferiv",
                &[
                    buffer.into(),
                    drawbuffer.into(),
                    values.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn clear_bufferuiv0(&self, buffer: &Any, drawbuffer: &Any, values: &Any) -> Undefined {
        self.inner
            .call(
                "clearBufferuiv",
                &[buffer.into(), drawbuffer.into(), values.into()],
            )
            .as_::<Undefined>()
    }

    pub fn clear_bufferuiv1(
        &self,
        buffer: &Any,
        drawbuffer: &Any,
        values: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "clearBufferuiv",
                &[
                    buffer.into(),
                    drawbuffer.into(),
                    values.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn clear_bufferfi(
        &self,
        buffer: &Any,
        drawbuffer: &Any,
        depth: &Any,
        stencil: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "clearBufferfi",
                &[
                    buffer.into(),
                    drawbuffer.into(),
                    depth.into(),
                    stencil.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn create_query(&self) -> WebGLQuery {
        self.inner.call("createQuery", &[]).as_::<WebGLQuery>()
    }
}
impl WebGL2RenderingContext {
    pub fn delete_query(&self, query: &WebGLQuery) -> Undefined {
        self.inner
            .call("deleteQuery", &[query.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn is_query(&self, query: &WebGLQuery) -> Any {
        self.inner.call("isQuery", &[query.into()]).as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn begin_query(&self, target: &Any, query: &WebGLQuery) -> Undefined {
        self.inner
            .call("beginQuery", &[target.into(), query.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn end_query(&self, target: &Any) -> Undefined {
        self.inner
            .call("endQuery", &[target.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_query(&self, target: &Any, pname: &Any) -> WebGLQuery {
        self.inner
            .call("getQuery", &[target.into(), pname.into()])
            .as_::<WebGLQuery>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_query_parameter(&self, query: &WebGLQuery, pname: &Any) -> Any {
        self.inner
            .call("getQueryParameter", &[query.into(), pname.into()])
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn create_sampler(&self) -> WebGLSampler {
        self.inner.call("createSampler", &[]).as_::<WebGLSampler>()
    }
}
impl WebGL2RenderingContext {
    pub fn delete_sampler(&self, sampler: &WebGLSampler) -> Undefined {
        self.inner
            .call("deleteSampler", &[sampler.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn is_sampler(&self, sampler: &WebGLSampler) -> Any {
        self.inner.call("isSampler", &[sampler.into()]).as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn bind_sampler(&self, unit: &Any, sampler: &WebGLSampler) -> Undefined {
        self.inner
            .call("bindSampler", &[unit.into(), sampler.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn sampler_parameteri(
        &self,
        sampler: &WebGLSampler,
        pname: &Any,
        param: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "samplerParameteri",
                &[sampler.into(), pname.into(), param.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn sampler_parameterf(
        &self,
        sampler: &WebGLSampler,
        pname: &Any,
        param: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "samplerParameterf",
                &[sampler.into(), pname.into(), param.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_sampler_parameter(&self, sampler: &WebGLSampler, pname: &Any) -> Any {
        self.inner
            .call("getSamplerParameter", &[sampler.into(), pname.into()])
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn fence_sync(&self, condition: &Any, flags: &Any) -> WebGLSync {
        self.inner
            .call("fenceSync", &[condition.into(), flags.into()])
            .as_::<WebGLSync>()
    }
}
impl WebGL2RenderingContext {
    pub fn is_sync(&self, sync: &WebGLSync) -> Any {
        self.inner.call("isSync", &[sync.into()]).as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn delete_sync(&self, sync: &WebGLSync) -> Undefined {
        self.inner
            .call("deleteSync", &[sync.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn client_wait_sync(&self, sync: &WebGLSync, flags: &Any, timeout: &Any) -> Any {
        self.inner
            .call(
                "clientWaitSync",
                &[sync.into(), flags.into(), timeout.into()],
            )
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn wait_sync(&self, sync: &WebGLSync, flags: &Any, timeout: &Any) -> Undefined {
        self.inner
            .call("waitSync", &[sync.into(), flags.into(), timeout.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_sync_parameter(&self, sync: &WebGLSync, pname: &Any) -> Any {
        self.inner
            .call("getSyncParameter", &[sync.into(), pname.into()])
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn create_transform_feedback(&self) -> WebGLTransformFeedback {
        self.inner
            .call("createTransformFeedback", &[])
            .as_::<WebGLTransformFeedback>()
    }
}
impl WebGL2RenderingContext {
    pub fn delete_transform_feedback(&self, tf: &WebGLTransformFeedback) -> Undefined {
        self.inner
            .call("deleteTransformFeedback", &[tf.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn is_transform_feedback(&self, tf: &WebGLTransformFeedback) -> Any {
        self.inner
            .call("isTransformFeedback", &[tf.into()])
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn bind_transform_feedback(&self, target: &Any, tf: &WebGLTransformFeedback) -> Undefined {
        self.inner
            .call("bindTransformFeedback", &[target.into(), tf.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn begin_transform_feedback(&self, primitive_mode: &Any) -> Undefined {
        self.inner
            .call("beginTransformFeedback", &[primitive_mode.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn end_transform_feedback(&self) -> Undefined {
        self.inner
            .call("endTransformFeedback", &[])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn transform_feedback_varyings(
        &self,
        program: &WebGLProgram,
        varyings: &Sequence<String>,
        buffer_mode: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "transformFeedbackVaryings",
                &[program.into(), varyings.into(), buffer_mode.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_transform_feedback_varying(
        &self,
        program: &WebGLProgram,
        index: &Any,
    ) -> WebGLActiveInfo {
        self.inner
            .call(
                "getTransformFeedbackVarying",
                &[program.into(), index.into()],
            )
            .as_::<WebGLActiveInfo>()
    }
}
impl WebGL2RenderingContext {
    pub fn pause_transform_feedback(&self) -> Undefined {
        self.inner
            .call("pauseTransformFeedback", &[])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn resume_transform_feedback(&self) -> Undefined {
        self.inner
            .call("resumeTransformFeedback", &[])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn bind_buffer_base(&self, target: &Any, index: &Any, buffer: &WebGLBuffer) -> Undefined {
        self.inner
            .call(
                "bindBufferBase",
                &[target.into(), index.into(), buffer.into()],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn bind_buffer_range(
        &self,
        target: &Any,
        index: &Any,
        buffer: &WebGLBuffer,
        offset: &Any,
        size: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "bindBufferRange",
                &[
                    target.into(),
                    index.into(),
                    buffer.into(),
                    offset.into(),
                    size.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_indexed_parameter(&self, target: &Any, index: &Any) -> Any {
        self.inner
            .call("getIndexedParameter", &[target.into(), index.into()])
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_uniform_indices(
        &self,
        program: &WebGLProgram,
        uniform_names: &Sequence<String>,
    ) -> Sequence<Any> {
        self.inner
            .call("getUniformIndices", &[program.into(), uniform_names.into()])
            .as_::<Sequence<Any>>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_active_uniforms(
        &self,
        program: &WebGLProgram,
        uniform_indices: &Sequence<Any>,
        pname: &Any,
    ) -> Any {
        self.inner
            .call(
                "getActiveUniforms",
                &[program.into(), uniform_indices.into(), pname.into()],
            )
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_uniform_block_index(&self, program: &WebGLProgram, uniform_block_name: &str) -> Any {
        self.inner
            .call(
                "getUniformBlockIndex",
                &[program.into(), uniform_block_name.into()],
            )
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_active_uniform_block_parameter(
        &self,
        program: &WebGLProgram,
        uniform_block_index: &Any,
        pname: &Any,
    ) -> Any {
        self.inner
            .call(
                "getActiveUniformBlockParameter",
                &[program.into(), uniform_block_index.into(), pname.into()],
            )
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn get_active_uniform_block_name(
        &self,
        program: &WebGLProgram,
        uniform_block_index: &Any,
    ) -> String {
        self.inner
            .call(
                "getActiveUniformBlockName",
                &[program.into(), uniform_block_index.into()],
            )
            .as_::<String>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform_block_binding(
        &self,
        program: &WebGLProgram,
        uniform_block_index: &Any,
        uniform_block_binding: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformBlockBinding",
                &[
                    program.into(),
                    uniform_block_index.into(),
                    uniform_block_binding.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn create_vertex_array(&self) -> WebGLVertexArrayObject {
        self.inner
            .call("createVertexArray", &[])
            .as_::<WebGLVertexArrayObject>()
    }
}
impl WebGL2RenderingContext {
    pub fn delete_vertex_array(&self, vertex_array: &WebGLVertexArrayObject) -> Undefined {
        self.inner
            .call("deleteVertexArray", &[vertex_array.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn is_vertex_array(&self, vertex_array: &WebGLVertexArrayObject) -> Any {
        self.inner
            .call("isVertexArray", &[vertex_array.into()])
            .as_::<Any>()
    }
}
impl WebGL2RenderingContext {
    pub fn bind_vertex_array(&self, array: &WebGLVertexArrayObject) -> Undefined {
        self.inner
            .call("bindVertexArray", &[array.into()])
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn buffer_data0(
        &self,
        target: &Any,
        src_data: &Any,
        usage: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "bufferData",
                &[
                    target.into(),
                    src_data.into(),
                    usage.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn buffer_data1(
        &self,
        target: &Any,
        src_data: &Any,
        usage: &Any,
        src_offset: u64,
        length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "bufferData",
                &[
                    target.into(),
                    src_data.into(),
                    usage.into(),
                    src_offset.into(),
                    length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn buffer_sub_data0(
        &self,
        target: &Any,
        dst_byte_offset: &Any,
        src_data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "bufferSubData",
                &[
                    target.into(),
                    dst_byte_offset.into(),
                    src_data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn buffer_sub_data1(
        &self,
        target: &Any,
        dst_byte_offset: &Any,
        src_data: &Any,
        src_offset: u64,
        length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "bufferSubData",
                &[
                    target.into(),
                    dst_byte_offset.into(),
                    src_data.into(),
                    src_offset.into(),
                    length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
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
        src_data: &Any,
        src_offset: u64,
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
                    src_data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
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
        src_data: &Any,
        src_offset: u64,
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
                    src_data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn compressed_tex_image2_d0(
        &self,
        target: &Any,
        level: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
        border: &Any,
        src_data: &Any,
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
                    src_data.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn compressed_tex_image2_d1(
        &self,
        target: &Any,
        level: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
        border: &Any,
        src_data: &Any,
        src_offset: u64,
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
                    src_data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn compressed_tex_image2_d2(
        &self,
        target: &Any,
        level: &Any,
        internalformat: &Any,
        width: &Any,
        height: &Any,
        border: &Any,
        src_data: &Any,
        src_offset: u64,
        src_length_override: &Any,
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
                    src_data.into(),
                    src_offset.into(),
                    src_length_override.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn compressed_tex_sub_image2_d0(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        width: &Any,
        height: &Any,
        format: &Any,
        src_data: &Any,
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
                    src_data.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn compressed_tex_sub_image2_d1(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        width: &Any,
        height: &Any,
        format: &Any,
        src_data: &Any,
        src_offset: u64,
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
                    src_data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn compressed_tex_sub_image2_d2(
        &self,
        target: &Any,
        level: &Any,
        xoffset: &Any,
        yoffset: &Any,
        width: &Any,
        height: &Any,
        format: &Any,
        src_data: &Any,
        src_offset: u64,
        src_length_override: &Any,
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
                    src_data.into(),
                    src_offset.into(),
                    src_length_override.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform1fv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform1fv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform1fv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform1fv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform1fv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform1fv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform2fv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform2fv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform2fv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform2fv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform2fv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform2fv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform3fv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform3fv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform3fv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform3fv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform3fv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform3fv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform4fv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform4fv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform4fv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform4fv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform4fv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform4fv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform1iv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform1iv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform1iv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform1iv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform1iv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform1iv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform2iv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform2iv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform2iv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform2iv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform2iv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform2iv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform3iv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform3iv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform3iv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform3iv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform3iv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform3iv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform4iv0(&self, location: &WebGLUniformLocation, data: &Any) -> Undefined {
        self.inner
            .call("uniform4iv", &[location.into(), data.into()])
            .as_::<Undefined>()
    }

    pub fn uniform4iv1(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniform4iv",
                &[location.into(), data.into(), src_offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform4iv2(
        &self,
        location: &WebGLUniformLocation,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniform4iv",
                &[
                    location.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform_matrix2fv0(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix2fv",
                &[location.into(), transpose.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix2fv1(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix2fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix2fv2(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix2fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform_matrix3fv0(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix3fv",
                &[location.into(), transpose.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix3fv1(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix3fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix3fv2(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix3fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn uniform_matrix4fv0(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix4fv",
                &[location.into(), transpose.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix4fv1(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix4fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn uniform_matrix4fv2(
        &self,
        location: &WebGLUniformLocation,
        transpose: &Any,
        data: &Any,
        src_offset: u64,
        src_length: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "uniformMatrix4fv",
                &[
                    location.into(),
                    transpose.into(),
                    data.into(),
                    src_offset.into(),
                    src_length.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WebGL2RenderingContext {
    pub fn read_pixels(
        &self,
        x: &Any,
        y: &Any,
        width: &Any,
        height: &Any,
        format: &Any,
        type_: &Any,
        dst_data: &Any,
        dst_offset: u64,
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
                    dst_data.into(),
                    dst_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
