use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBAlternateInterface {
    inner: emlite::Val,
}
impl FromVal for USBAlternateInterface {
    fn from_val(v: &emlite::Val) -> Self {
        USBAlternateInterface {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for USBAlternateInterface {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBAlternateInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBAlternateInterface {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBAlternateInterface {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USBAlternateInterface> for emlite::Val {
    fn from(s: USBAlternateInterface) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&USBAlternateInterface> for emlite::Val {
    fn from(s: &USBAlternateInterface) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(USBAlternateInterface);

impl USBAlternateInterface {
    pub fn new(device_interface: USBInterface, alternate_setting: u8) -> USBAlternateInterface {
        Self {
            inner: emlite::Val::global("USBAlternateInterface")
                .new(&[device_interface.into(), alternate_setting.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl USBAlternateInterface {
    pub fn alternate_setting(&self) -> u8 {
        self.inner.get("alternateSetting").as_::<u8>()
    }
}
impl USBAlternateInterface {
    pub fn interface_class(&self) -> u8 {
        self.inner.get("interfaceClass").as_::<u8>()
    }
}
impl USBAlternateInterface {
    pub fn interface_subclass(&self) -> u8 {
        self.inner.get("interfaceSubclass").as_::<u8>()
    }
}
impl USBAlternateInterface {
    pub fn interface_protocol(&self) -> u8 {
        self.inner.get("interfaceProtocol").as_::<u8>()
    }
}
impl USBAlternateInterface {
    pub fn interface_name(&self) -> DOMString {
        self.inner.get("interfaceName").as_::<DOMString>()
    }
}
impl USBAlternateInterface {
    pub fn endpoints(&self) -> FrozenArray<USBEndpoint> {
        self.inner
            .get("endpoints")
            .as_::<FrozenArray<USBEndpoint>>()
    }
}
