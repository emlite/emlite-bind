use super::*;

#[derive(Clone, Debug)]
pub struct GetRootNodeOptions {
    inner: emlite::Val,
}
impl FromVal for GetRootNodeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        GetRootNodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GetRootNodeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GetRootNodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GetRootNodeOptions> for emlite::Val {
    fn from(s: GetRootNodeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GetRootNodeOptions {
    pub fn composed(&self) -> bool {
        self.inner.get("composed").as_::<bool>()
    }

    pub fn set_composed(&mut self, value: bool) {
        self.inner.set("composed", value);
    }
}
#[derive(Clone, Debug)]
pub struct Node {
    inner: EventTarget,
}
impl FromVal for Node {
    fn from_val(v: &emlite::Val) -> Self {
        Node {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for Node {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Node> for emlite::Val {
    fn from(s: Node) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Node {
    pub fn node_type(&self) -> u16 {
        self.inner.get("nodeType").as_::<u16>()
    }
}
impl Node {
    pub fn node_name(&self) -> jsbind::DOMString {
        self.inner.get("nodeName").as_::<jsbind::DOMString>()
    }
}
impl Node {
    pub fn base_uri(&self) -> jsbind::USVString {
        self.inner.get("baseURI").as_::<jsbind::USVString>()
    }
}
impl Node {
    pub fn is_connected(&self) -> bool {
        self.inner.get("isConnected").as_::<bool>()
    }
}
impl Node {
    pub fn owner_document(&self) -> Document {
        self.inner.get("ownerDocument").as_::<Document>()
    }
}
impl Node {
    pub fn get_root_node0(&self) -> Node {
        self.inner.call("getRootNode", &[]).as_::<Node>()
    }

    pub fn get_root_node1(&self, options: GetRootNodeOptions) -> Node {
        self.inner
            .call("getRootNode", &[options.into()])
            .as_::<Node>()
    }
}
impl Node {
    pub fn parent_node(&self) -> Node {
        self.inner.get("parentNode").as_::<Node>()
    }
}
impl Node {
    pub fn parent_element(&self) -> Element {
        self.inner.get("parentElement").as_::<Element>()
    }
}
impl Node {
    pub fn has_child_nodes(&self) -> bool {
        self.inner.call("hasChildNodes", &[]).as_::<bool>()
    }
}
impl Node {
    pub fn child_nodes(&self) -> NodeList {
        self.inner.get("childNodes").as_::<NodeList>()
    }
}
impl Node {
    pub fn first_child(&self) -> Node {
        self.inner.get("firstChild").as_::<Node>()
    }
}
impl Node {
    pub fn last_child(&self) -> Node {
        self.inner.get("lastChild").as_::<Node>()
    }
}
impl Node {
    pub fn previous_sibling(&self) -> Node {
        self.inner.get("previousSibling").as_::<Node>()
    }
}
impl Node {
    pub fn next_sibling(&self) -> Node {
        self.inner.get("nextSibling").as_::<Node>()
    }
}
impl Node {
    pub fn node_value(&self) -> jsbind::DOMString {
        self.inner.get("nodeValue").as_::<jsbind::DOMString>()
    }

    pub fn set_node_value(&mut self, value: jsbind::DOMString) {
        self.inner.set("nodeValue", value);
    }
}
impl Node {
    pub fn text_content(&self) -> jsbind::DOMString {
        self.inner.get("textContent").as_::<jsbind::DOMString>()
    }

    pub fn set_text_content(&mut self, value: jsbind::DOMString) {
        self.inner.set("textContent", value);
    }
}
impl Node {
    pub fn normalize(&self) -> jsbind::Undefined {
        self.inner.call("normalize", &[]).as_::<jsbind::Undefined>()
    }
}
impl Node {
    pub fn clone_node0(&self) -> Node {
        self.inner.call("cloneNode", &[]).as_::<Node>()
    }

    pub fn clone_node1(&self, subtree: bool) -> Node {
        self.inner
            .call("cloneNode", &[subtree.into()])
            .as_::<Node>()
    }
}
impl Node {
    pub fn is_equal_node(&self, other_node: Node) -> bool {
        self.inner
            .call("isEqualNode", &[other_node.into()])
            .as_::<bool>()
    }
}
impl Node {
    pub fn is_same_node(&self, other_node: Node) -> bool {
        self.inner
            .call("isSameNode", &[other_node.into()])
            .as_::<bool>()
    }
}
impl Node {
    pub fn compare_document_position(&self, other: Node) -> u16 {
        self.inner
            .call("compareDocumentPosition", &[other.into()])
            .as_::<u16>()
    }
}
impl Node {
    pub fn contains(&self, other: Node) -> bool {
        self.inner.call("contains", &[other.into()]).as_::<bool>()
    }
}
impl Node {
    pub fn lookup_prefix(&self, namespace: jsbind::DOMString) -> jsbind::DOMString {
        self.inner
            .call("lookupPrefix", &[namespace.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl Node {
    pub fn lookup_namespace_uri(&self, prefix: jsbind::DOMString) -> jsbind::DOMString {
        self.inner
            .call("lookupNamespaceURI", &[prefix.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl Node {
    pub fn is_default_namespace(&self, namespace: jsbind::DOMString) -> bool {
        self.inner
            .call("isDefaultNamespace", &[namespace.into()])
            .as_::<bool>()
    }
}
impl Node {
    pub fn insert_before(&self, node: Node, child: Node) -> Node {
        self.inner
            .call("insertBefore", &[node.into(), child.into()])
            .as_::<Node>()
    }
}
impl Node {
    pub fn append_child(&self, node: Node) -> Node {
        self.inner.call("appendChild", &[node.into()]).as_::<Node>()
    }
}
impl Node {
    pub fn replace_child(&self, node: Node, child: Node) -> Node {
        self.inner
            .call("replaceChild", &[node.into(), child.into()])
            .as_::<Node>()
    }
}
impl Node {
    pub fn remove_child(&self, child: Node) -> Node {
        self.inner
            .call("removeChild", &[child.into()])
            .as_::<Node>()
    }
}
