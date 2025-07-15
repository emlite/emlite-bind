use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLCollection {
    inner: emlite::Val,
}
impl FromVal for HTMLCollection {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLCollection {
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
impl core::ops::Deref for HTMLCollection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLCollection {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLCollection {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLCollection> for emlite::Val {
    fn from(s: HTMLCollection) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLCollection> for emlite::Val {
    fn from(s: &HTMLCollection) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLCollection);

impl HTMLCollection {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl HTMLCollection {
    pub fn item(&self, index: u32) -> Element {
        self.inner.call("item", &[index.into()]).as_::<Element>()
    }
}
impl HTMLCollection {
    pub fn named_item(&self, name: DOMString) -> Element {
        self.inner
            .call("namedItem", &[name.into()])
            .as_::<Element>()
    }
}
