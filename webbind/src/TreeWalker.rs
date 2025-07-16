use super::*;

/// The TreeWalker class.
/// [`TreeWalker`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TreeWalker {
    inner: Any,
}
impl FromVal for TreeWalker {
    fn from_val(v: &Any) -> Self {
        TreeWalker {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TreeWalker {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TreeWalker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TreeWalker {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TreeWalker {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TreeWalker> for Any {
    fn from(s: TreeWalker) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TreeWalker> for Any {
    fn from(s: &TreeWalker) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TreeWalker);

impl TreeWalker {
    /// Getter of the `root` attribute.
    /// [`TreeWalker.root`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/root)
    pub fn root(&self) -> Node {
        self.inner.get("root").as_::<Node>()
    }
}
impl TreeWalker {
    /// Getter of the `whatToShow` attribute.
    /// [`TreeWalker.whatToShow`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/whatToShow)
    pub fn what_to_show(&self) -> u32 {
        self.inner.get("whatToShow").as_::<u32>()
    }
}
impl TreeWalker {
    /// Getter of the `filter` attribute.
    /// [`TreeWalker.filter`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/filter)
    pub fn filter(&self) -> Function {
        self.inner.get("filter").as_::<Function>()
    }
}
impl TreeWalker {
    /// Getter of the `currentNode` attribute.
    /// [`TreeWalker.currentNode`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)
    pub fn current_node(&self) -> Node {
        self.inner.get("currentNode").as_::<Node>()
    }

    /// Setter of the `currentNode` attribute.
    /// [`TreeWalker.currentNode`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)
    pub fn set_current_node(&mut self, value: &Node) {
        self.inner.set("currentNode", value);
    }
}
impl TreeWalker {
    /// The parentNode method.
    /// [`TreeWalker.parentNode`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/parentNode)
    pub fn parent_node(&self) -> Node {
        self.inner.call("parentNode", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    /// The firstChild method.
    /// [`TreeWalker.firstChild`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/firstChild)
    pub fn first_child(&self) -> Node {
        self.inner.call("firstChild", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    /// The lastChild method.
    /// [`TreeWalker.lastChild`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/lastChild)
    pub fn last_child(&self) -> Node {
        self.inner.call("lastChild", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    /// The previousSibling method.
    /// [`TreeWalker.previousSibling`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousSibling)
    pub fn previous_sibling(&self) -> Node {
        self.inner.call("previousSibling", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    /// The nextSibling method.
    /// [`TreeWalker.nextSibling`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextSibling)
    pub fn next_sibling(&self) -> Node {
        self.inner.call("nextSibling", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    /// The previousNode method.
    /// [`TreeWalker.previousNode`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousNode)
    pub fn previous_node(&self) -> Node {
        self.inner.call("previousNode", &[]).as_::<Node>()
    }
}
impl TreeWalker {
    /// The nextNode method.
    /// [`TreeWalker.nextNode`](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextNode)
    pub fn next_node(&self) -> Node {
        self.inner.call("nextNode", &[]).as_::<Node>()
    }
}
