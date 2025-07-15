use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBEndpoint {
    inner: emlite::Val,
}
impl FromVal for USBEndpoint {
    fn from_val(v: &emlite::Val) -> Self {
        USBEndpoint {
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
impl core::ops::Deref for USBEndpoint {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBEndpoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBEndpoint {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBEndpoint {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USBEndpoint> for emlite::Val {
    fn from(s: USBEndpoint) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(USBEndpoint);

impl USBEndpoint {
    pub fn new(
        alternate: USBAlternateInterface,
        endpoint_number: u8,
        direction: USBDirection,
    ) -> USBEndpoint {
        Self {
            inner: emlite::Val::global("USBEndpoint")
                .new(&[alternate.into(), endpoint_number.into(), direction.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl USBEndpoint {
    pub fn endpoint_number(&self) -> u8 {
        self.inner.get("endpointNumber").as_::<u8>()
    }
}
impl USBEndpoint {
    pub fn direction(&self) -> USBDirection {
        self.inner.get("direction").as_::<USBDirection>()
    }
}
impl USBEndpoint {
    pub fn type_(&self) -> USBEndpointType {
        self.inner.get("type").as_::<USBEndpointType>()
    }
}
impl USBEndpoint {
    pub fn packet_size(&self) -> u32 {
        self.inner.get("packetSize").as_::<u32>()
    }
}
