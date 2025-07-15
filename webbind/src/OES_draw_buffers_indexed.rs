use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OES_draw_buffers_indexed {
    inner: emlite::Val,
}
impl FromVal for OES_draw_buffers_indexed {
    fn from_val(v: &emlite::Val) -> Self {
        OES_draw_buffers_indexed { inner: emlite::Val::from_val(v) }
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
impl AsRef<emlite::Val> for OES_draw_buffers_indexed {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OES_draw_buffers_indexed {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(OES_draw_buffers_indexed);


impl OES_draw_buffers_indexed {
    pub fn enablei_oes(&self, target: Any, index: Any) -> Undefined {
        self.inner.call("enableiOES", &[target.into(), index.into(), ]).as_::<Undefined>()
    }

}
impl OES_draw_buffers_indexed {
    pub fn disablei_oes(&self, target: Any, index: Any) -> Undefined {
        self.inner.call("disableiOES", &[target.into(), index.into(), ]).as_::<Undefined>()
    }

}
impl OES_draw_buffers_indexed {
    pub fn blend_equationi_oes(&self, buf: Any, mode: Any) -> Undefined {
        self.inner.call("blendEquationiOES", &[buf.into(), mode.into(), ]).as_::<Undefined>()
    }

}
impl OES_draw_buffers_indexed {
    pub fn blend_equation_separatei_oes(&self, buf: Any, mode_rgb: Any, mode_alpha: Any) -> Undefined {
        self.inner.call("blendEquationSeparateiOES", &[buf.into(), mode_rgb.into(), mode_alpha.into(), ]).as_::<Undefined>()
    }

}
impl OES_draw_buffers_indexed {
    pub fn blend_funci_oes(&self, buf: Any, src: Any, dst: Any) -> Undefined {
        self.inner.call("blendFunciOES", &[buf.into(), src.into(), dst.into(), ]).as_::<Undefined>()
    }

}
impl OES_draw_buffers_indexed {
    pub fn blend_func_separatei_oes(&self, buf: Any, src_rgb: Any, dst_rgb: Any, src_alpha: Any, dst_alpha: Any) -> Undefined {
        self.inner.call("blendFuncSeparateiOES", &[buf.into(), src_rgb.into(), dst_rgb.into(), src_alpha.into(), dst_alpha.into(), ]).as_::<Undefined>()
    }

}
impl OES_draw_buffers_indexed {
    pub fn color_maski_oes(&self, buf: Any, r: Any, g: Any, b: Any, a: Any) -> Undefined {
        self.inner.call("colorMaskiOES", &[buf.into(), r.into(), g.into(), b.into(), a.into(), ]).as_::<Undefined>()
    }

}
