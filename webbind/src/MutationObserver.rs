use super::*;




/// The MutationObserver class.
/// [`MutationObserver`](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MutationObserver {
    inner: Any,
}

impl FromVal for MutationObserver {
    fn from_val(v: &Any) -> Self {
        MutationObserver { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MutationObserver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MutationObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MutationObserver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MutationObserver {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MutationObserver> for Any {
    fn from(s: MutationObserver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MutationObserver> for Any {
    fn from(s: &MutationObserver) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MutationObserver);



impl MutationObserver {
    /// The `new MutationObserver(..)` constructor, creating a new MutationObserver instance
    pub fn new(callback: &Function) -> MutationObserver {
        Self {
            inner: Any::global("MutationObserver").new(&[callback.into()]).as_::<Any>(),
        }
    }

}
impl MutationObserver {
    /// The observe method.
    /// [`MutationObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)
    pub fn observe0(&self, target: &Node) -> Undefined {
        self.inner.call("observe", &[target.into(), ]).as_::<Undefined>()
    }
    /// The observe method.
    /// [`MutationObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)
    pub fn observe1(&self, target: &Node, options: &MutationObserverInit) -> Undefined {
        self.inner.call("observe", &[target.into(), options.into(), ]).as_::<Undefined>()
    }
}
impl MutationObserver {
    /// The disconnect method.
    /// [`MutationObserver.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/disconnect)
    pub fn disconnect(&self, ) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
impl MutationObserver {
    /// The takeRecords method.
    /// [`MutationObserver.takeRecords`](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/takeRecords)
    pub fn take_records(&self, ) -> TypedArray<MutationRecord> {
        self.inner.call("takeRecords", &[]).as_::<TypedArray<MutationRecord>>()
    }
}
