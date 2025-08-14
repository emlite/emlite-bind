use super::*;




/// The RTCOutboundRtpStreamStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCOutboundRtpStreamStats {
    inner: Any,
}

impl FromVal for RTCOutboundRtpStreamStats {
    fn from_val(v: &Any) -> Self {
        RTCOutboundRtpStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCOutboundRtpStreamStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCOutboundRtpStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCOutboundRtpStreamStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCOutboundRtpStreamStats {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCOutboundRtpStreamStats> for Any {
    fn from(s: RTCOutboundRtpStreamStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCOutboundRtpStreamStats> for Any {
    fn from(s: &RTCOutboundRtpStreamStats) -> Any {
        s.inner.clone()
    }
}

impl RTCOutboundRtpStreamStats {
    /// Getter of the `mid` attribute.
    pub fn mid(&self) -> JsString {
        self.inner.get("mid").as_::<JsString>()
    }

    /// Setter of the `mid` attribute.
    pub fn set_mid(&mut self, value: &JsString) {
        self.inner.set("mid", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `mediaSourceId` attribute.
    pub fn media_source_id(&self) -> JsString {
        self.inner.get("mediaSourceId").as_::<JsString>()
    }

    /// Setter of the `mediaSourceId` attribute.
    pub fn set_media_source_id(&mut self, value: &JsString) {
        self.inner.set("mediaSourceId", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `remoteId` attribute.
    pub fn remote_id(&self) -> JsString {
        self.inner.get("remoteId").as_::<JsString>()
    }

    /// Setter of the `remoteId` attribute.
    pub fn set_remote_id(&mut self, value: &JsString) {
        self.inner.set("remoteId", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `rid` attribute.
    pub fn rid(&self) -> JsString {
        self.inner.get("rid").as_::<JsString>()
    }

    /// Setter of the `rid` attribute.
    pub fn set_rid(&mut self, value: &JsString) {
        self.inner.set("rid", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `encodingIndex` attribute.
    pub fn encoding_index(&self) -> u32 {
        self.inner.get("encodingIndex").as_::<u32>()
    }

    /// Setter of the `encodingIndex` attribute.
    pub fn set_encoding_index(&mut self, value: u32) {
        self.inner.set("encodingIndex", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `headerBytesSent` attribute.
    pub fn header_bytes_sent(&self) -> u64 {
        self.inner.get("headerBytesSent").as_::<u64>()
    }

    /// Setter of the `headerBytesSent` attribute.
    pub fn set_header_bytes_sent(&mut self, value: u64) {
        self.inner.set("headerBytesSent", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `retransmittedPacketsSent` attribute.
    pub fn retransmitted_packets_sent(&self) -> u64 {
        self.inner.get("retransmittedPacketsSent").as_::<u64>()
    }

    /// Setter of the `retransmittedPacketsSent` attribute.
    pub fn set_retransmitted_packets_sent(&mut self, value: u64) {
        self.inner.set("retransmittedPacketsSent", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `retransmittedBytesSent` attribute.
    pub fn retransmitted_bytes_sent(&self) -> u64 {
        self.inner.get("retransmittedBytesSent").as_::<u64>()
    }

    /// Setter of the `retransmittedBytesSent` attribute.
    pub fn set_retransmitted_bytes_sent(&mut self, value: u64) {
        self.inner.set("retransmittedBytesSent", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `rtxSsrc` attribute.
    pub fn rtx_ssrc(&self) -> u32 {
        self.inner.get("rtxSsrc").as_::<u32>()
    }

    /// Setter of the `rtxSsrc` attribute.
    pub fn set_rtx_ssrc(&mut self, value: u32) {
        self.inner.set("rtxSsrc", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `targetBitrate` attribute.
    pub fn target_bitrate(&self) -> f64 {
        self.inner.get("targetBitrate").as_::<f64>()
    }

    /// Setter of the `targetBitrate` attribute.
    pub fn set_target_bitrate(&mut self, value: f64) {
        self.inner.set("targetBitrate", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `totalEncodedBytesTarget` attribute.
    pub fn total_encoded_bytes_target(&self) -> u64 {
        self.inner.get("totalEncodedBytesTarget").as_::<u64>()
    }

    /// Setter of the `totalEncodedBytesTarget` attribute.
    pub fn set_total_encoded_bytes_target(&mut self, value: u64) {
        self.inner.set("totalEncodedBytesTarget", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `frameWidth` attribute.
    pub fn frame_width(&self) -> u32 {
        self.inner.get("frameWidth").as_::<u32>()
    }

    /// Setter of the `frameWidth` attribute.
    pub fn set_frame_width(&mut self, value: u32) {
        self.inner.set("frameWidth", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `frameHeight` attribute.
    pub fn frame_height(&self) -> u32 {
        self.inner.get("frameHeight").as_::<u32>()
    }

    /// Setter of the `frameHeight` attribute.
    pub fn set_frame_height(&mut self, value: u32) {
        self.inner.set("frameHeight", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `framesPerSecond` attribute.
    pub fn frames_per_second(&self) -> f64 {
        self.inner.get("framesPerSecond").as_::<f64>()
    }

    /// Setter of the `framesPerSecond` attribute.
    pub fn set_frames_per_second(&mut self, value: f64) {
        self.inner.set("framesPerSecond", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `framesSent` attribute.
    pub fn frames_sent(&self) -> u32 {
        self.inner.get("framesSent").as_::<u32>()
    }

    /// Setter of the `framesSent` attribute.
    pub fn set_frames_sent(&mut self, value: u32) {
        self.inner.set("framesSent", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `hugeFramesSent` attribute.
    pub fn huge_frames_sent(&self) -> u32 {
        self.inner.get("hugeFramesSent").as_::<u32>()
    }

    /// Setter of the `hugeFramesSent` attribute.
    pub fn set_huge_frames_sent(&mut self, value: u32) {
        self.inner.set("hugeFramesSent", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `framesEncoded` attribute.
    pub fn frames_encoded(&self) -> u32 {
        self.inner.get("framesEncoded").as_::<u32>()
    }

    /// Setter of the `framesEncoded` attribute.
    pub fn set_frames_encoded(&mut self, value: u32) {
        self.inner.set("framesEncoded", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `keyFramesEncoded` attribute.
    pub fn key_frames_encoded(&self) -> u32 {
        self.inner.get("keyFramesEncoded").as_::<u32>()
    }

    /// Setter of the `keyFramesEncoded` attribute.
    pub fn set_key_frames_encoded(&mut self, value: u32) {
        self.inner.set("keyFramesEncoded", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `qpSum` attribute.
    pub fn qp_sum(&self) -> u64 {
        self.inner.get("qpSum").as_::<u64>()
    }

    /// Setter of the `qpSum` attribute.
    pub fn set_qp_sum(&mut self, value: u64) {
        self.inner.set("qpSum", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `psnrSum` attribute.
    pub fn psnr_sum(&self) -> Record<JsString, f64> {
        self.inner.get("psnrSum").as_::<Record<JsString, f64>>()
    }

    /// Setter of the `psnrSum` attribute.
    pub fn set_psnr_sum(&mut self, value: Record<JsString, f64>) {
        self.inner.set("psnrSum", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `psnrMeasurements` attribute.
    pub fn psnr_measurements(&self) -> u64 {
        self.inner.get("psnrMeasurements").as_::<u64>()
    }

    /// Setter of the `psnrMeasurements` attribute.
    pub fn set_psnr_measurements(&mut self, value: u64) {
        self.inner.set("psnrMeasurements", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `totalEncodeTime` attribute.
    pub fn total_encode_time(&self) -> f64 {
        self.inner.get("totalEncodeTime").as_::<f64>()
    }

    /// Setter of the `totalEncodeTime` attribute.
    pub fn set_total_encode_time(&mut self, value: f64) {
        self.inner.set("totalEncodeTime", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `totalPacketSendDelay` attribute.
    pub fn total_packet_send_delay(&self) -> f64 {
        self.inner.get("totalPacketSendDelay").as_::<f64>()
    }

    /// Setter of the `totalPacketSendDelay` attribute.
    pub fn set_total_packet_send_delay(&mut self, value: f64) {
        self.inner.set("totalPacketSendDelay", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `qualityLimitationReason` attribute.
    pub fn quality_limitation_reason(&self) -> RTCQualityLimitationReason {
        self.inner.get("qualityLimitationReason").as_::<RTCQualityLimitationReason>()
    }

    /// Setter of the `qualityLimitationReason` attribute.
    pub fn set_quality_limitation_reason(&mut self, value: &RTCQualityLimitationReason) {
        self.inner.set("qualityLimitationReason", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `qualityLimitationDurations` attribute.
    pub fn quality_limitation_durations(&self) -> Record<JsString, f64> {
        self.inner.get("qualityLimitationDurations").as_::<Record<JsString, f64>>()
    }

    /// Setter of the `qualityLimitationDurations` attribute.
    pub fn set_quality_limitation_durations(&mut self, value: Record<JsString, f64>) {
        self.inner.set("qualityLimitationDurations", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `qualityLimitationResolutionChanges` attribute.
    pub fn quality_limitation_resolution_changes(&self) -> u32 {
        self.inner.get("qualityLimitationResolutionChanges").as_::<u32>()
    }

    /// Setter of the `qualityLimitationResolutionChanges` attribute.
    pub fn set_quality_limitation_resolution_changes(&mut self, value: u32) {
        self.inner.set("qualityLimitationResolutionChanges", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `nackCount` attribute.
    pub fn nack_count(&self) -> u32 {
        self.inner.get("nackCount").as_::<u32>()
    }

    /// Setter of the `nackCount` attribute.
    pub fn set_nack_count(&mut self, value: u32) {
        self.inner.set("nackCount", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `firCount` attribute.
    pub fn fir_count(&self) -> u32 {
        self.inner.get("firCount").as_::<u32>()
    }

    /// Setter of the `firCount` attribute.
    pub fn set_fir_count(&mut self, value: u32) {
        self.inner.set("firCount", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `pliCount` attribute.
    pub fn pli_count(&self) -> u32 {
        self.inner.get("pliCount").as_::<u32>()
    }

    /// Setter of the `pliCount` attribute.
    pub fn set_pli_count(&mut self, value: u32) {
        self.inner.set("pliCount", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `encoderImplementation` attribute.
    pub fn encoder_implementation(&self) -> JsString {
        self.inner.get("encoderImplementation").as_::<JsString>()
    }

    /// Setter of the `encoderImplementation` attribute.
    pub fn set_encoder_implementation(&mut self, value: &JsString) {
        self.inner.set("encoderImplementation", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `powerEfficientEncoder` attribute.
    pub fn power_efficient_encoder(&self) -> bool {
        self.inner.get("powerEfficientEncoder").as_::<bool>()
    }

    /// Setter of the `powerEfficientEncoder` attribute.
    pub fn set_power_efficient_encoder(&mut self, value: bool) {
        self.inner.set("powerEfficientEncoder", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `active` attribute.
    pub fn active(&self) -> bool {
        self.inner.get("active").as_::<bool>()
    }

    /// Setter of the `active` attribute.
    pub fn set_active(&mut self, value: bool) {
        self.inner.set("active", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `scalabilityMode` attribute.
    pub fn scalability_mode(&self) -> JsString {
        self.inner.get("scalabilityMode").as_::<JsString>()
    }

    /// Setter of the `scalabilityMode` attribute.
    pub fn set_scalability_mode(&mut self, value: &JsString) {
        self.inner.set("scalabilityMode", value);
    }
}
impl RTCOutboundRtpStreamStats {
    /// Getter of the `packetsSentWithEct1` attribute.
    pub fn packets_sent_with_ect1(&self) -> u64 {
        self.inner.get("packetsSentWithEct1").as_::<u64>()
    }

    /// Setter of the `packetsSentWithEct1` attribute.
    pub fn set_packets_sent_with_ect1(&mut self, value: u64) {
        self.inner.set("packetsSentWithEct1", value);
    }
}
