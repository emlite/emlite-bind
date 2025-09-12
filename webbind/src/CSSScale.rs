use super::*;

/// The CSSScale class.
/// [`CSSScale`](https://developer.mozilla.org/en-US/docs/Web/API/CSSScale)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSScale {
    inner: CSSTransformComponent,
}

impl FromVal for CSSScale {
    fn from_val(v: &Any) -> Self {
        CSSScale {
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

impl core::ops::Deref for CSSScale {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSScale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSScale {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSScale {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSScale> for Any {
    fn from(s: CSSScale) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSScale> for Any {
    fn from(s: &CSSScale) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSScale);

impl CSSScale {
    /// Getter of the `x` attribute.
    /// [`CSSScale.x`](https://developer.mozilla.org/en-US/docs/Web/API/CSSScale/x)
    pub fn x(&self) -> Any {
        self.inner.get("x").as_::<Any>()
    }

    /// Setter of the `x` attribute.
    /// [`CSSScale.x`](https://developer.mozilla.org/en-US/docs/Web/API/CSSScale/x)
    pub fn set_x(&mut self, value: &Any) {
        self.inner.set("x", value);
    }
}
impl CSSScale {
    /// Getter of the `y` attribute.
    /// [`CSSScale.y`](https://developer.mozilla.org/en-US/docs/Web/API/CSSScale/y)
    pub fn y(&self) -> Any {
        self.inner.get("y").as_::<Any>()
    }

    /// Setter of the `y` attribute.
    /// [`CSSScale.y`](https://developer.mozilla.org/en-US/docs/Web/API/CSSScale/y)
    pub fn set_y(&mut self, value: &Any) {
        self.inner.set("y", value);
    }
}
impl CSSScale {
    /// Getter of the `z` attribute.
    /// [`CSSScale.z`](https://developer.mozilla.org/en-US/docs/Web/API/CSSScale/z)
    pub fn z(&self) -> Any {
        self.inner.get("z").as_::<Any>()
    }

    /// Setter of the `z` attribute.
    /// [`CSSScale.z`](https://developer.mozilla.org/en-US/docs/Web/API/CSSScale/z)
    pub fn set_z(&mut self, value: &Any) {
        self.inner.set("z", value);
    }
}

impl CSSScale {
    /// The `new CSSScale(..)` constructor, creating a new CSSScale instance
    pub fn new(x: &Any, y: &Any) -> CSSScale {
        Self {
            inner: Any::global("CSSScale")
                .new(&[x.into(), y.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}

impl CSSScale {
    /// The `new CSSScale(..)` constructor, creating a new CSSScale instance
    pub fn new_with_z(x: &Any, y: &Any, z: &Any) -> CSSScale {
        Self {
            inner: Any::global("CSSScale")
                .new(&[x.into(), y.into(), z.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
