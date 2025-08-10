use super::*;

/// The IdleDeadline class.
/// [`IdleDeadline`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdleDeadline {
    inner: Any,
}

impl FromVal for IdleDeadline {
    fn from_val(v: &Any) -> Self {
        IdleDeadline {
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

impl core::ops::Deref for IdleDeadline {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdleDeadline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdleDeadline {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdleDeadline {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IdleDeadline> for Any {
    fn from(s: IdleDeadline) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdleDeadline> for Any {
    fn from(s: &IdleDeadline) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IdleDeadline);

impl IdleDeadline {
    /// The timeRemaining method.
    /// [`IdleDeadline.timeRemaining`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/timeRemaining)
    pub fn time_remaining(&self) -> Any {
        self.inner.call("timeRemaining", &[]).as_::<Any>()
    }
}
impl IdleDeadline {
    /// Getter of the `didTimeout` attribute.
    /// [`IdleDeadline.didTimeout`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/didTimeout)
    pub fn did_timeout(&self) -> bool {
        self.inner.get("didTimeout").as_::<bool>()
    }
}
