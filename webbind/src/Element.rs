use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShadowRootInit {
    inner: Any,
}
impl FromVal for ShadowRootInit {
    fn from_val(v: &Any) -> Self {
        ShadowRootInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ShadowRootInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ShadowRootInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ShadowRootInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ShadowRootInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ShadowRootInit> for Any {
    fn from(s: ShadowRootInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ShadowRootInit> for Any {
    fn from(s: &ShadowRootInit) -> Any {
        s.inner.clone()
    }
}

impl ShadowRootInit {
    pub fn mode(&self) -> ShadowRootMode {
        self.inner.get("mode").as_::<ShadowRootMode>()
    }

    pub fn set_mode(&mut self, value: &ShadowRootMode) {
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

    pub fn set_slot_assignment(&mut self, value: &SlotAssignmentMode) {
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

    pub fn set_custom_element_registry(&mut self, value: &CustomElementRegistry) {
        self.inner.set("customElementRegistry", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FocusableAreasOption {
    inner: Any,
}
impl FromVal for FocusableAreasOption {
    fn from_val(v: &Any) -> Self {
        FocusableAreasOption { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FocusableAreasOption {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FocusableAreasOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FocusableAreasOption {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FocusableAreasOption {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FocusableAreasOption> for Any {
    fn from(s: FocusableAreasOption) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FocusableAreasOption> for Any {
    fn from(s: &FocusableAreasOption) -> Any {
        s.inner.clone()
    }
}

impl FocusableAreasOption {
    pub fn mode(&self) -> FocusableAreaSearchMode {
        self.inner.get("mode").as_::<FocusableAreaSearchMode>()
    }

    pub fn set_mode(&mut self, value: &FocusableAreaSearchMode) {
        self.inner.set("mode", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpatialNavigationSearchOptions {
    inner: Any,
}
impl FromVal for SpatialNavigationSearchOptions {
    fn from_val(v: &Any) -> Self {
        SpatialNavigationSearchOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SpatialNavigationSearchOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpatialNavigationSearchOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SpatialNavigationSearchOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SpatialNavigationSearchOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SpatialNavigationSearchOptions> for Any {
    fn from(s: SpatialNavigationSearchOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SpatialNavigationSearchOptions> for Any {
    fn from(s: &SpatialNavigationSearchOptions) -> Any {
        s.inner.clone()
    }
}

impl SpatialNavigationSearchOptions {
    pub fn candidates(&self) -> Sequence<Node> {
        self.inner.get("candidates").as_::<Sequence<Node>>()
    }

    pub fn set_candidates(&mut self, value: &Sequence<Node>) {
        self.inner.set("candidates", value);
    }
}
impl SpatialNavigationSearchOptions {
    pub fn container(&self) -> Node {
        self.inner.get("container").as_::<Node>()
    }

    pub fn set_container(&mut self, value: &Node) {
        self.inner.set("container", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CheckVisibilityOptions {
    inner: Any,
}
impl FromVal for CheckVisibilityOptions {
    fn from_val(v: &Any) -> Self {
        CheckVisibilityOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CheckVisibilityOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CheckVisibilityOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CheckVisibilityOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CheckVisibilityOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CheckVisibilityOptions> for Any {
    fn from(s: CheckVisibilityOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CheckVisibilityOptions> for Any {
    fn from(s: &CheckVisibilityOptions) -> Any {
        s.inner.clone()
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
    inner: Any,
}
impl FromVal for FullscreenOptions {
    fn from_val(v: &Any) -> Self {
        FullscreenOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FullscreenOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FullscreenOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FullscreenOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FullscreenOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FullscreenOptions> for Any {
    fn from(s: FullscreenOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FullscreenOptions> for Any {
    fn from(s: &FullscreenOptions) -> Any {
        s.inner.clone()
    }
}

impl FullscreenOptions {
    pub fn screen(&self) -> ScreenDetailed {
        self.inner.get("screen").as_::<ScreenDetailed>()
    }

    pub fn set_screen(&mut self, value: &ScreenDetailed) {
        self.inner.set("screen", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetHTMLOptions {
    inner: Any,
}
impl FromVal for GetHTMLOptions {
    fn from_val(v: &Any) -> Self {
        GetHTMLOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GetHTMLOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GetHTMLOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GetHTMLOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GetHTMLOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GetHTMLOptions> for Any {
    fn from(s: GetHTMLOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GetHTMLOptions> for Any {
    fn from(s: &GetHTMLOptions) -> Any {
        s.inner.clone()
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
    pub fn shadow_roots(&self) -> Sequence<ShadowRoot> {
        self.inner.get("shadowRoots").as_::<Sequence<ShadowRoot>>()
    }

    pub fn set_shadow_roots(&mut self, value: &Sequence<ShadowRoot>) {
        self.inner.set("shadowRoots", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PointerLockOptions {
    inner: Any,
}
impl FromVal for PointerLockOptions {
    fn from_val(v: &Any) -> Self {
        PointerLockOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PointerLockOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PointerLockOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PointerLockOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PointerLockOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PointerLockOptions> for Any {
    fn from(s: PointerLockOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PointerLockOptions> for Any {
    fn from(s: &PointerLockOptions) -> Any {
        s.inner.clone()
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
    inner: Any,
}
impl FromVal for GetAnimationsOptions {
    fn from_val(v: &Any) -> Self {
        GetAnimationsOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GetAnimationsOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GetAnimationsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GetAnimationsOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GetAnimationsOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GetAnimationsOptions> for Any {
    fn from(s: GetAnimationsOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GetAnimationsOptions> for Any {
    fn from(s: &GetAnimationsOptions) -> Any {
        s.inner.clone()
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
    pub fn pseudo_element(&self) -> CSSOMString {
        self.inner.get("pseudoElement").as_::<CSSOMString>()
    }

    pub fn set_pseudo_element(&mut self, value: &CSSOMString) {
        self.inner.set("pseudoElement", value);
    }
}
/// The Element class.
/// [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Element {
    inner: Node,
}
impl FromVal for Element {
    fn from_val(v: &Any) -> Self {
        Element {
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
impl AsRef<Any> for Element {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Element {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Element> for Any {
    fn from(s: Element) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Element> for Any {
    fn from(s: &Element) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Element);

impl Element {
    /// Getter of the `namespaceURI` attribute.
    /// [`Element.namespaceURI`](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI)
    pub fn namespace_uri(&self) -> DOMString {
        self.inner.get("namespaceURI").as_::<DOMString>()
    }
}
impl Element {
    /// Getter of the `prefix` attribute.
    /// [`Element.prefix`](https://developer.mozilla.org/en-US/docs/Web/API/Element/prefix)
    pub fn prefix(&self) -> DOMString {
        self.inner.get("prefix").as_::<DOMString>()
    }
}
impl Element {
    /// Getter of the `localName` attribute.
    /// [`Element.localName`](https://developer.mozilla.org/en-US/docs/Web/API/Element/localName)
    pub fn local_name(&self) -> DOMString {
        self.inner.get("localName").as_::<DOMString>()
    }
}
impl Element {
    /// Getter of the `tagName` attribute.
    /// [`Element.tagName`](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)
    pub fn tag_name(&self) -> DOMString {
        self.inner.get("tagName").as_::<DOMString>()
    }
}
impl Element {
    /// Getter of the `id` attribute.
    /// [`Element.id`](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }

    /// Setter of the `id` attribute.
    /// [`Element.id`](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)
    pub fn set_id(&mut self, value: &DOMString) {
        self.inner.set("id", value);
    }
}
impl Element {
    /// Getter of the `className` attribute.
    /// [`Element.className`](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)
    pub fn class_name(&self) -> DOMString {
        self.inner.get("className").as_::<DOMString>()
    }

    /// Setter of the `className` attribute.
    /// [`Element.className`](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)
    pub fn set_class_name(&mut self, value: &DOMString) {
        self.inner.set("className", value);
    }
}
impl Element {
    /// Getter of the `classList` attribute.
    /// [`Element.classList`](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)
    pub fn class_list(&self) -> DOMTokenList {
        self.inner.get("classList").as_::<DOMTokenList>()
    }
}
impl Element {
    /// Getter of the `slot` attribute.
    /// [`Element.slot`](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)
    pub fn slot(&self) -> DOMString {
        self.inner.get("slot").as_::<DOMString>()
    }

    /// Setter of the `slot` attribute.
    /// [`Element.slot`](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)
    pub fn set_slot(&mut self, value: &DOMString) {
        self.inner.set("slot", value);
    }
}
impl Element {
    /// The hasAttributes method.
    /// [`Element.hasAttributes`](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributes)
    pub fn has_attributes(&self) -> bool {
        self.inner.call("hasAttributes", &[]).as_::<bool>()
    }
}
impl Element {
    /// Getter of the `attributes` attribute.
    /// [`Element.attributes`](https://developer.mozilla.org/en-US/docs/Web/API/Element/attributes)
    pub fn attributes(&self) -> NamedNodeMap {
        self.inner.get("attributes").as_::<NamedNodeMap>()
    }
}
impl Element {
    /// The getAttributeNames method.
    /// [`Element.getAttributeNames`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNames)
    pub fn get_attribute_names(&self) -> Sequence<DOMString> {
        self.inner
            .call("getAttributeNames", &[])
            .as_::<Sequence<DOMString>>()
    }
}
impl Element {
    /// The getAttribute method.
    /// [`Element.getAttribute`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute)
    pub fn get_attribute(&self, qualified_name: &DOMString) -> DOMString {
        self.inner
            .call("getAttribute", &[qualified_name.into()])
            .as_::<DOMString>()
    }
}
impl Element {
    /// The getAttributeNS method.
    /// [`Element.getAttributeNS`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNS)
    pub fn get_attribute_ns(&self, namespace: &DOMString, local_name: &DOMString) -> DOMString {
        self.inner
            .call("getAttributeNS", &[namespace.into(), local_name.into()])
            .as_::<DOMString>()
    }
}
impl Element {
    /// The setAttribute method.
    /// [`Element.setAttribute`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute)
    pub fn set_attribute(&self, qualified_name: &DOMString, value: &DOMString) -> Undefined {
        self.inner
            .call("setAttribute", &[qualified_name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The setAttributeNS method.
    /// [`Element.setAttributeNS`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNS)
    pub fn set_attribute_ns(
        &self,
        namespace: &DOMString,
        qualified_name: &DOMString,
        value: &DOMString,
    ) -> Undefined {
        self.inner
            .call(
                "setAttributeNS",
                &[namespace.into(), qualified_name.into(), value.into()],
            )
            .as_::<Undefined>()
    }
}
impl Element {
    /// The removeAttribute method.
    /// [`Element.removeAttribute`](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttribute)
    pub fn remove_attribute(&self, qualified_name: &DOMString) -> Undefined {
        self.inner
            .call("removeAttribute", &[qualified_name.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The removeAttributeNS method.
    /// [`Element.removeAttributeNS`](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNS)
    pub fn remove_attribute_ns(&self, namespace: &DOMString, local_name: &DOMString) -> Undefined {
        self.inner
            .call("removeAttributeNS", &[namespace.into(), local_name.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The toggleAttribute method.
    /// [`Element.toggleAttribute`](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)
    pub fn toggle_attribute0(&self, qualified_name: &DOMString) -> bool {
        self.inner
            .call("toggleAttribute", &[qualified_name.into()])
            .as_::<bool>()
    }
    /// The toggleAttribute method.
    /// [`Element.toggleAttribute`](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)
    pub fn toggle_attribute1(&self, qualified_name: &DOMString, force: bool) -> bool {
        self.inner
            .call("toggleAttribute", &[qualified_name.into(), force.into()])
            .as_::<bool>()
    }
}
impl Element {
    /// The hasAttribute method.
    /// [`Element.hasAttribute`](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttribute)
    pub fn has_attribute(&self, qualified_name: &DOMString) -> bool {
        self.inner
            .call("hasAttribute", &[qualified_name.into()])
            .as_::<bool>()
    }
}
impl Element {
    /// The hasAttributeNS method.
    /// [`Element.hasAttributeNS`](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributeNS)
    pub fn has_attribute_ns(&self, namespace: &DOMString, local_name: &DOMString) -> bool {
        self.inner
            .call("hasAttributeNS", &[namespace.into(), local_name.into()])
            .as_::<bool>()
    }
}
impl Element {
    /// The getAttributeNode method.
    /// [`Element.getAttributeNode`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNode)
    pub fn get_attribute_node(&self, qualified_name: &DOMString) -> Attr {
        self.inner
            .call("getAttributeNode", &[qualified_name.into()])
            .as_::<Attr>()
    }
}
impl Element {
    /// The getAttributeNodeNS method.
    /// [`Element.getAttributeNodeNS`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNodeNS)
    pub fn get_attribute_node_ns(&self, namespace: &DOMString, local_name: &DOMString) -> Attr {
        self.inner
            .call("getAttributeNodeNS", &[namespace.into(), local_name.into()])
            .as_::<Attr>()
    }
}
impl Element {
    /// The setAttributeNode method.
    /// [`Element.setAttributeNode`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNode)
    pub fn set_attribute_node(&self, attr: &Attr) -> Attr {
        self.inner
            .call("setAttributeNode", &[attr.into()])
            .as_::<Attr>()
    }
}
impl Element {
    /// The setAttributeNodeNS method.
    /// [`Element.setAttributeNodeNS`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNodeNS)
    pub fn set_attribute_node_ns(&self, attr: &Attr) -> Attr {
        self.inner
            .call("setAttributeNodeNS", &[attr.into()])
            .as_::<Attr>()
    }
}
impl Element {
    /// The removeAttributeNode method.
    /// [`Element.removeAttributeNode`](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNode)
    pub fn remove_attribute_node(&self, attr: &Attr) -> Attr {
        self.inner
            .call("removeAttributeNode", &[attr.into()])
            .as_::<Attr>()
    }
}
impl Element {
    /// The attachShadow method.
    /// [`Element.attachShadow`](https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow)
    pub fn attach_shadow(&self, init: &ShadowRootInit) -> ShadowRoot {
        self.inner
            .call("attachShadow", &[init.into()])
            .as_::<ShadowRoot>()
    }
}
impl Element {
    /// Getter of the `shadowRoot` attribute.
    /// [`Element.shadowRoot`](https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot)
    pub fn shadow_root(&self) -> ShadowRoot {
        self.inner.get("shadowRoot").as_::<ShadowRoot>()
    }
}
impl Element {
    /// Getter of the `customElementRegistry` attribute.
    /// [`Element.customElementRegistry`](https://developer.mozilla.org/en-US/docs/Web/API/Element/customElementRegistry)
    pub fn custom_element_registry(&self) -> CustomElementRegistry {
        self.inner
            .get("customElementRegistry")
            .as_::<CustomElementRegistry>()
    }
}
impl Element {
    /// The closest method.
    /// [`Element.closest`](https://developer.mozilla.org/en-US/docs/Web/API/Element/closest)
    pub fn closest(&self, selectors: &DOMString) -> Element {
        self.inner
            .call("closest", &[selectors.into()])
            .as_::<Element>()
    }
}
impl Element {
    /// The matches method.
    /// [`Element.matches`](https://developer.mozilla.org/en-US/docs/Web/API/Element/matches)
    pub fn matches(&self, selectors: &DOMString) -> bool {
        self.inner
            .call("matches", &[selectors.into()])
            .as_::<bool>()
    }
}
impl Element {
    /// The webkitMatchesSelector method.
    /// [`Element.webkitMatchesSelector`](https://developer.mozilla.org/en-US/docs/Web/API/Element/webkitMatchesSelector)
    pub fn webkit_matches_selector(&self, selectors: &DOMString) -> bool {
        self.inner
            .call("webkitMatchesSelector", &[selectors.into()])
            .as_::<bool>()
    }
}
impl Element {
    /// The getElementsByTagName method.
    /// [`Element.getElementsByTagName`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagName)
    pub fn get_elements_by_tag_name(&self, qualified_name: &DOMString) -> HTMLCollection {
        self.inner
            .call("getElementsByTagName", &[qualified_name.into()])
            .as_::<HTMLCollection>()
    }
}
impl Element {
    /// The getElementsByTagNameNS method.
    /// [`Element.getElementsByTagNameNS`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagNameNS)
    pub fn get_elements_by_tag_name_ns(
        &self,
        namespace: &DOMString,
        local_name: &DOMString,
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
    /// The getElementsByClassName method.
    /// [`Element.getElementsByClassName`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByClassName)
    pub fn get_elements_by_class_name(&self, class_names: &DOMString) -> HTMLCollection {
        self.inner
            .call("getElementsByClassName", &[class_names.into()])
            .as_::<HTMLCollection>()
    }
}
impl Element {
    /// The insertAdjacentElement method.
    /// [`Element.insertAdjacentElement`](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentElement)
    pub fn insert_adjacent_element(&self, where_: &DOMString, element: &Element) -> Element {
        self.inner
            .call("insertAdjacentElement", &[where_.into(), element.into()])
            .as_::<Element>()
    }
}
impl Element {
    /// The insertAdjacentText method.
    /// [`Element.insertAdjacentText`](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentText)
    pub fn insert_adjacent_text(&self, where_: &DOMString, data: &DOMString) -> Undefined {
        self.inner
            .call("insertAdjacentText", &[where_.into(), data.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The getSpatialNavigationContainer method.
    /// [`Element.getSpatialNavigationContainer`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getSpatialNavigationContainer)
    pub fn get_spatial_navigation_container(&self) -> Node {
        self.inner
            .call("getSpatialNavigationContainer", &[])
            .as_::<Node>()
    }
}
impl Element {
    /// The focusableAreas method.
    /// [`Element.focusableAreas`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusableAreas)
    pub fn focusable_areas0(&self) -> Sequence<Node> {
        self.inner
            .call("focusableAreas", &[])
            .as_::<Sequence<Node>>()
    }
    /// The focusableAreas method.
    /// [`Element.focusableAreas`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusableAreas)
    pub fn focusable_areas1(&self, option: &FocusableAreasOption) -> Sequence<Node> {
        self.inner
            .call("focusableAreas", &[option.into()])
            .as_::<Sequence<Node>>()
    }
}
impl Element {
    /// The spatialNavigationSearch method.
    /// [`Element.spatialNavigationSearch`](https://developer.mozilla.org/en-US/docs/Web/API/Element/spatialNavigationSearch)
    pub fn spatial_navigation_search0(&self, dir: &SpatialNavigationDirection) -> Node {
        self.inner
            .call("spatialNavigationSearch", &[dir.into()])
            .as_::<Node>()
    }
    /// The spatialNavigationSearch method.
    /// [`Element.spatialNavigationSearch`](https://developer.mozilla.org/en-US/docs/Web/API/Element/spatialNavigationSearch)
    pub fn spatial_navigation_search1(
        &self,
        dir: &SpatialNavigationDirection,
        options: &SpatialNavigationSearchOptions,
    ) -> Node {
        self.inner
            .call("spatialNavigationSearch", &[dir.into(), options.into()])
            .as_::<Node>()
    }
}
impl Element {
    /// The pseudo method.
    /// [`Element.pseudo`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pseudo)
    pub fn pseudo(&self, type_: &CSSOMString) -> CSSPseudoElement {
        self.inner
            .call("pseudo", &[type_.into()])
            .as_::<CSSPseudoElement>()
    }
}
impl Element {
    /// Getter of the `part` attribute.
    /// [`Element.part`](https://developer.mozilla.org/en-US/docs/Web/API/Element/part)
    pub fn part(&self) -> DOMTokenList {
        self.inner.get("part").as_::<DOMTokenList>()
    }
}
impl Element {
    /// The computedStyleMap method.
    /// [`Element.computedStyleMap`](https://developer.mozilla.org/en-US/docs/Web/API/Element/computedStyleMap)
    pub fn computed_style_map(&self) -> StylePropertyMapReadOnly {
        self.inner
            .call("computedStyleMap", &[])
            .as_::<StylePropertyMapReadOnly>()
    }
}
impl Element {
    /// The getClientRects method.
    /// [`Element.getClientRects`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getClientRects)
    pub fn get_client_rects(&self) -> DOMRectList {
        self.inner.call("getClientRects", &[]).as_::<DOMRectList>()
    }
}
impl Element {
    /// The getBoundingClientRect method.
    /// [`Element.getBoundingClientRect`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect)
    pub fn get_bounding_client_rect(&self) -> DOMRect {
        self.inner
            .call("getBoundingClientRect", &[])
            .as_::<DOMRect>()
    }
}
impl Element {
    /// The checkVisibility method.
    /// [`Element.checkVisibility`](https://developer.mozilla.org/en-US/docs/Web/API/Element/checkVisibility)
    pub fn check_visibility0(&self) -> bool {
        self.inner.call("checkVisibility", &[]).as_::<bool>()
    }
    /// The checkVisibility method.
    /// [`Element.checkVisibility`](https://developer.mozilla.org/en-US/docs/Web/API/Element/checkVisibility)
    pub fn check_visibility1(&self, options: &CheckVisibilityOptions) -> bool {
        self.inner
            .call("checkVisibility", &[options.into()])
            .as_::<bool>()
    }
}
impl Element {
    /// The scrollIntoView method.
    /// [`Element.scrollIntoView`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)
    pub fn scroll_into_view0(&self) -> Undefined {
        self.inner.call("scrollIntoView", &[]).as_::<Undefined>()
    }
    /// The scrollIntoView method.
    /// [`Element.scrollIntoView`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)
    pub fn scroll_into_view1(&self, arg: &Any) -> Undefined {
        self.inner
            .call("scrollIntoView", &[arg.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The scroll method.
    /// [`Element.scroll`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)
    pub fn scroll(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("scroll", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The scrollTo method.
    /// [`Element.scrollTo`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)
    pub fn scroll_to(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("scrollTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The scrollBy method.
    /// [`Element.scrollBy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)
    pub fn scroll_by(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("scrollBy", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// Getter of the `scrollTop` attribute.
    /// [`Element.scrollTop`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)
    pub fn scroll_top(&self) -> f64 {
        self.inner.get("scrollTop").as_::<f64>()
    }

    /// Setter of the `scrollTop` attribute.
    /// [`Element.scrollTop`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)
    pub fn set_scroll_top(&mut self, value: f64) {
        self.inner.set("scrollTop", value);
    }
}
impl Element {
    /// Getter of the `scrollLeft` attribute.
    /// [`Element.scrollLeft`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)
    pub fn scroll_left(&self) -> f64 {
        self.inner.get("scrollLeft").as_::<f64>()
    }

    /// Setter of the `scrollLeft` attribute.
    /// [`Element.scrollLeft`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)
    pub fn set_scroll_left(&mut self, value: f64) {
        self.inner.set("scrollLeft", value);
    }
}
impl Element {
    /// Getter of the `scrollWidth` attribute.
    /// [`Element.scrollWidth`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollWidth)
    pub fn scroll_width(&self) -> i32 {
        self.inner.get("scrollWidth").as_::<i32>()
    }
}
impl Element {
    /// Getter of the `scrollHeight` attribute.
    /// [`Element.scrollHeight`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight)
    pub fn scroll_height(&self) -> i32 {
        self.inner.get("scrollHeight").as_::<i32>()
    }
}
impl Element {
    /// Getter of the `clientTop` attribute.
    /// [`Element.clientTop`](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientTop)
    pub fn client_top(&self) -> i32 {
        self.inner.get("clientTop").as_::<i32>()
    }
}
impl Element {
    /// Getter of the `clientLeft` attribute.
    /// [`Element.clientLeft`](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientLeft)
    pub fn client_left(&self) -> i32 {
        self.inner.get("clientLeft").as_::<i32>()
    }
}
impl Element {
    /// Getter of the `clientWidth` attribute.
    /// [`Element.clientWidth`](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientWidth)
    pub fn client_width(&self) -> i32 {
        self.inner.get("clientWidth").as_::<i32>()
    }
}
impl Element {
    /// Getter of the `clientHeight` attribute.
    /// [`Element.clientHeight`](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientHeight)
    pub fn client_height(&self) -> i32 {
        self.inner.get("clientHeight").as_::<i32>()
    }
}
impl Element {
    /// Getter of the `currentCSSZoom` attribute.
    /// [`Element.currentCSSZoom`](https://developer.mozilla.org/en-US/docs/Web/API/Element/currentCSSZoom)
    pub fn current_css_zoom(&self) -> f64 {
        self.inner.get("currentCSSZoom").as_::<f64>()
    }
}
impl Element {
    /// Getter of the `elementTiming` attribute.
    /// [`Element.elementTiming`](https://developer.mozilla.org/en-US/docs/Web/API/Element/elementTiming)
    pub fn element_timing(&self) -> DOMString {
        self.inner.get("elementTiming").as_::<DOMString>()
    }

    /// Setter of the `elementTiming` attribute.
    /// [`Element.elementTiming`](https://developer.mozilla.org/en-US/docs/Web/API/Element/elementTiming)
    pub fn set_element_timing(&mut self, value: &DOMString) {
        self.inner.set("elementTiming", value);
    }
}
impl Element {
    /// The requestFullscreen method.
    /// [`Element.requestFullscreen`](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen)
    pub fn request_fullscreen0(&self) -> Promise<Undefined> {
        self.inner
            .call("requestFullscreen", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The requestFullscreen method.
    /// [`Element.requestFullscreen`](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen)
    pub fn request_fullscreen1(&self, options: &FullscreenOptions) -> Promise<Undefined> {
        self.inner
            .call("requestFullscreen", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl Element {
    /// Getter of the `onfullscreenchange` attribute.
    /// [`Element.onfullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Element/onfullscreenchange)
    pub fn onfullscreenchange(&self) -> Any {
        self.inner.get("onfullscreenchange").as_::<Any>()
    }

    /// Setter of the `onfullscreenchange` attribute.
    /// [`Element.onfullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Element/onfullscreenchange)
    pub fn set_onfullscreenchange(&mut self, value: &Any) {
        self.inner.set("onfullscreenchange", value);
    }
}
impl Element {
    /// Getter of the `onfullscreenerror` attribute.
    /// [`Element.onfullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Element/onfullscreenerror)
    pub fn onfullscreenerror(&self) -> Any {
        self.inner.get("onfullscreenerror").as_::<Any>()
    }

    /// Setter of the `onfullscreenerror` attribute.
    /// [`Element.onfullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Element/onfullscreenerror)
    pub fn set_onfullscreenerror(&mut self, value: &Any) {
        self.inner.set("onfullscreenerror", value);
    }
}
impl Element {
    /// The setHTMLUnsafe method.
    /// [`Element.setHTMLUnsafe`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setHTMLUnsafe)
    pub fn set_html_unsafe(&self, html: &Any) -> Undefined {
        self.inner
            .call("setHTMLUnsafe", &[html.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The getHTML method.
    /// [`Element.getHTML`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getHTML)
    pub fn get_html0(&self) -> DOMString {
        self.inner.call("getHTML", &[]).as_::<DOMString>()
    }
    /// The getHTML method.
    /// [`Element.getHTML`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getHTML)
    pub fn get_html1(&self, options: &GetHTMLOptions) -> DOMString {
        self.inner
            .call("getHTML", &[options.into()])
            .as_::<DOMString>()
    }
}
impl Element {
    /// Getter of the `innerHTML` attribute.
    /// [`Element.innerHTML`](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)
    pub fn inner_html(&self) -> Any {
        self.inner.get("innerHTML").as_::<Any>()
    }

    /// Setter of the `innerHTML` attribute.
    /// [`Element.innerHTML`](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)
    pub fn set_inner_html(&mut self, value: &Any) {
        self.inner.set("innerHTML", value);
    }
}
impl Element {
    /// Getter of the `outerHTML` attribute.
    /// [`Element.outerHTML`](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)
    pub fn outer_html(&self) -> Any {
        self.inner.get("outerHTML").as_::<Any>()
    }

    /// Setter of the `outerHTML` attribute.
    /// [`Element.outerHTML`](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)
    pub fn set_outer_html(&mut self, value: &Any) {
        self.inner.set("outerHTML", value);
    }
}
impl Element {
    /// The insertAdjacentHTML method.
    /// [`Element.insertAdjacentHTML`](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)
    pub fn insert_adjacent_html(&self, position: &DOMString, string: &Any) -> Undefined {
        self.inner
            .call("insertAdjacentHTML", &[position.into(), string.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The setPointerCapture method.
    /// [`Element.setPointerCapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture)
    pub fn set_pointer_capture(&self, pointer_id: i32) -> Undefined {
        self.inner
            .call("setPointerCapture", &[pointer_id.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The releasePointerCapture method.
    /// [`Element.releasePointerCapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/releasePointerCapture)
    pub fn release_pointer_capture(&self, pointer_id: i32) -> Undefined {
        self.inner
            .call("releasePointerCapture", &[pointer_id.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The hasPointerCapture method.
    /// [`Element.hasPointerCapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasPointerCapture)
    pub fn has_pointer_capture(&self, pointer_id: i32) -> bool {
        self.inner
            .call("hasPointerCapture", &[pointer_id.into()])
            .as_::<bool>()
    }
}
impl Element {
    /// The requestPointerLock method.
    /// [`Element.requestPointerLock`](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock)
    pub fn request_pointer_lock0(&self) -> Promise<Undefined> {
        self.inner
            .call("requestPointerLock", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The requestPointerLock method.
    /// [`Element.requestPointerLock`](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock)
    pub fn request_pointer_lock1(&self, options: &PointerLockOptions) -> Promise<Undefined> {
        self.inner
            .call("requestPointerLock", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl Element {
    /// Getter of the `regionOverset` attribute.
    /// [`Element.regionOverset`](https://developer.mozilla.org/en-US/docs/Web/API/Element/regionOverset)
    pub fn region_overset(&self) -> CSSOMString {
        self.inner.get("regionOverset").as_::<CSSOMString>()
    }
}
impl Element {
    /// The getRegionFlowRanges method.
    /// [`Element.getRegionFlowRanges`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getRegionFlowRanges)
    pub fn get_region_flow_ranges(&self) -> Sequence<Range> {
        self.inner
            .call("getRegionFlowRanges", &[])
            .as_::<Sequence<Range>>()
    }
}
impl Element {
    /// The getBoxQuads method.
    /// [`Element.getBoxQuads`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)
    pub fn get_box_quads0(&self) -> Sequence<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[])
            .as_::<Sequence<DOMQuad>>()
    }
    /// The getBoxQuads method.
    /// [`Element.getBoxQuads`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)
    pub fn get_box_quads1(&self, options: &BoxQuadOptions) -> Sequence<DOMQuad> {
        self.inner
            .call("getBoxQuads", &[options.into()])
            .as_::<Sequence<DOMQuad>>()
    }
}
impl Element {
    /// The convertQuadFromNode method.
    /// [`Element.convertQuadFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)
    pub fn convert_quad_from_node0(&self, quad: &DOMQuadInit, from: &Any) -> DOMQuad {
        self.inner
            .call("convertQuadFromNode", &[quad.into(), from.into()])
            .as_::<DOMQuad>()
    }
    /// The convertQuadFromNode method.
    /// [`Element.convertQuadFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)
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
impl Element {
    /// The convertRectFromNode method.
    /// [`Element.convertRectFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)
    pub fn convert_rect_from_node0(&self, rect: &DOMRectReadOnly, from: &Any) -> DOMQuad {
        self.inner
            .call("convertRectFromNode", &[rect.into(), from.into()])
            .as_::<DOMQuad>()
    }
    /// The convertRectFromNode method.
    /// [`Element.convertRectFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)
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
impl Element {
    /// The convertPointFromNode method.
    /// [`Element.convertPointFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)
    pub fn convert_point_from_node0(&self, point: &DOMPointInit, from: &Any) -> DOMPoint {
        self.inner
            .call("convertPointFromNode", &[point.into(), from.into()])
            .as_::<DOMPoint>()
    }
    /// The convertPointFromNode method.
    /// [`Element.convertPointFromNode`](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)
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
impl Element {
    /// Getter of the `children` attribute.
    /// [`Element.children`](https://developer.mozilla.org/en-US/docs/Web/API/Element/children)
    pub fn children(&self) -> HTMLCollection {
        self.inner.get("children").as_::<HTMLCollection>()
    }
}
impl Element {
    /// Getter of the `firstElementChild` attribute.
    /// [`Element.firstElementChild`](https://developer.mozilla.org/en-US/docs/Web/API/Element/firstElementChild)
    pub fn first_element_child(&self) -> Element {
        self.inner.get("firstElementChild").as_::<Element>()
    }
}
impl Element {
    /// Getter of the `lastElementChild` attribute.
    /// [`Element.lastElementChild`](https://developer.mozilla.org/en-US/docs/Web/API/Element/lastElementChild)
    pub fn last_element_child(&self) -> Element {
        self.inner.get("lastElementChild").as_::<Element>()
    }
}
impl Element {
    /// Getter of the `childElementCount` attribute.
    /// [`Element.childElementCount`](https://developer.mozilla.org/en-US/docs/Web/API/Element/childElementCount)
    pub fn child_element_count(&self) -> u32 {
        self.inner.get("childElementCount").as_::<u32>()
    }
}
impl Element {
    /// The prepend method.
    /// [`Element.prepend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)
    pub fn prepend(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("prepend", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The append method.
    /// [`Element.append`](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)
    pub fn append(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("append", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The replaceChildren method.
    /// [`Element.replaceChildren`](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)
    pub fn replace_children(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("replaceChildren", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The moveBefore method.
    /// [`Element.moveBefore`](https://developer.mozilla.org/en-US/docs/Web/API/Element/moveBefore)
    pub fn move_before(&self, node: &Node, child: &Node) -> Undefined {
        self.inner
            .call("moveBefore", &[node.into(), child.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The querySelector method.
    /// [`Element.querySelector`](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector)
    pub fn query_selector(&self, selectors: &DOMString) -> Element {
        self.inner
            .call("querySelector", &[selectors.into()])
            .as_::<Element>()
    }
}
impl Element {
    /// The querySelectorAll method.
    /// [`Element.querySelectorAll`](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll)
    pub fn query_selector_all(&self, selectors: &DOMString) -> NodeList {
        self.inner
            .call("querySelectorAll", &[selectors.into()])
            .as_::<NodeList>()
    }
}
impl Element {
    /// Getter of the `previousElementSibling` attribute.
    /// [`Element.previousElementSibling`](https://developer.mozilla.org/en-US/docs/Web/API/Element/previousElementSibling)
    pub fn previous_element_sibling(&self) -> Element {
        self.inner.get("previousElementSibling").as_::<Element>()
    }
}
impl Element {
    /// Getter of the `nextElementSibling` attribute.
    /// [`Element.nextElementSibling`](https://developer.mozilla.org/en-US/docs/Web/API/Element/nextElementSibling)
    pub fn next_element_sibling(&self) -> Element {
        self.inner.get("nextElementSibling").as_::<Element>()
    }
}
impl Element {
    /// The before method.
    /// [`Element.before`](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)
    pub fn before(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("before", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The after method.
    /// [`Element.after`](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)
    pub fn after(&self, nodes: &Any) -> Undefined {
        self.inner.call("after", &[nodes.into()]).as_::<Undefined>()
    }
}
impl Element {
    /// The replaceWith method.
    /// [`Element.replaceWith`](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)
    pub fn replace_with(&self, nodes: &Any) -> Undefined {
        self.inner
            .call("replaceWith", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl Element {
    /// The remove method.
    /// [`Element.remove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/remove)
    pub fn remove(&self) -> Undefined {
        self.inner.call("remove", &[]).as_::<Undefined>()
    }
}
impl Element {
    /// Getter of the `assignedSlot` attribute.
    /// [`Element.assignedSlot`](https://developer.mozilla.org/en-US/docs/Web/API/Element/assignedSlot)
    pub fn assigned_slot(&self) -> HTMLSlotElement {
        self.inner.get("assignedSlot").as_::<HTMLSlotElement>()
    }
}
impl Element {
    /// Getter of the `role` attribute.
    /// [`Element.role`](https://developer.mozilla.org/en-US/docs/Web/API/Element/role)
    pub fn role(&self) -> DOMString {
        self.inner.get("role").as_::<DOMString>()
    }

    /// Setter of the `role` attribute.
    /// [`Element.role`](https://developer.mozilla.org/en-US/docs/Web/API/Element/role)
    pub fn set_role(&mut self, value: &DOMString) {
        self.inner.set("role", value);
    }
}
impl Element {
    /// Getter of the `ariaActiveDescendantElement` attribute.
    /// [`Element.ariaActiveDescendantElement`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaActiveDescendantElement)
    pub fn aria_active_descendant_element(&self) -> Element {
        self.inner
            .get("ariaActiveDescendantElement")
            .as_::<Element>()
    }

    /// Setter of the `ariaActiveDescendantElement` attribute.
    /// [`Element.ariaActiveDescendantElement`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaActiveDescendantElement)
    pub fn set_aria_active_descendant_element(&mut self, value: &Element) {
        self.inner.set("ariaActiveDescendantElement", value);
    }
}
impl Element {
    /// Getter of the `ariaAtomic` attribute.
    /// [`Element.ariaAtomic`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaAtomic)
    pub fn aria_atomic(&self) -> DOMString {
        self.inner.get("ariaAtomic").as_::<DOMString>()
    }

    /// Setter of the `ariaAtomic` attribute.
    /// [`Element.ariaAtomic`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaAtomic)
    pub fn set_aria_atomic(&mut self, value: &DOMString) {
        self.inner.set("ariaAtomic", value);
    }
}
impl Element {
    /// Getter of the `ariaAutoComplete` attribute.
    /// [`Element.ariaAutoComplete`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaAutoComplete)
    pub fn aria_auto_complete(&self) -> DOMString {
        self.inner.get("ariaAutoComplete").as_::<DOMString>()
    }

    /// Setter of the `ariaAutoComplete` attribute.
    /// [`Element.ariaAutoComplete`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaAutoComplete)
    pub fn set_aria_auto_complete(&mut self, value: &DOMString) {
        self.inner.set("ariaAutoComplete", value);
    }
}
impl Element {
    /// Getter of the `ariaBrailleLabel` attribute.
    /// [`Element.ariaBrailleLabel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaBrailleLabel)
    pub fn aria_braille_label(&self) -> DOMString {
        self.inner.get("ariaBrailleLabel").as_::<DOMString>()
    }

    /// Setter of the `ariaBrailleLabel` attribute.
    /// [`Element.ariaBrailleLabel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaBrailleLabel)
    pub fn set_aria_braille_label(&mut self, value: &DOMString) {
        self.inner.set("ariaBrailleLabel", value);
    }
}
impl Element {
    /// Getter of the `ariaBrailleRoleDescription` attribute.
    /// [`Element.ariaBrailleRoleDescription`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaBrailleRoleDescription)
    pub fn aria_braille_role_description(&self) -> DOMString {
        self.inner
            .get("ariaBrailleRoleDescription")
            .as_::<DOMString>()
    }

    /// Setter of the `ariaBrailleRoleDescription` attribute.
    /// [`Element.ariaBrailleRoleDescription`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaBrailleRoleDescription)
    pub fn set_aria_braille_role_description(&mut self, value: &DOMString) {
        self.inner.set("ariaBrailleRoleDescription", value);
    }
}
impl Element {
    /// Getter of the `ariaBusy` attribute.
    /// [`Element.ariaBusy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaBusy)
    pub fn aria_busy(&self) -> DOMString {
        self.inner.get("ariaBusy").as_::<DOMString>()
    }

    /// Setter of the `ariaBusy` attribute.
    /// [`Element.ariaBusy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaBusy)
    pub fn set_aria_busy(&mut self, value: &DOMString) {
        self.inner.set("ariaBusy", value);
    }
}
impl Element {
    /// Getter of the `ariaChecked` attribute.
    /// [`Element.ariaChecked`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaChecked)
    pub fn aria_checked(&self) -> DOMString {
        self.inner.get("ariaChecked").as_::<DOMString>()
    }

    /// Setter of the `ariaChecked` attribute.
    /// [`Element.ariaChecked`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaChecked)
    pub fn set_aria_checked(&mut self, value: &DOMString) {
        self.inner.set("ariaChecked", value);
    }
}
impl Element {
    /// Getter of the `ariaColCount` attribute.
    /// [`Element.ariaColCount`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaColCount)
    pub fn aria_col_count(&self) -> DOMString {
        self.inner.get("ariaColCount").as_::<DOMString>()
    }

    /// Setter of the `ariaColCount` attribute.
    /// [`Element.ariaColCount`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaColCount)
    pub fn set_aria_col_count(&mut self, value: &DOMString) {
        self.inner.set("ariaColCount", value);
    }
}
impl Element {
    /// Getter of the `ariaColIndex` attribute.
    /// [`Element.ariaColIndex`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaColIndex)
    pub fn aria_col_index(&self) -> DOMString {
        self.inner.get("ariaColIndex").as_::<DOMString>()
    }

    /// Setter of the `ariaColIndex` attribute.
    /// [`Element.ariaColIndex`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaColIndex)
    pub fn set_aria_col_index(&mut self, value: &DOMString) {
        self.inner.set("ariaColIndex", value);
    }
}
impl Element {
    /// Getter of the `ariaColIndexText` attribute.
    /// [`Element.ariaColIndexText`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaColIndexText)
    pub fn aria_col_index_text(&self) -> DOMString {
        self.inner.get("ariaColIndexText").as_::<DOMString>()
    }

    /// Setter of the `ariaColIndexText` attribute.
    /// [`Element.ariaColIndexText`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaColIndexText)
    pub fn set_aria_col_index_text(&mut self, value: &DOMString) {
        self.inner.set("ariaColIndexText", value);
    }
}
impl Element {
    /// Getter of the `ariaColSpan` attribute.
    /// [`Element.ariaColSpan`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaColSpan)
    pub fn aria_col_span(&self) -> DOMString {
        self.inner.get("ariaColSpan").as_::<DOMString>()
    }

    /// Setter of the `ariaColSpan` attribute.
    /// [`Element.ariaColSpan`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaColSpan)
    pub fn set_aria_col_span(&mut self, value: &DOMString) {
        self.inner.set("ariaColSpan", value);
    }
}
impl Element {
    /// Getter of the `ariaControlsElements` attribute.
    /// [`Element.ariaControlsElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaControlsElements)
    pub fn aria_controls_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaControlsElements")
            .as_::<FrozenArray<Element>>()
    }

    /// Setter of the `ariaControlsElements` attribute.
    /// [`Element.ariaControlsElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaControlsElements)
    pub fn set_aria_controls_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaControlsElements", value);
    }
}
impl Element {
    /// Getter of the `ariaCurrent` attribute.
    /// [`Element.ariaCurrent`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaCurrent)
    pub fn aria_current(&self) -> DOMString {
        self.inner.get("ariaCurrent").as_::<DOMString>()
    }

    /// Setter of the `ariaCurrent` attribute.
    /// [`Element.ariaCurrent`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaCurrent)
    pub fn set_aria_current(&mut self, value: &DOMString) {
        self.inner.set("ariaCurrent", value);
    }
}
impl Element {
    /// Getter of the `ariaDescribedByElements` attribute.
    /// [`Element.ariaDescribedByElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaDescribedByElements)
    pub fn aria_described_by_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaDescribedByElements")
            .as_::<FrozenArray<Element>>()
    }

    /// Setter of the `ariaDescribedByElements` attribute.
    /// [`Element.ariaDescribedByElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaDescribedByElements)
    pub fn set_aria_described_by_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaDescribedByElements", value);
    }
}
impl Element {
    /// Getter of the `ariaDescription` attribute.
    /// [`Element.ariaDescription`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaDescription)
    pub fn aria_description(&self) -> DOMString {
        self.inner.get("ariaDescription").as_::<DOMString>()
    }

    /// Setter of the `ariaDescription` attribute.
    /// [`Element.ariaDescription`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaDescription)
    pub fn set_aria_description(&mut self, value: &DOMString) {
        self.inner.set("ariaDescription", value);
    }
}
impl Element {
    /// Getter of the `ariaDetailsElements` attribute.
    /// [`Element.ariaDetailsElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaDetailsElements)
    pub fn aria_details_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaDetailsElements")
            .as_::<FrozenArray<Element>>()
    }

    /// Setter of the `ariaDetailsElements` attribute.
    /// [`Element.ariaDetailsElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaDetailsElements)
    pub fn set_aria_details_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaDetailsElements", value);
    }
}
impl Element {
    /// Getter of the `ariaDisabled` attribute.
    /// [`Element.ariaDisabled`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaDisabled)
    pub fn aria_disabled(&self) -> DOMString {
        self.inner.get("ariaDisabled").as_::<DOMString>()
    }

    /// Setter of the `ariaDisabled` attribute.
    /// [`Element.ariaDisabled`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaDisabled)
    pub fn set_aria_disabled(&mut self, value: &DOMString) {
        self.inner.set("ariaDisabled", value);
    }
}
impl Element {
    /// Getter of the `ariaErrorMessageElements` attribute.
    /// [`Element.ariaErrorMessageElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaErrorMessageElements)
    pub fn aria_error_message_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaErrorMessageElements")
            .as_::<FrozenArray<Element>>()
    }

    /// Setter of the `ariaErrorMessageElements` attribute.
    /// [`Element.ariaErrorMessageElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaErrorMessageElements)
    pub fn set_aria_error_message_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaErrorMessageElements", value);
    }
}
impl Element {
    /// Getter of the `ariaExpanded` attribute.
    /// [`Element.ariaExpanded`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaExpanded)
    pub fn aria_expanded(&self) -> DOMString {
        self.inner.get("ariaExpanded").as_::<DOMString>()
    }

    /// Setter of the `ariaExpanded` attribute.
    /// [`Element.ariaExpanded`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaExpanded)
    pub fn set_aria_expanded(&mut self, value: &DOMString) {
        self.inner.set("ariaExpanded", value);
    }
}
impl Element {
    /// Getter of the `ariaFlowToElements` attribute.
    /// [`Element.ariaFlowToElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaFlowToElements)
    pub fn aria_flow_to_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaFlowToElements")
            .as_::<FrozenArray<Element>>()
    }

    /// Setter of the `ariaFlowToElements` attribute.
    /// [`Element.ariaFlowToElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaFlowToElements)
    pub fn set_aria_flow_to_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaFlowToElements", value);
    }
}
impl Element {
    /// Getter of the `ariaHasPopup` attribute.
    /// [`Element.ariaHasPopup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaHasPopup)
    pub fn aria_has_popup(&self) -> DOMString {
        self.inner.get("ariaHasPopup").as_::<DOMString>()
    }

    /// Setter of the `ariaHasPopup` attribute.
    /// [`Element.ariaHasPopup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaHasPopup)
    pub fn set_aria_has_popup(&mut self, value: &DOMString) {
        self.inner.set("ariaHasPopup", value);
    }
}
impl Element {
    /// Getter of the `ariaHidden` attribute.
    /// [`Element.ariaHidden`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaHidden)
    pub fn aria_hidden(&self) -> DOMString {
        self.inner.get("ariaHidden").as_::<DOMString>()
    }

    /// Setter of the `ariaHidden` attribute.
    /// [`Element.ariaHidden`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaHidden)
    pub fn set_aria_hidden(&mut self, value: &DOMString) {
        self.inner.set("ariaHidden", value);
    }
}
impl Element {
    /// Getter of the `ariaInvalid` attribute.
    /// [`Element.ariaInvalid`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaInvalid)
    pub fn aria_invalid(&self) -> DOMString {
        self.inner.get("ariaInvalid").as_::<DOMString>()
    }

    /// Setter of the `ariaInvalid` attribute.
    /// [`Element.ariaInvalid`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaInvalid)
    pub fn set_aria_invalid(&mut self, value: &DOMString) {
        self.inner.set("ariaInvalid", value);
    }
}
impl Element {
    /// Getter of the `ariaKeyShortcuts` attribute.
    /// [`Element.ariaKeyShortcuts`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaKeyShortcuts)
    pub fn aria_key_shortcuts(&self) -> DOMString {
        self.inner.get("ariaKeyShortcuts").as_::<DOMString>()
    }

    /// Setter of the `ariaKeyShortcuts` attribute.
    /// [`Element.ariaKeyShortcuts`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaKeyShortcuts)
    pub fn set_aria_key_shortcuts(&mut self, value: &DOMString) {
        self.inner.set("ariaKeyShortcuts", value);
    }
}
impl Element {
    /// Getter of the `ariaLabel` attribute.
    /// [`Element.ariaLabel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaLabel)
    pub fn aria_label(&self) -> DOMString {
        self.inner.get("ariaLabel").as_::<DOMString>()
    }

    /// Setter of the `ariaLabel` attribute.
    /// [`Element.ariaLabel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaLabel)
    pub fn set_aria_label(&mut self, value: &DOMString) {
        self.inner.set("ariaLabel", value);
    }
}
impl Element {
    /// Getter of the `ariaLabelledByElements` attribute.
    /// [`Element.ariaLabelledByElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaLabelledByElements)
    pub fn aria_labelled_by_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaLabelledByElements")
            .as_::<FrozenArray<Element>>()
    }

    /// Setter of the `ariaLabelledByElements` attribute.
    /// [`Element.ariaLabelledByElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaLabelledByElements)
    pub fn set_aria_labelled_by_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaLabelledByElements", value);
    }
}
impl Element {
    /// Getter of the `ariaLevel` attribute.
    /// [`Element.ariaLevel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaLevel)
    pub fn aria_level(&self) -> DOMString {
        self.inner.get("ariaLevel").as_::<DOMString>()
    }

    /// Setter of the `ariaLevel` attribute.
    /// [`Element.ariaLevel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaLevel)
    pub fn set_aria_level(&mut self, value: &DOMString) {
        self.inner.set("ariaLevel", value);
    }
}
impl Element {
    /// Getter of the `ariaLive` attribute.
    /// [`Element.ariaLive`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaLive)
    pub fn aria_live(&self) -> DOMString {
        self.inner.get("ariaLive").as_::<DOMString>()
    }

    /// Setter of the `ariaLive` attribute.
    /// [`Element.ariaLive`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaLive)
    pub fn set_aria_live(&mut self, value: &DOMString) {
        self.inner.set("ariaLive", value);
    }
}
impl Element {
    /// Getter of the `ariaModal` attribute.
    /// [`Element.ariaModal`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaModal)
    pub fn aria_modal(&self) -> DOMString {
        self.inner.get("ariaModal").as_::<DOMString>()
    }

    /// Setter of the `ariaModal` attribute.
    /// [`Element.ariaModal`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaModal)
    pub fn set_aria_modal(&mut self, value: &DOMString) {
        self.inner.set("ariaModal", value);
    }
}
impl Element {
    /// Getter of the `ariaMultiLine` attribute.
    /// [`Element.ariaMultiLine`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaMultiLine)
    pub fn aria_multi_line(&self) -> DOMString {
        self.inner.get("ariaMultiLine").as_::<DOMString>()
    }

    /// Setter of the `ariaMultiLine` attribute.
    /// [`Element.ariaMultiLine`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaMultiLine)
    pub fn set_aria_multi_line(&mut self, value: &DOMString) {
        self.inner.set("ariaMultiLine", value);
    }
}
impl Element {
    /// Getter of the `ariaMultiSelectable` attribute.
    /// [`Element.ariaMultiSelectable`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaMultiSelectable)
    pub fn aria_multi_selectable(&self) -> DOMString {
        self.inner.get("ariaMultiSelectable").as_::<DOMString>()
    }

    /// Setter of the `ariaMultiSelectable` attribute.
    /// [`Element.ariaMultiSelectable`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaMultiSelectable)
    pub fn set_aria_multi_selectable(&mut self, value: &DOMString) {
        self.inner.set("ariaMultiSelectable", value);
    }
}
impl Element {
    /// Getter of the `ariaOrientation` attribute.
    /// [`Element.ariaOrientation`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaOrientation)
    pub fn aria_orientation(&self) -> DOMString {
        self.inner.get("ariaOrientation").as_::<DOMString>()
    }

    /// Setter of the `ariaOrientation` attribute.
    /// [`Element.ariaOrientation`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaOrientation)
    pub fn set_aria_orientation(&mut self, value: &DOMString) {
        self.inner.set("ariaOrientation", value);
    }
}
impl Element {
    /// Getter of the `ariaOwnsElements` attribute.
    /// [`Element.ariaOwnsElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaOwnsElements)
    pub fn aria_owns_elements(&self) -> FrozenArray<Element> {
        self.inner
            .get("ariaOwnsElements")
            .as_::<FrozenArray<Element>>()
    }

    /// Setter of the `ariaOwnsElements` attribute.
    /// [`Element.ariaOwnsElements`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaOwnsElements)
    pub fn set_aria_owns_elements(&mut self, value: &FrozenArray<Element>) {
        self.inner.set("ariaOwnsElements", value);
    }
}
impl Element {
    /// Getter of the `ariaPlaceholder` attribute.
    /// [`Element.ariaPlaceholder`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaPlaceholder)
    pub fn aria_placeholder(&self) -> DOMString {
        self.inner.get("ariaPlaceholder").as_::<DOMString>()
    }

    /// Setter of the `ariaPlaceholder` attribute.
    /// [`Element.ariaPlaceholder`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaPlaceholder)
    pub fn set_aria_placeholder(&mut self, value: &DOMString) {
        self.inner.set("ariaPlaceholder", value);
    }
}
impl Element {
    /// Getter of the `ariaPosInSet` attribute.
    /// [`Element.ariaPosInSet`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaPosInSet)
    pub fn aria_pos_in_set(&self) -> DOMString {
        self.inner.get("ariaPosInSet").as_::<DOMString>()
    }

    /// Setter of the `ariaPosInSet` attribute.
    /// [`Element.ariaPosInSet`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaPosInSet)
    pub fn set_aria_pos_in_set(&mut self, value: &DOMString) {
        self.inner.set("ariaPosInSet", value);
    }
}
impl Element {
    /// Getter of the `ariaPressed` attribute.
    /// [`Element.ariaPressed`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaPressed)
    pub fn aria_pressed(&self) -> DOMString {
        self.inner.get("ariaPressed").as_::<DOMString>()
    }

    /// Setter of the `ariaPressed` attribute.
    /// [`Element.ariaPressed`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaPressed)
    pub fn set_aria_pressed(&mut self, value: &DOMString) {
        self.inner.set("ariaPressed", value);
    }
}
impl Element {
    /// Getter of the `ariaReadOnly` attribute.
    /// [`Element.ariaReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaReadOnly)
    pub fn aria_read_only(&self) -> DOMString {
        self.inner.get("ariaReadOnly").as_::<DOMString>()
    }

    /// Setter of the `ariaReadOnly` attribute.
    /// [`Element.ariaReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaReadOnly)
    pub fn set_aria_read_only(&mut self, value: &DOMString) {
        self.inner.set("ariaReadOnly", value);
    }
}
impl Element {
    /// Getter of the `ariaRelevant` attribute.
    /// [`Element.ariaRelevant`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRelevant)
    pub fn aria_relevant(&self) -> DOMString {
        self.inner.get("ariaRelevant").as_::<DOMString>()
    }

    /// Setter of the `ariaRelevant` attribute.
    /// [`Element.ariaRelevant`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRelevant)
    pub fn set_aria_relevant(&mut self, value: &DOMString) {
        self.inner.set("ariaRelevant", value);
    }
}
impl Element {
    /// Getter of the `ariaRequired` attribute.
    /// [`Element.ariaRequired`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRequired)
    pub fn aria_required(&self) -> DOMString {
        self.inner.get("ariaRequired").as_::<DOMString>()
    }

    /// Setter of the `ariaRequired` attribute.
    /// [`Element.ariaRequired`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRequired)
    pub fn set_aria_required(&mut self, value: &DOMString) {
        self.inner.set("ariaRequired", value);
    }
}
impl Element {
    /// Getter of the `ariaRoleDescription` attribute.
    /// [`Element.ariaRoleDescription`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRoleDescription)
    pub fn aria_role_description(&self) -> DOMString {
        self.inner.get("ariaRoleDescription").as_::<DOMString>()
    }

    /// Setter of the `ariaRoleDescription` attribute.
    /// [`Element.ariaRoleDescription`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRoleDescription)
    pub fn set_aria_role_description(&mut self, value: &DOMString) {
        self.inner.set("ariaRoleDescription", value);
    }
}
impl Element {
    /// Getter of the `ariaRowCount` attribute.
    /// [`Element.ariaRowCount`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRowCount)
    pub fn aria_row_count(&self) -> DOMString {
        self.inner.get("ariaRowCount").as_::<DOMString>()
    }

    /// Setter of the `ariaRowCount` attribute.
    /// [`Element.ariaRowCount`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRowCount)
    pub fn set_aria_row_count(&mut self, value: &DOMString) {
        self.inner.set("ariaRowCount", value);
    }
}
impl Element {
    /// Getter of the `ariaRowIndex` attribute.
    /// [`Element.ariaRowIndex`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRowIndex)
    pub fn aria_row_index(&self) -> DOMString {
        self.inner.get("ariaRowIndex").as_::<DOMString>()
    }

    /// Setter of the `ariaRowIndex` attribute.
    /// [`Element.ariaRowIndex`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRowIndex)
    pub fn set_aria_row_index(&mut self, value: &DOMString) {
        self.inner.set("ariaRowIndex", value);
    }
}
impl Element {
    /// Getter of the `ariaRowIndexText` attribute.
    /// [`Element.ariaRowIndexText`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRowIndexText)
    pub fn aria_row_index_text(&self) -> DOMString {
        self.inner.get("ariaRowIndexText").as_::<DOMString>()
    }

    /// Setter of the `ariaRowIndexText` attribute.
    /// [`Element.ariaRowIndexText`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRowIndexText)
    pub fn set_aria_row_index_text(&mut self, value: &DOMString) {
        self.inner.set("ariaRowIndexText", value);
    }
}
impl Element {
    /// Getter of the `ariaRowSpan` attribute.
    /// [`Element.ariaRowSpan`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRowSpan)
    pub fn aria_row_span(&self) -> DOMString {
        self.inner.get("ariaRowSpan").as_::<DOMString>()
    }

    /// Setter of the `ariaRowSpan` attribute.
    /// [`Element.ariaRowSpan`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaRowSpan)
    pub fn set_aria_row_span(&mut self, value: &DOMString) {
        self.inner.set("ariaRowSpan", value);
    }
}
impl Element {
    /// Getter of the `ariaSelected` attribute.
    /// [`Element.ariaSelected`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaSelected)
    pub fn aria_selected(&self) -> DOMString {
        self.inner.get("ariaSelected").as_::<DOMString>()
    }

    /// Setter of the `ariaSelected` attribute.
    /// [`Element.ariaSelected`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaSelected)
    pub fn set_aria_selected(&mut self, value: &DOMString) {
        self.inner.set("ariaSelected", value);
    }
}
impl Element {
    /// Getter of the `ariaSetSize` attribute.
    /// [`Element.ariaSetSize`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaSetSize)
    pub fn aria_set_size(&self) -> DOMString {
        self.inner.get("ariaSetSize").as_::<DOMString>()
    }

    /// Setter of the `ariaSetSize` attribute.
    /// [`Element.ariaSetSize`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaSetSize)
    pub fn set_aria_set_size(&mut self, value: &DOMString) {
        self.inner.set("ariaSetSize", value);
    }
}
impl Element {
    /// Getter of the `ariaSort` attribute.
    /// [`Element.ariaSort`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaSort)
    pub fn aria_sort(&self) -> DOMString {
        self.inner.get("ariaSort").as_::<DOMString>()
    }

    /// Setter of the `ariaSort` attribute.
    /// [`Element.ariaSort`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaSort)
    pub fn set_aria_sort(&mut self, value: &DOMString) {
        self.inner.set("ariaSort", value);
    }
}
impl Element {
    /// Getter of the `ariaValueMax` attribute.
    /// [`Element.ariaValueMax`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaValueMax)
    pub fn aria_value_max(&self) -> DOMString {
        self.inner.get("ariaValueMax").as_::<DOMString>()
    }

    /// Setter of the `ariaValueMax` attribute.
    /// [`Element.ariaValueMax`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaValueMax)
    pub fn set_aria_value_max(&mut self, value: &DOMString) {
        self.inner.set("ariaValueMax", value);
    }
}
impl Element {
    /// Getter of the `ariaValueMin` attribute.
    /// [`Element.ariaValueMin`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaValueMin)
    pub fn aria_value_min(&self) -> DOMString {
        self.inner.get("ariaValueMin").as_::<DOMString>()
    }

    /// Setter of the `ariaValueMin` attribute.
    /// [`Element.ariaValueMin`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaValueMin)
    pub fn set_aria_value_min(&mut self, value: &DOMString) {
        self.inner.set("ariaValueMin", value);
    }
}
impl Element {
    /// Getter of the `ariaValueNow` attribute.
    /// [`Element.ariaValueNow`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaValueNow)
    pub fn aria_value_now(&self) -> DOMString {
        self.inner.get("ariaValueNow").as_::<DOMString>()
    }

    /// Setter of the `ariaValueNow` attribute.
    /// [`Element.ariaValueNow`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaValueNow)
    pub fn set_aria_value_now(&mut self, value: &DOMString) {
        self.inner.set("ariaValueNow", value);
    }
}
impl Element {
    /// Getter of the `ariaValueText` attribute.
    /// [`Element.ariaValueText`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaValueText)
    pub fn aria_value_text(&self) -> DOMString {
        self.inner.get("ariaValueText").as_::<DOMString>()
    }

    /// Setter of the `ariaValueText` attribute.
    /// [`Element.ariaValueText`](https://developer.mozilla.org/en-US/docs/Web/API/Element/ariaValueText)
    pub fn set_aria_value_text(&mut self, value: &DOMString) {
        self.inner.set("ariaValueText", value);
    }
}
impl Element {
    /// The animate method.
    /// [`Element.animate`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animate)
    pub fn animate0(&self, keyframes: &Object) -> Animation {
        self.inner
            .call("animate", &[keyframes.into()])
            .as_::<Animation>()
    }
    /// The animate method.
    /// [`Element.animate`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animate)
    pub fn animate1(&self, keyframes: &Object, options: &Any) -> Animation {
        self.inner
            .call("animate", &[keyframes.into(), options.into()])
            .as_::<Animation>()
    }
}
impl Element {
    /// The getAnimations method.
    /// [`Element.getAnimations`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAnimations)
    pub fn get_animations0(&self) -> Sequence<Animation> {
        self.inner
            .call("getAnimations", &[])
            .as_::<Sequence<Animation>>()
    }
    /// The getAnimations method.
    /// [`Element.getAnimations`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAnimations)
    pub fn get_animations1(&self, options: &GetAnimationsOptions) -> Sequence<Animation> {
        self.inner
            .call("getAnimations", &[options.into()])
            .as_::<Sequence<Animation>>()
    }
}
