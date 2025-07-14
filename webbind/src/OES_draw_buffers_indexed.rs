use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OES_draw_buffers_indexed {
    inner: emlite::Val,
}
impl FromVal for OES_draw_buffers_indexed {
    fn from_val(v: &emlite::Val) -> Self {
        OES_draw_buffers_indexed {
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
impl core::ops::Deref for OES_draw_buffers_indexed {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OES_draw_buffers_indexed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OES_draw_buffers_indexed> for emlite::Val {
    fn from(s: OES_draw_buffers_indexed) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OES_draw_buffers_indexed {
    pub fn enablei_oes(&self, target: jsbind::Any, index: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("enableiOES", &[target.into(), index.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    pub fn disablei_oes(&self, target: jsbind::Any, index: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("disableiOES", &[target.into(), index.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    pub fn blend_equationi_oes(&self, buf: jsbind::Any, mode: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("blendEquationiOES", &[buf.into(), mode.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    pub fn blend_equation_separatei_oes(
        &self,
        buf: jsbind::Any,
        mode_rgb: jsbind::Any,
        mode_alpha: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "blendEquationSeparateiOES",
                &[buf.into(), mode_rgb.into(), mode_alpha.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    pub fn blend_funci_oes(
        &self,
        buf: jsbind::Any,
        src: jsbind::Any,
        dst: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("blendFunciOES", &[buf.into(), src.into(), dst.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    pub fn blend_func_separatei_oes(
        &self,
        buf: jsbind::Any,
        src_rgb: jsbind::Any,
        dst_rgb: jsbind::Any,
        src_alpha: jsbind::Any,
        dst_alpha: jsbind::Any,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }
}
impl OES_draw_buffers_indexed {
    pub fn color_maski_oes(
        &self,
        buf: jsbind::Any,
        r: jsbind::Any,
        g: jsbind::Any,
        b: jsbind::Any,
        a: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "colorMaskiOES",
                &[buf.into(), r.into(), g.into(), b.into(), a.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
