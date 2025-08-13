use super::*;




/// The TableDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TableDescriptor {
    inner: Any,
}

impl FromVal for TableDescriptor {
    fn from_val(v: &Any) -> Self {
        TableDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TableDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TableDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TableDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TableDescriptor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<TableDescriptor> for Any {
    fn from(s: TableDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TableDescriptor> for Any {
    fn from(s: &TableDescriptor) -> Any {
        s.inner.clone()
    }
}

impl TableDescriptor {
    /// Getter of the `element` attribute.
    pub fn element(&self) -> TableKind {
        self.inner.get("element").as_::<TableKind>()
    }

    /// Setter of the `element` attribute.
    pub fn set_element(&mut self, value: &TableKind) {
        self.inner.set("element", value);
    }
}
impl TableDescriptor {
    /// Getter of the `initial` attribute.
    pub fn initial(&self) -> u32 {
        self.inner.get("initial").as_::<u32>()
    }

    /// Setter of the `initial` attribute.
    pub fn set_initial(&mut self, value: u32) {
        self.inner.set("initial", value);
    }
}
impl TableDescriptor {
    /// Getter of the `maximum` attribute.
    pub fn maximum(&self) -> u32 {
        self.inner.get("maximum").as_::<u32>()
    }

    /// Setter of the `maximum` attribute.
    pub fn set_maximum(&mut self, value: u32) {
        self.inner.set("maximum", value);
    }
}
