use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMetaElement {
    inner: HTMLElement,
}
impl FromVal for HTMLMetaElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLMetaElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLMetaElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLMetaElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLMetaElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLMetaElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLMetaElement> for emlite::Val {
    fn from(s: HTMLMetaElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLMetaElement> for emlite::Val {
    fn from(s: &HTMLMetaElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLMetaElement);

impl HTMLMetaElement {
    pub fn new() -> HTMLMetaElement {
        Self {
            inner: emlite::Val::global("HTMLMetaElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLMetaElement {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl HTMLMetaElement {
    pub fn http_equiv(&self) -> String {
        self.inner.get("httpEquiv").as_::<String>()
    }

    pub fn set_http_equiv(&mut self, value: &str) {
        self.inner.set("httpEquiv", value);
    }
}
impl HTMLMetaElement {
    pub fn content(&self) -> String {
        self.inner.get("content").as_::<String>()
    }

    pub fn set_content(&mut self, value: &str) {
        self.inner.set("content", value);
    }
}
impl HTMLMetaElement {
    pub fn media(&self) -> String {
        self.inner.get("media").as_::<String>()
    }

    pub fn set_media(&mut self, value: &str) {
        self.inner.set("media", value);
    }
}
impl HTMLMetaElement {
    pub fn scheme(&self) -> String {
        self.inner.get("scheme").as_::<String>()
    }

    pub fn set_scheme(&mut self, value: &str) {
        self.inner.set("scheme", value);
    }
}
