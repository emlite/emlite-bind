use super::*;

/// The OES_draw_buffers_indexed class.
/// [`OES_draw_buffers_indexed`](https://developer.mozilla.org/en-US/docs/Web/API/OES_draw_buffers_indexed)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OES_draw_buffers_indexed {
    inner: Any,
}
impl FromVal for OES_draw_buffers_indexed {
    fn from_val(v: &Any) -> Self {
        OES_draw_buffers_indexed {
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
impl core::ops::Deref for OES_draw_buffers_indexed {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OES_draw_buffers_indexed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OES_draw_buffers_indexed {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OES_draw_buffers_indexed {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OES_draw_buffers_indexed> for Any {
    fn from(s: OES_draw_buffers_indexed) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OES_draw_buffers_indexed> for Any {
    fn from(s: &OES_draw_buffers_indexed) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OES_draw_buffers_indexed);

impl OES_draw_buffers_indexed {
    /// The enableiOES method.
    /// [`OES_draw_buffers_indexed.enableiOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_draw_buffers_indexed/enableiOES)
    pub fn enablei_oes(&self, target: &Any, index: &Any) -> Undefined {
        self.inner
            .call("enableiOES", &[target.into(), index.into()])
            .as_::<Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    /// The disableiOES method.
    /// [`OES_draw_buffers_indexed.disableiOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_draw_buffers_indexed/disableiOES)
    pub fn disablei_oes(&self, target: &Any, index: &Any) -> Undefined {
        self.inner
            .call("disableiOES", &[target.into(), index.into()])
            .as_::<Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    /// The blendEquationiOES method.
    /// [`OES_draw_buffers_indexed.blendEquationiOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_draw_buffers_indexed/blendEquationiOES)
    pub fn blend_equationi_oes(&self, buf: &Any, mode: &Any) -> Undefined {
        self.inner
            .call("blendEquationiOES", &[buf.into(), mode.into()])
            .as_::<Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    /// The blendEquationSeparateiOES method.
    /// [`OES_draw_buffers_indexed.blendEquationSeparateiOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_draw_buffers_indexed/blendEquationSeparateiOES)
    pub fn blend_equation_separatei_oes(
        &self,
        buf: &Any,
        mode_rgb: &Any,
        mode_alpha: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "blendEquationSeparateiOES",
                &[buf.into(), mode_rgb.into(), mode_alpha.into()],
            )
            .as_::<Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    /// The blendFunciOES method.
    /// [`OES_draw_buffers_indexed.blendFunciOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_draw_buffers_indexed/blendFunciOES)
    pub fn blend_funci_oes(&self, buf: &Any, src: &Any, dst: &Any) -> Undefined {
        self.inner
            .call("blendFunciOES", &[buf.into(), src.into(), dst.into()])
            .as_::<Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    /// The blendFuncSeparateiOES method.
    /// [`OES_draw_buffers_indexed.blendFuncSeparateiOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_draw_buffers_indexed/blendFuncSeparateiOES)
    pub fn blend_func_separatei_oes(
        &self,
        buf: &Any,
        src_rgb: &Any,
        dst_rgb: &Any,
        src_alpha: &Any,
        dst_alpha: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "blendFuncSeparateiOES",
                &[
                    buf.into(),
                    src_rgb.into(),
                    dst_rgb.into(),
                    src_alpha.into(),
                    dst_alpha.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    /// The colorMaskiOES method.
    /// [`OES_draw_buffers_indexed.colorMaskiOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_draw_buffers_indexed/colorMaskiOES)
    pub fn color_maski_oes(&self, buf: &Any, r: &Any, g: &Any, b: &Any, a: &Any) -> Undefined {
        self.inner
            .call(
                "colorMaskiOES",
                &[buf.into(), r.into(), g.into(), b.into(), a.into()],
            )
            .as_::<Undefined>()
    }
}
