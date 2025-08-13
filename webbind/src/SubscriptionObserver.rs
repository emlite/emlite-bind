use super::*;




/// The SubscriptionObserver dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SubscriptionObserver {
    inner: Any,
}

impl FromVal for SubscriptionObserver {
    fn from_val(v: &Any) -> Self {
        SubscriptionObserver { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SubscriptionObserver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SubscriptionObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SubscriptionObserver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SubscriptionObserver {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SubscriptionObserver> for Any {
    fn from(s: SubscriptionObserver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SubscriptionObserver> for Any {
    fn from(s: &SubscriptionObserver) -> Any {
        s.inner.clone()
    }
}

impl SubscriptionObserver {
    /// Getter of the `next` attribute.
    pub fn next(&self) -> Function {
        self.inner.get("next").as_::<Function>()
    }

    /// Setter of the `next` attribute.
    pub fn set_next(&mut self, value: &Function) {
        self.inner.set("next", value);
    }
}
impl SubscriptionObserver {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> Function {
        self.inner.get("error").as_::<Function>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &Function) {
        self.inner.set("error", value);
    }
}
impl SubscriptionObserver {
    /// Getter of the `complete` attribute.
    pub fn complete(&self) -> Function {
        self.inner.get("complete").as_::<Function>()
    }

    /// Setter of the `complete` attribute.
    pub fn set_complete(&mut self, value: &Function) {
        self.inner.set("complete", value);
    }
}
