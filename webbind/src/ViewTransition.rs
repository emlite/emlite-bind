use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for ViewTransition {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ViewTransition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ViewTransition> for emlite::Val {
    fn from(s: ViewTransition) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ViewTransition {
    pub fn update_callback_done(&self) -> jsbind::Promise {
        self.inner
            .get("updateCallbackDone")
            .as_::<jsbind::Promise>()
    }
}
impl ViewTransition {
    pub fn ready(&self) -> jsbind::Promise {
        self.inner.get("ready").as_::<jsbind::Promise>()
    }
}
impl ViewTransition {
    pub fn finished(&self) -> jsbind::Promise {
        self.inner.get("finished").as_::<jsbind::Promise>()
    }
}
impl ViewTransition {
    pub fn skip_transition(&self) -> jsbind::Undefined {
        self.inner
            .call("skipTransition", &[])
            .as_::<jsbind::Undefined>()
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
