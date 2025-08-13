use super::*;




/// The SVGUseElementShadowRoot class.
/// [`SVGUseElementShadowRoot`](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElementShadowRoot)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGUseElementShadowRoot {
    inner: ShadowRoot,
}

impl FromVal for SVGUseElementShadowRoot {
    fn from_val(v: &Any) -> Self {
        SVGUseElementShadowRoot { inner: ShadowRoot::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGUseElementShadowRoot {
    type Target = ShadowRoot;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGUseElementShadowRoot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGUseElementShadowRoot {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGUseElementShadowRoot {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGUseElementShadowRoot> for Any {
    fn from(s: SVGUseElementShadowRoot) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGUseElementShadowRoot> for Any {
    fn from(s: &SVGUseElementShadowRoot) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGUseElementShadowRoot);


