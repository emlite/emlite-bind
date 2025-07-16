use super::*;

/// The TaskController class.
/// [`TaskController`](https://developer.mozilla.org/en-US/docs/Web/API/TaskController)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TaskController {
    inner: AbortController,
}
impl FromVal for TaskController {
    fn from_val(v: &Any) -> Self {
        TaskController {
            inner: AbortController::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TaskController {
    type Target = AbortController;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TaskController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TaskController {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TaskController {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TaskController> for Any {
    fn from(s: TaskController) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TaskController> for Any {
    fn from(s: &TaskController) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TaskController);

impl TaskController {
    /// The `new TaskController(..)` constructor, creating a new TaskController instance
    pub fn new0() -> TaskController {
        Self {
            inner: Any::global("TaskController")
                .new(&[])
                .as_::<AbortController>(),
        }
    }

    /// The `new TaskController(..)` constructor, creating a new TaskController instance
    pub fn new1(init: &Any) -> TaskController {
        Self {
            inner: Any::global("TaskController")
                .new(&[init.into()])
                .as_::<AbortController>(),
        }
    }
}
impl TaskController {
    /// The setPriority method.
    /// [`TaskController.setPriority`](https://developer.mozilla.org/en-US/docs/Web/API/TaskController/setPriority)
    pub fn set_priority(&self, priority: &TaskPriority) -> Undefined {
        self.inner
            .call("setPriority", &[priority.into()])
            .as_::<Undefined>()
    }
}
