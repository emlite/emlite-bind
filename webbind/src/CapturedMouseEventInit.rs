use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CapturedMouseEventInit {
    inner: Any,
}
impl FromVal for CapturedMouseEventInit {
    fn from_val(v: &Any) -> Self {
        CapturedMouseEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CapturedMouseEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CapturedMouseEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CapturedMouseEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CapturedMouseEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CapturedMouseEventInit> for Any {
    fn from(s: CapturedMouseEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CapturedMouseEventInit> for Any {
    fn from(s: &CapturedMouseEventInit) -> Any {
        s.inner.clone()
    }
}

impl CapturedMouseEventInit {
    pub fn surface_x(&self) -> i32 {
        self.inner.get("surfaceX").as_::<i32>()
    }

    pub fn set_surface_x(&mut self, value: i32) {
        self.inner.set("surfaceX", value);
    }
}
impl CapturedMouseEventInit {
    pub fn surface_y(&self) -> i32 {
        self.inner.get("surfaceY").as_::<i32>()
    }

    pub fn set_surface_y(&mut self, value: i32) {
        self.inner.set("surfaceY", value);
    }
}
