use super::*;

/// The DevicePosture class.
/// [`DevicePosture`](https://developer.mozilla.org/en-US/docs/Web/API/DevicePosture)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DevicePosture {
    inner: EventTarget,
}

impl FromVal for DevicePosture {
    fn from_val(v: &Any) -> Self {
        DevicePosture {
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

impl core::ops::Deref for DevicePosture {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DevicePosture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DevicePosture {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DevicePosture {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DevicePosture> for Any {
    fn from(s: DevicePosture) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DevicePosture> for Any {
    fn from(s: &DevicePosture) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DevicePosture);

impl DevicePosture {
    /// Getter of the `type` attribute.
    /// [`DevicePosture.type`](https://developer.mozilla.org/en-US/docs/Web/API/DevicePosture/type)
    pub fn type_(&self) -> DevicePostureType {
        self.inner.get("type").as_::<DevicePostureType>()
    }
}
impl DevicePosture {
    /// Getter of the `onchange` attribute.
    /// [`DevicePosture.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/DevicePosture/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`DevicePosture.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/DevicePosture/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
