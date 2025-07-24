use super::*;

/// The TimeEvent class.
/// [`TimeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TimeEvent {
    inner: Event,
}
impl FromVal for TimeEvent {
    fn from_val(v: &Any) -> Self {
        TimeEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TimeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TimeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TimeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TimeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TimeEvent> for Any {
    fn from(s: TimeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TimeEvent> for Any {
    fn from(s: &TimeEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TimeEvent);

impl TimeEvent {
    /// Getter of the `view` attribute.
    /// [`TimeEvent.view`](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/view)
    pub fn view(&self) -> Any {
        self.inner.get("view").as_::<Any>()
    }
}
impl TimeEvent {
    /// Getter of the `detail` attribute.
    /// [`TimeEvent.detail`](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/detail)
    pub fn detail(&self) -> i32 {
        self.inner.get("detail").as_::<i32>()
    }
}
impl TimeEvent {
    /// The initTimeEvent method.
    /// [`TimeEvent.initTimeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)
    pub fn init_time_event(
        &self,
        type_arg: &DOMString,
        view_arg: &Window,
        detail_arg: i32,
    ) -> Undefined {
        self.inner
            .call(
                "initTimeEvent",
                &[type_arg.into(), view_arg.into(), detail_arg.into()],
            )
            .as_::<Undefined>()
    }
}
