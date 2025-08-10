use super::*;

/// The SpatialNavigationSearchOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpatialNavigationSearchOptions {
    inner: Any,
}

impl FromVal for SpatialNavigationSearchOptions {
    fn from_val(v: &Any) -> Self {
        SpatialNavigationSearchOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SpatialNavigationSearchOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpatialNavigationSearchOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpatialNavigationSearchOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpatialNavigationSearchOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SpatialNavigationSearchOptions> for Any {
    fn from(s: SpatialNavigationSearchOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpatialNavigationSearchOptions> for Any {
    fn from(s: &SpatialNavigationSearchOptions) -> Any {
        s.inner.clone()
    }
}

impl SpatialNavigationSearchOptions {
    /// Getter of the `candidates` attribute.
    pub fn candidates(&self) -> TypedArray<Node> {
        self.inner.get("candidates").as_::<TypedArray<Node>>()
    }

    /// Setter of the `candidates` attribute.
    pub fn set_candidates(&mut self, value: &TypedArray<Node>) {
        self.inner.set("candidates", value);
    }
}
impl SpatialNavigationSearchOptions {
    /// Getter of the `container` attribute.
    pub fn container(&self) -> Node {
        self.inner.get("container").as_::<Node>()
    }

    /// Setter of the `container` attribute.
    pub fn set_container(&mut self, value: &Node) {
        self.inner.set("container", value);
    }
}
