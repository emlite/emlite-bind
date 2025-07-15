use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XPathEvaluator {
    inner: emlite::Val,
}
impl FromVal for XPathEvaluator {
    fn from_val(v: &emlite::Val) -> Self {
        XPathEvaluator { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XPathEvaluator {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XPathEvaluator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XPathEvaluator {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XPathEvaluator {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<XPathEvaluator> for emlite::Val {
    fn from(s: XPathEvaluator) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XPathEvaluator);



impl XPathEvaluator {
    pub fn new() -> XPathEvaluator {
        Self {
            inner: emlite::Val::global("XPathEvaluator").new(&[]).as_::<emlite::Val>(),
        }
    }

}
impl XPathEvaluator {
    pub fn create_expression0(&self, expression: DOMString) -> XPathExpression {
        self.inner.call("createExpression", &[expression.into(), ]).as_::<XPathExpression>()
    }

    pub fn create_expression1(&self, expression: DOMString, resolver: Function) -> XPathExpression {
        self.inner.call("createExpression", &[expression.into(), resolver.into(), ]).as_::<XPathExpression>()
    }

}
impl XPathEvaluator {
    pub fn create_ns_resolver(&self, node_resolver: Node) -> Node {
        self.inner.call("createNSResolver", &[node_resolver.into(), ]).as_::<Node>()
    }

}
impl XPathEvaluator {
    pub fn evaluate0(&self, expression: DOMString, context_node: Node) -> XPathResult {
        self.inner.call("evaluate", &[expression.into(), context_node.into(), ]).as_::<XPathResult>()
    }

    pub fn evaluate1(&self, expression: DOMString, context_node: Node, resolver: Function) -> XPathResult {
        self.inner.call("evaluate", &[expression.into(), context_node.into(), resolver.into(), ]).as_::<XPathResult>()
    }

    pub fn evaluate2(&self, expression: DOMString, context_node: Node, resolver: Function, type_: u16) -> XPathResult {
        self.inner.call("evaluate", &[expression.into(), context_node.into(), resolver.into(), type_.into(), ]).as_::<XPathResult>()
    }

    pub fn evaluate3(&self, expression: DOMString, context_node: Node, resolver: Function, type_: u16, result: XPathResult) -> XPathResult {
        self.inner.call("evaluate", &[expression.into(), context_node.into(), resolver.into(), type_.into(), result.into(), ]).as_::<XPathResult>()
    }

}
