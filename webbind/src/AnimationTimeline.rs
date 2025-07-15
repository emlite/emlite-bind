use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationTimeline {
    inner: emlite::Val,
}
impl FromVal for AnimationTimeline {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationTimeline { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AnimationTimeline {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AnimationTimeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AnimationTimeline {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AnimationTimeline {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<AnimationTimeline> for emlite::Val {
    fn from(s: AnimationTimeline) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AnimationTimeline);


impl AnimationTimeline {
    pub fn current_time(&self) -> Any {
        self.inner.get("currentTime").as_::<Any>()
    }

}
impl AnimationTimeline {
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }

}
impl AnimationTimeline {
    pub fn play0(&self, ) -> Animation {
        self.inner.call("play", &[]).as_::<Animation>()
    }

    pub fn play1(&self, effect: AnimationEffect) -> Animation {
        self.inner.call("play", &[effect.into(), ]).as_::<Animation>()
    }

}
