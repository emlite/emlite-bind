use super::*;




/// The GetHTMLOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetHTMLOptions {
    inner: Any,
}

impl FromVal for GetHTMLOptions {
    fn from_val(v: &Any) -> Self {
        GetHTMLOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GetHTMLOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GetHTMLOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GetHTMLOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GetHTMLOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GetHTMLOptions> for Any {
    fn from(s: GetHTMLOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GetHTMLOptions> for Any {
    fn from(s: &GetHTMLOptions) -> Any {
        s.inner.clone()
    }
}

impl GetHTMLOptions {
    /// Getter of the `serializableShadowRoots` attribute.
    pub fn serializable_shadow_roots(&self) -> bool {
        self.inner.get("serializableShadowRoots").as_::<bool>()
    }

    /// Setter of the `serializableShadowRoots` attribute.
    pub fn set_serializable_shadow_roots(&mut self, value: bool) {
        self.inner.set("serializableShadowRoots", value);
    }
}
impl GetHTMLOptions {
    /// Getter of the `shadowRoots` attribute.
    pub fn shadow_roots(&self) -> TypedArray<ShadowRoot> {
        self.inner.get("shadowRoots").as_::<TypedArray<ShadowRoot>>()
    }

    /// Setter of the `shadowRoots` attribute.
    pub fn set_shadow_roots(&mut self, value: &TypedArray<ShadowRoot>) {
        self.inner.set("shadowRoots", value);
    }
}
