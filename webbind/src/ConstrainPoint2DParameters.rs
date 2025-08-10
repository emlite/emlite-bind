use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConstrainPoint2DParameters {
    inner: Any,
}
impl FromVal for ConstrainPoint2DParameters {
    fn from_val(v: &Any) -> Self {
        ConstrainPoint2DParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ConstrainPoint2DParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ConstrainPoint2DParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ConstrainPoint2DParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ConstrainPoint2DParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ConstrainPoint2DParameters> for Any {
    fn from(s: ConstrainPoint2DParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ConstrainPoint2DParameters> for Any {
    fn from(s: &ConstrainPoint2DParameters) -> Any {
        s.inner.clone()
    }
}

impl ConstrainPoint2DParameters {
    pub fn exact(&self) -> TypedArray<Point2D> {
        self.inner.get("exact").as_::<TypedArray<Point2D>>()
    }

    pub fn set_exact(&mut self, value: &TypedArray<Point2D>) {
        self.inner.set("exact", value);
    }
}
impl ConstrainPoint2DParameters {
    pub fn ideal(&self) -> TypedArray<Point2D> {
        self.inner.get("ideal").as_::<TypedArray<Point2D>>()
    }

    pub fn set_ideal(&mut self, value: &TypedArray<Point2D>) {
        self.inner.set("ideal", value);
    }
}
