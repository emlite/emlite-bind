use super::*;

/// The DocumentFragment class.
/// [`DocumentFragment`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentFragment {
    inner: Node,
}

impl FromVal for DocumentFragment {
    fn from_val(v: &Any) -> Self {
        DocumentFragment {
            inner: Node::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DocumentFragment {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DocumentFragment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DocumentFragment {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DocumentFragment {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DocumentFragment> for Any {
    fn from(s: DocumentFragment) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DocumentFragment> for Any {
    fn from(s: &DocumentFragment) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DocumentFragment);

impl DocumentFragment {
    /// Getter of the `children` attribute.
    /// [`DocumentFragment.children`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/children)
    pub fn children(&self) -> HTMLCollection {
        self.inner.get("children").as_::<HTMLCollection>()
    }
}
impl DocumentFragment {
    /// Getter of the `firstElementChild` attribute.
    /// [`DocumentFragment.firstElementChild`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/firstElementChild)
    pub fn first_element_child(&self) -> Element {
        self.inner.get("firstElementChild").as_::<Element>()
    }
}
impl DocumentFragment {
    /// Getter of the `lastElementChild` attribute.
    /// [`DocumentFragment.lastElementChild`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/lastElementChild)
    pub fn last_element_child(&self) -> Element {
        self.inner.get("lastElementChild").as_::<Element>()
    }
}
impl DocumentFragment {
    /// Getter of the `childElementCount` attribute.
    /// [`DocumentFragment.childElementCount`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/childElementCount)
    pub fn child_element_count(&self) -> u32 {
        self.inner.get("childElementCount").as_::<u32>()
    }
}

impl DocumentFragment {
    /// The `new DocumentFragment(..)` constructor, creating a new DocumentFragment instance
    pub fn new() -> DocumentFragment {
        Self {
            inner: Any::global("DocumentFragment").new(&[]).as_::<Node>(),
        }
    }
}

impl DocumentFragment {
    /// The getElementById method.
    /// [`DocumentFragment.getElementById`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/getElementById)
    pub fn get_element_by_id(&self, element_id: &JsString) -> Element {
        self.inner
            .call("getElementById", &[element_id.into()])
            .as_::<Element>()
    }
}
impl DocumentFragment {
    /// The prepend method.
    /// [`DocumentFragment.prepend`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)
    pub fn prepend(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("prepend", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl DocumentFragment {
    /// The append method.
    /// [`DocumentFragment.append`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)
    pub fn append(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("append", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl DocumentFragment {
    /// The replaceChildren method.
    /// [`DocumentFragment.replaceChildren`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)
    pub fn replace_children(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("replaceChildren", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl DocumentFragment {
    /// The moveBefore method.
    /// [`DocumentFragment.moveBefore`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/moveBefore)
    pub fn move_before(&self, node: &Node, child: &Node) -> Undefined {
        self.inner
            .call("moveBefore", &[node.into(), child.into()])
            .as_::<Undefined>()
    }
}
impl DocumentFragment {
    /// The querySelector method.
    /// [`DocumentFragment.querySelector`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelector)
    pub fn query_selector(&self, selectors: &JsString) -> Element {
        self.inner
            .call("querySelector", &[selectors.into()])
            .as_::<Element>()
    }
}
impl DocumentFragment {
    /// The querySelectorAll method.
    /// [`DocumentFragment.querySelectorAll`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelectorAll)
    pub fn query_selector_all(&self, selectors: &JsString) -> NodeList {
        self.inner
            .call("querySelectorAll", &[selectors.into()])
            .as_::<NodeList>()
    }
}
