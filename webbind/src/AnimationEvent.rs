use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationEvent {
    inner: Event,
}
impl FromVal for AnimationEvent {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationEvent {
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
impl AsRef<emlite::Val> for AnimationEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AnimationEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AnimationEvent> for emlite::Val {
    fn from(s: AnimationEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AnimationEvent);

impl AnimationEvent {
    pub fn new0(type_: jsbind::CSSOMString) -> AnimationEvent {
        Self {
            inner: emlite::Val::global("AnimationEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(
        type_: jsbind::CSSOMString,
        animation_event_init_dict: jsbind::Any,
    ) -> AnimationEvent {
        Self {
            inner: emlite::Val::global("AnimationEvent")
                .new(&[type_.into(), animation_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl AnimationEvent {
    pub fn animation_name(&self) -> jsbind::CSSOMString {
        self.inner.get("animationName").as_::<jsbind::CSSOMString>()
    }
}
impl AnimationEvent {
    pub fn elapsed_time(&self) -> f64 {
        self.inner.get("elapsedTime").as_::<f64>()
    }
}
impl AnimationEvent {
    pub fn pseudo_element(&self) -> jsbind::CSSOMString {
        self.inner.get("pseudoElement").as_::<jsbind::CSSOMString>()
    }
}
