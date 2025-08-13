use super::*;




/// The BaseKeyframe dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BaseKeyframe {
    inner: Any,
}

impl FromVal for BaseKeyframe {
    fn from_val(v: &Any) -> Self {
        BaseKeyframe { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BaseKeyframe {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BaseKeyframe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BaseKeyframe {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BaseKeyframe {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BaseKeyframe> for Any {
    fn from(s: BaseKeyframe) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BaseKeyframe> for Any {
    fn from(s: &BaseKeyframe) -> Any {
        s.inner.clone()
    }
}

impl BaseKeyframe {
    /// Getter of the `offset` attribute.
    pub fn offset(&self) -> f64 {
        self.inner.get("offset").as_::<f64>()
    }

    /// Setter of the `offset` attribute.
    pub fn set_offset(&mut self, value: f64) {
        self.inner.set("offset", value);
    }
}
impl BaseKeyframe {
    /// Getter of the `easing` attribute.
    pub fn easing(&self) -> JsString {
        self.inner.get("easing").as_::<JsString>()
    }

    /// Setter of the `easing` attribute.
    pub fn set_easing(&mut self, value: &JsString) {
        self.inner.set("easing", value);
    }
}
impl BaseKeyframe {
    /// Getter of the `composite` attribute.
    pub fn composite(&self) -> CompositeOperationOrAuto {
        self.inner.get("composite").as_::<CompositeOperationOrAuto>()
    }

    /// Setter of the `composite` attribute.
    pub fn set_composite(&mut self, value: &CompositeOperationOrAuto) {
        self.inner.set("composite", value);
    }
}
