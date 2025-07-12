use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for Presentation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Presentation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Presentation> for emlite::Val {
    fn from(s: Presentation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
