use super::*;




/// The ClipboardEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ClipboardEventInit {
    inner: Any,
}

impl FromVal for ClipboardEventInit {
    fn from_val(v: &Any) -> Self {
        ClipboardEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ClipboardEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ClipboardEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ClipboardEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ClipboardEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ClipboardEventInit> for Any {
    fn from(s: ClipboardEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ClipboardEventInit> for Any {
    fn from(s: &ClipboardEventInit) -> Any {
        s.inner.clone()
    }
}

impl ClipboardEventInit {
    /// Getter of the `clipboardData` attribute.
    pub fn clipboard_data(&self) -> DataTransfer {
        self.inner.get("clipboardData").as_::<DataTransfer>()
    }

    /// Setter of the `clipboardData` attribute.
    pub fn set_clipboard_data(&mut self, value: &DataTransfer) {
        self.inner.set("clipboardData", value);
    }
}
