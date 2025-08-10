use super::*;

/// The PressureRecord class.
/// [`PressureRecord`](https://developer.mozilla.org/en-US/docs/Web/API/PressureRecord)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PressureRecord {
    inner: Any,
}

impl FromVal for PressureRecord {
    fn from_val(v: &Any) -> Self {
        PressureRecord {
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

impl core::ops::Deref for PressureRecord {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PressureRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PressureRecord {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PressureRecord {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PressureRecord> for Any {
    fn from(s: PressureRecord) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PressureRecord> for Any {
    fn from(s: &PressureRecord) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PressureRecord);

impl PressureRecord {
    /// Getter of the `source` attribute.
    /// [`PressureRecord.source`](https://developer.mozilla.org/en-US/docs/Web/API/PressureRecord/source)
    pub fn source(&self) -> PressureSource {
        self.inner.get("source").as_::<PressureSource>()
    }
}
impl PressureRecord {
    /// Getter of the `state` attribute.
    /// [`PressureRecord.state`](https://developer.mozilla.org/en-US/docs/Web/API/PressureRecord/state)
    pub fn state(&self) -> PressureState {
        self.inner.get("state").as_::<PressureState>()
    }
}
impl PressureRecord {
    /// Getter of the `time` attribute.
    /// [`PressureRecord.time`](https://developer.mozilla.org/en-US/docs/Web/API/PressureRecord/time)
    pub fn time(&self) -> Any {
        self.inner.get("time").as_::<Any>()
    }
}
impl PressureRecord {
    /// The toJSON method.
    /// [`PressureRecord.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PressureRecord/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
