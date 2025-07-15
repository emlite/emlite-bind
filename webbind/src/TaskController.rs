use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TaskController {
    inner: AbortController,
}
impl FromVal for TaskController {
    fn from_val(v: &emlite::Val) -> Self {
        TaskController {
            inner: AbortController::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for TaskController {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TaskController {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TaskController> for emlite::Val {
    fn from(s: TaskController) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TaskController);

impl TaskController {
    pub fn new0() -> TaskController {
        Self {
            inner: emlite::Val::global("TaskController")
                .new(&[])
                .as_::<AbortController>(),
        }
    }

    pub fn new1(init: Any) -> TaskController {
        Self {
            inner: emlite::Val::global("TaskController")
                .new(&[init.into()])
                .as_::<AbortController>(),
        }
    }
}
impl TaskController {
    pub fn set_priority(&self, priority: TaskPriority) -> Undefined {
        self.inner
            .call("setPriority", &[priority.into()])
            .as_::<Undefined>()
    }
}
