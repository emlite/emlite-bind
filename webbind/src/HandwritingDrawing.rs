use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingPrediction {
    inner: emlite::Val,
}
impl FromVal for HandwritingPrediction {
    fn from_val(v: &emlite::Val) -> Self {
        HandwritingPrediction { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HandwritingPrediction {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingPrediction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HandwritingPrediction {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HandwritingPrediction {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HandwritingPrediction> for emlite::Val {
    fn from(s: HandwritingPrediction) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HandwritingPrediction> for emlite::Val {
    fn from(s: &HandwritingPrediction) -> emlite::Val {
        s.inner.clone()
    }
}

impl HandwritingPrediction {
    pub fn text(&self) -> String {
        self.inner.get("text").as_::<String>()
    }

    pub fn set_text(&mut self, value: &str) {
        self.inner.set("text", value);
    }
}
impl HandwritingPrediction {
    pub fn segmentation_result(&self) -> Sequence<Any> {
        self.inner.get("segmentationResult").as_::<Sequence<Any>>()
    }

    pub fn set_segmentation_result(&mut self, value: &Sequence<Any>) {
        self.inner.set("segmentationResult", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingDrawing {
    inner: emlite::Val,
}
impl FromVal for HandwritingDrawing {
    fn from_val(v: &emlite::Val) -> Self {
        HandwritingDrawing {
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
impl core::ops::Deref for HandwritingDrawing {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingDrawing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HandwritingDrawing {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HandwritingDrawing {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HandwritingDrawing> for emlite::Val {
    fn from(s: HandwritingDrawing) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HandwritingDrawing> for emlite::Val {
    fn from(s: &HandwritingDrawing) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HandwritingDrawing);

impl HandwritingDrawing {
    pub fn add_stroke(&self, stroke: &HandwritingStroke) -> Undefined {
        self.inner
            .call("addStroke", &[stroke.into()])
            .as_::<Undefined>()
    }
}
impl HandwritingDrawing {
    pub fn remove_stroke(&self, stroke: &HandwritingStroke) -> Undefined {
        self.inner
            .call("removeStroke", &[stroke.into()])
            .as_::<Undefined>()
    }
}
impl HandwritingDrawing {
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
impl HandwritingDrawing {
    pub fn get_strokes(&self) -> Sequence<HandwritingStroke> {
        self.inner
            .call("getStrokes", &[])
            .as_::<Sequence<HandwritingStroke>>()
    }
}
impl HandwritingDrawing {
    pub fn get_prediction(&self) -> Promise {
        self.inner.call("getPrediction", &[]).as_::<Promise>()
    }
}
