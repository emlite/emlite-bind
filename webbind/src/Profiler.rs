use super::*;

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
