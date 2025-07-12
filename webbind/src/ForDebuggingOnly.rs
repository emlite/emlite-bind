use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for ForDebuggingOnly {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ForDebuggingOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ForDebuggingOnly> for emlite::Val {
    fn from(s: ForDebuggingOnly) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ForDebuggingOnly {
    pub fn report_ad_auction_win(&self, url: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("reportAdAuctionWin", &[url.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl ForDebuggingOnly {
    pub fn report_ad_auction_loss(&self, url: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("reportAdAuctionLoss", &[url.into()])
            .as_::<jsbind::Undefined>()
    }
}
