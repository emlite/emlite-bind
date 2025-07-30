use super::*;

/// The TextTrackCue class.
/// [`TextTrackCue`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextTrackCue {
    inner: EventTarget,
}
impl FromVal for TextTrackCue {
    fn from_val(v: &Any) -> Self {
        TextTrackCue {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextTrackCue {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextTrackCue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextTrackCue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextTrackCue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextTrackCue> for Any {
    fn from(s: TextTrackCue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextTrackCue> for Any {
    fn from(s: &TextTrackCue) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextTrackCue);

impl TextTrackCue {
    /// Getter of the `track` attribute.
    /// [`TextTrackCue.track`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/track)
    pub fn track(&self) -> TextTrack {
        self.inner.get("track").as_::<TextTrack>()
    }
}
impl TextTrackCue {
    /// Getter of the `id` attribute.
    /// [`TextTrackCue.id`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    /// [`TextTrackCue.id`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl TextTrackCue {
    /// Getter of the `startTime` attribute.
    /// [`TextTrackCue.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)
    pub fn start_time(&self) -> f64 {
        self.inner.get("startTime").as_::<f64>()
    }

    /// Setter of the `startTime` attribute.
    /// [`TextTrackCue.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)
    pub fn set_start_time(&mut self, value: f64) {
        self.inner.set("startTime", value);
    }
}
impl TextTrackCue {
    /// Getter of the `endTime` attribute.
    /// [`TextTrackCue.endTime`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)
    pub fn end_time(&self) -> f64 {
        self.inner.get("endTime").as_::<f64>()
    }

    /// Setter of the `endTime` attribute.
    /// [`TextTrackCue.endTime`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)
    pub fn set_end_time(&mut self, value: f64) {
        self.inner.set("endTime", value);
    }
}
impl TextTrackCue {
    /// Getter of the `pauseOnExit` attribute.
    /// [`TextTrackCue.pauseOnExit`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)
    pub fn pause_on_exit(&self) -> bool {
        self.inner.get("pauseOnExit").as_::<bool>()
    }

    /// Setter of the `pauseOnExit` attribute.
    /// [`TextTrackCue.pauseOnExit`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)
    pub fn set_pause_on_exit(&mut self, value: bool) {
        self.inner.set("pauseOnExit", value);
    }
}
impl TextTrackCue {
    /// Getter of the `onenter` attribute.
    /// [`TextTrackCue.onenter`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)
    pub fn onenter(&self) -> Any {
        self.inner.get("onenter").as_::<Any>()
    }

    /// Setter of the `onenter` attribute.
    /// [`TextTrackCue.onenter`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)
    pub fn set_onenter(&mut self, value: &Any) {
        self.inner.set("onenter", value);
    }
}
impl TextTrackCue {
    /// Getter of the `onexit` attribute.
    /// [`TextTrackCue.onexit`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)
    pub fn onexit(&self) -> Any {
        self.inner.get("onexit").as_::<Any>()
    }

    /// Setter of the `onexit` attribute.
    /// [`TextTrackCue.onexit`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)
    pub fn set_onexit(&mut self, value: &Any) {
        self.inner.set("onexit", value);
    }
}
