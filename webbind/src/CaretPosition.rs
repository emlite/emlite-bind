use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaretPosition {
    inner: emlite::Val,
}
impl FromVal for CaretPosition {
    fn from_val(v: &emlite::Val) -> Self {
        CaretPosition {
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
impl core::ops::Deref for CaretPosition {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CaretPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CaretPosition {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CaretPosition {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CaretPosition> for emlite::Val {
    fn from(s: CaretPosition) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CaretPosition> for emlite::Val {
    fn from(s: &CaretPosition) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CaretPosition);

impl CaretPosition {
    pub fn offset_node(&self) -> Node {
        self.inner.get("offsetNode").as_::<Node>()
    }
}
impl CaretPosition {
    pub fn offset(&self) -> u32 {
        self.inner.get("offset").as_::<u32>()
    }
}
impl CaretPosition {
    pub fn get_client_rect(&self) -> DOMRect {
        self.inner.call("getClientRect", &[]).as_::<DOMRect>()
    }
}
