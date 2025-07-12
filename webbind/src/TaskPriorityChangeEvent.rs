use super::*;

#[derive(Clone, Debug)]
pub struct TaskPriorityChangeEvent {
    inner: Event,
}
impl FromVal for TaskPriorityChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        TaskPriorityChangeEvent {
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
impl std::ops::Deref for TaskPriorityChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TaskPriorityChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TaskPriorityChangeEvent> for emlite::Val {
    fn from(s: TaskPriorityChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TaskPriorityChangeEvent {
    pub fn new(
        type_: jsbind::DOMString,
        priority_change_event_init_dict: jsbind::Any,
    ) -> TaskPriorityChangeEvent {
        Self {
            inner: emlite::Val::global("TaskPriorityChangeEvent")
                .new(&[type_.into(), priority_change_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl TaskPriorityChangeEvent {
    pub fn previous_priority(&self) -> TaskPriority {
        self.inner.get("previousPriority").as_::<TaskPriority>()
    }
}
