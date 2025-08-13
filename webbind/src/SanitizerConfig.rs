use super::*;




/// The SanitizerConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SanitizerConfig {
    inner: Any,
}

impl FromVal for SanitizerConfig {
    fn from_val(v: &Any) -> Self {
        SanitizerConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SanitizerConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SanitizerConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SanitizerConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SanitizerConfig {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SanitizerConfig> for Any {
    fn from(s: SanitizerConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SanitizerConfig> for Any {
    fn from(s: &SanitizerConfig) -> Any {
        s.inner.clone()
    }
}

impl SanitizerConfig {
    /// Getter of the `elements` attribute.
    pub fn elements(&self) -> TypedArray<Any> {
        self.inner.get("elements").as_::<TypedArray<Any>>()
    }

    /// Setter of the `elements` attribute.
    pub fn set_elements(&mut self, value: &TypedArray<Any>) {
        self.inner.set("elements", value);
    }
}
impl SanitizerConfig {
    /// Getter of the `removeElements` attribute.
    pub fn remove_elements(&self) -> TypedArray<Any> {
        self.inner.get("removeElements").as_::<TypedArray<Any>>()
    }

    /// Setter of the `removeElements` attribute.
    pub fn set_remove_elements(&mut self, value: &TypedArray<Any>) {
        self.inner.set("removeElements", value);
    }
}
impl SanitizerConfig {
    /// Getter of the `replaceWithChildrenElements` attribute.
    pub fn replace_with_children_elements(&self) -> TypedArray<Any> {
        self.inner.get("replaceWithChildrenElements").as_::<TypedArray<Any>>()
    }

    /// Setter of the `replaceWithChildrenElements` attribute.
    pub fn set_replace_with_children_elements(&mut self, value: &TypedArray<Any>) {
        self.inner.set("replaceWithChildrenElements", value);
    }
}
impl SanitizerConfig {
    /// Getter of the `attributes` attribute.
    pub fn attributes(&self) -> TypedArray<Any> {
        self.inner.get("attributes").as_::<TypedArray<Any>>()
    }

    /// Setter of the `attributes` attribute.
    pub fn set_attributes(&mut self, value: &TypedArray<Any>) {
        self.inner.set("attributes", value);
    }
}
impl SanitizerConfig {
    /// Getter of the `removeAttributes` attribute.
    pub fn remove_attributes(&self) -> TypedArray<Any> {
        self.inner.get("removeAttributes").as_::<TypedArray<Any>>()
    }

    /// Setter of the `removeAttributes` attribute.
    pub fn set_remove_attributes(&mut self, value: &TypedArray<Any>) {
        self.inner.set("removeAttributes", value);
    }
}
impl SanitizerConfig {
    /// Getter of the `comments` attribute.
    pub fn comments(&self) -> bool {
        self.inner.get("comments").as_::<bool>()
    }

    /// Setter of the `comments` attribute.
    pub fn set_comments(&mut self, value: bool) {
        self.inner.set("comments", value);
    }
}
impl SanitizerConfig {
    /// Getter of the `dataAttributes` attribute.
    pub fn data_attributes(&self) -> bool {
        self.inner.get("dataAttributes").as_::<bool>()
    }

    /// Setter of the `dataAttributes` attribute.
    pub fn set_data_attributes(&mut self, value: bool) {
        self.inner.set("dataAttributes", value);
    }
}
