use super::*;

/// The TaskSignal class.
/// [`TaskSignal`](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TaskSignal {
    inner: AbortSignal,
}

impl FromVal for TaskSignal {
    fn from_val(v: &Any) -> Self {
        TaskSignal {
            inner: AbortSignal::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TaskSignal {
    type Target = AbortSignal;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TaskSignal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TaskSignal {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TaskSignal {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TaskSignal> for Any {
    fn from(s: TaskSignal) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TaskSignal> for Any {
    fn from(s: &TaskSignal) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TaskSignal);

impl TaskSignal {
    /// Getter of the `priority` attribute.
    /// [`TaskSignal.priority`](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/priority)
    pub fn priority(&self) -> TaskPriority {
        self.inner.get("priority").as_::<TaskPriority>()
    }
}
impl TaskSignal {
    /// Getter of the `onprioritychange` attribute.
    /// [`TaskSignal.onprioritychange`](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/onprioritychange)
    pub fn onprioritychange(&self) -> Any {
        self.inner.get("onprioritychange").as_::<Any>()
    }

    /// Setter of the `onprioritychange` attribute.
    /// [`TaskSignal.onprioritychange`](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/onprioritychange)
    pub fn set_onprioritychange(&mut self, value: &Any) {
        self.inner.set("onprioritychange", value);
    }
}
impl TaskSignal {
    /// The any method.
    /// [`TaskSignal.any`](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/any)
    pub fn any0(signals: &TypedArray<AbortSignal>) -> TaskSignal {
        Any::global("TaskSignal")
            .call("any", &[signals.into()])
            .as_::<TaskSignal>()
    }
    /// The any method.
    /// [`TaskSignal.any`](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/any)
    pub fn any1(signals: &TypedArray<AbortSignal>, init: &TaskSignalAnyInit) -> TaskSignal {
        Any::global("TaskSignal")
            .call("any", &[signals.into(), init.into()])
            .as_::<TaskSignal>()
    }
}
