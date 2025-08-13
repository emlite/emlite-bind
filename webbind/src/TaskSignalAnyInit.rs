use super::*;




/// The TaskSignalAnyInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TaskSignalAnyInit {
    inner: Any,
}

impl FromVal for TaskSignalAnyInit {
    fn from_val(v: &Any) -> Self {
        TaskSignalAnyInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TaskSignalAnyInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TaskSignalAnyInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TaskSignalAnyInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TaskSignalAnyInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<TaskSignalAnyInit> for Any {
    fn from(s: TaskSignalAnyInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TaskSignalAnyInit> for Any {
    fn from(s: &TaskSignalAnyInit) -> Any {
        s.inner.clone()
    }
}

impl TaskSignalAnyInit {
    /// Getter of the `priority` attribute.
    pub fn priority(&self) -> Any {
        self.inner.get("priority").as_::<Any>()
    }

    /// Setter of the `priority` attribute.
    pub fn set_priority(&mut self, value: &Any) {
        self.inner.set("priority", value);
    }
}
