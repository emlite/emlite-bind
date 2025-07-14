use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShadowRootInit {
    inner: emlite::Val,
}
impl FromVal for ShadowRootInit {
    fn from_val(v: &emlite::Val) -> Self {
        ShadowRootInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ShadowRootInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ShadowRootInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ShadowRootInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ShadowRootInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ShadowRootInit> for emlite::Val {
    fn from(s: ShadowRootInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ShadowRootInit {
    pub fn mode(&self) -> ShadowRootMode {
        self.inner.get("mode").as_::<ShadowRootMode>()
    }

    pub fn set_mode(&mut self, value: ShadowRootMode) {
        self.inner.set("mode", value);
    }
}
impl ShadowRootInit {
    pub fn delegates_focus(&self) -> bool {
        self.inner.get("delegatesFocus").as_::<bool>()
    }

    pub fn set_delegates_focus(&mut self, value: bool) {
        self.inner.set("delegatesFocus", value);
    }
}
impl ShadowRootInit {
    pub fn slot_assignment(&self) -> SlotAssignmentMode {
        self.inner.get("slotAssignment").as_::<SlotAssignmentMode>()
    }

    pub fn set_slot_assignment(&mut self, value: SlotAssignmentMode) {
        self.inner.set("slotAssignment", value);
    }
}
impl ShadowRootInit {
    pub fn clonable(&self) -> bool {
        self.inner.get("clonable").as_::<bool>()
    }

    pub fn set_clonable(&mut self, value: bool) {
        self.inner.set("clonable", value);
    }
}
impl ShadowRootInit {
    pub fn serializable(&self) -> bool {
        self.inner.get("serializable").as_::<bool>()
    }

    pub fn set_serializable(&mut self, value: bool) {
        self.inner.set("serializable", value);
    }
}
impl ShadowRootInit {
    pub fn custom_element_registry(&self) -> CustomElementRegistry {
        self.inner
            .get("customElementRegistry")
            .as_::<CustomElementRegistry>()
    }

    pub fn set_custom_element_registry(&mut self, value: CustomElementRegistry) {
        self.inner.set("customElementRegistry", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FocusableAreasOption {
    inner: emlite::Val,
}
impl FromVal for FocusableAreasOption {
    fn from_val(v: &emlite::Val) -> Self {
        FocusableAreasOption { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FocusableAreasOption {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FocusableAreasOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FocusableAreasOption {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FocusableAreasOption {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FocusableAreasOption> for emlite::Val {
    fn from(s: FocusableAreasOption) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FocusableAreasOption {
    pub fn mode(&self) -> FocusableAreaSearchMode {
        self.inner.get("mode").as_::<FocusableAreaSearchMode>()
    }

    pub fn set_mode(&mut self, value: FocusableAreaSearchMode) {
        self.inner.set("mode", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpatialNavigationSearchOptions {
    inner: emlite::Val,
}
impl FromVal for SpatialNavigationSearchOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SpatialNavigationSearchOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SpatialNavigationSearchOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpatialNavigationSearchOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SpatialNavigationSearchOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SpatialNavigationSearchOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SpatialNavigationSearchOptions> for emlite::Val {
    fn from(s: SpatialNavigationSearchOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpatialNavigationSearchOptions {
    pub fn candidates(&self) -> jsbind::Sequence<Node> {
        self.inner.get("candidates").as_::<jsbind::Sequence<Node>>()
    }

    pub fn set_candidates(&mut self, value: jsbind::Sequence<Node>) {
        self.inner.set("candidates", value);
    }
}
impl SpatialNavigationSearchOptions {
    pub fn container(&self) -> Node {
        self.inner.get("container").as_::<Node>()
    }

    pub fn set_container(&mut self, value: Node) {
        self.inner.set("container", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CheckVisibilityOptions {
    inner: emlite::Val,
}
impl FromVal for CheckVisibilityOptions {
    fn from_val(v: &emlite::Val) -> Self {
        CheckVisibilityOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CheckVisibilityOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CheckVisibilityOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CheckVisibilityOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CheckVisibilityOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CheckVisibilityOptions> for emlite::Val {
    fn from(s: CheckVisibilityOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CheckVisibilityOptions {
    pub fn check_opacity(&self) -> bool {
        self.inner.get("checkOpacity").as_::<bool>()
    }

    pub fn set_check_opacity(&mut self, value: bool) {
        self.inner.set("checkOpacity", value);
    }
}
impl CheckVisibilityOptions {
    pub fn check_visibility_css(&self) -> bool {
        self.inner.get("checkVisibilityCSS").as_::<bool>()
    }

    pub fn set_check_visibility_css(&mut self, value: bool) {
        self.inner.set("checkVisibilityCSS", value);
    }
}
impl CheckVisibilityOptions {
    pub fn content_visibility_auto(&self) -> bool {
        self.inner.get("contentVisibilityAuto").as_::<bool>()
    }

    pub fn set_content_visibility_auto(&mut self, value: bool) {
        self.inner.set("contentVisibilityAuto", value);
    }
}
impl CheckVisibilityOptions {
    pub fn opacity_property(&self) -> bool {
        self.inner.get("opacityProperty").as_::<bool>()
    }

    pub fn set_opacity_property(&mut self, value: bool) {
        self.inner.set("opacityProperty", value);
    }
}
impl CheckVisibilityOptions {
    pub fn visibility_property(&self) -> bool {
        self.inner.get("visibilityProperty").as_::<bool>()
    }

    pub fn set_visibility_property(&mut self, value: bool) {
        self.inner.set("visibilityProperty", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FullscreenOptions {
    inner: emlite::Val,
}
impl FromVal for FullscreenOptions {
    fn from_val(v: &emlite::Val) -> Self {
        FullscreenOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FullscreenOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FullscreenOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FullscreenOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FullscreenOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FullscreenOptions> for emlite::Val {
    fn from(s: FullscreenOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FullscreenOptions {
    pub fn screen(&self) -> ScreenDetailed {
        self.inner.get("screen").as_::<ScreenDetailed>()
    }

    pub fn set_screen(&mut self, value: ScreenDetailed) {
        self.inner.set("screen", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetHTMLOptions {
    inner: emlite::Val,
}
impl FromVal for GetHTMLOptions {
    fn from_val(v: &emlite::Val) -> Self {
        GetHTMLOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GetHTMLOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GetHTMLOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GetHTMLOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GetHTMLOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GetHTMLOptions> for emlite::Val {
    fn from(s: GetHTMLOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GetHTMLOptions {
    pub fn serializable_shadow_roots(&self) -> bool {
        self.inner.get("serializableShadowRoots").as_::<bool>()
    }

    pub fn set_serializable_shadow_roots(&mut self, value: bool) {
        self.inner.set("serializableShadowRoots", value);
    }
}
impl GetHTMLOptions {
    pub fn shadow_roots(&self) -> jsbind::Sequence<ShadowRoot> {
        self.inner
            .get("shadowRoots")
            .as_::<jsbind::Sequence<ShadowRoot>>()
    }

    pub fn set_shadow_roots(&mut self, value: jsbind::Sequence<ShadowRoot>) {
        self.inner.set("shadowRoots", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PointerLockOptions {
    inner: emlite::Val,
}
impl FromVal for PointerLockOptions {
    fn from_val(v: &emlite::Val) -> Self {
        PointerLockOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PointerLockOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PointerLockOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PointerLockOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PointerLockOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PointerLockOptions> for emlite::Val {
    fn from(s: PointerLockOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PointerLockOptions {
    pub fn unadjusted_movement(&self) -> bool {
        self.inner.get("unadjustedMovement").as_::<bool>()
    }

    pub fn set_unadjusted_movement(&mut self, value: bool) {
        self.inner.set("unadjustedMovement", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetAnimationsOptions {
    inner: emlite::Val,
}
impl FromVal for GetAnimationsOptions {
    fn from_val(v: &emlite::Val) -> Self {
        GetAnimationsOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GetAnimationsOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GetAnimationsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GetAnimationsOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GetAnimationsOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GetAnimationsOptions> for emlite::Val {
    fn from(s: GetAnimationsOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GetAnimationsOptions {
    pub fn subtree(&self) -> bool {
        self.inner.get("subtree").as_::<bool>()
    }

    pub fn set_subtree(&mut self, value: bool) {
        self.inner.set("subtree", value);
    }
}
impl GetAnimationsOptions {
    pub fn pseudo_element(&self) -> jsbind::CSSOMString {
        self.inner.get("pseudoElement").as_::<jsbind::CSSOMString>()
    }

    pub fn set_pseudo_element(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("pseudoElement", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Element {
    inner: Node,
}
impl FromVal for Element {
    fn from_val(v: &emlite::Val) -> Self {
        Element {
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
impl core::ops::Deref for Element {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Element {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Element {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Element {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Element> for emlite::Val {
    fn from(s: Element) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Element);

impl Element {
    pub fn namespace_uri(&self) -> jsbind::DOMString {
        self.inner.get("namespaceURI").as_::<jsbind::DOMString>()
    }
}
impl Element {
    pub fn prefix(&self) -> jsbind::DOMString {
        self.inner.get("prefix").as_::<jsbind::DOMString>()
    }
}
impl Element {
    pub fn local_name(&self) -> jsbind::DOMString {
        self.inner.get("localName").as_::<jsbind::DOMString>()
    }
}
impl Element {
    pub fn tag_name(&self) -> jsbind::DOMString {
        self.inner.get("tagName").as_::<jsbind::DOMString>()
    }
}
impl Element {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }

    pub fn set_id(&mut self, value: jsbind::DOMString) {
        self.inner.set("id", value);
    }
}
impl Element {
    pub fn class_name(&self) -> jsbind::DOMString {
        self.inner.get("className").as_::<jsbind::DOMString>()
    }

    pub fn set_class_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("className", value);
    }
}
impl Element {
    pub fn class_list(&self) -> DOMTokenList {
        self.inner.get("classList").as_::<DOMTokenList>()
    }
}
impl Element {
    pub fn slot(&self) -> jsbind::DOMString {
        self.inner.get("slot").as_::<jsbind::DOMString>()
    }

    pub fn set_slot(&mut self, value: jsbind::DOMString) {
        self.inner.set("slot", value);
    }
}
impl Element {
    pub fn has_attributes(&self) -> bool {
        self.inner.call("hasAttributes", &[]).as_::<bool>()
    }
}
impl Element {
    pub fn attributes(&self) -> NamedNodeMap {
        self.inner.get("attributes").as_::<NamedNodeMap>()
    }
}
impl Element {
    pub fn get_attribute_names(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .call("getAttributeNames", &[])
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }
}
impl Element {
    pub fn get_attribute(&self, qualified_name: jsbind::DOMString) -> jsbind::DOMString {
        self.inner
            .call("getAttribute", &[qualified_name.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl Element {
    pub fn get_attribute_ns(
        &self,
        namespace: jsbind::DOMString,
        local_name: jsbind::DOMString,
    ) -> jsbind::DOMString {
        self.inner
            .call("getAttributeNS", &[namespace.into(), local_name.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl Element {
    pub fn set_attribute(
        &self,
        qualified_name: jsbind::DOMString,
        value: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call("setAttribute", &[qualified_name.into(), value.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn set_attribute_ns(
        &self,
        namespace: jsbind::DOMString,
        qualified_name: jsbind::DOMString,
        value: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "setAttributeNS",
                &[namespace.into(), qualified_name.into(), value.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn remove_attribute(&self, qualified_name: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("removeAttribute", &[qualified_name.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn remove_attribute_ns(
        &self,
        namespace: jsbind::DOMString,
        local_name: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call("removeAttributeNS", &[namespace.into(), local_name.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn toggle_attribute0(&self, qualified_name: jsbind::DOMString) -> bool {
        self.inner
            .call("toggleAttribute", &[qualified_name.into()])
            .as_::<bool>()
    }

    pub fn toggle_attribute1(&self, qualified_name: jsbind::DOMString, force: bool) -> bool {
        self.inner
            .call("toggleAttribute", &[qualified_name.into(), force.into()])
            .as_::<bool>()
    }
}
impl Element {
    pub fn has_attribute(&self, qualified_name: jsbind::DOMString) -> bool {
        self.inner
            .call("hasAttribute", &[qualified_name.into()])
            .as_::<bool>()
    }
}
impl Element {
    pub fn has_attribute_ns(
        &self,
        namespace: jsbind::DOMString,
        local_name: jsbind::DOMString,
    ) -> bool {
        self.inner
            .call("hasAttributeNS", &[namespace.into(), local_name.into()])
            .as_::<bool>()
    }
}
impl Element {
    pub fn get_attribute_node(&self, qualified_name: jsbind::DOMString) -> Attr {
        self.inner
            .call("getAttributeNode", &[qualified_name.into()])
            .as_::<Attr>()
    }
}
impl Element {
    pub fn get_attribute_node_ns(
        &self,
        namespace: jsbind::DOMString,
        local_name: jsbind::DOMString,
    ) -> Attr {
        self.inner
            .call("getAttributeNodeNS", &[namespace.into(), local_name.into()])
            .as_::<Attr>()
    }
}
impl Element {
    pub fn set_attribute_node(&self, attr: Attr) -> Attr {
        self.inner
            .call("setAttributeNode", &[attr.into()])
            .as_::<Attr>()
    }
}
impl Element {
    pub fn set_attribute_node_ns(&self, attr: Attr) -> Attr {
        self.inner
            .call("setAttributeNodeNS", &[attr.into()])
            .as_::<Attr>()
    }
}
impl Element {
    pub fn remove_attribute_node(&self, attr: Attr) -> Attr {
        self.inner
            .call("removeAttributeNode", &[attr.into()])
            .as_::<Attr>()
    }
}
impl Element {
    pub fn attach_shadow(&self, init: ShadowRootInit) -> ShadowRoot {
        self.inner
            .call("attachShadow", &[init.into()])
            .as_::<ShadowRoot>()
    }
}
impl Element {
    pub fn shadow_root(&self) -> ShadowRoot {
        self.inner.get("shadowRoot").as_::<ShadowRoot>()
    }
}
impl Element {
    pub fn custom_element_registry(&self) -> CustomElementRegistry {
        self.inner
            .get("customElementRegistry")
            .as_::<CustomElementRegistry>()
    }
}
impl Element {
    pub fn closest(&self, selectors: jsbind::DOMString) -> Element {
        self.inner
            .call("closest", &[selectors.into()])
            .as_::<Element>()
    }
}
impl Element {
    pub fn matches(&self, selectors: jsbind::DOMString) -> bool {
        self.inner
            .call("matches", &[selectors.into()])
            .as_::<bool>()
    }
}
impl Element {
    pub fn webkit_matches_selector(&self, selectors: jsbind::DOMString) -> bool {
        self.inner
            .call("webkitMatchesSelector", &[selectors.into()])
            .as_::<bool>()
    }
}
impl Element {
    pub fn get_elements_by_tag_name(&self, qualified_name: jsbind::DOMString) -> HTMLCollection {
        self.inner
            .call("getElementsByTagName", &[qualified_name.into()])
            .as_::<HTMLCollection>()
    }
}
impl Element {
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
impl Element {
    pub fn get_elements_by_class_name(&self, class_names: jsbind::DOMString) -> HTMLCollection {
        self.inner
            .call("getElementsByClassName", &[class_names.into()])
            .as_::<HTMLCollection>()
    }
}
impl Element {
    pub fn insert_adjacent_element(&self, where_: jsbind::DOMString, element: Element) -> Element {
        self.inner
            .call("insertAdjacentElement", &[where_.into(), element.into()])
            .as_::<Element>()
    }
}
impl Element {
    pub fn insert_adjacent_text(
        &self,
        where_: jsbind::DOMString,
        data: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call("insertAdjacentText", &[where_.into(), data.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn get_spatial_navigation_container(&self) -> Node {
        self.inner
            .call("getSpatialNavigationContainer", &[])
            .as_::<Node>()
    }
}
impl Element {
    pub fn focusable_areas0(&self) -> jsbind::Sequence<Node> {
        self.inner
            .call("focusableAreas", &[])
            .as_::<jsbind::Sequence<Node>>()
    }

    pub fn focusable_areas1(&self, option: FocusableAreasOption) -> jsbind::Sequence<Node> {
        self.inner
            .call("focusableAreas", &[option.into()])
            .as_::<jsbind::Sequence<Node>>()
    }
}
impl Element {
    pub fn spatial_navigation_search0(&self, dir: SpatialNavigationDirection) -> Node {
        self.inner
            .call("spatialNavigationSearch", &[dir.into()])
            .as_::<Node>()
    }

    pub fn spatial_navigation_search1(
        &self,
        dir: SpatialNavigationDirection,
        options: SpatialNavigationSearchOptions,
    ) -> Node {
        self.inner
            .call("spatialNavigationSearch", &[dir.into(), options.into()])
            .as_::<Node>()
    }
}
impl Element {
    pub fn pseudo(&self, type_: jsbind::CSSOMString) -> CSSPseudoElement {
        self.inner
            .call("pseudo", &[type_.into()])
            .as_::<CSSPseudoElement>()
    }
}
impl Element {
    pub fn part(&self) -> DOMTokenList {
        self.inner.get("part").as_::<DOMTokenList>()
    }
}
impl Element {
    pub fn computed_style_map(&self) -> StylePropertyMapReadOnly {
        self.inner
            .call("computedStyleMap", &[])
            .as_::<StylePropertyMapReadOnly>()
    }
}
impl Element {
    pub fn get_client_rects(&self) -> DOMRectList {
        self.inner.call("getClientRects", &[]).as_::<DOMRectList>()
    }
}
impl Element {
    pub fn get_bounding_client_rect(&self) -> DOMRect {
        self.inner
            .call("getBoundingClientRect", &[])
            .as_::<DOMRect>()
    }
}
impl Element {
    pub fn check_visibility0(&self) -> bool {
        self.inner.call("checkVisibility", &[]).as_::<bool>()
    }

    pub fn check_visibility1(&self, options: CheckVisibilityOptions) -> bool {
        self.inner
            .call("checkVisibility", &[options.into()])
            .as_::<bool>()
    }
}
impl Element {
    pub fn scroll_into_view0(&self) -> jsbind::Undefined {
        self.inner
            .call("scrollIntoView", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn scroll_into_view1(&self, arg: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("scrollIntoView", &[arg.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn scroll(&self, x: f64, y: f64) -> jsbind::Undefined {
        self.inner
            .call("scroll", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn scroll_to(&self, x: f64, y: f64) -> jsbind::Undefined {
        self.inner
            .call("scrollTo", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn scroll_by(&self, x: f64, y: f64) -> jsbind::Undefined {
        self.inner
            .call("scrollBy", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn scroll_top(&self) -> f64 {
        self.inner.get("scrollTop").as_::<f64>()
    }

    pub fn set_scroll_top(&mut self, value: f64) {
        self.inner.set("scrollTop", value);
    }
}
impl Element {
    pub fn scroll_left(&self) -> f64 {
        self.inner.get("scrollLeft").as_::<f64>()
    }

    pub fn set_scroll_left(&mut self, value: f64) {
        self.inner.set("scrollLeft", value);
    }
}
impl Element {
    pub fn scroll_width(&self) -> i32 {
        self.inner.get("scrollWidth").as_::<i32>()
    }
}
impl Element {
    pub fn scroll_height(&self) -> i32 {
        self.inner.get("scrollHeight").as_::<i32>()
    }
}
impl Element {
    pub fn client_top(&self) -> i32 {
        self.inner.get("clientTop").as_::<i32>()
    }
}
impl Element {
    pub fn client_left(&self) -> i32 {
        self.inner.get("clientLeft").as_::<i32>()
    }
}
impl Element {
    pub fn client_width(&self) -> i32 {
        self.inner.get("clientWidth").as_::<i32>()
    }
}
impl Element {
    pub fn client_height(&self) -> i32 {
        self.inner.get("clientHeight").as_::<i32>()
    }
}
impl Element {
    pub fn current_css_zoom(&self) -> f64 {
        self.inner.get("currentCSSZoom").as_::<f64>()
    }
}
impl Element {
    pub fn element_timing(&self) -> jsbind::DOMString {
        self.inner.get("elementTiming").as_::<jsbind::DOMString>()
    }

    pub fn set_element_timing(&mut self, value: jsbind::DOMString) {
        self.inner.set("elementTiming", value);
    }
}
impl Element {
    pub fn request_fullscreen0(&self) -> jsbind::Promise {
        self.inner
            .call("requestFullscreen", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn request_fullscreen1(&self, options: FullscreenOptions) -> jsbind::Promise {
        self.inner
            .call("requestFullscreen", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Element {
    pub fn onfullscreenchange(&self) -> jsbind::Any {
        self.inner.get("onfullscreenchange").as_::<jsbind::Any>()
    }

    pub fn set_onfullscreenchange(&mut self, value: jsbind::Any) {
        self.inner.set("onfullscreenchange", value);
    }
}
impl Element {
    pub fn onfullscreenerror(&self) -> jsbind::Any {
        self.inner.get("onfullscreenerror").as_::<jsbind::Any>()
    }

    pub fn set_onfullscreenerror(&mut self, value: jsbind::Any) {
        self.inner.set("onfullscreenerror", value);
    }
}
impl Element {
    pub fn set_html_unsafe(&self, html: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("setHTMLUnsafe", &[html.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn get_html0(&self) -> jsbind::DOMString {
        self.inner.call("getHTML", &[]).as_::<jsbind::DOMString>()
    }

    pub fn get_html1(&self, options: GetHTMLOptions) -> jsbind::DOMString {
        self.inner
            .call("getHTML", &[options.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl Element {
    pub fn inner_html(&self) -> jsbind::Any {
        self.inner.get("innerHTML").as_::<jsbind::Any>()
    }

    pub fn set_inner_html(&mut self, value: jsbind::Any) {
        self.inner.set("innerHTML", value);
    }
}
impl Element {
    pub fn outer_html(&self) -> jsbind::Any {
        self.inner.get("outerHTML").as_::<jsbind::Any>()
    }

    pub fn set_outer_html(&mut self, value: jsbind::Any) {
        self.inner.set("outerHTML", value);
    }
}
impl Element {
    pub fn insert_adjacent_html(
        &self,
        position: jsbind::DOMString,
        string: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("insertAdjacentHTML", &[position.into(), string.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn set_pointer_capture(&self, pointer_id: i32) -> jsbind::Undefined {
        self.inner
            .call("setPointerCapture", &[pointer_id.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn release_pointer_capture(&self, pointer_id: i32) -> jsbind::Undefined {
        self.inner
            .call("releasePointerCapture", &[pointer_id.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn has_pointer_capture(&self, pointer_id: i32) -> bool {
        self.inner
            .call("hasPointerCapture", &[pointer_id.into()])
            .as_::<bool>()
    }
}
impl Element {
    pub fn request_pointer_lock0(&self) -> jsbind::Promise {
        self.inner
            .call("requestPointerLock", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn request_pointer_lock1(&self, options: PointerLockOptions) -> jsbind::Promise {
        self.inner
            .call("requestPointerLock", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Element {
    pub fn region_overset(&self) -> jsbind::CSSOMString {
        self.inner.get("regionOverset").as_::<jsbind::CSSOMString>()
    }
}
impl Element {
    pub fn get_region_flow_ranges(&self) -> jsbind::Sequence<Range> {
        self.inner
            .call("getRegionFlowRanges", &[])
            .as_::<jsbind::Sequence<Range>>()
    }
}
impl Element {
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
impl Element {
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
impl Element {
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
impl Element {
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
impl Element {
    pub fn children(&self) -> HTMLCollection {
        self.inner.get("children").as_::<HTMLCollection>()
    }
}
impl Element {
    pub fn first_element_child(&self) -> Element {
        self.inner.get("firstElementChild").as_::<Element>()
    }
}
impl Element {
    pub fn last_element_child(&self) -> Element {
        self.inner.get("lastElementChild").as_::<Element>()
    }
}
impl Element {
    pub fn child_element_count(&self) -> u32 {
        self.inner.get("childElementCount").as_::<u32>()
    }
}
impl Element {
    pub fn prepend(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("prepend", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn append(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("append", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn replace_children(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("replaceChildren", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn move_before(&self, node: Node, child: Node) -> jsbind::Undefined {
        self.inner
            .call("moveBefore", &[node.into(), child.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn query_selector(&self, selectors: jsbind::DOMString) -> Element {
        self.inner
            .call("querySelector", &[selectors.into()])
            .as_::<Element>()
    }
}
impl Element {
    pub fn query_selector_all(&self, selectors: jsbind::DOMString) -> NodeList {
        self.inner
            .call("querySelectorAll", &[selectors.into()])
            .as_::<NodeList>()
    }
}
impl Element {
    pub fn previous_element_sibling(&self) -> Element {
        self.inner.get("previousElementSibling").as_::<Element>()
    }
}
impl Element {
    pub fn next_element_sibling(&self) -> Element {
        self.inner.get("nextElementSibling").as_::<Element>()
    }
}
impl Element {
    pub fn before(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("before", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn after(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("after", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn replace_with(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("replaceWith", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn remove(&self) -> jsbind::Undefined {
        self.inner.call("remove", &[]).as_::<jsbind::Undefined>()
    }
}
impl Element {
    pub fn assigned_slot(&self) -> HTMLSlotElement {
        self.inner.get("assignedSlot").as_::<HTMLSlotElement>()
    }
}
impl Element {
    pub fn role(&self) -> jsbind::DOMString {
        self.inner.get("role").as_::<jsbind::DOMString>()
    }

    pub fn set_role(&mut self, value: jsbind::DOMString) {
        self.inner.set("role", value);
    }
}
impl Element {
    pub fn aria_active_descendant_element(&self) -> Element {
        self.inner
            .get("ariaActiveDescendantElement")
            .as_::<Element>()
    }

    pub fn set_aria_active_descendant_element(&mut self, value: Element) {
        self.inner.set("ariaActiveDescendantElement", value);
    }
}
impl Element {
    pub fn aria_atomic(&self) -> jsbind::DOMString {
        self.inner.get("ariaAtomic").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_atomic(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaAtomic", value);
    }
}
impl Element {
    pub fn aria_auto_complete(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaAutoComplete")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_auto_complete(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaAutoComplete", value);
    }
}
impl Element {
    pub fn aria_braille_label(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaBrailleLabel")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_braille_label(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaBrailleLabel", value);
    }
}
impl Element {
    pub fn aria_braille_role_description(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaBrailleRoleDescription")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_braille_role_description(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaBrailleRoleDescription", value);
    }
}
impl Element {
    pub fn aria_busy(&self) -> jsbind::DOMString {
        self.inner.get("ariaBusy").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_busy(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaBusy", value);
    }
}
impl Element {
    pub fn aria_checked(&self) -> jsbind::DOMString {
        self.inner.get("ariaChecked").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_checked(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaChecked", value);
    }
}
impl Element {
    pub fn aria_col_count(&self) -> jsbind::DOMString {
        self.inner.get("ariaColCount").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_col_count(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaColCount", value);
    }
}
impl Element {
    pub fn aria_col_index(&self) -> jsbind::DOMString {
        self.inner.get("ariaColIndex").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_col_index(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaColIndex", value);
    }
}
impl Element {
    pub fn aria_col_index_text(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaColIndexText")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_col_index_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaColIndexText", value);
    }
}
impl Element {
    pub fn aria_col_span(&self) -> jsbind::DOMString {
        self.inner.get("ariaColSpan").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_col_span(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaColSpan", value);
    }
}
impl Element {
    pub fn aria_controls_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaControlsElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_controls_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaControlsElements", value);
    }
}
impl Element {
    pub fn aria_current(&self) -> jsbind::DOMString {
        self.inner.get("ariaCurrent").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_current(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaCurrent", value);
    }
}
impl Element {
    pub fn aria_described_by_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaDescribedByElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_described_by_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaDescribedByElements", value);
    }
}
impl Element {
    pub fn aria_description(&self) -> jsbind::DOMString {
        self.inner.get("ariaDescription").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_description(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaDescription", value);
    }
}
impl Element {
    pub fn aria_details_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaDetailsElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_details_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaDetailsElements", value);
    }
}
impl Element {
    pub fn aria_disabled(&self) -> jsbind::DOMString {
        self.inner.get("ariaDisabled").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_disabled(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaDisabled", value);
    }
}
impl Element {
    pub fn aria_error_message_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaErrorMessageElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_error_message_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaErrorMessageElements", value);
    }
}
impl Element {
    pub fn aria_expanded(&self) -> jsbind::DOMString {
        self.inner.get("ariaExpanded").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_expanded(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaExpanded", value);
    }
}
impl Element {
    pub fn aria_flow_to_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaFlowToElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_flow_to_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaFlowToElements", value);
    }
}
impl Element {
    pub fn aria_has_popup(&self) -> jsbind::DOMString {
        self.inner.get("ariaHasPopup").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_has_popup(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaHasPopup", value);
    }
}
impl Element {
    pub fn aria_hidden(&self) -> jsbind::DOMString {
        self.inner.get("ariaHidden").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_hidden(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaHidden", value);
    }
}
impl Element {
    pub fn aria_invalid(&self) -> jsbind::DOMString {
        self.inner.get("ariaInvalid").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_invalid(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaInvalid", value);
    }
}
impl Element {
    pub fn aria_key_shortcuts(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaKeyShortcuts")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_key_shortcuts(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaKeyShortcuts", value);
    }
}
impl Element {
    pub fn aria_label(&self) -> jsbind::DOMString {
        self.inner.get("ariaLabel").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_label(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaLabel", value);
    }
}
impl Element {
    pub fn aria_labelled_by_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaLabelledByElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_labelled_by_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaLabelledByElements", value);
    }
}
impl Element {
    pub fn aria_level(&self) -> jsbind::DOMString {
        self.inner.get("ariaLevel").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_level(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaLevel", value);
    }
}
impl Element {
    pub fn aria_live(&self) -> jsbind::DOMString {
        self.inner.get("ariaLive").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_live(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaLive", value);
    }
}
impl Element {
    pub fn aria_modal(&self) -> jsbind::DOMString {
        self.inner.get("ariaModal").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_modal(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaModal", value);
    }
}
impl Element {
    pub fn aria_multi_line(&self) -> jsbind::DOMString {
        self.inner.get("ariaMultiLine").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_multi_line(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaMultiLine", value);
    }
}
impl Element {
    pub fn aria_multi_selectable(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaMultiSelectable")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_multi_selectable(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaMultiSelectable", value);
    }
}
impl Element {
    pub fn aria_orientation(&self) -> jsbind::DOMString {
        self.inner.get("ariaOrientation").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_orientation(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaOrientation", value);
    }
}
impl Element {
    pub fn aria_owns_elements(&self) -> jsbind::FrozenArray<Element> {
        self.inner
            .get("ariaOwnsElements")
            .as_::<jsbind::FrozenArray<Element>>()
    }

    pub fn set_aria_owns_elements(&mut self, value: jsbind::FrozenArray<Element>) {
        self.inner.set("ariaOwnsElements", value);
    }
}
impl Element {
    pub fn aria_placeholder(&self) -> jsbind::DOMString {
        self.inner.get("ariaPlaceholder").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_placeholder(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaPlaceholder", value);
    }
}
impl Element {
    pub fn aria_pos_in_set(&self) -> jsbind::DOMString {
        self.inner.get("ariaPosInSet").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_pos_in_set(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaPosInSet", value);
    }
}
impl Element {
    pub fn aria_pressed(&self) -> jsbind::DOMString {
        self.inner.get("ariaPressed").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_pressed(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaPressed", value);
    }
}
impl Element {
    pub fn aria_read_only(&self) -> jsbind::DOMString {
        self.inner.get("ariaReadOnly").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_read_only(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaReadOnly", value);
    }
}
impl Element {
    pub fn aria_relevant(&self) -> jsbind::DOMString {
        self.inner.get("ariaRelevant").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_relevant(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRelevant", value);
    }
}
impl Element {
    pub fn aria_required(&self) -> jsbind::DOMString {
        self.inner.get("ariaRequired").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_required(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRequired", value);
    }
}
impl Element {
    pub fn aria_role_description(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaRoleDescription")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_role_description(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRoleDescription", value);
    }
}
impl Element {
    pub fn aria_row_count(&self) -> jsbind::DOMString {
        self.inner.get("ariaRowCount").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_row_count(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRowCount", value);
    }
}
impl Element {
    pub fn aria_row_index(&self) -> jsbind::DOMString {
        self.inner.get("ariaRowIndex").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_row_index(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRowIndex", value);
    }
}
impl Element {
    pub fn aria_row_index_text(&self) -> jsbind::DOMString {
        self.inner
            .get("ariaRowIndexText")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_aria_row_index_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRowIndexText", value);
    }
}
impl Element {
    pub fn aria_row_span(&self) -> jsbind::DOMString {
        self.inner.get("ariaRowSpan").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_row_span(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaRowSpan", value);
    }
}
impl Element {
    pub fn aria_selected(&self) -> jsbind::DOMString {
        self.inner.get("ariaSelected").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_selected(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaSelected", value);
    }
}
impl Element {
    pub fn aria_set_size(&self) -> jsbind::DOMString {
        self.inner.get("ariaSetSize").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_set_size(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaSetSize", value);
    }
}
impl Element {
    pub fn aria_sort(&self) -> jsbind::DOMString {
        self.inner.get("ariaSort").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_sort(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaSort", value);
    }
}
impl Element {
    pub fn aria_value_max(&self) -> jsbind::DOMString {
        self.inner.get("ariaValueMax").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_value_max(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaValueMax", value);
    }
}
impl Element {
    pub fn aria_value_min(&self) -> jsbind::DOMString {
        self.inner.get("ariaValueMin").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_value_min(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaValueMin", value);
    }
}
impl Element {
    pub fn aria_value_now(&self) -> jsbind::DOMString {
        self.inner.get("ariaValueNow").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_value_now(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaValueNow", value);
    }
}
impl Element {
    pub fn aria_value_text(&self) -> jsbind::DOMString {
        self.inner.get("ariaValueText").as_::<jsbind::DOMString>()
    }

    pub fn set_aria_value_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("ariaValueText", value);
    }
}
impl Element {
    pub fn animate0(&self, keyframes: jsbind::Object) -> Animation {
        self.inner
            .call("animate", &[keyframes.into()])
            .as_::<Animation>()
    }

    pub fn animate1(&self, keyframes: jsbind::Object, options: jsbind::Any) -> Animation {
        self.inner
            .call("animate", &[keyframes.into(), options.into()])
            .as_::<Animation>()
    }
}
impl Element {
    pub fn get_animations0(&self) -> jsbind::Sequence<Animation> {
        self.inner
            .call("getAnimations", &[])
            .as_::<jsbind::Sequence<Animation>>()
    }

    pub fn get_animations1(&self, options: GetAnimationsOptions) -> jsbind::Sequence<Animation> {
        self.inner
            .call("getAnimations", &[options.into()])
            .as_::<jsbind::Sequence<Animation>>()
    }
}
