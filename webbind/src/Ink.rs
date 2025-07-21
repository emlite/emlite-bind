use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InkPresenterParam {
    inner: Any,
}
impl FromVal for InkPresenterParam {
    fn from_val(v: &Any) -> Self {
        InkPresenterParam { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for InkPresenterParam {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InkPresenterParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for InkPresenterParam {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for InkPresenterParam {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<InkPresenterParam> for Any {
    fn from(s: InkPresenterParam) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&InkPresenterParam> for Any {
    fn from(s: &InkPresenterParam) -> Any {
        s.inner.clone()
    }
}

impl InkPresenterParam {
    pub fn presentation_area(&self) -> Element {
        self.inner.get("presentationArea").as_::<Element>()
    }

    pub fn set_presentation_area(&mut self, value: &Element) {
        self.inner.set("presentationArea", value);
    }
}
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
    pub fn request_presenter0(&self) -> Promise<DelegatedInkTrailPresenter> {
        self.inner
            .call("requestPresenter", &[])
            .as_::<Promise<DelegatedInkTrailPresenter>>()
    }
    /// The requestPresenter method.
    /// [`Ink.requestPresenter`](https://developer.mozilla.org/en-US/docs/Web/API/Ink/requestPresenter)
    pub fn request_presenter1(
        &self,
        param: &InkPresenterParam,
    ) -> Promise<DelegatedInkTrailPresenter> {
        self.inner
            .call("requestPresenter", &[param.into()])
            .as_::<Promise<DelegatedInkTrailPresenter>>()
    }
}
