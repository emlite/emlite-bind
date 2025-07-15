use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLAudioElement {
    inner: HTMLMediaElement,
}
impl FromVal for HTMLAudioElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLAudioElement {
            inner: HTMLMediaElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLAudioElement {
    type Target = HTMLMediaElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLAudioElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLAudioElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLAudioElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLAudioElement> for emlite::Val {
    fn from(s: HTMLAudioElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLAudioElement);

impl HTMLAudioElement {
    pub fn new() -> HTMLAudioElement {
        Self {
            inner: emlite::Val::global("HTMLAudioElement")
                .new(&[])
                .as_::<HTMLMediaElement>(),
        }
    }
}
