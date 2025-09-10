use super::*;

/// The BatteryManager class.
/// [`BatteryManager`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BatteryManager {
    inner: EventTarget,
}

impl FromVal for BatteryManager {
    fn from_val(v: &Any) -> Self {
        BatteryManager {
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

impl AsRef<Any> for BatteryManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BatteryManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BatteryManager> for Any {
    fn from(s: BatteryManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BatteryManager> for Any {
    fn from(s: &BatteryManager) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BatteryManager);

impl BatteryManager {
    /// Getter of the `charging` attribute.
    /// [`BatteryManager.charging`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/charging)
    pub fn charging(&self) -> bool {
        self.inner.get("charging").as_::<bool>()
    }
}
impl BatteryManager {
    /// Getter of the `chargingTime` attribute.
    /// [`BatteryManager.chargingTime`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/chargingTime)
    pub fn charging_time(&self) -> f64 {
        self.inner.get("chargingTime").as_::<f64>()
    }
}
impl BatteryManager {
    /// Getter of the `dischargingTime` attribute.
    /// [`BatteryManager.dischargingTime`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/dischargingTime)
    pub fn discharging_time(&self) -> f64 {
        self.inner.get("dischargingTime").as_::<f64>()
    }
}
impl BatteryManager {
    /// Getter of the `level` attribute.
    /// [`BatteryManager.level`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/level)
    pub fn level(&self) -> f64 {
        self.inner.get("level").as_::<f64>()
    }
}
impl BatteryManager {
    /// Getter of the `onchargingchange` attribute.
    /// [`BatteryManager.onchargingchange`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)
    pub fn onchargingchange(&self) -> Any {
        self.inner.get("onchargingchange").as_::<Any>()
    }

    /// Setter of the `onchargingchange` attribute.
    /// [`BatteryManager.onchargingchange`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)
    pub fn set_onchargingchange(&mut self, value: &Any) {
        self.inner.set("onchargingchange", value);
    }
}
impl BatteryManager {
    /// Getter of the `onchargingtimechange` attribute.
    /// [`BatteryManager.onchargingtimechange`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)
    pub fn onchargingtimechange(&self) -> Any {
        self.inner.get("onchargingtimechange").as_::<Any>()
    }

    /// Setter of the `onchargingtimechange` attribute.
    /// [`BatteryManager.onchargingtimechange`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)
    pub fn set_onchargingtimechange(&mut self, value: &Any) {
        self.inner.set("onchargingtimechange", value);
    }
}
impl BatteryManager {
    /// Getter of the `ondischargingtimechange` attribute.
    /// [`BatteryManager.ondischargingtimechange`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)
    pub fn ondischargingtimechange(&self) -> Any {
        self.inner.get("ondischargingtimechange").as_::<Any>()
    }

    /// Setter of the `ondischargingtimechange` attribute.
    /// [`BatteryManager.ondischargingtimechange`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)
    pub fn set_ondischargingtimechange(&mut self, value: &Any) {
        self.inner.set("ondischargingtimechange", value);
    }
}
impl BatteryManager {
    /// Getter of the `onlevelchange` attribute.
    /// [`BatteryManager.onlevelchange`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)
    pub fn onlevelchange(&self) -> Any {
        self.inner.get("onlevelchange").as_::<Any>()
    }

    /// Setter of the `onlevelchange` attribute.
    /// [`BatteryManager.onlevelchange`](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)
    pub fn set_onlevelchange(&mut self, value: &Any) {
        self.inner.set("onlevelchange", value);
    }
}
