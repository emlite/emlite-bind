use super::*;




/// The KeyboardEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeyboardEventInit {
    inner: Any,
}

impl FromVal for KeyboardEventInit {
    fn from_val(v: &Any) -> Self {
        KeyboardEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for KeyboardEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KeyboardEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KeyboardEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KeyboardEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<KeyboardEventInit> for Any {
    fn from(s: KeyboardEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KeyboardEventInit> for Any {
    fn from(s: &KeyboardEventInit) -> Any {
        s.inner.clone()
    }
}

impl KeyboardEventInit {
    /// Getter of the `charCode` attribute.
    pub fn char_code(&self) -> u32 {
        self.inner.get("charCode").as_::<u32>()
    }

    /// Setter of the `charCode` attribute.
    pub fn set_char_code(&mut self, value: u32) {
        self.inner.set("charCode", value);
    }
}
impl KeyboardEventInit {
    /// Getter of the `keyCode` attribute.
    pub fn key_code(&self) -> u32 {
        self.inner.get("keyCode").as_::<u32>()
    }

    /// Setter of the `keyCode` attribute.
    pub fn set_key_code(&mut self, value: u32) {
        self.inner.set("keyCode", value);
    }
}
