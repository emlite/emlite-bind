use super::*;

/// The AnimationPlaybackEvent class.
/// [`AnimationPlaybackEvent`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationPlaybackEvent {
    inner: Event,
}
impl FromVal for AnimationPlaybackEvent {
    fn from_val(v: &Any) -> Self {
        AnimationPlaybackEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for AnimationPlaybackEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AnimationPlaybackEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AnimationPlaybackEvent> for Any {
    fn from(s: AnimationPlaybackEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AnimationPlaybackEvent> for Any {
    fn from(s: &AnimationPlaybackEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AnimationPlaybackEvent);

impl AnimationPlaybackEvent {
    /// The `new AnimationPlaybackEvent(..)` constructor, creating a new AnimationPlaybackEvent instance
    pub fn new0(type_: &DOMString) -> AnimationPlaybackEvent {
        Self {
            inner: Any::global("AnimationPlaybackEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new AnimationPlaybackEvent(..)` constructor, creating a new AnimationPlaybackEvent instance
    pub fn new1(type_: &DOMString, event_init_dict: &Any) -> AnimationPlaybackEvent {
        Self {
            inner: Any::global("AnimationPlaybackEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl AnimationPlaybackEvent {
    /// Getter of the `currentTime` attribute.
    /// [`AnimationPlaybackEvent.currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/currentTime)
    pub fn current_time(&self) -> Any {
        self.inner.get("currentTime").as_::<Any>()
    }
}
impl AnimationPlaybackEvent {
    /// Getter of the `timelineTime` attribute.
    /// [`AnimationPlaybackEvent.timelineTime`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/timelineTime)
    pub fn timeline_time(&self) -> Any {
        self.inner.get("timelineTime").as_::<Any>()
    }
}
