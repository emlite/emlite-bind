use super::*;

#[derive(Clone, Debug)]
pub struct XPathEvaluator {
    inner: emlite::Val,
}
impl FromVal for XPathEvaluator {
    fn from_val(v: &emlite::Val) -> Self {
        XPathEvaluator {
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
impl std::ops::Deref for XPathEvaluator {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XPathEvaluator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XPathEvaluator> for emlite::Val {
    fn from(s: XPathEvaluator) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XPathEvaluator {
    pub fn new() -> XPathEvaluator {
        Self {
            inner: emlite::Val::global("XPathEvaluator")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl XPathEvaluator {
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
impl XPathEvaluator {
    pub fn create_ns_resolver(&self, node_resolver: Node) -> Node {
        self.inner
            .call("createNSResolver", &[node_resolver.into()])
            .as_::<Node>()
    }
}
impl XPathEvaluator {
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
