use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieChangeEventInit {
    inner: Any,
}
impl FromVal for CookieChangeEventInit {
    fn from_val(v: &Any) -> Self {
        CookieChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CookieChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CookieChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CookieChangeEventInit> for Any {
    fn from(s: CookieChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CookieChangeEventInit> for Any {
    fn from(s: &CookieChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl CookieChangeEventInit {
    pub fn changed(&self) -> Any {
        self.inner.get("changed").as_::<Any>()
    }

    pub fn set_changed(&mut self, value: &Any) {
        self.inner.set("changed", value);
    }
}
impl CookieChangeEventInit {
    pub fn deleted(&self) -> Any {
        self.inner.get("deleted").as_::<Any>()
    }

    pub fn set_deleted(&mut self, value: &Any) {
        self.inner.set("deleted", value);
    }
}
