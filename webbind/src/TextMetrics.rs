use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextMetrics {
    inner: emlite::Val,
}
impl FromVal for TextMetrics {
    fn from_val(v: &emlite::Val) -> Self {
        TextMetrics {
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
impl core::ops::Deref for TextMetrics {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextMetrics> for emlite::Val {
    fn from(s: TextMetrics) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextMetrics {
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn actual_bounding_box_left(&self) -> f64 {
        self.inner.get("actualBoundingBoxLeft").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn actual_bounding_box_right(&self) -> f64 {
        self.inner.get("actualBoundingBoxRight").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn font_bounding_box_ascent(&self) -> f64 {
        self.inner.get("fontBoundingBoxAscent").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn font_bounding_box_descent(&self) -> f64 {
        self.inner.get("fontBoundingBoxDescent").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn actual_bounding_box_ascent(&self) -> f64 {
        self.inner.get("actualBoundingBoxAscent").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn actual_bounding_box_descent(&self) -> f64 {
        self.inner.get("actualBoundingBoxDescent").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn em_height_ascent(&self) -> f64 {
        self.inner.get("emHeightAscent").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn em_height_descent(&self) -> f64 {
        self.inner.get("emHeightDescent").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn hanging_baseline(&self) -> f64 {
        self.inner.get("hangingBaseline").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn alphabetic_baseline(&self) -> f64 {
        self.inner.get("alphabeticBaseline").as_::<f64>()
    }
}
impl TextMetrics {
    pub fn ideographic_baseline(&self) -> f64 {
        self.inner.get("ideographicBaseline").as_::<f64>()
    }
}
