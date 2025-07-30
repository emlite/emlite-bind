use super::*;

/// The FormData class.
/// [`FormData`](https://developer.mozilla.org/en-US/docs/Web/API/FormData)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FormData {
    inner: Any,
}
impl FromVal for FormData {
    fn from_val(v: &Any) -> Self {
        FormData {
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
impl core::ops::Deref for FormData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FormData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FormData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FormData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FormData> for Any {
    fn from(s: FormData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FormData> for Any {
    fn from(s: &FormData) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FormData);

impl FormData {
    /// The `new FormData(..)` constructor, creating a new FormData instance
    pub fn new0() -> FormData {
        Self {
            inner: Any::global("FormData").new(&[]).as_::<Any>(),
        }
    }

    /// The `new FormData(..)` constructor, creating a new FormData instance
    pub fn new1(form: &HTMLFormElement) -> FormData {
        Self {
            inner: Any::global("FormData").new(&[form.into()]).as_::<Any>(),
        }
    }

    /// The `new FormData(..)` constructor, creating a new FormData instance
    pub fn new2(form: &HTMLFormElement, submitter: &HTMLElement) -> FormData {
        Self {
            inner: Any::global("FormData")
                .new(&[form.into(), submitter.into()])
                .as_::<Any>(),
        }
    }
}
impl FormData {
    /// The append method.
    /// [`FormData.append`](https://developer.mozilla.org/en-US/docs/Web/API/FormData/append)
    pub fn append0(&self, name: &JsString, blob_value: &Blob) -> Undefined {
        self.inner
            .call("append", &[name.into(), blob_value.into()])
            .as_::<Undefined>()
    }
    /// The append method.
    /// [`FormData.append`](https://developer.mozilla.org/en-US/docs/Web/API/FormData/append)
    pub fn append1(&self, name: &JsString, blob_value: &Blob, filename: &JsString) -> Undefined {
        self.inner
            .call("append", &[name.into(), blob_value.into(), filename.into()])
            .as_::<Undefined>()
    }
}
impl FormData {
    /// The delete method.
    /// [`FormData.delete`](https://developer.mozilla.org/en-US/docs/Web/API/FormData/delete)
    pub fn delete(&self, name: &JsString) -> Undefined {
        self.inner.call("delete", &[name.into()]).as_::<Undefined>()
    }
}
impl FormData {
    /// The get method.
    /// [`FormData.get`](https://developer.mozilla.org/en-US/docs/Web/API/FormData/get)
    pub fn get(&self, name: &JsString) -> Any {
        self.inner.call("get", &[name.into()]).as_::<Any>()
    }
}
impl FormData {
    /// The getAll method.
    /// [`FormData.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/FormData/getAll)
    pub fn get_all(&self, name: &JsString) -> TypedArray<Any> {
        self.inner
            .call("getAll", &[name.into()])
            .as_::<TypedArray<Any>>()
    }
}
impl FormData {
    /// The has method.
    /// [`FormData.has`](https://developer.mozilla.org/en-US/docs/Web/API/FormData/has)
    pub fn has(&self, name: &JsString) -> bool {
        self.inner.call("has", &[name.into()]).as_::<bool>()
    }
}
impl FormData {
    /// The set method.
    /// [`FormData.set`](https://developer.mozilla.org/en-US/docs/Web/API/FormData/set)
    pub fn set0(&self, name: &JsString, blob_value: &Blob) -> Undefined {
        self.inner
            .call("set", &[name.into(), blob_value.into()])
            .as_::<Undefined>()
    }
    /// The set method.
    /// [`FormData.set`](https://developer.mozilla.org/en-US/docs/Web/API/FormData/set)
    pub fn set1(&self, name: &JsString, blob_value: &Blob, filename: &JsString) -> Undefined {
        self.inner
            .call("set", &[name.into(), blob_value.into(), filename.into()])
            .as_::<Undefined>()
    }
}
