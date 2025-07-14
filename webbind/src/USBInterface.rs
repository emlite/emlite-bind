use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBInterface {
    inner: emlite::Val,
}
impl FromVal for USBInterface {
    fn from_val(v: &emlite::Val) -> Self {
        USBInterface {
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
impl core::ops::Deref for USBInterface {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBInterface {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBInterface {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USBInterface> for emlite::Val {
    fn from(s: USBInterface) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(USBInterface);

impl USBInterface {
    pub fn new(configuration: USBConfiguration, interface_number: u8) -> USBInterface {
        Self {
            inner: emlite::Val::global("USBInterface")
                .new(&[configuration.into(), interface_number.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl USBInterface {
    pub fn interface_number(&self) -> u8 {
        self.inner.get("interfaceNumber").as_::<u8>()
    }
}
impl USBInterface {
    pub fn alternate(&self) -> USBAlternateInterface {
        self.inner.get("alternate").as_::<USBAlternateInterface>()
    }
}
impl USBInterface {
    pub fn alternates(&self) -> jsbind::FrozenArray<USBAlternateInterface> {
        self.inner
            .get("alternates")
            .as_::<jsbind::FrozenArray<USBAlternateInterface>>()
    }
}
impl USBInterface {
    pub fn claimed(&self) -> bool {
        self.inner.get("claimed").as_::<bool>()
    }
}
