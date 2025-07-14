use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HighlightHitResult {
    inner: emlite::Val,
}
impl FromVal for HighlightHitResult {
    fn from_val(v: &emlite::Val) -> Self {
        HighlightHitResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HighlightHitResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HighlightHitResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HighlightHitResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HighlightHitResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HighlightHitResult> for emlite::Val {
    fn from(s: HighlightHitResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HighlightHitResult {
    pub fn highlight(&self) -> Highlight {
        self.inner.get("highlight").as_::<Highlight>()
    }

    pub fn set_highlight(&mut self, value: Highlight) {
        self.inner.set("highlight", value);
    }
}
impl HighlightHitResult {
    pub fn ranges(&self) -> jsbind::Sequence<AbstractRange> {
        self.inner
            .get("ranges")
            .as_::<jsbind::Sequence<AbstractRange>>()
    }

    pub fn set_ranges(&mut self, value: jsbind::Sequence<AbstractRange>) {
        self.inner.set("ranges", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HighlightsFromPointOptions {
    inner: emlite::Val,
}
impl FromVal for HighlightsFromPointOptions {
    fn from_val(v: &emlite::Val) -> Self {
        HighlightsFromPointOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HighlightsFromPointOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HighlightsFromPointOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HighlightsFromPointOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HighlightsFromPointOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HighlightsFromPointOptions> for emlite::Val {
    fn from(s: HighlightsFromPointOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HighlightsFromPointOptions {
    pub fn shadow_roots(&self) -> jsbind::Sequence<ShadowRoot> {
        self.inner
            .get("shadowRoots")
            .as_::<jsbind::Sequence<ShadowRoot>>()
    }

    pub fn set_shadow_roots(&mut self, value: jsbind::Sequence<ShadowRoot>) {
        self.inner.set("shadowRoots", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HighlightRegistry {
    inner: emlite::Val,
}
impl FromVal for HighlightRegistry {
    fn from_val(v: &emlite::Val) -> Self {
        HighlightRegistry {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HighlightRegistry {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HighlightRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HighlightRegistry {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HighlightRegistry {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HighlightRegistry> for emlite::Val {
    fn from(s: HighlightRegistry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HighlightRegistry);

impl HighlightRegistry {
    pub fn highlights_from_point0(&self, x: f32, y: f32) -> jsbind::Sequence<HighlightHitResult> {
        self.inner
            .call("highlightsFromPoint", &[x.into(), y.into()])
            .as_::<jsbind::Sequence<HighlightHitResult>>()
    }

    pub fn highlights_from_point1(
        &self,
        x: f32,
        y: f32,
        options: HighlightsFromPointOptions,
    ) -> jsbind::Sequence<HighlightHitResult> {
        self.inner
            .call("highlightsFromPoint", &[x.into(), y.into(), options.into()])
            .as_::<jsbind::Sequence<HighlightHitResult>>()
    }
}
