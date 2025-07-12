use super::*;

#[derive(Clone, Debug)]
pub struct ResizeObserverOptions {
    inner: emlite::Val,
}
impl FromVal for ResizeObserverOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ResizeObserverOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ResizeObserverOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ResizeObserverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ResizeObserverOptions> for emlite::Val {
    fn from(s: ResizeObserverOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ResizeObserverOptions {
    pub fn box_(&self) -> ResizeObserverBoxOptions {
        self.inner.get("box").as_::<ResizeObserverBoxOptions>()
    }

    pub fn set_box_(&mut self, value: ResizeObserverBoxOptions) {
        self.inner.set("box", value);
    }
}
#[derive(Clone, Debug)]
pub struct ResizeObserver {
    inner: emlite::Val,
}
impl FromVal for ResizeObserver {
    fn from_val(v: &emlite::Val) -> Self {
        ResizeObserver {
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
impl std::ops::Deref for ResizeObserver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ResizeObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ResizeObserver> for emlite::Val {
    fn from(s: ResizeObserver) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ResizeObserver {
    pub fn new(callback: jsbind::Function) -> ResizeObserver {
        Self {
            inner: emlite::Val::global("ResizeObserver")
                .new(&[callback.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ResizeObserver {
    pub fn observe0(&self, target: Element) -> jsbind::Undefined {
        self.inner
            .call("observe", &[target.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn observe1(&self, target: Element, options: ResizeObserverOptions) -> jsbind::Undefined {
        self.inner
            .call("observe", &[target.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl ResizeObserver {
    pub fn unobserve(&self, target: Element) -> jsbind::Undefined {
        self.inner
            .call("unobserve", &[target.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl ResizeObserver {
    pub fn disconnect(&self) -> jsbind::Undefined {
        self.inner
            .call("disconnect", &[])
            .as_::<jsbind::Undefined>()
    }
}
