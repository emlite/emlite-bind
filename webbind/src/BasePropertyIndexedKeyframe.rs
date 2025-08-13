use super::*;




/// The BasePropertyIndexedKeyframe dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BasePropertyIndexedKeyframe {
    inner: Any,
}

impl FromVal for BasePropertyIndexedKeyframe {
    fn from_val(v: &Any) -> Self {
        BasePropertyIndexedKeyframe { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BasePropertyIndexedKeyframe {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BasePropertyIndexedKeyframe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BasePropertyIndexedKeyframe {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BasePropertyIndexedKeyframe {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BasePropertyIndexedKeyframe> for Any {
    fn from(s: BasePropertyIndexedKeyframe) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BasePropertyIndexedKeyframe> for Any {
    fn from(s: &BasePropertyIndexedKeyframe) -> Any {
        s.inner.clone()
    }
}

impl BasePropertyIndexedKeyframe {
    /// Getter of the `offset` attribute.
    pub fn offset(&self) -> Any {
        self.inner.get("offset").as_::<Any>()
    }

    /// Setter of the `offset` attribute.
    pub fn set_offset(&mut self, value: &Any) {
        self.inner.set("offset", value);
    }
}
impl BasePropertyIndexedKeyframe {
    /// Getter of the `easing` attribute.
    pub fn easing(&self) -> Any {
        self.inner.get("easing").as_::<Any>()
    }

    /// Setter of the `easing` attribute.
    pub fn set_easing(&mut self, value: &Any) {
        self.inner.set("easing", value);
    }
}
impl BasePropertyIndexedKeyframe {
    /// Getter of the `composite` attribute.
    pub fn composite(&self) -> Any {
        self.inner.get("composite").as_::<Any>()
    }

    /// Setter of the `composite` attribute.
    pub fn set_composite(&mut self, value: &Any) {
        self.inner.set("composite", value);
    }
}
