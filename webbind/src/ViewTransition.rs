use super::*;

/// The ViewTransition class.
/// [`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ViewTransition {
    inner: Any,
}
impl FromVal for ViewTransition {
    fn from_val(v: &Any) -> Self {
        ViewTransition {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ViewTransition {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ViewTransition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ViewTransition {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ViewTransition {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ViewTransition> for Any {
    fn from(s: ViewTransition) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ViewTransition> for Any {
    fn from(s: &ViewTransition) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ViewTransition);

impl ViewTransition {
    /// Getter of the `updateCallbackDone` attribute.
    /// [`ViewTransition.updateCallbackDone`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone)
    pub fn update_callback_done(&self) -> Promise {
        self.inner.get("updateCallbackDone").as_::<Promise>()
    }
}
impl ViewTransition {
    /// Getter of the `ready` attribute.
    /// [`ViewTransition.ready`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready)
    pub fn ready(&self) -> Promise {
        self.inner.get("ready").as_::<Promise>()
    }
}
impl ViewTransition {
    /// Getter of the `finished` attribute.
    /// [`ViewTransition.finished`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished)
    pub fn finished(&self) -> Promise {
        self.inner.get("finished").as_::<Promise>()
    }
}
impl ViewTransition {
    /// The skipTransition method.
    /// [`ViewTransition.skipTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition)
    pub fn skip_transition(&self) -> Undefined {
        self.inner.call("skipTransition", &[]).as_::<Undefined>()
    }
}
impl ViewTransition {
    /// Getter of the `types` attribute.
    /// [`ViewTransition.types`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types)
    pub fn types(&self) -> ViewTransitionTypeSet {
        self.inner.get("types").as_::<ViewTransitionTypeSet>()
    }

    /// Setter of the `types` attribute.
    /// [`ViewTransition.types`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types)
    pub fn set_types(&mut self, value: &ViewTransitionTypeSet) {
        self.inner.set("types", value);
    }
}
