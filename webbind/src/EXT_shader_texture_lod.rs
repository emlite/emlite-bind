use super::*;

#[derive(Clone, Debug)]
pub struct EXT_shader_texture_lod {
    inner: emlite::Val,
}
impl FromVal for EXT_shader_texture_lod {
    fn from_val(v: &emlite::Val) -> Self {
        EXT_shader_texture_lod {
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
impl std::ops::Deref for EXT_shader_texture_lod {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EXT_shader_texture_lod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EXT_shader_texture_lod> for emlite::Val {
    fn from(s: EXT_shader_texture_lod) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
