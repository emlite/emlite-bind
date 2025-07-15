use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Presentation {
    inner: emlite::Val,
}
impl FromVal for Presentation {
    fn from_val(v: &emlite::Val) -> Self {
        Presentation {
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
impl core::ops::Deref for Presentation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Presentation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Presentation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Presentation {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Presentation> for emlite::Val {
    fn from(s: Presentation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Presentation> for emlite::Val {
    fn from(s: &Presentation) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Presentation);

impl Presentation {
    pub fn default_request(&self) -> PresentationRequest {
        self.inner
            .get("defaultRequest")
            .as_::<PresentationRequest>()
    }

    pub fn set_default_request(&mut self, value: PresentationRequest) {
        self.inner.set("defaultRequest", value);
    }
}
impl Presentation {
    pub fn receiver(&self) -> PresentationReceiver {
        self.inner.get("receiver").as_::<PresentationReceiver>()
    }
}
