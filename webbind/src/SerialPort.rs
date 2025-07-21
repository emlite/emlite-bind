use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialPortInfo {
    inner: Any,
}
impl FromVal for SerialPortInfo {
    fn from_val(v: &Any) -> Self {
        SerialPortInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SerialPortInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SerialPortInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SerialPortInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SerialPortInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SerialPortInfo> for Any {
    fn from(s: SerialPortInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SerialPortInfo> for Any {
    fn from(s: &SerialPortInfo) -> Any {
        s.inner.clone()
    }
}

impl SerialPortInfo {
    pub fn usb_vendor_id(&self) -> u16 {
        self.inner.get("usbVendorId").as_::<u16>()
    }

    pub fn set_usb_vendor_id(&mut self, value: u16) {
        self.inner.set("usbVendorId", value);
    }
}
impl SerialPortInfo {
    pub fn usb_product_id(&self) -> u16 {
        self.inner.get("usbProductId").as_::<u16>()
    }

    pub fn set_usb_product_id(&mut self, value: u16) {
        self.inner.set("usbProductId", value);
    }
}
impl SerialPortInfo {
    pub fn bluetooth_service_class_id(&self) -> Any {
        self.inner.get("bluetoothServiceClassId").as_::<Any>()
    }

    pub fn set_bluetooth_service_class_id(&mut self, value: &Any) {
        self.inner.set("bluetoothServiceClassId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialOptions {
    inner: Any,
}
impl FromVal for SerialOptions {
    fn from_val(v: &Any) -> Self {
        SerialOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SerialOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SerialOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SerialOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SerialOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SerialOptions> for Any {
    fn from(s: SerialOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SerialOptions> for Any {
    fn from(s: &SerialOptions) -> Any {
        s.inner.clone()
    }
}

impl SerialOptions {
    pub fn baud_rate(&self) -> u32 {
        self.inner.get("baudRate").as_::<u32>()
    }

    pub fn set_baud_rate(&mut self, value: u32) {
        self.inner.set("baudRate", value);
    }
}
impl SerialOptions {
    pub fn data_bits(&self) -> u8 {
        self.inner.get("dataBits").as_::<u8>()
    }

    pub fn set_data_bits(&mut self, value: u8) {
        self.inner.set("dataBits", value);
    }
}
impl SerialOptions {
    pub fn stop_bits(&self) -> u8 {
        self.inner.get("stopBits").as_::<u8>()
    }

    pub fn set_stop_bits(&mut self, value: u8) {
        self.inner.set("stopBits", value);
    }
}
impl SerialOptions {
    pub fn parity(&self) -> ParityType {
        self.inner.get("parity").as_::<ParityType>()
    }

    pub fn set_parity(&mut self, value: &ParityType) {
        self.inner.set("parity", value);
    }
}
impl SerialOptions {
    pub fn buffer_size(&self) -> u32 {
        self.inner.get("bufferSize").as_::<u32>()
    }

    pub fn set_buffer_size(&mut self, value: u32) {
        self.inner.set("bufferSize", value);
    }
}
impl SerialOptions {
    pub fn flow_control(&self) -> FlowControlType {
        self.inner.get("flowControl").as_::<FlowControlType>()
    }

    pub fn set_flow_control(&mut self, value: &FlowControlType) {
        self.inner.set("flowControl", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialOutputSignals {
    inner: Any,
}
impl FromVal for SerialOutputSignals {
    fn from_val(v: &Any) -> Self {
        SerialOutputSignals { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SerialOutputSignals {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SerialOutputSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SerialOutputSignals {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SerialOutputSignals {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SerialOutputSignals> for Any {
    fn from(s: SerialOutputSignals) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SerialOutputSignals> for Any {
    fn from(s: &SerialOutputSignals) -> Any {
        s.inner.clone()
    }
}

impl SerialOutputSignals {
    pub fn data_terminal_ready(&self) -> bool {
        self.inner.get("dataTerminalReady").as_::<bool>()
    }

    pub fn set_data_terminal_ready(&mut self, value: bool) {
        self.inner.set("dataTerminalReady", value);
    }
}
impl SerialOutputSignals {
    pub fn request_to_send(&self) -> bool {
        self.inner.get("requestToSend").as_::<bool>()
    }

    pub fn set_request_to_send(&mut self, value: bool) {
        self.inner.set("requestToSend", value);
    }
}
impl SerialOutputSignals {
    pub fn break_(&self) -> bool {
        self.inner.get("break").as_::<bool>()
    }

    pub fn set_break_(&mut self, value: bool) {
        self.inner.set("break", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialInputSignals {
    inner: Any,
}
impl FromVal for SerialInputSignals {
    fn from_val(v: &Any) -> Self {
        SerialInputSignals { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SerialInputSignals {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SerialInputSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SerialInputSignals {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SerialInputSignals {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SerialInputSignals> for Any {
    fn from(s: SerialInputSignals) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SerialInputSignals> for Any {
    fn from(s: &SerialInputSignals) -> Any {
        s.inner.clone()
    }
}

impl SerialInputSignals {
    pub fn data_carrier_detect(&self) -> bool {
        self.inner.get("dataCarrierDetect").as_::<bool>()
    }

    pub fn set_data_carrier_detect(&mut self, value: bool) {
        self.inner.set("dataCarrierDetect", value);
    }
}
impl SerialInputSignals {
    pub fn clear_to_send(&self) -> bool {
        self.inner.get("clearToSend").as_::<bool>()
    }

    pub fn set_clear_to_send(&mut self, value: bool) {
        self.inner.set("clearToSend", value);
    }
}
impl SerialInputSignals {
    pub fn ring_indicator(&self) -> bool {
        self.inner.get("ringIndicator").as_::<bool>()
    }

    pub fn set_ring_indicator(&mut self, value: bool) {
        self.inner.set("ringIndicator", value);
    }
}
impl SerialInputSignals {
    pub fn data_set_ready(&self) -> bool {
        self.inner.get("dataSetReady").as_::<bool>()
    }

    pub fn set_data_set_ready(&mut self, value: bool) {
        self.inner.set("dataSetReady", value);
    }
}
/// The SerialPort class.
/// [`SerialPort`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialPort {
    inner: EventTarget,
}
impl FromVal for SerialPort {
    fn from_val(v: &Any) -> Self {
        SerialPort {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SerialPort {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SerialPort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SerialPort {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SerialPort {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SerialPort> for Any {
    fn from(s: SerialPort) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SerialPort> for Any {
    fn from(s: &SerialPort) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SerialPort);

impl SerialPort {
    /// Getter of the `onconnect` attribute.
    /// [`SerialPort.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/onconnect)
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    /// Setter of the `onconnect` attribute.
    /// [`SerialPort.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/onconnect)
    pub fn set_onconnect(&mut self, value: &Any) {
        self.inner.set("onconnect", value);
    }
}
impl SerialPort {
    /// Getter of the `ondisconnect` attribute.
    /// [`SerialPort.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/ondisconnect)
    pub fn ondisconnect(&self) -> Any {
        self.inner.get("ondisconnect").as_::<Any>()
    }

    /// Setter of the `ondisconnect` attribute.
    /// [`SerialPort.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/ondisconnect)
    pub fn set_ondisconnect(&mut self, value: &Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl SerialPort {
    /// Getter of the `connected` attribute.
    /// [`SerialPort.connected`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/connected)
    pub fn connected(&self) -> bool {
        self.inner.get("connected").as_::<bool>()
    }
}
impl SerialPort {
    /// Getter of the `readable` attribute.
    /// [`SerialPort.readable`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl SerialPort {
    /// Getter of the `writable` attribute.
    /// [`SerialPort.writable`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/writable)
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
impl SerialPort {
    /// The getInfo method.
    /// [`SerialPort.getInfo`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/getInfo)
    pub fn get_info(&self) -> SerialPortInfo {
        self.inner.call("getInfo", &[]).as_::<SerialPortInfo>()
    }
}
impl SerialPort {
    /// The open method.
    /// [`SerialPort.open`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/open)
    pub fn open(&self, options: &SerialOptions) -> Promise<Undefined> {
        self.inner
            .call("open", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl SerialPort {
    /// The setSignals method.
    /// [`SerialPort.setSignals`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/setSignals)
    pub fn set_signals0(&self) -> Promise<Undefined> {
        self.inner
            .call("setSignals", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The setSignals method.
    /// [`SerialPort.setSignals`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/setSignals)
    pub fn set_signals1(&self, signals: &SerialOutputSignals) -> Promise<Undefined> {
        self.inner
            .call("setSignals", &[signals.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl SerialPort {
    /// The getSignals method.
    /// [`SerialPort.getSignals`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/getSignals)
    pub fn get_signals(&self) -> Promise<SerialInputSignals> {
        self.inner
            .call("getSignals", &[])
            .as_::<Promise<SerialInputSignals>>()
    }
}
impl SerialPort {
    /// The close method.
    /// [`SerialPort.close`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/close)
    pub fn close(&self) -> Promise<Undefined> {
        self.inner.call("close", &[]).as_::<Promise<Undefined>>()
    }
}
impl SerialPort {
    /// The forget method.
    /// [`SerialPort.forget`](https://developer.mozilla.org/en-US/docs/Web/API/SerialPort/forget)
    pub fn forget(&self) -> Promise<Undefined> {
        self.inner.call("forget", &[]).as_::<Promise<Undefined>>()
    }
}
