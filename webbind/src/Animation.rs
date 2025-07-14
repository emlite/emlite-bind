use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Animation {
    inner: EventTarget,
}
impl FromVal for Animation {
    fn from_val(v: &emlite::Val) -> Self {
        Animation {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for Animation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Animation {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Animation> for emlite::Val {
    fn from(s: Animation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Animation);

impl Animation {
    pub fn new0() -> Animation {
        Self {
            inner: emlite::Val::global("Animation")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(effect: AnimationEffect) -> Animation {
        Self {
            inner: emlite::Val::global("Animation")
                .new(&[effect.into()])
                .as_::<EventTarget>(),
        }
    }

    pub fn new2(effect: AnimationEffect, timeline: AnimationTimeline) -> Animation {
        Self {
            inner: emlite::Val::global("Animation")
                .new(&[effect.into(), timeline.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl Animation {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }

    pub fn set_id(&mut self, value: jsbind::DOMString) {
        self.inner.set("id", value);
    }
}
impl Animation {
    pub fn effect(&self) -> AnimationEffect {
        self.inner.get("effect").as_::<AnimationEffect>()
    }

    pub fn set_effect(&mut self, value: AnimationEffect) {
        self.inner.set("effect", value);
    }
}
impl Animation {
    pub fn timeline(&self) -> AnimationTimeline {
        self.inner.get("timeline").as_::<AnimationTimeline>()
    }

    pub fn set_timeline(&mut self, value: AnimationTimeline) {
        self.inner.set("timeline", value);
    }
}
impl Animation {
    pub fn playback_rate(&self) -> f64 {
        self.inner.get("playbackRate").as_::<f64>()
    }

    pub fn set_playback_rate(&mut self, value: f64) {
        self.inner.set("playbackRate", value);
    }
}
impl Animation {
    pub fn play_state(&self) -> AnimationPlayState {
        self.inner.get("playState").as_::<AnimationPlayState>()
    }
}
impl Animation {
    pub fn replace_state(&self) -> AnimationReplaceState {
        self.inner
            .get("replaceState")
            .as_::<AnimationReplaceState>()
    }
}
impl Animation {
    pub fn pending(&self) -> bool {
        self.inner.get("pending").as_::<bool>()
    }
}
impl Animation {
    pub fn ready(&self) -> jsbind::Promise {
        self.inner.get("ready").as_::<jsbind::Promise>()
    }
}
impl Animation {
    pub fn finished(&self) -> jsbind::Promise {
        self.inner.get("finished").as_::<jsbind::Promise>()
    }
}
impl Animation {
    pub fn onfinish(&self) -> jsbind::Any {
        self.inner.get("onfinish").as_::<jsbind::Any>()
    }

    pub fn set_onfinish(&mut self, value: jsbind::Any) {
        self.inner.set("onfinish", value);
    }
}
impl Animation {
    pub fn oncancel(&self) -> jsbind::Any {
        self.inner.get("oncancel").as_::<jsbind::Any>()
    }

    pub fn set_oncancel(&mut self, value: jsbind::Any) {
        self.inner.set("oncancel", value);
    }
}
impl Animation {
    pub fn onremove(&self) -> jsbind::Any {
        self.inner.get("onremove").as_::<jsbind::Any>()
    }

    pub fn set_onremove(&mut self, value: jsbind::Any) {
        self.inner.set("onremove", value);
    }
}
impl Animation {
    pub fn cancel(&self) -> jsbind::Undefined {
        self.inner.call("cancel", &[]).as_::<jsbind::Undefined>()
    }
}
impl Animation {
    pub fn finish(&self) -> jsbind::Undefined {
        self.inner.call("finish", &[]).as_::<jsbind::Undefined>()
    }
}
impl Animation {
    pub fn play(&self) -> jsbind::Undefined {
        self.inner.call("play", &[]).as_::<jsbind::Undefined>()
    }
}
impl Animation {
    pub fn pause(&self) -> jsbind::Undefined {
        self.inner.call("pause", &[]).as_::<jsbind::Undefined>()
    }
}
impl Animation {
    pub fn update_playback_rate(&self, playback_rate: f64) -> jsbind::Undefined {
        self.inner
            .call("updatePlaybackRate", &[playback_rate.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Animation {
    pub fn reverse(&self) -> jsbind::Undefined {
        self.inner.call("reverse", &[]).as_::<jsbind::Undefined>()
    }
}
impl Animation {
    pub fn persist(&self) -> jsbind::Undefined {
        self.inner.call("persist", &[]).as_::<jsbind::Undefined>()
    }
}
impl Animation {
    pub fn commit_styles(&self) -> jsbind::Undefined {
        self.inner
            .call("commitStyles", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl Animation {
    pub fn start_time(&self) -> jsbind::Any {
        self.inner.get("startTime").as_::<jsbind::Any>()
    }

    pub fn set_start_time(&mut self, value: jsbind::Any) {
        self.inner.set("startTime", value);
    }
}
impl Animation {
    pub fn current_time(&self) -> jsbind::Any {
        self.inner.get("currentTime").as_::<jsbind::Any>()
    }

    pub fn set_current_time(&mut self, value: jsbind::Any) {
        self.inner.set("currentTime", value);
    }
}
impl Animation {
    pub fn trigger(&self) -> AnimationTrigger {
        self.inner.get("trigger").as_::<AnimationTrigger>()
    }

    pub fn set_trigger(&mut self, value: AnimationTrigger) {
        self.inner.set("trigger", value);
    }
}
impl Animation {
    pub fn overall_progress(&self) -> f64 {
        self.inner.get("overallProgress").as_::<f64>()
    }
}
