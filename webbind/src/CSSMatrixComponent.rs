use super::*;

/// The CSSMatrixComponent class.
/// [`CSSMatrixComponent`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMatrixComponent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMatrixComponent {
    inner: CSSTransformComponent,
}

impl FromVal for CSSMatrixComponent {
    fn from_val(v: &Any) -> Self {
        CSSMatrixComponent {
            inner: CSSTransformComponent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for CSSMatrixComponent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSMatrixComponent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSMatrixComponent> for Any {
    fn from(s: CSSMatrixComponent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSMatrixComponent> for Any {
    fn from(s: &CSSMatrixComponent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSMatrixComponent);

impl CSSMatrixComponent {
    /// Getter of the `matrix` attribute.
    /// [`CSSMatrixComponent.matrix`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMatrixComponent/matrix)
    pub fn matrix(&self) -> DOMMatrix {
        self.inner.get("matrix").as_::<DOMMatrix>()
    }

    /// Setter of the `matrix` attribute.
    /// [`CSSMatrixComponent.matrix`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMatrixComponent/matrix)
    pub fn set_matrix(&mut self, value: &DOMMatrix) {
        self.inner.set("matrix", value);
    }
}

impl CSSMatrixComponent {
    /// The `new CSSMatrixComponent(..)` constructor, creating a new CSSMatrixComponent instance
    pub fn new(matrix: &DOMMatrixReadOnly) -> CSSMatrixComponent {
        Self {
            inner: Any::global("CSSMatrixComponent")
                .new(&[matrix.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}

impl CSSMatrixComponent {
    /// The `new CSSMatrixComponent(..)` constructor, creating a new CSSMatrixComponent instance
    pub fn new_with_options(
        matrix: &DOMMatrixReadOnly,
        options: &CSSMatrixComponentOptions,
    ) -> CSSMatrixComponent {
        Self {
            inner: Any::global("CSSMatrixComponent")
                .new(&[matrix.into(), options.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
