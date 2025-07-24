use super::*;

/// The IntersectionObserver class.
/// [`IntersectionObserver`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IntersectionObserver {
    inner: Any,
}
impl FromVal for IntersectionObserver {
    fn from_val(v: &Any) -> Self {
        IntersectionObserver {
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
impl core::ops::Deref for IntersectionObserver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IntersectionObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IntersectionObserver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IntersectionObserver {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IntersectionObserver> for Any {
    fn from(s: IntersectionObserver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IntersectionObserver> for Any {
    fn from(s: &IntersectionObserver) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IntersectionObserver);

impl IntersectionObserver {
    /// The `new IntersectionObserver(..)` constructor, creating a new IntersectionObserver instance
    pub fn new0(callback: &Function) -> IntersectionObserver {
        Self {
            inner: Any::global("IntersectionObserver")
                .new(&[callback.into()])
                .as_::<Any>(),
        }
    }

    /// The `new IntersectionObserver(..)` constructor, creating a new IntersectionObserver instance
    pub fn new1(callback: &Function, options: &Any) -> IntersectionObserver {
        Self {
            inner: Any::global("IntersectionObserver")
                .new(&[callback.into(), options.into()])
                .as_::<Any>(),
        }
    }
}
impl IntersectionObserver {
    /// Getter of the `root` attribute.
    /// [`IntersectionObserver.root`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/root)
    pub fn root(&self) -> Any {
        self.inner.get("root").as_::<Any>()
    }
}
impl IntersectionObserver {
    /// Getter of the `rootMargin` attribute.
    /// [`IntersectionObserver.rootMargin`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/rootMargin)
    pub fn root_margin(&self) -> DOMString {
        self.inner.get("rootMargin").as_::<DOMString>()
    }
}
impl IntersectionObserver {
    /// Getter of the `scrollMargin` attribute.
    /// [`IntersectionObserver.scrollMargin`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/scrollMargin)
    pub fn scroll_margin(&self) -> DOMString {
        self.inner.get("scrollMargin").as_::<DOMString>()
    }
}
impl IntersectionObserver {
    /// Getter of the `thresholds` attribute.
    /// [`IntersectionObserver.thresholds`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/thresholds)
    pub fn thresholds(&self) -> FrozenArray<f64> {
        self.inner.get("thresholds").as_::<FrozenArray<f64>>()
    }
}
impl IntersectionObserver {
    /// Getter of the `delay` attribute.
    /// [`IntersectionObserver.delay`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/delay)
    pub fn delay(&self) -> i32 {
        self.inner.get("delay").as_::<i32>()
    }
}
impl IntersectionObserver {
    /// Getter of the `trackVisibility` attribute.
    /// [`IntersectionObserver.trackVisibility`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/trackVisibility)
    pub fn track_visibility(&self) -> bool {
        self.inner.get("trackVisibility").as_::<bool>()
    }
}
impl IntersectionObserver {
    /// The observe method.
    /// [`IntersectionObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/observe)
    pub fn observe(&self, target: &Element) -> Undefined {
        self.inner
            .call("observe", &[target.into()])
            .as_::<Undefined>()
    }
}
impl IntersectionObserver {
    /// The unobserve method.
    /// [`IntersectionObserver.unobserve`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/unobserve)
    pub fn unobserve(&self, target: &Element) -> Undefined {
        self.inner
            .call("unobserve", &[target.into()])
            .as_::<Undefined>()
    }
}
impl IntersectionObserver {
    /// The disconnect method.
    /// [`IntersectionObserver.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/disconnect)
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
impl IntersectionObserver {
    /// The takeRecords method.
    /// [`IntersectionObserver.takeRecords`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/takeRecords)
    pub fn take_records(&self) -> Sequence<IntersectionObserverEntry> {
        self.inner
            .call("takeRecords", &[])
            .as_::<Sequence<IntersectionObserverEntry>>()
    }
}
