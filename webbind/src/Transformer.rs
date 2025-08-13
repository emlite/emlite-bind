use super::*;




/// The Transformer dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Transformer {
    inner: Any,
}

impl FromVal for Transformer {
    fn from_val(v: &Any) -> Self {
        Transformer { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Transformer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Transformer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Transformer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Transformer {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Transformer> for Any {
    fn from(s: Transformer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Transformer> for Any {
    fn from(s: &Transformer) -> Any {
        s.inner.clone()
    }
}

impl Transformer {
    /// Getter of the `start` attribute.
    pub fn start(&self) -> Function {
        self.inner.get("start").as_::<Function>()
    }

    /// Setter of the `start` attribute.
    pub fn set_start(&mut self, value: &Function) {
        self.inner.set("start", value);
    }
}
impl Transformer {
    /// Getter of the `transform` attribute.
    pub fn transform(&self) -> Function {
        self.inner.get("transform").as_::<Function>()
    }

    /// Setter of the `transform` attribute.
    pub fn set_transform(&mut self, value: &Function) {
        self.inner.set("transform", value);
    }
}
impl Transformer {
    /// Getter of the `flush` attribute.
    pub fn flush(&self) -> Function {
        self.inner.get("flush").as_::<Function>()
    }

    /// Setter of the `flush` attribute.
    pub fn set_flush(&mut self, value: &Function) {
        self.inner.set("flush", value);
    }
}
impl Transformer {
    /// Getter of the `cancel` attribute.
    pub fn cancel(&self) -> Function {
        self.inner.get("cancel").as_::<Function>()
    }

    /// Setter of the `cancel` attribute.
    pub fn set_cancel(&mut self, value: &Function) {
        self.inner.set("cancel", value);
    }
}
impl Transformer {
    /// Getter of the `readableType` attribute.
    pub fn readable_type(&self) -> Any {
        self.inner.get("readableType").as_::<Any>()
    }

    /// Setter of the `readableType` attribute.
    pub fn set_readable_type(&mut self, value: &Any) {
        self.inner.set("readableType", value);
    }
}
impl Transformer {
    /// Getter of the `writableType` attribute.
    pub fn writable_type(&self) -> Any {
        self.inner.get("writableType").as_::<Any>()
    }

    /// Setter of the `writableType` attribute.
    pub fn set_writable_type(&mut self, value: &Any) {
        self.inner.set("writableType", value);
    }
}
