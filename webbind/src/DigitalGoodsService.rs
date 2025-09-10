use super::*;

/// The DigitalGoodsService class.
/// [`DigitalGoodsService`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalGoodsService)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DigitalGoodsService {
    inner: Any,
}

impl FromVal for DigitalGoodsService {
    fn from_val(v: &Any) -> Self {
        DigitalGoodsService {
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

impl core::ops::Deref for DigitalGoodsService {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DigitalGoodsService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DigitalGoodsService {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DigitalGoodsService {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DigitalGoodsService> for Any {
    fn from(s: DigitalGoodsService) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DigitalGoodsService> for Any {
    fn from(s: &DigitalGoodsService) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DigitalGoodsService);

impl DigitalGoodsService {
    /// The getDetails method.
    /// [`DigitalGoodsService.getDetails`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalGoodsService/getDetails)
    pub fn get_details(&self, item_ids: &TypedArray<JsString>) -> Promise<TypedArray<ItemDetails>> {
        self.inner
            .call("getDetails", &[item_ids.into()])
            .as_::<Promise<TypedArray<ItemDetails>>>()
    }
}
impl DigitalGoodsService {
    /// The listPurchases method.
    /// [`DigitalGoodsService.listPurchases`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalGoodsService/listPurchases)
    pub fn list_purchases(&self) -> Promise<TypedArray<PurchaseDetails>> {
        self.inner
            .call("listPurchases", &[])
            .as_::<Promise<TypedArray<PurchaseDetails>>>()
    }
}
impl DigitalGoodsService {
    /// The listPurchaseHistory method.
    /// [`DigitalGoodsService.listPurchaseHistory`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalGoodsService/listPurchaseHistory)
    pub fn list_purchase_history(&self) -> Promise<TypedArray<PurchaseDetails>> {
        self.inner
            .call("listPurchaseHistory", &[])
            .as_::<Promise<TypedArray<PurchaseDetails>>>()
    }
}
impl DigitalGoodsService {
    /// The consume method.
    /// [`DigitalGoodsService.consume`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalGoodsService/consume)
    pub fn consume(&self, purchase_token: &JsString) -> Promise<Undefined> {
        self.inner
            .call("consume", &[purchase_token.into()])
            .as_::<Promise<Undefined>>()
    }
}
