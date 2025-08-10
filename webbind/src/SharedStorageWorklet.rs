use super::*;

/// The SharedStorageWorklet class.
/// [`SharedStorageWorklet`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorklet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageWorklet {
    inner: Worklet,
}

impl FromVal for SharedStorageWorklet {
    fn from_val(v: &Any) -> Self {
        SharedStorageWorklet {
            inner: Worklet::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SharedStorageWorklet {
    type Target = Worklet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SharedStorageWorklet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SharedStorageWorklet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SharedStorageWorklet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SharedStorageWorklet> for Any {
    fn from(s: SharedStorageWorklet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SharedStorageWorklet> for Any {
    fn from(s: &SharedStorageWorklet) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SharedStorageWorklet);

impl SharedStorageWorklet {
    /// The selectURL method.
    /// [`SharedStorageWorklet.selectURL`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorklet/selectURL)
    pub fn select_url0(
        &self,
        name: &JsString,
        urls: &TypedArray<SharedStorageUrlWithMetadata>,
    ) -> Promise<Any> {
        self.inner
            .call("selectURL", &[name.into(), urls.into()])
            .as_::<Promise<Any>>()
    }
    /// The selectURL method.
    /// [`SharedStorageWorklet.selectURL`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorklet/selectURL)
    pub fn select_url1(
        &self,
        name: &JsString,
        urls: &TypedArray<SharedStorageUrlWithMetadata>,
        options: &SharedStorageRunOperationMethodOptions,
    ) -> Promise<Any> {
        self.inner
            .call("selectURL", &[name.into(), urls.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl SharedStorageWorklet {
    /// The run method.
    /// [`SharedStorageWorklet.run`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorklet/run)
    pub fn run0(&self, name: &JsString) -> Promise<Any> {
        self.inner.call("run", &[name.into()]).as_::<Promise<Any>>()
    }
    /// The run method.
    /// [`SharedStorageWorklet.run`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorklet/run)
    pub fn run1(
        &self,
        name: &JsString,
        options: &SharedStorageRunOperationMethodOptions,
    ) -> Promise<Any> {
        self.inner
            .call("run", &[name.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
