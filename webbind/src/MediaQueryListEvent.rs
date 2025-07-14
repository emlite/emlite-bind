use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MediaQueryListEvent {
    inner: Event,
}
impl FromVal for MediaQueryListEvent {
    fn from_val(v: &emlite::Val) -> Self {
        MediaQueryListEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaQueryListEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaQueryListEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaQueryListEvent> for emlite::Val {
    fn from(s: MediaQueryListEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaQueryListEvent {
    pub fn new0(type_: jsbind::CSSOMString) -> MediaQueryListEvent {
        Self {
            inner: emlite::Val::global("MediaQueryListEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::CSSOMString, event_init_dict: jsbind::Any) -> MediaQueryListEvent {
        Self {
            inner: emlite::Val::global("MediaQueryListEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MediaQueryListEvent {
    pub fn media(&self) -> jsbind::CSSOMString {
        self.inner.get("media").as_::<jsbind::CSSOMString>()
    }
}
impl MediaQueryListEvent {
    pub fn matches(&self) -> bool {
        self.inner.get("matches").as_::<bool>()
    }
}
