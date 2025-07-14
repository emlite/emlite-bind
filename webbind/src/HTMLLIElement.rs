use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLLIElement {
    inner: HTMLElement,
}
impl FromVal for HTMLLIElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLLIElement {
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
impl core::ops::Deref for HTMLLIElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLLIElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLLIElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLLIElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLLIElement> for emlite::Val {
    fn from(s: HTMLLIElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLLIElement);

impl HTMLLIElement {
    pub fn new() -> HTMLLIElement {
        Self {
            inner: emlite::Val::global("HTMLLIElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLLIElement {
    pub fn value(&self) -> i32 {
        self.inner.get("value").as_::<i32>()
    }

    pub fn set_value(&mut self, value: i32) {
        self.inner.set("value", value);
    }
}
impl HTMLLIElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
