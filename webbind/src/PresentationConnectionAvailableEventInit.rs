use super::*;

/// The PresentationConnectionAvailableEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationConnectionAvailableEventInit {
    inner: Any,
}

impl FromVal for PresentationConnectionAvailableEventInit {
    fn from_val(v: &Any) -> Self {
        PresentationConnectionAvailableEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PresentationConnectionAvailableEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PresentationConnectionAvailableEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PresentationConnectionAvailableEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PresentationConnectionAvailableEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PresentationConnectionAvailableEventInit> for Any {
    fn from(s: PresentationConnectionAvailableEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PresentationConnectionAvailableEventInit> for Any {
    fn from(s: &PresentationConnectionAvailableEventInit) -> Any {
        s.inner.clone()
    }
}

impl PresentationConnectionAvailableEventInit {
    /// Getter of the `connection` attribute.
    pub fn connection(&self) -> PresentationConnection {
        self.inner.get("connection").as_::<PresentationConnection>()
    }

    /// Setter of the `connection` attribute.
    pub fn set_connection(&mut self, value: &PresentationConnection) {
        self.inner.set("connection", value);
    }
}
