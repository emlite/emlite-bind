use super::*;




/// The KHR_parallel_shader_compile class.
/// [`KHR_parallel_shader_compile`](https://developer.mozilla.org/en-US/docs/Web/API/KHR_parallel_shader_compile)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KHR_parallel_shader_compile {
    inner: Any,
}

impl FromVal for KHR_parallel_shader_compile {
    fn from_val(v: &Any) -> Self {
        KHR_parallel_shader_compile { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for KHR_parallel_shader_compile {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KHR_parallel_shader_compile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KHR_parallel_shader_compile {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KHR_parallel_shader_compile {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<KHR_parallel_shader_compile> for Any {
    fn from(s: KHR_parallel_shader_compile) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KHR_parallel_shader_compile> for Any {
    fn from(s: &KHR_parallel_shader_compile) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(KHR_parallel_shader_compile);


