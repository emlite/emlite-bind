use super::*;




/// The KeyframeAnimationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeyframeAnimationOptions {
    inner: Any,
}

impl FromVal for KeyframeAnimationOptions {
    fn from_val(v: &Any) -> Self {
        KeyframeAnimationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for KeyframeAnimationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KeyframeAnimationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KeyframeAnimationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KeyframeAnimationOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<KeyframeAnimationOptions> for Any {
    fn from(s: KeyframeAnimationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KeyframeAnimationOptions> for Any {
    fn from(s: &KeyframeAnimationOptions) -> Any {
        s.inner.clone()
    }
}

impl KeyframeAnimationOptions {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl KeyframeAnimationOptions {
    /// Getter of the `timeline` attribute.
    pub fn timeline(&self) -> AnimationTimeline {
        self.inner.get("timeline").as_::<AnimationTimeline>()
    }

    /// Setter of the `timeline` attribute.
    pub fn set_timeline(&mut self, value: &AnimationTimeline) {
        self.inner.set("timeline", value);
    }
}
