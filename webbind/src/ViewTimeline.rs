use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ViewTimeline {
    inner: ScrollTimeline,
}
impl FromVal for ViewTimeline {
    fn from_val(v: &emlite::Val) -> Self {
        ViewTimeline {
            inner: ScrollTimeline::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ViewTimeline {
    type Target = ScrollTimeline;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ViewTimeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ViewTimeline {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ViewTimeline {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ViewTimeline> for emlite::Val {
    fn from(s: ViewTimeline) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ViewTimeline);

impl ViewTimeline {
    pub fn new0() -> ViewTimeline {
        Self {
            inner: emlite::Val::global("ViewTimeline")
                .new(&[])
                .as_::<ScrollTimeline>(),
        }
    }

    pub fn new1(options: jsbind::Any) -> ViewTimeline {
        Self {
            inner: emlite::Val::global("ViewTimeline")
                .new(&[options.into()])
                .as_::<ScrollTimeline>(),
        }
    }
}
impl ViewTimeline {
    pub fn subject(&self) -> Element {
        self.inner.get("subject").as_::<Element>()
    }
}
impl ViewTimeline {
    pub fn start_offset(&self) -> CSSNumericValue {
        self.inner.get("startOffset").as_::<CSSNumericValue>()
    }
}
impl ViewTimeline {
    pub fn end_offset(&self) -> CSSNumericValue {
        self.inner.get("endOffset").as_::<CSSNumericValue>()
    }
}
