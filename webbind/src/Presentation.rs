use super::*;




/// The Presentation class.
/// [`Presentation`](https://developer.mozilla.org/en-US/docs/Web/API/Presentation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Presentation {
    inner: Any,
}

impl FromVal for Presentation {
    fn from_val(v: &Any) -> Self {
        Presentation { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Presentation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Presentation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Presentation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Presentation {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Presentation> for Any {
    fn from(s: Presentation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Presentation> for Any {
    fn from(s: &Presentation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Presentation);


impl Presentation {
    /// Getter of the `defaultRequest` attribute.
    /// [`Presentation.defaultRequest`](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)
    pub fn default_request(&self) -> PresentationRequest {
        self.inner.get("defaultRequest").as_::<PresentationRequest>()
    }

    /// Setter of the `defaultRequest` attribute.
    /// [`Presentation.defaultRequest`](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)
    pub fn set_default_request(&mut self, value: &PresentationRequest) {
        self.inner.set("defaultRequest", value);
    }
}
impl Presentation {
    /// Getter of the `receiver` attribute.
    /// [`Presentation.receiver`](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/receiver)
    pub fn receiver(&self) -> PresentationReceiver {
        self.inner.get("receiver").as_::<PresentationReceiver>()
    }

}
