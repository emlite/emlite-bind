use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentMethodData {
    inner: Any,
}
impl FromVal for PaymentMethodData {
    fn from_val(v: &Any) -> Self {
        PaymentMethodData { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentMethodData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentMethodData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentMethodData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentMethodData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentMethodData> for Any {
    fn from(s: PaymentMethodData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentMethodData> for Any {
    fn from(s: &PaymentMethodData) -> Any {
        s.inner.clone()
    }
}

impl PaymentMethodData {
    pub fn supported_methods(&self) -> DOMString {
        self.inner.get("supportedMethods").as_::<DOMString>()
    }

    pub fn set_supported_methods(&mut self, value: &DOMString) {
        self.inner.set("supportedMethods", value);
    }
}
impl PaymentMethodData {
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }

    pub fn set_data(&mut self, value: &Object) {
        self.inner.set("data", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentDetailsModifier {
    inner: Any,
}
impl FromVal for PaymentDetailsModifier {
    fn from_val(v: &Any) -> Self {
        PaymentDetailsModifier { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentDetailsModifier {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentDetailsModifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentDetailsModifier {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentDetailsModifier {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentDetailsModifier> for Any {
    fn from(s: PaymentDetailsModifier) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentDetailsModifier> for Any {
    fn from(s: &PaymentDetailsModifier) -> Any {
        s.inner.clone()
    }
}

impl PaymentDetailsModifier {
    pub fn supported_methods(&self) -> DOMString {
        self.inner.get("supportedMethods").as_::<DOMString>()
    }

    pub fn set_supported_methods(&mut self, value: &DOMString) {
        self.inner.set("supportedMethods", value);
    }
}
impl PaymentDetailsModifier {
    pub fn total(&self) -> Any {
        self.inner.get("total").as_::<Any>()
    }

    pub fn set_total(&mut self, value: &Any) {
        self.inner.set("total", value);
    }
}
impl PaymentDetailsModifier {
    pub fn additional_display_items(&self) -> Sequence<Any> {
        self.inner
            .get("additionalDisplayItems")
            .as_::<Sequence<Any>>()
    }

    pub fn set_additional_display_items(&mut self, value: &Sequence<Any>) {
        self.inner.set("additionalDisplayItems", value);
    }
}
impl PaymentDetailsModifier {
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }

    pub fn set_data(&mut self, value: &Object) {
        self.inner.set("data", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentShippingOption {
    inner: Any,
}
impl FromVal for PaymentShippingOption {
    fn from_val(v: &Any) -> Self {
        PaymentShippingOption { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentShippingOption {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentShippingOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentShippingOption {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentShippingOption {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentShippingOption> for Any {
    fn from(s: PaymentShippingOption) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentShippingOption> for Any {
    fn from(s: &PaymentShippingOption) -> Any {
        s.inner.clone()
    }
}

impl PaymentShippingOption {
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }

    pub fn set_id(&mut self, value: &DOMString) {
        self.inner.set("id", value);
    }
}
impl PaymentShippingOption {
    pub fn label(&self) -> DOMString {
        self.inner.get("label").as_::<DOMString>()
    }

    pub fn set_label(&mut self, value: &DOMString) {
        self.inner.set("label", value);
    }
}
impl PaymentShippingOption {
    pub fn amount(&self) -> Any {
        self.inner.get("amount").as_::<Any>()
    }

    pub fn set_amount(&mut self, value: &Any) {
        self.inner.set("amount", value);
    }
}
impl PaymentShippingOption {
    pub fn selected(&self) -> bool {
        self.inner.get("selected").as_::<bool>()
    }

    pub fn set_selected(&mut self, value: bool) {
        self.inner.set("selected", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentRequestDetailsUpdate {
    inner: Any,
}
impl FromVal for PaymentRequestDetailsUpdate {
    fn from_val(v: &Any) -> Self {
        PaymentRequestDetailsUpdate { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentRequestDetailsUpdate {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentRequestDetailsUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentRequestDetailsUpdate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentRequestDetailsUpdate {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentRequestDetailsUpdate> for Any {
    fn from(s: PaymentRequestDetailsUpdate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentRequestDetailsUpdate> for Any {
    fn from(s: &PaymentRequestDetailsUpdate) -> Any {
        s.inner.clone()
    }
}

impl PaymentRequestDetailsUpdate {
    pub fn error(&self) -> DOMString {
        self.inner.get("error").as_::<DOMString>()
    }

    pub fn set_error(&mut self, value: &DOMString) {
        self.inner.set("error", value);
    }
}
impl PaymentRequestDetailsUpdate {
    pub fn total(&self) -> Any {
        self.inner.get("total").as_::<Any>()
    }

    pub fn set_total(&mut self, value: &Any) {
        self.inner.set("total", value);
    }
}
impl PaymentRequestDetailsUpdate {
    pub fn modifiers(&self) -> Sequence<PaymentDetailsModifier> {
        self.inner
            .get("modifiers")
            .as_::<Sequence<PaymentDetailsModifier>>()
    }

    pub fn set_modifiers(&mut self, value: &Sequence<PaymentDetailsModifier>) {
        self.inner.set("modifiers", value);
    }
}
impl PaymentRequestDetailsUpdate {
    pub fn shipping_options(&self) -> Sequence<PaymentShippingOption> {
        self.inner
            .get("shippingOptions")
            .as_::<Sequence<PaymentShippingOption>>()
    }

    pub fn set_shipping_options(&mut self, value: &Sequence<PaymentShippingOption>) {
        self.inner.set("shippingOptions", value);
    }
}
impl PaymentRequestDetailsUpdate {
    pub fn payment_method_errors(&self) -> Object {
        self.inner.get("paymentMethodErrors").as_::<Object>()
    }

    pub fn set_payment_method_errors(&mut self, value: &Object) {
        self.inner.set("paymentMethodErrors", value);
    }
}
impl PaymentRequestDetailsUpdate {
    pub fn shipping_address_errors(&self) -> Any {
        self.inner.get("shippingAddressErrors").as_::<Any>()
    }

    pub fn set_shipping_address_errors(&mut self, value: &Any) {
        self.inner.set("shippingAddressErrors", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AddressInit {
    inner: Any,
}
impl FromVal for AddressInit {
    fn from_val(v: &Any) -> Self {
        AddressInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AddressInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AddressInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AddressInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AddressInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AddressInit> for Any {
    fn from(s: AddressInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AddressInit> for Any {
    fn from(s: &AddressInit) -> Any {
        s.inner.clone()
    }
}

impl AddressInit {
    pub fn country(&self) -> DOMString {
        self.inner.get("country").as_::<DOMString>()
    }

    pub fn set_country(&mut self, value: &DOMString) {
        self.inner.set("country", value);
    }
}
impl AddressInit {
    pub fn address_line(&self) -> Sequence<DOMString> {
        self.inner.get("addressLine").as_::<Sequence<DOMString>>()
    }

    pub fn set_address_line(&mut self, value: &Sequence<DOMString>) {
        self.inner.set("addressLine", value);
    }
}
impl AddressInit {
    pub fn region(&self) -> DOMString {
        self.inner.get("region").as_::<DOMString>()
    }

    pub fn set_region(&mut self, value: &DOMString) {
        self.inner.set("region", value);
    }
}
impl AddressInit {
    pub fn city(&self) -> DOMString {
        self.inner.get("city").as_::<DOMString>()
    }

    pub fn set_city(&mut self, value: &DOMString) {
        self.inner.set("city", value);
    }
}
impl AddressInit {
    pub fn dependent_locality(&self) -> DOMString {
        self.inner.get("dependentLocality").as_::<DOMString>()
    }

    pub fn set_dependent_locality(&mut self, value: &DOMString) {
        self.inner.set("dependentLocality", value);
    }
}
impl AddressInit {
    pub fn postal_code(&self) -> DOMString {
        self.inner.get("postalCode").as_::<DOMString>()
    }

    pub fn set_postal_code(&mut self, value: &DOMString) {
        self.inner.set("postalCode", value);
    }
}
impl AddressInit {
    pub fn sorting_code(&self) -> DOMString {
        self.inner.get("sortingCode").as_::<DOMString>()
    }

    pub fn set_sorting_code(&mut self, value: &DOMString) {
        self.inner.set("sortingCode", value);
    }
}
impl AddressInit {
    pub fn organization(&self) -> DOMString {
        self.inner.get("organization").as_::<DOMString>()
    }

    pub fn set_organization(&mut self, value: &DOMString) {
        self.inner.set("organization", value);
    }
}
impl AddressInit {
    pub fn recipient(&self) -> DOMString {
        self.inner.get("recipient").as_::<DOMString>()
    }

    pub fn set_recipient(&mut self, value: &DOMString) {
        self.inner.set("recipient", value);
    }
}
impl AddressInit {
    pub fn phone(&self) -> DOMString {
        self.inner.get("phone").as_::<DOMString>()
    }

    pub fn set_phone(&mut self, value: &DOMString) {
        self.inner.set("phone", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentHandlerResponse {
    inner: Any,
}
impl FromVal for PaymentHandlerResponse {
    fn from_val(v: &Any) -> Self {
        PaymentHandlerResponse { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentHandlerResponse {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentHandlerResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentHandlerResponse {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentHandlerResponse {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentHandlerResponse> for Any {
    fn from(s: PaymentHandlerResponse) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentHandlerResponse> for Any {
    fn from(s: &PaymentHandlerResponse) -> Any {
        s.inner.clone()
    }
}

impl PaymentHandlerResponse {
    pub fn method_name(&self) -> DOMString {
        self.inner.get("methodName").as_::<DOMString>()
    }

    pub fn set_method_name(&mut self, value: &DOMString) {
        self.inner.set("methodName", value);
    }
}
impl PaymentHandlerResponse {
    pub fn details(&self) -> Object {
        self.inner.get("details").as_::<Object>()
    }

    pub fn set_details(&mut self, value: &Object) {
        self.inner.set("details", value);
    }
}
impl PaymentHandlerResponse {
    pub fn payer_name(&self) -> DOMString {
        self.inner.get("payerName").as_::<DOMString>()
    }

    pub fn set_payer_name(&mut self, value: &DOMString) {
        self.inner.set("payerName", value);
    }
}
impl PaymentHandlerResponse {
    pub fn payer_email(&self) -> DOMString {
        self.inner.get("payerEmail").as_::<DOMString>()
    }

    pub fn set_payer_email(&mut self, value: &DOMString) {
        self.inner.set("payerEmail", value);
    }
}
impl PaymentHandlerResponse {
    pub fn payer_phone(&self) -> DOMString {
        self.inner.get("payerPhone").as_::<DOMString>()
    }

    pub fn set_payer_phone(&mut self, value: &DOMString) {
        self.inner.set("payerPhone", value);
    }
}
impl PaymentHandlerResponse {
    pub fn shipping_address(&self) -> AddressInit {
        self.inner.get("shippingAddress").as_::<AddressInit>()
    }

    pub fn set_shipping_address(&mut self, value: &AddressInit) {
        self.inner.set("shippingAddress", value);
    }
}
impl PaymentHandlerResponse {
    pub fn shipping_option(&self) -> DOMString {
        self.inner.get("shippingOption").as_::<DOMString>()
    }

    pub fn set_shipping_option(&mut self, value: &DOMString) {
        self.inner.set("shippingOption", value);
    }
}
/// The PaymentRequestEvent class.
/// [`PaymentRequestEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentRequestEvent {
    inner: ExtendableEvent,
}
impl FromVal for PaymentRequestEvent {
    fn from_val(v: &Any) -> Self {
        PaymentRequestEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaymentRequestEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentRequestEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentRequestEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentRequestEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentRequestEvent> for Any {
    fn from(s: PaymentRequestEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentRequestEvent> for Any {
    fn from(s: &PaymentRequestEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PaymentRequestEvent);

impl PaymentRequestEvent {
    /// The `new PaymentRequestEvent(..)` constructor, creating a new PaymentRequestEvent instance
    pub fn new0(type_: &DOMString) -> PaymentRequestEvent {
        Self {
            inner: Any::global("PaymentRequestEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }

    /// The `new PaymentRequestEvent(..)` constructor, creating a new PaymentRequestEvent instance
    pub fn new1(type_: &DOMString, event_init_dict: &Any) -> PaymentRequestEvent {
        Self {
            inner: Any::global("PaymentRequestEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl PaymentRequestEvent {
    /// Getter of the `topOrigin` attribute.
    /// [`PaymentRequestEvent.topOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/topOrigin)
    pub fn top_origin(&self) -> USVString {
        self.inner.get("topOrigin").as_::<USVString>()
    }
}
impl PaymentRequestEvent {
    /// Getter of the `paymentRequestOrigin` attribute.
    /// [`PaymentRequestEvent.paymentRequestOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/paymentRequestOrigin)
    pub fn payment_request_origin(&self) -> USVString {
        self.inner.get("paymentRequestOrigin").as_::<USVString>()
    }
}
impl PaymentRequestEvent {
    /// Getter of the `paymentRequestId` attribute.
    /// [`PaymentRequestEvent.paymentRequestId`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/paymentRequestId)
    pub fn payment_request_id(&self) -> DOMString {
        self.inner.get("paymentRequestId").as_::<DOMString>()
    }
}
impl PaymentRequestEvent {
    /// Getter of the `methodData` attribute.
    /// [`PaymentRequestEvent.methodData`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/methodData)
    pub fn method_data(&self) -> FrozenArray<PaymentMethodData> {
        self.inner
            .get("methodData")
            .as_::<FrozenArray<PaymentMethodData>>()
    }
}
impl PaymentRequestEvent {
    /// Getter of the `total` attribute.
    /// [`PaymentRequestEvent.total`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/total)
    pub fn total(&self) -> Object {
        self.inner.get("total").as_::<Object>()
    }
}
impl PaymentRequestEvent {
    /// Getter of the `modifiers` attribute.
    /// [`PaymentRequestEvent.modifiers`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/modifiers)
    pub fn modifiers(&self) -> FrozenArray<PaymentDetailsModifier> {
        self.inner
            .get("modifiers")
            .as_::<FrozenArray<PaymentDetailsModifier>>()
    }
}
impl PaymentRequestEvent {
    /// Getter of the `paymentOptions` attribute.
    /// [`PaymentRequestEvent.paymentOptions`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/paymentOptions)
    pub fn payment_options(&self) -> Object {
        self.inner.get("paymentOptions").as_::<Object>()
    }
}
impl PaymentRequestEvent {
    /// Getter of the `shippingOptions` attribute.
    /// [`PaymentRequestEvent.shippingOptions`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/shippingOptions)
    pub fn shipping_options(&self) -> FrozenArray<PaymentShippingOption> {
        self.inner
            .get("shippingOptions")
            .as_::<FrozenArray<PaymentShippingOption>>()
    }
}
impl PaymentRequestEvent {
    /// The openWindow method.
    /// [`PaymentRequestEvent.openWindow`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/openWindow)
    pub fn open_window(&self, url: &USVString) -> Promise<WindowClient> {
        self.inner
            .call("openWindow", &[url.into()])
            .as_::<Promise<WindowClient>>()
    }
}
impl PaymentRequestEvent {
    /// The changePaymentMethod method.
    /// [`PaymentRequestEvent.changePaymentMethod`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/changePaymentMethod)
    pub fn change_payment_method0(
        &self,
        method_name: &DOMString,
    ) -> Promise<PaymentRequestDetailsUpdate> {
        self.inner
            .call("changePaymentMethod", &[method_name.into()])
            .as_::<Promise<PaymentRequestDetailsUpdate>>()
    }
    /// The changePaymentMethod method.
    /// [`PaymentRequestEvent.changePaymentMethod`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/changePaymentMethod)
    pub fn change_payment_method1(
        &self,
        method_name: &DOMString,
        method_details: &Object,
    ) -> Promise<PaymentRequestDetailsUpdate> {
        self.inner
            .call(
                "changePaymentMethod",
                &[method_name.into(), method_details.into()],
            )
            .as_::<Promise<PaymentRequestDetailsUpdate>>()
    }
}
impl PaymentRequestEvent {
    /// The changeShippingAddress method.
    /// [`PaymentRequestEvent.changeShippingAddress`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/changeShippingAddress)
    pub fn change_shipping_address0(&self) -> Promise<PaymentRequestDetailsUpdate> {
        self.inner
            .call("changeShippingAddress", &[])
            .as_::<Promise<PaymentRequestDetailsUpdate>>()
    }
    /// The changeShippingAddress method.
    /// [`PaymentRequestEvent.changeShippingAddress`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/changeShippingAddress)
    pub fn change_shipping_address1(
        &self,
        shipping_address: &AddressInit,
    ) -> Promise<PaymentRequestDetailsUpdate> {
        self.inner
            .call("changeShippingAddress", &[shipping_address.into()])
            .as_::<Promise<PaymentRequestDetailsUpdate>>()
    }
}
impl PaymentRequestEvent {
    /// The changeShippingOption method.
    /// [`PaymentRequestEvent.changeShippingOption`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/changeShippingOption)
    pub fn change_shipping_option(
        &self,
        shipping_option: &DOMString,
    ) -> Promise<PaymentRequestDetailsUpdate> {
        self.inner
            .call("changeShippingOption", &[shipping_option.into()])
            .as_::<Promise<PaymentRequestDetailsUpdate>>()
    }
}
impl PaymentRequestEvent {
    /// The respondWith method.
    /// [`PaymentRequestEvent.respondWith`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/respondWith)
    pub fn respond_with(
        &self,
        handler_response_promise: &Promise<PaymentHandlerResponse>,
    ) -> Undefined {
        self.inner
            .call("respondWith", &[handler_response_promise.into()])
            .as_::<Undefined>()
    }
}
