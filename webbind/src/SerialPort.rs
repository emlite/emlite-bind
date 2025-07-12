use super::*;

#[derive(Clone, Debug)]
pub struct SerialPortInfo {
    inner: emlite::Val,
}
impl FromVal for SerialPortInfo {
    fn from_val(v: &emlite::Val) -> Self {
        SerialPortInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SerialPortInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SerialPortInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SerialPortInfo> for emlite::Val {
    fn from(s: SerialPortInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
    pub fn bluetooth_service_class_id(&self) -> jsbind::Any {
        self.inner
            .get("bluetoothServiceClassId")
            .as_::<jsbind::Any>()
    }

    pub fn set_bluetooth_service_class_id(&mut self, value: jsbind::Any) {
        self.inner.set("bluetoothServiceClassId", value);
    }
}
#[derive(Clone, Debug)]
pub struct SerialOptions {
    inner: emlite::Val,
}
impl FromVal for SerialOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SerialOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SerialOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SerialOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SerialOptions> for emlite::Val {
    fn from(s: SerialOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
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

    pub fn set_parity(&mut self, value: ParityType) {
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

    pub fn set_flow_control(&mut self, value: FlowControlType) {
        self.inner.set("flowControl", value);
    }
}
#[derive(Clone, Debug)]
pub struct SerialOutputSignals {
    inner: emlite::Val,
}
impl FromVal for SerialOutputSignals {
    fn from_val(v: &emlite::Val) -> Self {
        SerialOutputSignals { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SerialOutputSignals {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SerialOutputSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SerialOutputSignals> for emlite::Val {
    fn from(s: SerialOutputSignals) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
#[derive(Clone, Debug)]
pub struct SerialInputSignals {
    inner: emlite::Val,
}
impl FromVal for SerialInputSignals {
    fn from_val(v: &emlite::Val) -> Self {
        SerialInputSignals { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SerialInputSignals {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SerialInputSignals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SerialInputSignals> for emlite::Val {
    fn from(s: SerialInputSignals) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
#[derive(Clone, Debug)]
pub struct SerialPort {
    inner: EventTarget,
}
impl FromVal for SerialPort {
    fn from_val(v: &emlite::Val) -> Self {
        SerialPort {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SerialPort {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SerialPort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SerialPort> for emlite::Val {
    fn from(s: SerialPort) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SerialPort {
    pub fn onconnect(&self) -> jsbind::Any {
        self.inner.get("onconnect").as_::<jsbind::Any>()
    }

    pub fn set_onconnect(&mut self, value: jsbind::Any) {
        self.inner.set("onconnect", value);
    }
}
impl SerialPort {
    pub fn ondisconnect(&self) -> jsbind::Any {
        self.inner.get("ondisconnect").as_::<jsbind::Any>()
    }

    pub fn set_ondisconnect(&mut self, value: jsbind::Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl SerialPort {
    pub fn connected(&self) -> bool {
        self.inner.get("connected").as_::<bool>()
    }
}
impl SerialPort {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl SerialPort {
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }
}
impl SerialPort {
    pub fn get_info(&self) -> SerialPortInfo {
        self.inner.call("getInfo", &[]).as_::<SerialPortInfo>()
    }
}
impl SerialPort {
    pub fn open(&self, options: SerialOptions) -> jsbind::Promise {
        self.inner
            .call("open", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SerialPort {
    pub fn set_signals0(&self) -> jsbind::Promise {
        self.inner.call("setSignals", &[]).as_::<jsbind::Promise>()
    }

    pub fn set_signals1(&self, signals: SerialOutputSignals) -> jsbind::Promise {
        self.inner
            .call("setSignals", &[signals.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SerialPort {
    pub fn get_signals(&self) -> jsbind::Promise {
        self.inner.call("getSignals", &[]).as_::<jsbind::Promise>()
    }
}
impl SerialPort {
    pub fn close(&self) -> jsbind::Promise {
        self.inner.call("close", &[]).as_::<jsbind::Promise>()
    }
}
impl SerialPort {
    pub fn forget(&self) -> jsbind::Promise {
        self.inner.call("forget", &[]).as_::<jsbind::Promise>()
    }
}
