use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<Sensor> for emlite::Val {
    fn from(s: Sensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn timestamp(&self) -> jsbind::Any {
        self.inner.get("timestamp").as_::<jsbind::Any>()
    }
}
impl Sensor {
    pub fn start(&self) -> jsbind::Undefined {
        self.inner.call("start", &[]).as_::<jsbind::Undefined>()
    }
}
impl Sensor {
    pub fn stop(&self) -> jsbind::Undefined {
        self.inner.call("stop", &[]).as_::<jsbind::Undefined>()
    }
}
impl Sensor {
    pub fn onreading(&self) -> jsbind::Any {
        self.inner.get("onreading").as_::<jsbind::Any>()
    }

    pub fn set_onreading(&mut self, value: jsbind::Any) {
        self.inner.set("onreading", value);
    }
}
impl Sensor {
    pub fn onactivate(&self) -> jsbind::Any {
        self.inner.get("onactivate").as_::<jsbind::Any>()
    }

    pub fn set_onactivate(&mut self, value: jsbind::Any) {
        self.inner.set("onactivate", value);
    }
}
impl Sensor {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
