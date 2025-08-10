use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PopStateEventInit {
    inner: Any,
}
impl FromVal for PopStateEventInit {
    fn from_val(v: &Any) -> Self {
        PopStateEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PopStateEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PopStateEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PopStateEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PopStateEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PopStateEventInit> for Any {
    fn from(s: PopStateEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PopStateEventInit> for Any {
    fn from(s: &PopStateEventInit) -> Any {
        s.inner.clone()
    }
}

impl PopStateEventInit {
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }

    pub fn set_state(&mut self, value: &Any) {
        self.inner.set("state", value);
    }
}
impl PopStateEventInit {
    pub fn has_ua_visual_transition(&self) -> bool {
        self.inner.get("hasUAVisualTransition").as_::<bool>()
    }

    pub fn set_has_ua_visual_transition(&mut self, value: bool) {
        self.inner.set("hasUAVisualTransition", value);
    }
}
