use super::*;




/// The LayoutChild class.
/// [`LayoutChild`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutChild)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutChild {
    inner: Any,
}

impl FromVal for LayoutChild {
    fn from_val(v: &Any) -> Self {
        LayoutChild { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for LayoutChild {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LayoutChild {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LayoutChild {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LayoutChild {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<LayoutChild> for Any {
    fn from(s: LayoutChild) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LayoutChild> for Any {
    fn from(s: &LayoutChild) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(LayoutChild);


impl LayoutChild {
    /// Getter of the `styleMap` attribute.
    /// [`LayoutChild.styleMap`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutChild/styleMap)
    pub fn style_map(&self) -> StylePropertyMapReadOnly {
        self.inner.get("styleMap").as_::<StylePropertyMapReadOnly>()
    }

}
impl LayoutChild {
    /// The intrinsicSizes method.
    /// [`LayoutChild.intrinsicSizes`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutChild/intrinsicSizes)
    pub fn intrinsic_sizes(&self, ) -> Promise<IntrinsicSizes> {
        self.inner.call("intrinsicSizes", &[]).as_::<Promise<IntrinsicSizes>>()
    }
}
impl LayoutChild {
    /// The layoutNextFragment method.
    /// [`LayoutChild.layoutNextFragment`](https://developer.mozilla.org/en-US/docs/Web/API/LayoutChild/layoutNextFragment)
    pub fn layout_next_fragment(&self, constraints: &LayoutConstraintsOptions, break_token: &ChildBreakToken) -> Promise<LayoutFragment> {
        self.inner.call("layoutNextFragment", &[constraints.into(), break_token.into(), ]).as_::<Promise<LayoutFragment>>()
    }
}
