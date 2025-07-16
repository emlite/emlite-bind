use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HighlightHitResult {
    inner: Any,
}
impl FromVal for HighlightHitResult {
    fn from_val(v: &Any) -> Self {
        HighlightHitResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HighlightHitResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HighlightHitResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HighlightHitResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HighlightHitResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HighlightHitResult> for Any {
    fn from(s: HighlightHitResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HighlightHitResult> for Any {
    fn from(s: &HighlightHitResult) -> Any {
        s.inner.clone()
    }
}

impl HighlightHitResult {
    pub fn highlight(&self) -> Highlight {
        self.inner.get("highlight").as_::<Highlight>()
    }

    pub fn set_highlight(&mut self, value: &Highlight) {
        self.inner.set("highlight", value);
    }
}
impl HighlightHitResult {
    pub fn ranges(&self) -> Sequence<AbstractRange> {
        self.inner.get("ranges").as_::<Sequence<AbstractRange>>()
    }

    pub fn set_ranges(&mut self, value: &Sequence<AbstractRange>) {
        self.inner.set("ranges", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HighlightsFromPointOptions {
    inner: Any,
}
impl FromVal for HighlightsFromPointOptions {
    fn from_val(v: &Any) -> Self {
        HighlightsFromPointOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HighlightsFromPointOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HighlightsFromPointOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HighlightsFromPointOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HighlightsFromPointOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HighlightsFromPointOptions> for Any {
    fn from(s: HighlightsFromPointOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HighlightsFromPointOptions> for Any {
    fn from(s: &HighlightsFromPointOptions) -> Any {
        s.inner.clone()
    }
}

impl HighlightsFromPointOptions {
    pub fn shadow_roots(&self) -> Sequence<ShadowRoot> {
        self.inner.get("shadowRoots").as_::<Sequence<ShadowRoot>>()
    }

    pub fn set_shadow_roots(&mut self, value: &Sequence<ShadowRoot>) {
        self.inner.set("shadowRoots", value);
    }
}
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
    pub fn highlights_from_point0(&self, x: f32, y: f32) -> Sequence<HighlightHitResult> {
        self.inner
            .call("highlightsFromPoint", &[x.into(), y.into()])
            .as_::<Sequence<HighlightHitResult>>()
    }
    /// The highlightsFromPoint method.
    /// [`HighlightRegistry.highlightsFromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/highlightsFromPoint)
    pub fn highlights_from_point1(
        &self,
        x: f32,
        y: f32,
        options: &HighlightsFromPointOptions,
    ) -> Sequence<HighlightHitResult> {
        self.inner
            .call("highlightsFromPoint", &[x.into(), y.into(), options.into()])
            .as_::<Sequence<HighlightHitResult>>()
    }
}
