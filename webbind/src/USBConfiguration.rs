use super::*;

/// The USBConfiguration class.
/// [`USBConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/USBConfiguration)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBConfiguration {
    inner: Any,
}
impl FromVal for USBConfiguration {
    fn from_val(v: &Any) -> Self {
        USBConfiguration {
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
impl core::ops::Deref for USBConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for USBConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for USBConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<USBConfiguration> for Any {
    fn from(s: USBConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&USBConfiguration> for Any {
    fn from(s: &USBConfiguration) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(USBConfiguration);

impl USBConfiguration {
    /// The `new USBConfiguration(..)` constructor, creating a new USBConfiguration instance
    pub fn new(device: &USBDevice, configuration_value: u8) -> USBConfiguration {
        Self {
            inner: Any::global("USBConfiguration")
                .new(&[device.into(), configuration_value.into()])
                .as_::<Any>(),
        }
    }
}
impl USBConfiguration {
    /// Getter of the `configurationValue` attribute.
    /// [`USBConfiguration.configurationValue`](https://developer.mozilla.org/en-US/docs/Web/API/USBConfiguration/configurationValue)
    pub fn configuration_value(&self) -> u8 {
        self.inner.get("configurationValue").as_::<u8>()
    }
}
impl USBConfiguration {
    /// Getter of the `configurationName` attribute.
    /// [`USBConfiguration.configurationName`](https://developer.mozilla.org/en-US/docs/Web/API/USBConfiguration/configurationName)
    pub fn configuration_name(&self) -> JsString {
        self.inner.get("configurationName").as_::<JsString>()
    }
}
impl USBConfiguration {
    /// Getter of the `interfaces` attribute.
    /// [`USBConfiguration.interfaces`](https://developer.mozilla.org/en-US/docs/Web/API/USBConfiguration/interfaces)
    pub fn interfaces(&self) -> TypedArray<USBInterface> {
        self.inner
            .get("interfaces")
            .as_::<TypedArray<USBInterface>>()
    }
}
