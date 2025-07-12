use super::*;

#[derive(Clone, Debug)]
pub struct AnimationPlaybackEvent {
    inner: Event,
}
impl FromVal for AnimationPlaybackEvent {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationPlaybackEvent {
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
impl std::ops::Deref for AnimationPlaybackEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AnimationPlaybackEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AnimationPlaybackEvent> for emlite::Val {
    fn from(s: AnimationPlaybackEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AnimationPlaybackEvent {
    pub fn new0(type_: jsbind::DOMString) -> AnimationPlaybackEvent {
        Self {
            inner: emlite::Val::global("AnimationPlaybackEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> AnimationPlaybackEvent {
        Self {
            inner: emlite::Val::global("AnimationPlaybackEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl AnimationPlaybackEvent {
    pub fn current_time(&self) -> jsbind::Any {
        self.inner.get("currentTime").as_::<jsbind::Any>()
    }
}
impl AnimationPlaybackEvent {
    pub fn timeline_time(&self) -> jsbind::Any {
        self.inner.get("timelineTime").as_::<jsbind::Any>()
    }
}
