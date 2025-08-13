use super::*;




/// The EXT_shader_texture_lod class.
/// [`EXT_shader_texture_lod`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_shader_texture_lod)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EXT_shader_texture_lod {
    inner: Any,
}

impl FromVal for EXT_shader_texture_lod {
    fn from_val(v: &Any) -> Self {
        EXT_shader_texture_lod { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EXT_shader_texture_lod {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EXT_shader_texture_lod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EXT_shader_texture_lod {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EXT_shader_texture_lod {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<EXT_shader_texture_lod> for Any {
    fn from(s: EXT_shader_texture_lod) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EXT_shader_texture_lod> for Any {
    fn from(s: &EXT_shader_texture_lod) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(EXT_shader_texture_lod);


