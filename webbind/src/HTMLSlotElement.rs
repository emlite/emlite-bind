use super::*;

/// The HTMLSlotElement class.
/// [`HTMLSlotElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLSlotElement {
    inner: HTMLElement,
}

impl FromVal for HTMLSlotElement {
    fn from_val(v: &Any) -> Self {
        HTMLSlotElement {
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

impl core::ops::Deref for HTMLSlotElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLSlotElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLSlotElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLSlotElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLSlotElement> for Any {
    fn from(s: HTMLSlotElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLSlotElement> for Any {
    fn from(s: &HTMLSlotElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLSlotElement);

impl HTMLSlotElement {
    /// The `new HTMLSlotElement(..)` constructor, creating a new HTMLSlotElement instance
    pub fn new() -> HTMLSlotElement {
        Self {
            inner: Any::global("HTMLSlotElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLSlotElement {
    /// Getter of the `name` attribute.
    /// [`HTMLSlotElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLSlotElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLSlotElement {
    /// The assignedNodes method.
    /// [`HTMLSlotElement.assignedNodes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)
    pub fn assigned_nodes0(&self) -> TypedArray<Node> {
        self.inner
            .call("assignedNodes", &[])
            .as_::<TypedArray<Node>>()
    }
    /// The assignedNodes method.
    /// [`HTMLSlotElement.assignedNodes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)
    pub fn assigned_nodes1(&self, options: &AssignedNodesOptions) -> TypedArray<Node> {
        self.inner
            .call("assignedNodes", &[options.into()])
            .as_::<TypedArray<Node>>()
    }
}
impl HTMLSlotElement {
    /// The assignedElements method.
    /// [`HTMLSlotElement.assignedElements`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedElements)
    pub fn assigned_elements0(&self) -> TypedArray<Element> {
        self.inner
            .call("assignedElements", &[])
            .as_::<TypedArray<Element>>()
    }
    /// The assignedElements method.
    /// [`HTMLSlotElement.assignedElements`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedElements)
    pub fn assigned_elements1(&self, options: &AssignedNodesOptions) -> TypedArray<Element> {
        self.inner
            .call("assignedElements", &[options.into()])
            .as_::<TypedArray<Element>>()
    }
}
impl HTMLSlotElement {
    /// The assign method.
    /// [`HTMLSlotElement.assign`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assign)
    pub fn assign(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("assign", &[nodes.into()])
            .as_::<Undefined>()
    }
}
