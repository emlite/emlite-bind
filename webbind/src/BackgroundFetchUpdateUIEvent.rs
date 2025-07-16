use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchUIOptions {
    inner: Any,
}
impl FromVal for BackgroundFetchUIOptions {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchUIOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BackgroundFetchUIOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchUIOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BackgroundFetchUIOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BackgroundFetchUIOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BackgroundFetchUIOptions> for Any {
    fn from(s: BackgroundFetchUIOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BackgroundFetchUIOptions> for Any {
    fn from(s: &BackgroundFetchUIOptions) -> Any {
        s.inner.clone()
    }
}

impl BackgroundFetchUIOptions {
    pub fn icons(&self) -> Sequence<Any> {
        self.inner.get("icons").as_::<Sequence<Any>>()
    }

    pub fn set_icons(&mut self, value: &Sequence<Any>) {
        self.inner.set("icons", value);
    }
}
impl BackgroundFetchUIOptions {
    pub fn title(&self) -> String {
        self.inner.get("title").as_::<String>()
    }

    pub fn set_title(&mut self, value: &str) {
        self.inner.set("title", value);
    }
}
/// The BackgroundFetchUpdateUIEvent class.
/// [`BackgroundFetchUpdateUIEvent`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchUpdateUIEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchUpdateUIEvent {
    inner: BackgroundFetchEvent,
}
impl FromVal for BackgroundFetchUpdateUIEvent {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchUpdateUIEvent {
            inner: BackgroundFetchEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BackgroundFetchUpdateUIEvent {
    type Target = BackgroundFetchEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchUpdateUIEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BackgroundFetchUpdateUIEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BackgroundFetchUpdateUIEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BackgroundFetchUpdateUIEvent> for Any {
    fn from(s: BackgroundFetchUpdateUIEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BackgroundFetchUpdateUIEvent> for Any {
    fn from(s: &BackgroundFetchUpdateUIEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BackgroundFetchUpdateUIEvent);

impl BackgroundFetchUpdateUIEvent {
    /// The `new BackgroundFetchUpdateUIEvent(..)` constructor, creating a new BackgroundFetchUpdateUIEvent instance
    pub fn new(type_: &str, init: &Any) -> BackgroundFetchUpdateUIEvent {
        Self {
            inner: Any::global("BackgroundFetchUpdateUIEvent")
                .new(&[type_.into(), init.into()])
                .as_::<BackgroundFetchEvent>(),
        }
    }
}
impl BackgroundFetchUpdateUIEvent {
    /// The updateUI method.
    /// [`BackgroundFetchUpdateUIEvent.updateUI`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchUpdateUIEvent/updateUI)
    pub fn update_ui0(&self) -> Promise {
        self.inner.call("updateUI", &[]).as_::<Promise>()
    }
    /// The updateUI method.
    /// [`BackgroundFetchUpdateUIEvent.updateUI`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchUpdateUIEvent/updateUI)
    pub fn update_ui1(&self, options: &BackgroundFetchUIOptions) -> Promise {
        self.inner
            .call("updateUI", &[options.into()])
            .as_::<Promise>()
    }
}
