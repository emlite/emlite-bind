use super::*;




/// The ShadowRootInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShadowRootInit {
    inner: Any,
}

impl FromVal for ShadowRootInit {
    fn from_val(v: &Any) -> Self {
        ShadowRootInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ShadowRootInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ShadowRootInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ShadowRootInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ShadowRootInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ShadowRootInit> for Any {
    fn from(s: ShadowRootInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ShadowRootInit> for Any {
    fn from(s: &ShadowRootInit) -> Any {
        s.inner.clone()
    }
}

impl ShadowRootInit {
    /// Getter of the `mode` attribute.
    pub fn mode(&self) -> ShadowRootMode {
        self.inner.get("mode").as_::<ShadowRootMode>()
    }

    /// Setter of the `mode` attribute.
    pub fn set_mode(&mut self, value: &ShadowRootMode) {
        self.inner.set("mode", value);
    }
}
impl ShadowRootInit {
    /// Getter of the `delegatesFocus` attribute.
    pub fn delegates_focus(&self) -> bool {
        self.inner.get("delegatesFocus").as_::<bool>()
    }

    /// Setter of the `delegatesFocus` attribute.
    pub fn set_delegates_focus(&mut self, value: bool) {
        self.inner.set("delegatesFocus", value);
    }
}
impl ShadowRootInit {
    /// Getter of the `slotAssignment` attribute.
    pub fn slot_assignment(&self) -> SlotAssignmentMode {
        self.inner.get("slotAssignment").as_::<SlotAssignmentMode>()
    }

    /// Setter of the `slotAssignment` attribute.
    pub fn set_slot_assignment(&mut self, value: &SlotAssignmentMode) {
        self.inner.set("slotAssignment", value);
    }
}
impl ShadowRootInit {
    /// Getter of the `clonable` attribute.
    pub fn clonable(&self) -> bool {
        self.inner.get("clonable").as_::<bool>()
    }

    /// Setter of the `clonable` attribute.
    pub fn set_clonable(&mut self, value: bool) {
        self.inner.set("clonable", value);
    }
}
impl ShadowRootInit {
    /// Getter of the `serializable` attribute.
    pub fn serializable(&self) -> bool {
        self.inner.get("serializable").as_::<bool>()
    }

    /// Setter of the `serializable` attribute.
    pub fn set_serializable(&mut self, value: bool) {
        self.inner.set("serializable", value);
    }
}
impl ShadowRootInit {
    /// Getter of the `customElementRegistry` attribute.
    pub fn custom_element_registry(&self) -> CustomElementRegistry {
        self.inner.get("customElementRegistry").as_::<CustomElementRegistry>()
    }

    /// Setter of the `customElementRegistry` attribute.
    pub fn set_custom_element_registry(&mut self, value: &CustomElementRegistry) {
        self.inner.set("customElementRegistry", value);
    }
}
