use super::*;




/// The MutationObserverInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MutationObserverInit {
    inner: Any,
}

impl FromVal for MutationObserverInit {
    fn from_val(v: &Any) -> Self {
        MutationObserverInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MutationObserverInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MutationObserverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MutationObserverInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MutationObserverInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MutationObserverInit> for Any {
    fn from(s: MutationObserverInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MutationObserverInit> for Any {
    fn from(s: &MutationObserverInit) -> Any {
        s.inner.clone()
    }
}

impl MutationObserverInit {
    /// Getter of the `childList` attribute.
    pub fn child_list(&self) -> bool {
        self.inner.get("childList").as_::<bool>()
    }

    /// Setter of the `childList` attribute.
    pub fn set_child_list(&mut self, value: bool) {
        self.inner.set("childList", value);
    }
}
impl MutationObserverInit {
    /// Getter of the `attributes` attribute.
    pub fn attributes(&self) -> bool {
        self.inner.get("attributes").as_::<bool>()
    }

    /// Setter of the `attributes` attribute.
    pub fn set_attributes(&mut self, value: bool) {
        self.inner.set("attributes", value);
    }
}
impl MutationObserverInit {
    /// Getter of the `characterData` attribute.
    pub fn character_data(&self) -> bool {
        self.inner.get("characterData").as_::<bool>()
    }

    /// Setter of the `characterData` attribute.
    pub fn set_character_data(&mut self, value: bool) {
        self.inner.set("characterData", value);
    }
}
impl MutationObserverInit {
    /// Getter of the `subtree` attribute.
    pub fn subtree(&self) -> bool {
        self.inner.get("subtree").as_::<bool>()
    }

    /// Setter of the `subtree` attribute.
    pub fn set_subtree(&mut self, value: bool) {
        self.inner.set("subtree", value);
    }
}
impl MutationObserverInit {
    /// Getter of the `attributeOldValue` attribute.
    pub fn attribute_old_value(&self) -> bool {
        self.inner.get("attributeOldValue").as_::<bool>()
    }

    /// Setter of the `attributeOldValue` attribute.
    pub fn set_attribute_old_value(&mut self, value: bool) {
        self.inner.set("attributeOldValue", value);
    }
}
impl MutationObserverInit {
    /// Getter of the `characterDataOldValue` attribute.
    pub fn character_data_old_value(&self) -> bool {
        self.inner.get("characterDataOldValue").as_::<bool>()
    }

    /// Setter of the `characterDataOldValue` attribute.
    pub fn set_character_data_old_value(&mut self, value: bool) {
        self.inner.set("characterDataOldValue", value);
    }
}
impl MutationObserverInit {
    /// Getter of the `attributeFilter` attribute.
    pub fn attribute_filter(&self) -> TypedArray<JsString> {
        self.inner.get("attributeFilter").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `attributeFilter` attribute.
    pub fn set_attribute_filter(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("attributeFilter", value);
    }
}
