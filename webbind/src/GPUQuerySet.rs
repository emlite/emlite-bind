use super::*;

/// The GPUQuerySet class.
/// [`GPUQuerySet`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQuerySet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUQuerySet {
    inner: Any,
}

impl FromVal for GPUQuerySet {
    fn from_val(v: &Any) -> Self {
        GPUQuerySet {
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

impl core::ops::Deref for GPUQuerySet {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUQuerySet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUQuerySet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUQuerySet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUQuerySet> for Any {
    fn from(s: GPUQuerySet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUQuerySet> for Any {
    fn from(s: &GPUQuerySet) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUQuerySet);

impl GPUQuerySet {
    /// Getter of the `type` attribute.
    /// [`GPUQuerySet.type`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQuerySet/type)
    pub fn type_(&self) -> GPUQueryType {
        self.inner.get("type").as_::<GPUQueryType>()
    }
}
impl GPUQuerySet {
    /// Getter of the `count` attribute.
    /// [`GPUQuerySet.count`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQuerySet/count)
    pub fn count(&self) -> Any {
        self.inner.get("count").as_::<Any>()
    }
}
impl GPUQuerySet {
    /// Getter of the `label` attribute.
    /// [`GPUQuerySet.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQuerySet/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUQuerySet.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQuerySet/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl GPUQuerySet {
    /// The destroy method.
    /// [`GPUQuerySet.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQuerySet/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
