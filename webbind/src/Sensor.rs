use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Sensor {
    inner: EventTarget,
}
impl FromVal for Sensor {
    fn from_val(v: &emlite::Val) -> Self {
        Sensor {
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
impl core::ops::Deref for Sensor {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Sensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Sensor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Sensor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Sensor> for emlite::Val {
    fn from(s: Sensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Sensor> for emlite::Val {
    fn from(s: &Sensor) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Sensor);

impl Sensor {
    pub fn activated(&self) -> bool {
        self.inner.get("activated").as_::<bool>()
    }
}
impl Sensor {
    pub fn has_reading(&self) -> bool {
        self.inner.get("hasReading").as_::<bool>()
    }
}
impl Sensor {
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }
}
impl Sensor {
    pub fn start(&self) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }
}
impl Sensor {
    pub fn stop(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
impl Sensor {
    pub fn onreading(&self) -> Any {
        self.inner.get("onreading").as_::<Any>()
    }

    pub fn set_onreading(&mut self, value: Any) {
        self.inner.set("onreading", value);
    }
}
impl Sensor {
    pub fn onactivate(&self) -> Any {
        self.inner.get("onactivate").as_::<Any>()
    }

    pub fn set_onactivate(&mut self, value: Any) {
        self.inner.set("onactivate", value);
    }
}
impl Sensor {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: Any) {
        self.inner.set("onerror", value);
    }
}
