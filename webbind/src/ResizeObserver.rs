use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ResizeObserverOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ResizeObserverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ResizeObserverOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ResizeObserverOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ResizeObserverOptions> for emlite::Val {
    fn from(s: ResizeObserverOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ResizeObserverOptions> for emlite::Val {
    fn from(s: &ResizeObserverOptions) -> emlite::Val {
        s.inner.clone()
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ResizeObserver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ResizeObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ResizeObserver {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ResizeObserver {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ResizeObserver> for emlite::Val {
    fn from(s: ResizeObserver) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ResizeObserver> for emlite::Val {
    fn from(s: &ResizeObserver) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ResizeObserver);

impl ResizeObserver {
    pub fn new(callback: Function) -> ResizeObserver {
        Self {
            inner: emlite::Val::global("ResizeObserver")
                .new(&[callback.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ResizeObserver {
    pub fn observe0(&self, target: Element) -> Undefined {
        self.inner
            .call("observe", &[target.into()])
            .as_::<Undefined>()
    }

    pub fn observe1(&self, target: Element, options: ResizeObserverOptions) -> Undefined {
        self.inner
            .call("observe", &[target.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl ResizeObserver {
    pub fn unobserve(&self, target: Element) -> Undefined {
        self.inner
            .call("unobserve", &[target.into()])
            .as_::<Undefined>()
    }
}
impl ResizeObserver {
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
