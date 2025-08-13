use super::*;




/// The ScrollOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScrollOptions {
    inner: Any,
}

impl FromVal for ScrollOptions {
    fn from_val(v: &Any) -> Self {
        ScrollOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ScrollOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ScrollOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ScrollOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ScrollOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ScrollOptions> for Any {
    fn from(s: ScrollOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ScrollOptions> for Any {
    fn from(s: &ScrollOptions) -> Any {
        s.inner.clone()
    }
}

impl ScrollOptions {
    /// Getter of the `behavior` attribute.
    pub fn behavior(&self) -> ScrollBehavior {
        self.inner.get("behavior").as_::<ScrollBehavior>()
    }

    /// Setter of the `behavior` attribute.
    pub fn set_behavior(&mut self, value: &ScrollBehavior) {
        self.inner.set("behavior", value);
    }
}
