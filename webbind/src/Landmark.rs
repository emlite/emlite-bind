use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Landmark {
    inner: Any,
}
impl FromVal for Landmark {
    fn from_val(v: &Any) -> Self {
        Landmark { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Landmark {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Landmark {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Landmark {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Landmark {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Landmark> for Any {
    fn from(s: Landmark) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Landmark> for Any {
    fn from(s: &Landmark) -> Any {
        s.inner.clone()
    }
}

impl Landmark {
    pub fn locations(&self) -> TypedArray<Point2D> {
        self.inner.get("locations").as_::<TypedArray<Point2D>>()
    }

    pub fn set_locations(&mut self, value: &TypedArray<Point2D>) {
        self.inner.set("locations", value);
    }
}
impl Landmark {
    pub fn type_(&self) -> LandmarkType {
        self.inner.get("type").as_::<LandmarkType>()
    }

    pub fn set_type_(&mut self, value: &LandmarkType) {
        self.inner.set("type", value);
    }
}
