use super::*;

/// The URLSearchParams class.
/// [`URLSearchParams`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLSearchParams {
    inner: Any,
}

impl FromVal for URLSearchParams {
    fn from_val(v: &Any) -> Self {
        URLSearchParams {
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

impl core::ops::Deref for URLSearchParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for URLSearchParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for URLSearchParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for URLSearchParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<URLSearchParams> for Any {
    fn from(s: URLSearchParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&URLSearchParams> for Any {
    fn from(s: &URLSearchParams) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(URLSearchParams);

impl URLSearchParams {
    /// The `new URLSearchParams(..)` constructor, creating a new URLSearchParams instance
    pub fn new0() -> URLSearchParams {
        Self {
            inner: Any::global("URLSearchParams").new(&[]).as_::<Any>(),
        }
    }

    /// The `new URLSearchParams(..)` constructor, creating a new URLSearchParams instance
    pub fn new1(init: &Any) -> URLSearchParams {
        Self {
            inner: Any::global("URLSearchParams")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}
impl URLSearchParams {
    /// Getter of the `size` attribute.
    /// [`URLSearchParams.size`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/size)
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }
}
impl URLSearchParams {
    /// The append method.
    /// [`URLSearchParams.append`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/append)
    pub fn append(&self, name: &JsString, value: &JsString) -> Undefined {
        self.inner
            .call("append", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl URLSearchParams {
    /// The delete method.
    /// [`URLSearchParams.delete`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/delete)
    pub fn delete0(&self, name: &JsString) -> Undefined {
        self.inner.call("delete", &[name.into()]).as_::<Undefined>()
    }
    /// The delete method.
    /// [`URLSearchParams.delete`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/delete)
    pub fn delete1(&self, name: &JsString, value: &JsString) -> Undefined {
        self.inner
            .call("delete", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl URLSearchParams {
    /// The get method.
    /// [`URLSearchParams.get`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/get)
    pub fn get(&self, name: &JsString) -> JsString {
        self.inner.call("get", &[name.into()]).as_::<JsString>()
    }
}
impl URLSearchParams {
    /// The getAll method.
    /// [`URLSearchParams.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/getAll)
    pub fn get_all(&self, name: &JsString) -> TypedArray<JsString> {
        self.inner
            .call("getAll", &[name.into()])
            .as_::<TypedArray<JsString>>()
    }
}
impl URLSearchParams {
    /// The has method.
    /// [`URLSearchParams.has`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/has)
    pub fn has0(&self, name: &JsString) -> bool {
        self.inner.call("has", &[name.into()]).as_::<bool>()
    }
    /// The has method.
    /// [`URLSearchParams.has`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/has)
    pub fn has1(&self, name: &JsString, value: &JsString) -> bool {
        self.inner
            .call("has", &[name.into(), value.into()])
            .as_::<bool>()
    }
}
impl URLSearchParams {
    /// The set method.
    /// [`URLSearchParams.set`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/set)
    pub fn set(&self, name: &JsString, value: &JsString) -> Undefined {
        self.inner
            .call("set", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl URLSearchParams {
    /// The sort method.
    /// [`URLSearchParams.sort`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/sort)
    pub fn sort(&self) -> Undefined {
        self.inner.call("sort", &[]).as_::<Undefined>()
    }
}
