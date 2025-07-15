use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLModElement {
    inner: HTMLElement,
}
impl FromVal for HTMLModElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLModElement {
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
impl core::ops::Deref for HTMLModElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLModElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLModElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLModElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLModElement> for emlite::Val {
    fn from(s: HTMLModElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLModElement> for emlite::Val {
    fn from(s: &HTMLModElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLModElement);

impl HTMLModElement {
    pub fn new() -> HTMLModElement {
        Self {
            inner: emlite::Val::global("HTMLModElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLModElement {
    pub fn cite(&self) -> String {
        self.inner.get("cite").as_::<String>()
    }

    pub fn set_cite(&mut self, value: &str) {
        self.inner.set("cite", value);
    }
}
impl HTMLModElement {
    pub fn date_time(&self) -> String {
        self.inner.get("dateTime").as_::<String>()
    }

    pub fn set_date_time(&mut self, value: &str) {
        self.inner.set("dateTime", value);
    }
}
