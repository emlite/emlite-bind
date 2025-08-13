use super::*;




/// The DragEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DragEventInit {
    inner: Any,
}

impl FromVal for DragEventInit {
    fn from_val(v: &Any) -> Self {
        DragEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DragEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DragEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DragEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DragEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DragEventInit> for Any {
    fn from(s: DragEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DragEventInit> for Any {
    fn from(s: &DragEventInit) -> Any {
        s.inner.clone()
    }
}

impl DragEventInit {
    /// Getter of the `dataTransfer` attribute.
    pub fn data_transfer(&self) -> DataTransfer {
        self.inner.get("dataTransfer").as_::<DataTransfer>()
    }

    /// Setter of the `dataTransfer` attribute.
    pub fn set_data_transfer(&mut self, value: &DataTransfer) {
        self.inner.set("dataTransfer", value);
    }
}
