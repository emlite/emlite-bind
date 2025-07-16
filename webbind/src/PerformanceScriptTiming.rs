use super::*;

/// The PerformanceScriptTiming class.
/// [`PerformanceScriptTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceScriptTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceScriptTiming {
    fn from_val(v: &Any) -> Self {
        PerformanceScriptTiming {
            inner: PerformanceEntry::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceScriptTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceScriptTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PerformanceScriptTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceScriptTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceScriptTiming> for Any {
    fn from(s: PerformanceScriptTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceScriptTiming> for Any {
    fn from(s: &PerformanceScriptTiming) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceScriptTiming);

impl PerformanceScriptTiming {
    /// Getter of the `startTime` attribute.
    /// [`PerformanceScriptTiming.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/startTime)
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `duration` attribute.
    /// [`PerformanceScriptTiming.duration`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/duration)
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `name` attribute.
    /// [`PerformanceScriptTiming.name`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `entryType` attribute.
    /// [`PerformanceScriptTiming.entryType`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/entryType)
    pub fn entry_type(&self) -> String {
        self.inner.get("entryType").as_::<String>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `invokerType` attribute.
    /// [`PerformanceScriptTiming.invokerType`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/invokerType)
    pub fn invoker_type(&self) -> ScriptInvokerType {
        self.inner.get("invokerType").as_::<ScriptInvokerType>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `invoker` attribute.
    /// [`PerformanceScriptTiming.invoker`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/invoker)
    pub fn invoker(&self) -> String {
        self.inner.get("invoker").as_::<String>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `executionStart` attribute.
    /// [`PerformanceScriptTiming.executionStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/executionStart)
    pub fn execution_start(&self) -> Any {
        self.inner.get("executionStart").as_::<Any>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `sourceURL` attribute.
    /// [`PerformanceScriptTiming.sourceURL`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/sourceURL)
    pub fn source_url(&self) -> String {
        self.inner.get("sourceURL").as_::<String>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `sourceFunctionName` attribute.
    /// [`PerformanceScriptTiming.sourceFunctionName`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/sourceFunctionName)
    pub fn source_function_name(&self) -> String {
        self.inner.get("sourceFunctionName").as_::<String>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `sourceCharPosition` attribute.
    /// [`PerformanceScriptTiming.sourceCharPosition`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/sourceCharPosition)
    pub fn source_char_position(&self) -> i64 {
        self.inner.get("sourceCharPosition").as_::<i64>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `pauseDuration` attribute.
    /// [`PerformanceScriptTiming.pauseDuration`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/pauseDuration)
    pub fn pause_duration(&self) -> Any {
        self.inner.get("pauseDuration").as_::<Any>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `forcedStyleAndLayoutDuration` attribute.
    /// [`PerformanceScriptTiming.forcedStyleAndLayoutDuration`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/forcedStyleAndLayoutDuration)
    pub fn forced_style_and_layout_duration(&self) -> Any {
        self.inner.get("forcedStyleAndLayoutDuration").as_::<Any>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `window` attribute.
    /// [`PerformanceScriptTiming.window`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/window)
    pub fn window(&self) -> Window {
        self.inner.get("window").as_::<Window>()
    }
}
impl PerformanceScriptTiming {
    /// Getter of the `windowAttribution` attribute.
    /// [`PerformanceScriptTiming.windowAttribution`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/windowAttribution)
    pub fn window_attribution(&self) -> ScriptWindowAttribution {
        self.inner
            .get("windowAttribution")
            .as_::<ScriptWindowAttribution>()
    }
}
impl PerformanceScriptTiming {
    /// The toJSON method.
    /// [`PerformanceScriptTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceScriptTiming/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
