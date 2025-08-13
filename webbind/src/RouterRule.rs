use super::*;




/// The RouterRule dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RouterRule {
    inner: Any,
}

impl FromVal for RouterRule {
    fn from_val(v: &Any) -> Self {
        RouterRule { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RouterRule {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RouterRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RouterRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RouterRule {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RouterRule> for Any {
    fn from(s: RouterRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RouterRule> for Any {
    fn from(s: &RouterRule) -> Any {
        s.inner.clone()
    }
}

impl RouterRule {
    /// Getter of the `condition` attribute.
    pub fn condition(&self) -> RouterCondition {
        self.inner.get("condition").as_::<RouterCondition>()
    }

    /// Setter of the `condition` attribute.
    pub fn set_condition(&mut self, value: &RouterCondition) {
        self.inner.set("condition", value);
    }
}
impl RouterRule {
    /// Getter of the `source` attribute.
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }

    /// Setter of the `source` attribute.
    pub fn set_source(&mut self, value: &Any) {
        self.inner.set("source", value);
    }
}
