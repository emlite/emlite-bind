use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IsInputPendingOptions {
    inner: emlite::Val,
}
impl FromVal for IsInputPendingOptions {
    fn from_val(v: &emlite::Val) -> Self {
        IsInputPendingOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IsInputPendingOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IsInputPendingOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IsInputPendingOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IsInputPendingOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IsInputPendingOptions> for emlite::Val {
    fn from(s: IsInputPendingOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&IsInputPendingOptions> for emlite::Val {
    fn from(s: &IsInputPendingOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl IsInputPendingOptions {
    pub fn include_continuous(&self) -> bool {
        self.inner.get("includeContinuous").as_::<bool>()
    }

    pub fn set_include_continuous(&mut self, value: bool) {
        self.inner.set("includeContinuous", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Scheduling {
    inner: emlite::Val,
}
impl FromVal for Scheduling {
    fn from_val(v: &emlite::Val) -> Self {
        Scheduling {
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
impl core::ops::Deref for Scheduling {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Scheduling {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Scheduling {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Scheduling {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Scheduling> for emlite::Val {
    fn from(s: Scheduling) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Scheduling> for emlite::Val {
    fn from(s: &Scheduling) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Scheduling);

impl Scheduling {
    pub fn is_input_pending0(&self) -> bool {
        self.inner.call("isInputPending", &[]).as_::<bool>()
    }

    pub fn is_input_pending1(&self, is_input_pending_options: IsInputPendingOptions) -> bool {
        self.inner
            .call("isInputPending", &[is_input_pending_options.into()])
            .as_::<bool>()
    }
}
