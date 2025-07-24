use super::*;

/// The USBAlternateInterface class.
/// [`USBAlternateInterface`](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBAlternateInterface {
    inner: Any,
}
impl FromVal for USBAlternateInterface {
    fn from_val(v: &Any) -> Self {
        USBAlternateInterface {
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
impl core::ops::Deref for USBAlternateInterface {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBAlternateInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for USBAlternateInterface {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for USBAlternateInterface {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<USBAlternateInterface> for Any {
    fn from(s: USBAlternateInterface) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&USBAlternateInterface> for Any {
    fn from(s: &USBAlternateInterface) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(USBAlternateInterface);

impl USBAlternateInterface {
    /// The `new USBAlternateInterface(..)` constructor, creating a new USBAlternateInterface instance
    pub fn new(device_interface: &USBInterface, alternate_setting: u8) -> USBAlternateInterface {
        Self {
            inner: Any::global("USBAlternateInterface")
                .new(&[device_interface.into(), alternate_setting.into()])
                .as_::<Any>(),
        }
    }
}
impl USBAlternateInterface {
    /// Getter of the `alternateSetting` attribute.
    /// [`USBAlternateInterface.alternateSetting`](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/alternateSetting)
    pub fn alternate_setting(&self) -> u8 {
        self.inner.get("alternateSetting").as_::<u8>()
    }
}
impl USBAlternateInterface {
    /// Getter of the `interfaceClass` attribute.
    /// [`USBAlternateInterface.interfaceClass`](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceClass)
    pub fn interface_class(&self) -> u8 {
        self.inner.get("interfaceClass").as_::<u8>()
    }
}
impl USBAlternateInterface {
    /// Getter of the `interfaceSubclass` attribute.
    /// [`USBAlternateInterface.interfaceSubclass`](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceSubclass)
    pub fn interface_subclass(&self) -> u8 {
        self.inner.get("interfaceSubclass").as_::<u8>()
    }
}
impl USBAlternateInterface {
    /// Getter of the `interfaceProtocol` attribute.
    /// [`USBAlternateInterface.interfaceProtocol`](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceProtocol)
    pub fn interface_protocol(&self) -> u8 {
        self.inner.get("interfaceProtocol").as_::<u8>()
    }
}
impl USBAlternateInterface {
    /// Getter of the `interfaceName` attribute.
    /// [`USBAlternateInterface.interfaceName`](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceName)
    pub fn interface_name(&self) -> DOMString {
        self.inner.get("interfaceName").as_::<DOMString>()
    }
}
impl USBAlternateInterface {
    /// Getter of the `endpoints` attribute.
    /// [`USBAlternateInterface.endpoints`](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/endpoints)
    pub fn endpoints(&self) -> FrozenArray<USBEndpoint> {
        self.inner
            .get("endpoints")
            .as_::<FrozenArray<USBEndpoint>>()
    }
}
