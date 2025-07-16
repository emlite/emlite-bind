use super::*;

/// The Instance class.
/// [`Instance`](https://developer.mozilla.org/en-US/docs/Web/API/Instance)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Instance {
    inner: Any,
}
impl FromVal for Instance {
    fn from_val(v: &Any) -> Self {
        Instance {
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
impl core::ops::Deref for Instance {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Instance {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Instance {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Instance> for Any {
    fn from(s: Instance) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Instance> for Any {
    fn from(s: &Instance) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Instance);

impl Instance {
    /// The `new Instance(..)` constructor, creating a new Instance instance
    pub fn new0(module: &Module) -> Instance {
        Self {
            inner: Any::global("Instance").new(&[module.into()]).as_::<Any>(),
        }
    }

    /// The `new Instance(..)` constructor, creating a new Instance instance
    pub fn new1(module: &Module, import_object: &Object) -> Instance {
        Self {
            inner: Any::global("Instance")
                .new(&[module.into(), import_object.into()])
                .as_::<Any>(),
        }
    }
}
impl Instance {
    /// Getter of the `exports` attribute.
    /// [`Instance.exports`](https://developer.mozilla.org/en-US/docs/Web/API/Instance/exports)
    pub fn exports(&self) -> Object {
        self.inner.get("exports").as_::<Object>()
    }
}
