use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationEventInit {
    inner: Any,
}
impl FromVal for AnimationEventInit {
    fn from_val(v: &Any) -> Self {
        AnimationEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AnimationEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AnimationEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AnimationEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AnimationEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AnimationEventInit> for Any {
    fn from(s: AnimationEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AnimationEventInit> for Any {
    fn from(s: &AnimationEventInit) -> Any {
        s.inner.clone()
    }
}

impl AnimationEventInit {
    pub fn animation_name(&self) -> JsString {
        self.inner.get("animationName").as_::<JsString>()
    }

    pub fn set_animation_name(&mut self, value: &JsString) {
        self.inner.set("animationName", value);
    }
}
impl AnimationEventInit {
    pub fn elapsed_time(&self) -> f64 {
        self.inner.get("elapsedTime").as_::<f64>()
    }

    pub fn set_elapsed_time(&mut self, value: f64) {
        self.inner.set("elapsedTime", value);
    }
}
impl AnimationEventInit {
    pub fn pseudo_element(&self) -> JsString {
        self.inner.get("pseudoElement").as_::<JsString>()
    }

    pub fn set_pseudo_element(&mut self, value: &JsString) {
        self.inner.set("pseudoElement", value);
    }
}
