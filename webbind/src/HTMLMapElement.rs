use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMapElement {
    inner: HTMLElement,
}
impl FromVal for HTMLMapElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLMapElement {
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
impl core::ops::Deref for HTMLMapElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLMapElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLMapElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLMapElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLMapElement> for emlite::Val {
    fn from(s: HTMLMapElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLMapElement> for emlite::Val {
    fn from(s: &HTMLMapElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLMapElement);

impl HTMLMapElement {
    pub fn new() -> HTMLMapElement {
        Self {
            inner: emlite::Val::global("HTMLMapElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLMapElement {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl HTMLMapElement {
    pub fn areas(&self) -> HTMLCollection {
        self.inner.get("areas").as_::<HTMLCollection>()
    }
}
