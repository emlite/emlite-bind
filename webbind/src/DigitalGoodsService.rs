use super::*;

#[derive(Clone, Debug)]
pub struct ItemDetails {
    inner: emlite::Val,
}
impl FromVal for ItemDetails {
    fn from_val(v: &emlite::Val) -> Self {
        ItemDetails { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ItemDetails {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ItemDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ItemDetails> for emlite::Val {
    fn from(s: ItemDetails) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ItemDetails {
    pub fn item_id(&self) -> jsbind::DOMString {
        self.inner.get("itemId").as_::<jsbind::DOMString>()
    }

    pub fn set_item_id(&mut self, value: jsbind::DOMString) {
        self.inner.set("itemId", value);
    }
}
impl ItemDetails {
    pub fn title(&self) -> jsbind::DOMString {
        self.inner.get("title").as_::<jsbind::DOMString>()
    }

    pub fn set_title(&mut self, value: jsbind::DOMString) {
        self.inner.set("title", value);
    }
}
impl ItemDetails {
    pub fn price(&self) -> jsbind::Any {
        self.inner.get("price").as_::<jsbind::Any>()
    }

    pub fn set_price(&mut self, value: jsbind::Any) {
        self.inner.set("price", value);
    }
}
impl ItemDetails {
    pub fn type_(&self) -> ItemType {
        self.inner.get("type").as_::<ItemType>()
    }

    pub fn set_type_(&mut self, value: ItemType) {
        self.inner.set("type", value);
    }
}
impl ItemDetails {
    pub fn description(&self) -> jsbind::DOMString {
        self.inner.get("description").as_::<jsbind::DOMString>()
    }

    pub fn set_description(&mut self, value: jsbind::DOMString) {
        self.inner.set("description", value);
    }
}
impl ItemDetails {
    pub fn icon_ur_ls(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("iconURLs")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_icon_ur_ls(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
        self.inner.set("iconURLs", value);
    }
}
impl ItemDetails {
    pub fn subscription_period(&self) -> jsbind::DOMString {
        self.inner
            .get("subscriptionPeriod")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_subscription_period(&mut self, value: jsbind::DOMString) {
        self.inner.set("subscriptionPeriod", value);
    }
}
impl ItemDetails {
    pub fn free_trial_period(&self) -> jsbind::DOMString {
        self.inner.get("freeTrialPeriod").as_::<jsbind::DOMString>()
    }

    pub fn set_free_trial_period(&mut self, value: jsbind::DOMString) {
        self.inner.set("freeTrialPeriod", value);
    }
}
impl ItemDetails {
    pub fn introductory_price(&self) -> jsbind::Any {
        self.inner.get("introductoryPrice").as_::<jsbind::Any>()
    }

    pub fn set_introductory_price(&mut self, value: jsbind::Any) {
        self.inner.set("introductoryPrice", value);
    }
}
impl ItemDetails {
    pub fn introductory_price_period(&self) -> jsbind::DOMString {
        self.inner
            .get("introductoryPricePeriod")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_introductory_price_period(&mut self, value: jsbind::DOMString) {
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
#[derive(Clone, Debug)]
pub struct PurchaseDetails {
    inner: emlite::Val,
}
impl FromVal for PurchaseDetails {
    fn from_val(v: &emlite::Val) -> Self {
        PurchaseDetails { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PurchaseDetails {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PurchaseDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PurchaseDetails> for emlite::Val {
    fn from(s: PurchaseDetails) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PurchaseDetails {
    pub fn item_id(&self) -> jsbind::DOMString {
        self.inner.get("itemId").as_::<jsbind::DOMString>()
    }

    pub fn set_item_id(&mut self, value: jsbind::DOMString) {
        self.inner.set("itemId", value);
    }
}
impl PurchaseDetails {
    pub fn purchase_token(&self) -> jsbind::DOMString {
        self.inner.get("purchaseToken").as_::<jsbind::DOMString>()
    }

    pub fn set_purchase_token(&mut self, value: jsbind::DOMString) {
        self.inner.set("purchaseToken", value);
    }
}
#[derive(Clone, Debug)]
pub struct DigitalGoodsService {
    inner: emlite::Val,
}
impl FromVal for DigitalGoodsService {
    fn from_val(v: &emlite::Val) -> Self {
        DigitalGoodsService {
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
impl std::ops::Deref for DigitalGoodsService {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DigitalGoodsService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DigitalGoodsService> for emlite::Val {
    fn from(s: DigitalGoodsService) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DigitalGoodsService {
    pub fn get_details(&self, item_ids: jsbind::Sequence<jsbind::DOMString>) -> jsbind::Promise {
        self.inner
            .call("getDetails", &[item_ids.into()])
            .as_::<jsbind::Promise>()
    }
}
impl DigitalGoodsService {
    pub fn list_purchases(&self) -> jsbind::Promise {
        self.inner
            .call("listPurchases", &[])
            .as_::<jsbind::Promise>()
    }
}
impl DigitalGoodsService {
    pub fn list_purchase_history(&self) -> jsbind::Promise {
        self.inner
            .call("listPurchaseHistory", &[])
            .as_::<jsbind::Promise>()
    }
}
impl DigitalGoodsService {
    pub fn consume(&self, purchase_token: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("consume", &[purchase_token.into()])
            .as_::<jsbind::Promise>()
    }
}
