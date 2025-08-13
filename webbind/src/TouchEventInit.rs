use super::*;




/// The TouchEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TouchEventInit {
    inner: Any,
}

impl FromVal for TouchEventInit {
    fn from_val(v: &Any) -> Self {
        TouchEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TouchEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TouchEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TouchEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TouchEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<TouchEventInit> for Any {
    fn from(s: TouchEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TouchEventInit> for Any {
    fn from(s: &TouchEventInit) -> Any {
        s.inner.clone()
    }
}

impl TouchEventInit {
    /// Getter of the `touches` attribute.
    pub fn touches(&self) -> TypedArray<Touch> {
        self.inner.get("touches").as_::<TypedArray<Touch>>()
    }

    /// Setter of the `touches` attribute.
    pub fn set_touches(&mut self, value: &TypedArray<Touch>) {
        self.inner.set("touches", value);
    }
}
impl TouchEventInit {
    /// Getter of the `targetTouches` attribute.
    pub fn target_touches(&self) -> TypedArray<Touch> {
        self.inner.get("targetTouches").as_::<TypedArray<Touch>>()
    }

    /// Setter of the `targetTouches` attribute.
    pub fn set_target_touches(&mut self, value: &TypedArray<Touch>) {
        self.inner.set("targetTouches", value);
    }
}
impl TouchEventInit {
    /// Getter of the `changedTouches` attribute.
    pub fn changed_touches(&self) -> TypedArray<Touch> {
        self.inner.get("changedTouches").as_::<TypedArray<Touch>>()
    }

    /// Setter of the `changedTouches` attribute.
    pub fn set_changed_touches(&mut self, value: &TypedArray<Touch>) {
        self.inner.set("changedTouches", value);
    }
}
