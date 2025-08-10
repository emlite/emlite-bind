use super::*;

/// The DocumentTimeline class.
/// [`DocumentTimeline`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentTimeline {
    inner: AnimationTimeline,
}
impl FromVal for DocumentTimeline {
    fn from_val(v: &Any) -> Self {
        DocumentTimeline {
            inner: AnimationTimeline::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for DocumentTimeline {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DocumentTimeline {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DocumentTimeline> for Any {
    fn from(s: DocumentTimeline) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DocumentTimeline> for Any {
    fn from(s: &DocumentTimeline) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DocumentTimeline);

impl DocumentTimeline {
    /// The `new DocumentTimeline(..)` constructor, creating a new DocumentTimeline instance
    pub fn new0() -> DocumentTimeline {
        Self {
            inner: Any::global("DocumentTimeline")
                .new(&[])
                .as_::<AnimationTimeline>(),
        }
    }

    /// The `new DocumentTimeline(..)` constructor, creating a new DocumentTimeline instance
    pub fn new1(options: &DocumentTimelineOptions) -> DocumentTimeline {
        Self {
            inner: Any::global("DocumentTimeline")
                .new(&[options.into()])
                .as_::<AnimationTimeline>(),
        }
    }
}
