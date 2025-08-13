use super::*;




/// The GetRootNodeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetRootNodeOptions {
    inner: Any,
}

impl FromVal for GetRootNodeOptions {
    fn from_val(v: &Any) -> Self {
        GetRootNodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GetRootNodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GetRootNodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GetRootNodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GetRootNodeOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GetRootNodeOptions> for Any {
    fn from(s: GetRootNodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GetRootNodeOptions> for Any {
    fn from(s: &GetRootNodeOptions) -> Any {
        s.inner.clone()
    }
}

impl GetRootNodeOptions {
    /// Getter of the `composed` attribute.
    pub fn composed(&self) -> bool {
        self.inner.get("composed").as_::<bool>()
    }

    /// Setter of the `composed` attribute.
    pub fn set_composed(&mut self, value: bool) {
        self.inner.set("composed", value);
    }
}
