use super::*;




/// The CreateMonitor class.
/// [`CreateMonitor`](https://developer.mozilla.org/en-US/docs/Web/API/CreateMonitor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CreateMonitor {
    inner: EventTarget,
}

impl FromVal for CreateMonitor {
    fn from_val(v: &Any) -> Self {
        CreateMonitor { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CreateMonitor {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CreateMonitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CreateMonitor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CreateMonitor {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CreateMonitor> for Any {
    fn from(s: CreateMonitor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CreateMonitor> for Any {
    fn from(s: &CreateMonitor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CreateMonitor);


impl CreateMonitor {
    /// Getter of the `ondownloadprogress` attribute.
    /// [`CreateMonitor.ondownloadprogress`](https://developer.mozilla.org/en-US/docs/Web/API/CreateMonitor/ondownloadprogress)
    pub fn ondownloadprogress(&self) -> Any {
        self.inner.get("ondownloadprogress").as_::<Any>()
    }

    /// Setter of the `ondownloadprogress` attribute.
    /// [`CreateMonitor.ondownloadprogress`](https://developer.mozilla.org/en-US/docs/Web/API/CreateMonitor/ondownloadprogress)
    pub fn set_ondownloadprogress(&mut self, value: &Any) {
        self.inner.set("ondownloadprogress", value);
    }
}
