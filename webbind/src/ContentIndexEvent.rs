use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContentIndexEvent {
    inner: ExtendableEvent,
}
impl FromVal for ContentIndexEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ContentIndexEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ContentIndexEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContentIndexEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ContentIndexEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ContentIndexEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ContentIndexEvent> for emlite::Val {
    fn from(s: ContentIndexEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ContentIndexEvent> for emlite::Val {
    fn from(s: &ContentIndexEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ContentIndexEvent);

impl ContentIndexEvent {
    pub fn new(type_: DOMString, init: Any) -> ContentIndexEvent {
        Self {
            inner: emlite::Val::global("ContentIndexEvent")
                .new(&[type_.into(), init.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl ContentIndexEvent {
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }
}
