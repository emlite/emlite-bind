use super::*;

/// The DOMTokenList class.
/// [`DOMTokenList`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMTokenList {
    inner: Any,
}

impl FromVal for DOMTokenList {
    fn from_val(v: &Any) -> Self {
        DOMTokenList {
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

impl core::ops::Deref for DOMTokenList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMTokenList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMTokenList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMTokenList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DOMTokenList> for Any {
    fn from(s: DOMTokenList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMTokenList> for Any {
    fn from(s: &DOMTokenList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DOMTokenList);

impl DOMTokenList {
    /// Getter of the `length` attribute.
    /// [`DOMTokenList.length`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl DOMTokenList {
    /// Getter of the `value` attribute.
    /// [`DOMTokenList.value`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/value)
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    /// [`DOMTokenList.value`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/value)
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
impl DOMTokenList {
    /// The item method.
    /// [`DOMTokenList.item`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/item)
    pub fn item(&self, index: u32) -> JsString {
        self.inner.call("item", &[index.into()]).as_::<JsString>()
    }
}
impl DOMTokenList {
    /// The contains method.
    /// [`DOMTokenList.contains`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/contains)
    pub fn contains(&self, token: &JsString) -> bool {
        self.inner.call("contains", &[token.into()]).as_::<bool>()
    }
}
impl DOMTokenList {
    /// The add method.
    /// [`DOMTokenList.add`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)
    pub fn add(&self, tokens: &JsString) -> Undefined {
        self.inner.call("add", &[tokens.into()]).as_::<Undefined>()
    }
}
impl DOMTokenList {
    /// The remove method.
    /// [`DOMTokenList.remove`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)
    pub fn remove(&self, tokens: &JsString) -> Undefined {
        self.inner
            .call("remove", &[tokens.into()])
            .as_::<Undefined>()
    }
}
impl DOMTokenList {
    /// The toggle method.
    /// [`DOMTokenList.toggle`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle)
    pub fn toggle0(&self, token: &JsString) -> bool {
        self.inner.call("toggle", &[token.into()]).as_::<bool>()
    }
    /// The toggle method.
    /// [`DOMTokenList.toggle`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle)
    pub fn toggle1(&self, token: &JsString, force: bool) -> bool {
        self.inner
            .call("toggle", &[token.into(), force.into()])
            .as_::<bool>()
    }
}
impl DOMTokenList {
    /// The replace method.
    /// [`DOMTokenList.replace`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/replace)
    pub fn replace(&self, token: &JsString, new_token: &JsString) -> bool {
        self.inner
            .call("replace", &[token.into(), new_token.into()])
            .as_::<bool>()
    }
}
impl DOMTokenList {
    /// The supports method.
    /// [`DOMTokenList.supports`](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/supports)
    pub fn supports(&self, token: &JsString) -> bool {
        self.inner.call("supports", &[token.into()]).as_::<bool>()
    }
}
