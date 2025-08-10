use super::*;

/// The ViewTimeline class.
/// [`ViewTimeline`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTimeline)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ViewTimeline {
    inner: ScrollTimeline,
}

impl FromVal for ViewTimeline {
    fn from_val(v: &Any) -> Self {
        ViewTimeline {
            inner: ScrollTimeline::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for ViewTimeline {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ViewTimeline {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ViewTimeline> for Any {
    fn from(s: ViewTimeline) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ViewTimeline> for Any {
    fn from(s: &ViewTimeline) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ViewTimeline);

impl ViewTimeline {
    /// The `new ViewTimeline(..)` constructor, creating a new ViewTimeline instance
    pub fn new0() -> ViewTimeline {
        Self {
            inner: Any::global("ViewTimeline").new(&[]).as_::<ScrollTimeline>(),
        }
    }

    /// The `new ViewTimeline(..)` constructor, creating a new ViewTimeline instance
    pub fn new1(options: &ViewTimelineOptions) -> ViewTimeline {
        Self {
            inner: Any::global("ViewTimeline")
                .new(&[options.into()])
                .as_::<ScrollTimeline>(),
        }
    }
}
impl ViewTimeline {
    /// Getter of the `subject` attribute.
    /// [`ViewTimeline.subject`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTimeline/subject)
    pub fn subject(&self) -> Element {
        self.inner.get("subject").as_::<Element>()
    }
}
impl ViewTimeline {
    /// Getter of the `startOffset` attribute.
    /// [`ViewTimeline.startOffset`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTimeline/startOffset)
    pub fn start_offset(&self) -> CSSNumericValue {
        self.inner.get("startOffset").as_::<CSSNumericValue>()
    }
}
impl ViewTimeline {
    /// Getter of the `endOffset` attribute.
    /// [`ViewTimeline.endOffset`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTimeline/endOffset)
    pub fn end_offset(&self) -> CSSNumericValue {
        self.inner.get("endOffset").as_::<CSSNumericValue>()
    }
}
