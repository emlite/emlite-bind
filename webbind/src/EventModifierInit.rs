use super::*;

/// The EventModifierInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EventModifierInit {
    inner: Any,
}

impl FromVal for EventModifierInit {
    fn from_val(v: &Any) -> Self {
        EventModifierInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EventModifierInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EventModifierInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EventModifierInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EventModifierInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EventModifierInit> for Any {
    fn from(s: EventModifierInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EventModifierInit> for Any {
    fn from(s: &EventModifierInit) -> Any {
        s.inner.clone()
    }
}

impl EventModifierInit {
    /// Getter of the `ctrlKey` attribute.
    pub fn ctrl_key(&self) -> bool {
        self.inner.get("ctrlKey").as_::<bool>()
    }

    /// Setter of the `ctrlKey` attribute.
    pub fn set_ctrl_key(&mut self, value: bool) {
        self.inner.set("ctrlKey", value);
    }
}
impl EventModifierInit {
    /// Getter of the `shiftKey` attribute.
    pub fn shift_key(&self) -> bool {
        self.inner.get("shiftKey").as_::<bool>()
    }

    /// Setter of the `shiftKey` attribute.
    pub fn set_shift_key(&mut self, value: bool) {
        self.inner.set("shiftKey", value);
    }
}
impl EventModifierInit {
    /// Getter of the `altKey` attribute.
    pub fn alt_key(&self) -> bool {
        self.inner.get("altKey").as_::<bool>()
    }

    /// Setter of the `altKey` attribute.
    pub fn set_alt_key(&mut self, value: bool) {
        self.inner.set("altKey", value);
    }
}
impl EventModifierInit {
    /// Getter of the `metaKey` attribute.
    pub fn meta_key(&self) -> bool {
        self.inner.get("metaKey").as_::<bool>()
    }

    /// Setter of the `metaKey` attribute.
    pub fn set_meta_key(&mut self, value: bool) {
        self.inner.set("metaKey", value);
    }
}
impl EventModifierInit {
    /// Getter of the `modifierAltGraph` attribute.
    pub fn modifier_alt_graph(&self) -> bool {
        self.inner.get("modifierAltGraph").as_::<bool>()
    }

    /// Setter of the `modifierAltGraph` attribute.
    pub fn set_modifier_alt_graph(&mut self, value: bool) {
        self.inner.set("modifierAltGraph", value);
    }
}
impl EventModifierInit {
    /// Getter of the `modifierCapsLock` attribute.
    pub fn modifier_caps_lock(&self) -> bool {
        self.inner.get("modifierCapsLock").as_::<bool>()
    }

    /// Setter of the `modifierCapsLock` attribute.
    pub fn set_modifier_caps_lock(&mut self, value: bool) {
        self.inner.set("modifierCapsLock", value);
    }
}
impl EventModifierInit {
    /// Getter of the `modifierFn` attribute.
    pub fn modifier_fn(&self) -> bool {
        self.inner.get("modifierFn").as_::<bool>()
    }

    /// Setter of the `modifierFn` attribute.
    pub fn set_modifier_fn(&mut self, value: bool) {
        self.inner.set("modifierFn", value);
    }
}
impl EventModifierInit {
    /// Getter of the `modifierFnLock` attribute.
    pub fn modifier_fn_lock(&self) -> bool {
        self.inner.get("modifierFnLock").as_::<bool>()
    }

    /// Setter of the `modifierFnLock` attribute.
    pub fn set_modifier_fn_lock(&mut self, value: bool) {
        self.inner.set("modifierFnLock", value);
    }
}
impl EventModifierInit {
    /// Getter of the `modifierHyper` attribute.
    pub fn modifier_hyper(&self) -> bool {
        self.inner.get("modifierHyper").as_::<bool>()
    }

    /// Setter of the `modifierHyper` attribute.
    pub fn set_modifier_hyper(&mut self, value: bool) {
        self.inner.set("modifierHyper", value);
    }
}
impl EventModifierInit {
    /// Getter of the `modifierNumLock` attribute.
    pub fn modifier_num_lock(&self) -> bool {
        self.inner.get("modifierNumLock").as_::<bool>()
    }

    /// Setter of the `modifierNumLock` attribute.
    pub fn set_modifier_num_lock(&mut self, value: bool) {
        self.inner.set("modifierNumLock", value);
    }
}
impl EventModifierInit {
    /// Getter of the `modifierScrollLock` attribute.
    pub fn modifier_scroll_lock(&self) -> bool {
        self.inner.get("modifierScrollLock").as_::<bool>()
    }

    /// Setter of the `modifierScrollLock` attribute.
    pub fn set_modifier_scroll_lock(&mut self, value: bool) {
        self.inner.set("modifierScrollLock", value);
    }
}
impl EventModifierInit {
    /// Getter of the `modifierSuper` attribute.
    pub fn modifier_super(&self) -> bool {
        self.inner.get("modifierSuper").as_::<bool>()
    }

    /// Setter of the `modifierSuper` attribute.
    pub fn set_modifier_super(&mut self, value: bool) {
        self.inner.set("modifierSuper", value);
    }
}
impl EventModifierInit {
    /// Getter of the `modifierSymbol` attribute.
    pub fn modifier_symbol(&self) -> bool {
        self.inner.get("modifierSymbol").as_::<bool>()
    }

    /// Setter of the `modifierSymbol` attribute.
    pub fn set_modifier_symbol(&mut self, value: bool) {
        self.inner.set("modifierSymbol", value);
    }
}
impl EventModifierInit {
    /// Getter of the `modifierSymbolLock` attribute.
    pub fn modifier_symbol_lock(&self) -> bool {
        self.inner.get("modifierSymbolLock").as_::<bool>()
    }

    /// Setter of the `modifierSymbolLock` attribute.
    pub fn set_modifier_symbol_lock(&mut self, value: bool) {
        self.inner.set("modifierSymbolLock", value);
    }
}
