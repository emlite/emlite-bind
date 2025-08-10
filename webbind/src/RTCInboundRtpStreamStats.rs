use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCInboundRtpStreamStats {
    inner: Any,
}
impl FromVal for RTCInboundRtpStreamStats {
    fn from_val(v: &Any) -> Self {
        RTCInboundRtpStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCInboundRtpStreamStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCInboundRtpStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCInboundRtpStreamStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCInboundRtpStreamStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCInboundRtpStreamStats> for Any {
    fn from(s: RTCInboundRtpStreamStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCInboundRtpStreamStats> for Any {
    fn from(s: &RTCInboundRtpStreamStats) -> Any {
        s.inner.clone()
    }
}

impl RTCInboundRtpStreamStats {
    pub fn track_identifier(&self) -> JsString {
        self.inner.get("trackIdentifier").as_::<JsString>()
    }

    pub fn set_track_identifier(&mut self, value: &JsString) {
        self.inner.set("trackIdentifier", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn mid(&self) -> JsString {
        self.inner.get("mid").as_::<JsString>()
    }

    pub fn set_mid(&mut self, value: &JsString) {
        self.inner.set("mid", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn remote_id(&self) -> JsString {
        self.inner.get("remoteId").as_::<JsString>()
    }

    pub fn set_remote_id(&mut self, value: &JsString) {
        self.inner.set("remoteId", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn frames_decoded(&self) -> u32 {
        self.inner.get("framesDecoded").as_::<u32>()
    }

    pub fn set_frames_decoded(&mut self, value: u32) {
        self.inner.set("framesDecoded", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn key_frames_decoded(&self) -> u32 {
        self.inner.get("keyFramesDecoded").as_::<u32>()
    }

    pub fn set_key_frames_decoded(&mut self, value: u32) {
        self.inner.set("keyFramesDecoded", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn frames_rendered(&self) -> u32 {
        self.inner.get("framesRendered").as_::<u32>()
    }

    pub fn set_frames_rendered(&mut self, value: u32) {
        self.inner.set("framesRendered", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn frames_dropped(&self) -> u32 {
        self.inner.get("framesDropped").as_::<u32>()
    }

    pub fn set_frames_dropped(&mut self, value: u32) {
        self.inner.set("framesDropped", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn frame_width(&self) -> u32 {
        self.inner.get("frameWidth").as_::<u32>()
    }

    pub fn set_frame_width(&mut self, value: u32) {
        self.inner.set("frameWidth", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn frame_height(&self) -> u32 {
        self.inner.get("frameHeight").as_::<u32>()
    }

    pub fn set_frame_height(&mut self, value: u32) {
        self.inner.set("frameHeight", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn frames_per_second(&self) -> f64 {
        self.inner.get("framesPerSecond").as_::<f64>()
    }

    pub fn set_frames_per_second(&mut self, value: f64) {
        self.inner.set("framesPerSecond", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn qp_sum(&self) -> u64 {
        self.inner.get("qpSum").as_::<u64>()
    }

    pub fn set_qp_sum(&mut self, value: u64) {
        self.inner.set("qpSum", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_decode_time(&self) -> f64 {
        self.inner.get("totalDecodeTime").as_::<f64>()
    }

    pub fn set_total_decode_time(&mut self, value: f64) {
        self.inner.set("totalDecodeTime", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_inter_frame_delay(&self) -> f64 {
        self.inner.get("totalInterFrameDelay").as_::<f64>()
    }

    pub fn set_total_inter_frame_delay(&mut self, value: f64) {
        self.inner.set("totalInterFrameDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_squared_inter_frame_delay(&self) -> f64 {
        self.inner.get("totalSquaredInterFrameDelay").as_::<f64>()
    }

    pub fn set_total_squared_inter_frame_delay(&mut self, value: f64) {
        self.inner.set("totalSquaredInterFrameDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn pause_count(&self) -> u32 {
        self.inner.get("pauseCount").as_::<u32>()
    }

    pub fn set_pause_count(&mut self, value: u32) {
        self.inner.set("pauseCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_pauses_duration(&self) -> f64 {
        self.inner.get("totalPausesDuration").as_::<f64>()
    }

    pub fn set_total_pauses_duration(&mut self, value: f64) {
        self.inner.set("totalPausesDuration", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn freeze_count(&self) -> u32 {
        self.inner.get("freezeCount").as_::<u32>()
    }

    pub fn set_freeze_count(&mut self, value: u32) {
        self.inner.set("freezeCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_freezes_duration(&self) -> f64 {
        self.inner.get("totalFreezesDuration").as_::<f64>()
    }

    pub fn set_total_freezes_duration(&mut self, value: f64) {
        self.inner.set("totalFreezesDuration", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn last_packet_received_timestamp(&self) -> Any {
        self.inner.get("lastPacketReceivedTimestamp").as_::<Any>()
    }

    pub fn set_last_packet_received_timestamp(&mut self, value: &Any) {
        self.inner.set("lastPacketReceivedTimestamp", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn header_bytes_received(&self) -> u64 {
        self.inner.get("headerBytesReceived").as_::<u64>()
    }

    pub fn set_header_bytes_received(&mut self, value: u64) {
        self.inner.set("headerBytesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn packets_discarded(&self) -> u64 {
        self.inner.get("packetsDiscarded").as_::<u64>()
    }

    pub fn set_packets_discarded(&mut self, value: u64) {
        self.inner.set("packetsDiscarded", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn fec_bytes_received(&self) -> u64 {
        self.inner.get("fecBytesReceived").as_::<u64>()
    }

    pub fn set_fec_bytes_received(&mut self, value: u64) {
        self.inner.set("fecBytesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn fec_packets_received(&self) -> u64 {
        self.inner.get("fecPacketsReceived").as_::<u64>()
    }

    pub fn set_fec_packets_received(&mut self, value: u64) {
        self.inner.set("fecPacketsReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn fec_packets_discarded(&self) -> u64 {
        self.inner.get("fecPacketsDiscarded").as_::<u64>()
    }

    pub fn set_fec_packets_discarded(&mut self, value: u64) {
        self.inner.set("fecPacketsDiscarded", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn bytes_received(&self) -> u64 {
        self.inner.get("bytesReceived").as_::<u64>()
    }

    pub fn set_bytes_received(&mut self, value: u64) {
        self.inner.set("bytesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn nack_count(&self) -> u32 {
        self.inner.get("nackCount").as_::<u32>()
    }

    pub fn set_nack_count(&mut self, value: u32) {
        self.inner.set("nackCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn fir_count(&self) -> u32 {
        self.inner.get("firCount").as_::<u32>()
    }

    pub fn set_fir_count(&mut self, value: u32) {
        self.inner.set("firCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn pli_count(&self) -> u32 {
        self.inner.get("pliCount").as_::<u32>()
    }

    pub fn set_pli_count(&mut self, value: u32) {
        self.inner.set("pliCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_processing_delay(&self) -> f64 {
        self.inner.get("totalProcessingDelay").as_::<f64>()
    }

    pub fn set_total_processing_delay(&mut self, value: f64) {
        self.inner.set("totalProcessingDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn estimated_playout_timestamp(&self) -> Any {
        self.inner.get("estimatedPlayoutTimestamp").as_::<Any>()
    }

    pub fn set_estimated_playout_timestamp(&mut self, value: &Any) {
        self.inner.set("estimatedPlayoutTimestamp", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn jitter_buffer_delay(&self) -> f64 {
        self.inner.get("jitterBufferDelay").as_::<f64>()
    }

    pub fn set_jitter_buffer_delay(&mut self, value: f64) {
        self.inner.set("jitterBufferDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn jitter_buffer_target_delay(&self) -> f64 {
        self.inner.get("jitterBufferTargetDelay").as_::<f64>()
    }

    pub fn set_jitter_buffer_target_delay(&mut self, value: f64) {
        self.inner.set("jitterBufferTargetDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn jitter_buffer_emitted_count(&self) -> u64 {
        self.inner.get("jitterBufferEmittedCount").as_::<u64>()
    }

    pub fn set_jitter_buffer_emitted_count(&mut self, value: u64) {
        self.inner.set("jitterBufferEmittedCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn jitter_buffer_minimum_delay(&self) -> f64 {
        self.inner.get("jitterBufferMinimumDelay").as_::<f64>()
    }

    pub fn set_jitter_buffer_minimum_delay(&mut self, value: f64) {
        self.inner.set("jitterBufferMinimumDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_samples_received(&self) -> u64 {
        self.inner.get("totalSamplesReceived").as_::<u64>()
    }

    pub fn set_total_samples_received(&mut self, value: u64) {
        self.inner.set("totalSamplesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn concealed_samples(&self) -> u64 {
        self.inner.get("concealedSamples").as_::<u64>()
    }

    pub fn set_concealed_samples(&mut self, value: u64) {
        self.inner.set("concealedSamples", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn silent_concealed_samples(&self) -> u64 {
        self.inner.get("silentConcealedSamples").as_::<u64>()
    }

    pub fn set_silent_concealed_samples(&mut self, value: u64) {
        self.inner.set("silentConcealedSamples", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn concealment_events(&self) -> u64 {
        self.inner.get("concealmentEvents").as_::<u64>()
    }

    pub fn set_concealment_events(&mut self, value: u64) {
        self.inner.set("concealmentEvents", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn inserted_samples_for_deceleration(&self) -> u64 {
        self.inner
            .get("insertedSamplesForDeceleration")
            .as_::<u64>()
    }

    pub fn set_inserted_samples_for_deceleration(&mut self, value: u64) {
        self.inner.set("insertedSamplesForDeceleration", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn removed_samples_for_acceleration(&self) -> u64 {
        self.inner.get("removedSamplesForAcceleration").as_::<u64>()
    }

    pub fn set_removed_samples_for_acceleration(&mut self, value: u64) {
        self.inner.set("removedSamplesForAcceleration", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn audio_level(&self) -> f64 {
        self.inner.get("audioLevel").as_::<f64>()
    }

    pub fn set_audio_level(&mut self, value: f64) {
        self.inner.set("audioLevel", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_audio_energy(&self) -> f64 {
        self.inner.get("totalAudioEnergy").as_::<f64>()
    }

    pub fn set_total_audio_energy(&mut self, value: f64) {
        self.inner.set("totalAudioEnergy", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_samples_duration(&self) -> f64 {
        self.inner.get("totalSamplesDuration").as_::<f64>()
    }

    pub fn set_total_samples_duration(&mut self, value: f64) {
        self.inner.set("totalSamplesDuration", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn frames_received(&self) -> u32 {
        self.inner.get("framesReceived").as_::<u32>()
    }

    pub fn set_frames_received(&mut self, value: u32) {
        self.inner.set("framesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn decoder_implementation(&self) -> JsString {
        self.inner.get("decoderImplementation").as_::<JsString>()
    }

    pub fn set_decoder_implementation(&mut self, value: &JsString) {
        self.inner.set("decoderImplementation", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn playout_id(&self) -> JsString {
        self.inner.get("playoutId").as_::<JsString>()
    }

    pub fn set_playout_id(&mut self, value: &JsString) {
        self.inner.set("playoutId", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn power_efficient_decoder(&self) -> bool {
        self.inner.get("powerEfficientDecoder").as_::<bool>()
    }

    pub fn set_power_efficient_decoder(&mut self, value: bool) {
        self.inner.set("powerEfficientDecoder", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn frames_assembled_from_multiple_packets(&self) -> u32 {
        self.inner
            .get("framesAssembledFromMultiplePackets")
            .as_::<u32>()
    }

    pub fn set_frames_assembled_from_multiple_packets(&mut self, value: u32) {
        self.inner.set("framesAssembledFromMultiplePackets", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_assembly_time(&self) -> f64 {
        self.inner.get("totalAssemblyTime").as_::<f64>()
    }

    pub fn set_total_assembly_time(&mut self, value: f64) {
        self.inner.set("totalAssemblyTime", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn retransmitted_packets_received(&self) -> u64 {
        self.inner.get("retransmittedPacketsReceived").as_::<u64>()
    }

    pub fn set_retransmitted_packets_received(&mut self, value: u64) {
        self.inner.set("retransmittedPacketsReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn retransmitted_bytes_received(&self) -> u64 {
        self.inner.get("retransmittedBytesReceived").as_::<u64>()
    }

    pub fn set_retransmitted_bytes_received(&mut self, value: u64) {
        self.inner.set("retransmittedBytesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn rtx_ssrc(&self) -> u32 {
        self.inner.get("rtxSsrc").as_::<u32>()
    }

    pub fn set_rtx_ssrc(&mut self, value: u32) {
        self.inner.set("rtxSsrc", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn fec_ssrc(&self) -> u32 {
        self.inner.get("fecSsrc").as_::<u32>()
    }

    pub fn set_fec_ssrc(&mut self, value: u32) {
        self.inner.set("fecSsrc", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_corruption_probability(&self) -> f64 {
        self.inner.get("totalCorruptionProbability").as_::<f64>()
    }

    pub fn set_total_corruption_probability(&mut self, value: f64) {
        self.inner.set("totalCorruptionProbability", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn total_squared_corruption_probability(&self) -> f64 {
        self.inner
            .get("totalSquaredCorruptionProbability")
            .as_::<f64>()
    }

    pub fn set_total_squared_corruption_probability(&mut self, value: f64) {
        self.inner.set("totalSquaredCorruptionProbability", value);
    }
}
impl RTCInboundRtpStreamStats {
    pub fn corruption_measurements(&self) -> u64 {
        self.inner.get("corruptionMeasurements").as_::<u64>()
    }

    pub fn set_corruption_measurements(&mut self, value: u64) {
        self.inner.set("corruptionMeasurements", value);
    }
}
