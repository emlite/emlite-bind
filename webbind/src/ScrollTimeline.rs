use super::*;

/// The ScrollTimeline class.
/// [`ScrollTimeline`](https://developer.mozilla.org/en-US/docs/Web/API/ScrollTimeline)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScrollTimeline {
    inner: AnimationTimeline,
}
impl FromVal for ScrollTimeline {
    fn from_val(v: &Any) -> Self {
        ScrollTimeline {
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
impl AsRef<Any> for ScrollTimeline {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ScrollTimeline {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ScrollTimeline> for Any {
    fn from(s: ScrollTimeline) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ScrollTimeline> for Any {
    fn from(s: &ScrollTimeline) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ScrollTimeline);

impl ScrollTimeline {
    /// The `new ScrollTimeline(..)` constructor, creating a new ScrollTimeline instance
    pub fn new0() -> ScrollTimeline {
        Self {
            inner: Any::global("ScrollTimeline")
                .new(&[])
                .as_::<AnimationTimeline>(),
        }
    }

    /// The `new ScrollTimeline(..)` constructor, creating a new ScrollTimeline instance
    pub fn new1(options: &ScrollTimelineOptions) -> ScrollTimeline {
        Self {
            inner: Any::global("ScrollTimeline")
                .new(&[options.into()])
                .as_::<AnimationTimeline>(),
        }
    }
}
impl ScrollTimeline {
    /// Getter of the `source` attribute.
    /// [`ScrollTimeline.source`](https://developer.mozilla.org/en-US/docs/Web/API/ScrollTimeline/source)
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }
}
impl ScrollTimeline {
    /// Getter of the `axis` attribute.
    /// [`ScrollTimeline.axis`](https://developer.mozilla.org/en-US/docs/Web/API/ScrollTimeline/axis)
    pub fn axis(&self) -> ScrollAxis {
        self.inner.get("axis").as_::<ScrollAxis>()
    }
}
