use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentFragment {
    inner: Node,
}
impl FromVal for DocumentFragment {
    fn from_val(v: &emlite::Val) -> Self {
        DocumentFragment { inner: Node::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DocumentFragment {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DocumentFragment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DocumentFragment {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DocumentFragment {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<DocumentFragment> for emlite::Val {
    fn from(s: DocumentFragment) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DocumentFragment);



impl DocumentFragment {
    pub fn new() -> DocumentFragment {
        Self {
            inner: emlite::Val::global("DocumentFragment").new(&[]).as_::<Node>(),
        }
    }

}
impl DocumentFragment {
    pub fn get_element_by_id(&self, element_id: DOMString) -> Element {
        self.inner.call("getElementById", &[element_id.into(), ]).as_::<Element>()
    }

}
impl DocumentFragment {
    pub fn children(&self) -> HTMLCollection {
        self.inner.get("children").as_::<HTMLCollection>()
    }

}
impl DocumentFragment {
    pub fn first_element_child(&self) -> Element {
        self.inner.get("firstElementChild").as_::<Element>()
    }

}
impl DocumentFragment {
    pub fn last_element_child(&self) -> Element {
        self.inner.get("lastElementChild").as_::<Element>()
    }

}
impl DocumentFragment {
    pub fn child_element_count(&self) -> u32 {
        self.inner.get("childElementCount").as_::<u32>()
    }

}
impl DocumentFragment {
    pub fn prepend(&self, nodes: Any) -> Undefined {
        self.inner.call("prepend", &[nodes.into(), ]).as_::<Undefined>()
    }

}
impl DocumentFragment {
    pub fn append(&self, nodes: Any) -> Undefined {
        self.inner.call("append", &[nodes.into(), ]).as_::<Undefined>()
    }

}
impl DocumentFragment {
    pub fn replace_children(&self, nodes: Any) -> Undefined {
        self.inner.call("replaceChildren", &[nodes.into(), ]).as_::<Undefined>()
    }

}
impl DocumentFragment {
    pub fn move_before(&self, node: Node, child: Node) -> Undefined {
        self.inner.call("moveBefore", &[node.into(), child.into(), ]).as_::<Undefined>()
    }

}
impl DocumentFragment {
    pub fn query_selector(&self, selectors: DOMString) -> Element {
        self.inner.call("querySelector", &[selectors.into(), ]).as_::<Element>()
    }

}
impl DocumentFragment {
    pub fn query_selector_all(&self, selectors: DOMString) -> NodeList {
        self.inner.call("querySelectorAll", &[selectors.into(), ]).as_::<NodeList>()
    }

}
