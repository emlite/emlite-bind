use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLDetailsElement {
    inner: HTMLElement,
}
impl FromVal for HTMLDetailsElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLDetailsElement {
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
impl core::ops::Deref for HTMLDetailsElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLDetailsElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLDetailsElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLDetailsElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLDetailsElement> for emlite::Val {
    fn from(s: HTMLDetailsElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLDetailsElement);

impl HTMLDetailsElement {
    pub fn new() -> HTMLDetailsElement {
        Self {
            inner: emlite::Val::global("HTMLDetailsElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLDetailsElement {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLDetailsElement {
    pub fn open(&self) -> bool {
        self.inner.get("open").as_::<bool>()
    }

    pub fn set_open(&mut self, value: bool) {
        self.inner.set("open", value);
    }
}
