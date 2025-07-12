use super::*;

#[derive(Clone, Debug)]
pub struct EXT_texture_compression_rgtc {
    inner: emlite::Val,
}
impl FromVal for EXT_texture_compression_rgtc {
    fn from_val(v: &emlite::Val) -> Self {
        EXT_texture_compression_rgtc {
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
impl std::ops::Deref for EXT_texture_compression_rgtc {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EXT_texture_compression_rgtc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EXT_texture_compression_rgtc> for emlite::Val {
    fn from(s: EXT_texture_compression_rgtc) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
