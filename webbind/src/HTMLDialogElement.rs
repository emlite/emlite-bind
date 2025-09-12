use super::*;

/// The HTMLDialogElement class.
/// [`HTMLDialogElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLDialogElement {
    inner: HTMLElement,
}

impl FromVal for HTMLDialogElement {
    fn from_val(v: &Any) -> Self {
        HTMLDialogElement {
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

impl core::ops::Deref for HTMLDialogElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLDialogElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLDialogElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLDialogElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLDialogElement> for Any {
    fn from(s: HTMLDialogElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLDialogElement> for Any {
    fn from(s: &HTMLDialogElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLDialogElement);

impl HTMLDialogElement {
    /// Getter of the `open` attribute.
    /// [`HTMLDialogElement.open`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)
    pub fn open(&self) -> bool {
        self.inner.get("open").as_::<bool>()
    }

    /// Setter of the `open` attribute.
    /// [`HTMLDialogElement.open`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)
    pub fn set_open(&mut self, value: bool) {
        self.inner.set("open", value);
    }
}
impl HTMLDialogElement {
    /// Getter of the `returnValue` attribute.
    /// [`HTMLDialogElement.returnValue`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)
    pub fn return_value(&self) -> JsString {
        self.inner.get("returnValue").as_::<JsString>()
    }

    /// Setter of the `returnValue` attribute.
    /// [`HTMLDialogElement.returnValue`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)
    pub fn set_return_value(&mut self, value: &JsString) {
        self.inner.set("returnValue", value);
    }
}
impl HTMLDialogElement {
    /// Getter of the `closedBy` attribute.
    /// [`HTMLDialogElement.closedBy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/closedBy)
    pub fn closed_by(&self) -> JsString {
        self.inner.get("closedBy").as_::<JsString>()
    }

    /// Setter of the `closedBy` attribute.
    /// [`HTMLDialogElement.closedBy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/closedBy)
    pub fn set_closed_by(&mut self, value: &JsString) {
        self.inner.set("closedBy", value);
    }
}

impl HTMLDialogElement {
    /// The `new HTMLDialogElement(..)` constructor, creating a new HTMLDialogElement instance
    pub fn new() -> HTMLDialogElement {
        Self {
            inner: Any::global("HTMLDialogElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLDialogElement {
    /// The show method.
    /// [`HTMLDialogElement.show`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/show)
    pub fn show(&self) -> Undefined {
        self.inner.call("show", &[]).as_::<Undefined>()
    }
}
impl HTMLDialogElement {
    /// The showModal method.
    /// [`HTMLDialogElement.showModal`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/showModal)
    pub fn show_modal(&self) -> Undefined {
        self.inner.call("showModal", &[]).as_::<Undefined>()
    }
}
impl HTMLDialogElement {
    /// The close method.
    /// [`HTMLDialogElement.close`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)
    pub fn close0(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
    /// The close method.
    /// [`HTMLDialogElement.close`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)
    pub fn close1(&self, return_value: &JsString) -> Undefined {
        self.inner
            .call("close", &[return_value.into()])
            .as_::<Undefined>()
    }
}
impl HTMLDialogElement {
    /// The requestClose method.
    /// [`HTMLDialogElement.requestClose`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/requestClose)
    pub fn request_close0(&self) -> Undefined {
        self.inner.call("requestClose", &[]).as_::<Undefined>()
    }
    /// The requestClose method.
    /// [`HTMLDialogElement.requestClose`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/requestClose)
    pub fn request_close1(&self, return_value: &JsString) -> Undefined {
        self.inner
            .call("requestClose", &[return_value.into()])
            .as_::<Undefined>()
    }
}
