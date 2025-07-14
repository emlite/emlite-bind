use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationTrigger {
    inner: emlite::Val,
}
impl FromVal for AnimationTrigger {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationTrigger {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AnimationTrigger {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AnimationTrigger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AnimationTrigger {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AnimationTrigger {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AnimationTrigger> for emlite::Val {
    fn from(s: AnimationTrigger) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AnimationTrigger);

impl AnimationTrigger {
    pub fn new0() -> AnimationTrigger {
        Self {
            inner: emlite::Val::global("AnimationTrigger")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(options: jsbind::Any) -> AnimationTrigger {
        Self {
            inner: emlite::Val::global("AnimationTrigger")
                .new(&[options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl AnimationTrigger {
    pub fn timeline(&self) -> AnimationTimeline {
        self.inner.get("timeline").as_::<AnimationTimeline>()
    }

    pub fn set_timeline(&mut self, value: AnimationTimeline) {
        self.inner.set("timeline", value);
    }
}
impl AnimationTrigger {
    pub fn behavior(&self) -> AnimationTriggerBehavior {
        self.inner.get("behavior").as_::<AnimationTriggerBehavior>()
    }

    pub fn set_behavior(&mut self, value: AnimationTriggerBehavior) {
        self.inner.set("behavior", value);
    }
}
impl AnimationTrigger {
    pub fn range_start(&self) -> jsbind::Any {
        self.inner.get("rangeStart").as_::<jsbind::Any>()
    }

    pub fn set_range_start(&mut self, value: jsbind::Any) {
        self.inner.set("rangeStart", value);
    }
}
impl AnimationTrigger {
    pub fn range_end(&self) -> jsbind::Any {
        self.inner.get("rangeEnd").as_::<jsbind::Any>()
    }

    pub fn set_range_end(&mut self, value: jsbind::Any) {
        self.inner.set("rangeEnd", value);
    }
}
impl AnimationTrigger {
    pub fn exit_range_start(&self) -> jsbind::Any {
        self.inner.get("exitRangeStart").as_::<jsbind::Any>()
    }

    pub fn set_exit_range_start(&mut self, value: jsbind::Any) {
        self.inner.set("exitRangeStart", value);
    }
}
impl AnimationTrigger {
    pub fn exit_range_end(&self) -> jsbind::Any {
        self.inner.get("exitRangeEnd").as_::<jsbind::Any>()
    }

    pub fn set_exit_range_end(&mut self, value: jsbind::Any) {
        self.inner.set("exitRangeEnd", value);
    }
}
