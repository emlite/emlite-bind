use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BatteryManager {
    inner: EventTarget,
}
impl FromVal for BatteryManager {
    fn from_val(v: &emlite::Val) -> Self {
        BatteryManager {
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
impl core::ops::Deref for BatteryManager {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BatteryManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BatteryManager {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BatteryManager {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BatteryManager> for emlite::Val {
    fn from(s: BatteryManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BatteryManager);

impl BatteryManager {
    pub fn charging(&self) -> bool {
        self.inner.get("charging").as_::<bool>()
    }
}
impl BatteryManager {
    pub fn charging_time(&self) -> f64 {
        self.inner.get("chargingTime").as_::<f64>()
    }
}
impl BatteryManager {
    pub fn discharging_time(&self) -> f64 {
        self.inner.get("dischargingTime").as_::<f64>()
    }
}
impl BatteryManager {
    pub fn level(&self) -> f64 {
        self.inner.get("level").as_::<f64>()
    }
}
impl BatteryManager {
    pub fn onchargingchange(&self) -> jsbind::Any {
        self.inner.get("onchargingchange").as_::<jsbind::Any>()
    }

    pub fn set_onchargingchange(&mut self, value: jsbind::Any) {
        self.inner.set("onchargingchange", value);
    }
}
impl BatteryManager {
    pub fn onchargingtimechange(&self) -> jsbind::Any {
        self.inner.get("onchargingtimechange").as_::<jsbind::Any>()
    }

    pub fn set_onchargingtimechange(&mut self, value: jsbind::Any) {
        self.inner.set("onchargingtimechange", value);
    }
}
impl BatteryManager {
    pub fn ondischargingtimechange(&self) -> jsbind::Any {
        self.inner
            .get("ondischargingtimechange")
            .as_::<jsbind::Any>()
    }

    pub fn set_ondischargingtimechange(&mut self, value: jsbind::Any) {
        self.inner.set("ondischargingtimechange", value);
    }
}
impl BatteryManager {
    pub fn onlevelchange(&self) -> jsbind::Any {
        self.inner.get("onlevelchange").as_::<jsbind::Any>()
    }

    pub fn set_onlevelchange(&mut self, value: jsbind::Any) {
        self.inner.set("onlevelchange", value);
    }
}
