use super::*;

#[derive(Clone, Debug)]
pub struct ProfilerTrace {
    inner: emlite::Val,
}
impl FromVal for ProfilerTrace {
    fn from_val(v: &emlite::Val) -> Self {
        ProfilerTrace { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ProfilerTrace {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ProfilerTrace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ProfilerTrace> for emlite::Val {
    fn from(s: ProfilerTrace) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ProfilerTrace {
    pub fn resources(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("resources")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_resources(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("resources", value);
    }
}
impl ProfilerTrace {
    pub fn frames(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("frames")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_frames(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("frames", value);
    }
}
impl ProfilerTrace {
    pub fn stacks(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("stacks")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_stacks(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("stacks", value);
    }
}
impl ProfilerTrace {
    pub fn samples(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("samples")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_samples(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("samples", value);
    }
}
#[derive(Clone, Debug)]
pub struct Profiler {
    inner: EventTarget,
}
impl FromVal for Profiler {
    fn from_val(v: &emlite::Val) -> Self {
        Profiler {
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
impl std::ops::Deref for Profiler {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Profiler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Profiler> for emlite::Val {
    fn from(s: Profiler) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Profiler {
    pub fn sample_interval(&self) -> jsbind::Any {
        self.inner.get("sampleInterval").as_::<jsbind::Any>()
    }
}
impl Profiler {
    pub fn stopped(&self) -> bool {
        self.inner.get("stopped").as_::<bool>()
    }
}

impl Profiler {
    pub fn new(options: jsbind::Any) -> Profiler {
        Self {
            inner: emlite::Val::global("Profiler")
                .new(&[options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl Profiler {
    pub fn stop(&self) -> jsbind::Promise {
        self.inner.call("stop", &[]).as_::<jsbind::Promise>()
    }
}
