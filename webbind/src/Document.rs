use super::*;

#[derive(Clone, Debug)]
pub struct CaretPositionFromPointOptions {
    inner: emlite::Val,
}
impl FromVal for CaretPositionFromPointOptions {
    fn from_val(v: &emlite::Val) -> Self {
        CaretPositionFromPointOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CaretPositionFromPointOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CaretPositionFromPointOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CaretPositionFromPointOptions> for emlite::Val {
    fn from(s: CaretPositionFromPointOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CaretPositionFromPointOptions {
    pub fn shadow_roots(&self) -> jsbind::Sequence<ShadowRoot> {
        self.inner
            .get("shadowRoots")
            .as_::<jsbind::Sequence<ShadowRoot>>()
    }

    pub fn set_shadow_roots(&mut self, value: jsbind::Sequence<ShadowRoot>) {
        self.inner.set("shadowRoots", value);
    }
}
#[derive(Clone, Debug)]
pub struct BoxQuadOptions {
    inner: emlite::Val,
}
impl FromVal for BoxQuadOptions {
    fn from_val(v: &emlite::Val) -> Self {
        BoxQuadOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BoxQuadOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BoxQuadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BoxQuadOptions> for emlite::Val {
    fn from(s: BoxQuadOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BoxQuadOptions {
    pub fn box_(&self) -> CSSBoxType {
        self.inner.get("box").as_::<CSSBoxType>()
    }

    pub fn set_box_(&mut self, value: CSSBoxType) {
        self.inner.set("box", value);
    }
}
impl BoxQuadOptions {
    pub fn relative_to(&self) -> jsbind::Any {
        self.inner.get("relativeTo").as_::<jsbind::Any>()
    }

    pub fn set_relative_to(&mut self, value: jsbind::Any) {
        self.inner.set("relativeTo", value);
    }
}
#[derive(Clone, Debug)]
pub struct DOMQuadInit {
    inner: emlite::Val,
}
impl FromVal for DOMQuadInit {
    fn from_val(v: &emlite::Val) -> Self {
        DOMQuadInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DOMQuadInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DOMQuadInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DOMQuadInit> for emlite::Val {
    fn from(s: DOMQuadInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DOMQuadInit {
    pub fn p1(&self) -> DOMPointInit {
        self.inner.get("p1").as_::<DOMPointInit>()
    }

    pub fn set_p1(&mut self, value: DOMPointInit) {
        self.inner.set("p1", value);
    }
}
impl DOMQuadInit {
    pub fn p2(&self) -> DOMPointInit {
        self.inner.get("p2").as_::<DOMPointInit>()
    }

    pub fn set_p2(&mut self, value: DOMPointInit) {
        self.inner.set("p2", value);
    }
}
impl DOMQuadInit {
    pub fn p3(&self) -> DOMPointInit {
        self.inner.get("p3").as_::<DOMPointInit>()
    }

    pub fn set_p3(&mut self, value: DOMPointInit) {
        self.inner.set("p3", value);
    }
}
impl DOMQuadInit {
    pub fn p4(&self) -> DOMPointInit {
        self.inner.get("p4").as_::<DOMPointInit>()
    }

    pub fn set_p4(&mut self, value: DOMPointInit) {
        self.inner.set("p4", value);
    }
}
#[derive(Clone, Debug)]
pub struct ConvertCoordinateOptions {
    inner: emlite::Val,
}
impl FromVal for ConvertCoordinateOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ConvertCoordinateOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ConvertCoordinateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ConvertCoordinateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ConvertCoordinateOptions> for emlite::Val {
    fn from(s: ConvertCoordinateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ConvertCoordinateOptions {
    pub fn from_box(&self) -> CSSBoxType {
        self.inner.get("fromBox").as_::<CSSBoxType>()
    }

    pub fn set_from_box(&mut self, value: CSSBoxType) {
        self.inner.set("fromBox", value);
    }
}
impl ConvertCoordinateOptions {
    pub fn to_box(&self) -> CSSBoxType {
        self.inner.get("toBox").as_::<CSSBoxType>()
    }

    pub fn set_to_box(&mut self, value: CSSBoxType) {
        self.inner.set("toBox", value);
    }
}
#[derive(Clone, Debug)]
pub struct Document {
    inner: Node,
}
impl FromVal for Document {
    fn from_val(v: &emlite::Val) -> Self {
        Document {
            inner: Node::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for Document {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Document> for emlite::Val {
    fn from(s: Document) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Document {
    pub fn new() -> Document {
        Self {
            inner: emlite::Val::global("Document").new(&[]).as_::<Node>(),
        }
    }
}
impl Document {
    pub fn implementation(&self) -> DOMImplementation {
        self.inner.get("implementation").as_::<DOMImplementation>()
    }
}
impl Document {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("URL").as_::<jsbind::USVString>()
    }
}
impl Document {
    pub fn document_uri(&self) -> jsbind::USVString {
        self.inner.get("documentURI").as_::<jsbind::USVString>()
    }
}
impl Document {
    pub fn compat_mode(&self) -> jsbind::DOMString {
        self.inner.get("compatMode").as_::<jsbind::DOMString>()
    }
}
impl Document {
    pub fn character_set(&self) -> jsbind::DOMString {
        self.inner.get("characterSet").as_::<jsbind::DOMString>()
    }
}
impl Document {
    pub fn charset(&self) -> jsbind::DOMString {
        self.inner.get("charset").as_::<jsbind::DOMString>()
    }
}
impl Document {
    pub fn input_encoding(&self) -> jsbind::DOMString {
        self.inner.get("inputEncoding").as_::<jsbind::DOMString>()
    }
}
impl Document {
    pub fn content_type(&self) -> jsbind::DOMString {
        self.inner.get("contentType").as_::<jsbind::DOMString>()
    }
}
impl Document {
    pub fn doctype(&self) -> DocumentType {
        self.inner.get("doctype").as_::<DocumentType>()
    }
}
impl Document {
    pub fn document_element(&self) -> Element {
        self.inner.get("documentElement").as_::<Element>()
    }
}
impl Document {
    pub fn get_elements_by_tag_name(&self, qualified_name: jsbind::DOMString) -> HTMLCollection {
        self.inner
            .call("getElementsByTagName", &[qualified_name.into()])
            .as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn get_elements_by_tag_name_ns(
        &self,
        namespace: jsbind::DOMString,
        local_name: jsbind::DOMString,
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
    pub fn get_elements_by_class_name(&self, class_names: jsbind::DOMString) -> HTMLCollection {
        self.inner
            .call("getElementsByClassName", &[class_names.into()])
            .as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn create_element0(&self, local_name: jsbind::DOMString) -> Element {
        self.inner
            .call("createElement", &[local_name.into()])
            .as_::<Element>()
    }

    pub fn create_element1(&self, local_name: jsbind::DOMString, options: jsbind::Any) -> Element {
        self.inner
            .call("createElement", &[local_name.into(), options.into()])
            .as_::<Element>()
    }
}
impl Document {
    pub fn create_element_ns0(
        &self,
        namespace: jsbind::DOMString,
        qualified_name: jsbind::DOMString,
    ) -> Element {
        self.inner
            .call(
                "createElementNS",
                &[namespace.into(), qualified_name.into()],
            )
            .as_::<Element>()
    }

    pub fn create_element_ns1(
        &self,
        namespace: jsbind::DOMString,
        qualified_name: jsbind::DOMString,
        options: jsbind::Any,
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
    pub fn create_document_fragment(&self) -> DocumentFragment {
        self.inner
            .call("createDocumentFragment", &[])
            .as_::<DocumentFragment>()
    }
}
impl Document {
    pub fn create_text_node(&self, data: jsbind::DOMString) -> Text {
        self.inner
            .call("createTextNode", &[data.into()])
            .as_::<Text>()
    }
}
impl Document {
    pub fn create_cdata_section(&self, data: jsbind::DOMString) -> CDATASection {
        self.inner
            .call("createCDATASection", &[data.into()])
            .as_::<CDATASection>()
    }
}
impl Document {
    pub fn create_comment(&self, data: jsbind::DOMString) -> Comment {
        self.inner
            .call("createComment", &[data.into()])
            .as_::<Comment>()
    }
}
impl Document {
    pub fn create_processing_instruction(
        &self,
        target: jsbind::DOMString,
        data: jsbind::DOMString,
    ) -> ProcessingInstruction {
        self.inner
            .call("createProcessingInstruction", &[target.into(), data.into()])
            .as_::<ProcessingInstruction>()
    }
}
impl Document {
    pub fn import_node0(&self, node: Node) -> Node {
        self.inner.call("importNode", &[node.into()]).as_::<Node>()
    }

    pub fn import_node1(&self, node: Node, options: jsbind::Any) -> Node {
        self.inner
            .call("importNode", &[node.into(), options.into()])
            .as_::<Node>()
    }
}
impl Document {
    pub fn adopt_node(&self, node: Node) -> Node {
        self.inner.call("adoptNode", &[node.into()]).as_::<Node>()
    }
}
impl Document {
    pub fn create_attribute(&self, local_name: jsbind::DOMString) -> Attr {
        self.inner
            .call("createAttribute", &[local_name.into()])
            .as_::<Attr>()
    }
}
impl Document {
    pub fn create_attribute_ns(
        &self,
        namespace: jsbind::DOMString,
        qualified_name: jsbind::DOMString,
    ) -> Attr {
        self.inner
            .call(
                "createAttributeNS",
                &[namespace.into(), qualified_name.into()],
            )
            .as_::<Attr>()
    }
}
impl Document {
    pub fn create_event(&self, interface: jsbind::DOMString) -> Event {
        self.inner
            .call("createEvent", &[interface.into()])
            .as_::<Event>()
    }
}
impl Document {
    pub fn create_range(&self) -> Range {
        self.inner.call("createRange", &[]).as_::<Range>()
    }
}
impl Document {
    pub fn create_node_iterator0(&self, root: Node) -> NodeIterator {
        self.inner
            .call("createNodeIterator", &[root.into()])
            .as_::<NodeIterator>()
    }

    pub fn create_node_iterator1(&self, root: Node, what_to_show: u32) -> NodeIterator {
        self.inner
            .call("createNodeIterator", &[root.into(), what_to_show.into()])
            .as_::<NodeIterator>()
    }

    pub fn create_node_iterator2(
        &self,
        root: Node,
        what_to_show: u32,
        filter: jsbind::Function,
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
    pub fn create_tree_walker0(&self, root: Node) -> TreeWalker {
        self.inner
            .call("createTreeWalker", &[root.into()])
            .as_::<TreeWalker>()
    }

    pub fn create_tree_walker1(&self, root: Node, what_to_show: u32) -> TreeWalker {
        self.inner
            .call("createTreeWalker", &[root.into(), what_to_show.into()])
            .as_::<TreeWalker>()
    }

    pub fn create_tree_walker2(
        &self,
        root: Node,
        what_to_show: u32,
        filter: jsbind::Function,
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
    pub fn root_element(&self) -> SVGSVGElement {
        self.inner.get("rootElement").as_::<SVGSVGElement>()
    }
}
impl Document {
    pub fn named_flows(&self) -> NamedFlowMap {
        self.inner.get("namedFlows").as_::<NamedFlowMap>()
    }
}
impl Document {
    pub fn start_view_transition0(&self) -> ViewTransition {
        self.inner
            .call("startViewTransition", &[])
            .as_::<ViewTransition>()
    }

    pub fn start_view_transition1(&self, callback_options: jsbind::Any) -> ViewTransition {
        self.inner
            .call("startViewTransition", &[callback_options.into()])
            .as_::<ViewTransition>()
    }
}
impl Document {
    pub fn element_from_point(&self, x: f64, y: f64) -> Element {
        self.inner
            .call("elementFromPoint", &[x.into(), y.into()])
            .as_::<Element>()
    }
}
impl Document {
    pub fn elements_from_point(&self, x: f64, y: f64) -> jsbind::Sequence<Element> {
        self.inner
            .call("elementsFromPoint", &[x.into(), y.into()])
            .as_::<jsbind::Sequence<Element>>()
    }
}
impl Document {
    pub fn caret_position_from_point0(&self, x: f64, y: f64) -> CaretPosition {
        self.inner
            .call("caretPositionFromPoint", &[x.into(), y.into()])
            .as_::<CaretPosition>()
    }

    pub fn caret_position_from_point1(
        &self,
        x: f64,
        y: f64,
        options: CaretPositionFromPointOptions,
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
    pub fn scrolling_element(&self) -> Element {
        self.inner.get("scrollingElement").as_::<Element>()
    }
}
impl Document {
    pub fn measure_element(&self, element: Element) -> FontMetrics {
        self.inner
            .call("measureElement", &[element.into()])
            .as_::<FontMetrics>()
    }
}
impl Document {
    pub fn measure_text(
        &self,
        text: jsbind::DOMString,
        style_map: StylePropertyMapReadOnly,
    ) -> FontMetrics {
        self.inner
            .call("measureText", &[text.into(), style_map.into()])
            .as_::<FontMetrics>()
    }
}
impl Document {
    pub fn fullscreen_enabled(&self) -> bool {
        self.inner.get("fullscreenEnabled").as_::<bool>()
    }
}
impl Document {
    pub fn fullscreen(&self) -> bool {
        self.inner.get("fullscreen").as_::<bool>()
    }
}
impl Document {
    pub fn exit_fullscreen(&self) -> jsbind::Promise {
        self.inner
            .call("exitFullscreen", &[])
            .as_::<jsbind::Promise>()
    }
}
impl Document {
    pub fn onfullscreenchange(&self) -> jsbind::Any {
        self.inner.get("onfullscreenchange").as_::<jsbind::Any>()
    }

    pub fn set_onfullscreenchange(&mut self, value: jsbind::Any) {
        self.inner.set("onfullscreenchange", value);
    }
}
impl Document {
    pub fn onfullscreenerror(&self) -> jsbind::Any {
        self.inner.get("onfullscreenerror").as_::<jsbind::Any>()
    }

    pub fn set_onfullscreenerror(&mut self, value: jsbind::Any) {
        self.inner.set("onfullscreenerror", value);
    }
}
impl Document {
    pub fn parse_html_unsafe(html: jsbind::Any) -> Document {
        emlite::Val::global("document")
            .call("parseHTMLUnsafe", &[html.into()])
            .as_::<Document>()
    }
}
impl Document {
    pub fn location(&self) -> jsbind::Any {
        self.inner.get("location").as_::<jsbind::Any>()
    }
}
impl Document {
    pub fn domain(&self) -> jsbind::USVString {
        self.inner.get("domain").as_::<jsbind::USVString>()
    }

    pub fn set_domain(&mut self, value: jsbind::USVString) {
        self.inner.set("domain", value);
    }
}
impl Document {
    pub fn referrer(&self) -> jsbind::USVString {
        self.inner.get("referrer").as_::<jsbind::USVString>()
    }
}
impl Document {
    pub fn cookie(&self) -> jsbind::USVString {
        self.inner.get("cookie").as_::<jsbind::USVString>()
    }

    pub fn set_cookie(&mut self, value: jsbind::USVString) {
        self.inner.set("cookie", value);
    }
}
impl Document {
    pub fn last_modified(&self) -> jsbind::DOMString {
        self.inner.get("lastModified").as_::<jsbind::DOMString>()
    }
}
impl Document {
    pub fn ready_state(&self) -> DocumentReadyState {
        self.inner.get("readyState").as_::<DocumentReadyState>()
    }
}
impl Document {
    pub fn title(&self) -> jsbind::DOMString {
        self.inner.get("title").as_::<jsbind::DOMString>()
    }

    pub fn set_title(&mut self, value: jsbind::DOMString) {
        self.inner.set("title", value);
    }
}
impl Document {
    pub fn dir(&self) -> jsbind::DOMString {
        self.inner.get("dir").as_::<jsbind::DOMString>()
    }

    pub fn set_dir(&mut self, value: jsbind::DOMString) {
        self.inner.set("dir", value);
    }
}
impl Document {
    pub fn body(&self) -> HTMLElement {
        self.inner.get("body").as_::<HTMLElement>()
    }

    pub fn set_body(&mut self, value: HTMLElement) {
        self.inner.set("body", value);
    }
}
impl Document {
    pub fn head(&self) -> HTMLHeadElement {
        self.inner.get("head").as_::<HTMLHeadElement>()
    }
}
impl Document {
    pub fn images(&self) -> HTMLCollection {
        self.inner.get("images").as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn embeds(&self) -> HTMLCollection {
        self.inner.get("embeds").as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn plugins(&self) -> HTMLCollection {
        self.inner.get("plugins").as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn links(&self) -> HTMLCollection {
        self.inner.get("links").as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn forms(&self) -> HTMLCollection {
        self.inner.get("forms").as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn scripts(&self) -> HTMLCollection {
        self.inner.get("scripts").as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn get_elements_by_name(&self, element_name: jsbind::DOMString) -> NodeList {
        self.inner
            .call("getElementsByName", &[element_name.into()])
            .as_::<NodeList>()
    }
}
impl Document {
    pub fn current_script(&self) -> jsbind::Any {
        self.inner.get("currentScript").as_::<jsbind::Any>()
    }
}
impl Document {
    pub fn open(
        &self,
        url: jsbind::USVString,
        name: jsbind::DOMString,
        features: jsbind::DOMString,
    ) -> jsbind::Any {
        self.inner
            .call("open", &[url.into(), name.into(), features.into()])
            .as_::<jsbind::Any>()
    }
}
impl Document {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn write(&self, text: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("write", &[text.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn writeln(&self, text: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("writeln", &[text.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn default_view(&self) -> jsbind::Any {
        self.inner.get("defaultView").as_::<jsbind::Any>()
    }
}
impl Document {
    pub fn has_focus(&self) -> bool {
        self.inner.call("hasFocus", &[]).as_::<bool>()
    }
}
impl Document {
    pub fn design_mode(&self) -> jsbind::DOMString {
        self.inner.get("designMode").as_::<jsbind::DOMString>()
    }

    pub fn set_design_mode(&mut self, value: jsbind::DOMString) {
        self.inner.set("designMode", value);
    }
}
impl Document {
    pub fn exec_command0(&self, command_id: jsbind::DOMString) -> bool {
        self.inner
            .call("execCommand", &[command_id.into()])
            .as_::<bool>()
    }

    pub fn exec_command1(&self, command_id: jsbind::DOMString, show_ui: bool) -> bool {
        self.inner
            .call("execCommand", &[command_id.into(), show_ui.into()])
            .as_::<bool>()
    }

    pub fn exec_command2(
        &self,
        command_id: jsbind::DOMString,
        show_ui: bool,
        value: jsbind::DOMString,
    ) -> bool {
        self.inner
            .call(
                "execCommand",
                &[command_id.into(), show_ui.into(), value.into()],
            )
            .as_::<bool>()
    }
}
impl Document {
    pub fn query_command_enabled(&self, command_id: jsbind::DOMString) -> bool {
        self.inner
            .call("queryCommandEnabled", &[command_id.into()])
            .as_::<bool>()
    }
}
impl Document {
    pub fn query_command_indeterm(&self, command_id: jsbind::DOMString) -> bool {
        self.inner
            .call("queryCommandIndeterm", &[command_id.into()])
            .as_::<bool>()
    }
}
impl Document {
    pub fn query_command_state(&self, command_id: jsbind::DOMString) -> bool {
        self.inner
            .call("queryCommandState", &[command_id.into()])
            .as_::<bool>()
    }
}
impl Document {
    pub fn query_command_supported(&self, command_id: jsbind::DOMString) -> bool {
        self.inner
            .call("queryCommandSupported", &[command_id.into()])
            .as_::<bool>()
    }
}
impl Document {
    pub fn query_command_value(&self, command_id: jsbind::DOMString) -> jsbind::DOMString {
        self.inner
            .call("queryCommandValue", &[command_id.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl Document {
    pub fn hidden(&self) -> bool {
        self.inner.get("hidden").as_::<bool>()
    }
}
impl Document {
    pub fn visibility_state(&self) -> DocumentVisibilityState {
        self.inner
            .get("visibilityState")
            .as_::<DocumentVisibilityState>()
    }
}
impl Document {
    pub fn onreadystatechange(&self) -> jsbind::Any {
        self.inner.get("onreadystatechange").as_::<jsbind::Any>()
    }

    pub fn set_onreadystatechange(&mut self, value: jsbind::Any) {
        self.inner.set("onreadystatechange", value);
    }
}
impl Document {
    pub fn onvisibilitychange(&self) -> jsbind::Any {
        self.inner.get("onvisibilitychange").as_::<jsbind::Any>()
    }

    pub fn set_onvisibilitychange(&mut self, value: jsbind::Any) {
        self.inner.set("onvisibilitychange", value);
    }
}
impl Document {
    pub fn fg_color(&self) -> jsbind::DOMString {
        self.inner.get("fgColor").as_::<jsbind::DOMString>()
    }

    pub fn set_fg_color(&mut self, value: jsbind::DOMString) {
        self.inner.set("fgColor", value);
    }
}
impl Document {
    pub fn link_color(&self) -> jsbind::DOMString {
        self.inner.get("linkColor").as_::<jsbind::DOMString>()
    }

    pub fn set_link_color(&mut self, value: jsbind::DOMString) {
        self.inner.set("linkColor", value);
    }
}
impl Document {
    pub fn vlink_color(&self) -> jsbind::DOMString {
        self.inner.get("vlinkColor").as_::<jsbind::DOMString>()
    }

    pub fn set_vlink_color(&mut self, value: jsbind::DOMString) {
        self.inner.set("vlinkColor", value);
    }
}
impl Document {
    pub fn alink_color(&self) -> jsbind::DOMString {
        self.inner.get("alinkColor").as_::<jsbind::DOMString>()
    }

    pub fn set_alink_color(&mut self, value: jsbind::DOMString) {
        self.inner.set("alinkColor", value);
    }
}
impl Document {
    pub fn bg_color(&self) -> jsbind::DOMString {
        self.inner.get("bgColor").as_::<jsbind::DOMString>()
    }

    pub fn set_bg_color(&mut self, value: jsbind::DOMString) {
        self.inner.set("bgColor", value);
    }
}
impl Document {
    pub fn anchors(&self) -> HTMLCollection {
        self.inner.get("anchors").as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn applets(&self) -> HTMLCollection {
        self.inner.get("applets").as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn clear(&self) -> jsbind::Undefined {
        self.inner.call("clear", &[]).as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn capture_events(&self) -> jsbind::Undefined {
        self.inner
            .call("captureEvents", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn release_events(&self) -> jsbind::Undefined {
        self.inner
            .call("releaseEvents", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn all(&self) -> HTMLAllCollection {
        self.inner.get("all").as_::<HTMLAllCollection>()
    }
}
impl Document {
    pub fn onfreeze(&self) -> jsbind::Any {
        self.inner.get("onfreeze").as_::<jsbind::Any>()
    }

    pub fn set_onfreeze(&mut self, value: jsbind::Any) {
        self.inner.set("onfreeze", value);
    }
}
impl Document {
    pub fn onresume(&self) -> jsbind::Any {
        self.inner.get("onresume").as_::<jsbind::Any>()
    }

    pub fn set_onresume(&mut self, value: jsbind::Any) {
        self.inner.set("onresume", value);
    }
}
impl Document {
    pub fn was_discarded(&self) -> bool {
        self.inner.get("wasDiscarded").as_::<bool>()
    }
}
impl Document {
    pub fn permissions_policy(&self) -> PermissionsPolicy {
        self.inner
            .get("permissionsPolicy")
            .as_::<PermissionsPolicy>()
    }
}
impl Document {
    pub fn picture_in_picture_enabled(&self) -> bool {
        self.inner.get("pictureInPictureEnabled").as_::<bool>()
    }
}
impl Document {
    pub fn exit_picture_in_picture(&self) -> jsbind::Promise {
        self.inner
            .call("exitPictureInPicture", &[])
            .as_::<jsbind::Promise>()
    }
}
impl Document {
    pub fn onpointerlockchange(&self) -> jsbind::Any {
        self.inner.get("onpointerlockchange").as_::<jsbind::Any>()
    }

    pub fn set_onpointerlockchange(&mut self, value: jsbind::Any) {
        self.inner.set("onpointerlockchange", value);
    }
}
impl Document {
    pub fn onpointerlockerror(&self) -> jsbind::Any {
        self.inner.get("onpointerlockerror").as_::<jsbind::Any>()
    }

    pub fn set_onpointerlockerror(&mut self, value: jsbind::Any) {
        self.inner.set("onpointerlockerror", value);
    }
}
impl Document {
    pub fn exit_pointer_lock(&self) -> jsbind::Undefined {
        self.inner
            .call("exitPointerLock", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn prerendering(&self) -> bool {
        self.inner.get("prerendering").as_::<bool>()
    }
}
impl Document {
    pub fn onprerenderingchange(&self) -> jsbind::Any {
        self.inner.get("onprerenderingchange").as_::<jsbind::Any>()
    }

    pub fn set_onprerenderingchange(&mut self, value: jsbind::Any) {
        self.inner.set("onprerenderingchange", value);
    }
}
impl Document {
    pub fn request_storage_access_for(
        &self,
        requested_origin: jsbind::USVString,
    ) -> jsbind::Promise {
        self.inner
            .call("requestStorageAccessFor", &[requested_origin.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Document {
    pub fn has_unpartitioned_cookie_access(&self) -> jsbind::Promise {
        self.inner
            .call("hasUnpartitionedCookieAccess", &[])
            .as_::<jsbind::Promise>()
    }
}
impl Document {
    pub fn fragment_directive(&self) -> FragmentDirective {
        self.inner
            .get("fragmentDirective")
            .as_::<FragmentDirective>()
    }
}
impl Document {
    pub fn get_selection(&self) -> Selection {
        self.inner.call("getSelection", &[]).as_::<Selection>()
    }
}
impl Document {
    pub fn has_storage_access(&self) -> jsbind::Promise {
        self.inner
            .call("hasStorageAccess", &[])
            .as_::<jsbind::Promise>()
    }
}
impl Document {
    pub fn request_storage_access(&self) -> jsbind::Promise {
        self.inner
            .call("requestStorageAccess", &[])
            .as_::<jsbind::Promise>()
    }
}
impl Document {
    pub fn has_private_token(&self, issuer: jsbind::USVString) -> jsbind::Promise {
        self.inner
            .call("hasPrivateToken", &[issuer.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Document {
    pub fn has_redemption_record(&self, issuer: jsbind::USVString) -> jsbind::Promise {
        self.inner
            .call("hasRedemptionRecord", &[issuer.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Document {
    pub fn timeline(&self) -> DocumentTimeline {
        self.inner.get("timeline").as_::<DocumentTimeline>()
    }
}
impl Document {
    pub fn fonts(&self) -> FontFaceSet {
        self.inner.get("fonts").as_::<FontFaceSet>()
    }
}
impl Document {
    pub fn get_box_quads0(&self) -> jsbind::Sequence<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[])
            .as_::<jsbind::Sequence<DOMQuad>>()
    }

    pub fn get_box_quads1(&self, options: BoxQuadOptions) -> jsbind::Sequence<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[options.into()])
            .as_::<jsbind::Sequence<DOMQuad>>()
    }
}
impl Document {
    pub fn convert_quad_from_node0(&self, quad: DOMQuadInit, from: jsbind::Any) -> DOMQuad {
        self.inner
            .call("convertQuadFromNode", &[quad.into(), from.into()])
            .as_::<DOMQuad>()
    }

    pub fn convert_quad_from_node1(
        &self,
        quad: DOMQuadInit,
        from: jsbind::Any,
        options: ConvertCoordinateOptions,
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
    pub fn convert_rect_from_node0(&self, rect: DOMRectReadOnly, from: jsbind::Any) -> DOMQuad {
        self.inner
            .call("convertRectFromNode", &[rect.into(), from.into()])
            .as_::<DOMQuad>()
    }

    pub fn convert_rect_from_node1(
        &self,
        rect: DOMRectReadOnly,
        from: jsbind::Any,
        options: ConvertCoordinateOptions,
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
    pub fn convert_point_from_node0(&self, point: DOMPointInit, from: jsbind::Any) -> DOMPoint {
        self.inner
            .call("convertPointFromNode", &[point.into(), from.into()])
            .as_::<DOMPoint>()
    }

    pub fn convert_point_from_node1(
        &self,
        point: DOMPointInit,
        from: jsbind::Any,
        options: ConvertCoordinateOptions,
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
    pub fn get_element_by_id(&self, element_id: jsbind::DOMString) -> Element {
        self.inner
            .call("getElementById", &[element_id.into()])
            .as_::<Element>()
    }
}
impl Document {
    pub fn get_animations(&self) -> jsbind::Sequence<Animation> {
        self.inner
            .call("getAnimations", &[])
            .as_::<jsbind::Sequence<Animation>>()
    }
}
impl Document {
    pub fn children(&self) -> HTMLCollection {
        self.inner.get("children").as_::<HTMLCollection>()
    }
}
impl Document {
    pub fn first_element_child(&self) -> Element {
        self.inner.get("firstElementChild").as_::<Element>()
    }
}
impl Document {
    pub fn last_element_child(&self) -> Element {
        self.inner.get("lastElementChild").as_::<Element>()
    }
}
impl Document {
    pub fn child_element_count(&self) -> u32 {
        self.inner.get("childElementCount").as_::<u32>()
    }
}
impl Document {
    pub fn prepend(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("prepend", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn append(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("append", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn replace_children(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("replaceChildren", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn move_before(&self, node: Node, child: Node) -> jsbind::Undefined {
        self.inner
            .call("moveBefore", &[node.into(), child.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Document {
    pub fn query_selector(&self, selectors: jsbind::DOMString) -> Element {
        self.inner
            .call("querySelector", &[selectors.into()])
            .as_::<Element>()
    }
}
impl Document {
    pub fn query_selector_all(&self, selectors: jsbind::DOMString) -> NodeList {
        self.inner
            .call("querySelectorAll", &[selectors.into()])
            .as_::<NodeList>()
    }
}
impl Document {
    pub fn create_expression0(&self, expression: jsbind::DOMString) -> XPathExpression {
        self.inner
            .call("createExpression", &[expression.into()])
            .as_::<XPathExpression>()
    }

    pub fn create_expression1(
        &self,
        expression: jsbind::DOMString,
        resolver: jsbind::Function,
    ) -> XPathExpression {
        self.inner
            .call("createExpression", &[expression.into(), resolver.into()])
            .as_::<XPathExpression>()
    }
}
impl Document {
    pub fn create_ns_resolver(&self, node_resolver: Node) -> Node {
        self.inner
            .call("createNSResolver", &[node_resolver.into()])
            .as_::<Node>()
    }
}
impl Document {
    pub fn evaluate0(&self, expression: jsbind::DOMString, context_node: Node) -> XPathResult {
        self.inner
            .call("evaluate", &[expression.into(), context_node.into()])
            .as_::<XPathResult>()
    }

    pub fn evaluate1(
        &self,
        expression: jsbind::DOMString,
        context_node: Node,
        resolver: jsbind::Function,
    ) -> XPathResult {
        self.inner
            .call(
                "evaluate",
                &[expression.into(), context_node.into(), resolver.into()],
            )
            .as_::<XPathResult>()
    }

    pub fn evaluate2(
        &self,
        expression: jsbind::DOMString,
        context_node: Node,
        resolver: jsbind::Function,
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

    pub fn evaluate3(
        &self,
        expression: jsbind::DOMString,
        context_node: Node,
        resolver: jsbind::Function,
        type_: u16,
        result: XPathResult,
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
    pub fn onbeforexrselect(&self) -> jsbind::Any {
        self.inner.get("onbeforexrselect").as_::<jsbind::Any>()
    }

    pub fn set_onbeforexrselect(&mut self, value: jsbind::Any) {
        self.inner.set("onbeforexrselect", value);
    }
}
