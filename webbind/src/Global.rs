use super::*;

/// The Global class.
/// [`Global`](https://developer.mozilla.org/en-US/docs/Web/API/Global)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Global {
    inner: Any,
}

impl FromVal for Global {
    fn from_val(v: &Any) -> Self {
        Global {
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

impl core::ops::Deref for Global {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Global {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Global {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Global {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Global> for Any {
    fn from(s: Global) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Global> for Any {
    fn from(s: &Global) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Global);

impl Global {
    /// The `new Global(..)` constructor, creating a new Global instance
    pub fn new0(descriptor: &GlobalDescriptor) -> Global {
        Self {
            inner: Any::global("Global").new(&[descriptor.into()]).as_::<Any>(),
        }
    }

    /// The `new Global(..)` constructor, creating a new Global instance
    pub fn new1(descriptor: &GlobalDescriptor, v: &Any) -> Global {
        Self {
            inner: Any::global("Global")
                .new(&[descriptor.into(), v.into()])
                .as_::<Any>(),
        }
    }
}
impl Global {
    /// The valueOf method.
    /// [`Global.valueOf`](https://developer.mozilla.org/en-US/docs/Web/API/Global/valueOf)
    pub fn value_of(&self) -> Any {
        self.inner.call("valueOf", &[]).as_::<Any>()
    }
}
impl Global {
    /// Getter of the `value` attribute.
    /// [`Global.value`](https://developer.mozilla.org/en-US/docs/Web/API/Global/value)
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }

    /// Setter of the `value` attribute.
    /// [`Global.value`](https://developer.mozilla.org/en-US/docs/Web/API/Global/value)
    pub fn set_value(&mut self, value: &Any) {
        self.inner.set("value", value);
    }
}
