use super::*;

/// The LaunchQueue class.
/// [`LaunchQueue`](https://developer.mozilla.org/en-US/docs/Web/API/LaunchQueue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LaunchQueue {
    inner: Any,
}

impl FromVal for LaunchQueue {
    fn from_val(v: &Any) -> Self {
        LaunchQueue {
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

impl core::ops::Deref for LaunchQueue {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LaunchQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LaunchQueue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LaunchQueue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<LaunchQueue> for Any {
    fn from(s: LaunchQueue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LaunchQueue> for Any {
    fn from(s: &LaunchQueue) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(LaunchQueue);

impl LaunchQueue {
    /// The setConsumer method.
    /// [`LaunchQueue.setConsumer`](https://developer.mozilla.org/en-US/docs/Web/API/LaunchQueue/setConsumer)
    pub fn set_consumer(&self, consumer: &Function) -> Undefined {
        self.inner
            .call("setConsumer", &[consumer.into()])
            .as_::<Undefined>()
    }
}
