use super::*;




/// The RTCInboundRtpStreamStats dictionary.
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
    /// Getter of the `trackIdentifier` attribute.
    pub fn track_identifier(&self) -> JsString {
        self.inner.get("trackIdentifier").as_::<JsString>()
    }

    /// Setter of the `trackIdentifier` attribute.
    pub fn set_track_identifier(&mut self, value: &JsString) {
        self.inner.set("trackIdentifier", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `mid` attribute.
    pub fn mid(&self) -> JsString {
        self.inner.get("mid").as_::<JsString>()
    }

    /// Setter of the `mid` attribute.
    pub fn set_mid(&mut self, value: &JsString) {
        self.inner.set("mid", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `remoteId` attribute.
    pub fn remote_id(&self) -> JsString {
        self.inner.get("remoteId").as_::<JsString>()
    }

    /// Setter of the `remoteId` attribute.
    pub fn set_remote_id(&mut self, value: &JsString) {
        self.inner.set("remoteId", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `framesDecoded` attribute.
    pub fn frames_decoded(&self) -> u32 {
        self.inner.get("framesDecoded").as_::<u32>()
    }

    /// Setter of the `framesDecoded` attribute.
    pub fn set_frames_decoded(&mut self, value: u32) {
        self.inner.set("framesDecoded", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `keyFramesDecoded` attribute.
    pub fn key_frames_decoded(&self) -> u32 {
        self.inner.get("keyFramesDecoded").as_::<u32>()
    }

    /// Setter of the `keyFramesDecoded` attribute.
    pub fn set_key_frames_decoded(&mut self, value: u32) {
        self.inner.set("keyFramesDecoded", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `framesRendered` attribute.
    pub fn frames_rendered(&self) -> u32 {
        self.inner.get("framesRendered").as_::<u32>()
    }

    /// Setter of the `framesRendered` attribute.
    pub fn set_frames_rendered(&mut self, value: u32) {
        self.inner.set("framesRendered", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `framesDropped` attribute.
    pub fn frames_dropped(&self) -> u32 {
        self.inner.get("framesDropped").as_::<u32>()
    }

    /// Setter of the `framesDropped` attribute.
    pub fn set_frames_dropped(&mut self, value: u32) {
        self.inner.set("framesDropped", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `frameWidth` attribute.
    pub fn frame_width(&self) -> u32 {
        self.inner.get("frameWidth").as_::<u32>()
    }

    /// Setter of the `frameWidth` attribute.
    pub fn set_frame_width(&mut self, value: u32) {
        self.inner.set("frameWidth", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `frameHeight` attribute.
    pub fn frame_height(&self) -> u32 {
        self.inner.get("frameHeight").as_::<u32>()
    }

    /// Setter of the `frameHeight` attribute.
    pub fn set_frame_height(&mut self, value: u32) {
        self.inner.set("frameHeight", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `framesPerSecond` attribute.
    pub fn frames_per_second(&self) -> f64 {
        self.inner.get("framesPerSecond").as_::<f64>()
    }

    /// Setter of the `framesPerSecond` attribute.
    pub fn set_frames_per_second(&mut self, value: f64) {
        self.inner.set("framesPerSecond", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `qpSum` attribute.
    pub fn qp_sum(&self) -> u64 {
        self.inner.get("qpSum").as_::<u64>()
    }

    /// Setter of the `qpSum` attribute.
    pub fn set_qp_sum(&mut self, value: u64) {
        self.inner.set("qpSum", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalDecodeTime` attribute.
    pub fn total_decode_time(&self) -> f64 {
        self.inner.get("totalDecodeTime").as_::<f64>()
    }

    /// Setter of the `totalDecodeTime` attribute.
    pub fn set_total_decode_time(&mut self, value: f64) {
        self.inner.set("totalDecodeTime", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalInterFrameDelay` attribute.
    pub fn total_inter_frame_delay(&self) -> f64 {
        self.inner.get("totalInterFrameDelay").as_::<f64>()
    }

    /// Setter of the `totalInterFrameDelay` attribute.
    pub fn set_total_inter_frame_delay(&mut self, value: f64) {
        self.inner.set("totalInterFrameDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalSquaredInterFrameDelay` attribute.
    pub fn total_squared_inter_frame_delay(&self) -> f64 {
        self.inner.get("totalSquaredInterFrameDelay").as_::<f64>()
    }

    /// Setter of the `totalSquaredInterFrameDelay` attribute.
    pub fn set_total_squared_inter_frame_delay(&mut self, value: f64) {
        self.inner.set("totalSquaredInterFrameDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `pauseCount` attribute.
    pub fn pause_count(&self) -> u32 {
        self.inner.get("pauseCount").as_::<u32>()
    }

    /// Setter of the `pauseCount` attribute.
    pub fn set_pause_count(&mut self, value: u32) {
        self.inner.set("pauseCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalPausesDuration` attribute.
    pub fn total_pauses_duration(&self) -> f64 {
        self.inner.get("totalPausesDuration").as_::<f64>()
    }

    /// Setter of the `totalPausesDuration` attribute.
    pub fn set_total_pauses_duration(&mut self, value: f64) {
        self.inner.set("totalPausesDuration", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `freezeCount` attribute.
    pub fn freeze_count(&self) -> u32 {
        self.inner.get("freezeCount").as_::<u32>()
    }

    /// Setter of the `freezeCount` attribute.
    pub fn set_freeze_count(&mut self, value: u32) {
        self.inner.set("freezeCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalFreezesDuration` attribute.
    pub fn total_freezes_duration(&self) -> f64 {
        self.inner.get("totalFreezesDuration").as_::<f64>()
    }

    /// Setter of the `totalFreezesDuration` attribute.
    pub fn set_total_freezes_duration(&mut self, value: f64) {
        self.inner.set("totalFreezesDuration", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `lastPacketReceivedTimestamp` attribute.
    pub fn last_packet_received_timestamp(&self) -> Any {
        self.inner.get("lastPacketReceivedTimestamp").as_::<Any>()
    }

    /// Setter of the `lastPacketReceivedTimestamp` attribute.
    pub fn set_last_packet_received_timestamp(&mut self, value: &Any) {
        self.inner.set("lastPacketReceivedTimestamp", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `headerBytesReceived` attribute.
    pub fn header_bytes_received(&self) -> u64 {
        self.inner.get("headerBytesReceived").as_::<u64>()
    }

    /// Setter of the `headerBytesReceived` attribute.
    pub fn set_header_bytes_received(&mut self, value: u64) {
        self.inner.set("headerBytesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `packetsDiscarded` attribute.
    pub fn packets_discarded(&self) -> u64 {
        self.inner.get("packetsDiscarded").as_::<u64>()
    }

    /// Setter of the `packetsDiscarded` attribute.
    pub fn set_packets_discarded(&mut self, value: u64) {
        self.inner.set("packetsDiscarded", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `fecBytesReceived` attribute.
    pub fn fec_bytes_received(&self) -> u64 {
        self.inner.get("fecBytesReceived").as_::<u64>()
    }

    /// Setter of the `fecBytesReceived` attribute.
    pub fn set_fec_bytes_received(&mut self, value: u64) {
        self.inner.set("fecBytesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `fecPacketsReceived` attribute.
    pub fn fec_packets_received(&self) -> u64 {
        self.inner.get("fecPacketsReceived").as_::<u64>()
    }

    /// Setter of the `fecPacketsReceived` attribute.
    pub fn set_fec_packets_received(&mut self, value: u64) {
        self.inner.set("fecPacketsReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `fecPacketsDiscarded` attribute.
    pub fn fec_packets_discarded(&self) -> u64 {
        self.inner.get("fecPacketsDiscarded").as_::<u64>()
    }

    /// Setter of the `fecPacketsDiscarded` attribute.
    pub fn set_fec_packets_discarded(&mut self, value: u64) {
        self.inner.set("fecPacketsDiscarded", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `bytesReceived` attribute.
    pub fn bytes_received(&self) -> u64 {
        self.inner.get("bytesReceived").as_::<u64>()
    }

    /// Setter of the `bytesReceived` attribute.
    pub fn set_bytes_received(&mut self, value: u64) {
        self.inner.set("bytesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `nackCount` attribute.
    pub fn nack_count(&self) -> u32 {
        self.inner.get("nackCount").as_::<u32>()
    }

    /// Setter of the `nackCount` attribute.
    pub fn set_nack_count(&mut self, value: u32) {
        self.inner.set("nackCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `firCount` attribute.
    pub fn fir_count(&self) -> u32 {
        self.inner.get("firCount").as_::<u32>()
    }

    /// Setter of the `firCount` attribute.
    pub fn set_fir_count(&mut self, value: u32) {
        self.inner.set("firCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `pliCount` attribute.
    pub fn pli_count(&self) -> u32 {
        self.inner.get("pliCount").as_::<u32>()
    }

    /// Setter of the `pliCount` attribute.
    pub fn set_pli_count(&mut self, value: u32) {
        self.inner.set("pliCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalProcessingDelay` attribute.
    pub fn total_processing_delay(&self) -> f64 {
        self.inner.get("totalProcessingDelay").as_::<f64>()
    }

    /// Setter of the `totalProcessingDelay` attribute.
    pub fn set_total_processing_delay(&mut self, value: f64) {
        self.inner.set("totalProcessingDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `estimatedPlayoutTimestamp` attribute.
    pub fn estimated_playout_timestamp(&self) -> Any {
        self.inner.get("estimatedPlayoutTimestamp").as_::<Any>()
    }

    /// Setter of the `estimatedPlayoutTimestamp` attribute.
    pub fn set_estimated_playout_timestamp(&mut self, value: &Any) {
        self.inner.set("estimatedPlayoutTimestamp", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `jitterBufferDelay` attribute.
    pub fn jitter_buffer_delay(&self) -> f64 {
        self.inner.get("jitterBufferDelay").as_::<f64>()
    }

    /// Setter of the `jitterBufferDelay` attribute.
    pub fn set_jitter_buffer_delay(&mut self, value: f64) {
        self.inner.set("jitterBufferDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `jitterBufferTargetDelay` attribute.
    pub fn jitter_buffer_target_delay(&self) -> f64 {
        self.inner.get("jitterBufferTargetDelay").as_::<f64>()
    }

    /// Setter of the `jitterBufferTargetDelay` attribute.
    pub fn set_jitter_buffer_target_delay(&mut self, value: f64) {
        self.inner.set("jitterBufferTargetDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `jitterBufferEmittedCount` attribute.
    pub fn jitter_buffer_emitted_count(&self) -> u64 {
        self.inner.get("jitterBufferEmittedCount").as_::<u64>()
    }

    /// Setter of the `jitterBufferEmittedCount` attribute.
    pub fn set_jitter_buffer_emitted_count(&mut self, value: u64) {
        self.inner.set("jitterBufferEmittedCount", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `jitterBufferMinimumDelay` attribute.
    pub fn jitter_buffer_minimum_delay(&self) -> f64 {
        self.inner.get("jitterBufferMinimumDelay").as_::<f64>()
    }

    /// Setter of the `jitterBufferMinimumDelay` attribute.
    pub fn set_jitter_buffer_minimum_delay(&mut self, value: f64) {
        self.inner.set("jitterBufferMinimumDelay", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalSamplesReceived` attribute.
    pub fn total_samples_received(&self) -> u64 {
        self.inner.get("totalSamplesReceived").as_::<u64>()
    }

    /// Setter of the `totalSamplesReceived` attribute.
    pub fn set_total_samples_received(&mut self, value: u64) {
        self.inner.set("totalSamplesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `concealedSamples` attribute.
    pub fn concealed_samples(&self) -> u64 {
        self.inner.get("concealedSamples").as_::<u64>()
    }

    /// Setter of the `concealedSamples` attribute.
    pub fn set_concealed_samples(&mut self, value: u64) {
        self.inner.set("concealedSamples", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `silentConcealedSamples` attribute.
    pub fn silent_concealed_samples(&self) -> u64 {
        self.inner.get("silentConcealedSamples").as_::<u64>()
    }

    /// Setter of the `silentConcealedSamples` attribute.
    pub fn set_silent_concealed_samples(&mut self, value: u64) {
        self.inner.set("silentConcealedSamples", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `concealmentEvents` attribute.
    pub fn concealment_events(&self) -> u64 {
        self.inner.get("concealmentEvents").as_::<u64>()
    }

    /// Setter of the `concealmentEvents` attribute.
    pub fn set_concealment_events(&mut self, value: u64) {
        self.inner.set("concealmentEvents", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `insertedSamplesForDeceleration` attribute.
    pub fn inserted_samples_for_deceleration(&self) -> u64 {
        self.inner.get("insertedSamplesForDeceleration").as_::<u64>()
    }

    /// Setter of the `insertedSamplesForDeceleration` attribute.
    pub fn set_inserted_samples_for_deceleration(&mut self, value: u64) {
        self.inner.set("insertedSamplesForDeceleration", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `removedSamplesForAcceleration` attribute.
    pub fn removed_samples_for_acceleration(&self) -> u64 {
        self.inner.get("removedSamplesForAcceleration").as_::<u64>()
    }

    /// Setter of the `removedSamplesForAcceleration` attribute.
    pub fn set_removed_samples_for_acceleration(&mut self, value: u64) {
        self.inner.set("removedSamplesForAcceleration", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `audioLevel` attribute.
    pub fn audio_level(&self) -> f64 {
        self.inner.get("audioLevel").as_::<f64>()
    }

    /// Setter of the `audioLevel` attribute.
    pub fn set_audio_level(&mut self, value: f64) {
        self.inner.set("audioLevel", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalAudioEnergy` attribute.
    pub fn total_audio_energy(&self) -> f64 {
        self.inner.get("totalAudioEnergy").as_::<f64>()
    }

    /// Setter of the `totalAudioEnergy` attribute.
    pub fn set_total_audio_energy(&mut self, value: f64) {
        self.inner.set("totalAudioEnergy", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalSamplesDuration` attribute.
    pub fn total_samples_duration(&self) -> f64 {
        self.inner.get("totalSamplesDuration").as_::<f64>()
    }

    /// Setter of the `totalSamplesDuration` attribute.
    pub fn set_total_samples_duration(&mut self, value: f64) {
        self.inner.set("totalSamplesDuration", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `framesReceived` attribute.
    pub fn frames_received(&self) -> u32 {
        self.inner.get("framesReceived").as_::<u32>()
    }

    /// Setter of the `framesReceived` attribute.
    pub fn set_frames_received(&mut self, value: u32) {
        self.inner.set("framesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `decoderImplementation` attribute.
    pub fn decoder_implementation(&self) -> JsString {
        self.inner.get("decoderImplementation").as_::<JsString>()
    }

    /// Setter of the `decoderImplementation` attribute.
    pub fn set_decoder_implementation(&mut self, value: &JsString) {
        self.inner.set("decoderImplementation", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `playoutId` attribute.
    pub fn playout_id(&self) -> JsString {
        self.inner.get("playoutId").as_::<JsString>()
    }

    /// Setter of the `playoutId` attribute.
    pub fn set_playout_id(&mut self, value: &JsString) {
        self.inner.set("playoutId", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `powerEfficientDecoder` attribute.
    pub fn power_efficient_decoder(&self) -> bool {
        self.inner.get("powerEfficientDecoder").as_::<bool>()
    }

    /// Setter of the `powerEfficientDecoder` attribute.
    pub fn set_power_efficient_decoder(&mut self, value: bool) {
        self.inner.set("powerEfficientDecoder", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `framesAssembledFromMultiplePackets` attribute.
    pub fn frames_assembled_from_multiple_packets(&self) -> u32 {
        self.inner.get("framesAssembledFromMultiplePackets").as_::<u32>()
    }

    /// Setter of the `framesAssembledFromMultiplePackets` attribute.
    pub fn set_frames_assembled_from_multiple_packets(&mut self, value: u32) {
        self.inner.set("framesAssembledFromMultiplePackets", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalAssemblyTime` attribute.
    pub fn total_assembly_time(&self) -> f64 {
        self.inner.get("totalAssemblyTime").as_::<f64>()
    }

    /// Setter of the `totalAssemblyTime` attribute.
    pub fn set_total_assembly_time(&mut self, value: f64) {
        self.inner.set("totalAssemblyTime", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `retransmittedPacketsReceived` attribute.
    pub fn retransmitted_packets_received(&self) -> u64 {
        self.inner.get("retransmittedPacketsReceived").as_::<u64>()
    }

    /// Setter of the `retransmittedPacketsReceived` attribute.
    pub fn set_retransmitted_packets_received(&mut self, value: u64) {
        self.inner.set("retransmittedPacketsReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `retransmittedBytesReceived` attribute.
    pub fn retransmitted_bytes_received(&self) -> u64 {
        self.inner.get("retransmittedBytesReceived").as_::<u64>()
    }

    /// Setter of the `retransmittedBytesReceived` attribute.
    pub fn set_retransmitted_bytes_received(&mut self, value: u64) {
        self.inner.set("retransmittedBytesReceived", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `rtxSsrc` attribute.
    pub fn rtx_ssrc(&self) -> u32 {
        self.inner.get("rtxSsrc").as_::<u32>()
    }

    /// Setter of the `rtxSsrc` attribute.
    pub fn set_rtx_ssrc(&mut self, value: u32) {
        self.inner.set("rtxSsrc", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `fecSsrc` attribute.
    pub fn fec_ssrc(&self) -> u32 {
        self.inner.get("fecSsrc").as_::<u32>()
    }

    /// Setter of the `fecSsrc` attribute.
    pub fn set_fec_ssrc(&mut self, value: u32) {
        self.inner.set("fecSsrc", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalCorruptionProbability` attribute.
    pub fn total_corruption_probability(&self) -> f64 {
        self.inner.get("totalCorruptionProbability").as_::<f64>()
    }

    /// Setter of the `totalCorruptionProbability` attribute.
    pub fn set_total_corruption_probability(&mut self, value: f64) {
        self.inner.set("totalCorruptionProbability", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `totalSquaredCorruptionProbability` attribute.
    pub fn total_squared_corruption_probability(&self) -> f64 {
        self.inner.get("totalSquaredCorruptionProbability").as_::<f64>()
    }

    /// Setter of the `totalSquaredCorruptionProbability` attribute.
    pub fn set_total_squared_corruption_probability(&mut self, value: f64) {
        self.inner.set("totalSquaredCorruptionProbability", value);
    }
}
impl RTCInboundRtpStreamStats {
    /// Getter of the `corruptionMeasurements` attribute.
    pub fn corruption_measurements(&self) -> u64 {
        self.inner.get("corruptionMeasurements").as_::<u64>()
    }

    /// Setter of the `corruptionMeasurements` attribute.
    pub fn set_corruption_measurements(&mut self, value: u64) {
        self.inner.set("corruptionMeasurements", value);
    }
}
