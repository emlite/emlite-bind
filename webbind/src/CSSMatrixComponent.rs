use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSMatrixComponent {
    inner: CSSTransformComponent,
}
impl FromVal for CSSMatrixComponent {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMatrixComponent {
            inner: CSSTransformComponent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSMatrixComponent {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMatrixComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSMatrixComponent> for emlite::Val {
    fn from(s: CSSMatrixComponent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSMatrixComponent {
    pub fn new0(matrix: DOMMatrixReadOnly) -> CSSMatrixComponent {
        Self {
            inner: emlite::Val::global("CSSMatrixComponent")
                .new(&[matrix.into()])
                .as_::<CSSTransformComponent>(),
        }
    }

    pub fn new1(matrix: DOMMatrixReadOnly, options: jsbind::Any) -> CSSMatrixComponent {
        Self {
            inner: emlite::Val::global("CSSMatrixComponent")
                .new(&[matrix.into(), options.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSMatrixComponent {
    pub fn matrix(&self) -> DOMMatrix {
        self.inner.get("matrix").as_::<DOMMatrix>()
    }

    pub fn set_matrix(&mut self, value: DOMMatrix) {
        self.inner.set("matrix", value);
    }
}
