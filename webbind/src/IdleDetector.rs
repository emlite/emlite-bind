use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdleOptions {
    inner: Any,
}
impl FromVal for IdleOptions {
    fn from_val(v: &Any) -> Self {
        IdleOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdleOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdleOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdleOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdleOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdleOptions> for Any {
    fn from(s: IdleOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdleOptions> for Any {
    fn from(s: &IdleOptions) -> Any {
        s.inner.clone()
    }
}

impl IdleOptions {
    pub fn threshold(&self) -> u64 {
        self.inner.get("threshold").as_::<u64>()
    }

    pub fn set_threshold(&mut self, value: u64) {
        self.inner.set("threshold", value);
    }
}
impl IdleOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
/// The IdleDetector class.
/// [`IdleDetector`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDetector)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdleDetector {
    inner: EventTarget,
}
impl FromVal for IdleDetector {
    fn from_val(v: &Any) -> Self {
        IdleDetector {
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
impl core::ops::Deref for IdleDetector {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdleDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdleDetector {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdleDetector {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdleDetector> for Any {
    fn from(s: IdleDetector) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdleDetector> for Any {
    fn from(s: &IdleDetector) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IdleDetector);

impl IdleDetector {
    /// The `new IdleDetector(..)` constructor, creating a new IdleDetector instance
    pub fn new() -> IdleDetector {
        Self {
            inner: Any::global("IdleDetector").new(&[]).as_::<EventTarget>(),
        }
    }
}
impl IdleDetector {
    /// Getter of the `userState` attribute.
    /// [`IdleDetector.userState`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDetector/userState)
    pub fn user_state(&self) -> UserIdleState {
        self.inner.get("userState").as_::<UserIdleState>()
    }
}
impl IdleDetector {
    /// Getter of the `screenState` attribute.
    /// [`IdleDetector.screenState`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDetector/screenState)
    pub fn screen_state(&self) -> ScreenIdleState {
        self.inner.get("screenState").as_::<ScreenIdleState>()
    }
}
impl IdleDetector {
    /// Getter of the `onchange` attribute.
    /// [`IdleDetector.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDetector/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`IdleDetector.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDetector/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
impl IdleDetector {
    /// The requestPermission method.
    /// [`IdleDetector.requestPermission`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDetector/requestPermission)
    pub fn request_permission() -> Promise {
        Any::global("IdleDetector")
            .call("requestPermission", &[])
            .as_::<Promise>()
    }
}
impl IdleDetector {
    /// The start method.
    /// [`IdleDetector.start`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDetector/start)
    pub fn start0(&self) -> Promise {
        self.inner.call("start", &[]).as_::<Promise>()
    }
    /// The start method.
    /// [`IdleDetector.start`](https://developer.mozilla.org/en-US/docs/Web/API/IdleDetector/start)
    pub fn start1(&self, options: &IdleOptions) -> Promise {
        self.inner.call("start", &[options.into()]).as_::<Promise>()
    }
}
