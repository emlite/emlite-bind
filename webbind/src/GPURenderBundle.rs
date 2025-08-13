use super::*;




/// The GPURenderBundle class.
/// [`GPURenderBundle`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundle)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderBundle {
    inner: Any,
}

impl FromVal for GPURenderBundle {
    fn from_val(v: &Any) -> Self {
        GPURenderBundle { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPURenderBundle {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPURenderBundle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPURenderBundle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPURenderBundle {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPURenderBundle> for Any {
    fn from(s: GPURenderBundle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPURenderBundle> for Any {
    fn from(s: &GPURenderBundle) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPURenderBundle);


impl GPURenderBundle {
    /// Getter of the `label` attribute.
    /// [`GPURenderBundle.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundle/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPURenderBundle.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundle/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
