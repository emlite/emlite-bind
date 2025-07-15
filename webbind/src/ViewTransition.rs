use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ViewTransition {
    inner: emlite::Val,
}
impl FromVal for ViewTransition {
    fn from_val(v: &emlite::Val) -> Self {
        ViewTransition {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ViewTransition {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ViewTransition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ViewTransition {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ViewTransition {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ViewTransition> for emlite::Val {
    fn from(s: ViewTransition) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ViewTransition> for emlite::Val {
    fn from(s: &ViewTransition) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ViewTransition);

impl ViewTransition {
    pub fn update_callback_done(&self) -> Promise {
        self.inner.get("updateCallbackDone").as_::<Promise>()
    }
}
impl ViewTransition {
    pub fn ready(&self) -> Promise {
        self.inner.get("ready").as_::<Promise>()
    }
}
impl ViewTransition {
    pub fn finished(&self) -> Promise {
        self.inner.get("finished").as_::<Promise>()
    }
}
impl ViewTransition {
    pub fn skip_transition(&self) -> Undefined {
        self.inner.call("skipTransition", &[]).as_::<Undefined>()
    }
}
impl ViewTransition {
    pub fn types(&self) -> ViewTransitionTypeSet {
        self.inner.get("types").as_::<ViewTransitionTypeSet>()
    }

    pub fn set_types(&mut self, value: ViewTransitionTypeSet) {
        self.inner.set("types", value);
    }
}
