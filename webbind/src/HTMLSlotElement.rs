use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AssignedNodesOptions {
    inner: emlite::Val,
}
impl FromVal for AssignedNodesOptions {
    fn from_val(v: &emlite::Val) -> Self {
        AssignedNodesOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AssignedNodesOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AssignedNodesOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AssignedNodesOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AssignedNodesOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AssignedNodesOptions> for emlite::Val {
    fn from(s: AssignedNodesOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLSlotElement {
    inner: HTMLElement,
}
impl FromVal for HTMLSlotElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLSlotElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for HTMLSlotElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLSlotElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLSlotElement> for emlite::Val {
    fn from(s: HTMLSlotElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLSlotElement);

impl HTMLSlotElement {
    pub fn new() -> HTMLSlotElement {
        Self {
            inner: emlite::Val::global("HTMLSlotElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLSlotElement {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLSlotElement {
    pub fn assigned_nodes0(&self) -> jsbind::Sequence<Node> {
        self.inner
            .call("assignedNodes", &[])
            .as_::<jsbind::Sequence<Node>>()
    }

    pub fn assigned_nodes1(&self, options: AssignedNodesOptions) -> jsbind::Sequence<Node> {
        self.inner
            .call("assignedNodes", &[options.into()])
            .as_::<jsbind::Sequence<Node>>()
    }
}
impl HTMLSlotElement {
    pub fn assigned_elements0(&self) -> jsbind::Sequence<Element> {
        self.inner
            .call("assignedElements", &[])
            .as_::<jsbind::Sequence<Element>>()
    }

    pub fn assigned_elements1(&self, options: AssignedNodesOptions) -> jsbind::Sequence<Element> {
        self.inner
            .call("assignedElements", &[options.into()])
            .as_::<jsbind::Sequence<Element>>()
    }
}
impl HTMLSlotElement {
    pub fn assign(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("assign", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
