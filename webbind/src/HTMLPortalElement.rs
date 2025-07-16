use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PortalActivateOptions {
    inner: Any,
}
impl FromVal for PortalActivateOptions {
    fn from_val(v: &Any) -> Self {
        PortalActivateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PortalActivateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PortalActivateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PortalActivateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PortalActivateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PortalActivateOptions> for Any {
    fn from(s: PortalActivateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PortalActivateOptions> for Any {
    fn from(s: &PortalActivateOptions) -> Any {
        s.inner.clone()
    }
}

impl PortalActivateOptions {
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
/// The HTMLPortalElement class.
/// [`HTMLPortalElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLPortalElement {
    inner: HTMLElement,
}
impl FromVal for HTMLPortalElement {
    fn from_val(v: &Any) -> Self {
        HTMLPortalElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLPortalElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLPortalElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLPortalElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLPortalElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLPortalElement> for Any {
    fn from(s: HTMLPortalElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLPortalElement> for Any {
    fn from(s: &HTMLPortalElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLPortalElement);

impl HTMLPortalElement {
    /// The `new HTMLPortalElement(..)` constructor, creating a new HTMLPortalElement instance
    pub fn new() -> HTMLPortalElement {
        Self {
            inner: Any::global("HTMLPortalElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLPortalElement {
    /// Getter of the `src` attribute.
    /// [`HTMLPortalElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/src)
    pub fn src(&self) -> String {
        self.inner.get("src").as_::<String>()
    }

    /// Setter of the `src` attribute.
    /// [`HTMLPortalElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/src)
    pub fn set_src(&mut self, value: &str) {
        self.inner.set("src", value);
    }
}
impl HTMLPortalElement {
    /// Getter of the `referrerPolicy` attribute.
    /// [`HTMLPortalElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/referrerPolicy)
    pub fn referrer_policy(&self) -> String {
        self.inner.get("referrerPolicy").as_::<String>()
    }

    /// Setter of the `referrerPolicy` attribute.
    /// [`HTMLPortalElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/referrerPolicy)
    pub fn set_referrer_policy(&mut self, value: &str) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLPortalElement {
    /// The activate method.
    /// [`HTMLPortalElement.activate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/activate)
    pub fn activate0(&self) -> Promise {
        self.inner.call("activate", &[]).as_::<Promise>()
    }
    /// The activate method.
    /// [`HTMLPortalElement.activate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/activate)
    pub fn activate1(&self, options: &PortalActivateOptions) -> Promise {
        self.inner
            .call("activate", &[options.into()])
            .as_::<Promise>()
    }
}
impl HTMLPortalElement {
    /// The postMessage method.
    /// [`HTMLPortalElement.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/postMessage)
    pub fn post_message0(&self, message: &Any) -> Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<Undefined>()
    }
    /// The postMessage method.
    /// [`HTMLPortalElement.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/postMessage)
    pub fn post_message1(&self, message: &Any, options: &StructuredSerializeOptions) -> Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl HTMLPortalElement {
    /// Getter of the `onmessage` attribute.
    /// [`HTMLPortalElement.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`HTMLPortalElement.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl HTMLPortalElement {
    /// Getter of the `onmessageerror` attribute.
    /// [`HTMLPortalElement.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/onmessageerror)
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    /// Setter of the `onmessageerror` attribute.
    /// [`HTMLPortalElement.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPortalElement/onmessageerror)
    pub fn set_onmessageerror(&mut self, value: &Any) {
        self.inner.set("onmessageerror", value);
    }
}
