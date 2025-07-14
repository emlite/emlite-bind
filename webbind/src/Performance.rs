use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MemoryMeasurement {
    inner: emlite::Val,
}
impl FromVal for MemoryMeasurement {
    fn from_val(v: &emlite::Val) -> Self {
        MemoryMeasurement { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MemoryMeasurement {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MemoryMeasurement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MemoryMeasurement> for emlite::Val {
    fn from(s: MemoryMeasurement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MemoryMeasurement {
    pub fn bytes(&self) -> u64 {
        self.inner.get("bytes").as_::<u64>()
    }

    pub fn set_bytes(&mut self, value: u64) {
        self.inner.set("bytes", value);
    }
}
impl MemoryMeasurement {
    pub fn breakdown(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("breakdown")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_breakdown(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("breakdown", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PerformanceMarkOptions {
    inner: emlite::Val,
}
impl FromVal for PerformanceMarkOptions {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceMarkOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceMarkOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceMarkOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PerformanceMarkOptions> for emlite::Val {
    fn from(s: PerformanceMarkOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PerformanceMarkOptions {
    pub fn detail(&self) -> jsbind::Any {
        self.inner.get("detail").as_::<jsbind::Any>()
    }

    pub fn set_detail(&mut self, value: jsbind::Any) {
        self.inner.set("detail", value);
    }
}
impl PerformanceMarkOptions {
    pub fn start_time(&self) -> jsbind::Any {
        self.inner.get("startTime").as_::<jsbind::Any>()
    }

    pub fn set_start_time(&mut self, value: jsbind::Any) {
        self.inner.set("startTime", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Performance {
    inner: EventTarget,
}
impl FromVal for Performance {
    fn from_val(v: &emlite::Val) -> Self {
        Performance {
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
impl core::ops::Deref for Performance {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Performance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Performance> for emlite::Val {
    fn from(s: Performance) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Performance {
    pub fn now(&self) -> jsbind::Any {
        self.inner.call("now", &[]).as_::<jsbind::Any>()
    }
}
impl Performance {
    pub fn time_origin(&self) -> jsbind::Any {
        self.inner.get("timeOrigin").as_::<jsbind::Any>()
    }
}
impl Performance {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
impl Performance {
    pub fn event_counts(&self) -> EventCounts {
        self.inner.get("eventCounts").as_::<EventCounts>()
    }
}
impl Performance {
    pub fn interaction_count(&self) -> u64 {
        self.inner.get("interactionCount").as_::<u64>()
    }
}
impl Performance {
    pub fn timing(&self) -> PerformanceTiming {
        self.inner.get("timing").as_::<PerformanceTiming>()
    }
}
impl Performance {
    pub fn navigation(&self) -> PerformanceNavigation {
        self.inner.get("navigation").as_::<PerformanceNavigation>()
    }
}
impl Performance {
    pub fn measure_user_agent_specific_memory(&self) -> jsbind::Promise {
        self.inner
            .call("measureUserAgentSpecificMemory", &[])
            .as_::<jsbind::Promise>()
    }
}
impl Performance {
    pub fn get_entries(&self) -> jsbind::Any {
        self.inner.call("getEntries", &[]).as_::<jsbind::Any>()
    }
}
impl Performance {
    pub fn get_entries_by_type(&self, type_: jsbind::DOMString) -> jsbind::Any {
        self.inner
            .call("getEntriesByType", &[type_.into()])
            .as_::<jsbind::Any>()
    }
}
impl Performance {
    pub fn get_entries_by_name0(&self, name: jsbind::DOMString) -> jsbind::Any {
        self.inner
            .call("getEntriesByName", &[name.into()])
            .as_::<jsbind::Any>()
    }

    pub fn get_entries_by_name1(
        &self,
        name: jsbind::DOMString,
        type_: jsbind::DOMString,
    ) -> jsbind::Any {
        self.inner
            .call("getEntriesByName", &[name.into(), type_.into()])
            .as_::<jsbind::Any>()
    }
}
impl Performance {
    pub fn clear_resource_timings(&self) -> jsbind::Undefined {
        self.inner
            .call("clearResourceTimings", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl Performance {
    pub fn set_resource_timing_buffer_size(&self, max_size: u32) -> jsbind::Undefined {
        self.inner
            .call("setResourceTimingBufferSize", &[max_size.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Performance {
    pub fn onresourcetimingbufferfull(&self) -> jsbind::Any {
        self.inner
            .get("onresourcetimingbufferfull")
            .as_::<jsbind::Any>()
    }

    pub fn set_onresourcetimingbufferfull(&mut self, value: jsbind::Any) {
        self.inner.set("onresourcetimingbufferfull", value);
    }
}
impl Performance {
    pub fn mark0(&self, mark_name: jsbind::DOMString) -> PerformanceMark {
        self.inner
            .call("mark", &[mark_name.into()])
            .as_::<PerformanceMark>()
    }

    pub fn mark1(
        &self,
        mark_name: jsbind::DOMString,
        mark_options: PerformanceMarkOptions,
    ) -> PerformanceMark {
        self.inner
            .call("mark", &[mark_name.into(), mark_options.into()])
            .as_::<PerformanceMark>()
    }
}
impl Performance {
    pub fn clear_marks0(&self) -> jsbind::Undefined {
        self.inner
            .call("clearMarks", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn clear_marks1(&self, mark_name: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("clearMarks", &[mark_name.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Performance {
    pub fn measure0(&self, measure_name: jsbind::DOMString) -> PerformanceMeasure {
        self.inner
            .call("measure", &[measure_name.into()])
            .as_::<PerformanceMeasure>()
    }

    pub fn measure1(
        &self,
        measure_name: jsbind::DOMString,
        start_or_measure_options: jsbind::Any,
    ) -> PerformanceMeasure {
        self.inner
            .call(
                "measure",
                &[measure_name.into(), start_or_measure_options.into()],
            )
            .as_::<PerformanceMeasure>()
    }

    pub fn measure2(
        &self,
        measure_name: jsbind::DOMString,
        start_or_measure_options: jsbind::Any,
        end_mark: jsbind::DOMString,
    ) -> PerformanceMeasure {
        self.inner
            .call(
                "measure",
                &[
                    measure_name.into(),
                    start_or_measure_options.into(),
                    end_mark.into(),
                ],
            )
            .as_::<PerformanceMeasure>()
    }
}
impl Performance {
    pub fn clear_measures0(&self) -> jsbind::Undefined {
        self.inner
            .call("clearMeasures", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn clear_measures1(&self, measure_name: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("clearMeasures", &[measure_name.into()])
            .as_::<jsbind::Undefined>()
    }
}
