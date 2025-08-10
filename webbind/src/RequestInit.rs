use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RequestInit {
    inner: Any,
}
impl FromVal for RequestInit {
    fn from_val(v: &Any) -> Self {
        RequestInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RequestInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RequestInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RequestInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RequestInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RequestInit> for Any {
    fn from(s: RequestInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RequestInit> for Any {
    fn from(s: &RequestInit) -> Any {
        s.inner.clone()
    }
}

impl RequestInit {
    pub fn ad_auction_headers(&self) -> bool {
        self.inner.get("adAuctionHeaders").as_::<bool>()
    }

    pub fn set_ad_auction_headers(&mut self, value: bool) {
        self.inner.set("adAuctionHeaders", value);
    }
}
