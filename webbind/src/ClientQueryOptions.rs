use super::*;




/// The ClientQueryOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ClientQueryOptions {
    inner: Any,
}

impl FromVal for ClientQueryOptions {
    fn from_val(v: &Any) -> Self {
        ClientQueryOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ClientQueryOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ClientQueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ClientQueryOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ClientQueryOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ClientQueryOptions> for Any {
    fn from(s: ClientQueryOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ClientQueryOptions> for Any {
    fn from(s: &ClientQueryOptions) -> Any {
        s.inner.clone()
    }
}

impl ClientQueryOptions {
    /// Getter of the `includeUncontrolled` attribute.
    pub fn include_uncontrolled(&self) -> bool {
        self.inner.get("includeUncontrolled").as_::<bool>()
    }

    /// Setter of the `includeUncontrolled` attribute.
    pub fn set_include_uncontrolled(&mut self, value: bool) {
        self.inner.set("includeUncontrolled", value);
    }
}
impl ClientQueryOptions {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> ClientType {
        self.inner.get("type").as_::<ClientType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &ClientType) {
        self.inner.set("type", value);
    }
}
