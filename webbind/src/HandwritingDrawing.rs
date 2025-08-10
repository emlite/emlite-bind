use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingPrediction {
    inner: Any,
}
impl FromVal for HandwritingPrediction {
    fn from_val(v: &Any) -> Self {
        HandwritingPrediction { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HandwritingPrediction {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingPrediction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HandwritingPrediction {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HandwritingPrediction {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HandwritingPrediction> for Any {
    fn from(s: HandwritingPrediction) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HandwritingPrediction> for Any {
    fn from(s: &HandwritingPrediction) -> Any {
        s.inner.clone()
    }
}

impl HandwritingPrediction {
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }

    pub fn set_text(&mut self, value: &JsString) {
        self.inner.set("text", value);
    }
}
impl HandwritingPrediction {
    pub fn segmentation_result(&self) -> TypedArray<HandwritingSegment> {
        self.inner
            .get("segmentationResult")
            .as_::<TypedArray<HandwritingSegment>>()
    }

    pub fn set_segmentation_result(&mut self, value: &TypedArray<HandwritingSegment>) {
        self.inner.set("segmentationResult", value);
    }
}
/// The HandwritingDrawing class.
/// [`HandwritingDrawing`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingDrawing)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingDrawing {
    inner: Any,
}
impl FromVal for HandwritingDrawing {
    fn from_val(v: &Any) -> Self {
        HandwritingDrawing {
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
impl core::ops::Deref for HandwritingDrawing {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingDrawing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HandwritingDrawing {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HandwritingDrawing {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HandwritingDrawing> for Any {
    fn from(s: HandwritingDrawing) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HandwritingDrawing> for Any {
    fn from(s: &HandwritingDrawing) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HandwritingDrawing);

impl HandwritingDrawing {
    /// The addStroke method.
    /// [`HandwritingDrawing.addStroke`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingDrawing/addStroke)
    pub fn add_stroke(&self, stroke: &HandwritingStroke) -> Undefined {
        self.inner
            .call("addStroke", &[stroke.into()])
            .as_::<Undefined>()
    }
}
impl HandwritingDrawing {
    /// The removeStroke method.
    /// [`HandwritingDrawing.removeStroke`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingDrawing/removeStroke)
    pub fn remove_stroke(&self, stroke: &HandwritingStroke) -> Undefined {
        self.inner
            .call("removeStroke", &[stroke.into()])
            .as_::<Undefined>()
    }
}
impl HandwritingDrawing {
    /// The clear method.
    /// [`HandwritingDrawing.clear`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingDrawing/clear)
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
impl HandwritingDrawing {
    /// The getStrokes method.
    /// [`HandwritingDrawing.getStrokes`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingDrawing/getStrokes)
    pub fn get_strokes(&self) -> TypedArray<HandwritingStroke> {
        self.inner
            .call("getStrokes", &[])
            .as_::<TypedArray<HandwritingStroke>>()
    }
}
impl HandwritingDrawing {
    /// The getPrediction method.
    /// [`HandwritingDrawing.getPrediction`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingDrawing/getPrediction)
    pub fn get_prediction(&self) -> Promise<TypedArray<HandwritingPrediction>> {
        self.inner
            .call("getPrediction", &[])
            .as_::<Promise<TypedArray<HandwritingPrediction>>>()
    }
}
