use super::*;




/// The Animation class.
/// [`Animation`](https://developer.mozilla.org/en-US/docs/Web/API/Animation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Animation {
    inner: EventTarget,
}

impl FromVal for Animation {
    fn from_val(v: &Any) -> Self {
        Animation { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Animation {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Animation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Animation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Animation {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Animation> for Any {
    fn from(s: Animation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Animation> for Any {
    fn from(s: &Animation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Animation);



impl Animation {
    /// The `new Animation(..)` constructor, creating a new Animation instance
    pub fn new0() -> Animation {
        Self {
            inner: Any::global("Animation").new(&[]).as_::<EventTarget>(),
        }
    }

    /// The `new Animation(..)` constructor, creating a new Animation instance
    pub fn new1(effect: &AnimationEffect) -> Animation {
        Self {
            inner: Any::global("Animation").new(&[effect.into()]).as_::<EventTarget>(),
        }
    }

    /// The `new Animation(..)` constructor, creating a new Animation instance
    pub fn new2(effect: &AnimationEffect, timeline: &AnimationTimeline) -> Animation {
        Self {
            inner: Any::global("Animation").new(&[effect.into(), timeline.into()]).as_::<EventTarget>(),
        }
    }

}
impl Animation {
    /// Getter of the `id` attribute.
    /// [`Animation.id`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    /// [`Animation.id`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/id)
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl Animation {
    /// Getter of the `effect` attribute.
    /// [`Animation.effect`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/effect)
    pub fn effect(&self) -> AnimationEffect {
        self.inner.get("effect").as_::<AnimationEffect>()
    }

    /// Setter of the `effect` attribute.
    /// [`Animation.effect`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/effect)
    pub fn set_effect(&mut self, value: &AnimationEffect) {
        self.inner.set("effect", value);
    }
}
impl Animation {
    /// Getter of the `timeline` attribute.
    /// [`Animation.timeline`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/timeline)
    pub fn timeline(&self) -> AnimationTimeline {
        self.inner.get("timeline").as_::<AnimationTimeline>()
    }

    /// Setter of the `timeline` attribute.
    /// [`Animation.timeline`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/timeline)
    pub fn set_timeline(&mut self, value: &AnimationTimeline) {
        self.inner.set("timeline", value);
    }
}
impl Animation {
    /// Getter of the `playbackRate` attribute.
    /// [`Animation.playbackRate`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playbackRate)
    pub fn playback_rate(&self) -> f64 {
        self.inner.get("playbackRate").as_::<f64>()
    }

    /// Setter of the `playbackRate` attribute.
    /// [`Animation.playbackRate`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playbackRate)
    pub fn set_playback_rate(&mut self, value: f64) {
        self.inner.set("playbackRate", value);
    }
}
impl Animation {
    /// Getter of the `playState` attribute.
    /// [`Animation.playState`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playState)
    pub fn play_state(&self) -> AnimationPlayState {
        self.inner.get("playState").as_::<AnimationPlayState>()
    }

}
impl Animation {
    /// Getter of the `replaceState` attribute.
    /// [`Animation.replaceState`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/replaceState)
    pub fn replace_state(&self) -> AnimationReplaceState {
        self.inner.get("replaceState").as_::<AnimationReplaceState>()
    }

}
impl Animation {
    /// Getter of the `pending` attribute.
    /// [`Animation.pending`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/pending)
    pub fn pending(&self) -> bool {
        self.inner.get("pending").as_::<bool>()
    }

}
impl Animation {
    /// Getter of the `ready` attribute.
    /// [`Animation.ready`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/ready)
    pub fn ready(&self) -> Promise<Animation> {
        self.inner.get("ready").as_::<Promise<Animation>>()
    }

}
impl Animation {
    /// Getter of the `finished` attribute.
    /// [`Animation.finished`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finished)
    pub fn finished(&self) -> Promise<Animation> {
        self.inner.get("finished").as_::<Promise<Animation>>()
    }

}
impl Animation {
    /// Getter of the `onfinish` attribute.
    /// [`Animation.onfinish`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onfinish)
    pub fn onfinish(&self) -> Any {
        self.inner.get("onfinish").as_::<Any>()
    }

    /// Setter of the `onfinish` attribute.
    /// [`Animation.onfinish`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onfinish)
    pub fn set_onfinish(&mut self, value: &Any) {
        self.inner.set("onfinish", value);
    }
}
impl Animation {
    /// Getter of the `oncancel` attribute.
    /// [`Animation.oncancel`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/oncancel)
    pub fn oncancel(&self) -> Any {
        self.inner.get("oncancel").as_::<Any>()
    }

    /// Setter of the `oncancel` attribute.
    /// [`Animation.oncancel`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/oncancel)
    pub fn set_oncancel(&mut self, value: &Any) {
        self.inner.set("oncancel", value);
    }
}
impl Animation {
    /// Getter of the `onremove` attribute.
    /// [`Animation.onremove`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onremove)
    pub fn onremove(&self) -> Any {
        self.inner.get("onremove").as_::<Any>()
    }

    /// Setter of the `onremove` attribute.
    /// [`Animation.onremove`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onremove)
    pub fn set_onremove(&mut self, value: &Any) {
        self.inner.set("onremove", value);
    }
}
impl Animation {
    /// The cancel method.
    /// [`Animation.cancel`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/cancel)
    pub fn cancel(&self, ) -> Undefined {
        self.inner.call("cancel", &[]).as_::<Undefined>()
    }
}
impl Animation {
    /// The finish method.
    /// [`Animation.finish`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finish)
    pub fn finish(&self, ) -> Undefined {
        self.inner.call("finish", &[]).as_::<Undefined>()
    }
}
impl Animation {
    /// The play method.
    /// [`Animation.play`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/play)
    pub fn play(&self, ) -> Undefined {
        self.inner.call("play", &[]).as_::<Undefined>()
    }
}
impl Animation {
    /// The pause method.
    /// [`Animation.pause`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/pause)
    pub fn pause(&self, ) -> Undefined {
        self.inner.call("pause", &[]).as_::<Undefined>()
    }
}
impl Animation {
    /// The updatePlaybackRate method.
    /// [`Animation.updatePlaybackRate`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/updatePlaybackRate)
    pub fn update_playback_rate(&self, playback_rate: f64) -> Undefined {
        self.inner.call("updatePlaybackRate", &[playback_rate.into(), ]).as_::<Undefined>()
    }
}
impl Animation {
    /// The reverse method.
    /// [`Animation.reverse`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/reverse)
    pub fn reverse(&self, ) -> Undefined {
        self.inner.call("reverse", &[]).as_::<Undefined>()
    }
}
impl Animation {
    /// The persist method.
    /// [`Animation.persist`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/persist)
    pub fn persist(&self, ) -> Undefined {
        self.inner.call("persist", &[]).as_::<Undefined>()
    }
}
impl Animation {
    /// The commitStyles method.
    /// [`Animation.commitStyles`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/commitStyles)
    pub fn commit_styles(&self, ) -> Undefined {
        self.inner.call("commitStyles", &[]).as_::<Undefined>()
    }
}
impl Animation {
    /// Getter of the `startTime` attribute.
    /// [`Animation.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/startTime)
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }

    /// Setter of the `startTime` attribute.
    /// [`Animation.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/startTime)
    pub fn set_start_time(&mut self, value: &Any) {
        self.inner.set("startTime", value);
    }
}
impl Animation {
    /// Getter of the `currentTime` attribute.
    /// [`Animation.currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/currentTime)
    pub fn current_time(&self) -> Any {
        self.inner.get("currentTime").as_::<Any>()
    }

    /// Setter of the `currentTime` attribute.
    /// [`Animation.currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/currentTime)
    pub fn set_current_time(&mut self, value: &Any) {
        self.inner.set("currentTime", value);
    }
}
impl Animation {
    /// Getter of the `trigger` attribute.
    /// [`Animation.trigger`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/trigger)
    pub fn trigger(&self) -> AnimationTrigger {
        self.inner.get("trigger").as_::<AnimationTrigger>()
    }

    /// Setter of the `trigger` attribute.
    /// [`Animation.trigger`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/trigger)
    pub fn set_trigger(&mut self, value: &AnimationTrigger) {
        self.inner.set("trigger", value);
    }
}
impl Animation {
    /// Getter of the `overallProgress` attribute.
    /// [`Animation.overallProgress`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/overallProgress)
    pub fn overall_progress(&self) -> f64 {
        self.inner.get("overallProgress").as_::<f64>()
    }

}
