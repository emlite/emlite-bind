use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScrollTimeline {
    inner: AnimationTimeline,
}
impl FromVal for ScrollTimeline {
    fn from_val(v: &emlite::Val) -> Self {
        ScrollTimeline {
            inner: AnimationTimeline::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ScrollTimeline {
    type Target = AnimationTimeline;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ScrollTimeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ScrollTimeline {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ScrollTimeline {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ScrollTimeline> for emlite::Val {
    fn from(s: ScrollTimeline) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ScrollTimeline> for emlite::Val {
    fn from(s: &ScrollTimeline) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ScrollTimeline);

impl ScrollTimeline {
    pub fn new0() -> ScrollTimeline {
        Self {
            inner: emlite::Val::global("ScrollTimeline")
                .new(&[])
                .as_::<AnimationTimeline>(),
        }
    }

    pub fn new1(options: &Any) -> ScrollTimeline {
        Self {
            inner: emlite::Val::global("ScrollTimeline")
                .new(&[options.into()])
                .as_::<AnimationTimeline>(),
        }
    }
}
impl ScrollTimeline {
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }
}
impl ScrollTimeline {
    pub fn axis(&self) -> ScrollAxis {
        self.inner.get("axis").as_::<ScrollAxis>()
    }
}
