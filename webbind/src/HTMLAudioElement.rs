use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for HTMLAudioElement {
    type Target = HTMLMediaElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLAudioElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLAudioElement> for emlite::Val {
    fn from(s: HTMLAudioElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLAudioElement {
    pub fn new() -> HTMLAudioElement {
        Self {
            inner: emlite::Val::global("HTMLAudioElement")
                .new(&[])
                .as_::<HTMLMediaElement>(),
        }
    }
}
