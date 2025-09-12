use super::*;

/// The CSSPerspective class.
/// [`CSSPerspective`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPerspective)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPerspective {
    inner: CSSTransformComponent,
}

impl FromVal for CSSPerspective {
    fn from_val(v: &Any) -> Self {
        CSSPerspective {
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

impl core::ops::Deref for CSSPerspective {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSPerspective {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSPerspective {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSPerspective {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSPerspective> for Any {
    fn from(s: CSSPerspective) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSPerspective> for Any {
    fn from(s: &CSSPerspective) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSPerspective);

impl CSSPerspective {
    /// Getter of the `length` attribute.
    /// [`CSSPerspective.length`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPerspective/length)
    pub fn length(&self) -> Any {
        self.inner.get("length").as_::<Any>()
    }

    /// Setter of the `length` attribute.
    /// [`CSSPerspective.length`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPerspective/length)
    pub fn set_length(&mut self, value: &Any) {
        self.inner.set("length", value);
    }
}

impl CSSPerspective {
    /// The `new CSSPerspective(..)` constructor, creating a new CSSPerspective instance
    pub fn new(length: &Any) -> CSSPerspective {
        Self {
            inner: Any::global("CSSPerspective")
                .new(&[length.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
