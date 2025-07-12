use super::*;

#[derive(Clone, Debug)]
pub struct CSSTransition {
    inner: Animation,
}
impl FromVal for CSSTransition {
    fn from_val(v: &emlite::Val) -> Self {
        CSSTransition {
            inner: Animation::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSTransition {
    type Target = Animation;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSTransition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSTransition> for emlite::Val {
    fn from(s: CSSTransition) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSTransition {
    pub fn transition_property(&self) -> jsbind::CSSOMString {
        self.inner
            .get("transitionProperty")
            .as_::<jsbind::CSSOMString>()
    }
}
