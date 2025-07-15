use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ForDebuggingOnly {
    inner: emlite::Val,
}
impl FromVal for ForDebuggingOnly {
    fn from_val(v: &emlite::Val) -> Self {
        ForDebuggingOnly {
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
impl core::ops::Deref for ForDebuggingOnly {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ForDebuggingOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ForDebuggingOnly {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ForDebuggingOnly {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ForDebuggingOnly> for emlite::Val {
    fn from(s: ForDebuggingOnly) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ForDebuggingOnly> for emlite::Val {
    fn from(s: &ForDebuggingOnly) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ForDebuggingOnly);

impl ForDebuggingOnly {
    pub fn report_ad_auction_win(&self, url: USVString) -> Undefined {
        self.inner
            .call("reportAdAuctionWin", &[url.into()])
            .as_::<Undefined>()
    }
}
impl ForDebuggingOnly {
    pub fn report_ad_auction_loss(&self, url: USVString) -> Undefined {
        self.inner
            .call("reportAdAuctionLoss", &[url.into()])
            .as_::<Undefined>()
    }
}
