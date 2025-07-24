use super::*;

/// The ForDebuggingOnly class.
/// [`ForDebuggingOnly`](https://developer.mozilla.org/en-US/docs/Web/API/ForDebuggingOnly)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ForDebuggingOnly {
    inner: Any,
}
impl FromVal for ForDebuggingOnly {
    fn from_val(v: &Any) -> Self {
        ForDebuggingOnly {
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
impl core::ops::Deref for ForDebuggingOnly {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ForDebuggingOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ForDebuggingOnly {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ForDebuggingOnly {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ForDebuggingOnly> for Any {
    fn from(s: ForDebuggingOnly) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ForDebuggingOnly> for Any {
    fn from(s: &ForDebuggingOnly) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ForDebuggingOnly);

impl ForDebuggingOnly {
    /// The reportAdAuctionWin method.
    /// [`ForDebuggingOnly.reportAdAuctionWin`](https://developer.mozilla.org/en-US/docs/Web/API/ForDebuggingOnly/reportAdAuctionWin)
    pub fn report_ad_auction_win(&self, url: &USVString) -> Undefined {
        self.inner
            .call("reportAdAuctionWin", &[url.into()])
            .as_::<Undefined>()
    }
}
impl ForDebuggingOnly {
    /// The reportAdAuctionLoss method.
    /// [`ForDebuggingOnly.reportAdAuctionLoss`](https://developer.mozilla.org/en-US/docs/Web/API/ForDebuggingOnly/reportAdAuctionLoss)
    pub fn report_ad_auction_loss(&self, url: &USVString) -> Undefined {
        self.inner
            .call("reportAdAuctionLoss", &[url.into()])
            .as_::<Undefined>()
    }
}
