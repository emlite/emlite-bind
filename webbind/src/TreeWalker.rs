use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TreeWalker {
    inner: emlite::Val,
}
impl FromVal for TreeWalker {
    fn from_val(v: &emlite::Val) -> Self {
        TreeWalker {
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
impl core::ops::Deref for TreeWalker {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TreeWalker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TreeWalker {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TreeWalker {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TreeWalker> for emlite::Val {
    fn from(s: TreeWalker) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TreeWalker);

impl TreeWalker {
    pub fn root(&self) -> Node {
        self.inner.get("root").as_::<Node>()
    }
}
impl TreeWalker {
    pub fn what_to_show(&self) -> u32 {
        self.inner.get("whatToShow").as_::<u32>()
    }
}
impl TreeWalker {
    pub fn filter(&self) -> jsbind::Function {
        self.inner.get("filter").as_::<jsbind::Function>()
    }
}
impl TreeWalker {
    pub fn current_node(&self) -> Node {
        self.inner.get("currentNode").as_::<Node>()
    }

    pub fn set_current_node(&mut self, value: Node) {
        self.inner.set("currentNode", value);
    }
}
impl TreeWalker {
    pub fn parent_node(&self) -> Node {
        self.inner.call("parentNode", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    pub fn first_child(&self) -> Node {
        self.inner.call("firstChild", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    pub fn last_child(&self) -> Node {
        self.inner.call("lastChild", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    pub fn previous_sibling(&self) -> Node {
        self.inner.call("previousSibling", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    pub fn next_sibling(&self) -> Node {
        self.inner.call("nextSibling", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    pub fn previous_node(&self) -> Node {
        self.inner.call("previousNode", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    pub fn next_node(&self) -> Node {
        self.inner.call("nextNode", &[]).as_::<Node>()
    }
}
