use super::*;

/// The AnimationTimeline class.
/// [`AnimationTimeline`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationTimeline {
    inner: Any,
}

impl FromVal for AnimationTimeline {
    fn from_val(v: &Any) -> Self {
        AnimationTimeline {
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

impl core::ops::Deref for AnimationTimeline {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AnimationTimeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AnimationTimeline {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AnimationTimeline {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AnimationTimeline> for Any {
    fn from(s: AnimationTimeline) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AnimationTimeline> for Any {
    fn from(s: &AnimationTimeline) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AnimationTimeline);

impl AnimationTimeline {
    /// Getter of the `currentTime` attribute.
    /// [`AnimationTimeline.currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline/currentTime)
    pub fn current_time(&self) -> Any {
        self.inner.get("currentTime").as_::<Any>()
    }
}
impl AnimationTimeline {
    /// Getter of the `duration` attribute.
    /// [`AnimationTimeline.duration`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline/duration)
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }
}
impl AnimationTimeline {
    /// The play method.
    /// [`AnimationTimeline.play`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline/play)
    pub fn play0(&self) -> Animation {
        self.inner.call("play", &[]).as_::<Animation>()
    }
    /// The play method.
    /// [`AnimationTimeline.play`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline/play)
    pub fn play1(&self, effect: &AnimationEffect) -> Animation {
        self.inner.call("play", &[effect.into()]).as_::<Animation>()
    }
}
