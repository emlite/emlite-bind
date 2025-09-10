use super::*;

/// The PerformanceServerTiming class.
/// [`PerformanceServerTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceServerTiming {
    inner: Any,
}

impl FromVal for PerformanceServerTiming {
    fn from_val(v: &Any) -> Self {
        PerformanceServerTiming {
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

impl core::ops::Deref for PerformanceServerTiming {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PerformanceServerTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PerformanceServerTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformanceServerTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PerformanceServerTiming> for Any {
    fn from(s: PerformanceServerTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformanceServerTiming> for Any {
    fn from(s: &PerformanceServerTiming) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PerformanceServerTiming);

impl PerformanceServerTiming {
    /// Getter of the `name` attribute.
    /// [`PerformanceServerTiming.name`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl PerformanceServerTiming {
    /// Getter of the `duration` attribute.
    /// [`PerformanceServerTiming.duration`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/duration)
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }
}
impl PerformanceServerTiming {
    /// Getter of the `description` attribute.
    /// [`PerformanceServerTiming.description`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/description)
    pub fn description(&self) -> JsString {
        self.inner.get("description").as_::<JsString>()
    }
}
impl PerformanceServerTiming {
    /// The toJSON method.
    /// [`PerformanceServerTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
