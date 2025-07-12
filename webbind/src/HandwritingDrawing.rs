use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for HandwritingPrediction {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HandwritingPrediction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HandwritingPrediction> for emlite::Val {
    fn from(s: HandwritingPrediction) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HandwritingPrediction {
    pub fn text(&self) -> jsbind::DOMString {
        self.inner.get("text").as_::<jsbind::DOMString>()
    }

    pub fn set_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("text", value);
    }
}
impl HandwritingPrediction {
    pub fn segmentation_result(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("segmentationResult")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_segmentation_result(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("segmentationResult", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for HandwritingDrawing {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HandwritingDrawing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HandwritingDrawing> for emlite::Val {
    fn from(s: HandwritingDrawing) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HandwritingDrawing {
    pub fn add_stroke(&self, stroke: HandwritingStroke) -> jsbind::Undefined {
        self.inner
            .call("addStroke", &[stroke.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HandwritingDrawing {
    pub fn remove_stroke(&self, stroke: HandwritingStroke) -> jsbind::Undefined {
        self.inner
            .call("removeStroke", &[stroke.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HandwritingDrawing {
    pub fn clear(&self) -> jsbind::Undefined {
        self.inner.call("clear", &[]).as_::<jsbind::Undefined>()
    }
}
impl HandwritingDrawing {
    pub fn get_strokes(&self) -> jsbind::Sequence<HandwritingStroke> {
        self.inner
            .call("getStrokes", &[])
            .as_::<jsbind::Sequence<HandwritingStroke>>()
    }
}
impl HandwritingDrawing {
    pub fn get_prediction(&self) -> jsbind::Promise {
        self.inner
            .call("getPrediction", &[])
            .as_::<jsbind::Promise>()
    }
}
