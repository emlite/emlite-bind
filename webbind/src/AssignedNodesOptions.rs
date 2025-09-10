use super::*;

/// The AssignedNodesOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AssignedNodesOptions {
    inner: Any,
}

impl FromVal for AssignedNodesOptions {
    fn from_val(v: &Any) -> Self {
        AssignedNodesOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AssignedNodesOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AssignedNodesOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AssignedNodesOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AssignedNodesOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AssignedNodesOptions> for Any {
    fn from(s: AssignedNodesOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AssignedNodesOptions> for Any {
    fn from(s: &AssignedNodesOptions) -> Any {
        s.inner.clone()
    }
}

impl AssignedNodesOptions {
    /// Getter of the `flatten` attribute.
    pub fn flatten(&self) -> bool {
        self.inner.get("flatten").as_::<bool>()
    }

    /// Setter of the `flatten` attribute.
    pub fn set_flatten(&mut self, value: bool) {
        self.inner.set("flatten", value);
    }
}
