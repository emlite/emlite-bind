use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFormControlsCollection {
    inner: HTMLCollection,
}
impl FromVal for HTMLFormControlsCollection {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLFormControlsCollection {
            inner: HTMLCollection::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLFormControlsCollection {
    type Target = HTMLCollection;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLFormControlsCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLFormControlsCollection {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLFormControlsCollection {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLFormControlsCollection> for emlite::Val {
    fn from(s: HTMLFormControlsCollection) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLFormControlsCollection> for emlite::Val {
    fn from(s: &HTMLFormControlsCollection) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLFormControlsCollection);

impl HTMLFormControlsCollection {
    pub fn named_item(&self, name: &str) -> Any {
        self.inner.call("namedItem", &[name.into()]).as_::<Any>()
    }
}
