use super::*;

/// The AnimationTrigger class.
/// [`AnimationTrigger`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationTrigger {
    inner: Any,
}

impl FromVal for AnimationTrigger {
    fn from_val(v: &Any) -> Self {
        AnimationTrigger {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AnimationTrigger {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AnimationTrigger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AnimationTrigger {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AnimationTrigger {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AnimationTrigger> for Any {
    fn from(s: AnimationTrigger) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AnimationTrigger> for Any {
    fn from(s: &AnimationTrigger) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AnimationTrigger);

impl AnimationTrigger {
    /// The `new AnimationTrigger(..)` constructor, creating a new AnimationTrigger instance
    pub fn new0() -> AnimationTrigger {
        Self {
            inner: Any::global("AnimationTrigger").new(&[]).as_::<Any>(),
        }
    }

    /// The `new AnimationTrigger(..)` constructor, creating a new AnimationTrigger instance
    pub fn new1(options: &AnimationTriggerOptions) -> AnimationTrigger {
        Self {
            inner: Any::global("AnimationTrigger")
                .new(&[options.into()])
                .as_::<Any>(),
        }
    }
}
impl AnimationTrigger {
    /// Getter of the `timeline` attribute.
    /// [`AnimationTrigger.timeline`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/timeline)
    pub fn timeline(&self) -> AnimationTimeline {
        self.inner.get("timeline").as_::<AnimationTimeline>()
    }

    /// Setter of the `timeline` attribute.
    /// [`AnimationTrigger.timeline`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/timeline)
    pub fn set_timeline(&mut self, value: &AnimationTimeline) {
        self.inner.set("timeline", value);
    }
}
impl AnimationTrigger {
    /// Getter of the `behavior` attribute.
    /// [`AnimationTrigger.behavior`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/behavior)
    pub fn behavior(&self) -> AnimationTriggerBehavior {
        self.inner.get("behavior").as_::<AnimationTriggerBehavior>()
    }

    /// Setter of the `behavior` attribute.
    /// [`AnimationTrigger.behavior`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/behavior)
    pub fn set_behavior(&mut self, value: &AnimationTriggerBehavior) {
        self.inner.set("behavior", value);
    }
}
impl AnimationTrigger {
    /// Getter of the `rangeStart` attribute.
    /// [`AnimationTrigger.rangeStart`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/rangeStart)
    pub fn range_start(&self) -> Any {
        self.inner.get("rangeStart").as_::<Any>()
    }

    /// Setter of the `rangeStart` attribute.
    /// [`AnimationTrigger.rangeStart`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/rangeStart)
    pub fn set_range_start(&mut self, value: &Any) {
        self.inner.set("rangeStart", value);
    }
}
impl AnimationTrigger {
    /// Getter of the `rangeEnd` attribute.
    /// [`AnimationTrigger.rangeEnd`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/rangeEnd)
    pub fn range_end(&self) -> Any {
        self.inner.get("rangeEnd").as_::<Any>()
    }

    /// Setter of the `rangeEnd` attribute.
    /// [`AnimationTrigger.rangeEnd`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/rangeEnd)
    pub fn set_range_end(&mut self, value: &Any) {
        self.inner.set("rangeEnd", value);
    }
}
impl AnimationTrigger {
    /// Getter of the `exitRangeStart` attribute.
    /// [`AnimationTrigger.exitRangeStart`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/exitRangeStart)
    pub fn exit_range_start(&self) -> Any {
        self.inner.get("exitRangeStart").as_::<Any>()
    }

    /// Setter of the `exitRangeStart` attribute.
    /// [`AnimationTrigger.exitRangeStart`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/exitRangeStart)
    pub fn set_exit_range_start(&mut self, value: &Any) {
        self.inner.set("exitRangeStart", value);
    }
}
impl AnimationTrigger {
    /// Getter of the `exitRangeEnd` attribute.
    /// [`AnimationTrigger.exitRangeEnd`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/exitRangeEnd)
    pub fn exit_range_end(&self) -> Any {
        self.inner.get("exitRangeEnd").as_::<Any>()
    }

    /// Setter of the `exitRangeEnd` attribute.
    /// [`AnimationTrigger.exitRangeEnd`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTrigger/exitRangeEnd)
    pub fn set_exit_range_end(&mut self, value: &Any) {
        self.inner.set("exitRangeEnd", value);
    }
}
