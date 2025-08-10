use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProfilerTrace {
    inner: Any,
}
impl FromVal for ProfilerTrace {
    fn from_val(v: &Any) -> Self {
        ProfilerTrace { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ProfilerTrace {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ProfilerTrace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ProfilerTrace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ProfilerTrace {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ProfilerTrace> for Any {
    fn from(s: ProfilerTrace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ProfilerTrace> for Any {
    fn from(s: &ProfilerTrace) -> Any {
        s.inner.clone()
    }
}

impl ProfilerTrace {
    pub fn resources(&self) -> TypedArray<Any> {
        self.inner.get("resources").as_::<TypedArray<Any>>()
    }

    pub fn set_resources(&mut self, value: &TypedArray<Any>) {
        self.inner.set("resources", value);
    }
}
impl ProfilerTrace {
    pub fn frames(&self) -> TypedArray<ProfilerFrame> {
        self.inner.get("frames").as_::<TypedArray<ProfilerFrame>>()
    }

    pub fn set_frames(&mut self, value: &TypedArray<ProfilerFrame>) {
        self.inner.set("frames", value);
    }
}
impl ProfilerTrace {
    pub fn stacks(&self) -> TypedArray<ProfilerStack> {
        self.inner.get("stacks").as_::<TypedArray<ProfilerStack>>()
    }

    pub fn set_stacks(&mut self, value: &TypedArray<ProfilerStack>) {
        self.inner.set("stacks", value);
    }
}
impl ProfilerTrace {
    pub fn samples(&self) -> TypedArray<ProfilerSample> {
        self.inner
            .get("samples")
            .as_::<TypedArray<ProfilerSample>>()
    }

    pub fn set_samples(&mut self, value: &TypedArray<ProfilerSample>) {
        self.inner.set("samples", value);
    }
}
/// The Profiler class.
/// [`Profiler`](https://developer.mozilla.org/en-US/docs/Web/API/Profiler)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Profiler {
    inner: EventTarget,
}
impl FromVal for Profiler {
    fn from_val(v: &Any) -> Self {
        Profiler {
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
impl core::ops::Deref for Profiler {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Profiler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Profiler {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Profiler {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Profiler> for Any {
    fn from(s: Profiler) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Profiler> for Any {
    fn from(s: &Profiler) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Profiler);

impl Profiler {
    /// Getter of the `sampleInterval` attribute.
    /// [`Profiler.sampleInterval`](https://developer.mozilla.org/en-US/docs/Web/API/Profiler/sampleInterval)
    pub fn sample_interval(&self) -> Any {
        self.inner.get("sampleInterval").as_::<Any>()
    }
}
impl Profiler {
    /// Getter of the `stopped` attribute.
    /// [`Profiler.stopped`](https://developer.mozilla.org/en-US/docs/Web/API/Profiler/stopped)
    pub fn stopped(&self) -> bool {
        self.inner.get("stopped").as_::<bool>()
    }
}

impl Profiler {
    /// The `new Profiler(..)` constructor, creating a new Profiler instance
    pub fn new(options: &ProfilerInitOptions) -> Profiler {
        Self {
            inner: Any::global("Profiler")
                .new(&[options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl Profiler {
    /// The stop method.
    /// [`Profiler.stop`](https://developer.mozilla.org/en-US/docs/Web/API/Profiler/stop)
    pub fn stop(&self) -> Promise<ProfilerTrace> {
        self.inner.call("stop", &[]).as_::<Promise<ProfilerTrace>>()
    }
}
