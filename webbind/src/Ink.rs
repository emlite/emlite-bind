use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InkPresenterParam {
    inner: emlite::Val,
}
impl FromVal for InkPresenterParam {
    fn from_val(v: &emlite::Val) -> Self {
        InkPresenterParam { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for InkPresenterParam {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InkPresenterParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for InkPresenterParam {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for InkPresenterParam {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<InkPresenterParam> for emlite::Val {
    fn from(s: InkPresenterParam) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl InkPresenterParam {
    pub fn presentation_area(&self) -> Element {
        self.inner.get("presentationArea").as_::<Element>()
    }

    pub fn set_presentation_area(&mut self, value: Element) {
        self.inner.set("presentationArea", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Ink {
    inner: emlite::Val,
}
impl FromVal for Ink {
    fn from_val(v: &emlite::Val) -> Self {
        Ink {
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
impl core::ops::Deref for Ink {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Ink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Ink {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Ink {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Ink> for emlite::Val {
    fn from(s: Ink) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Ink);

impl Ink {
    pub fn request_presenter0(&self) -> jsbind::Promise {
        self.inner
            .call("requestPresenter", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn request_presenter1(&self, param: InkPresenterParam) -> jsbind::Promise {
        self.inner
            .call("requestPresenter", &[param.into()])
            .as_::<jsbind::Promise>()
    }
}
