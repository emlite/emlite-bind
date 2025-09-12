use super::*;

/// The Highlight class.
/// [`Highlight`](https://developer.mozilla.org/en-US/docs/Web/API/Highlight)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Highlight {
    inner: Any,
}

impl FromVal for Highlight {
    fn from_val(v: &Any) -> Self {
        Highlight {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Highlight {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Highlight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Highlight {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Highlight {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Highlight> for Any {
    fn from(s: Highlight) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Highlight> for Any {
    fn from(s: &Highlight) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Highlight);

impl Highlight {
    /// Getter of the `priority` attribute.
    /// [`Highlight.priority`](https://developer.mozilla.org/en-US/docs/Web/API/Highlight/priority)
    pub fn priority(&self) -> i32 {
        self.inner.get("priority").as_::<i32>()
    }

    /// Setter of the `priority` attribute.
    /// [`Highlight.priority`](https://developer.mozilla.org/en-US/docs/Web/API/Highlight/priority)
    pub fn set_priority(&mut self, value: i32) {
        self.inner.set("priority", value);
    }
}
impl Highlight {
    /// Getter of the `type` attribute.
    /// [`Highlight.type`](https://developer.mozilla.org/en-US/docs/Web/API/Highlight/type)
    pub fn type_(&self) -> HighlightType {
        self.inner.get("type").as_::<HighlightType>()
    }

    /// Setter of the `type` attribute.
    /// [`Highlight.type`](https://developer.mozilla.org/en-US/docs/Web/API/Highlight/type)
    pub fn set_type_(&mut self, value: &HighlightType) {
        self.inner.set("type", value);
    }
}

impl Highlight {
    /// The `new Highlight(..)` constructor, creating a new Highlight instance
    pub fn new(initial_ranges: &AbstractRange) -> Highlight {
        Self {
            inner: Any::global("Highlight")
                .new(&[initial_ranges.into()])
                .as_::<Any>(),
        }
    }
}
