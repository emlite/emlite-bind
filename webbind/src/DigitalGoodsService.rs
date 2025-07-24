use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ItemDetails {
    inner: Any,
}
impl FromVal for ItemDetails {
    fn from_val(v: &Any) -> Self {
        ItemDetails { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ItemDetails {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ItemDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ItemDetails {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ItemDetails {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ItemDetails> for Any {
    fn from(s: ItemDetails) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ItemDetails> for Any {
    fn from(s: &ItemDetails) -> Any {
        s.inner.clone()
    }
}

impl ItemDetails {
    pub fn item_id(&self) -> DOMString {
        self.inner.get("itemId").as_::<DOMString>()
    }

    pub fn set_item_id(&mut self, value: &DOMString) {
        self.inner.set("itemId", value);
    }
}
impl ItemDetails {
    pub fn title(&self) -> DOMString {
        self.inner.get("title").as_::<DOMString>()
    }

    pub fn set_title(&mut self, value: &DOMString) {
        self.inner.set("title", value);
    }
}
impl ItemDetails {
    pub fn price(&self) -> Any {
        self.inner.get("price").as_::<Any>()
    }

    pub fn set_price(&mut self, value: &Any) {
        self.inner.set("price", value);
    }
}
impl ItemDetails {
    pub fn type_(&self) -> ItemType {
        self.inner.get("type").as_::<ItemType>()
    }

    pub fn set_type_(&mut self, value: &ItemType) {
        self.inner.set("type", value);
    }
}
impl ItemDetails {
    pub fn description(&self) -> DOMString {
        self.inner.get("description").as_::<DOMString>()
    }

    pub fn set_description(&mut self, value: &DOMString) {
        self.inner.set("description", value);
    }
}
impl ItemDetails {
    pub fn icon_ur_ls(&self) -> Sequence<DOMString> {
        self.inner.get("iconURLs").as_::<Sequence<DOMString>>()
    }

    pub fn set_icon_ur_ls(&mut self, value: &Sequence<DOMString>) {
        self.inner.set("iconURLs", value);
    }
}
impl ItemDetails {
    pub fn subscription_period(&self) -> DOMString {
        self.inner.get("subscriptionPeriod").as_::<DOMString>()
    }

    pub fn set_subscription_period(&mut self, value: &DOMString) {
        self.inner.set("subscriptionPeriod", value);
    }
}
impl ItemDetails {
    pub fn free_trial_period(&self) -> DOMString {
        self.inner.get("freeTrialPeriod").as_::<DOMString>()
    }

    pub fn set_free_trial_period(&mut self, value: &DOMString) {
        self.inner.set("freeTrialPeriod", value);
    }
}
impl ItemDetails {
    pub fn introductory_price(&self) -> Any {
        self.inner.get("introductoryPrice").as_::<Any>()
    }

    pub fn set_introductory_price(&mut self, value: &Any) {
        self.inner.set("introductoryPrice", value);
    }
}
impl ItemDetails {
    pub fn introductory_price_period(&self) -> DOMString {
        self.inner.get("introductoryPricePeriod").as_::<DOMString>()
    }

    pub fn set_introductory_price_period(&mut self, value: &DOMString) {
        self.inner.set("introductoryPricePeriod", value);
    }
}
impl ItemDetails {
    pub fn introductory_price_cycles(&self) -> u64 {
        self.inner.get("introductoryPriceCycles").as_::<u64>()
    }

    pub fn set_introductory_price_cycles(&mut self, value: u64) {
        self.inner.set("introductoryPriceCycles", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PurchaseDetails {
    inner: Any,
}
impl FromVal for PurchaseDetails {
    fn from_val(v: &Any) -> Self {
        PurchaseDetails { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PurchaseDetails {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PurchaseDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PurchaseDetails {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PurchaseDetails {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PurchaseDetails> for Any {
    fn from(s: PurchaseDetails) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PurchaseDetails> for Any {
    fn from(s: &PurchaseDetails) -> Any {
        s.inner.clone()
    }
}

impl PurchaseDetails {
    pub fn item_id(&self) -> DOMString {
        self.inner.get("itemId").as_::<DOMString>()
    }

    pub fn set_item_id(&mut self, value: &DOMString) {
        self.inner.set("itemId", value);
    }
}
impl PurchaseDetails {
    pub fn purchase_token(&self) -> DOMString {
        self.inner.get("purchaseToken").as_::<DOMString>()
    }

    pub fn set_purchase_token(&mut self, value: &DOMString) {
        self.inner.set("purchaseToken", value);
    }
}
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
    pub fn get_details(&self, item_ids: &Sequence<DOMString>) -> Promise<Sequence<ItemDetails>> {
        self.inner
            .call("getDetails", &[item_ids.into()])
            .as_::<Promise<Sequence<ItemDetails>>>()
    }
}
impl DigitalGoodsService {
    /// The listPurchases method.
    /// [`DigitalGoodsService.listPurchases`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalGoodsService/listPurchases)
    pub fn list_purchases(&self) -> Promise<Sequence<PurchaseDetails>> {
        self.inner
            .call("listPurchases", &[])
            .as_::<Promise<Sequence<PurchaseDetails>>>()
    }
}
impl DigitalGoodsService {
    /// The listPurchaseHistory method.
    /// [`DigitalGoodsService.listPurchaseHistory`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalGoodsService/listPurchaseHistory)
    pub fn list_purchase_history(&self) -> Promise<Sequence<PurchaseDetails>> {
        self.inner
            .call("listPurchaseHistory", &[])
            .as_::<Promise<Sequence<PurchaseDetails>>>()
    }
}
impl DigitalGoodsService {
    /// The consume method.
    /// [`DigitalGoodsService.consume`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalGoodsService/consume)
    pub fn consume(&self, purchase_token: &DOMString) -> Promise<Undefined> {
        self.inner
            .call("consume", &[purchase_token.into()])
            .as_::<Promise<Undefined>>()
    }
}
