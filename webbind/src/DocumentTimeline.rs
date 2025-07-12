use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for DocumentTimeline {
    type Target = AnimationTimeline;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DocumentTimeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DocumentTimeline> for emlite::Val {
    fn from(s: DocumentTimeline) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
