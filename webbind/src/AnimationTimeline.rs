use super::*;

#[derive(Clone, Debug)]
pub struct AnimationTimeline {
    inner: emlite::Val,
}
impl FromVal for AnimationTimeline {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationTimeline {
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
impl std::ops::Deref for AnimationTimeline {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AnimationTimeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AnimationTimeline> for emlite::Val {
    fn from(s: AnimationTimeline) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AnimationTimeline {
    pub fn current_time(&self) -> jsbind::Any {
        self.inner.get("currentTime").as_::<jsbind::Any>()
    }
}
impl AnimationTimeline {
    pub fn duration(&self) -> jsbind::Any {
        self.inner.get("duration").as_::<jsbind::Any>()
    }
}
impl AnimationTimeline {
    pub fn play0(&self) -> Animation {
        self.inner.call("play", &[]).as_::<Animation>()
    }

    pub fn play1(&self, effect: AnimationEffect) -> Animation {
        self.inner.call("play", &[effect.into()]).as_::<Animation>()
    }
}
