use super::*;

/// The TaskPriorityChangeEvent class.
/// [`TaskPriorityChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TaskPriorityChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TaskPriorityChangeEvent {
    inner: Event,
}
impl FromVal for TaskPriorityChangeEvent {
    fn from_val(v: &Any) -> Self {
        TaskPriorityChangeEvent {
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
impl core::ops::Deref for TaskPriorityChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TaskPriorityChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TaskPriorityChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TaskPriorityChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TaskPriorityChangeEvent> for Any {
    fn from(s: TaskPriorityChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TaskPriorityChangeEvent> for Any {
    fn from(s: &TaskPriorityChangeEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TaskPriorityChangeEvent);

impl TaskPriorityChangeEvent {
    /// The `new TaskPriorityChangeEvent(..)` constructor, creating a new TaskPriorityChangeEvent instance
    pub fn new(
        type_: &DOMString,
        priority_change_event_init_dict: &Any,
    ) -> TaskPriorityChangeEvent {
        Self {
            inner: Any::global("TaskPriorityChangeEvent")
                .new(&[type_.into(), priority_change_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl TaskPriorityChangeEvent {
    /// Getter of the `previousPriority` attribute.
    /// [`TaskPriorityChangeEvent.previousPriority`](https://developer.mozilla.org/en-US/docs/Web/API/TaskPriorityChangeEvent/previousPriority)
    pub fn previous_priority(&self) -> TaskPriority {
        self.inner.get("previousPriority").as_::<TaskPriority>()
    }
}
