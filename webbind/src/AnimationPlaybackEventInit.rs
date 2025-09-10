use super::*;

/// The AnimationPlaybackEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationPlaybackEventInit {
    inner: Any,
}

impl FromVal for AnimationPlaybackEventInit {
    fn from_val(v: &Any) -> Self {
        AnimationPlaybackEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AnimationPlaybackEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AnimationPlaybackEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AnimationPlaybackEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AnimationPlaybackEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AnimationPlaybackEventInit> for Any {
    fn from(s: AnimationPlaybackEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AnimationPlaybackEventInit> for Any {
    fn from(s: &AnimationPlaybackEventInit) -> Any {
        s.inner.clone()
    }
}

impl AnimationPlaybackEventInit {
    /// Getter of the `currentTime` attribute.
    pub fn current_time(&self) -> Any {
        self.inner.get("currentTime").as_::<Any>()
    }

    /// Setter of the `currentTime` attribute.
    pub fn set_current_time(&mut self, value: &Any) {
        self.inner.set("currentTime", value);
    }
}
impl AnimationPlaybackEventInit {
    /// Getter of the `timelineTime` attribute.
    pub fn timeline_time(&self) -> Any {
        self.inner.get("timelineTime").as_::<Any>()
    }

    /// Setter of the `timelineTime` attribute.
    pub fn set_timeline_time(&mut self, value: &Any) {
        self.inner.set("timelineTime", value);
    }
}
