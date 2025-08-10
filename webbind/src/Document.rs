use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaretPositionFromPointOptions {
    inner: Any,
}
impl FromVal for CaretPositionFromPointOptions {
    fn from_val(v: &Any) -> Self {
        CaretPositionFromPointOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CaretPositionFromPointOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CaretPositionFromPointOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CaretPositionFromPointOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CaretPositionFromPointOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CaretPositionFromPointOptions> for Any {
    fn from(s: CaretPositionFromPointOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CaretPositionFromPointOptions> for Any {
    fn from(s: &CaretPositionFromPointOptions) -> Any {
        s.inner.clone()
    }
}

impl CaretPositionFromPointOptions {
    pub fn shadow_roots(&self) -> TypedArray<ShadowRoot> {
        self.inner
            .get("shadowRoots")
            .as_::<TypedArray<ShadowRoot>>()
    }

    pub fn set_shadow_roots(&mut self, value: &TypedArray<ShadowRoot>) {
        self.inner.set("shadowRoots", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BoxQuadOptions {
    inner: Any,
}
impl FromVal for BoxQuadOptions {
    fn from_val(v: &Any) -> Self {
        BoxQuadOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BoxQuadOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BoxQuadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BoxQuadOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BoxQuadOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BoxQuadOptions> for Any {
    fn from(s: BoxQuadOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BoxQuadOptions> for Any {
    fn from(s: &BoxQuadOptions) -> Any {
        s.inner.clone()
    }
}

impl BoxQuadOptions {
    pub fn box_(&self) -> CSSBoxType {
        self.inner.get("box").as_::<CSSBoxType>()
    }

    pub fn set_box_(&mut self, value: &CSSBoxType) {
        self.inner.set("box", value);
    }
}
impl BoxQuadOptions {
    pub fn relative_to(&self) -> Any {
        self.inner.get("relativeTo").as_::<Any>()
    }

    pub fn set_relative_to(&mut self, value: &Any) {
        self.inner.set("relativeTo", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMQuadInit {
    inner: Any,
}
impl FromVal for DOMQuadInit {
    fn from_val(v: &Any) -> Self {
        DOMQuadInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DOMQuadInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMQuadInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DOMQuadInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DOMQuadInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DOMQuadInit> for Any {
    fn from(s: DOMQuadInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DOMQuadInit> for Any {
    fn from(s: &DOMQuadInit) -> Any {
        s.inner.clone()
    }
}

impl DOMQuadInit {
    pub fn p1(&self) -> DOMPointInit {
        self.inner.get("p1").as_::<DOMPointInit>()
    }

    pub fn set_p1(&mut self, value: &DOMPointInit) {
        self.inner.set("p1", value);
    }
}
impl DOMQuadInit {
    pub fn p2(&self) -> DOMPointInit {
        self.inner.get("p2").as_::<DOMPointInit>()
    }

    pub fn set_p2(&mut self, value: &DOMPointInit) {
        self.inner.set("p2", value);
    }
}
impl DOMQuadInit {
    pub fn p3(&self) -> DOMPointInit {
        self.inner.get("p3").as_::<DOMPointInit>()
    }

    pub fn set_p3(&mut self, value: &DOMPointInit) {
        self.inner.set("p3", value);
    }
}
impl DOMQuadInit {
    pub fn p4(&self) -> DOMPointInit {
        self.inner.get("p4").as_::<DOMPointInit>()
    }

    pub fn set_p4(&mut self, value: &DOMPointInit) {
        self.inner.set("p4", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConvertCoordinateOptions {
    inner: Any,
}
impl FromVal for ConvertCoordinateOptions {
    fn from_val(v: &Any) -> Self {
        ConvertCoordinateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ConvertCoordinateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ConvertCoordinateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ConvertCoordinateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ConvertCoordinateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ConvertCoordinateOptions> for Any {
    fn from(s: ConvertCoordinateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ConvertCoordinateOptions> for Any {
    fn from(s: &ConvertCoordinateOptions) -> Any {
        s.inner.clone()
    }
}

impl ConvertCoordinateOptions {
    pub fn from_box(&self) -> CSSBoxType {
        self.inner.get("fromBox").as_::<CSSBoxType>()
    }

    pub fn set_from_box(&mut self, value: &CSSBoxType) {
        self.inner.set("fromBox", value);
    }
}
impl ConvertCoordinateOptions {
    pub fn to_box(&self) -> CSSBoxType {
        self.inner.get("toBox").as_::<CSSBoxType>()
    }

    pub fn set_to_box(&mut self, value: &CSSBoxType) {
        self.inner.set("toBox", value);
    }
}
/// The Document class.
/// [`Document`](https://developer.mozilla.org/en-US/docs/Web/API/Document)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Document {
    inner: Node,
}
impl FromVal for Document {
    fn from_val(v: &Any) -> Self {
        Document {
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
impl core::ops::Deref for Document {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Document {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Document {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Document> for Any {
    fn from(s: Document) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Document> for Any {
    fn from(s: &Document) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Document);

impl Document {
    /// The `new Document(..)` constructor, creating a new Document instance
    pub fn new() -> Document {
        Self {
            inner: Any::global("Document").new(&[]).as_::<Node>(),
        }
    }
}
impl Document {
    /// Getter of the `implementation` attribute.
    /// [`Document.implementation`](https://developer.mozilla.org/en-US/docs/Web/API/Document/implementation)
    pub fn implementation(&self) -> DOMImplementation {
        self.inner.get("implementation").as_::<DOMImplementation>()
    }
}
impl Document {
    /// Getter of the `URL` attribute.
    /// [`Document.URL`](https://developer.mozilla.org/en-US/docs/Web/API/Document/URL)
    pub fn url(&self) -> JsString {
        self.inner.get("URL").as_::<JsString>()
    }
}
impl Document {
    /// Getter of the `documentURI` attribute.
    /// [`Document.documentURI`](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentURI)
    pub fn document_uri(&self) -> JsString {
        self.inner.get("documentURI").as_::<JsString>()
    }
}
impl Document {
    /// Getter of the `compatMode` attribute.
    /// [`Document.compatMode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/compatMode)
    pub fn compat_mode(&self) -> JsString {
        self.inner.get("compatMode").as_::<JsString>()
    }
}
impl Document {
    /// Getter of the `characterSet` attribute.
    /// [`Document.characterSet`](https://developer.mozilla.org/en-US/docs/Web/API/Document/characterSet)
    pub fn character_set(&self) -> JsString {
        self.inner.get("characterSet").as_::<JsString>()
    }
}
impl Document {
    /// Getter of the `charset` attribute.
    /// [`Document.charset`](https://developer.mozilla.org/en-US/docs/Web/API/Document/charset)
    pub fn charset(&self) -> JsString {
        self.inner.get("charset").as_::<JsString>()
    }
}
impl Document {
    /// Getter of the `inputEncoding` attribute.
    /// [`Document.inputEncoding`](https://developer.mozilla.org/en-US/docs/Web/API/Document/inputEncoding)
    pub fn input_encoding(&self) -> JsString {
        self.inner.get("inputEncoding").as_::<JsString>()
    }
}
impl Document {
    /// Getter of the `contentType` attribute.
    /// [`Document.contentType`](https://developer.mozilla.org/en-US/docs/Web/API/Document/contentType)
    pub fn content_type(&self) -> JsString {
        self.inner.get("contentType").as_::<JsString>()
    }
}
impl Document {
    /// Getter of the `doctype` attribute.
    /// [`Document.doctype`](https://developer.mozilla.org/en-US/docs/Web/API/Document/doctype)
    pub fn doctype(&self) -> DocumentType {
        self.inner.get("doctype").as_::<DocumentType>()
    }
}
impl Document {
    /// Getter of the `documentElement` attribute.
    /// [`Document.documentElement`](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentElement)
    pub fn document_element(&self) -> Element {
        self.inner.get("documentElement").as_::<Element>()
    }
}
impl Document {
    /// The getElementsByTagName method.
    /// [`Document.getElementsByTagName`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagName)
    pub fn get_elements_by_tag_name(&self, qualified_name: &JsString) -> HTMLCollection {
        self.inner
            .call("getElementsByTagName", &[qualified_name.into()])
            .as_::<HTMLCollection>()
    }
}
impl Document {
    /// The getElementsByTagNameNS method.
    /// [`Document.getElementsByTagNameNS`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagNameNS)
    pub fn get_elements_by_tag_name_ns(
        &self,
        namespace: &JsString,
        local_name: &JsString,
    ) -> HTMLCollection {
        self.inner
            .call(
                "getElementsByTagNameNS",
                &[namespace.into(), local_name.into()],
            )
            .as_::<HTMLCollection>()
    }
}
impl Document {
    /// The getElementsByClassName method.
    /// [`Document.getElementsByClassName`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByClassName)
    pub fn get_elements_by_class_name(&self, class_names: &JsString) -> HTMLCollection {
        self.inner
            .call("getElementsByClassName", &[class_names.into()])
            .as_::<HTMLCollection>()
    }
}
impl Document {
    /// The createElement method.
    /// [`Document.createElement`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)
    pub fn create_element0(&self, local_name: &JsString) -> Element {
        self.inner
            .call("createElement", &[local_name.into()])
            .as_::<Element>()
    }
    /// The createElement method.
    /// [`Document.createElement`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)
    pub fn create_element1(&self, local_name: &JsString, options: &Any) -> Element {
        self.inner
            .call("createElement", &[local_name.into(), options.into()])
            .as_::<Element>()
    }
}
impl Document {
    /// The createElementNS method.
    /// [`Document.createElementNS`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)
    pub fn create_element_ns0(&self, namespace: &JsString, qualified_name: &JsString) -> Element {
        self.inner
            .call(
                "createElementNS",
                &[namespace.into(), qualified_name.into()],
            )
            .as_::<Element>()
    }
    /// The createElementNS method.
    /// [`Document.createElementNS`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)
    pub fn create_element_ns1(
        &self,
        namespace: &JsString,
        qualified_name: &JsString,
        options: &Any,
    ) -> Element {
        self.inner
            .call(
                "createElementNS",
                &[namespace.into(), qualified_name.into(), options.into()],
            )
            .as_::<Element>()
    }
}
impl Document {
    /// The createDocumentFragment method.
    /// [`Document.createDocumentFragment`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createDocumentFragment)
    pub fn create_document_fragment(&self) -> DocumentFragment {
        self.inner
            .call("createDocumentFragment", &[])
            .as_::<DocumentFragment>()
    }
}
impl Document {
    /// The createTextNode method.
    /// [`Document.createTextNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
    pub fn create_text_node(&self, data: &JsString) -> Text {
        self.inner
            .call("createTextNode", &[data.into()])
            .as_::<Text>()
    }
}
impl Document {
    /// The createCDATASection method.
    /// [`Document.createCDATASection`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createCDATASection)
    pub fn create_cdata_section(&self, data: &JsString) -> CDATASection {
        self.inner
            .call("createCDATASection", &[data.into()])
            .as_::<CDATASection>()
    }
}
impl Document {
    /// The createComment method.
    /// [`Document.createComment`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createComment)
    pub fn create_comment(&self, data: &JsString) -> Comment {
        self.inner
            .call("createComment", &[data.into()])
            .as_::<Comment>()
    }
}
impl Document {
    /// The createProcessingInstruction method.
    /// [`Document.createProcessingInstruction`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createProcessingInstruction)
    pub fn create_processing_instruction(
        &self,
        target: &JsString,
        data: &JsString,
    ) -> ProcessingInstruction {
        self.inner
            .call("createProcessingInstruction", &[target.into(), data.into()])
            .as_::<ProcessingInstruction>()
    }
}
impl Document {
    /// The importNode method.
    /// [`Document.importNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode)
    pub fn import_node0(&self, node: &Node) -> Node {
        self.inner.call("importNode", &[node.into()]).as_::<Node>()
    }
    /// The importNode method.
    /// [`Document.importNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode)
    pub fn import_node1(&self, node: &Node, options: &Any) -> Node {
        self.inner
            .call("importNode", &[node.into(), options.into()])
            .as_::<Node>()
    }
}
impl Document {
    /// The adoptNode method.
    /// [`Document.adoptNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptNode)
    pub fn adopt_node(&self, node: &Node) -> Node {
        self.inner.call("adoptNode", &[node.into()]).as_::<Node>()
    }
}
impl Document {
    /// The createAttribute method.
    /// [`Document.createAttribute`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttribute)
    pub fn create_attribute(&self, local_name: &JsString) -> Attr {
        self.inner
            .call("createAttribute", &[local_name.into()])
            .as_::<Attr>()
    }
}
impl Document {
    /// The createAttributeNS method.
    /// [`Document.createAttributeNS`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttributeNS)
    pub fn create_attribute_ns(&self, namespace: &JsString, qualified_name: &JsString) -> Attr {
        self.inner
            .call(
                "createAttributeNS",
                &[namespace.into(), qualified_name.into()],
            )
            .as_::<Attr>()
    }
}
impl Document {
    /// The createEvent method.
    /// [`Document.createEvent`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createEvent)
    pub fn create_event(&self, interface: &JsString) -> Event {
        self.inner
            .call("createEvent", &[interface.into()])
            .as_::<Event>()
    }
}
impl Document {
    /// The createRange method.
    /// [`Document.createRange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createRange)
    pub fn create_range(&self) -> Range {
        self.inner.call("createRange", &[]).as_::<Range>()
    }
}
impl Document {
    /// The createNodeIterator method.
    /// [`Document.createNodeIterator`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)
    pub fn create_node_iterator0(&self, root: &Node) -> NodeIterator {
        self.inner
            .call("createNodeIterator", &[root.into()])
            .as_::<NodeIterator>()
    }
    /// The createNodeIterator method.
    /// [`Document.createNodeIterator`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)
    pub fn create_node_iterator1(&self, root: &Node, what_to_show: u32) -> NodeIterator {
        self.inner
            .call("createNodeIterator", &[root.into(), what_to_show.into()])
            .as_::<NodeIterator>()
    }
    /// The createNodeIterator method.
    /// [`Document.createNodeIterator`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)
    pub fn create_node_iterator2(
        &self,
        root: &Node,
        what_to_show: u32,
        filter: &Function,
    ) -> NodeIterator {
        self.inner
            .call(
                "createNodeIterator",
                &[root.into(), what_to_show.into(), filter.into()],
            )
            .as_::<NodeIterator>()
    }
}
impl Document {
    /// The createTreeWalker method.
    /// [`Document.createTreeWalker`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)
    pub fn create_tree_walker0(&self, root: &Node) -> TreeWalker {
        self.inner
            .call("createTreeWalker", &[root.into()])
            .as_::<TreeWalker>()
    }
    /// The createTreeWalker method.
    /// [`Document.createTreeWalker`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)
    pub fn create_tree_walker1(&self, root: &Node, what_to_show: u32) -> TreeWalker {
        self.inner
            .call("createTreeWalker", &[root.into(), what_to_show.into()])
            .as_::<TreeWalker>()
    }
    /// The createTreeWalker method.
    /// [`Document.createTreeWalker`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)
    pub fn create_tree_walker2(
        &self,
        root: &Node,
        what_to_show: u32,
        filter: &Function,
    ) -> TreeWalker {
        self.inner
            .call(
                "createTreeWalker",
                &[root.into(), what_to_show.into(), filter.into()],
            )
            .as_::<TreeWalker>()
    }
}
impl Document {
    /// Getter of the `rootElement` attribute.
    /// [`Document.rootElement`](https://developer.mozilla.org/en-US/docs/Web/API/Document/rootElement)
    pub fn root_element(&self) -> SVGSVGElement {
        self.inner.get("rootElement").as_::<SVGSVGElement>()
    }
}
impl Document {
    /// Getter of the `namedFlows` attribute.
    /// [`Document.namedFlows`](https://developer.mozilla.org/en-US/docs/Web/API/Document/namedFlows)
    pub fn named_flows(&self) -> NamedFlowMap {
        self.inner.get("namedFlows").as_::<NamedFlowMap>()
    }
}
impl Document {
    /// The startViewTransition method.
    /// [`Document.startViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition)
    pub fn start_view_transition0(&self) -> ViewTransition {
        self.inner
            .call("startViewTransition", &[])
            .as_::<ViewTransition>()
    }
    /// The startViewTransition method.
    /// [`Document.startViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition)
    pub fn start_view_transition1(&self, callback_options: &Any) -> ViewTransition {
        self.inner
            .call("startViewTransition", &[callback_options.into()])
            .as_::<ViewTransition>()
    }
}
impl Document {
    /// The elementFromPoint method.
    /// [`Document.elementFromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/Document/elementFromPoint)
    pub fn element_from_point(&self, x: f64, y: f64) -> Element {
        self.inner
            .call("elementFromPoint", &[x.into(), y.into()])
            .as_::<Element>()
    }
}
impl Document {
    /// The elementsFromPoint method.
    /// [`Document.elementsFromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/Document/elementsFromPoint)
    pub fn elements_from_point(&self, x: f64, y: f64) -> TypedArray<Element> {
        self.inner
            .call("elementsFromPoint", &[x.into(), y.into()])
            .as_::<TypedArray<Element>>()
    }
}
impl Document {
    /// The caretPositionFromPoint method.
    /// [`Document.caretPositionFromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/Document/caretPositionFromPoint)
    pub fn caret_position_from_point0(&self, x: f64, y: f64) -> CaretPosition {
        self.inner
            .call("caretPositionFromPoint", &[x.into(), y.into()])
            .as_::<CaretPosition>()
    }
    /// The caretPositionFromPoint method.
    /// [`Document.caretPositionFromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/Document/caretPositionFromPoint)
    pub fn caret_position_from_point1(
        &self,
        x: f64,
        y: f64,
        options: &CaretPositionFromPointOptions,
    ) -> CaretPosition {
        self.inner
            .call(
                "caretPositionFromPoint",
                &[x.into(), y.into(), options.into()],
            )
            .as_::<CaretPosition>()
    }
}
impl Document {
    /// Getter of the `scrollingElement` attribute.
    /// [`Document.scrollingElement`](https://developer.mozilla.org/en-US/docs/Web/API/Document/scrollingElement)
    pub fn scrolling_element(&self) -> Element {
        self.inner.get("scrollingElement").as_::<Element>()
    }
}
impl Document {
    /// The measureElement method.
    /// [`Document.measureElement`](https://developer.mozilla.org/en-US/docs/Web/API/Document/measureElement)
    pub fn measure_element(&self, element: &Element) -> FontMetrics {
        self.inner
            .call("measureElement", &[element.into()])
            .as_::<FontMetrics>()
    }
}
impl Document {
    /// The measureText method.
    /// [`Document.measureText`](https://developer.mozilla.org/en-US/docs/Web/API/Document/measureText)
    pub fn measure_text(
        &self,
        text: &JsString,
        style_map: &StylePropertyMapReadOnly,
    ) -> FontMetrics {
        self.inner
            .call("measureText", &[text.into(), style_map.into()])
            .as_::<FontMetrics>()
    }
}
impl Document {
    /// Getter of the `fullscreenEnabled` attribute.
    /// [`Document.fullscreenEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreenEnabled)
    pub fn fullscreen_enabled(&self) -> bool {
        self.inner.get("fullscreenEnabled").as_::<bool>()
    }
}
impl Document {
    /// Getter of the `fullscreen` attribute.
    /// [`Document.fullscreen`](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreen)
    pub fn fullscreen(&self) -> bool {
        self.inner.get("fullscreen").as_::<bool>()
    }
}
impl Document {
    /// The exitFullscreen method.
    /// [`Document.exitFullscreen`](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitFullscreen)
    pub fn exit_fullscreen(&self) -> Promise<Undefined> {
        self.inner
            .call("exitFullscreen", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl Document {
    /// Getter of the `onfullscreenchange` attribute.
    /// [`Document.onfullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenchange)
    pub fn onfullscreenchange(&self) -> Any {
        self.inner.get("onfullscreenchange").as_::<Any>()
    }

    /// Setter of the `onfullscreenchange` attribute.
    /// [`Document.onfullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenchange)
    pub fn set_onfullscreenchange(&mut self, value: &Any) {
        self.inner.set("onfullscreenchange", value);
    }
}
impl Document {
    /// Getter of the `onfullscreenerror` attribute.
    /// [`Document.onfullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenerror)
    pub fn onfullscreenerror(&self) -> Any {
        self.inner.get("onfullscreenerror").as_::<Any>()
    }

    /// Setter of the `onfullscreenerror` attribute.
    /// [`Document.onfullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenerror)
    pub fn set_onfullscreenerror(&mut self, value: &Any) {
        self.inner.set("onfullscreenerror", value);
    }
}
impl Document {
    /// The parseHTMLUnsafe method.
    /// [`Document.parseHTMLUnsafe`](https://developer.mozilla.org/en-US/docs/Web/API/Document/parseHTMLUnsafe)
    pub fn parse_html_unsafe(html: &Any) -> Document {
        Any::global("Document")
            .call("parseHTMLUnsafe", &[html.into()])
            .as_::<Document>()
    }
}
impl Document {
    /// Getter of the `location` attribute.
    /// [`Document.location`](https://developer.mozilla.org/en-US/docs/Web/API/Document/location)
    pub fn location(&self) -> Location {
        self.inner.get("location").as_::<Location>()
    }
}
impl Document {
    /// Getter of the `domain` attribute.
    /// [`Document.domain`](https://developer.mozilla.org/en-US/docs/Web/API/Document/domain)
    pub fn domain(&self) -> JsString {
        self.inner.get("domain").as_::<JsString>()
    }

    /// Setter of the `domain` attribute.
    /// [`Document.domain`](https://developer.mozilla.org/en-US/docs/Web/API/Document/domain)
    pub fn set_domain(&mut self, value: &JsString) {
        self.inner.set("domain", value);
    }
}
impl Document {
    /// Getter of the `referrer` attribute.
    /// [`Document.referrer`](https://developer.mozilla.org/en-US/docs/Web/API/Document/referrer)
    pub fn referrer(&self) -> JsString {
        self.inner.get("referrer").as_::<JsString>()
    }
}
impl Document {
    /// Getter of the `cookie` attribute.
    /// [`Document.cookie`](https://developer.mozilla.org/en-US/docs/Web/API/Document/cookie)
    pub fn cookie(&self) -> JsString {
        self.inner.get("cookie").as_::<JsString>()
    }

    /// Setter of the `cookie` attribute.
    /// [`Document.cookie`](https://developer.mozilla.org/en-US/docs/Web/API/Document/cookie)
    pub fn set_cookie(&mut self, value: &JsString) {
        self.inner.set("cookie", value);
    }
}
impl Document {
    /// Getter of the `lastModified` attribute.
    /// [`Document.lastModified`](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastModified)
    pub fn last_modified(&self) -> JsString {
        self.inner.get("lastModified").as_::<JsString>()
    }
}
impl Document {
    /// Getter of the `readyState` attribute.
    /// [`Document.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/Document/readyState)
    pub fn ready_state(&self) -> DocumentReadyState {
        self.inner.get("readyState").as_::<DocumentReadyState>()
    }
}
impl Document {
    /// Getter of the `title` attribute.
    /// [`Document.title`](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }

    /// Setter of the `title` attribute.
    /// [`Document.title`](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)
    pub fn set_title(&mut self, value: &JsString) {
        self.inner.set("title", value);
    }
}
impl Document {
    /// Getter of the `dir` attribute.
    /// [`Document.dir`](https://developer.mozilla.org/en-US/docs/Web/API/Document/dir)
    pub fn dir(&self) -> JsString {
        self.inner.get("dir").as_::<JsString>()
    }

    /// Setter of the `dir` attribute.
    /// [`Document.dir`](https://developer.mozilla.org/en-US/docs/Web/API/Document/dir)
    pub fn set_dir(&mut self, value: &JsString) {
        self.inner.set("dir", value);
    }
}
impl Document {
    /// Getter of the `body` attribute.
    /// [`Document.body`](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)
    pub fn body(&self) -> HTMLElement {
        self.inner.get("body").as_::<HTMLElement>()
    }

    /// Setter of the `body` attribute.
    /// [`Document.body`](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)
    pub fn set_body(&mut self, value: &HTMLElement) {
        self.inner.set("body", value);
    }
}
impl Document {
    /// Getter of the `head` attribute.
    /// [`Document.head`](https://developer.mozilla.org/en-US/docs/Web/API/Document/head)
    pub fn head(&self) -> HTMLHeadElement {
        self.inner.get("head").as_::<HTMLHeadElement>()
    }
}
impl Document {
    /// Getter of the `images` attribute.
    /// [`Document.images`](https://developer.mozilla.org/en-US/docs/Web/API/Document/images)
    pub fn images(&self) -> HTMLCollection {
        self.inner.get("images").as_::<HTMLCollection>()
    }
}
impl Document {
    /// Getter of the `embeds` attribute.
    /// [`Document.embeds`](https://developer.mozilla.org/en-US/docs/Web/API/Document/embeds)
    pub fn embeds(&self) -> HTMLCollection {
        self.inner.get("embeds").as_::<HTMLCollection>()
    }
}
impl Document {
    /// Getter of the `plugins` attribute.
    /// [`Document.plugins`](https://developer.mozilla.org/en-US/docs/Web/API/Document/plugins)
    pub fn plugins(&self) -> HTMLCollection {
        self.inner.get("plugins").as_::<HTMLCollection>()
    }
}
impl Document {
    /// Getter of the `links` attribute.
    /// [`Document.links`](https://developer.mozilla.org/en-US/docs/Web/API/Document/links)
    pub fn links(&self) -> HTMLCollection {
        self.inner.get("links").as_::<HTMLCollection>()
    }
}
impl Document {
    /// Getter of the `forms` attribute.
    /// [`Document.forms`](https://developer.mozilla.org/en-US/docs/Web/API/Document/forms)
    pub fn forms(&self) -> HTMLCollection {
        self.inner.get("forms").as_::<HTMLCollection>()
    }
}
impl Document {
    /// Getter of the `scripts` attribute.
    /// [`Document.scripts`](https://developer.mozilla.org/en-US/docs/Web/API/Document/scripts)
    pub fn scripts(&self) -> HTMLCollection {
        self.inner.get("scripts").as_::<HTMLCollection>()
    }
}
impl Document {
    /// The getElementsByName method.
    /// [`Document.getElementsByName`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByName)
    pub fn get_elements_by_name(&self, element_name: &JsString) -> NodeList {
        self.inner
            .call("getElementsByName", &[element_name.into()])
            .as_::<NodeList>()
    }
}
impl Document {
    /// Getter of the `currentScript` attribute.
    /// [`Document.currentScript`](https://developer.mozilla.org/en-US/docs/Web/API/Document/currentScript)
    pub fn current_script(&self) -> Any {
        self.inner.get("currentScript").as_::<Any>()
    }
}
impl Document {
    /// The open method.
    /// [`Document.open`](https://developer.mozilla.org/en-US/docs/Web/API/Document/open)
    pub fn open(&self, url: &JsString, name: &JsString, features: &JsString) -> Any {
        self.inner
            .call("open", &[url.into(), name.into(), features.into()])
            .as_::<Any>()
    }
}
impl Document {
    /// The close method.
    /// [`Document.close`](https://developer.mozilla.org/en-US/docs/Web/API/Document/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl Document {
    /// The write method.
    /// [`Document.write`](https://developer.mozilla.org/en-US/docs/Web/API/Document/write)
    pub fn write(&self, text: &Any) -> Undefined {
        self.inner.call("write", &[text.into()]).as_::<Undefined>()
    }
}
impl Document {
    /// The writeln method.
    /// [`Document.writeln`](https://developer.mozilla.org/en-US/docs/Web/API/Document/writeln)
    pub fn writeln(&self, text: &Any) -> Undefined {
        self.inner
            .call("writeln", &[text.into()])
            .as_::<Undefined>()
    }
}
impl Document {
    /// Getter of the `defaultView` attribute.
    /// [`Document.defaultView`](https://developer.mozilla.org/en-US/docs/Web/API/Document/defaultView)
    pub fn default_view(&self) -> Any {
        self.inner.get("defaultView").as_::<Any>()
    }
}
impl Document {
    /// The hasFocus method.
    /// [`Document.hasFocus`](https://developer.mozilla.org/en-US/docs/Web/API/Document/hasFocus)
    pub fn has_focus(&self) -> bool {
        self.inner.call("hasFocus", &[]).as_::<bool>()
    }
}
impl Document {
    /// Getter of the `designMode` attribute.
    /// [`Document.designMode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/designMode)
    pub fn design_mode(&self) -> JsString {
        self.inner.get("designMode").as_::<JsString>()
    }

    /// Setter of the `designMode` attribute.
    /// [`Document.designMode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/designMode)
    pub fn set_design_mode(&mut self, value: &JsString) {
        self.inner.set("designMode", value);
    }
}
impl Document {
    /// The execCommand method.
    /// [`Document.execCommand`](https://developer.mozilla.org/en-US/docs/Web/API/Document/execCommand)
    pub fn exec_command0(&self, command_id: &JsString) -> bool {
        self.inner
            .call("execCommand", &[command_id.into()])
            .as_::<bool>()
    }
    /// The execCommand method.
    /// [`Document.execCommand`](https://developer.mozilla.org/en-US/docs/Web/API/Document/execCommand)
    pub fn exec_command1(&self, command_id: &JsString, show_ui: bool) -> bool {
        self.inner
            .call("execCommand", &[command_id.into(), show_ui.into()])
            .as_::<bool>()
    }
    /// The execCommand method.
    /// [`Document.execCommand`](https://developer.mozilla.org/en-US/docs/Web/API/Document/execCommand)
    pub fn exec_command2(&self, command_id: &JsString, show_ui: bool, value: &JsString) -> bool {
        self.inner
            .call(
                "execCommand",
                &[command_id.into(), show_ui.into(), value.into()],
            )
            .as_::<bool>()
    }
}
impl Document {
    /// The queryCommandEnabled method.
    /// [`Document.queryCommandEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Document/queryCommandEnabled)
    pub fn query_command_enabled(&self, command_id: &JsString) -> bool {
        self.inner
            .call("queryCommandEnabled", &[command_id.into()])
            .as_::<bool>()
    }
}
impl Document {
    /// The queryCommandIndeterm method.
    /// [`Document.queryCommandIndeterm`](https://developer.mozilla.org/en-US/docs/Web/API/Document/queryCommandIndeterm)
    pub fn query_command_indeterm(&self, command_id: &JsString) -> bool {
        self.inner
            .call("queryCommandIndeterm", &[command_id.into()])
            .as_::<bool>()
    }
}
impl Document {
    /// The queryCommandState method.
    /// [`Document.queryCommandState`](https://developer.mozilla.org/en-US/docs/Web/API/Document/queryCommandState)
    pub fn query_command_state(&self, command_id: &JsString) -> bool {
        self.inner
            .call("queryCommandState", &[command_id.into()])
            .as_::<bool>()
    }
}
impl Document {
    /// The queryCommandSupported method.
    /// [`Document.queryCommandSupported`](https://developer.mozilla.org/en-US/docs/Web/API/Document/queryCommandSupported)
    pub fn query_command_supported(&self, command_id: &JsString) -> bool {
        self.inner
            .call("queryCommandSupported", &[command_id.into()])
            .as_::<bool>()
    }
}
impl Document {
    /// The queryCommandValue method.
    /// [`Document.queryCommandValue`](https://developer.mozilla.org/en-US/docs/Web/API/Document/queryCommandValue)
    pub fn query_command_value(&self, command_id: &JsString) -> JsString {
        self.inner
            .call("queryCommandValue", &[command_id.into()])
            .as_::<JsString>()
    }
}
impl Document {
    /// Getter of the `hidden` attribute.
    /// [`Document.hidden`](https://developer.mozilla.org/en-US/docs/Web/API/Document/hidden)
    pub fn hidden(&self) -> bool {
        self.inner.get("hidden").as_::<bool>()
    }
}
impl Document {
    /// Getter of the `visibilityState` attribute.
    /// [`Document.visibilityState`](https://developer.mozilla.org/en-US/docs/Web/API/Document/visibilityState)
    pub fn visibility_state(&self) -> DocumentVisibilityState {
        self.inner
            .get("visibilityState")
            .as_::<DocumentVisibilityState>()
    }
}
impl Document {
    /// Getter of the `onreadystatechange` attribute.
    /// [`Document.onreadystatechange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreadystatechange)
    pub fn onreadystatechange(&self) -> Any {
        self.inner.get("onreadystatechange").as_::<Any>()
    }

    /// Setter of the `onreadystatechange` attribute.
    /// [`Document.onreadystatechange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreadystatechange)
    pub fn set_onreadystatechange(&mut self, value: &Any) {
        self.inner.set("onreadystatechange", value);
    }
}
impl Document {
    /// Getter of the `onvisibilitychange` attribute.
    /// [`Document.onvisibilitychange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvisibilitychange)
    pub fn onvisibilitychange(&self) -> Any {
        self.inner.get("onvisibilitychange").as_::<Any>()
    }

    /// Setter of the `onvisibilitychange` attribute.
    /// [`Document.onvisibilitychange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvisibilitychange)
    pub fn set_onvisibilitychange(&mut self, value: &Any) {
        self.inner.set("onvisibilitychange", value);
    }
}
impl Document {
    /// Getter of the `fgColor` attribute.
    /// [`Document.fgColor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/fgColor)
    pub fn fg_color(&self) -> JsString {
        self.inner.get("fgColor").as_::<JsString>()
    }

    /// Setter of the `fgColor` attribute.
    /// [`Document.fgColor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/fgColor)
    pub fn set_fg_color(&mut self, value: &JsString) {
        self.inner.set("fgColor", value);
    }
}
impl Document {
    /// Getter of the `linkColor` attribute.
    /// [`Document.linkColor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/linkColor)
    pub fn link_color(&self) -> JsString {
        self.inner.get("linkColor").as_::<JsString>()
    }

    /// Setter of the `linkColor` attribute.
    /// [`Document.linkColor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/linkColor)
    pub fn set_link_color(&mut self, value: &JsString) {
        self.inner.set("linkColor", value);
    }
}
impl Document {
    /// Getter of the `vlinkColor` attribute.
    /// [`Document.vlinkColor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/vlinkColor)
    pub fn vlink_color(&self) -> JsString {
        self.inner.get("vlinkColor").as_::<JsString>()
    }

    /// Setter of the `vlinkColor` attribute.
    /// [`Document.vlinkColor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/vlinkColor)
    pub fn set_vlink_color(&mut self, value: &JsString) {
        self.inner.set("vlinkColor", value);
    }
}
impl Document {
    /// Getter of the `alinkColor` attribute.
    /// [`Document.alinkColor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/alinkColor)
    pub fn alink_color(&self) -> JsString {
        self.inner.get("alinkColor").as_::<JsString>()
    }

    /// Setter of the `alinkColor` attribute.
    /// [`Document.alinkColor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/alinkColor)
    pub fn set_alink_color(&mut self, value: &JsString) {
        self.inner.set("alinkColor", value);
    }
}
impl Document {
    /// Getter of the `bgColor` attribute.
    /// [`Document.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/bgColor)
    pub fn bg_color(&self) -> JsString {
        self.inner.get("bgColor").as_::<JsString>()
    }

    /// Setter of the `bgColor` attribute.
    /// [`Document.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/bgColor)
    pub fn set_bg_color(&mut self, value: &JsString) {
        self.inner.set("bgColor", value);
    }
}
impl Document {
    /// Getter of the `anchors` attribute.
    /// [`Document.anchors`](https://developer.mozilla.org/en-US/docs/Web/API/Document/anchors)
    pub fn anchors(&self) -> HTMLCollection {
        self.inner.get("anchors").as_::<HTMLCollection>()
    }
}
impl Document {
    /// Getter of the `applets` attribute.
    /// [`Document.applets`](https://developer.mozilla.org/en-US/docs/Web/API/Document/applets)
    pub fn applets(&self) -> HTMLCollection {
        self.inner.get("applets").as_::<HTMLCollection>()
    }
}
impl Document {
    /// The clear method.
    /// [`Document.clear`](https://developer.mozilla.org/en-US/docs/Web/API/Document/clear)
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
impl Document {
    /// The captureEvents method.
    /// [`Document.captureEvents`](https://developer.mozilla.org/en-US/docs/Web/API/Document/captureEvents)
    pub fn capture_events(&self) -> Undefined {
        self.inner.call("captureEvents", &[]).as_::<Undefined>()
    }
}
impl Document {
    /// The releaseEvents method.
    /// [`Document.releaseEvents`](https://developer.mozilla.org/en-US/docs/Web/API/Document/releaseEvents)
    pub fn release_events(&self) -> Undefined {
        self.inner.call("releaseEvents", &[]).as_::<Undefined>()
    }
}
impl Document {
    /// Getter of the `all` attribute.
    /// [`Document.all`](https://developer.mozilla.org/en-US/docs/Web/API/Document/all)
    pub fn all(&self) -> HTMLAllCollection {
        self.inner.get("all").as_::<HTMLAllCollection>()
    }
}
impl Document {
    /// Getter of the `onfreeze` attribute.
    /// [`Document.onfreeze`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfreeze)
    pub fn onfreeze(&self) -> Any {
        self.inner.get("onfreeze").as_::<Any>()
    }

    /// Setter of the `onfreeze` attribute.
    /// [`Document.onfreeze`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfreeze)
    pub fn set_onfreeze(&mut self, value: &Any) {
        self.inner.set("onfreeze", value);
    }
}
impl Document {
    /// Getter of the `onresume` attribute.
    /// [`Document.onresume`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onresume)
    pub fn onresume(&self) -> Any {
        self.inner.get("onresume").as_::<Any>()
    }

    /// Setter of the `onresume` attribute.
    /// [`Document.onresume`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onresume)
    pub fn set_onresume(&mut self, value: &Any) {
        self.inner.set("onresume", value);
    }
}
impl Document {
    /// Getter of the `wasDiscarded` attribute.
    /// [`Document.wasDiscarded`](https://developer.mozilla.org/en-US/docs/Web/API/Document/wasDiscarded)
    pub fn was_discarded(&self) -> bool {
        self.inner.get("wasDiscarded").as_::<bool>()
    }
}
impl Document {
    /// Getter of the `permissionsPolicy` attribute.
    /// [`Document.permissionsPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/Document/permissionsPolicy)
    pub fn permissions_policy(&self) -> PermissionsPolicy {
        self.inner
            .get("permissionsPolicy")
            .as_::<PermissionsPolicy>()
    }
}
impl Document {
    /// Getter of the `pictureInPictureEnabled` attribute.
    /// [`Document.pictureInPictureEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Document/pictureInPictureEnabled)
    pub fn picture_in_picture_enabled(&self) -> bool {
        self.inner.get("pictureInPictureEnabled").as_::<bool>()
    }
}
impl Document {
    /// The exitPictureInPicture method.
    /// [`Document.exitPictureInPicture`](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPictureInPicture)
    pub fn exit_picture_in_picture(&self) -> Promise<Undefined> {
        self.inner
            .call("exitPictureInPicture", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl Document {
    /// Getter of the `onpointerlockchange` attribute.
    /// [`Document.onpointerlockchange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockchange)
    pub fn onpointerlockchange(&self) -> Any {
        self.inner.get("onpointerlockchange").as_::<Any>()
    }

    /// Setter of the `onpointerlockchange` attribute.
    /// [`Document.onpointerlockchange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockchange)
    pub fn set_onpointerlockchange(&mut self, value: &Any) {
        self.inner.set("onpointerlockchange", value);
    }
}
impl Document {
    /// Getter of the `onpointerlockerror` attribute.
    /// [`Document.onpointerlockerror`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockerror)
    pub fn onpointerlockerror(&self) -> Any {
        self.inner.get("onpointerlockerror").as_::<Any>()
    }

    /// Setter of the `onpointerlockerror` attribute.
    /// [`Document.onpointerlockerror`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockerror)
    pub fn set_onpointerlockerror(&mut self, value: &Any) {
        self.inner.set("onpointerlockerror", value);
    }
}
impl Document {
    /// The exitPointerLock method.
    /// [`Document.exitPointerLock`](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPointerLock)
    pub fn exit_pointer_lock(&self) -> Undefined {
        self.inner.call("exitPointerLock", &[]).as_::<Undefined>()
    }
}
impl Document {
    /// Getter of the `prerendering` attribute.
    /// [`Document.prerendering`](https://developer.mozilla.org/en-US/docs/Web/API/Document/prerendering)
    pub fn prerendering(&self) -> bool {
        self.inner.get("prerendering").as_::<bool>()
    }
}
impl Document {
    /// Getter of the `onprerenderingchange` attribute.
    /// [`Document.onprerenderingchange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onprerenderingchange)
    pub fn onprerenderingchange(&self) -> Any {
        self.inner.get("onprerenderingchange").as_::<Any>()
    }

    /// Setter of the `onprerenderingchange` attribute.
    /// [`Document.onprerenderingchange`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onprerenderingchange)
    pub fn set_onprerenderingchange(&mut self, value: &Any) {
        self.inner.set("onprerenderingchange", value);
    }
}
impl Document {
    /// The requestStorageAccessFor method.
    /// [`Document.requestStorageAccessFor`](https://developer.mozilla.org/en-US/docs/Web/API/Document/requestStorageAccessFor)
    pub fn request_storage_access_for(&self, requested_origin: &JsString) -> Promise<Undefined> {
        self.inner
            .call("requestStorageAccessFor", &[requested_origin.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl Document {
    /// The hasUnpartitionedCookieAccess method.
    /// [`Document.hasUnpartitionedCookieAccess`](https://developer.mozilla.org/en-US/docs/Web/API/Document/hasUnpartitionedCookieAccess)
    pub fn has_unpartitioned_cookie_access(&self) -> Promise<bool> {
        self.inner
            .call("hasUnpartitionedCookieAccess", &[])
            .as_::<Promise<bool>>()
    }
}
impl Document {
    /// Getter of the `fragmentDirective` attribute.
    /// [`Document.fragmentDirective`](https://developer.mozilla.org/en-US/docs/Web/API/Document/fragmentDirective)
    pub fn fragment_directive(&self) -> FragmentDirective {
        self.inner
            .get("fragmentDirective")
            .as_::<FragmentDirective>()
    }
}
impl Document {
    /// The getSelection method.
    /// [`Document.getSelection`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getSelection)
    pub fn get_selection(&self) -> Selection {
        self.inner.call("getSelection", &[]).as_::<Selection>()
    }
}
impl Document {
    /// The hasStorageAccess method.
    /// [`Document.hasStorageAccess`](https://developer.mozilla.org/en-US/docs/Web/API/Document/hasStorageAccess)
    pub fn has_storage_access(&self) -> Promise<bool> {
        self.inner
            .call("hasStorageAccess", &[])
            .as_::<Promise<bool>>()
    }
}
impl Document {
    /// The requestStorageAccess method.
    /// [`Document.requestStorageAccess`](https://developer.mozilla.org/en-US/docs/Web/API/Document/requestStorageAccess)
    pub fn request_storage_access(&self) -> Promise<Undefined> {
        self.inner
            .call("requestStorageAccess", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl Document {
    /// The hasPrivateToken method.
    /// [`Document.hasPrivateToken`](https://developer.mozilla.org/en-US/docs/Web/API/Document/hasPrivateToken)
    pub fn has_private_token(&self, issuer: &JsString) -> Promise<bool> {
        self.inner
            .call("hasPrivateToken", &[issuer.into()])
            .as_::<Promise<bool>>()
    }
}
impl Document {
    /// The hasRedemptionRecord method.
    /// [`Document.hasRedemptionRecord`](https://developer.mozilla.org/en-US/docs/Web/API/Document/hasRedemptionRecord)
    pub fn has_redemption_record(&self, issuer: &JsString) -> Promise<bool> {
        self.inner
            .call("hasRedemptionRecord", &[issuer.into()])
            .as_::<Promise<bool>>()
    }
}
impl Document {
    /// Getter of the `timeline` attribute.
    /// [`Document.timeline`](https://developer.mozilla.org/en-US/docs/Web/API/Document/timeline)
    pub fn timeline(&self) -> DocumentTimeline {
        self.inner.get("timeline").as_::<DocumentTimeline>()
    }
}
impl Document {
    /// Getter of the `fonts` attribute.
    /// [`Document.fonts`](https://developer.mozilla.org/en-US/docs/Web/API/Document/fonts)
    pub fn fonts(&self) -> FontFaceSet {
        self.inner.get("fonts").as_::<FontFaceSet>()
    }
}
impl Document {
    /// The getBoxQuads method.
    /// [`Document.getBoxQuads`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getBoxQuads)
    pub fn get_box_quads0(&self) -> TypedArray<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[])
            .as_::<TypedArray<DOMQuad>>()
    }
    /// The getBoxQuads method.
    /// [`Document.getBoxQuads`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getBoxQuads)
    pub fn get_box_quads1(&self, options: &BoxQuadOptions) -> TypedArray<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[options.into()])
            .as_::<TypedArray<DOMQuad>>()
    }
}
impl Document {
    /// The convertQuadFromNode method.
    /// [`Document.convertQuadFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)
    pub fn convert_quad_from_node0(&self, quad: &DOMQuadInit, from: &Any) -> DOMQuad {
        self.inner
            .call("convertQuadFromNode", &[quad.into(), from.into()])
            .as_::<DOMQuad>()
    }
    /// The convertQuadFromNode method.
    /// [`Document.convertQuadFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)
    pub fn convert_quad_from_node1(
        &self,
        quad: &DOMQuadInit,
        from: &Any,
        options: &ConvertCoordinateOptions,
    ) -> DOMQuad {
        self.inner
            .call(
                "convertQuadFromNode",
                &[quad.into(), from.into(), options.into()],
            )
            .as_::<DOMQuad>()
    }
}
impl Document {
    /// The convertRectFromNode method.
    /// [`Document.convertRectFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)
    pub fn convert_rect_from_node0(&self, rect: &DOMRectReadOnly, from: &Any) -> DOMQuad {
        self.inner
            .call("convertRectFromNode", &[rect.into(), from.into()])
            .as_::<DOMQuad>()
    }
    /// The convertRectFromNode method.
    /// [`Document.convertRectFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)
    pub fn convert_rect_from_node1(
        &self,
        rect: &DOMRectReadOnly,
        from: &Any,
        options: &ConvertCoordinateOptions,
    ) -> DOMQuad {
        self.inner
            .call(
                "convertRectFromNode",
                &[rect.into(), from.into(), options.into()],
            )
            .as_::<DOMQuad>()
    }
}
impl Document {
    /// The convertPointFromNode method.
    /// [`Document.convertPointFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)
    pub fn convert_point_from_node0(&self, point: &DOMPointInit, from: &Any) -> DOMPoint {
        self.inner
            .call("convertPointFromNode", &[point.into(), from.into()])
            .as_::<DOMPoint>()
    }
    /// The convertPointFromNode method.
    /// [`Document.convertPointFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)
    pub fn convert_point_from_node1(
        &self,
        point: &DOMPointInit,
        from: &Any,
        options: &ConvertCoordinateOptions,
    ) -> DOMPoint {
        self.inner
            .call(
                "convertPointFromNode",
                &[point.into(), from.into(), options.into()],
            )
            .as_::<DOMPoint>()
    }
}
impl Document {
    /// The getElementById method.
    /// [`Document.getElementById`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById)
    pub fn get_element_by_id(&self, element_id: &JsString) -> Element {
        self.inner
            .call("getElementById", &[element_id.into()])
            .as_::<Element>()
    }
}
impl Document {
    /// The getAnimations method.
    /// [`Document.getAnimations`](https://developer.mozilla.org/en-US/docs/Web/API/Document/getAnimations)
    pub fn get_animations(&self) -> TypedArray<Animation> {
        self.inner
            .call("getAnimations", &[])
            .as_::<TypedArray<Animation>>()
    }
}
impl Document {
    /// Getter of the `children` attribute.
    /// [`Document.children`](https://developer.mozilla.org/en-US/docs/Web/API/Document/children)
    pub fn children(&self) -> HTMLCollection {
        self.inner.get("children").as_::<HTMLCollection>()
    }
}
impl Document {
    /// Getter of the `firstElementChild` attribute.
    /// [`Document.firstElementChild`](https://developer.mozilla.org/en-US/docs/Web/API/Document/firstElementChild)
    pub fn first_element_child(&self) -> Element {
        self.inner.get("firstElementChild").as_::<Element>()
    }
}
impl Document {
    /// Getter of the `lastElementChild` attribute.
    /// [`Document.lastElementChild`](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastElementChild)
    pub fn last_element_child(&self) -> Element {
        self.inner.get("lastElementChild").as_::<Element>()
    }
}
impl Document {
    /// Getter of the `childElementCount` attribute.
    /// [`Document.childElementCount`](https://developer.mozilla.org/en-US/docs/Web/API/Document/childElementCount)
    pub fn child_element_count(&self) -> u32 {
        self.inner.get("childElementCount").as_::<u32>()
    }
}
impl Document {
    /// The prepend method.
    /// [`Document.prepend`](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)
    pub fn prepend(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("prepend", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl Document {
    /// The append method.
    /// [`Document.append`](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)
    pub fn append(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("append", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl Document {
    /// The replaceChildren method.
    /// [`Document.replaceChildren`](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)
    pub fn replace_children(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("replaceChildren", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl Document {
    /// The moveBefore method.
    /// [`Document.moveBefore`](https://developer.mozilla.org/en-US/docs/Web/API/Document/moveBefore)
    pub fn move_before(&self, node: &Node, child: &Node) -> Undefined {
        self.inner
            .call("moveBefore", &[node.into(), child.into()])
            .as_::<Undefined>()
    }
}
impl Document {
    /// The querySelector method.
    /// [`Document.querySelector`](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector)
    pub fn query_selector(&self, selectors: &JsString) -> Element {
        self.inner
            .call("querySelector", &[selectors.into()])
            .as_::<Element>()
    }
}
impl Document {
    /// The querySelectorAll method.
    /// [`Document.querySelectorAll`](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll)
    pub fn query_selector_all(&self, selectors: &JsString) -> NodeList {
        self.inner
            .call("querySelectorAll", &[selectors.into()])
            .as_::<NodeList>()
    }
}
impl Document {
    /// The createExpression method.
    /// [`Document.createExpression`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)
    pub fn create_expression0(&self, expression: &JsString) -> XPathExpression {
        self.inner
            .call("createExpression", &[expression.into()])
            .as_::<XPathExpression>()
    }
    /// The createExpression method.
    /// [`Document.createExpression`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)
    pub fn create_expression1(
        &self,
        expression: &JsString,
        resolver: &Function,
    ) -> XPathExpression {
        self.inner
            .call("createExpression", &[expression.into(), resolver.into()])
            .as_::<XPathExpression>()
    }
}
impl Document {
    /// The createNSResolver method.
    /// [`Document.createNSResolver`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNSResolver)
    pub fn create_ns_resolver(&self, node_resolver: &Node) -> Node {
        self.inner
            .call("createNSResolver", &[node_resolver.into()])
            .as_::<Node>()
    }
}
impl Document {
    /// The evaluate method.
    /// [`Document.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    pub fn evaluate0(&self, expression: &JsString, context_node: &Node) -> XPathResult {
        self.inner
            .call("evaluate", &[expression.into(), context_node.into()])
            .as_::<XPathResult>()
    }
    /// The evaluate method.
    /// [`Document.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    pub fn evaluate1(
        &self,
        expression: &JsString,
        context_node: &Node,
        resolver: &Function,
    ) -> XPathResult {
        self.inner
            .call(
                "evaluate",
                &[expression.into(), context_node.into(), resolver.into()],
            )
            .as_::<XPathResult>()
    }
    /// The evaluate method.
    /// [`Document.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    pub fn evaluate2(
        &self,
        expression: &JsString,
        context_node: &Node,
        resolver: &Function,
        type_: u16,
    ) -> XPathResult {
        self.inner
            .call(
                "evaluate",
                &[
                    expression.into(),
                    context_node.into(),
                    resolver.into(),
                    type_.into(),
                ],
            )
            .as_::<XPathResult>()
    }
    /// The evaluate method.
    /// [`Document.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)
    pub fn evaluate3(
        &self,
        expression: &JsString,
        context_node: &Node,
        resolver: &Function,
        type_: u16,
        result: &XPathResult,
    ) -> XPathResult {
        self.inner
            .call(
                "evaluate",
                &[
                    expression.into(),
                    context_node.into(),
                    resolver.into(),
                    type_.into(),
                    result.into(),
                ],
            )
            .as_::<XPathResult>()
    }
}
impl Document {
    /// Getter of the `onbeforexrselect` attribute.
    /// [`Document.onbeforexrselect`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforexrselect)
    pub fn onbeforexrselect(&self) -> Any {
        self.inner.get("onbeforexrselect").as_::<Any>()
    }

    /// Setter of the `onbeforexrselect` attribute.
    /// [`Document.onbeforexrselect`](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforexrselect)
    pub fn set_onbeforexrselect(&mut self, value: &Any) {
        self.inner.set("onbeforexrselect", value);
    }
}
