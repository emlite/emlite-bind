use super::*;

/// The ExtendableCookieChangeEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ExtendableCookieChangeEventInit {
    inner: Any,
}

impl FromVal for ExtendableCookieChangeEventInit {
    fn from_val(v: &Any) -> Self {
        ExtendableCookieChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ExtendableCookieChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ExtendableCookieChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ExtendableCookieChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ExtendableCookieChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ExtendableCookieChangeEventInit> for Any {
    fn from(s: ExtendableCookieChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ExtendableCookieChangeEventInit> for Any {
    fn from(s: &ExtendableCookieChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl ExtendableCookieChangeEventInit {
    /// Getter of the `changed` attribute.
    pub fn changed(&self) -> Any {
        self.inner.get("changed").as_::<Any>()
    }

    /// Setter of the `changed` attribute.
    pub fn set_changed(&mut self, value: &Any) {
        self.inner.set("changed", value);
    }
}
impl ExtendableCookieChangeEventInit {
    /// Getter of the `deleted` attribute.
    pub fn deleted(&self) -> Any {
        self.inner.get("deleted").as_::<Any>()
    }

    /// Setter of the `deleted` attribute.
    pub fn set_deleted(&mut self, value: &Any) {
        self.inner.set("deleted", value);
    }
}
