use super::*;

/// The MLGraph class.
/// [`MLGraph`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraph)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLGraph {
    inner: Any,
}

impl FromVal for MLGraph {
    fn from_val(v: &Any) -> Self {
        MLGraph {
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

impl core::ops::Deref for MLGraph {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLGraph {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLGraph {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLGraph> for Any {
    fn from(s: MLGraph) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLGraph> for Any {
    fn from(s: &MLGraph) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MLGraph);

impl MLGraph {
    /// The destroy method.
    /// [`MLGraph.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraph/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
