use super::*;

#[derive(Clone, Debug)]
pub struct MediaStreamTrackEvent {
    inner: Event,
}
impl FromVal for MediaStreamTrackEvent {
    fn from_val(v: &emlite::Val) -> Self {
        MediaStreamTrackEvent {
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
impl std::ops::Deref for MediaStreamTrackEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaStreamTrackEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaStreamTrackEvent> for emlite::Val {
    fn from(s: MediaStreamTrackEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaStreamTrackEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> MediaStreamTrackEvent {
        Self {
            inner: emlite::Val::global("MediaStreamTrackEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MediaStreamTrackEvent {
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}
