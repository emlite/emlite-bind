use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PerformanceScriptTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceScriptTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceScriptTiming {
            inner: PerformanceEntry::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<PerformanceScriptTiming> for emlite::Val {
    fn from(s: PerformanceScriptTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PerformanceScriptTiming {
    pub fn start_time(&self) -> jsbind::Any {
        self.inner.get("startTime").as_::<jsbind::Any>()
    }
}
impl PerformanceScriptTiming {
    pub fn duration(&self) -> jsbind::Any {
        self.inner.get("duration").as_::<jsbind::Any>()
    }
}
impl PerformanceScriptTiming {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl PerformanceScriptTiming {
    pub fn entry_type(&self) -> jsbind::DOMString {
        self.inner.get("entryType").as_::<jsbind::DOMString>()
    }
}
impl PerformanceScriptTiming {
    pub fn invoker_type(&self) -> ScriptInvokerType {
        self.inner.get("invokerType").as_::<ScriptInvokerType>()
    }
}
impl PerformanceScriptTiming {
    pub fn invoker(&self) -> jsbind::DOMString {
        self.inner.get("invoker").as_::<jsbind::DOMString>()
    }
}
impl PerformanceScriptTiming {
    pub fn execution_start(&self) -> jsbind::Any {
        self.inner.get("executionStart").as_::<jsbind::Any>()
    }
}
impl PerformanceScriptTiming {
    pub fn source_url(&self) -> jsbind::DOMString {
        self.inner.get("sourceURL").as_::<jsbind::DOMString>()
    }
}
impl PerformanceScriptTiming {
    pub fn source_function_name(&self) -> jsbind::DOMString {
        self.inner
            .get("sourceFunctionName")
            .as_::<jsbind::DOMString>()
    }
}
impl PerformanceScriptTiming {
    pub fn source_char_position(&self) -> i64 {
        self.inner.get("sourceCharPosition").as_::<i64>()
    }
}
impl PerformanceScriptTiming {
    pub fn pause_duration(&self) -> jsbind::Any {
        self.inner.get("pauseDuration").as_::<jsbind::Any>()
    }
}
impl PerformanceScriptTiming {
    pub fn forced_style_and_layout_duration(&self) -> jsbind::Any {
        self.inner
            .get("forcedStyleAndLayoutDuration")
            .as_::<jsbind::Any>()
    }
}
impl PerformanceScriptTiming {
    pub fn window(&self) -> Window {
        self.inner.get("window").as_::<Window>()
    }
}
impl PerformanceScriptTiming {
    pub fn window_attribution(&self) -> ScriptWindowAttribution {
        self.inner
            .get("windowAttribution")
            .as_::<ScriptWindowAttribution>()
    }
}
impl PerformanceScriptTiming {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
