use super::*;

/// The External class.
/// [`External`](https://developer.mozilla.org/en-US/docs/Web/API/External)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct External {
    inner: Any,
}
impl FromVal for External {
    fn from_val(v: &Any) -> Self {
        External {
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
impl core::ops::Deref for External {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for External {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for External {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for External {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<External> for Any {
    fn from(s: External) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&External> for Any {
    fn from(s: &External) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(External);

impl External {
    /// The AddSearchProvider method.
    /// [`External.AddSearchProvider`](https://developer.mozilla.org/en-US/docs/Web/API/External/AddSearchProvider)
    pub fn add_search_provider(&self) -> Undefined {
        self.inner.call("AddSearchProvider", &[]).as_::<Undefined>()
    }
}
impl External {
    /// The IsSearchProviderInstalled method.
    /// [`External.IsSearchProviderInstalled`](https://developer.mozilla.org/en-US/docs/Web/API/External/IsSearchProviderInstalled)
    pub fn is_search_provider_installed(&self) -> Undefined {
        self.inner
            .call("IsSearchProviderInstalled", &[])
            .as_::<Undefined>()
    }
}
