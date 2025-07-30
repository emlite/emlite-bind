use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetRootNodeOptions {
    inner: Any,
}
impl FromVal for GetRootNodeOptions {
    fn from_val(v: &Any) -> Self {
        GetRootNodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GetRootNodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GetRootNodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GetRootNodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GetRootNodeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GetRootNodeOptions> for Any {
    fn from(s: GetRootNodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GetRootNodeOptions> for Any {
    fn from(s: &GetRootNodeOptions) -> Any {
        s.inner.clone()
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
/// The Node class.
/// [`Node`](https://developer.mozilla.org/en-US/docs/Web/API/Node)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Node {
    inner: EventTarget,
}
impl FromVal for Node {
    fn from_val(v: &Any) -> Self {
        Node {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Node {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Node {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Node {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Node> for Any {
    fn from(s: Node) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Node> for Any {
    fn from(s: &Node) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Node);

impl Node {
    /// Getter of the `nodeType` attribute.
    /// [`Node.nodeType`](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)
    pub fn node_type(&self) -> u16 {
        self.inner.get("nodeType").as_::<u16>()
    }
}
impl Node {
    /// Getter of the `nodeName` attribute.
    /// [`Node.nodeName`](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeName)
    pub fn node_name(&self) -> JsString {
        self.inner.get("nodeName").as_::<JsString>()
    }
}
impl Node {
    /// Getter of the `baseURI` attribute.
    /// [`Node.baseURI`](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI)
    pub fn base_uri(&self) -> JsString {
        self.inner.get("baseURI").as_::<JsString>()
    }
}
impl Node {
    /// Getter of the `isConnected` attribute.
    /// [`Node.isConnected`](https://developer.mozilla.org/en-US/docs/Web/API/Node/isConnected)
    pub fn is_connected(&self) -> bool {
        self.inner.get("isConnected").as_::<bool>()
    }
}
impl Node {
    /// Getter of the `ownerDocument` attribute.
    /// [`Node.ownerDocument`](https://developer.mozilla.org/en-US/docs/Web/API/Node/ownerDocument)
    pub fn owner_document(&self) -> Document {
        self.inner.get("ownerDocument").as_::<Document>()
    }
}
impl Node {
    /// The getRootNode method.
    /// [`Node.getRootNode`](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)
    pub fn get_root_node0(&self) -> Node {
        self.inner.call("getRootNode", &[]).as_::<Node>()
    }
    /// The getRootNode method.
    /// [`Node.getRootNode`](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)
    pub fn get_root_node1(&self, options: &GetRootNodeOptions) -> Node {
        self.inner
            .call("getRootNode", &[options.into()])
            .as_::<Node>()
    }
}
impl Node {
    /// Getter of the `parentNode` attribute.
    /// [`Node.parentNode`](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentNode)
    pub fn parent_node(&self) -> Node {
        self.inner.get("parentNode").as_::<Node>()
    }
}
impl Node {
    /// Getter of the `parentElement` attribute.
    /// [`Node.parentElement`](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentElement)
    pub fn parent_element(&self) -> Element {
        self.inner.get("parentElement").as_::<Element>()
    }
}
impl Node {
    /// The hasChildNodes method.
    /// [`Node.hasChildNodes`](https://developer.mozilla.org/en-US/docs/Web/API/Node/hasChildNodes)
    pub fn has_child_nodes(&self) -> bool {
        self.inner.call("hasChildNodes", &[]).as_::<bool>()
    }
}
impl Node {
    /// Getter of the `childNodes` attribute.
    /// [`Node.childNodes`](https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes)
    pub fn child_nodes(&self) -> NodeList {
        self.inner.get("childNodes").as_::<NodeList>()
    }
}
impl Node {
    /// Getter of the `firstChild` attribute.
    /// [`Node.firstChild`](https://developer.mozilla.org/en-US/docs/Web/API/Node/firstChild)
    pub fn first_child(&self) -> Node {
        self.inner.get("firstChild").as_::<Node>()
    }
}
impl Node {
    /// Getter of the `lastChild` attribute.
    /// [`Node.lastChild`](https://developer.mozilla.org/en-US/docs/Web/API/Node/lastChild)
    pub fn last_child(&self) -> Node {
        self.inner.get("lastChild").as_::<Node>()
    }
}
impl Node {
    /// Getter of the `previousSibling` attribute.
    /// [`Node.previousSibling`](https://developer.mozilla.org/en-US/docs/Web/API/Node/previousSibling)
    pub fn previous_sibling(&self) -> Node {
        self.inner.get("previousSibling").as_::<Node>()
    }
}
impl Node {
    /// Getter of the `nextSibling` attribute.
    /// [`Node.nextSibling`](https://developer.mozilla.org/en-US/docs/Web/API/Node/nextSibling)
    pub fn next_sibling(&self) -> Node {
        self.inner.get("nextSibling").as_::<Node>()
    }
}
impl Node {
    /// Getter of the `nodeValue` attribute.
    /// [`Node.nodeValue`](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)
    pub fn node_value(&self) -> JsString {
        self.inner.get("nodeValue").as_::<JsString>()
    }

    /// Setter of the `nodeValue` attribute.
    /// [`Node.nodeValue`](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)
    pub fn set_node_value(&mut self, value: &JsString) {
        self.inner.set("nodeValue", value);
    }
}
impl Node {
    /// Getter of the `textContent` attribute.
    /// [`Node.textContent`](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)
    pub fn text_content(&self) -> JsString {
        self.inner.get("textContent").as_::<JsString>()
    }

    /// Setter of the `textContent` attribute.
    /// [`Node.textContent`](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)
    pub fn set_text_content(&mut self, value: &JsString) {
        self.inner.set("textContent", value);
    }
}
impl Node {
    /// The normalize method.
    /// [`Node.normalize`](https://developer.mozilla.org/en-US/docs/Web/API/Node/normalize)
    pub fn normalize(&self) -> Undefined {
        self.inner.call("normalize", &[]).as_::<Undefined>()
    }
}
impl Node {
    /// The cloneNode method.
    /// [`Node.cloneNode`](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)
    pub fn clone_node0(&self) -> Node {
        self.inner.call("cloneNode", &[]).as_::<Node>()
    }
    /// The cloneNode method.
    /// [`Node.cloneNode`](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)
    pub fn clone_node1(&self, subtree: bool) -> Node {
        self.inner
            .call("cloneNode", &[subtree.into()])
            .as_::<Node>()
    }
}
impl Node {
    /// The isEqualNode method.
    /// [`Node.isEqualNode`](https://developer.mozilla.org/en-US/docs/Web/API/Node/isEqualNode)
    pub fn is_equal_node(&self, other_node: &Node) -> bool {
        self.inner
            .call("isEqualNode", &[other_node.into()])
            .as_::<bool>()
    }
}
impl Node {
    /// The isSameNode method.
    /// [`Node.isSameNode`](https://developer.mozilla.org/en-US/docs/Web/API/Node/isSameNode)
    pub fn is_same_node(&self, other_node: &Node) -> bool {
        self.inner
            .call("isSameNode", &[other_node.into()])
            .as_::<bool>()
    }
}
impl Node {
    /// The compareDocumentPosition method.
    /// [`Node.compareDocumentPosition`](https://developer.mozilla.org/en-US/docs/Web/API/Node/compareDocumentPosition)
    pub fn compare_document_position(&self, other: &Node) -> u16 {
        self.inner
            .call("compareDocumentPosition", &[other.into()])
            .as_::<u16>()
    }
}
impl Node {
    /// The contains method.
    /// [`Node.contains`](https://developer.mozilla.org/en-US/docs/Web/API/Node/contains)
    pub fn contains(&self, other: &Node) -> bool {
        self.inner.call("contains", &[other.into()]).as_::<bool>()
    }
}
impl Node {
    /// The lookupPrefix method.
    /// [`Node.lookupPrefix`](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupPrefix)
    pub fn lookup_prefix(&self, namespace: &JsString) -> JsString {
        self.inner
            .call("lookupPrefix", &[namespace.into()])
            .as_::<JsString>()
    }
}
impl Node {
    /// The lookupNamespaceURI method.
    /// [`Node.lookupNamespaceURI`](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupNamespaceURI)
    pub fn lookup_namespace_uri(&self, prefix: &JsString) -> JsString {
        self.inner
            .call("lookupNamespaceURI", &[prefix.into()])
            .as_::<JsString>()
    }
}
impl Node {
    /// The isDefaultNamespace method.
    /// [`Node.isDefaultNamespace`](https://developer.mozilla.org/en-US/docs/Web/API/Node/isDefaultNamespace)
    pub fn is_default_namespace(&self, namespace: &JsString) -> bool {
        self.inner
            .call("isDefaultNamespace", &[namespace.into()])
            .as_::<bool>()
    }
}
impl Node {
    /// The insertBefore method.
    /// [`Node.insertBefore`](https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore)
    pub fn insert_before(&self, node: &Node, child: &Node) -> Node {
        self.inner
            .call("insertBefore", &[node.into(), child.into()])
            .as_::<Node>()
    }
}
impl Node {
    /// The appendChild method.
    /// [`Node.appendChild`](https://developer.mozilla.org/en-US/docs/Web/API/Node/appendChild)
    pub fn append_child(&self, node: &Node) -> Node {
        self.inner.call("appendChild", &[node.into()]).as_::<Node>()
    }
}
impl Node {
    /// The replaceChild method.
    /// [`Node.replaceChild`](https://developer.mozilla.org/en-US/docs/Web/API/Node/replaceChild)
    pub fn replace_child(&self, node: &Node, child: &Node) -> Node {
        self.inner
            .call("replaceChild", &[node.into(), child.into()])
            .as_::<Node>()
    }
}
impl Node {
    /// The removeChild method.
    /// [`Node.removeChild`](https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild)
    pub fn remove_child(&self, child: &Node) -> Node {
        self.inner
            .call("removeChild", &[child.into()])
            .as_::<Node>()
    }
}
