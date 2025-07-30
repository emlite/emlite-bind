use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingPoint {
    inner: Any,
}
impl FromVal for HandwritingPoint {
    fn from_val(v: &Any) -> Self {
        HandwritingPoint { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HandwritingPoint {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HandwritingPoint {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HandwritingPoint {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HandwritingPoint> for Any {
    fn from(s: HandwritingPoint) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HandwritingPoint> for Any {
    fn from(s: &HandwritingPoint) -> Any {
        s.inner.clone()
    }
}

impl HandwritingPoint {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl HandwritingPoint {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl HandwritingPoint {
    pub fn t(&self) -> Any {
        self.inner.get("t").as_::<Any>()
    }

    pub fn set_t(&mut self, value: &Any) {
        self.inner.set("t", value);
    }
}
/// The HandwritingStroke class.
/// [`HandwritingStroke`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingStroke)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingStroke {
    inner: Any,
}
impl FromVal for HandwritingStroke {
    fn from_val(v: &Any) -> Self {
        HandwritingStroke {
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
impl core::ops::Deref for HandwritingStroke {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingStroke {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HandwritingStroke {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HandwritingStroke {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HandwritingStroke> for Any {
    fn from(s: HandwritingStroke) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HandwritingStroke> for Any {
    fn from(s: &HandwritingStroke) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HandwritingStroke);

impl HandwritingStroke {
    /// The `new HandwritingStroke(..)` constructor, creating a new HandwritingStroke instance
    pub fn new() -> HandwritingStroke {
        Self {
            inner: Any::global("HandwritingStroke").new(&[]).as_::<Any>(),
        }
    }
}
impl HandwritingStroke {
    /// The addPoint method.
    /// [`HandwritingStroke.addPoint`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingStroke/addPoint)
    pub fn add_point(&self, point: &HandwritingPoint) -> Undefined {
        self.inner
            .call("addPoint", &[point.into()])
            .as_::<Undefined>()
    }
}
impl HandwritingStroke {
    /// The getPoints method.
    /// [`HandwritingStroke.getPoints`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingStroke/getPoints)
    pub fn get_points(&self) -> TypedArray<HandwritingPoint> {
        self.inner
            .call("getPoints", &[])
            .as_::<TypedArray<HandwritingPoint>>()
    }
}
impl HandwritingStroke {
    /// The clear method.
    /// [`HandwritingStroke.clear`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingStroke/clear)
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
