use super::*;

/// The Subscriber class.
/// [`Subscriber`](https://developer.mozilla.org/en-US/docs/Web/API/Subscriber)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Subscriber {
    inner: Any,
}
impl FromVal for Subscriber {
    fn from_val(v: &Any) -> Self {
        Subscriber {
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
impl core::ops::Deref for Subscriber {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Subscriber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Subscriber {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Subscriber {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Subscriber> for Any {
    fn from(s: Subscriber) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Subscriber> for Any {
    fn from(s: &Subscriber) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Subscriber);

impl Subscriber {
    /// The next method.
    /// [`Subscriber.next`](https://developer.mozilla.org/en-US/docs/Web/API/Subscriber/next)
    pub fn next(&self, value: &Any) -> Undefined {
        self.inner.call("next", &[value.into()]).as_::<Undefined>()
    }
}
impl Subscriber {
    /// The error method.
    /// [`Subscriber.error`](https://developer.mozilla.org/en-US/docs/Web/API/Subscriber/error)
    pub fn error(&self, error: &Any) -> Undefined {
        self.inner.call("error", &[error.into()]).as_::<Undefined>()
    }
}
impl Subscriber {
    /// The complete method.
    /// [`Subscriber.complete`](https://developer.mozilla.org/en-US/docs/Web/API/Subscriber/complete)
    pub fn complete(&self) -> Undefined {
        self.inner.call("complete", &[]).as_::<Undefined>()
    }
}
impl Subscriber {
    /// The addTeardown method.
    /// [`Subscriber.addTeardown`](https://developer.mozilla.org/en-US/docs/Web/API/Subscriber/addTeardown)
    pub fn add_teardown(&self, teardown: &Any) -> Undefined {
        self.inner
            .call("addTeardown", &[teardown.into()])
            .as_::<Undefined>()
    }
}
impl Subscriber {
    /// Getter of the `active` attribute.
    /// [`Subscriber.active`](https://developer.mozilla.org/en-US/docs/Web/API/Subscriber/active)
    pub fn active(&self) -> bool {
        self.inner.get("active").as_::<bool>()
    }
}
impl Subscriber {
    /// Getter of the `signal` attribute.
    /// [`Subscriber.signal`](https://developer.mozilla.org/en-US/docs/Web/API/Subscriber/signal)
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }
}
