use super::*;

/// The HighlightRegistry class.
/// [`HighlightRegistry`](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HighlightRegistry {
    inner: Any,
}

impl FromVal for HighlightRegistry {
    fn from_val(v: &Any) -> Self {
        HighlightRegistry {
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

impl core::ops::Deref for HighlightRegistry {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HighlightRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HighlightRegistry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HighlightRegistry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HighlightRegistry> for Any {
    fn from(s: HighlightRegistry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HighlightRegistry> for Any {
    fn from(s: &HighlightRegistry) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HighlightRegistry);

impl HighlightRegistry {
    /// The highlightsFromPoint method.
    /// [`HighlightRegistry.highlightsFromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/highlightsFromPoint)
    pub fn highlights_from_point0(&self, x: f32, y: f32) -> TypedArray<HighlightHitResult> {
        self.inner
            .call("highlightsFromPoint", &[x.into(), y.into()])
            .as_::<TypedArray<HighlightHitResult>>()
    }
    /// The highlightsFromPoint method.
    /// [`HighlightRegistry.highlightsFromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/highlightsFromPoint)
    pub fn highlights_from_point1(
        &self,
        x: f32,
        y: f32,
        options: &HighlightsFromPointOptions,
    ) -> TypedArray<HighlightHitResult> {
        self.inner
            .call("highlightsFromPoint", &[x.into(), y.into(), options.into()])
            .as_::<TypedArray<HighlightHitResult>>()
    }
}
