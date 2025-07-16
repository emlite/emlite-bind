use super::*;

/// The TextMetrics class.
/// [`TextMetrics`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextMetrics {
    inner: Any,
}
impl FromVal for TextMetrics {
    fn from_val(v: &Any) -> Self {
        TextMetrics {
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
impl core::ops::Deref for TextMetrics {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextMetrics {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextMetrics {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextMetrics> for Any {
    fn from(s: TextMetrics) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextMetrics> for Any {
    fn from(s: &TextMetrics) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextMetrics);

impl TextMetrics {
    /// Getter of the `width` attribute.
    /// [`TextMetrics.width`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/width)
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `actualBoundingBoxLeft` attribute.
    /// [`TextMetrics.actualBoundingBoxLeft`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxLeft)
    pub fn actual_bounding_box_left(&self) -> f64 {
        self.inner.get("actualBoundingBoxLeft").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `actualBoundingBoxRight` attribute.
    /// [`TextMetrics.actualBoundingBoxRight`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxRight)
    pub fn actual_bounding_box_right(&self) -> f64 {
        self.inner.get("actualBoundingBoxRight").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `fontBoundingBoxAscent` attribute.
    /// [`TextMetrics.fontBoundingBoxAscent`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/fontBoundingBoxAscent)
    pub fn font_bounding_box_ascent(&self) -> f64 {
        self.inner.get("fontBoundingBoxAscent").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `fontBoundingBoxDescent` attribute.
    /// [`TextMetrics.fontBoundingBoxDescent`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/fontBoundingBoxDescent)
    pub fn font_bounding_box_descent(&self) -> f64 {
        self.inner.get("fontBoundingBoxDescent").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `actualBoundingBoxAscent` attribute.
    /// [`TextMetrics.actualBoundingBoxAscent`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxAscent)
    pub fn actual_bounding_box_ascent(&self) -> f64 {
        self.inner.get("actualBoundingBoxAscent").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `actualBoundingBoxDescent` attribute.
    /// [`TextMetrics.actualBoundingBoxDescent`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxDescent)
    pub fn actual_bounding_box_descent(&self) -> f64 {
        self.inner.get("actualBoundingBoxDescent").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `emHeightAscent` attribute.
    /// [`TextMetrics.emHeightAscent`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/emHeightAscent)
    pub fn em_height_ascent(&self) -> f64 {
        self.inner.get("emHeightAscent").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `emHeightDescent` attribute.
    /// [`TextMetrics.emHeightDescent`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/emHeightDescent)
    pub fn em_height_descent(&self) -> f64 {
        self.inner.get("emHeightDescent").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `hangingBaseline` attribute.
    /// [`TextMetrics.hangingBaseline`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/hangingBaseline)
    pub fn hanging_baseline(&self) -> f64 {
        self.inner.get("hangingBaseline").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `alphabeticBaseline` attribute.
    /// [`TextMetrics.alphabeticBaseline`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/alphabeticBaseline)
    pub fn alphabetic_baseline(&self) -> f64 {
        self.inner.get("alphabeticBaseline").as_::<f64>()
    }
}
impl TextMetrics {
    /// Getter of the `ideographicBaseline` attribute.
    /// [`TextMetrics.ideographicBaseline`](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/ideographicBaseline)
    pub fn ideographic_baseline(&self) -> f64 {
        self.inner.get("ideographicBaseline").as_::<f64>()
    }
}
