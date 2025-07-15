use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FragmentResult {
    inner: emlite::Val,
}
impl FromVal for FragmentResult {
    fn from_val(v: &emlite::Val) -> Self {
        FragmentResult {
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
impl core::ops::Deref for FragmentResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FragmentResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FragmentResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FragmentResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FragmentResult> for emlite::Val {
    fn from(s: FragmentResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FragmentResult);

impl FragmentResult {
    pub fn new0() -> FragmentResult {
        Self {
            inner: emlite::Val::global("FragmentResult")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(options: Any) -> FragmentResult {
        Self {
            inner: emlite::Val::global("FragmentResult")
                .new(&[options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl FragmentResult {
    pub fn inline_size(&self) -> f64 {
        self.inner.get("inlineSize").as_::<f64>()
    }
}
impl FragmentResult {
    pub fn block_size(&self) -> f64 {
        self.inner.get("blockSize").as_::<f64>()
    }
}
