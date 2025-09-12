use super::*;

/// The PointerTimeline class.
/// [`PointerTimeline`](https://developer.mozilla.org/en-US/docs/Web/API/PointerTimeline)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PointerTimeline {
    inner: AnimationTimeline,
}

impl FromVal for PointerTimeline {
    fn from_val(v: &Any) -> Self {
        PointerTimeline {
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

impl AsRef<Any> for PointerTimeline {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PointerTimeline {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PointerTimeline> for Any {
    fn from(s: PointerTimeline) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PointerTimeline> for Any {
    fn from(s: &PointerTimeline) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PointerTimeline);

impl PointerTimeline {
    /// Getter of the `source` attribute.
    /// [`PointerTimeline.source`](https://developer.mozilla.org/en-US/docs/Web/API/PointerTimeline/source)
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }
}
impl PointerTimeline {
    /// Getter of the `axis` attribute.
    /// [`PointerTimeline.axis`](https://developer.mozilla.org/en-US/docs/Web/API/PointerTimeline/axis)
    pub fn axis(&self) -> PointerAxis {
        self.inner.get("axis").as_::<PointerAxis>()
    }
}

impl PointerTimeline {
    /// The `new PointerTimeline(..)` constructor, creating a new PointerTimeline instance
    pub fn new0() -> PointerTimeline {
        Self {
            inner: Any::global("PointerTimeline")
                .new(&[])
                .as_::<AnimationTimeline>(),
        }
    }

    /// The `new PointerTimeline(..)` constructor, creating a new PointerTimeline instance
    pub fn new1(options: &PointerTimelineOptions) -> PointerTimeline {
        Self {
            inner: Any::global("PointerTimeline")
                .new(&[options.into()])
                .as_::<AnimationTimeline>(),
        }
    }
}
