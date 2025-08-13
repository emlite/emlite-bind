use super::*;




/// The SharedStorageWorkletGlobalScope class.
/// [`SharedStorageWorkletGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageWorkletGlobalScope {
    inner: WorkletGlobalScope,
}

impl FromVal for SharedStorageWorkletGlobalScope {
    fn from_val(v: &Any) -> Self {
        SharedStorageWorkletGlobalScope { inner: WorkletGlobalScope::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SharedStorageWorkletGlobalScope {
    type Target = WorkletGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SharedStorageWorkletGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SharedStorageWorkletGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SharedStorageWorkletGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SharedStorageWorkletGlobalScope> for Any {
    fn from(s: SharedStorageWorkletGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SharedStorageWorkletGlobalScope> for Any {
    fn from(s: &SharedStorageWorkletGlobalScope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SharedStorageWorkletGlobalScope);


impl SharedStorageWorkletGlobalScope {
    /// The register method.
    /// [`SharedStorageWorkletGlobalScope.register`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope/register)
    pub fn register(&self, name: &JsString, operation_ctor: &Function) -> Undefined {
        self.inner.call("register", &[name.into(), operation_ctor.into(), ]).as_::<Undefined>()
    }
}
impl SharedStorageWorkletGlobalScope {
    /// Getter of the `sharedStorage` attribute.
    /// [`SharedStorageWorkletGlobalScope.sharedStorage`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope/sharedStorage)
    pub fn shared_storage(&self) -> SharedStorage {
        self.inner.get("sharedStorage").as_::<SharedStorage>()
    }

}
impl SharedStorageWorkletGlobalScope {
    /// Getter of the `privateAggregation` attribute.
    /// [`SharedStorageWorkletGlobalScope.privateAggregation`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope/privateAggregation)
    pub fn private_aggregation(&self) -> PrivateAggregation {
        self.inner.get("privateAggregation").as_::<PrivateAggregation>()
    }

}
impl SharedStorageWorkletGlobalScope {
    /// The interestGroups method.
    /// [`SharedStorageWorkletGlobalScope.interestGroups`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope/interestGroups)
    pub fn interest_groups(&self, ) -> Promise<TypedArray<StorageInterestGroup>> {
        self.inner.call("interestGroups", &[]).as_::<Promise<TypedArray<StorageInterestGroup>>>()
    }
}
impl SharedStorageWorkletGlobalScope {
    /// Getter of the `navigator` attribute.
    /// [`SharedStorageWorkletGlobalScope.navigator`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorkletGlobalScope/navigator)
    pub fn navigator(&self) -> SharedStorageWorkletNavigator {
        self.inner.get("navigator").as_::<SharedStorageWorkletNavigator>()
    }

}
