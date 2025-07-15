use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PointerTimeline {
    inner: AnimationTimeline,
}
impl FromVal for PointerTimeline {
    fn from_val(v: &emlite::Val) -> Self {
        PointerTimeline { inner: AnimationTimeline::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PointerTimeline {
    type Target = AnimationTimeline;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PointerTimeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PointerTimeline {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PointerTimeline {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PointerTimeline> for emlite::Val {
    fn from(s: PointerTimeline) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PointerTimeline);



impl PointerTimeline {
    pub fn new0() -> PointerTimeline {
        Self {
            inner: emlite::Val::global("PointerTimeline").new(&[]).as_::<AnimationTimeline>(),
        }
    }

    pub fn new1(options: Any) -> PointerTimeline {
        Self {
            inner: emlite::Val::global("PointerTimeline").new(&[options.into()]).as_::<AnimationTimeline>(),
        }
    }

}
impl PointerTimeline {
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }

}
impl PointerTimeline {
    pub fn axis(&self) -> PointerAxis {
        self.inner.get("axis").as_::<PointerAxis>()
    }

}
