use super::*;

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
    /// Getter of the `topOrigin` attribute.
    /// [`PaymentRequestEvent.topOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/topOrigin)
    pub fn top_origin(&self) -> JsString {
        self.inner.get("topOrigin").as_::<JsString>()
    }
}
impl PaymentRequestEvent {
    /// Getter of the `paymentRequestOrigin` attribute.
    /// [`PaymentRequestEvent.paymentRequestOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/paymentRequestOrigin)
    pub fn payment_request_origin(&self) -> JsString {
        self.inner.get("paymentRequestOrigin").as_::<JsString>()
    }
}
impl PaymentRequestEvent {
    /// Getter of the `paymentRequestId` attribute.
    /// [`PaymentRequestEvent.paymentRequestId`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/paymentRequestId)
    pub fn payment_request_id(&self) -> JsString {
        self.inner.get("paymentRequestId").as_::<JsString>()
    }
}
impl PaymentRequestEvent {
    /// Getter of the `methodData` attribute.
    /// [`PaymentRequestEvent.methodData`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/methodData)
    pub fn method_data(&self) -> TypedArray<PaymentMethodData> {
        self.inner
            .get("methodData")
            .as_::<TypedArray<PaymentMethodData>>()
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
    pub fn modifiers(&self) -> TypedArray<PaymentDetailsModifier> {
        self.inner
            .get("modifiers")
            .as_::<TypedArray<PaymentDetailsModifier>>()
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
    pub fn shipping_options(&self) -> TypedArray<PaymentShippingOption> {
        self.inner
            .get("shippingOptions")
            .as_::<TypedArray<PaymentShippingOption>>()
    }
}

impl PaymentRequestEvent {
    /// The `new PaymentRequestEvent(..)` constructor, creating a new PaymentRequestEvent instance
    pub fn new0(type_: &JsString) -> PaymentRequestEvent {
        Self {
            inner: Any::global("PaymentRequestEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }

    /// The `new PaymentRequestEvent(..)` constructor, creating a new PaymentRequestEvent instance
    pub fn new1(
        type_: &JsString,
        event_init_dict: &PaymentRequestEventInit,
    ) -> PaymentRequestEvent {
        Self {
            inner: Any::global("PaymentRequestEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl PaymentRequestEvent {
    /// The openWindow method.
    /// [`PaymentRequestEvent.openWindow`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/openWindow)
    pub fn open_window(&self, url: &JsString) -> Promise<WindowClient> {
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
        method_name: &JsString,
    ) -> Promise<PaymentRequestDetailsUpdate> {
        self.inner
            .call("changePaymentMethod", &[method_name.into()])
            .as_::<Promise<PaymentRequestDetailsUpdate>>()
    }
    /// The changePaymentMethod method.
    /// [`PaymentRequestEvent.changePaymentMethod`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestEvent/changePaymentMethod)
    pub fn change_payment_method1(
        &self,
        method_name: &JsString,
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
        shipping_option: &JsString,
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
