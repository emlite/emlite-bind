use super::*;

/// The AnimationEvent class.
/// [`AnimationEvent`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationEvent {
    inner: Event,
}

impl FromVal for AnimationEvent {
    fn from_val(v: &Any) -> Self {
        AnimationEvent {
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

impl core::ops::Deref for AnimationEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AnimationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AnimationEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AnimationEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AnimationEvent> for Any {
    fn from(s: AnimationEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AnimationEvent> for Any {
    fn from(s: &AnimationEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AnimationEvent);

impl AnimationEvent {
    /// Getter of the `animationName` attribute.
    /// [`AnimationEvent.animationName`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/animationName)
    pub fn animation_name(&self) -> JsString {
        self.inner.get("animationName").as_::<JsString>()
    }
}
impl AnimationEvent {
    /// Getter of the `elapsedTime` attribute.
    /// [`AnimationEvent.elapsedTime`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/elapsedTime)
    pub fn elapsed_time(&self) -> f64 {
        self.inner.get("elapsedTime").as_::<f64>()
    }
}
impl AnimationEvent {
    /// Getter of the `pseudoElement` attribute.
    /// [`AnimationEvent.pseudoElement`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/pseudoElement)
    pub fn pseudo_element(&self) -> JsString {
        self.inner.get("pseudoElement").as_::<JsString>()
    }
}

impl AnimationEvent {
    /// The `new AnimationEvent(..)` constructor, creating a new AnimationEvent instance
    pub fn new0(type_: &JsString) -> AnimationEvent {
        Self {
            inner: Any::global("AnimationEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new AnimationEvent(..)` constructor, creating a new AnimationEvent instance
    pub fn new1(
        type_: &JsString,
        animation_event_init_dict: &AnimationEventInit,
    ) -> AnimationEvent {
        Self {
            inner: Any::global("AnimationEvent")
                .new(&[type_.into(), animation_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
