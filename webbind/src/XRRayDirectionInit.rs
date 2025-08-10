use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRRayDirectionInit {
    inner: Any,
}
impl FromVal for XRRayDirectionInit {
    fn from_val(v: &Any) -> Self {
        XRRayDirectionInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRRayDirectionInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRRayDirectionInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRRayDirectionInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRRayDirectionInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRRayDirectionInit> for Any {
    fn from(s: XRRayDirectionInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRRayDirectionInit> for Any {
    fn from(s: &XRRayDirectionInit) -> Any {
        s.inner.clone()
    }
}

impl XRRayDirectionInit {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl XRRayDirectionInit {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl XRRayDirectionInit {
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }

    pub fn set_z(&mut self, value: f64) {
        self.inner.set("z", value);
    }
}
impl XRRayDirectionInit {
    pub fn w(&self) -> f64 {
        self.inner.get("w").as_::<f64>()
    }

    pub fn set_w(&mut self, value: f64) {
        self.inner.set("w", value);
    }
}
