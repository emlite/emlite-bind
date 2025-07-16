use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMPointInit {
    inner: Any,
}
impl FromVal for DOMPointInit {
    fn from_val(v: &Any) -> Self {
        DOMPointInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DOMPointInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMPointInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DOMPointInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DOMPointInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DOMPointInit> for Any {
    fn from(s: DOMPointInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DOMPointInit> for Any {
    fn from(s: &DOMPointInit) -> Any {
        s.inner.clone()
    }
}

impl DOMPointInit {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl DOMPointInit {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl DOMPointInit {
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }

    pub fn set_z(&mut self, value: f64) {
        self.inner.set("z", value);
    }
}
impl DOMPointInit {
    pub fn w(&self) -> f64 {
        self.inner.get("w").as_::<f64>()
    }

    pub fn set_w(&mut self, value: f64) {
        self.inner.set("w", value);
    }
}
/// The SVGGeometryElement class.
/// [`SVGGeometryElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGGeometryElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGGeometryElement {
    fn from_val(v: &Any) -> Self {
        SVGGeometryElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGGeometryElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGGeometryElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGGeometryElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGGeometryElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGGeometryElement> for Any {
    fn from(s: SVGGeometryElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGGeometryElement> for Any {
    fn from(s: &SVGGeometryElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGGeometryElement);

impl SVGGeometryElement {
    /// Getter of the `pathLength` attribute.
    /// [`SVGGeometryElement.pathLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/pathLength)
    pub fn path_length(&self) -> SVGAnimatedNumber {
        self.inner.get("pathLength").as_::<SVGAnimatedNumber>()
    }
}
impl SVGGeometryElement {
    /// The isPointInFill method.
    /// [`SVGGeometryElement.isPointInFill`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInFill)
    pub fn is_point_in_fill0(&self) -> bool {
        self.inner.call("isPointInFill", &[]).as_::<bool>()
    }
    /// The isPointInFill method.
    /// [`SVGGeometryElement.isPointInFill`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInFill)
    pub fn is_point_in_fill1(&self, point: &DOMPointInit) -> bool {
        self.inner
            .call("isPointInFill", &[point.into()])
            .as_::<bool>()
    }
}
impl SVGGeometryElement {
    /// The isPointInStroke method.
    /// [`SVGGeometryElement.isPointInStroke`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInStroke)
    pub fn is_point_in_stroke0(&self) -> bool {
        self.inner.call("isPointInStroke", &[]).as_::<bool>()
    }
    /// The isPointInStroke method.
    /// [`SVGGeometryElement.isPointInStroke`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInStroke)
    pub fn is_point_in_stroke1(&self, point: &DOMPointInit) -> bool {
        self.inner
            .call("isPointInStroke", &[point.into()])
            .as_::<bool>()
    }
}
impl SVGGeometryElement {
    /// The getTotalLength method.
    /// [`SVGGeometryElement.getTotalLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getTotalLength)
    pub fn get_total_length(&self) -> f32 {
        self.inner.call("getTotalLength", &[]).as_::<f32>()
    }
}
impl SVGGeometryElement {
    /// The getPointAtLength method.
    /// [`SVGGeometryElement.getPointAtLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getPointAtLength)
    pub fn get_point_at_length(&self, distance: f32) -> DOMPoint {
        self.inner
            .call("getPointAtLength", &[distance.into()])
            .as_::<DOMPoint>()
    }
}
