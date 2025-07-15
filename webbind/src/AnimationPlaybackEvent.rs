use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationPlaybackEvent {
    inner: Event,
}
impl FromVal for AnimationPlaybackEvent {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationPlaybackEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AnimationPlaybackEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AnimationPlaybackEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AnimationPlaybackEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AnimationPlaybackEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<AnimationPlaybackEvent> for emlite::Val {
    fn from(s: AnimationPlaybackEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AnimationPlaybackEvent);



impl AnimationPlaybackEvent {
    pub fn new0(type_: DOMString) -> AnimationPlaybackEvent {
        Self {
            inner: emlite::Val::global("AnimationPlaybackEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> AnimationPlaybackEvent {
        Self {
            inner: emlite::Val::global("AnimationPlaybackEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl AnimationPlaybackEvent {
    pub fn current_time(&self) -> Any {
        self.inner.get("currentTime").as_::<Any>()
    }

}
impl AnimationPlaybackEvent {
    pub fn timeline_time(&self) -> Any {
        self.inner.get("timelineTime").as_::<Any>()
    }

}
