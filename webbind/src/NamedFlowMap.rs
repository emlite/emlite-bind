use super::*;




/// The NamedFlowMap class.
/// [`NamedFlowMap`](https://developer.mozilla.org/en-US/docs/Web/API/NamedFlowMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NamedFlowMap {
    inner: Any,
}

impl FromVal for NamedFlowMap {
    fn from_val(v: &Any) -> Self {
        NamedFlowMap { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NamedFlowMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NamedFlowMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NamedFlowMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NamedFlowMap {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NamedFlowMap> for Any {
    fn from(s: NamedFlowMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NamedFlowMap> for Any {
    fn from(s: &NamedFlowMap) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NamedFlowMap);


