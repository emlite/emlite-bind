use super::*;

/// The SharedWorker class.
/// [`SharedWorker`](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedWorker {
    inner: EventTarget,
}
impl FromVal for SharedWorker {
    fn from_val(v: &Any) -> Self {
        SharedWorker {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedWorker {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedWorker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedWorker {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedWorker {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedWorker> for Any {
    fn from(s: SharedWorker) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedWorker> for Any {
    fn from(s: &SharedWorker) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedWorker);

impl SharedWorker {
    /// The `new SharedWorker(..)` constructor, creating a new SharedWorker instance
    pub fn new0(script_url: &Any) -> SharedWorker {
        Self {
            inner: Any::global("SharedWorker")
                .new(&[script_url.into()])
                .as_::<EventTarget>(),
        }
    }

    /// The `new SharedWorker(..)` constructor, creating a new SharedWorker instance
    pub fn new1(script_url: &Any, options: &Any) -> SharedWorker {
        Self {
            inner: Any::global("SharedWorker")
                .new(&[script_url.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl SharedWorker {
    /// Getter of the `port` attribute.
    /// [`SharedWorker.port`](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/port)
    pub fn port(&self) -> Any {
        self.inner.get("port").as_::<Any>()
    }
}
impl SharedWorker {
    /// Getter of the `onerror` attribute.
    /// [`SharedWorker.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`SharedWorker.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
