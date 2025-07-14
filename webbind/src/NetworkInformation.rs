use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NetworkInformation {
    inner: EventTarget,
}
impl FromVal for NetworkInformation {
    fn from_val(v: &emlite::Val) -> Self {
        NetworkInformation {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NetworkInformation {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NetworkInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NetworkInformation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NetworkInformation {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NetworkInformation> for emlite::Val {
    fn from(s: NetworkInformation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NetworkInformation);

impl NetworkInformation {
    pub fn type_(&self) -> ConnectionType {
        self.inner.get("type").as_::<ConnectionType>()
    }
}
impl NetworkInformation {
    pub fn effective_type(&self) -> EffectiveConnectionType {
        self.inner
            .get("effectiveType")
            .as_::<EffectiveConnectionType>()
    }
}
impl NetworkInformation {
    pub fn downlink_max(&self) -> jsbind::Any {
        self.inner.get("downlinkMax").as_::<jsbind::Any>()
    }
}
impl NetworkInformation {
    pub fn downlink(&self) -> jsbind::Any {
        self.inner.get("downlink").as_::<jsbind::Any>()
    }
}
impl NetworkInformation {
    pub fn rtt(&self) -> jsbind::Any {
        self.inner.get("rtt").as_::<jsbind::Any>()
    }
}
impl NetworkInformation {
    pub fn onchange(&self) -> jsbind::Any {
        self.inner.get("onchange").as_::<jsbind::Any>()
    }

    pub fn set_onchange(&mut self, value: jsbind::Any) {
        self.inner.set("onchange", value);
    }
}
impl NetworkInformation {
    pub fn save_data(&self) -> bool {
        self.inner.get("saveData").as_::<bool>()
    }
}
