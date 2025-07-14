use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontMetrics {
    inner: emlite::Val,
}
impl FromVal for FontMetrics {
    fn from_val(v: &emlite::Val) -> Self {
        FontMetrics {
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
impl core::ops::Deref for FontMetrics {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FontMetrics {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FontMetrics {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FontMetrics> for emlite::Val {
    fn from(s: FontMetrics) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FontMetrics);

impl FontMetrics {
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }
}
impl FontMetrics {
    pub fn advances(&self) -> jsbind::FrozenArray<f64> {
        self.inner.get("advances").as_::<jsbind::FrozenArray<f64>>()
    }
}
impl FontMetrics {
    pub fn bounding_box_left(&self) -> f64 {
        self.inner.get("boundingBoxLeft").as_::<f64>()
    }
}
impl FontMetrics {
    pub fn bounding_box_right(&self) -> f64 {
        self.inner.get("boundingBoxRight").as_::<f64>()
    }
}
impl FontMetrics {
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }
}
impl FontMetrics {
    pub fn em_height_ascent(&self) -> f64 {
        self.inner.get("emHeightAscent").as_::<f64>()
    }
}
impl FontMetrics {
    pub fn em_height_descent(&self) -> f64 {
        self.inner.get("emHeightDescent").as_::<f64>()
    }
}
impl FontMetrics {
    pub fn bounding_box_ascent(&self) -> f64 {
        self.inner.get("boundingBoxAscent").as_::<f64>()
    }
}
impl FontMetrics {
    pub fn bounding_box_descent(&self) -> f64 {
        self.inner.get("boundingBoxDescent").as_::<f64>()
    }
}
impl FontMetrics {
    pub fn font_bounding_box_ascent(&self) -> f64 {
        self.inner.get("fontBoundingBoxAscent").as_::<f64>()
    }
}
impl FontMetrics {
    pub fn font_bounding_box_descent(&self) -> f64 {
        self.inner.get("fontBoundingBoxDescent").as_::<f64>()
    }
}
impl FontMetrics {
    pub fn dominant_baseline(&self) -> Baseline {
        self.inner.get("dominantBaseline").as_::<Baseline>()
    }
}
impl FontMetrics {
    pub fn baselines(&self) -> jsbind::FrozenArray<Baseline> {
        self.inner
            .get("baselines")
            .as_::<jsbind::FrozenArray<Baseline>>()
    }
}
impl FontMetrics {
    pub fn fonts(&self) -> jsbind::FrozenArray<Font> {
        self.inner.get("fonts").as_::<jsbind::FrozenArray<Font>>()
    }
}
