use super::*;

/// The Headers class.
/// [`Headers`](https://developer.mozilla.org/en-US/docs/Web/API/Headers)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Headers {
    inner: Any,
}
impl FromVal for Headers {
    fn from_val(v: &Any) -> Self {
        Headers {
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
impl core::ops::Deref for Headers {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Headers {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Headers {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Headers> for Any {
    fn from(s: Headers) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Headers> for Any {
    fn from(s: &Headers) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Headers);

impl Headers {
    /// The `new Headers(..)` constructor, creating a new Headers instance
    pub fn new0() -> Headers {
        Self {
            inner: Any::global("Headers").new(&[]).as_::<Any>(),
        }
    }

    /// The `new Headers(..)` constructor, creating a new Headers instance
    pub fn new1(init: &Any) -> Headers {
        Self {
            inner: Any::global("Headers").new(&[init.into()]).as_::<Any>(),
        }
    }
}
impl Headers {
    /// The append method.
    /// [`Headers.append`](https://developer.mozilla.org/en-US/docs/Web/API/Headers/append)
    pub fn append(&self, name: &str, value: &str) -> Undefined {
        self.inner
            .call("append", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl Headers {
    /// The delete method.
    /// [`Headers.delete`](https://developer.mozilla.org/en-US/docs/Web/API/Headers/delete)
    pub fn delete(&self, name: &str) -> Undefined {
        self.inner.call("delete", &[name.into()]).as_::<Undefined>()
    }
}
impl Headers {
    /// The get method.
    /// [`Headers.get`](https://developer.mozilla.org/en-US/docs/Web/API/Headers/get)
    pub fn get(&self, name: &str) -> String {
        self.inner.call("get", &[name.into()]).as_::<String>()
    }
}
impl Headers {
    /// The getSetCookie method.
    /// [`Headers.getSetCookie`](https://developer.mozilla.org/en-US/docs/Web/API/Headers/getSetCookie)
    pub fn get_set_cookie(&self) -> Sequence<String> {
        self.inner
            .call("getSetCookie", &[])
            .as_::<Sequence<String>>()
    }
}
impl Headers {
    /// The has method.
    /// [`Headers.has`](https://developer.mozilla.org/en-US/docs/Web/API/Headers/has)
    pub fn has(&self, name: &str) -> bool {
        self.inner.call("has", &[name.into()]).as_::<bool>()
    }
}
impl Headers {
    /// The set method.
    /// [`Headers.set`](https://developer.mozilla.org/en-US/docs/Web/API/Headers/set)
    pub fn set(&self, name: &str, value: &str) -> Undefined {
        self.inner
            .call("set", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
