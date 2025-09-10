use super::*;

/// The SVGBoundingBoxOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGBoundingBoxOptions {
    inner: Any,
}

impl FromVal for SVGBoundingBoxOptions {
    fn from_val(v: &Any) -> Self {
        SVGBoundingBoxOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGBoundingBoxOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGBoundingBoxOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGBoundingBoxOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGBoundingBoxOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGBoundingBoxOptions> for Any {
    fn from(s: SVGBoundingBoxOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGBoundingBoxOptions> for Any {
    fn from(s: &SVGBoundingBoxOptions) -> Any {
        s.inner.clone()
    }
}

impl SVGBoundingBoxOptions {
    /// Getter of the `fill` attribute.
    pub fn fill(&self) -> bool {
        self.inner.get("fill").as_::<bool>()
    }

    /// Setter of the `fill` attribute.
    pub fn set_fill(&mut self, value: bool) {
        self.inner.set("fill", value);
    }
}
impl SVGBoundingBoxOptions {
    /// Getter of the `stroke` attribute.
    pub fn stroke(&self) -> bool {
        self.inner.get("stroke").as_::<bool>()
    }

    /// Setter of the `stroke` attribute.
    pub fn set_stroke(&mut self, value: bool) {
        self.inner.set("stroke", value);
    }
}
impl SVGBoundingBoxOptions {
    /// Getter of the `markers` attribute.
    pub fn markers(&self) -> bool {
        self.inner.get("markers").as_::<bool>()
    }

    /// Setter of the `markers` attribute.
    pub fn set_markers(&mut self, value: bool) {
        self.inner.set("markers", value);
    }
}
impl SVGBoundingBoxOptions {
    /// Getter of the `clipped` attribute.
    pub fn clipped(&self) -> bool {
        self.inner.get("clipped").as_::<bool>()
    }

    /// Setter of the `clipped` attribute.
    pub fn set_clipped(&mut self, value: bool) {
        self.inner.set("clipped", value);
    }
}
