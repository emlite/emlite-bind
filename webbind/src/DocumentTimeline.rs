use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentTimeline {
    inner: AnimationTimeline,
}
impl FromVal for DocumentTimeline {
    fn from_val(v: &emlite::Val) -> Self {
        DocumentTimeline {
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
impl core::ops::Deref for DocumentTimeline {
    type Target = AnimationTimeline;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DocumentTimeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DocumentTimeline {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DocumentTimeline {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DocumentTimeline> for emlite::Val {
    fn from(s: DocumentTimeline) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DocumentTimeline);

impl DocumentTimeline {
    pub fn new0() -> DocumentTimeline {
        Self {
            inner: emlite::Val::global("DocumentTimeline")
                .new(&[])
                .as_::<AnimationTimeline>(),
        }
    }

    pub fn new1(options: jsbind::Any) -> DocumentTimeline {
        Self {
            inner: emlite::Val::global("DocumentTimeline")
                .new(&[options.into()])
                .as_::<AnimationTimeline>(),
        }
    }
}
