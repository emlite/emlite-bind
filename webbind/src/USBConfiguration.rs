use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBConfiguration {
    inner: emlite::Val,
}
impl FromVal for USBConfiguration {
    fn from_val(v: &emlite::Val) -> Self {
        USBConfiguration {
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
impl core::ops::Deref for USBConfiguration {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBConfiguration {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBConfiguration {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USBConfiguration> for emlite::Val {
    fn from(s: USBConfiguration) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(USBConfiguration);

impl USBConfiguration {
    pub fn new(device: USBDevice, configuration_value: u8) -> USBConfiguration {
        Self {
            inner: emlite::Val::global("USBConfiguration")
                .new(&[device.into(), configuration_value.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl USBConfiguration {
    pub fn configuration_value(&self) -> u8 {
        self.inner.get("configurationValue").as_::<u8>()
    }
}
impl USBConfiguration {
    pub fn configuration_name(&self) -> DOMString {
        self.inner.get("configurationName").as_::<DOMString>()
    }
}
impl USBConfiguration {
    pub fn interfaces(&self) -> FrozenArray<USBInterface> {
        self.inner
            .get("interfaces")
            .as_::<FrozenArray<USBInterface>>()
    }
}
