use super::*;

/// The GPUSampler class.
/// [`GPUSampler`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSampler)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUSampler {
    inner: Any,
}

impl FromVal for GPUSampler {
    fn from_val(v: &Any) -> Self {
        GPUSampler {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUSampler {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUSampler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUSampler {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUSampler {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUSampler> for Any {
    fn from(s: GPUSampler) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUSampler> for Any {
    fn from(s: &GPUSampler) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUSampler);

impl GPUSampler {
    /// Getter of the `label` attribute.
    /// [`GPUSampler.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSampler/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUSampler.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSampler/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
