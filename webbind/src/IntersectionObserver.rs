use super::*;

#[derive(Clone, Debug)]
pub struct IntersectionObserver {
    inner: emlite::Val,
}
impl FromVal for IntersectionObserver {
    fn from_val(v: &emlite::Val) -> Self {
        IntersectionObserver {
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
impl std::ops::Deref for IntersectionObserver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IntersectionObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IntersectionObserver> for emlite::Val {
    fn from(s: IntersectionObserver) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IntersectionObserver {
    pub fn new0(callback: jsbind::Function) -> IntersectionObserver {
        Self {
            inner: emlite::Val::global("IntersectionObserver")
                .new(&[callback.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(callback: jsbind::Function, options: jsbind::Any) -> IntersectionObserver {
        Self {
            inner: emlite::Val::global("IntersectionObserver")
                .new(&[callback.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl IntersectionObserver {
    pub fn root(&self) -> jsbind::Any {
        self.inner.get("root").as_::<jsbind::Any>()
    }
}
impl IntersectionObserver {
    pub fn root_margin(&self) -> jsbind::DOMString {
        self.inner.get("rootMargin").as_::<jsbind::DOMString>()
    }
}
impl IntersectionObserver {
    pub fn scroll_margin(&self) -> jsbind::DOMString {
        self.inner.get("scrollMargin").as_::<jsbind::DOMString>()
    }
}
impl IntersectionObserver {
    pub fn thresholds(&self) -> jsbind::FrozenArray<f64> {
        self.inner
            .get("thresholds")
            .as_::<jsbind::FrozenArray<f64>>()
    }
}
impl IntersectionObserver {
    pub fn delay(&self) -> i32 {
        self.inner.get("delay").as_::<i32>()
    }
}
impl IntersectionObserver {
    pub fn track_visibility(&self) -> bool {
        self.inner.get("trackVisibility").as_::<bool>()
    }
}
impl IntersectionObserver {
    pub fn observe(&self, target: Element) -> jsbind::Undefined {
        self.inner
            .call("observe", &[target.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl IntersectionObserver {
    pub fn unobserve(&self, target: Element) -> jsbind::Undefined {
        self.inner
            .call("unobserve", &[target.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl IntersectionObserver {
    pub fn disconnect(&self) -> jsbind::Undefined {
        self.inner
            .call("disconnect", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl IntersectionObserver {
    pub fn take_records(&self) -> jsbind::Sequence<IntersectionObserverEntry> {
        self.inner
            .call("takeRecords", &[])
            .as_::<jsbind::Sequence<IntersectionObserverEntry>>()
    }
}
