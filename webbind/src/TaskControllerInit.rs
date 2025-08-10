use super::*;

/// The TaskControllerInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TaskControllerInit {
    inner: Any,
}

impl FromVal for TaskControllerInit {
    fn from_val(v: &Any) -> Self {
        TaskControllerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TaskControllerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TaskControllerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TaskControllerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TaskControllerInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TaskControllerInit> for Any {
    fn from(s: TaskControllerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TaskControllerInit> for Any {
    fn from(s: &TaskControllerInit) -> Any {
        s.inner.clone()
    }
}

impl TaskControllerInit {
    /// Getter of the `priority` attribute.
    pub fn priority(&self) -> TaskPriority {
        self.inner.get("priority").as_::<TaskPriority>()
    }

    /// Setter of the `priority` attribute.
    pub fn set_priority(&mut self, value: &TaskPriority) {
        self.inner.set("priority", value);
    }
}
