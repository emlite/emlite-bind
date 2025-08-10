use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationTriggerOptions {
    inner: Any,
}
impl FromVal for AnimationTriggerOptions {
    fn from_val(v: &Any) -> Self {
        AnimationTriggerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AnimationTriggerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AnimationTriggerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AnimationTriggerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AnimationTriggerOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AnimationTriggerOptions> for Any {
    fn from(s: AnimationTriggerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AnimationTriggerOptions> for Any {
    fn from(s: &AnimationTriggerOptions) -> Any {
        s.inner.clone()
    }
}

impl AnimationTriggerOptions {
    pub fn timeline(&self) -> AnimationTimeline {
        self.inner.get("timeline").as_::<AnimationTimeline>()
    }

    pub fn set_timeline(&mut self, value: &AnimationTimeline) {
        self.inner.set("timeline", value);
    }
}
impl AnimationTriggerOptions {
    pub fn behavior(&self) -> AnimationTriggerBehavior {
        self.inner.get("behavior").as_::<AnimationTriggerBehavior>()
    }

    pub fn set_behavior(&mut self, value: &AnimationTriggerBehavior) {
        self.inner.set("behavior", value);
    }
}
impl AnimationTriggerOptions {
    pub fn range_start(&self) -> Any {
        self.inner.get("rangeStart").as_::<Any>()
    }

    pub fn set_range_start(&mut self, value: &Any) {
        self.inner.set("rangeStart", value);
    }
}
impl AnimationTriggerOptions {
    pub fn range_end(&self) -> Any {
        self.inner.get("rangeEnd").as_::<Any>()
    }

    pub fn set_range_end(&mut self, value: &Any) {
        self.inner.set("rangeEnd", value);
    }
}
impl AnimationTriggerOptions {
    pub fn exit_range_start(&self) -> Any {
        self.inner.get("exitRangeStart").as_::<Any>()
    }

    pub fn set_exit_range_start(&mut self, value: &Any) {
        self.inner.set("exitRangeStart", value);
    }
}
impl AnimationTriggerOptions {
    pub fn exit_range_end(&self) -> Any {
        self.inner.get("exitRangeEnd").as_::<Any>()
    }

    pub fn set_exit_range_end(&mut self, value: &Any) {
        self.inner.set("exitRangeEnd", value);
    }
}
