use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TimeEvent {
    inner: Event,
}
impl FromVal for TimeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        TimeEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for TimeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TimeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TimeEvent> for emlite::Val {
    fn from(s: TimeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TimeEvent);

impl TimeEvent {
    pub fn view(&self) -> Any {
        self.inner.get("view").as_::<Any>()
    }
}
impl TimeEvent {
    pub fn detail(&self) -> i32 {
        self.inner.get("detail").as_::<i32>()
    }
}
impl TimeEvent {
    pub fn init_time_event(
        &self,
        type_arg: DOMString,
        view_arg: Window,
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
