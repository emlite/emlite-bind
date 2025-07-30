use super::*;

/// The FontMetrics class.
/// [`FontMetrics`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontMetrics {
    inner: Any,
}
impl FromVal for FontMetrics {
    fn from_val(v: &Any) -> Self {
        FontMetrics {
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
impl core::ops::Deref for FontMetrics {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FontMetrics {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FontMetrics {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FontMetrics> for Any {
    fn from(s: FontMetrics) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FontMetrics> for Any {
    fn from(s: &FontMetrics) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FontMetrics);

impl FontMetrics {
    /// Getter of the `width` attribute.
    /// [`FontMetrics.width`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/width)
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }
}
impl FontMetrics {
    /// Getter of the `advances` attribute.
    /// [`FontMetrics.advances`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/advances)
    pub fn advances(&self) -> TypedArray<f64> {
        self.inner.get("advances").as_::<TypedArray<f64>>()
    }
}
impl FontMetrics {
    /// Getter of the `boundingBoxLeft` attribute.
    /// [`FontMetrics.boundingBoxLeft`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/boundingBoxLeft)
    pub fn bounding_box_left(&self) -> f64 {
        self.inner.get("boundingBoxLeft").as_::<f64>()
    }
}
impl FontMetrics {
    /// Getter of the `boundingBoxRight` attribute.
    /// [`FontMetrics.boundingBoxRight`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/boundingBoxRight)
    pub fn bounding_box_right(&self) -> f64 {
        self.inner.get("boundingBoxRight").as_::<f64>()
    }
}
impl FontMetrics {
    /// Getter of the `height` attribute.
    /// [`FontMetrics.height`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/height)
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }
}
impl FontMetrics {
    /// Getter of the `emHeightAscent` attribute.
    /// [`FontMetrics.emHeightAscent`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/emHeightAscent)
    pub fn em_height_ascent(&self) -> f64 {
        self.inner.get("emHeightAscent").as_::<f64>()
    }
}
impl FontMetrics {
    /// Getter of the `emHeightDescent` attribute.
    /// [`FontMetrics.emHeightDescent`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/emHeightDescent)
    pub fn em_height_descent(&self) -> f64 {
        self.inner.get("emHeightDescent").as_::<f64>()
    }
}
impl FontMetrics {
    /// Getter of the `boundingBoxAscent` attribute.
    /// [`FontMetrics.boundingBoxAscent`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/boundingBoxAscent)
    pub fn bounding_box_ascent(&self) -> f64 {
        self.inner.get("boundingBoxAscent").as_::<f64>()
    }
}
impl FontMetrics {
    /// Getter of the `boundingBoxDescent` attribute.
    /// [`FontMetrics.boundingBoxDescent`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/boundingBoxDescent)
    pub fn bounding_box_descent(&self) -> f64 {
        self.inner.get("boundingBoxDescent").as_::<f64>()
    }
}
impl FontMetrics {
    /// Getter of the `fontBoundingBoxAscent` attribute.
    /// [`FontMetrics.fontBoundingBoxAscent`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/fontBoundingBoxAscent)
    pub fn font_bounding_box_ascent(&self) -> f64 {
        self.inner.get("fontBoundingBoxAscent").as_::<f64>()
    }
}
impl FontMetrics {
    /// Getter of the `fontBoundingBoxDescent` attribute.
    /// [`FontMetrics.fontBoundingBoxDescent`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/fontBoundingBoxDescent)
    pub fn font_bounding_box_descent(&self) -> f64 {
        self.inner.get("fontBoundingBoxDescent").as_::<f64>()
    }
}
impl FontMetrics {
    /// Getter of the `dominantBaseline` attribute.
    /// [`FontMetrics.dominantBaseline`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/dominantBaseline)
    pub fn dominant_baseline(&self) -> Baseline {
        self.inner.get("dominantBaseline").as_::<Baseline>()
    }
}
impl FontMetrics {
    /// Getter of the `baselines` attribute.
    /// [`FontMetrics.baselines`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/baselines)
    pub fn baselines(&self) -> TypedArray<Baseline> {
        self.inner.get("baselines").as_::<TypedArray<Baseline>>()
    }
}
impl FontMetrics {
    /// Getter of the `fonts` attribute.
    /// [`FontMetrics.fonts`](https://developer.mozilla.org/en-US/docs/Web/API/FontMetrics/fonts)
    pub fn fonts(&self) -> TypedArray<Font> {
        self.inner.get("fonts").as_::<TypedArray<Font>>()
    }
}
