use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MemoryMeasurement {
    inner: Any,
}
impl FromVal for MemoryMeasurement {
    fn from_val(v: &Any) -> Self {
        MemoryMeasurement { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MemoryMeasurement {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MemoryMeasurement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MemoryMeasurement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MemoryMeasurement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MemoryMeasurement> for Any {
    fn from(s: MemoryMeasurement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MemoryMeasurement> for Any {
    fn from(s: &MemoryMeasurement) -> Any {
        s.inner.clone()
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
    pub fn breakdown(&self) -> Sequence<Any> {
        self.inner.get("breakdown").as_::<Sequence<Any>>()
    }

    pub fn set_breakdown(&mut self, value: &Sequence<Any>) {
        self.inner.set("breakdown", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceMarkOptions {
    inner: Any,
}
impl FromVal for PerformanceMarkOptions {
    fn from_val(v: &Any) -> Self {
        PerformanceMarkOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceMarkOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceMarkOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PerformanceMarkOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceMarkOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceMarkOptions> for Any {
    fn from(s: PerformanceMarkOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceMarkOptions> for Any {
    fn from(s: &PerformanceMarkOptions) -> Any {
        s.inner.clone()
    }
}

impl PerformanceMarkOptions {
    pub fn detail(&self) -> Any {
        self.inner.get("detail").as_::<Any>()
    }

    pub fn set_detail(&mut self, value: &Any) {
        self.inner.set("detail", value);
    }
}
impl PerformanceMarkOptions {
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }

    pub fn set_start_time(&mut self, value: &Any) {
        self.inner.set("startTime", value);
    }
}
/// The Performance class.
/// [`Performance`](https://developer.mozilla.org/en-US/docs/Web/API/Performance)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Performance {
    inner: EventTarget,
}
impl FromVal for Performance {
    fn from_val(v: &Any) -> Self {
        Performance {
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
impl AsRef<Any> for Performance {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Performance {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Performance> for Any {
    fn from(s: Performance) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Performance> for Any {
    fn from(s: &Performance) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Performance);

impl Performance {
    /// The now method.
    /// [`Performance.now`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now)
    pub fn now(&self) -> Any {
        self.inner.call("now", &[]).as_::<Any>()
    }
}
impl Performance {
    /// Getter of the `timeOrigin` attribute.
    /// [`Performance.timeOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin)
    pub fn time_origin(&self) -> Any {
        self.inner.get("timeOrigin").as_::<Any>()
    }
}
impl Performance {
    /// The toJSON method.
    /// [`Performance.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl Performance {
    /// Getter of the `eventCounts` attribute.
    /// [`Performance.eventCounts`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/eventCounts)
    pub fn event_counts(&self) -> EventCounts {
        self.inner.get("eventCounts").as_::<EventCounts>()
    }
}
impl Performance {
    /// Getter of the `interactionCount` attribute.
    /// [`Performance.interactionCount`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/interactionCount)
    pub fn interaction_count(&self) -> u64 {
        self.inner.get("interactionCount").as_::<u64>()
    }
}
impl Performance {
    /// Getter of the `timing` attribute.
    /// [`Performance.timing`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timing)
    pub fn timing(&self) -> PerformanceTiming {
        self.inner.get("timing").as_::<PerformanceTiming>()
    }
}
impl Performance {
    /// Getter of the `navigation` attribute.
    /// [`Performance.navigation`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/navigation)
    pub fn navigation(&self) -> PerformanceNavigation {
        self.inner.get("navigation").as_::<PerformanceNavigation>()
    }
}
impl Performance {
    /// The measureUserAgentSpecificMemory method.
    /// [`Performance.measureUserAgentSpecificMemory`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measureUserAgentSpecificMemory)
    pub fn measure_user_agent_specific_memory(&self) -> Promise<MemoryMeasurement> {
        self.inner
            .call("measureUserAgentSpecificMemory", &[])
            .as_::<Promise<MemoryMeasurement>>()
    }
}
impl Performance {
    /// The getEntries method.
    /// [`Performance.getEntries`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntries)
    pub fn get_entries(&self) -> Any {
        self.inner.call("getEntries", &[]).as_::<Any>()
    }
}
impl Performance {
    /// The getEntriesByType method.
    /// [`Performance.getEntriesByType`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByType)
    pub fn get_entries_by_type(&self, type_: &str) -> Any {
        self.inner
            .call("getEntriesByType", &[type_.into()])
            .as_::<Any>()
    }
}
impl Performance {
    /// The getEntriesByName method.
    /// [`Performance.getEntriesByName`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByName)
    pub fn get_entries_by_name0(&self, name: &str) -> Any {
        self.inner
            .call("getEntriesByName", &[name.into()])
            .as_::<Any>()
    }
    /// The getEntriesByName method.
    /// [`Performance.getEntriesByName`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByName)
    pub fn get_entries_by_name1(&self, name: &str, type_: &str) -> Any {
        self.inner
            .call("getEntriesByName", &[name.into(), type_.into()])
            .as_::<Any>()
    }
}
impl Performance {
    /// The clearResourceTimings method.
    /// [`Performance.clearResourceTimings`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearResourceTimings)
    pub fn clear_resource_timings(&self) -> Undefined {
        self.inner
            .call("clearResourceTimings", &[])
            .as_::<Undefined>()
    }
}
impl Performance {
    /// The setResourceTimingBufferSize method.
    /// [`Performance.setResourceTimingBufferSize`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/setResourceTimingBufferSize)
    pub fn set_resource_timing_buffer_size(&self, max_size: u32) -> Undefined {
        self.inner
            .call("setResourceTimingBufferSize", &[max_size.into()])
            .as_::<Undefined>()
    }
}
impl Performance {
    /// Getter of the `onresourcetimingbufferfull` attribute.
    /// [`Performance.onresourcetimingbufferfull`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/onresourcetimingbufferfull)
    pub fn onresourcetimingbufferfull(&self) -> Any {
        self.inner.get("onresourcetimingbufferfull").as_::<Any>()
    }

    /// Setter of the `onresourcetimingbufferfull` attribute.
    /// [`Performance.onresourcetimingbufferfull`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/onresourcetimingbufferfull)
    pub fn set_onresourcetimingbufferfull(&mut self, value: &Any) {
        self.inner.set("onresourcetimingbufferfull", value);
    }
}
impl Performance {
    /// The mark method.
    /// [`Performance.mark`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/mark)
    pub fn mark0(&self, mark_name: &str) -> PerformanceMark {
        self.inner
            .call("mark", &[mark_name.into()])
            .as_::<PerformanceMark>()
    }
    /// The mark method.
    /// [`Performance.mark`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/mark)
    pub fn mark1(&self, mark_name: &str, mark_options: &PerformanceMarkOptions) -> PerformanceMark {
        self.inner
            .call("mark", &[mark_name.into(), mark_options.into()])
            .as_::<PerformanceMark>()
    }
}
impl Performance {
    /// The clearMarks method.
    /// [`Performance.clearMarks`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMarks)
    pub fn clear_marks0(&self) -> Undefined {
        self.inner.call("clearMarks", &[]).as_::<Undefined>()
    }
    /// The clearMarks method.
    /// [`Performance.clearMarks`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMarks)
    pub fn clear_marks1(&self, mark_name: &str) -> Undefined {
        self.inner
            .call("clearMarks", &[mark_name.into()])
            .as_::<Undefined>()
    }
}
impl Performance {
    /// The measure method.
    /// [`Performance.measure`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)
    pub fn measure0(&self, measure_name: &str) -> PerformanceMeasure {
        self.inner
            .call("measure", &[measure_name.into()])
            .as_::<PerformanceMeasure>()
    }
    /// The measure method.
    /// [`Performance.measure`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)
    pub fn measure1(
        &self,
        measure_name: &str,
        start_or_measure_options: &Any,
    ) -> PerformanceMeasure {
        self.inner
            .call(
                "measure",
                &[measure_name.into(), start_or_measure_options.into()],
            )
            .as_::<PerformanceMeasure>()
    }
    /// The measure method.
    /// [`Performance.measure`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)
    pub fn measure2(
        &self,
        measure_name: &str,
        start_or_measure_options: &Any,
        end_mark: &str,
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
    /// The clearMeasures method.
    /// [`Performance.clearMeasures`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMeasures)
    pub fn clear_measures0(&self) -> Undefined {
        self.inner.call("clearMeasures", &[]).as_::<Undefined>()
    }
    /// The clearMeasures method.
    /// [`Performance.clearMeasures`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMeasures)
    pub fn clear_measures1(&self, measure_name: &str) -> Undefined {
        self.inner
            .call("clearMeasures", &[measure_name.into()])
            .as_::<Undefined>()
    }
}
