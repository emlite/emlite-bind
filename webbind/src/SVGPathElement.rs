use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPathDataSettings {
    inner: Any,
}
impl FromVal for SVGPathDataSettings {
    fn from_val(v: &Any) -> Self {
        SVGPathDataSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGPathDataSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGPathDataSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGPathDataSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGPathDataSettings {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGPathDataSettings> for Any {
    fn from(s: SVGPathDataSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGPathDataSettings> for Any {
    fn from(s: &SVGPathDataSettings) -> Any {
        s.inner.clone()
    }
}

impl SVGPathDataSettings {
    pub fn normalize(&self) -> bool {
        self.inner.get("normalize").as_::<bool>()
    }

    pub fn set_normalize(&mut self, value: bool) {
        self.inner.set("normalize", value);
    }
}
/// The SVGPathElement class.
/// [`SVGPathElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPathElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGPathElement {
    fn from_val(v: &Any) -> Self {
        SVGPathElement {
            inner: SVGGeometryElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGPathElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGPathElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGPathElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGPathElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGPathElement> for Any {
    fn from(s: SVGPathElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGPathElement> for Any {
    fn from(s: &SVGPathElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGPathElement);

impl SVGPathElement {
    /// Getter of the `pathLength` attribute.
    /// [`SVGPathElement.pathLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/pathLength)
    pub fn path_length(&self) -> SVGAnimatedNumber {
        self.inner.get("pathLength").as_::<SVGAnimatedNumber>()
    }
}
impl SVGPathElement {
    /// The getTotalLength method.
    /// [`SVGPathElement.getTotalLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/getTotalLength)
    pub fn get_total_length(&self) -> f32 {
        self.inner.call("getTotalLength", &[]).as_::<f32>()
    }
}
impl SVGPathElement {
    /// The getPointAtLength method.
    /// [`SVGPathElement.getPointAtLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/getPointAtLength)
    pub fn get_point_at_length(&self, distance: f32) -> DOMPoint {
        self.inner
            .call("getPointAtLength", &[distance.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGPathElement {
    /// The getPathSegmentAtLength method.
    /// [`SVGPathElement.getPathSegmentAtLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/getPathSegmentAtLength)
    pub fn get_path_segment_at_length(&self, distance: f32) -> SVGPathSegment {
        self.inner
            .call("getPathSegmentAtLength", &[distance.into()])
            .as_::<SVGPathSegment>()
    }
}
impl SVGPathElement {
    /// The getPathData method.
    /// [`SVGPathElement.getPathData`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/getPathData)
    pub fn get_path_data0(&self) -> Sequence<SVGPathSegment> {
        self.inner
            .call("getPathData", &[])
            .as_::<Sequence<SVGPathSegment>>()
    }
    /// The getPathData method.
    /// [`SVGPathElement.getPathData`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/getPathData)
    pub fn get_path_data1(&self, settings: &SVGPathDataSettings) -> Sequence<SVGPathSegment> {
        self.inner
            .call("getPathData", &[settings.into()])
            .as_::<Sequence<SVGPathSegment>>()
    }
}
impl SVGPathElement {
    /// The setPathData method.
    /// [`SVGPathElement.setPathData`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/setPathData)
    pub fn set_path_data(&self, path_data: &Sequence<SVGPathSegment>) -> Undefined {
        self.inner
            .call("setPathData", &[path_data.into()])
            .as_::<Undefined>()
    }
}
