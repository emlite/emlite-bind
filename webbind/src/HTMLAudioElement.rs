use super::*;

/// The HTMLAudioElement class.
/// [`HTMLAudioElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLAudioElement {
    inner: HTMLMediaElement,
}

impl FromVal for HTMLAudioElement {
    fn from_val(v: &Any) -> Self {
        HTMLAudioElement {
            inner: HTMLMediaElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for HTMLAudioElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLAudioElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLAudioElement> for Any {
    fn from(s: HTMLAudioElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLAudioElement> for Any {
    fn from(s: &HTMLAudioElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLAudioElement);

impl HTMLAudioElement {
    /// The `new HTMLAudioElement(..)` constructor, creating a new HTMLAudioElement instance
    pub fn new() -> HTMLAudioElement {
        Self {
            inner: Any::global("HTMLAudioElement")
                .new(&[])
                .as_::<HTMLMediaElement>(),
        }
    }
}
