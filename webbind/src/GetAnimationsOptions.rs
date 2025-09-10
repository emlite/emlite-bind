use super::*;

/// The GetAnimationsOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetAnimationsOptions {
    inner: Any,
}

impl FromVal for GetAnimationsOptions {
    fn from_val(v: &Any) -> Self {
        GetAnimationsOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GetAnimationsOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GetAnimationsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GetAnimationsOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GetAnimationsOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GetAnimationsOptions> for Any {
    fn from(s: GetAnimationsOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GetAnimationsOptions> for Any {
    fn from(s: &GetAnimationsOptions) -> Any {
        s.inner.clone()
    }
}

impl GetAnimationsOptions {
    /// Getter of the `subtree` attribute.
    pub fn subtree(&self) -> bool {
        self.inner.get("subtree").as_::<bool>()
    }

    /// Setter of the `subtree` attribute.
    pub fn set_subtree(&mut self, value: bool) {
        self.inner.set("subtree", value);
    }
}
impl GetAnimationsOptions {
    /// Getter of the `pseudoElement` attribute.
    pub fn pseudo_element(&self) -> JsString {
        self.inner.get("pseudoElement").as_::<JsString>()
    }

    /// Setter of the `pseudoElement` attribute.
    pub fn set_pseudo_element(&mut self, value: &JsString) {
        self.inner.set("pseudoElement", value);
    }
}
