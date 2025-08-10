use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Point2D {
    inner: Any,
}
impl FromVal for Point2D {
    fn from_val(v: &Any) -> Self {
        Point2D { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Point2D {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Point2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Point2D {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Point2D {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Point2D> for Any {
    fn from(s: Point2D) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Point2D> for Any {
    fn from(s: &Point2D) -> Any {
        s.inner.clone()
    }
}

impl Point2D {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl Point2D {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
