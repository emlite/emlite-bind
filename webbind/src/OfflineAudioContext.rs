use super::*;




/// The OfflineAudioContext class.
/// [`OfflineAudioContext`](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OfflineAudioContext {
    inner: BaseAudioContext,
}

impl FromVal for OfflineAudioContext {
    fn from_val(v: &Any) -> Self {
        OfflineAudioContext { inner: BaseAudioContext::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for OfflineAudioContext {
    type Target = BaseAudioContext;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OfflineAudioContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OfflineAudioContext {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OfflineAudioContext {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<OfflineAudioContext> for Any {
    fn from(s: OfflineAudioContext) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OfflineAudioContext> for Any {
    fn from(s: &OfflineAudioContext) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(OfflineAudioContext);



impl OfflineAudioContext {
    /// The `new OfflineAudioContext(..)` constructor, creating a new OfflineAudioContext instance
    pub fn new(number_of_channels: u32, length: u32, sample_rate: f32) -> OfflineAudioContext {
        Self {
            inner: Any::global("OfflineAudioContext").new(&[number_of_channels.into(), length.into(), sample_rate.into()]).as_::<BaseAudioContext>(),
        }
    }

}
impl OfflineAudioContext {
    /// The startRendering method.
    /// [`OfflineAudioContext.startRendering`](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/startRendering)
    pub fn start_rendering(&self, ) -> Promise<AudioBuffer> {
        self.inner.call("startRendering", &[]).as_::<Promise<AudioBuffer>>()
    }
}
impl OfflineAudioContext {
    /// The resume method.
    /// [`OfflineAudioContext.resume`](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/resume)
    pub fn resume(&self, ) -> Promise<Undefined> {
        self.inner.call("resume", &[]).as_::<Promise<Undefined>>()
    }
}
impl OfflineAudioContext {
    /// The suspend method.
    /// [`OfflineAudioContext.suspend`](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/suspend)
    pub fn suspend(&self, suspend_time: f64) -> Promise<Undefined> {
        self.inner.call("suspend", &[suspend_time.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl OfflineAudioContext {
    /// Getter of the `length` attribute.
    /// [`OfflineAudioContext.length`](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl OfflineAudioContext {
    /// Getter of the `oncomplete` attribute.
    /// [`OfflineAudioContext.oncomplete`](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/oncomplete)
    pub fn oncomplete(&self) -> Any {
        self.inner.get("oncomplete").as_::<Any>()
    }

    /// Setter of the `oncomplete` attribute.
    /// [`OfflineAudioContext.oncomplete`](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/oncomplete)
    pub fn set_oncomplete(&mut self, value: &Any) {
        self.inner.set("oncomplete", value);
    }
}
