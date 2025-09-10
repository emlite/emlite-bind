use super::*;

/// The SVGUnitTypes class.
/// [`SVGUnitTypes`](https://developer.mozilla.org/en-US/docs/Web/API/SVGUnitTypes)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGUnitTypes {
    inner: Any,
}

impl FromVal for SVGUnitTypes {
    fn from_val(v: &Any) -> Self {
        SVGUnitTypes {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGUnitTypes {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGUnitTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGUnitTypes {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGUnitTypes {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGUnitTypes> for Any {
    fn from(s: SVGUnitTypes) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGUnitTypes> for Any {
    fn from(s: &SVGUnitTypes) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGUnitTypes);
