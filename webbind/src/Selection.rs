use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetComposedRangesOptions {
    inner: emlite::Val,
}
impl FromVal for GetComposedRangesOptions {
    fn from_val(v: &emlite::Val) -> Self {
        GetComposedRangesOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GetComposedRangesOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GetComposedRangesOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GetComposedRangesOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GetComposedRangesOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GetComposedRangesOptions> for emlite::Val {
    fn from(s: GetComposedRangesOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GetComposedRangesOptions> for emlite::Val {
    fn from(s: &GetComposedRangesOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl GetComposedRangesOptions {
    pub fn shadow_roots(&self) -> Sequence<ShadowRoot> {
        self.inner.get("shadowRoots").as_::<Sequence<ShadowRoot>>()
    }

    pub fn set_shadow_roots(&mut self, value: Sequence<ShadowRoot>) {
        self.inner.set("shadowRoots", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Selection {
    inner: emlite::Val,
}
impl FromVal for Selection {
    fn from_val(v: &emlite::Val) -> Self {
        Selection {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Selection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Selection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Selection {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Selection {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Selection> for emlite::Val {
    fn from(s: Selection) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Selection> for emlite::Val {
    fn from(s: &Selection) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Selection);

impl Selection {
    pub fn anchor_node(&self) -> Node {
        self.inner.get("anchorNode").as_::<Node>()
    }
}
impl Selection {
    pub fn anchor_offset(&self) -> u32 {
        self.inner.get("anchorOffset").as_::<u32>()
    }
}
impl Selection {
    pub fn focus_node(&self) -> Node {
        self.inner.get("focusNode").as_::<Node>()
    }
}
impl Selection {
    pub fn focus_offset(&self) -> u32 {
        self.inner.get("focusOffset").as_::<u32>()
    }
}
impl Selection {
    pub fn is_collapsed(&self) -> bool {
        self.inner.get("isCollapsed").as_::<bool>()
    }
}
impl Selection {
    pub fn range_count(&self) -> u32 {
        self.inner.get("rangeCount").as_::<u32>()
    }
}
impl Selection {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }
}
impl Selection {
    pub fn direction(&self) -> DOMString {
        self.inner.get("direction").as_::<DOMString>()
    }
}
impl Selection {
    pub fn get_range_at(&self, index: u32) -> Range {
        self.inner
            .call("getRangeAt", &[index.into()])
            .as_::<Range>()
    }
}
impl Selection {
    pub fn add_range(&self, range: Range) -> Undefined {
        self.inner
            .call("addRange", &[range.into()])
            .as_::<Undefined>()
    }
}
impl Selection {
    pub fn remove_range(&self, range: Range) -> Undefined {
        self.inner
            .call("removeRange", &[range.into()])
            .as_::<Undefined>()
    }
}
impl Selection {
    pub fn remove_all_ranges(&self) -> Undefined {
        self.inner.call("removeAllRanges", &[]).as_::<Undefined>()
    }
}
impl Selection {
    pub fn empty(&self) -> Undefined {
        self.inner.call("empty", &[]).as_::<Undefined>()
    }
}
impl Selection {
    pub fn get_composed_ranges0(&self) -> Sequence<StaticRange> {
        self.inner
            .call("getComposedRanges", &[])
            .as_::<Sequence<StaticRange>>()
    }

    pub fn get_composed_ranges1(&self, options: GetComposedRangesOptions) -> Sequence<StaticRange> {
        self.inner
            .call("getComposedRanges", &[options.into()])
            .as_::<Sequence<StaticRange>>()
    }
}
impl Selection {
    pub fn collapse0(&self, node: Node) -> Undefined {
        self.inner
            .call("collapse", &[node.into()])
            .as_::<Undefined>()
    }

    pub fn collapse1(&self, node: Node, offset: u32) -> Undefined {
        self.inner
            .call("collapse", &[node.into(), offset.into()])
            .as_::<Undefined>()
    }
}
impl Selection {
    pub fn set_position0(&self, node: Node) -> Undefined {
        self.inner
            .call("setPosition", &[node.into()])
            .as_::<Undefined>()
    }

    pub fn set_position1(&self, node: Node, offset: u32) -> Undefined {
        self.inner
            .call("setPosition", &[node.into(), offset.into()])
            .as_::<Undefined>()
    }
}
impl Selection {
    pub fn collapse_to_start(&self) -> Undefined {
        self.inner.call("collapseToStart", &[]).as_::<Undefined>()
    }
}
impl Selection {
    pub fn collapse_to_end(&self) -> Undefined {
        self.inner.call("collapseToEnd", &[]).as_::<Undefined>()
    }
}
impl Selection {
    pub fn extend0(&self, node: Node) -> Undefined {
        self.inner.call("extend", &[node.into()]).as_::<Undefined>()
    }

    pub fn extend1(&self, node: Node, offset: u32) -> Undefined {
        self.inner
            .call("extend", &[node.into(), offset.into()])
            .as_::<Undefined>()
    }
}
impl Selection {
    pub fn set_base_and_extent(
        &self,
        anchor_node: Node,
        anchor_offset: u32,
        focus_node: Node,
        focus_offset: u32,
    ) -> Undefined {
        self.inner
            .call(
                "setBaseAndExtent",
                &[
                    anchor_node.into(),
                    anchor_offset.into(),
                    focus_node.into(),
                    focus_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl Selection {
    pub fn select_all_children(&self, node: Node) -> Undefined {
        self.inner
            .call("selectAllChildren", &[node.into()])
            .as_::<Undefined>()
    }
}
impl Selection {
    pub fn modify0(&self) -> Undefined {
        self.inner.call("modify", &[]).as_::<Undefined>()
    }

    pub fn modify1(&self, alter: DOMString) -> Undefined {
        self.inner
            .call("modify", &[alter.into()])
            .as_::<Undefined>()
    }

    pub fn modify2(&self, alter: DOMString, direction: DOMString) -> Undefined {
        self.inner
            .call("modify", &[alter.into(), direction.into()])
            .as_::<Undefined>()
    }

    pub fn modify3(
        &self,
        alter: DOMString,
        direction: DOMString,
        granularity: DOMString,
    ) -> Undefined {
        self.inner
            .call(
                "modify",
                &[alter.into(), direction.into(), granularity.into()],
            )
            .as_::<Undefined>()
    }
}
impl Selection {
    pub fn delete_from_document(&self) -> Undefined {
        self.inner
            .call("deleteFromDocument", &[])
            .as_::<Undefined>()
    }
}
impl Selection {
    pub fn contains_node0(&self, node: Node) -> bool {
        self.inner
            .call("containsNode", &[node.into()])
            .as_::<bool>()
    }

    pub fn contains_node1(&self, node: Node, allow_partial_containment: bool) -> bool {
        self.inner
            .call(
                "containsNode",
                &[node.into(), allow_partial_containment.into()],
            )
            .as_::<bool>()
    }
}
