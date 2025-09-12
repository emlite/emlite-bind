use super::*;

/// The Ink class.
/// [`Ink`](https://developer.mozilla.org/en-US/docs/Web/API/Ink)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Ink {
    inner: Any,
}

impl FromVal for Ink {
    fn from_val(v: &Any) -> Self {
        Ink {
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

impl core::ops::Deref for Ink {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Ink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Ink {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Ink {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Ink> for Any {
    fn from(s: Ink) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Ink> for Any {
    fn from(s: &Ink) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Ink);

impl Ink {
    /// The requestPresenter method.
    /// [`Ink.requestPresenter`](https://developer.mozilla.org/en-US/docs/Web/API/Ink/requestPresenter)
    pub fn request_presenter(&self) -> Promise<DelegatedInkTrailPresenter> {
        self.inner
            .call("requestPresenter", &[])
            .as_::<Promise<DelegatedInkTrailPresenter>>()
    }
}
impl Ink {
    /// The requestPresenter method.
    /// [`Ink.requestPresenter`](https://developer.mozilla.org/en-US/docs/Web/API/Ink/requestPresenter)
    pub fn request_presenter_with_param(
        &self,
        param: &InkPresenterParam,
    ) -> Promise<DelegatedInkTrailPresenter> {
        self.inner
            .call("requestPresenter", &[param.into()])
            .as_::<Promise<DelegatedInkTrailPresenter>>()
    }
}
