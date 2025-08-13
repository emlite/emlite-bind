use super::*;




/// The TaskPriorityChangeEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TaskPriorityChangeEventInit {
    inner: Any,
}

impl FromVal for TaskPriorityChangeEventInit {
    fn from_val(v: &Any) -> Self {
        TaskPriorityChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TaskPriorityChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TaskPriorityChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TaskPriorityChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TaskPriorityChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<TaskPriorityChangeEventInit> for Any {
    fn from(s: TaskPriorityChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TaskPriorityChangeEventInit> for Any {
    fn from(s: &TaskPriorityChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl TaskPriorityChangeEventInit {
    /// Getter of the `previousPriority` attribute.
    pub fn previous_priority(&self) -> TaskPriority {
        self.inner.get("previousPriority").as_::<TaskPriority>()
    }

    /// Setter of the `previousPriority` attribute.
    pub fn set_previous_priority(&mut self, value: &TaskPriority) {
        self.inner.set("previousPriority", value);
    }
}
