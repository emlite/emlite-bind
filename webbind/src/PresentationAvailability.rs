use super::*;

/// The PresentationAvailability class.
/// [`PresentationAvailability`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationAvailability {
    inner: EventTarget,
}
impl FromVal for PresentationAvailability {
    fn from_val(v: &Any) -> Self {
        PresentationAvailability {
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
impl core::ops::Deref for PresentationAvailability {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PresentationAvailability {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PresentationAvailability {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PresentationAvailability {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PresentationAvailability> for Any {
    fn from(s: PresentationAvailability) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PresentationAvailability> for Any {
    fn from(s: &PresentationAvailability) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PresentationAvailability);

impl PresentationAvailability {
    /// Getter of the `value` attribute.
    /// [`PresentationAvailability.value`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/value)
    pub fn value(&self) -> bool {
        self.inner.get("value").as_::<bool>()
    }
}
impl PresentationAvailability {
    /// Getter of the `onchange` attribute.
    /// [`PresentationAvailability.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`PresentationAvailability.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
