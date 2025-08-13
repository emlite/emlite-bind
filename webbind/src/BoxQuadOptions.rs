use super::*;




/// The BoxQuadOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BoxQuadOptions {
    inner: Any,
}

impl FromVal for BoxQuadOptions {
    fn from_val(v: &Any) -> Self {
        BoxQuadOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BoxQuadOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BoxQuadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BoxQuadOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BoxQuadOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BoxQuadOptions> for Any {
    fn from(s: BoxQuadOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BoxQuadOptions> for Any {
    fn from(s: &BoxQuadOptions) -> Any {
        s.inner.clone()
    }
}

impl BoxQuadOptions {
    /// Getter of the `box` attribute.
    pub fn box_(&self) -> CSSBoxType {
        self.inner.get("box").as_::<CSSBoxType>()
    }

    /// Setter of the `box` attribute.
    pub fn set_box_(&mut self, value: &CSSBoxType) {
        self.inner.set("box", value);
    }
}
impl BoxQuadOptions {
    /// Getter of the `relativeTo` attribute.
    pub fn relative_to(&self) -> Any {
        self.inner.get("relativeTo").as_::<Any>()
    }

    /// Setter of the `relativeTo` attribute.
    pub fn set_relative_to(&mut self, value: &Any) {
        self.inner.set("relativeTo", value);
    }
}
