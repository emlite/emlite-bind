use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AssignedNodesOptions {
    inner: Any,
}
impl FromVal for AssignedNodesOptions {
    fn from_val(v: &Any) -> Self {
        AssignedNodesOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AssignedNodesOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AssignedNodesOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AssignedNodesOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AssignedNodesOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AssignedNodesOptions> for Any {
    fn from(s: AssignedNodesOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AssignedNodesOptions> for Any {
    fn from(s: &AssignedNodesOptions) -> Any {
        s.inner.clone()
    }
}

impl AssignedNodesOptions {
    pub fn flatten(&self) -> bool {
        self.inner.get("flatten").as_::<bool>()
    }

    pub fn set_flatten(&mut self, value: bool) {
        self.inner.set("flatten", value);
    }
}
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
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLSlotElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)
    pub fn set_name(&mut self, value: &DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLSlotElement {
    /// The assignedNodes method.
    /// [`HTMLSlotElement.assignedNodes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)
    pub fn assigned_nodes0(&self) -> Sequence<Node> {
        self.inner
            .call("assignedNodes", &[])
            .as_::<Sequence<Node>>()
    }
    /// The assignedNodes method.
    /// [`HTMLSlotElement.assignedNodes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)
    pub fn assigned_nodes1(&self, options: &AssignedNodesOptions) -> Sequence<Node> {
        self.inner
            .call("assignedNodes", &[options.into()])
            .as_::<Sequence<Node>>()
    }
}
impl HTMLSlotElement {
    /// The assignedElements method.
    /// [`HTMLSlotElement.assignedElements`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedElements)
    pub fn assigned_elements0(&self) -> Sequence<Element> {
        self.inner
            .call("assignedElements", &[])
            .as_::<Sequence<Element>>()
    }
    /// The assignedElements method.
    /// [`HTMLSlotElement.assignedElements`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedElements)
    pub fn assigned_elements1(&self, options: &AssignedNodesOptions) -> Sequence<Element> {
        self.inner
            .call("assignedElements", &[options.into()])
            .as_::<Sequence<Element>>()
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
