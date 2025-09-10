use super::*;

/// The CustomEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CustomEventInit {
    inner: Any,
}

impl FromVal for CustomEventInit {
    fn from_val(v: &Any) -> Self {
        CustomEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CustomEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CustomEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CustomEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CustomEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CustomEventInit> for Any {
    fn from(s: CustomEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CustomEventInit> for Any {
    fn from(s: &CustomEventInit) -> Any {
        s.inner.clone()
    }
}

impl CustomEventInit {
    /// Getter of the `detail` attribute.
    pub fn detail(&self) -> Any {
        self.inner.get("detail").as_::<Any>()
    }

    /// Setter of the `detail` attribute.
    pub fn set_detail(&mut self, value: &Any) {
        self.inner.set("detail", value);
    }
}
