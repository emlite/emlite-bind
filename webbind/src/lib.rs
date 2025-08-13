#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::too_many_arguments)]
#![no_std]
extern crate alloc;

use alloc::string::String;
use jsbind::prelude::*;

macro_rules! web_feature {
    ($feature:literal, $mod_ident:ident, $path:literal, $type_ident:ident) => {
        // #[cfg(feature = $feature)]
        #[path = $path]
        pub mod $mod_ident;
        // #[cfg(feature = $feature)]
        pub use $mod_ident::*;

        // #[cfg(not(feature = $feature))]
        // pub type $type_ident = Any;
    };
}
web_feature!(
    "ANGLE_instanced_arrays",
    angle_instanced_arrays,
    "ANGLE_instanced_arrays.rs",
    ANGLE_instanced_arrays
);

web_feature!(
    "AacEncoderConfig",
    aac_encoder_config,
    "AacEncoderConfig.rs",
    AacEncoderConfig
);

web_feature!(
    "AbortController",
    abort_controller,
    "AbortController.rs",
    AbortController
);

web_feature!("AbortSignal", abort_signal, "AbortSignal.rs", AbortSignal);

web_feature!(
    "AbsoluteOrientationSensor",
    absolute_orientation_sensor,
    "AbsoluteOrientationSensor.rs",
    AbsoluteOrientationSensor
);

web_feature!(
    "AbstractRange",
    abstract_range,
    "AbstractRange.rs",
    AbstractRange
);

web_feature!(
    "Accelerometer",
    accelerometer,
    "Accelerometer.rs",
    Accelerometer
);

web_feature!(
    "AccelerometerSensorOptions",
    accelerometer_sensor_options,
    "AccelerometerSensorOptions.rs",
    AccelerometerSensorOptions
);

web_feature!(
    "AdAuctionData",
    ad_auction_data,
    "AdAuctionData.rs",
    AdAuctionData
);

web_feature!(
    "AdAuctionDataBuyerConfig",
    ad_auction_data_buyer_config,
    "AdAuctionDataBuyerConfig.rs",
    AdAuctionDataBuyerConfig
);

web_feature!(
    "AdAuctionDataConfig",
    ad_auction_data_config,
    "AdAuctionDataConfig.rs",
    AdAuctionDataConfig
);

web_feature!(
    "AdAuctionOneSeller",
    ad_auction_one_seller,
    "AdAuctionOneSeller.rs",
    AdAuctionOneSeller
);

web_feature!(
    "AdAuctionPerSellerData",
    ad_auction_per_seller_data,
    "AdAuctionPerSellerData.rs",
    AdAuctionPerSellerData
);

web_feature!("AdRender", ad_render, "AdRender.rs", AdRender);

web_feature!(
    "AddEventListenerOptions",
    add_event_listener_options,
    "AddEventListenerOptions.rs",
    AddEventListenerOptions
);

web_feature!(
    "AddressErrors",
    address_errors,
    "AddressErrors.rs",
    AddressErrors
);

web_feature!("AddressInit", address_init, "AddressInit.rs", AddressInit);

web_feature!(
    "AesCbcParams",
    aes_cbc_params,
    "AesCbcParams.rs",
    AesCbcParams
);

web_feature!(
    "AesCtrParams",
    aes_ctr_params,
    "AesCtrParams.rs",
    AesCtrParams
);

web_feature!(
    "AesDerivedKeyParams",
    aes_derived_key_params,
    "AesDerivedKeyParams.rs",
    AesDerivedKeyParams
);

web_feature!(
    "AesGcmParams",
    aes_gcm_params,
    "AesGcmParams.rs",
    AesGcmParams
);

web_feature!(
    "AesKeyAlgorithm",
    aes_key_algorithm,
    "AesKeyAlgorithm.rs",
    AesKeyAlgorithm
);

web_feature!(
    "AesKeyGenParams",
    aes_key_gen_params,
    "AesKeyGenParams.rs",
    AesKeyGenParams
);

web_feature!("Algorithm", algorithm, "Algorithm.rs", Algorithm);

web_feature!(
    "AllAcceptedCredentialsOptions",
    all_accepted_credentials_options,
    "AllAcceptedCredentialsOptions.rs",
    AllAcceptedCredentialsOptions
);

web_feature!(
    "AllowedBluetoothDevice",
    allowed_bluetooth_device,
    "AllowedBluetoothDevice.rs",
    AllowedBluetoothDevice
);

web_feature!(
    "AllowedUSBDevice",
    allowed_usb_device,
    "AllowedUSBDevice.rs",
    AllowedUSBDevice
);

web_feature!(
    "AmbientLightSensor",
    ambient_light_sensor,
    "AmbientLightSensor.rs",
    AmbientLightSensor
);

web_feature!(
    "AnalyserNode",
    analyser_node,
    "AnalyserNode.rs",
    AnalyserNode
);

web_feature!(
    "AnalyserOptions",
    analyser_options,
    "AnalyserOptions.rs",
    AnalyserOptions
);

web_feature!("Animation", animation, "Animation.rs", Animation);

web_feature!(
    "AnimationEffect",
    animation_effect,
    "AnimationEffect.rs",
    AnimationEffect
);

web_feature!(
    "AnimationEvent",
    animation_event,
    "AnimationEvent.rs",
    AnimationEvent
);

web_feature!(
    "AnimationEventInit",
    animation_event_init,
    "AnimationEventInit.rs",
    AnimationEventInit
);

web_feature!(
    "AnimationNodeList",
    animation_node_list,
    "AnimationNodeList.rs",
    AnimationNodeList
);

web_feature!(
    "AnimationPlaybackEvent",
    animation_playback_event,
    "AnimationPlaybackEvent.rs",
    AnimationPlaybackEvent
);

web_feature!(
    "AnimationPlaybackEventInit",
    animation_playback_event_init,
    "AnimationPlaybackEventInit.rs",
    AnimationPlaybackEventInit
);

web_feature!(
    "AnimationTimeline",
    animation_timeline,
    "AnimationTimeline.rs",
    AnimationTimeline
);

web_feature!(
    "AnimationTrigger",
    animation_trigger,
    "AnimationTrigger.rs",
    AnimationTrigger
);

web_feature!(
    "AnimationTriggerOptions",
    animation_trigger_options,
    "AnimationTriggerOptions.rs",
    AnimationTriggerOptions
);

web_feature!(
    "AnimationWorkletGlobalScope",
    animation_worklet_global_scope,
    "AnimationWorkletGlobalScope.rs",
    AnimationWorkletGlobalScope
);

web_feature!(
    "AssignedNodesOptions",
    assigned_nodes_options,
    "AssignedNodesOptions.rs",
    AssignedNodesOptions
);

web_feature!("Attr", attr, "Attr.rs", Attr);

web_feature!("Attribution", attribution, "Attribution.rs", Attribution);

web_feature!(
    "AttributionAggregationService",
    attribution_aggregation_service,
    "AttributionAggregationService.rs",
    AttributionAggregationService
);

web_feature!(
    "AttributionAggregationServices",
    attribution_aggregation_services,
    "AttributionAggregationServices.rs",
    AttributionAggregationServices
);

web_feature!(
    "AttributionConversionOptions",
    attribution_conversion_options,
    "AttributionConversionOptions.rs",
    AttributionConversionOptions
);

web_feature!(
    "AttributionConversionResult",
    attribution_conversion_result,
    "AttributionConversionResult.rs",
    AttributionConversionResult
);

web_feature!(
    "AttributionImpressionOptions",
    attribution_impression_options,
    "AttributionImpressionOptions.rs",
    AttributionImpressionOptions
);

web_feature!(
    "AttributionImpressionResult",
    attribution_impression_result,
    "AttributionImpressionResult.rs",
    AttributionImpressionResult
);

web_feature!(
    "AttributionReportingRequestOptions",
    attribution_reporting_request_options,
    "AttributionReportingRequestOptions.rs",
    AttributionReportingRequestOptions
);

web_feature!("AuctionAd", auction_ad, "AuctionAd.rs", AuctionAd);

web_feature!(
    "AuctionAdConfig",
    auction_ad_config,
    "AuctionAdConfig.rs",
    AuctionAdConfig
);

web_feature!(
    "AuctionAdInterestGroup",
    auction_ad_interest_group,
    "AuctionAdInterestGroup.rs",
    AuctionAdInterestGroup
);

web_feature!(
    "AuctionAdInterestGroupKey",
    auction_ad_interest_group_key,
    "AuctionAdInterestGroupKey.rs",
    AuctionAdInterestGroupKey
);

web_feature!(
    "AuctionAdInterestGroupSize",
    auction_ad_interest_group_size,
    "AuctionAdInterestGroupSize.rs",
    AuctionAdInterestGroupSize
);

web_feature!(
    "AuctionRealTimeReportingConfig",
    auction_real_time_reporting_config,
    "AuctionRealTimeReportingConfig.rs",
    AuctionRealTimeReportingConfig
);

web_feature!(
    "AuctionReportBuyerDebugModeConfig",
    auction_report_buyer_debug_mode_config,
    "AuctionReportBuyerDebugModeConfig.rs",
    AuctionReportBuyerDebugModeConfig
);

web_feature!(
    "AuctionReportBuyersConfig",
    auction_report_buyers_config,
    "AuctionReportBuyersConfig.rs",
    AuctionReportBuyersConfig
);

web_feature!("AudioBuffer", audio_buffer, "AudioBuffer.rs", AudioBuffer);

web_feature!(
    "AudioBufferOptions",
    audio_buffer_options,
    "AudioBufferOptions.rs",
    AudioBufferOptions
);

web_feature!(
    "AudioBufferSourceNode",
    audio_buffer_source_node,
    "AudioBufferSourceNode.rs",
    AudioBufferSourceNode
);

web_feature!(
    "AudioBufferSourceOptions",
    audio_buffer_source_options,
    "AudioBufferSourceOptions.rs",
    AudioBufferSourceOptions
);

web_feature!(
    "AudioConfiguration",
    audio_configuration,
    "AudioConfiguration.rs",
    AudioConfiguration
);

web_feature!(
    "AudioContext",
    audio_context,
    "AudioContext.rs",
    AudioContext
);

web_feature!(
    "AudioContextOptions",
    audio_context_options,
    "AudioContextOptions.rs",
    AudioContextOptions
);

web_feature!("AudioData", audio_data, "AudioData.rs", AudioData);

web_feature!(
    "AudioDataCopyToOptions",
    audio_data_copy_to_options,
    "AudioDataCopyToOptions.rs",
    AudioDataCopyToOptions
);

web_feature!(
    "AudioDataInit",
    audio_data_init,
    "AudioDataInit.rs",
    AudioDataInit
);

web_feature!(
    "AudioDecoder",
    audio_decoder,
    "AudioDecoder.rs",
    AudioDecoder
);

web_feature!(
    "AudioDecoderConfig",
    audio_decoder_config,
    "AudioDecoderConfig.rs",
    AudioDecoderConfig
);

web_feature!(
    "AudioDecoderInit",
    audio_decoder_init,
    "AudioDecoderInit.rs",
    AudioDecoderInit
);

web_feature!(
    "AudioDecoderSupport",
    audio_decoder_support,
    "AudioDecoderSupport.rs",
    AudioDecoderSupport
);

web_feature!(
    "AudioDestinationNode",
    audio_destination_node,
    "AudioDestinationNode.rs",
    AudioDestinationNode
);

web_feature!(
    "AudioEncoder",
    audio_encoder,
    "AudioEncoder.rs",
    AudioEncoder
);

web_feature!(
    "AudioEncoderConfig",
    audio_encoder_config,
    "AudioEncoderConfig.rs",
    AudioEncoderConfig
);

web_feature!(
    "AudioEncoderInit",
    audio_encoder_init,
    "AudioEncoderInit.rs",
    AudioEncoderInit
);

web_feature!(
    "AudioEncoderSupport",
    audio_encoder_support,
    "AudioEncoderSupport.rs",
    AudioEncoderSupport
);

web_feature!(
    "AudioListener",
    audio_listener,
    "AudioListener.rs",
    AudioListener
);

web_feature!("AudioNode", audio_node, "AudioNode.rs", AudioNode);

web_feature!(
    "AudioNodeOptions",
    audio_node_options,
    "AudioNodeOptions.rs",
    AudioNodeOptions
);

web_feature!(
    "AudioOutputOptions",
    audio_output_options,
    "AudioOutputOptions.rs",
    AudioOutputOptions
);

web_feature!("AudioParam", audio_param, "AudioParam.rs", AudioParam);

web_feature!(
    "AudioParamDescriptor",
    audio_param_descriptor,
    "AudioParamDescriptor.rs",
    AudioParamDescriptor
);

web_feature!(
    "AudioParamMap",
    audio_param_map,
    "AudioParamMap.rs",
    AudioParamMap
);

web_feature!(
    "AudioProcessingEvent",
    audio_processing_event,
    "AudioProcessingEvent.rs",
    AudioProcessingEvent
);

web_feature!(
    "AudioProcessingEventInit",
    audio_processing_event_init,
    "AudioProcessingEventInit.rs",
    AudioProcessingEventInit
);

web_feature!(
    "AudioScheduledSourceNode",
    audio_scheduled_source_node,
    "AudioScheduledSourceNode.rs",
    AudioScheduledSourceNode
);

web_feature!(
    "AudioSession",
    audio_session,
    "AudioSession.rs",
    AudioSession
);

web_feature!(
    "AudioSinkInfo",
    audio_sink_info,
    "AudioSinkInfo.rs",
    AudioSinkInfo
);

web_feature!(
    "AudioSinkOptions",
    audio_sink_options,
    "AudioSinkOptions.rs",
    AudioSinkOptions
);

web_feature!(
    "AudioTimestamp",
    audio_timestamp,
    "AudioTimestamp.rs",
    AudioTimestamp
);

web_feature!("AudioTrack", audio_track, "AudioTrack.rs", AudioTrack);

web_feature!(
    "AudioTrackList",
    audio_track_list,
    "AudioTrackList.rs",
    AudioTrackList
);

web_feature!(
    "AudioWorklet",
    audio_worklet,
    "AudioWorklet.rs",
    AudioWorklet
);

web_feature!(
    "AudioWorkletGlobalScope",
    audio_worklet_global_scope,
    "AudioWorkletGlobalScope.rs",
    AudioWorkletGlobalScope
);

web_feature!(
    "AudioWorkletNode",
    audio_worklet_node,
    "AudioWorkletNode.rs",
    AudioWorkletNode
);

web_feature!(
    "AudioWorkletNodeOptions",
    audio_worklet_node_options,
    "AudioWorkletNodeOptions.rs",
    AudioWorkletNodeOptions
);

web_feature!(
    "AudioWorkletProcessor",
    audio_worklet_processor,
    "AudioWorkletProcessor.rs",
    AudioWorkletProcessor
);

web_feature!(
    "AuthenticationExtensionsClientInputs",
    authentication_extensions_client_inputs,
    "AuthenticationExtensionsClientInputs.rs",
    AuthenticationExtensionsClientInputs
);

web_feature!(
    "AuthenticationExtensionsClientInputsJSON",
    authentication_extensions_client_inputs_json,
    "AuthenticationExtensionsClientInputsJSON.rs",
    AuthenticationExtensionsClientInputsJSON
);

web_feature!(
    "AuthenticationExtensionsClientOutputs",
    authentication_extensions_client_outputs,
    "AuthenticationExtensionsClientOutputs.rs",
    AuthenticationExtensionsClientOutputs
);

web_feature!(
    "AuthenticationExtensionsClientOutputsJSON",
    authentication_extensions_client_outputs_json,
    "AuthenticationExtensionsClientOutputsJSON.rs",
    AuthenticationExtensionsClientOutputsJSON
);

web_feature!(
    "AuthenticationExtensionsLargeBlobInputs",
    authentication_extensions_large_blob_inputs,
    "AuthenticationExtensionsLargeBlobInputs.rs",
    AuthenticationExtensionsLargeBlobInputs
);

web_feature!(
    "AuthenticationExtensionsLargeBlobInputsJSON",
    authentication_extensions_large_blob_inputs_json,
    "AuthenticationExtensionsLargeBlobInputsJSON.rs",
    AuthenticationExtensionsLargeBlobInputsJSON
);

web_feature!(
    "AuthenticationExtensionsLargeBlobOutputs",
    authentication_extensions_large_blob_outputs,
    "AuthenticationExtensionsLargeBlobOutputs.rs",
    AuthenticationExtensionsLargeBlobOutputs
);

web_feature!(
    "AuthenticationExtensionsLargeBlobOutputsJSON",
    authentication_extensions_large_blob_outputs_json,
    "AuthenticationExtensionsLargeBlobOutputsJSON.rs",
    AuthenticationExtensionsLargeBlobOutputsJSON
);

web_feature!(
    "AuthenticationExtensionsPRFInputs",
    authentication_extensions_prf_inputs,
    "AuthenticationExtensionsPRFInputs.rs",
    AuthenticationExtensionsPRFInputs
);

web_feature!(
    "AuthenticationExtensionsPRFInputsJSON",
    authentication_extensions_prf_inputs_json,
    "AuthenticationExtensionsPRFInputsJSON.rs",
    AuthenticationExtensionsPRFInputsJSON
);

web_feature!(
    "AuthenticationExtensionsPRFOutputs",
    authentication_extensions_prf_outputs,
    "AuthenticationExtensionsPRFOutputs.rs",
    AuthenticationExtensionsPRFOutputs
);

web_feature!(
    "AuthenticationExtensionsPRFOutputsJSON",
    authentication_extensions_prf_outputs_json,
    "AuthenticationExtensionsPRFOutputsJSON.rs",
    AuthenticationExtensionsPRFOutputsJSON
);

web_feature!(
    "AuthenticationExtensionsPRFValues",
    authentication_extensions_prf_values,
    "AuthenticationExtensionsPRFValues.rs",
    AuthenticationExtensionsPRFValues
);

web_feature!(
    "AuthenticationExtensionsPRFValuesJSON",
    authentication_extensions_prf_values_json,
    "AuthenticationExtensionsPRFValuesJSON.rs",
    AuthenticationExtensionsPRFValuesJSON
);

web_feature!(
    "AuthenticationExtensionsPaymentInputs",
    authentication_extensions_payment_inputs,
    "AuthenticationExtensionsPaymentInputs.rs",
    AuthenticationExtensionsPaymentInputs
);

web_feature!(
    "AuthenticationExtensionsPaymentOutputs",
    authentication_extensions_payment_outputs,
    "AuthenticationExtensionsPaymentOutputs.rs",
    AuthenticationExtensionsPaymentOutputs
);

web_feature!(
    "AuthenticationResponseJSON",
    authentication_response_json,
    "AuthenticationResponseJSON.rs",
    AuthenticationResponseJSON
);

web_feature!(
    "AuthenticatorAssertionResponse",
    authenticator_assertion_response,
    "AuthenticatorAssertionResponse.rs",
    AuthenticatorAssertionResponse
);

web_feature!(
    "AuthenticatorAssertionResponseJSON",
    authenticator_assertion_response_json,
    "AuthenticatorAssertionResponseJSON.rs",
    AuthenticatorAssertionResponseJSON
);

web_feature!(
    "AuthenticatorAttestationResponse",
    authenticator_attestation_response,
    "AuthenticatorAttestationResponse.rs",
    AuthenticatorAttestationResponse
);

web_feature!(
    "AuthenticatorAttestationResponseJSON",
    authenticator_attestation_response_json,
    "AuthenticatorAttestationResponseJSON.rs",
    AuthenticatorAttestationResponseJSON
);

web_feature!(
    "AuthenticatorResponse",
    authenticator_response,
    "AuthenticatorResponse.rs",
    AuthenticatorResponse
);

web_feature!(
    "AuthenticatorSelectionCriteria",
    authenticator_selection_criteria,
    "AuthenticatorSelectionCriteria.rs",
    AuthenticatorSelectionCriteria
);

web_feature!(
    "AvcEncoderConfig",
    avc_encoder_config,
    "AvcEncoderConfig.rs",
    AvcEncoderConfig
);

web_feature!(
    "BackgroundFetchEvent",
    background_fetch_event,
    "BackgroundFetchEvent.rs",
    BackgroundFetchEvent
);

web_feature!(
    "BackgroundFetchEventInit",
    background_fetch_event_init,
    "BackgroundFetchEventInit.rs",
    BackgroundFetchEventInit
);

web_feature!(
    "BackgroundFetchManager",
    background_fetch_manager,
    "BackgroundFetchManager.rs",
    BackgroundFetchManager
);

web_feature!(
    "BackgroundFetchOptions",
    background_fetch_options,
    "BackgroundFetchOptions.rs",
    BackgroundFetchOptions
);

web_feature!(
    "BackgroundFetchRecord",
    background_fetch_record,
    "BackgroundFetchRecord.rs",
    BackgroundFetchRecord
);

web_feature!(
    "BackgroundFetchRegistration",
    background_fetch_registration,
    "BackgroundFetchRegistration.rs",
    BackgroundFetchRegistration
);

web_feature!(
    "BackgroundFetchUIOptions",
    background_fetch_ui_options,
    "BackgroundFetchUIOptions.rs",
    BackgroundFetchUIOptions
);

web_feature!(
    "BackgroundFetchUpdateUIEvent",
    background_fetch_update_ui_event,
    "BackgroundFetchUpdateUIEvent.rs",
    BackgroundFetchUpdateUIEvent
);

web_feature!(
    "BackgroundSyncOptions",
    background_sync_options,
    "BackgroundSyncOptions.rs",
    BackgroundSyncOptions
);

web_feature!("BarProp", bar_prop, "BarProp.rs", BarProp);

web_feature!(
    "BarcodeDetector",
    barcode_detector,
    "BarcodeDetector.rs",
    BarcodeDetector
);

web_feature!(
    "BarcodeDetectorOptions",
    barcode_detector_options,
    "BarcodeDetectorOptions.rs",
    BarcodeDetectorOptions
);

web_feature!(
    "BaseAudioContext",
    base_audio_context,
    "BaseAudioContext.rs",
    BaseAudioContext
);

web_feature!(
    "BaseComputedKeyframe",
    base_computed_keyframe,
    "BaseComputedKeyframe.rs",
    BaseComputedKeyframe
);

web_feature!(
    "BaseKeyframe",
    base_keyframe,
    "BaseKeyframe.rs",
    BaseKeyframe
);

web_feature!(
    "BasePropertyIndexedKeyframe",
    base_property_indexed_keyframe,
    "BasePropertyIndexedKeyframe.rs",
    BasePropertyIndexedKeyframe
);

web_feature!("Baseline", baseline, "Baseline.rs", Baseline);

web_feature!(
    "BatteryManager",
    battery_manager,
    "BatteryManager.rs",
    BatteryManager
);

web_feature!(
    "BeforeInstallPromptEvent",
    before_install_prompt_event,
    "BeforeInstallPromptEvent.rs",
    BeforeInstallPromptEvent
);

web_feature!(
    "BeforeUnloadEvent",
    before_unload_event,
    "BeforeUnloadEvent.rs",
    BeforeUnloadEvent
);

web_feature!(
    "BiddingBrowserSignals",
    bidding_browser_signals,
    "BiddingBrowserSignals.rs",
    BiddingBrowserSignals
);

web_feature!(
    "BiquadFilterNode",
    biquad_filter_node,
    "BiquadFilterNode.rs",
    BiquadFilterNode
);

web_feature!(
    "BiquadFilterOptions",
    biquad_filter_options,
    "BiquadFilterOptions.rs",
    BiquadFilterOptions
);

web_feature!("Blob", blob, "Blob.rs", Blob);

web_feature!("BlobEvent", blob_event, "BlobEvent.rs", BlobEvent);

web_feature!(
    "BlobEventInit",
    blob_event_init,
    "BlobEventInit.rs",
    BlobEventInit
);

web_feature!(
    "BlobPropertyBag",
    blob_property_bag,
    "BlobPropertyBag.rs",
    BlobPropertyBag
);

web_feature!("Bluetooth", bluetooth, "Bluetooth.rs", Bluetooth);

web_feature!(
    "BluetoothAdvertisingEvent",
    bluetooth_advertising_event,
    "BluetoothAdvertisingEvent.rs",
    BluetoothAdvertisingEvent
);

web_feature!(
    "BluetoothAdvertisingEventInit",
    bluetooth_advertising_event_init,
    "BluetoothAdvertisingEventInit.rs",
    BluetoothAdvertisingEventInit
);

web_feature!(
    "BluetoothCharacteristicProperties",
    bluetooth_characteristic_properties,
    "BluetoothCharacteristicProperties.rs",
    BluetoothCharacteristicProperties
);

web_feature!(
    "BluetoothDataFilter",
    bluetooth_data_filter,
    "BluetoothDataFilter.rs",
    BluetoothDataFilter
);

web_feature!(
    "BluetoothDataFilterInit",
    bluetooth_data_filter_init,
    "BluetoothDataFilterInit.rs",
    BluetoothDataFilterInit
);

web_feature!(
    "BluetoothDevice",
    bluetooth_device,
    "BluetoothDevice.rs",
    BluetoothDevice
);

web_feature!(
    "BluetoothLEScan",
    bluetooth_le_scan,
    "BluetoothLEScan.rs",
    BluetoothLEScan
);

web_feature!(
    "BluetoothLEScanFilter",
    bluetooth_le_scan_filter,
    "BluetoothLEScanFilter.rs",
    BluetoothLEScanFilter
);

web_feature!(
    "BluetoothLEScanFilterInit",
    bluetooth_le_scan_filter_init,
    "BluetoothLEScanFilterInit.rs",
    BluetoothLEScanFilterInit
);

web_feature!(
    "BluetoothLEScanOptions",
    bluetooth_le_scan_options,
    "BluetoothLEScanOptions.rs",
    BluetoothLEScanOptions
);

web_feature!(
    "BluetoothLEScanPermissionDescriptor",
    bluetooth_le_scan_permission_descriptor,
    "BluetoothLEScanPermissionDescriptor.rs",
    BluetoothLEScanPermissionDescriptor
);

web_feature!(
    "BluetoothLEScanPermissionResult",
    bluetooth_le_scan_permission_result,
    "BluetoothLEScanPermissionResult.rs",
    BluetoothLEScanPermissionResult
);

web_feature!(
    "BluetoothManufacturerDataFilter",
    bluetooth_manufacturer_data_filter,
    "BluetoothManufacturerDataFilter.rs",
    BluetoothManufacturerDataFilter
);

web_feature!(
    "BluetoothManufacturerDataFilterInit",
    bluetooth_manufacturer_data_filter_init,
    "BluetoothManufacturerDataFilterInit.rs",
    BluetoothManufacturerDataFilterInit
);

web_feature!(
    "BluetoothManufacturerDataMap",
    bluetooth_manufacturer_data_map,
    "BluetoothManufacturerDataMap.rs",
    BluetoothManufacturerDataMap
);

web_feature!(
    "BluetoothPermissionDescriptor",
    bluetooth_permission_descriptor,
    "BluetoothPermissionDescriptor.rs",
    BluetoothPermissionDescriptor
);

web_feature!(
    "BluetoothPermissionResult",
    bluetooth_permission_result,
    "BluetoothPermissionResult.rs",
    BluetoothPermissionResult
);

web_feature!(
    "BluetoothPermissionStorage",
    bluetooth_permission_storage,
    "BluetoothPermissionStorage.rs",
    BluetoothPermissionStorage
);

web_feature!(
    "BluetoothRemoteGATTCharacteristic",
    bluetooth_remote_gatt_characteristic,
    "BluetoothRemoteGATTCharacteristic.rs",
    BluetoothRemoteGATTCharacteristic
);

web_feature!(
    "BluetoothRemoteGATTDescriptor",
    bluetooth_remote_gatt_descriptor,
    "BluetoothRemoteGATTDescriptor.rs",
    BluetoothRemoteGATTDescriptor
);

web_feature!(
    "BluetoothRemoteGATTServer",
    bluetooth_remote_gatt_server,
    "BluetoothRemoteGATTServer.rs",
    BluetoothRemoteGATTServer
);

web_feature!(
    "BluetoothRemoteGATTService",
    bluetooth_remote_gatt_service,
    "BluetoothRemoteGATTService.rs",
    BluetoothRemoteGATTService
);

web_feature!(
    "BluetoothServiceDataFilter",
    bluetooth_service_data_filter,
    "BluetoothServiceDataFilter.rs",
    BluetoothServiceDataFilter
);

web_feature!(
    "BluetoothServiceDataFilterInit",
    bluetooth_service_data_filter_init,
    "BluetoothServiceDataFilterInit.rs",
    BluetoothServiceDataFilterInit
);

web_feature!(
    "BluetoothServiceDataMap",
    bluetooth_service_data_map,
    "BluetoothServiceDataMap.rs",
    BluetoothServiceDataMap
);

web_feature!(
    "BluetoothUUID",
    bluetooth_uuid,
    "BluetoothUUID.rs",
    BluetoothUUID
);

web_feature!(
    "BoxQuadOptions",
    box_quad_options,
    "BoxQuadOptions.rs",
    BoxQuadOptions
);

web_feature!("BreakToken", break_token, "BreakToken.rs", BreakToken);

web_feature!(
    "BreakTokenOptions",
    break_token_options,
    "BreakTokenOptions.rs",
    BreakTokenOptions
);

web_feature!(
    "BroadcastChannel",
    broadcast_channel,
    "BroadcastChannel.rs",
    BroadcastChannel
);

web_feature!(
    "BrowserBoundSignature",
    browser_bound_signature,
    "BrowserBoundSignature.rs",
    BrowserBoundSignature
);

web_feature!(
    "BrowserCaptureMediaStreamTrack",
    browser_capture_media_stream_track,
    "BrowserCaptureMediaStreamTrack.rs",
    BrowserCaptureMediaStreamTrack
);

web_feature!(
    "BufferedChangeEvent",
    buffered_change_event,
    "BufferedChangeEvent.rs",
    BufferedChangeEvent
);

web_feature!(
    "BufferedChangeEventInit",
    buffered_change_event_init,
    "BufferedChangeEventInit.rs",
    BufferedChangeEventInit
);

web_feature!(
    "ByteLengthQueuingStrategy",
    byte_length_queuing_strategy,
    "ByteLengthQueuingStrategy.rs",
    ByteLengthQueuingStrategy
);

web_feature!(
    "CDATASection",
    cdata_section,
    "CDATASection.rs",
    CDATASection
);

web_feature!(
    "CSPViolationReportBody",
    csp_violation_report_body,
    "CSPViolationReportBody.rs",
    CSPViolationReportBody
);

web_feature!("CSS", css, "CSS.rs", CSS);

web_feature!(
    "CSSAnimation",
    css_animation,
    "CSSAnimation.rs",
    CSSAnimation
);

web_feature!("CSSColor", css_color, "CSSColor.rs", CSSColor);

web_feature!(
    "CSSColorProfileRule",
    css_color_profile_rule,
    "CSSColorProfileRule.rs",
    CSSColorProfileRule
);

web_feature!(
    "CSSColorValue",
    css_color_value,
    "CSSColorValue.rs",
    CSSColorValue
);

web_feature!(
    "CSSConditionRule",
    css_condition_rule,
    "CSSConditionRule.rs",
    CSSConditionRule
);

web_feature!(
    "CSSContainerRule",
    css_container_rule,
    "CSSContainerRule.rs",
    CSSContainerRule
);

web_feature!(
    "CSSCounterStyleRule",
    css_counter_style_rule,
    "CSSCounterStyleRule.rs",
    CSSCounterStyleRule
);

web_feature!(
    "CSSFontFaceDescriptors",
    css_font_face_descriptors,
    "CSSFontFaceDescriptors.rs",
    CSSFontFaceDescriptors
);

web_feature!(
    "CSSFontFaceRule",
    css_font_face_rule,
    "CSSFontFaceRule.rs",
    CSSFontFaceRule
);

web_feature!(
    "CSSFontFeatureValuesMap",
    css_font_feature_values_map,
    "CSSFontFeatureValuesMap.rs",
    CSSFontFeatureValuesMap
);

web_feature!(
    "CSSFontFeatureValuesRule",
    css_font_feature_values_rule,
    "CSSFontFeatureValuesRule.rs",
    CSSFontFeatureValuesRule
);

web_feature!(
    "CSSFontPaletteValuesRule",
    css_font_palette_values_rule,
    "CSSFontPaletteValuesRule.rs",
    CSSFontPaletteValuesRule
);

web_feature!(
    "CSSFunctionDeclarations",
    css_function_declarations,
    "CSSFunctionDeclarations.rs",
    CSSFunctionDeclarations
);

web_feature!(
    "CSSFunctionDescriptors",
    css_function_descriptors,
    "CSSFunctionDescriptors.rs",
    CSSFunctionDescriptors
);

web_feature!(
    "CSSFunctionRule",
    css_function_rule,
    "CSSFunctionRule.rs",
    CSSFunctionRule
);

web_feature!(
    "CSSGroupingRule",
    css_grouping_rule,
    "CSSGroupingRule.rs",
    CSSGroupingRule
);

web_feature!("CSSHSL", csshsl, "CSSHSL.rs", CSSHSL);

web_feature!("CSSHWB", csshwb, "CSSHWB.rs", CSSHWB);

web_feature!(
    "CSSImageValue",
    css_image_value,
    "CSSImageValue.rs",
    CSSImageValue
);

web_feature!(
    "CSSImportRule",
    css_import_rule,
    "CSSImportRule.rs",
    CSSImportRule
);

web_feature!(
    "CSSKeyframeRule",
    css_keyframe_rule,
    "CSSKeyframeRule.rs",
    CSSKeyframeRule
);

web_feature!(
    "CSSKeyframesRule",
    css_keyframes_rule,
    "CSSKeyframesRule.rs",
    CSSKeyframesRule
);

web_feature!(
    "CSSKeywordValue",
    css_keyword_value,
    "CSSKeywordValue.rs",
    CSSKeywordValue
);

web_feature!("CSSLCH", csslch, "CSSLCH.rs", CSSLCH);

web_feature!("CSSLab", css_lab, "CSSLab.rs", CSSLab);

web_feature!(
    "CSSLayerBlockRule",
    css_layer_block_rule,
    "CSSLayerBlockRule.rs",
    CSSLayerBlockRule
);

web_feature!(
    "CSSLayerStatementRule",
    css_layer_statement_rule,
    "CSSLayerStatementRule.rs",
    CSSLayerStatementRule
);

web_feature!(
    "CSSMarginRule",
    css_margin_rule,
    "CSSMarginRule.rs",
    CSSMarginRule
);

web_feature!(
    "CSSMathClamp",
    css_math_clamp,
    "CSSMathClamp.rs",
    CSSMathClamp
);

web_feature!(
    "CSSMathInvert",
    css_math_invert,
    "CSSMathInvert.rs",
    CSSMathInvert
);

web_feature!("CSSMathMax", css_math_max, "CSSMathMax.rs", CSSMathMax);

web_feature!("CSSMathMin", css_math_min, "CSSMathMin.rs", CSSMathMin);

web_feature!(
    "CSSMathNegate",
    css_math_negate,
    "CSSMathNegate.rs",
    CSSMathNegate
);

web_feature!(
    "CSSMathProduct",
    css_math_product,
    "CSSMathProduct.rs",
    CSSMathProduct
);

web_feature!("CSSMathSum", css_math_sum, "CSSMathSum.rs", CSSMathSum);

web_feature!(
    "CSSMathValue",
    css_math_value,
    "CSSMathValue.rs",
    CSSMathValue
);

web_feature!(
    "CSSMatrixComponent",
    css_matrix_component,
    "CSSMatrixComponent.rs",
    CSSMatrixComponent
);

web_feature!(
    "CSSMatrixComponentOptions",
    css_matrix_component_options,
    "CSSMatrixComponentOptions.rs",
    CSSMatrixComponentOptions
);

web_feature!(
    "CSSMediaRule",
    css_media_rule,
    "CSSMediaRule.rs",
    CSSMediaRule
);

web_feature!(
    "CSSNamespaceRule",
    css_namespace_rule,
    "CSSNamespaceRule.rs",
    CSSNamespaceRule
);

web_feature!(
    "CSSNestedDeclarations",
    css_nested_declarations,
    "CSSNestedDeclarations.rs",
    CSSNestedDeclarations
);

web_feature!(
    "CSSNumericArray",
    css_numeric_array,
    "CSSNumericArray.rs",
    CSSNumericArray
);

web_feature!(
    "CSSNumericType",
    css_numeric_type,
    "CSSNumericType.rs",
    CSSNumericType
);

web_feature!(
    "CSSNumericValue",
    css_numeric_value,
    "CSSNumericValue.rs",
    CSSNumericValue
);

web_feature!("CSSOKLCH", cssoklch, "CSSOKLCH.rs", CSSOKLCH);

web_feature!("CSSOKLab", cssok_lab, "CSSOKLab.rs", CSSOKLab);

web_feature!(
    "CSSPageDescriptors",
    css_page_descriptors,
    "CSSPageDescriptors.rs",
    CSSPageDescriptors
);

web_feature!("CSSPageRule", css_page_rule, "CSSPageRule.rs", CSSPageRule);

web_feature!(
    "CSSParserAtRule",
    css_parser_at_rule,
    "CSSParserAtRule.rs",
    CSSParserAtRule
);

web_feature!(
    "CSSParserBlock",
    css_parser_block,
    "CSSParserBlock.rs",
    CSSParserBlock
);

web_feature!(
    "CSSParserDeclaration",
    css_parser_declaration,
    "CSSParserDeclaration.rs",
    CSSParserDeclaration
);

web_feature!(
    "CSSParserFunction",
    css_parser_function,
    "CSSParserFunction.rs",
    CSSParserFunction
);

web_feature!(
    "CSSParserOptions",
    css_parser_options,
    "CSSParserOptions.rs",
    CSSParserOptions
);

web_feature!(
    "CSSParserQualifiedRule",
    css_parser_qualified_rule,
    "CSSParserQualifiedRule.rs",
    CSSParserQualifiedRule
);

web_feature!(
    "CSSParserRule",
    css_parser_rule,
    "CSSParserRule.rs",
    CSSParserRule
);

web_feature!(
    "CSSParserValue",
    css_parser_value,
    "CSSParserValue.rs",
    CSSParserValue
);

web_feature!(
    "CSSPerspective",
    css_perspective,
    "CSSPerspective.rs",
    CSSPerspective
);

web_feature!(
    "CSSPositionTryDescriptors",
    css_position_try_descriptors,
    "CSSPositionTryDescriptors.rs",
    CSSPositionTryDescriptors
);

web_feature!(
    "CSSPositionTryRule",
    css_position_try_rule,
    "CSSPositionTryRule.rs",
    CSSPositionTryRule
);

web_feature!(
    "CSSPropertyRule",
    css_property_rule,
    "CSSPropertyRule.rs",
    CSSPropertyRule
);

web_feature!(
    "CSSPseudoElement",
    css_pseudo_element,
    "CSSPseudoElement.rs",
    CSSPseudoElement
);

web_feature!("CSSRGB", cssrgb, "CSSRGB.rs", CSSRGB);

web_feature!("CSSRotate", css_rotate, "CSSRotate.rs", CSSRotate);

web_feature!("CSSRule", css_rule, "CSSRule.rs", CSSRule);

web_feature!("CSSRuleList", css_rule_list, "CSSRuleList.rs", CSSRuleList);

web_feature!("CSSScale", css_scale, "CSSScale.rs", CSSScale);

web_feature!(
    "CSSScopeRule",
    css_scope_rule,
    "CSSScopeRule.rs",
    CSSScopeRule
);

web_feature!("CSSSkew", css_skew, "CSSSkew.rs", CSSSkew);

web_feature!("CSSSkewX", css_skew_x, "CSSSkewX.rs", CSSSkewX);

web_feature!("CSSSkewY", css_skew_y, "CSSSkewY.rs", CSSSkewY);

web_feature!(
    "CSSStartingStyleRule",
    css_starting_style_rule,
    "CSSStartingStyleRule.rs",
    CSSStartingStyleRule
);

web_feature!(
    "CSSStyleDeclaration",
    css_style_declaration,
    "CSSStyleDeclaration.rs",
    CSSStyleDeclaration
);

web_feature!(
    "CSSStyleProperties",
    css_style_properties,
    "CSSStyleProperties.rs",
    CSSStyleProperties
);

web_feature!(
    "CSSStyleRule",
    css_style_rule,
    "CSSStyleRule.rs",
    CSSStyleRule
);

web_feature!(
    "CSSStyleSheet",
    css_style_sheet,
    "CSSStyleSheet.rs",
    CSSStyleSheet
);

web_feature!(
    "CSSStyleSheetInit",
    css_style_sheet_init,
    "CSSStyleSheetInit.rs",
    CSSStyleSheetInit
);

web_feature!(
    "CSSStyleValue",
    css_style_value,
    "CSSStyleValue.rs",
    CSSStyleValue
);

web_feature!(
    "CSSSupportsRule",
    css_supports_rule,
    "CSSSupportsRule.rs",
    CSSSupportsRule
);

web_feature!(
    "CSSTransformComponent",
    css_transform_component,
    "CSSTransformComponent.rs",
    CSSTransformComponent
);

web_feature!(
    "CSSTransformValue",
    css_transform_value,
    "CSSTransformValue.rs",
    CSSTransformValue
);

web_feature!(
    "CSSTransition",
    css_transition,
    "CSSTransition.rs",
    CSSTransition
);

web_feature!(
    "CSSTranslate",
    css_translate,
    "CSSTranslate.rs",
    CSSTranslate
);

web_feature!(
    "CSSUnitValue",
    css_unit_value,
    "CSSUnitValue.rs",
    CSSUnitValue
);

web_feature!(
    "CSSUnparsedValue",
    css_unparsed_value,
    "CSSUnparsedValue.rs",
    CSSUnparsedValue
);

web_feature!(
    "CSSVariableReferenceValue",
    css_variable_reference_value,
    "CSSVariableReferenceValue.rs",
    CSSVariableReferenceValue
);

web_feature!(
    "CSSViewTransitionRule",
    css_view_transition_rule,
    "CSSViewTransitionRule.rs",
    CSSViewTransitionRule
);

web_feature!("Cache", cache, "Cache.rs", Cache);

web_feature!(
    "CacheQueryOptions",
    cache_query_options,
    "CacheQueryOptions.rs",
    CacheQueryOptions
);

web_feature!(
    "CacheStorage",
    cache_storage,
    "CacheStorage.rs",
    CacheStorage
);

web_feature!(
    "CameraDevicePermissionDescriptor",
    camera_device_permission_descriptor,
    "CameraDevicePermissionDescriptor.rs",
    CameraDevicePermissionDescriptor
);

web_feature!(
    "CanMakePaymentEvent",
    can_make_payment_event,
    "CanMakePaymentEvent.rs",
    CanMakePaymentEvent
);

web_feature!(
    "CanvasCaptureMediaStreamTrack",
    canvas_capture_media_stream_track,
    "CanvasCaptureMediaStreamTrack.rs",
    CanvasCaptureMediaStreamTrack
);

web_feature!(
    "CanvasGradient",
    canvas_gradient,
    "CanvasGradient.rs",
    CanvasGradient
);

web_feature!(
    "CanvasPattern",
    canvas_pattern,
    "CanvasPattern.rs",
    CanvasPattern
);

web_feature!(
    "CanvasRenderingContext2D",
    canvas_rendering_context2d,
    "CanvasRenderingContext2D.rs",
    CanvasRenderingContext2D
);

web_feature!(
    "CanvasRenderingContext2DSettings",
    canvas_rendering_context2d_settings,
    "CanvasRenderingContext2DSettings.rs",
    CanvasRenderingContext2DSettings
);

web_feature!(
    "CaptureActionEvent",
    capture_action_event,
    "CaptureActionEvent.rs",
    CaptureActionEvent
);

web_feature!(
    "CaptureActionEventInit",
    capture_action_event_init,
    "CaptureActionEventInit.rs",
    CaptureActionEventInit
);

web_feature!(
    "CaptureController",
    capture_controller,
    "CaptureController.rs",
    CaptureController
);

web_feature!(
    "CaptureHandle",
    capture_handle,
    "CaptureHandle.rs",
    CaptureHandle
);

web_feature!(
    "CaptureHandleConfig",
    capture_handle_config,
    "CaptureHandleConfig.rs",
    CaptureHandleConfig
);

web_feature!(
    "CapturedMouseEvent",
    captured_mouse_event,
    "CapturedMouseEvent.rs",
    CapturedMouseEvent
);

web_feature!(
    "CapturedMouseEventInit",
    captured_mouse_event_init,
    "CapturedMouseEventInit.rs",
    CapturedMouseEventInit
);

web_feature!(
    "CaretPosition",
    caret_position,
    "CaretPosition.rs",
    CaretPosition
);

web_feature!(
    "CaretPositionFromPointOptions",
    caret_position_from_point_options,
    "CaretPositionFromPointOptions.rs",
    CaretPositionFromPointOptions
);

web_feature!(
    "ChannelMergerNode",
    channel_merger_node,
    "ChannelMergerNode.rs",
    ChannelMergerNode
);

web_feature!(
    "ChannelMergerOptions",
    channel_merger_options,
    "ChannelMergerOptions.rs",
    ChannelMergerOptions
);

web_feature!(
    "ChannelSplitterNode",
    channel_splitter_node,
    "ChannelSplitterNode.rs",
    ChannelSplitterNode
);

web_feature!(
    "ChannelSplitterOptions",
    channel_splitter_options,
    "ChannelSplitterOptions.rs",
    ChannelSplitterOptions
);

web_feature!(
    "ChapterInformation",
    chapter_information,
    "ChapterInformation.rs",
    ChapterInformation
);

web_feature!(
    "ChapterInformationInit",
    chapter_information_init,
    "ChapterInformationInit.rs",
    ChapterInformationInit
);

web_feature!(
    "CharacterBoundsUpdateEvent",
    character_bounds_update_event,
    "CharacterBoundsUpdateEvent.rs",
    CharacterBoundsUpdateEvent
);

web_feature!(
    "CharacterBoundsUpdateEventInit",
    character_bounds_update_event_init,
    "CharacterBoundsUpdateEventInit.rs",
    CharacterBoundsUpdateEventInit
);

web_feature!(
    "CharacterData",
    character_data,
    "CharacterData.rs",
    CharacterData
);

web_feature!(
    "CheckVisibilityOptions",
    check_visibility_options,
    "CheckVisibilityOptions.rs",
    CheckVisibilityOptions
);

web_feature!(
    "ChildBreakToken",
    child_break_token,
    "ChildBreakToken.rs",
    ChildBreakToken
);

web_feature!("Client", client, "Client.rs", Client);

web_feature!(
    "ClientQueryOptions",
    client_query_options,
    "ClientQueryOptions.rs",
    ClientQueryOptions
);

web_feature!("Clients", clients, "Clients.rs", Clients);

web_feature!("Clipboard", clipboard, "Clipboard.rs", Clipboard);

web_feature!(
    "ClipboardEvent",
    clipboard_event,
    "ClipboardEvent.rs",
    ClipboardEvent
);

web_feature!(
    "ClipboardEventInit",
    clipboard_event_init,
    "ClipboardEventInit.rs",
    ClipboardEventInit
);

web_feature!(
    "ClipboardItem",
    clipboard_item,
    "ClipboardItem.rs",
    ClipboardItem
);

web_feature!(
    "ClipboardItemOptions",
    clipboard_item_options,
    "ClipboardItemOptions.rs",
    ClipboardItemOptions
);

web_feature!(
    "ClipboardPermissionDescriptor",
    clipboard_permission_descriptor,
    "ClipboardPermissionDescriptor.rs",
    ClipboardPermissionDescriptor
);

web_feature!(
    "ClipboardUnsanitizedFormats",
    clipboard_unsanitized_formats,
    "ClipboardUnsanitizedFormats.rs",
    ClipboardUnsanitizedFormats
);

web_feature!("CloseEvent", close_event, "CloseEvent.rs", CloseEvent);

web_feature!(
    "CloseEventInit",
    close_event_init,
    "CloseEventInit.rs",
    CloseEventInit
);

web_feature!(
    "CloseWatcher",
    close_watcher,
    "CloseWatcher.rs",
    CloseWatcher
);

web_feature!(
    "CloseWatcherOptions",
    close_watcher_options,
    "CloseWatcherOptions.rs",
    CloseWatcherOptions
);

web_feature!(
    "CollectedClientAdditionalPaymentData",
    collected_client_additional_payment_data,
    "CollectedClientAdditionalPaymentData.rs",
    CollectedClientAdditionalPaymentData
);

web_feature!(
    "CollectedClientAdditionalPaymentRegistrationData",
    collected_client_additional_payment_registration_data,
    "CollectedClientAdditionalPaymentRegistrationData.rs",
    CollectedClientAdditionalPaymentRegistrationData
);

web_feature!(
    "CollectedClientData",
    collected_client_data,
    "CollectedClientData.rs",
    CollectedClientData
);

web_feature!(
    "CollectedClientPaymentData",
    collected_client_payment_data,
    "CollectedClientPaymentData.rs",
    CollectedClientPaymentData
);

web_feature!(
    "ColorSelectionOptions",
    color_selection_options,
    "ColorSelectionOptions.rs",
    ColorSelectionOptions
);

web_feature!(
    "ColorSelectionResult",
    color_selection_result,
    "ColorSelectionResult.rs",
    ColorSelectionResult
);

web_feature!(
    "CommandEvent",
    command_event,
    "CommandEvent.rs",
    CommandEvent
);

web_feature!(
    "CommandEventInit",
    command_event_init,
    "CommandEventInit.rs",
    CommandEventInit
);

web_feature!("Comment", comment, "Comment.rs", Comment);

web_feature!(
    "CompositionEvent",
    composition_event,
    "CompositionEvent.rs",
    CompositionEvent
);

web_feature!(
    "CompositionEventInit",
    composition_event_init,
    "CompositionEventInit.rs",
    CompositionEventInit
);

web_feature!(
    "CompressionStream",
    compression_stream,
    "CompressionStream.rs",
    CompressionStream
);

web_feature!(
    "ComputedEffectTiming",
    computed_effect_timing,
    "ComputedEffectTiming.rs",
    ComputedEffectTiming
);

web_feature!(
    "ConstantSourceNode",
    constant_source_node,
    "ConstantSourceNode.rs",
    ConstantSourceNode
);

web_feature!(
    "ConstantSourceOptions",
    constant_source_options,
    "ConstantSourceOptions.rs",
    ConstantSourceOptions
);

web_feature!(
    "ConstrainBooleanOrDOMStringParameters",
    constrain_boolean_or_dom_string_parameters,
    "ConstrainBooleanOrDOMStringParameters.rs",
    ConstrainBooleanOrDOMStringParameters
);

web_feature!(
    "ConstrainBooleanParameters",
    constrain_boolean_parameters,
    "ConstrainBooleanParameters.rs",
    ConstrainBooleanParameters
);

web_feature!(
    "ConstrainDOMStringParameters",
    constrain_dom_string_parameters,
    "ConstrainDOMStringParameters.rs",
    ConstrainDOMStringParameters
);

web_feature!(
    "ConstrainDoubleRange",
    constrain_double_range,
    "ConstrainDoubleRange.rs",
    ConstrainDoubleRange
);

web_feature!(
    "ConstrainPoint2DParameters",
    constrain_point2d_parameters,
    "ConstrainPoint2DParameters.rs",
    ConstrainPoint2DParameters
);

web_feature!(
    "ConstrainULongRange",
    constrain_u_long_range,
    "ConstrainULongRange.rs",
    ConstrainULongRange
);

web_feature!(
    "ContactAddress",
    contact_address,
    "ContactAddress.rs",
    ContactAddress
);

web_feature!("ContactInfo", contact_info, "ContactInfo.rs", ContactInfo);

web_feature!(
    "ContactsManager",
    contacts_manager,
    "ContactsManager.rs",
    ContactsManager
);

web_feature!(
    "ContactsSelectOptions",
    contacts_select_options,
    "ContactsSelectOptions.rs",
    ContactsSelectOptions
);

web_feature!(
    "ContentDescription",
    content_description,
    "ContentDescription.rs",
    ContentDescription
);

web_feature!(
    "ContentIndex",
    content_index,
    "ContentIndex.rs",
    ContentIndex
);

web_feature!(
    "ContentIndexEvent",
    content_index_event,
    "ContentIndexEvent.rs",
    ContentIndexEvent
);

web_feature!(
    "ContentIndexEventInit",
    content_index_event_init,
    "ContentIndexEventInit.rs",
    ContentIndexEventInit
);

web_feature!(
    "ContentVisibilityAutoStateChangeEvent",
    content_visibility_auto_state_change_event,
    "ContentVisibilityAutoStateChangeEvent.rs",
    ContentVisibilityAutoStateChangeEvent
);

web_feature!(
    "ContentVisibilityAutoStateChangeEventInit",
    content_visibility_auto_state_change_event_init,
    "ContentVisibilityAutoStateChangeEventInit.rs",
    ContentVisibilityAutoStateChangeEventInit
);

web_feature!(
    "ConvertCoordinateOptions",
    convert_coordinate_options,
    "ConvertCoordinateOptions.rs",
    ConvertCoordinateOptions
);

web_feature!(
    "ConvolverNode",
    convolver_node,
    "ConvolverNode.rs",
    ConvolverNode
);

web_feature!(
    "ConvolverOptions",
    convolver_options,
    "ConvolverOptions.rs",
    ConvolverOptions
);

web_feature!(
    "CookieChangeEvent",
    cookie_change_event,
    "CookieChangeEvent.rs",
    CookieChangeEvent
);

web_feature!(
    "CookieChangeEventInit",
    cookie_change_event_init,
    "CookieChangeEventInit.rs",
    CookieChangeEventInit
);

web_feature!("CookieInit", cookie_init, "CookieInit.rs", CookieInit);

web_feature!(
    "CookieListItem",
    cookie_list_item,
    "CookieListItem.rs",
    CookieListItem
);

web_feature!("CookieStore", cookie_store, "CookieStore.rs", CookieStore);

web_feature!(
    "CookieStoreDeleteOptions",
    cookie_store_delete_options,
    "CookieStoreDeleteOptions.rs",
    CookieStoreDeleteOptions
);

web_feature!(
    "CookieStoreGetOptions",
    cookie_store_get_options,
    "CookieStoreGetOptions.rs",
    CookieStoreGetOptions
);

web_feature!(
    "CookieStoreManager",
    cookie_store_manager,
    "CookieStoreManager.rs",
    CookieStoreManager
);

web_feature!(
    "CountQueuingStrategy",
    count_queuing_strategy,
    "CountQueuingStrategy.rs",
    CountQueuingStrategy
);

web_feature!(
    "CrashReportBody",
    crash_report_body,
    "CrashReportBody.rs",
    CrashReportBody
);

web_feature!(
    "CreateMonitor",
    create_monitor,
    "CreateMonitor.rs",
    CreateMonitor
);

web_feature!("Credential", credential, "Credential.rs", Credential);

web_feature!(
    "CredentialCreationOptions",
    credential_creation_options,
    "CredentialCreationOptions.rs",
    CredentialCreationOptions
);

web_feature!(
    "CredentialData",
    credential_data,
    "CredentialData.rs",
    CredentialData
);

web_feature!(
    "CredentialPropertiesOutput",
    credential_properties_output,
    "CredentialPropertiesOutput.rs",
    CredentialPropertiesOutput
);

web_feature!(
    "CredentialRequestOptions",
    credential_request_options,
    "CredentialRequestOptions.rs",
    CredentialRequestOptions
);

web_feature!(
    "CredentialsContainer",
    credentials_container,
    "CredentialsContainer.rs",
    CredentialsContainer
);

web_feature!("CropTarget", crop_target, "CropTarget.rs", CropTarget);

web_feature!("Crypto", crypto, "Crypto.rs", Crypto);

web_feature!("CryptoKey", crypto_key, "CryptoKey.rs", CryptoKey);

web_feature!(
    "CryptoKeyPair",
    crypto_key_pair,
    "CryptoKeyPair.rs",
    CryptoKeyPair
);

web_feature!(
    "CurrentUserDetailsOptions",
    current_user_details_options,
    "CurrentUserDetailsOptions.rs",
    CurrentUserDetailsOptions
);

web_feature!(
    "CustomElementRegistry",
    custom_element_registry,
    "CustomElementRegistry.rs",
    CustomElementRegistry
);

web_feature!("CustomEvent", custom_event, "CustomEvent.rs", CustomEvent);

web_feature!(
    "CustomEventInit",
    custom_event_init,
    "CustomEventInit.rs",
    CustomEventInit
);

web_feature!(
    "CustomStateSet",
    custom_state_set,
    "CustomStateSet.rs",
    CustomStateSet
);

web_feature!(
    "DOMException",
    dom_exception,
    "DOMException.rs",
    DOMException
);

web_feature!(
    "DOMImplementation",
    dom_implementation,
    "DOMImplementation.rs",
    DOMImplementation
);

web_feature!("DOMMatrix", dom_matrix, "DOMMatrix.rs", DOMMatrix);

web_feature!(
    "DOMMatrix2DInit",
    dom_matrix2d_init,
    "DOMMatrix2DInit.rs",
    DOMMatrix2DInit
);

web_feature!(
    "DOMMatrixInit",
    dom_matrix_init,
    "DOMMatrixInit.rs",
    DOMMatrixInit
);

web_feature!(
    "DOMMatrixReadOnly",
    dom_matrix_read_only,
    "DOMMatrixReadOnly.rs",
    DOMMatrixReadOnly
);

web_feature!("DOMParser", dom_parser, "DOMParser.rs", DOMParser);

web_feature!("DOMPoint", dom_point, "DOMPoint.rs", DOMPoint);

web_feature!(
    "DOMPointInit",
    dom_point_init,
    "DOMPointInit.rs",
    DOMPointInit
);

web_feature!(
    "DOMPointReadOnly",
    dom_point_read_only,
    "DOMPointReadOnly.rs",
    DOMPointReadOnly
);

web_feature!("DOMQuad", dom_quad, "DOMQuad.rs", DOMQuad);

web_feature!("DOMQuadInit", dom_quad_init, "DOMQuadInit.rs", DOMQuadInit);

web_feature!("DOMRect", dom_rect, "DOMRect.rs", DOMRect);

web_feature!("DOMRectInit", dom_rect_init, "DOMRectInit.rs", DOMRectInit);

web_feature!("DOMRectList", dom_rect_list, "DOMRectList.rs", DOMRectList);

web_feature!(
    "DOMRectReadOnly",
    dom_rect_read_only,
    "DOMRectReadOnly.rs",
    DOMRectReadOnly
);

web_feature!(
    "DOMStringList",
    dom_string_list,
    "DOMStringList.rs",
    DOMStringList
);

web_feature!(
    "DOMStringMap",
    dom_string_map,
    "DOMStringMap.rs",
    DOMStringMap
);

web_feature!(
    "DOMTokenList",
    dom_token_list,
    "DOMTokenList.rs",
    DOMTokenList
);

web_feature!("DataCue", data_cue, "DataCue.rs", DataCue);

web_feature!(
    "DataTransfer",
    data_transfer,
    "DataTransfer.rs",
    DataTransfer
);

web_feature!(
    "DataTransferItem",
    data_transfer_item,
    "DataTransferItem.rs",
    DataTransferItem
);

web_feature!(
    "DataTransferItemList",
    data_transfer_item_list,
    "DataTransferItemList.rs",
    DataTransferItemList
);

web_feature!(
    "DecompressionStream",
    decompression_stream,
    "DecompressionStream.rs",
    DecompressionStream
);

web_feature!(
    "DedicatedWorkerGlobalScope",
    dedicated_worker_global_scope,
    "DedicatedWorkerGlobalScope.rs",
    DedicatedWorkerGlobalScope
);

web_feature!("DelayNode", delay_node, "DelayNode.rs", DelayNode);

web_feature!(
    "DelayOptions",
    delay_options,
    "DelayOptions.rs",
    DelayOptions
);

web_feature!(
    "DelegatedInkTrailPresenter",
    delegated_ink_trail_presenter,
    "DelegatedInkTrailPresenter.rs",
    DelegatedInkTrailPresenter
);

web_feature!(
    "DeprecationReportBody",
    deprecation_report_body,
    "DeprecationReportBody.rs",
    DeprecationReportBody
);

web_feature!(
    "DetectedBarcode",
    detected_barcode,
    "DetectedBarcode.rs",
    DetectedBarcode
);

web_feature!(
    "DetectedFace",
    detected_face,
    "DetectedFace.rs",
    DetectedFace
);

web_feature!(
    "DetectedText",
    detected_text,
    "DetectedText.rs",
    DetectedText
);

web_feature!(
    "DeviceChangeEvent",
    device_change_event,
    "DeviceChangeEvent.rs",
    DeviceChangeEvent
);

web_feature!(
    "DeviceChangeEventInit",
    device_change_event_init,
    "DeviceChangeEventInit.rs",
    DeviceChangeEventInit
);

web_feature!(
    "DeviceMotionEvent",
    device_motion_event,
    "DeviceMotionEvent.rs",
    DeviceMotionEvent
);

web_feature!(
    "DeviceMotionEventAcceleration",
    device_motion_event_acceleration,
    "DeviceMotionEventAcceleration.rs",
    DeviceMotionEventAcceleration
);

web_feature!(
    "DeviceMotionEventAccelerationInit",
    device_motion_event_acceleration_init,
    "DeviceMotionEventAccelerationInit.rs",
    DeviceMotionEventAccelerationInit
);

web_feature!(
    "DeviceMotionEventInit",
    device_motion_event_init,
    "DeviceMotionEventInit.rs",
    DeviceMotionEventInit
);

web_feature!(
    "DeviceMotionEventRotationRate",
    device_motion_event_rotation_rate,
    "DeviceMotionEventRotationRate.rs",
    DeviceMotionEventRotationRate
);

web_feature!(
    "DeviceMotionEventRotationRateInit",
    device_motion_event_rotation_rate_init,
    "DeviceMotionEventRotationRateInit.rs",
    DeviceMotionEventRotationRateInit
);

web_feature!(
    "DeviceOrientationEvent",
    device_orientation_event,
    "DeviceOrientationEvent.rs",
    DeviceOrientationEvent
);

web_feature!(
    "DeviceOrientationEventInit",
    device_orientation_event_init,
    "DeviceOrientationEventInit.rs",
    DeviceOrientationEventInit
);

web_feature!(
    "DevicePosture",
    device_posture,
    "DevicePosture.rs",
    DevicePosture
);

web_feature!(
    "DigitalCredential",
    digital_credential,
    "DigitalCredential.rs",
    DigitalCredential
);

web_feature!(
    "DigitalCredentialCreateRequest",
    digital_credential_create_request,
    "DigitalCredentialCreateRequest.rs",
    DigitalCredentialCreateRequest
);

web_feature!(
    "DigitalCredentialCreationOptions",
    digital_credential_creation_options,
    "DigitalCredentialCreationOptions.rs",
    DigitalCredentialCreationOptions
);

web_feature!(
    "DigitalCredentialGetRequest",
    digital_credential_get_request,
    "DigitalCredentialGetRequest.rs",
    DigitalCredentialGetRequest
);

web_feature!(
    "DigitalCredentialRequestOptions",
    digital_credential_request_options,
    "DigitalCredentialRequestOptions.rs",
    DigitalCredentialRequestOptions
);

web_feature!(
    "DigitalGoodsService",
    digital_goods_service,
    "DigitalGoodsService.rs",
    DigitalGoodsService
);

web_feature!(
    "DirectFromSellerSignalsForBuyer",
    direct_from_seller_signals_for_buyer,
    "DirectFromSellerSignalsForBuyer.rs",
    DirectFromSellerSignalsForBuyer
);

web_feature!(
    "DirectFromSellerSignalsForSeller",
    direct_from_seller_signals_for_seller,
    "DirectFromSellerSignalsForSeller.rs",
    DirectFromSellerSignalsForSeller
);

web_feature!(
    "DirectoryPickerOptions",
    directory_picker_options,
    "DirectoryPickerOptions.rs",
    DirectoryPickerOptions
);

web_feature!(
    "DisconnectedAccount",
    disconnected_account,
    "DisconnectedAccount.rs",
    DisconnectedAccount
);

web_feature!(
    "DisplayMediaStreamOptions",
    display_media_stream_options,
    "DisplayMediaStreamOptions.rs",
    DisplayMediaStreamOptions
);

web_feature!("Document", document, "Document.rs", Document);

web_feature!(
    "DocumentFragment",
    document_fragment,
    "DocumentFragment.rs",
    DocumentFragment
);

web_feature!(
    "DocumentPictureInPicture",
    document_picture_in_picture,
    "DocumentPictureInPicture.rs",
    DocumentPictureInPicture
);

web_feature!(
    "DocumentPictureInPictureEvent",
    document_picture_in_picture_event,
    "DocumentPictureInPictureEvent.rs",
    DocumentPictureInPictureEvent
);

web_feature!(
    "DocumentPictureInPictureEventInit",
    document_picture_in_picture_event_init,
    "DocumentPictureInPictureEventInit.rs",
    DocumentPictureInPictureEventInit
);

web_feature!(
    "DocumentPictureInPictureOptions",
    document_picture_in_picture_options,
    "DocumentPictureInPictureOptions.rs",
    DocumentPictureInPictureOptions
);

web_feature!(
    "DocumentTimeline",
    document_timeline,
    "DocumentTimeline.rs",
    DocumentTimeline
);

web_feature!(
    "DocumentTimelineOptions",
    document_timeline_options,
    "DocumentTimelineOptions.rs",
    DocumentTimelineOptions
);

web_feature!(
    "DocumentType",
    document_type,
    "DocumentType.rs",
    DocumentType
);

web_feature!("DoubleRange", double_range, "DoubleRange.rs", DoubleRange);

web_feature!("DragEvent", drag_event, "DragEvent.rs", DragEvent);

web_feature!(
    "DragEventInit",
    drag_event_init,
    "DragEventInit.rs",
    DragEventInit
);

web_feature!(
    "DynamicsCompressorNode",
    dynamics_compressor_node,
    "DynamicsCompressorNode.rs",
    DynamicsCompressorNode
);

web_feature!(
    "DynamicsCompressorOptions",
    dynamics_compressor_options,
    "DynamicsCompressorOptions.rs",
    DynamicsCompressorOptions
);

web_feature!(
    "EXT_blend_minmax",
    ext_blend_minmax,
    "EXT_blend_minmax.rs",
    EXT_blend_minmax
);

web_feature!(
    "EXT_color_buffer_float",
    ext_color_buffer_float,
    "EXT_color_buffer_float.rs",
    EXT_color_buffer_float
);

web_feature!(
    "EXT_color_buffer_half_float",
    ext_color_buffer_half_float,
    "EXT_color_buffer_half_float.rs",
    EXT_color_buffer_half_float
);

web_feature!(
    "EXT_disjoint_timer_query",
    ext_disjoint_timer_query,
    "EXT_disjoint_timer_query.rs",
    EXT_disjoint_timer_query
);

web_feature!(
    "EXT_disjoint_timer_query_webgl2",
    ext_disjoint_timer_query_webgl2,
    "EXT_disjoint_timer_query_webgl2.rs",
    EXT_disjoint_timer_query_webgl2
);

web_feature!(
    "EXT_float_blend",
    ext_float_blend,
    "EXT_float_blend.rs",
    EXT_float_blend
);

web_feature!(
    "EXT_frag_depth",
    ext_frag_depth,
    "EXT_frag_depth.rs",
    EXT_frag_depth
);

web_feature!("EXT_sRGB", ext_s_rgb, "EXT_sRGB.rs", EXT_sRGB);

web_feature!(
    "EXT_shader_texture_lod",
    ext_shader_texture_lod,
    "EXT_shader_texture_lod.rs",
    EXT_shader_texture_lod
);

web_feature!(
    "EXT_texture_compression_bptc",
    ext_texture_compression_bptc,
    "EXT_texture_compression_bptc.rs",
    EXT_texture_compression_bptc
);

web_feature!(
    "EXT_texture_compression_rgtc",
    ext_texture_compression_rgtc,
    "EXT_texture_compression_rgtc.rs",
    EXT_texture_compression_rgtc
);

web_feature!(
    "EXT_texture_filter_anisotropic",
    ext_texture_filter_anisotropic,
    "EXT_texture_filter_anisotropic.rs",
    EXT_texture_filter_anisotropic
);

web_feature!(
    "EXT_texture_norm16",
    ext_texture_norm16,
    "EXT_texture_norm16.rs",
    EXT_texture_norm16
);

web_feature!(
    "EcKeyAlgorithm",
    ec_key_algorithm,
    "EcKeyAlgorithm.rs",
    EcKeyAlgorithm
);

web_feature!(
    "EcKeyGenParams",
    ec_key_gen_params,
    "EcKeyGenParams.rs",
    EcKeyGenParams
);

web_feature!(
    "EcKeyImportParams",
    ec_key_import_params,
    "EcKeyImportParams.rs",
    EcKeyImportParams
);

web_feature!(
    "EcdhKeyDeriveParams",
    ecdh_key_derive_params,
    "EcdhKeyDeriveParams.rs",
    EcdhKeyDeriveParams
);

web_feature!("EcdsaParams", ecdsa_params, "EcdsaParams.rs", EcdsaParams);

web_feature!("Ed448Params", ed448_params, "Ed448Params.rs", Ed448Params);

web_feature!("EditContext", edit_context, "EditContext.rs", EditContext);

web_feature!(
    "EditContextInit",
    edit_context_init,
    "EditContextInit.rs",
    EditContextInit
);

web_feature!(
    "EffectTiming",
    effect_timing,
    "EffectTiming.rs",
    EffectTiming
);

web_feature!("Element", element, "Element.rs", Element);

web_feature!(
    "ElementCreationOptions",
    element_creation_options,
    "ElementCreationOptions.rs",
    ElementCreationOptions
);

web_feature!(
    "ElementDefinitionOptions",
    element_definition_options,
    "ElementDefinitionOptions.rs",
    ElementDefinitionOptions
);

web_feature!(
    "ElementInternals",
    element_internals,
    "ElementInternals.rs",
    ElementInternals
);

web_feature!(
    "EncodedAudioChunk",
    encoded_audio_chunk,
    "EncodedAudioChunk.rs",
    EncodedAudioChunk
);

web_feature!(
    "EncodedAudioChunkInit",
    encoded_audio_chunk_init,
    "EncodedAudioChunkInit.rs",
    EncodedAudioChunkInit
);

web_feature!(
    "EncodedAudioChunkMetadata",
    encoded_audio_chunk_metadata,
    "EncodedAudioChunkMetadata.rs",
    EncodedAudioChunkMetadata
);

web_feature!(
    "EncodedVideoChunk",
    encoded_video_chunk,
    "EncodedVideoChunk.rs",
    EncodedVideoChunk
);

web_feature!(
    "EncodedVideoChunkInit",
    encoded_video_chunk_init,
    "EncodedVideoChunkInit.rs",
    EncodedVideoChunkInit
);

web_feature!(
    "EncodedVideoChunkMetadata",
    encoded_video_chunk_metadata,
    "EncodedVideoChunkMetadata.rs",
    EncodedVideoChunkMetadata
);

web_feature!("ErrorEvent", error_event, "ErrorEvent.rs", ErrorEvent);

web_feature!(
    "ErrorEventInit",
    error_event_init,
    "ErrorEventInit.rs",
    ErrorEventInit
);

web_feature!("Event", event, "Event.rs", Event);

web_feature!("EventCounts", event_counts, "EventCounts.rs", EventCounts);

web_feature!("EventInit", event_init, "EventInit.rs", EventInit);

web_feature!(
    "EventListenerOptions",
    event_listener_options,
    "EventListenerOptions.rs",
    EventListenerOptions
);

web_feature!(
    "EventModifierInit",
    event_modifier_init,
    "EventModifierInit.rs",
    EventModifierInit
);

web_feature!("EventSource", event_source, "EventSource.rs", EventSource);

web_feature!(
    "EventSourceInit",
    event_source_init,
    "EventSourceInit.rs",
    EventSourceInit
);

web_feature!("EventTarget", event_target, "EventTarget.rs", EventTarget);

web_feature!(
    "ExtendableCookieChangeEvent",
    extendable_cookie_change_event,
    "ExtendableCookieChangeEvent.rs",
    ExtendableCookieChangeEvent
);

web_feature!(
    "ExtendableCookieChangeEventInit",
    extendable_cookie_change_event_init,
    "ExtendableCookieChangeEventInit.rs",
    ExtendableCookieChangeEventInit
);

web_feature!(
    "ExtendableEvent",
    extendable_event,
    "ExtendableEvent.rs",
    ExtendableEvent
);

web_feature!(
    "ExtendableEventInit",
    extendable_event_init,
    "ExtendableEventInit.rs",
    ExtendableEventInit
);

web_feature!(
    "ExtendableMessageEvent",
    extendable_message_event,
    "ExtendableMessageEvent.rs",
    ExtendableMessageEvent
);

web_feature!(
    "ExtendableMessageEventInit",
    extendable_message_event_init,
    "ExtendableMessageEventInit.rs",
    ExtendableMessageEventInit
);

web_feature!("External", external, "External.rs", External);

web_feature!("EyeDropper", eye_dropper, "EyeDropper.rs", EyeDropper);

web_feature!(
    "FaceDetector",
    face_detector,
    "FaceDetector.rs",
    FaceDetector
);

web_feature!(
    "FaceDetectorOptions",
    face_detector_options,
    "FaceDetectorOptions.rs",
    FaceDetectorOptions
);

web_feature!(
    "FederatedCredential",
    federated_credential,
    "FederatedCredential.rs",
    FederatedCredential
);

web_feature!(
    "FederatedCredentialInit",
    federated_credential_init,
    "FederatedCredentialInit.rs",
    FederatedCredentialInit
);

web_feature!(
    "FederatedCredentialRequestOptions",
    federated_credential_request_options,
    "FederatedCredentialRequestOptions.rs",
    FederatedCredentialRequestOptions
);

web_feature!("Fence", fence, "Fence.rs", Fence);

web_feature!("FenceEvent", fence_event, "FenceEvent.rs", FenceEvent);

web_feature!(
    "FencedFrameConfig",
    fenced_frame_config,
    "FencedFrameConfig.rs",
    FencedFrameConfig
);

web_feature!("FetchEvent", fetch_event, "FetchEvent.rs", FetchEvent);

web_feature!(
    "FetchEventInit",
    fetch_event_init,
    "FetchEventInit.rs",
    FetchEventInit
);

web_feature!("File", file, "File.rs", File);

web_feature!("FileList", file_list, "FileList.rs", FileList);

web_feature!(
    "FilePickerAcceptType",
    file_picker_accept_type,
    "FilePickerAcceptType.rs",
    FilePickerAcceptType
);

web_feature!(
    "FilePickerOptions",
    file_picker_options,
    "FilePickerOptions.rs",
    FilePickerOptions
);

web_feature!(
    "FilePropertyBag",
    file_property_bag,
    "FilePropertyBag.rs",
    FilePropertyBag
);

web_feature!("FileReader", file_reader, "FileReader.rs", FileReader);

web_feature!(
    "FileReaderSync",
    file_reader_sync,
    "FileReaderSync.rs",
    FileReaderSync
);

web_feature!("FileSystem", file_system, "FileSystem.rs", FileSystem);

web_feature!(
    "FileSystemCreateWritableOptions",
    file_system_create_writable_options,
    "FileSystemCreateWritableOptions.rs",
    FileSystemCreateWritableOptions
);

web_feature!(
    "FileSystemDirectoryEntry",
    file_system_directory_entry,
    "FileSystemDirectoryEntry.rs",
    FileSystemDirectoryEntry
);

web_feature!(
    "FileSystemDirectoryHandle",
    file_system_directory_handle,
    "FileSystemDirectoryHandle.rs",
    FileSystemDirectoryHandle
);

web_feature!(
    "FileSystemDirectoryReader",
    file_system_directory_reader,
    "FileSystemDirectoryReader.rs",
    FileSystemDirectoryReader
);

web_feature!(
    "FileSystemEntry",
    file_system_entry,
    "FileSystemEntry.rs",
    FileSystemEntry
);

web_feature!(
    "FileSystemFileEntry",
    file_system_file_entry,
    "FileSystemFileEntry.rs",
    FileSystemFileEntry
);

web_feature!(
    "FileSystemFileHandle",
    file_system_file_handle,
    "FileSystemFileHandle.rs",
    FileSystemFileHandle
);

web_feature!(
    "FileSystemFlags",
    file_system_flags,
    "FileSystemFlags.rs",
    FileSystemFlags
);

web_feature!(
    "FileSystemGetDirectoryOptions",
    file_system_get_directory_options,
    "FileSystemGetDirectoryOptions.rs",
    FileSystemGetDirectoryOptions
);

web_feature!(
    "FileSystemGetFileOptions",
    file_system_get_file_options,
    "FileSystemGetFileOptions.rs",
    FileSystemGetFileOptions
);

web_feature!(
    "FileSystemHandle",
    file_system_handle,
    "FileSystemHandle.rs",
    FileSystemHandle
);

web_feature!(
    "FileSystemHandlePermissionDescriptor",
    file_system_handle_permission_descriptor,
    "FileSystemHandlePermissionDescriptor.rs",
    FileSystemHandlePermissionDescriptor
);

web_feature!(
    "FileSystemPermissionDescriptor",
    file_system_permission_descriptor,
    "FileSystemPermissionDescriptor.rs",
    FileSystemPermissionDescriptor
);

web_feature!(
    "FileSystemReadWriteOptions",
    file_system_read_write_options,
    "FileSystemReadWriteOptions.rs",
    FileSystemReadWriteOptions
);

web_feature!(
    "FileSystemRemoveOptions",
    file_system_remove_options,
    "FileSystemRemoveOptions.rs",
    FileSystemRemoveOptions
);

web_feature!(
    "FileSystemSyncAccessHandle",
    file_system_sync_access_handle,
    "FileSystemSyncAccessHandle.rs",
    FileSystemSyncAccessHandle
);

web_feature!(
    "FileSystemWritableFileStream",
    file_system_writable_file_stream,
    "FileSystemWritableFileStream.rs",
    FileSystemWritableFileStream
);

web_feature!(
    "FlacEncoderConfig",
    flac_encoder_config,
    "FlacEncoderConfig.rs",
    FlacEncoderConfig
);

web_feature!("FocusEvent", focus_event, "FocusEvent.rs", FocusEvent);

web_feature!(
    "FocusEventInit",
    focus_event_init,
    "FocusEventInit.rs",
    FocusEventInit
);

web_feature!(
    "FocusOptions",
    focus_options,
    "FocusOptions.rs",
    FocusOptions
);

web_feature!(
    "FocusableAreasOption",
    focusable_areas_option,
    "FocusableAreasOption.rs",
    FocusableAreasOption
);

web_feature!("Font", font, "Font.rs", Font);

web_feature!("FontData", font_data, "FontData.rs", FontData);

web_feature!("FontFace", font_face, "FontFace.rs", FontFace);

web_feature!(
    "FontFaceDescriptors",
    font_face_descriptors,
    "FontFaceDescriptors.rs",
    FontFaceDescriptors
);

web_feature!(
    "FontFaceFeatures",
    font_face_features,
    "FontFaceFeatures.rs",
    FontFaceFeatures
);

web_feature!(
    "FontFacePalette",
    font_face_palette,
    "FontFacePalette.rs",
    FontFacePalette
);

web_feature!(
    "FontFacePalettes",
    font_face_palettes,
    "FontFacePalettes.rs",
    FontFacePalettes
);

web_feature!("FontFaceSet", font_face_set, "FontFaceSet.rs", FontFaceSet);

web_feature!(
    "FontFaceSetLoadEvent",
    font_face_set_load_event,
    "FontFaceSetLoadEvent.rs",
    FontFaceSetLoadEvent
);

web_feature!(
    "FontFaceSetLoadEventInit",
    font_face_set_load_event_init,
    "FontFaceSetLoadEventInit.rs",
    FontFaceSetLoadEventInit
);

web_feature!(
    "FontFaceVariationAxis",
    font_face_variation_axis,
    "FontFaceVariationAxis.rs",
    FontFaceVariationAxis
);

web_feature!(
    "FontFaceVariations",
    font_face_variations,
    "FontFaceVariations.rs",
    FontFaceVariations
);

web_feature!("FontMetrics", font_metrics, "FontMetrics.rs", FontMetrics);

web_feature!(
    "ForDebuggingOnly",
    for_debugging_only,
    "ForDebuggingOnly.rs",
    ForDebuggingOnly
);

web_feature!("FormData", form_data, "FormData.rs", FormData);

web_feature!(
    "FormDataEvent",
    form_data_event,
    "FormDataEvent.rs",
    FormDataEvent
);

web_feature!(
    "FormDataEventInit",
    form_data_event_init,
    "FormDataEventInit.rs",
    FormDataEventInit
);

web_feature!(
    "FragmentDirective",
    fragment_directive,
    "FragmentDirective.rs",
    FragmentDirective
);

web_feature!(
    "FragmentResult",
    fragment_result,
    "FragmentResult.rs",
    FragmentResult
);

web_feature!(
    "FragmentResultOptions",
    fragment_result_options,
    "FragmentResultOptions.rs",
    FragmentResultOptions
);

web_feature!(
    "FullscreenOptions",
    fullscreen_options,
    "FullscreenOptions.rs",
    FullscreenOptions
);

web_feature!(
    "FunctionParameter",
    function_parameter,
    "FunctionParameter.rs",
    FunctionParameter
);

web_feature!("GPU", gpu, "GPU.rs", GPU);

web_feature!("GPUAdapter", gpu_adapter, "GPUAdapter.rs", GPUAdapter);

web_feature!(
    "GPUAdapterInfo",
    gpu_adapter_info,
    "GPUAdapterInfo.rs",
    GPUAdapterInfo
);

web_feature!(
    "GPUBindGroup",
    gpu_bind_group,
    "GPUBindGroup.rs",
    GPUBindGroup
);

web_feature!(
    "GPUBindGroupDescriptor",
    gpu_bind_group_descriptor,
    "GPUBindGroupDescriptor.rs",
    GPUBindGroupDescriptor
);

web_feature!(
    "GPUBindGroupEntry",
    gpu_bind_group_entry,
    "GPUBindGroupEntry.rs",
    GPUBindGroupEntry
);

web_feature!(
    "GPUBindGroupLayout",
    gpu_bind_group_layout,
    "GPUBindGroupLayout.rs",
    GPUBindGroupLayout
);

web_feature!(
    "GPUBindGroupLayoutDescriptor",
    gpu_bind_group_layout_descriptor,
    "GPUBindGroupLayoutDescriptor.rs",
    GPUBindGroupLayoutDescriptor
);

web_feature!(
    "GPUBindGroupLayoutEntry",
    gpu_bind_group_layout_entry,
    "GPUBindGroupLayoutEntry.rs",
    GPUBindGroupLayoutEntry
);

web_feature!(
    "GPUBlendComponent",
    gpu_blend_component,
    "GPUBlendComponent.rs",
    GPUBlendComponent
);

web_feature!(
    "GPUBlendState",
    gpu_blend_state,
    "GPUBlendState.rs",
    GPUBlendState
);

web_feature!("GPUBuffer", gpu_buffer, "GPUBuffer.rs", GPUBuffer);

web_feature!(
    "GPUBufferBinding",
    gpu_buffer_binding,
    "GPUBufferBinding.rs",
    GPUBufferBinding
);

web_feature!(
    "GPUBufferBindingLayout",
    gpu_buffer_binding_layout,
    "GPUBufferBindingLayout.rs",
    GPUBufferBindingLayout
);

web_feature!(
    "GPUBufferDescriptor",
    gpu_buffer_descriptor,
    "GPUBufferDescriptor.rs",
    GPUBufferDescriptor
);

web_feature!(
    "GPUBufferUsage",
    gpu_buffer_usage,
    "GPUBufferUsage.rs",
    GPUBufferUsage
);

web_feature!(
    "GPUCanvasConfiguration",
    gpu_canvas_configuration,
    "GPUCanvasConfiguration.rs",
    GPUCanvasConfiguration
);

web_feature!(
    "GPUCanvasContext",
    gpu_canvas_context,
    "GPUCanvasContext.rs",
    GPUCanvasContext
);

web_feature!(
    "GPUCanvasToneMapping",
    gpu_canvas_tone_mapping,
    "GPUCanvasToneMapping.rs",
    GPUCanvasToneMapping
);

web_feature!(
    "GPUColorDict",
    gpu_color_dict,
    "GPUColorDict.rs",
    GPUColorDict
);

web_feature!(
    "GPUColorTargetState",
    gpu_color_target_state,
    "GPUColorTargetState.rs",
    GPUColorTargetState
);

web_feature!(
    "GPUColorWrite",
    gpu_color_write,
    "GPUColorWrite.rs",
    GPUColorWrite
);

web_feature!(
    "GPUCommandBuffer",
    gpu_command_buffer,
    "GPUCommandBuffer.rs",
    GPUCommandBuffer
);

web_feature!(
    "GPUCommandBufferDescriptor",
    gpu_command_buffer_descriptor,
    "GPUCommandBufferDescriptor.rs",
    GPUCommandBufferDescriptor
);

web_feature!(
    "GPUCommandEncoder",
    gpu_command_encoder,
    "GPUCommandEncoder.rs",
    GPUCommandEncoder
);

web_feature!(
    "GPUCommandEncoderDescriptor",
    gpu_command_encoder_descriptor,
    "GPUCommandEncoderDescriptor.rs",
    GPUCommandEncoderDescriptor
);

web_feature!(
    "GPUCompilationInfo",
    gpu_compilation_info,
    "GPUCompilationInfo.rs",
    GPUCompilationInfo
);

web_feature!(
    "GPUCompilationMessage",
    gpu_compilation_message,
    "GPUCompilationMessage.rs",
    GPUCompilationMessage
);

web_feature!(
    "GPUComputePassDescriptor",
    gpu_compute_pass_descriptor,
    "GPUComputePassDescriptor.rs",
    GPUComputePassDescriptor
);

web_feature!(
    "GPUComputePassEncoder",
    gpu_compute_pass_encoder,
    "GPUComputePassEncoder.rs",
    GPUComputePassEncoder
);

web_feature!(
    "GPUComputePassTimestampWrites",
    gpu_compute_pass_timestamp_writes,
    "GPUComputePassTimestampWrites.rs",
    GPUComputePassTimestampWrites
);

web_feature!(
    "GPUComputePipeline",
    gpu_compute_pipeline,
    "GPUComputePipeline.rs",
    GPUComputePipeline
);

web_feature!(
    "GPUComputePipelineDescriptor",
    gpu_compute_pipeline_descriptor,
    "GPUComputePipelineDescriptor.rs",
    GPUComputePipelineDescriptor
);

web_feature!(
    "GPUCopyExternalImageDestInfo",
    gpu_copy_external_image_dest_info,
    "GPUCopyExternalImageDestInfo.rs",
    GPUCopyExternalImageDestInfo
);

web_feature!(
    "GPUCopyExternalImageSourceInfo",
    gpu_copy_external_image_source_info,
    "GPUCopyExternalImageSourceInfo.rs",
    GPUCopyExternalImageSourceInfo
);

web_feature!(
    "GPUDepthStencilState",
    gpu_depth_stencil_state,
    "GPUDepthStencilState.rs",
    GPUDepthStencilState
);

web_feature!("GPUDevice", gpu_device, "GPUDevice.rs", GPUDevice);

web_feature!(
    "GPUDeviceDescriptor",
    gpu_device_descriptor,
    "GPUDeviceDescriptor.rs",
    GPUDeviceDescriptor
);

web_feature!(
    "GPUDeviceLostInfo",
    gpu_device_lost_info,
    "GPUDeviceLostInfo.rs",
    GPUDeviceLostInfo
);

web_feature!("GPUError", gpu_error, "GPUError.rs", GPUError);

web_feature!(
    "GPUExtent3DDict",
    gpu_extent3d_dict,
    "GPUExtent3DDict.rs",
    GPUExtent3DDict
);

web_feature!(
    "GPUExternalTexture",
    gpu_external_texture,
    "GPUExternalTexture.rs",
    GPUExternalTexture
);

web_feature!(
    "GPUExternalTextureBindingLayout",
    gpu_external_texture_binding_layout,
    "GPUExternalTextureBindingLayout.rs",
    GPUExternalTextureBindingLayout
);

web_feature!(
    "GPUExternalTextureDescriptor",
    gpu_external_texture_descriptor,
    "GPUExternalTextureDescriptor.rs",
    GPUExternalTextureDescriptor
);

web_feature!(
    "GPUFragmentState",
    gpu_fragment_state,
    "GPUFragmentState.rs",
    GPUFragmentState
);

web_feature!(
    "GPUInternalError",
    gpu_internal_error,
    "GPUInternalError.rs",
    GPUInternalError
);

web_feature!("GPUMapMode", gpu_map_mode, "GPUMapMode.rs", GPUMapMode);

web_feature!(
    "GPUMultisampleState",
    gpu_multisample_state,
    "GPUMultisampleState.rs",
    GPUMultisampleState
);

web_feature!(
    "GPUObjectDescriptorBase",
    gpu_object_descriptor_base,
    "GPUObjectDescriptorBase.rs",
    GPUObjectDescriptorBase
);

web_feature!(
    "GPUOrigin2DDict",
    gpu_origin2d_dict,
    "GPUOrigin2DDict.rs",
    GPUOrigin2DDict
);

web_feature!(
    "GPUOrigin3DDict",
    gpu_origin3d_dict,
    "GPUOrigin3DDict.rs",
    GPUOrigin3DDict
);

web_feature!(
    "GPUOutOfMemoryError",
    gpu_out_of_memory_error,
    "GPUOutOfMemoryError.rs",
    GPUOutOfMemoryError
);

web_feature!(
    "GPUPipelineDescriptorBase",
    gpu_pipeline_descriptor_base,
    "GPUPipelineDescriptorBase.rs",
    GPUPipelineDescriptorBase
);

web_feature!(
    "GPUPipelineError",
    gpu_pipeline_error,
    "GPUPipelineError.rs",
    GPUPipelineError
);

web_feature!(
    "GPUPipelineErrorInit",
    gpu_pipeline_error_init,
    "GPUPipelineErrorInit.rs",
    GPUPipelineErrorInit
);

web_feature!(
    "GPUPipelineLayout",
    gpu_pipeline_layout,
    "GPUPipelineLayout.rs",
    GPUPipelineLayout
);

web_feature!(
    "GPUPipelineLayoutDescriptor",
    gpu_pipeline_layout_descriptor,
    "GPUPipelineLayoutDescriptor.rs",
    GPUPipelineLayoutDescriptor
);

web_feature!(
    "GPUPrimitiveState",
    gpu_primitive_state,
    "GPUPrimitiveState.rs",
    GPUPrimitiveState
);

web_feature!(
    "GPUProgrammableStage",
    gpu_programmable_stage,
    "GPUProgrammableStage.rs",
    GPUProgrammableStage
);

web_feature!("GPUQuerySet", gpu_query_set, "GPUQuerySet.rs", GPUQuerySet);

web_feature!(
    "GPUQuerySetDescriptor",
    gpu_query_set_descriptor,
    "GPUQuerySetDescriptor.rs",
    GPUQuerySetDescriptor
);

web_feature!("GPUQueue", gpu_queue, "GPUQueue.rs", GPUQueue);

web_feature!(
    "GPUQueueDescriptor",
    gpu_queue_descriptor,
    "GPUQueueDescriptor.rs",
    GPUQueueDescriptor
);

web_feature!(
    "GPURenderBundle",
    gpu_render_bundle,
    "GPURenderBundle.rs",
    GPURenderBundle
);

web_feature!(
    "GPURenderBundleDescriptor",
    gpu_render_bundle_descriptor,
    "GPURenderBundleDescriptor.rs",
    GPURenderBundleDescriptor
);

web_feature!(
    "GPURenderBundleEncoder",
    gpu_render_bundle_encoder,
    "GPURenderBundleEncoder.rs",
    GPURenderBundleEncoder
);

web_feature!(
    "GPURenderBundleEncoderDescriptor",
    gpu_render_bundle_encoder_descriptor,
    "GPURenderBundleEncoderDescriptor.rs",
    GPURenderBundleEncoderDescriptor
);

web_feature!(
    "GPURenderPassColorAttachment",
    gpu_render_pass_color_attachment,
    "GPURenderPassColorAttachment.rs",
    GPURenderPassColorAttachment
);

web_feature!(
    "GPURenderPassDepthStencilAttachment",
    gpu_render_pass_depth_stencil_attachment,
    "GPURenderPassDepthStencilAttachment.rs",
    GPURenderPassDepthStencilAttachment
);

web_feature!(
    "GPURenderPassDescriptor",
    gpu_render_pass_descriptor,
    "GPURenderPassDescriptor.rs",
    GPURenderPassDescriptor
);

web_feature!(
    "GPURenderPassEncoder",
    gpu_render_pass_encoder,
    "GPURenderPassEncoder.rs",
    GPURenderPassEncoder
);

web_feature!(
    "GPURenderPassLayout",
    gpu_render_pass_layout,
    "GPURenderPassLayout.rs",
    GPURenderPassLayout
);

web_feature!(
    "GPURenderPassTimestampWrites",
    gpu_render_pass_timestamp_writes,
    "GPURenderPassTimestampWrites.rs",
    GPURenderPassTimestampWrites
);

web_feature!(
    "GPURenderPipeline",
    gpu_render_pipeline,
    "GPURenderPipeline.rs",
    GPURenderPipeline
);

web_feature!(
    "GPURenderPipelineDescriptor",
    gpu_render_pipeline_descriptor,
    "GPURenderPipelineDescriptor.rs",
    GPURenderPipelineDescriptor
);

web_feature!(
    "GPURequestAdapterOptions",
    gpu_request_adapter_options,
    "GPURequestAdapterOptions.rs",
    GPURequestAdapterOptions
);

web_feature!("GPUSampler", gpu_sampler, "GPUSampler.rs", GPUSampler);

web_feature!(
    "GPUSamplerBindingLayout",
    gpu_sampler_binding_layout,
    "GPUSamplerBindingLayout.rs",
    GPUSamplerBindingLayout
);

web_feature!(
    "GPUSamplerDescriptor",
    gpu_sampler_descriptor,
    "GPUSamplerDescriptor.rs",
    GPUSamplerDescriptor
);

web_feature!(
    "GPUShaderModule",
    gpu_shader_module,
    "GPUShaderModule.rs",
    GPUShaderModule
);

web_feature!(
    "GPUShaderModuleCompilationHint",
    gpu_shader_module_compilation_hint,
    "GPUShaderModuleCompilationHint.rs",
    GPUShaderModuleCompilationHint
);

web_feature!(
    "GPUShaderModuleDescriptor",
    gpu_shader_module_descriptor,
    "GPUShaderModuleDescriptor.rs",
    GPUShaderModuleDescriptor
);

web_feature!(
    "GPUShaderStage",
    gpu_shader_stage,
    "GPUShaderStage.rs",
    GPUShaderStage
);

web_feature!(
    "GPUStencilFaceState",
    gpu_stencil_face_state,
    "GPUStencilFaceState.rs",
    GPUStencilFaceState
);

web_feature!(
    "GPUStorageTextureBindingLayout",
    gpu_storage_texture_binding_layout,
    "GPUStorageTextureBindingLayout.rs",
    GPUStorageTextureBindingLayout
);

web_feature!(
    "GPUSupportedFeatures",
    gpu_supported_features,
    "GPUSupportedFeatures.rs",
    GPUSupportedFeatures
);

web_feature!(
    "GPUSupportedLimits",
    gpu_supported_limits,
    "GPUSupportedLimits.rs",
    GPUSupportedLimits
);

web_feature!(
    "GPUTexelCopyBufferInfo",
    gpu_texel_copy_buffer_info,
    "GPUTexelCopyBufferInfo.rs",
    GPUTexelCopyBufferInfo
);

web_feature!(
    "GPUTexelCopyBufferLayout",
    gpu_texel_copy_buffer_layout,
    "GPUTexelCopyBufferLayout.rs",
    GPUTexelCopyBufferLayout
);

web_feature!(
    "GPUTexelCopyTextureInfo",
    gpu_texel_copy_texture_info,
    "GPUTexelCopyTextureInfo.rs",
    GPUTexelCopyTextureInfo
);

web_feature!("GPUTexture", gpu_texture, "GPUTexture.rs", GPUTexture);

web_feature!(
    "GPUTextureBindingLayout",
    gpu_texture_binding_layout,
    "GPUTextureBindingLayout.rs",
    GPUTextureBindingLayout
);

web_feature!(
    "GPUTextureDescriptor",
    gpu_texture_descriptor,
    "GPUTextureDescriptor.rs",
    GPUTextureDescriptor
);

web_feature!(
    "GPUTextureUsage",
    gpu_texture_usage,
    "GPUTextureUsage.rs",
    GPUTextureUsage
);

web_feature!(
    "GPUTextureView",
    gpu_texture_view,
    "GPUTextureView.rs",
    GPUTextureView
);

web_feature!(
    "GPUTextureViewDescriptor",
    gpu_texture_view_descriptor,
    "GPUTextureViewDescriptor.rs",
    GPUTextureViewDescriptor
);

web_feature!(
    "GPUUncapturedErrorEvent",
    gpu_uncaptured_error_event,
    "GPUUncapturedErrorEvent.rs",
    GPUUncapturedErrorEvent
);

web_feature!(
    "GPUUncapturedErrorEventInit",
    gpu_uncaptured_error_event_init,
    "GPUUncapturedErrorEventInit.rs",
    GPUUncapturedErrorEventInit
);

web_feature!(
    "GPUValidationError",
    gpu_validation_error,
    "GPUValidationError.rs",
    GPUValidationError
);

web_feature!(
    "GPUVertexAttribute",
    gpu_vertex_attribute,
    "GPUVertexAttribute.rs",
    GPUVertexAttribute
);

web_feature!(
    "GPUVertexBufferLayout",
    gpu_vertex_buffer_layout,
    "GPUVertexBufferLayout.rs",
    GPUVertexBufferLayout
);

web_feature!(
    "GPUVertexState",
    gpu_vertex_state,
    "GPUVertexState.rs",
    GPUVertexState
);

web_feature!("GainNode", gain_node, "GainNode.rs", GainNode);

web_feature!("GainOptions", gain_options, "GainOptions.rs", GainOptions);

web_feature!("Gamepad", gamepad, "Gamepad.rs", Gamepad);

web_feature!(
    "GamepadButton",
    gamepad_button,
    "GamepadButton.rs",
    GamepadButton
);

web_feature!(
    "GamepadEffectParameters",
    gamepad_effect_parameters,
    "GamepadEffectParameters.rs",
    GamepadEffectParameters
);

web_feature!(
    "GamepadEvent",
    gamepad_event,
    "GamepadEvent.rs",
    GamepadEvent
);

web_feature!(
    "GamepadEventInit",
    gamepad_event_init,
    "GamepadEventInit.rs",
    GamepadEventInit
);

web_feature!(
    "GamepadHapticActuator",
    gamepad_haptic_actuator,
    "GamepadHapticActuator.rs",
    GamepadHapticActuator
);

web_feature!("GamepadPose", gamepad_pose, "GamepadPose.rs", GamepadPose);

web_feature!(
    "GamepadTouch",
    gamepad_touch,
    "GamepadTouch.rs",
    GamepadTouch
);

web_feature!(
    "GenerateBidInterestGroup",
    generate_bid_interest_group,
    "GenerateBidInterestGroup.rs",
    GenerateBidInterestGroup
);

web_feature!(
    "GenerateBidOutput",
    generate_bid_output,
    "GenerateBidOutput.rs",
    GenerateBidOutput
);

web_feature!(
    "GenerateTestReportParameters",
    generate_test_report_parameters,
    "GenerateTestReportParameters.rs",
    GenerateTestReportParameters
);

web_feature!("Geolocation", geolocation, "Geolocation.rs", Geolocation);

web_feature!(
    "GeolocationCoordinates",
    geolocation_coordinates,
    "GeolocationCoordinates.rs",
    GeolocationCoordinates
);

web_feature!(
    "GeolocationPosition",
    geolocation_position,
    "GeolocationPosition.rs",
    GeolocationPosition
);

web_feature!(
    "GeolocationPositionError",
    geolocation_position_error,
    "GeolocationPositionError.rs",
    GeolocationPositionError
);

web_feature!(
    "GeolocationSensor",
    geolocation_sensor,
    "GeolocationSensor.rs",
    GeolocationSensor
);

web_feature!(
    "GeolocationSensorOptions",
    geolocation_sensor_options,
    "GeolocationSensorOptions.rs",
    GeolocationSensorOptions
);

web_feature!(
    "GeolocationSensorReading",
    geolocation_sensor_reading,
    "GeolocationSensorReading.rs",
    GeolocationSensorReading
);

web_feature!(
    "GetAnimationsOptions",
    get_animations_options,
    "GetAnimationsOptions.rs",
    GetAnimationsOptions
);

web_feature!(
    "GetComposedRangesOptions",
    get_composed_ranges_options,
    "GetComposedRangesOptions.rs",
    GetComposedRangesOptions
);

web_feature!(
    "GetHTMLOptions",
    get_html_options,
    "GetHTMLOptions.rs",
    GetHTMLOptions
);

web_feature!(
    "GetNotificationOptions",
    get_notification_options,
    "GetNotificationOptions.rs",
    GetNotificationOptions
);

web_feature!(
    "GetRootNodeOptions",
    get_root_node_options,
    "GetRootNodeOptions.rs",
    GetRootNodeOptions
);

web_feature!("Global", global, "Global.rs", Global);

web_feature!(
    "GlobalDescriptor",
    global_descriptor,
    "GlobalDescriptor.rs",
    GlobalDescriptor
);

web_feature!(
    "GravitySensor",
    gravity_sensor,
    "GravitySensor.rs",
    GravitySensor
);

web_feature!("GroupEffect", group_effect, "GroupEffect.rs", GroupEffect);

web_feature!("Gyroscope", gyroscope, "Gyroscope.rs", Gyroscope);

web_feature!(
    "GyroscopeSensorOptions",
    gyroscope_sensor_options,
    "GyroscopeSensorOptions.rs",
    GyroscopeSensorOptions
);

web_feature!("HID", hid, "HID.rs", HID);

web_feature!(
    "HIDCollectionInfo",
    hid_collection_info,
    "HIDCollectionInfo.rs",
    HIDCollectionInfo
);

web_feature!(
    "HIDConnectionEvent",
    hid_connection_event,
    "HIDConnectionEvent.rs",
    HIDConnectionEvent
);

web_feature!(
    "HIDConnectionEventInit",
    hid_connection_event_init,
    "HIDConnectionEventInit.rs",
    HIDConnectionEventInit
);

web_feature!("HIDDevice", hid_device, "HIDDevice.rs", HIDDevice);

web_feature!(
    "HIDDeviceFilter",
    hid_device_filter,
    "HIDDeviceFilter.rs",
    HIDDeviceFilter
);

web_feature!(
    "HIDDeviceRequestOptions",
    hid_device_request_options,
    "HIDDeviceRequestOptions.rs",
    HIDDeviceRequestOptions
);

web_feature!(
    "HIDInputReportEvent",
    hid_input_report_event,
    "HIDInputReportEvent.rs",
    HIDInputReportEvent
);

web_feature!(
    "HIDInputReportEventInit",
    hid_input_report_event_init,
    "HIDInputReportEventInit.rs",
    HIDInputReportEventInit
);

web_feature!(
    "HIDReportInfo",
    hid_report_info,
    "HIDReportInfo.rs",
    HIDReportInfo
);

web_feature!(
    "HIDReportItem",
    hid_report_item,
    "HIDReportItem.rs",
    HIDReportItem
);

web_feature!(
    "HMACGetSecretInput",
    hmac_get_secret_input,
    "HMACGetSecretInput.rs",
    HMACGetSecretInput
);

web_feature!(
    "HMACGetSecretOutput",
    hmac_get_secret_output,
    "HMACGetSecretOutput.rs",
    HMACGetSecretOutput
);

web_feature!(
    "HTMLAllCollection",
    html_all_collection,
    "HTMLAllCollection.rs",
    HTMLAllCollection
);

web_feature!(
    "HTMLAnchorElement",
    html_anchor_element,
    "HTMLAnchorElement.rs",
    HTMLAnchorElement
);

web_feature!(
    "HTMLAreaElement",
    html_area_element,
    "HTMLAreaElement.rs",
    HTMLAreaElement
);

web_feature!(
    "HTMLAudioElement",
    html_audio_element,
    "HTMLAudioElement.rs",
    HTMLAudioElement
);

web_feature!(
    "HTMLBRElement",
    htmlbr_element,
    "HTMLBRElement.rs",
    HTMLBRElement
);

web_feature!(
    "HTMLBaseElement",
    html_base_element,
    "HTMLBaseElement.rs",
    HTMLBaseElement
);

web_feature!(
    "HTMLBodyElement",
    html_body_element,
    "HTMLBodyElement.rs",
    HTMLBodyElement
);

web_feature!(
    "HTMLButtonElement",
    html_button_element,
    "HTMLButtonElement.rs",
    HTMLButtonElement
);

web_feature!(
    "HTMLCanvasElement",
    html_canvas_element,
    "HTMLCanvasElement.rs",
    HTMLCanvasElement
);

web_feature!(
    "HTMLCollection",
    html_collection,
    "HTMLCollection.rs",
    HTMLCollection
);

web_feature!(
    "HTMLDListElement",
    htmld_list_element,
    "HTMLDListElement.rs",
    HTMLDListElement
);

web_feature!(
    "HTMLDataElement",
    html_data_element,
    "HTMLDataElement.rs",
    HTMLDataElement
);

web_feature!(
    "HTMLDataListElement",
    html_data_list_element,
    "HTMLDataListElement.rs",
    HTMLDataListElement
);

web_feature!(
    "HTMLDetailsElement",
    html_details_element,
    "HTMLDetailsElement.rs",
    HTMLDetailsElement
);

web_feature!(
    "HTMLDialogElement",
    html_dialog_element,
    "HTMLDialogElement.rs",
    HTMLDialogElement
);

web_feature!(
    "HTMLDirectoryElement",
    html_directory_element,
    "HTMLDirectoryElement.rs",
    HTMLDirectoryElement
);

web_feature!(
    "HTMLDivElement",
    html_div_element,
    "HTMLDivElement.rs",
    HTMLDivElement
);

web_feature!("HTMLElement", html_element, "HTMLElement.rs", HTMLElement);

web_feature!(
    "HTMLEmbedElement",
    html_embed_element,
    "HTMLEmbedElement.rs",
    HTMLEmbedElement
);

web_feature!(
    "HTMLFencedFrameElement",
    html_fenced_frame_element,
    "HTMLFencedFrameElement.rs",
    HTMLFencedFrameElement
);

web_feature!(
    "HTMLFieldSetElement",
    html_field_set_element,
    "HTMLFieldSetElement.rs",
    HTMLFieldSetElement
);

web_feature!(
    "HTMLFontElement",
    html_font_element,
    "HTMLFontElement.rs",
    HTMLFontElement
);

web_feature!(
    "HTMLFormControlsCollection",
    html_form_controls_collection,
    "HTMLFormControlsCollection.rs",
    HTMLFormControlsCollection
);

web_feature!(
    "HTMLFormElement",
    html_form_element,
    "HTMLFormElement.rs",
    HTMLFormElement
);

web_feature!(
    "HTMLFrameElement",
    html_frame_element,
    "HTMLFrameElement.rs",
    HTMLFrameElement
);

web_feature!(
    "HTMLFrameSetElement",
    html_frame_set_element,
    "HTMLFrameSetElement.rs",
    HTMLFrameSetElement
);

web_feature!(
    "HTMLHRElement",
    htmlhr_element,
    "HTMLHRElement.rs",
    HTMLHRElement
);

web_feature!(
    "HTMLHeadElement",
    html_head_element,
    "HTMLHeadElement.rs",
    HTMLHeadElement
);

web_feature!(
    "HTMLHeadingElement",
    html_heading_element,
    "HTMLHeadingElement.rs",
    HTMLHeadingElement
);

web_feature!(
    "HTMLHtmlElement",
    html_html_element,
    "HTMLHtmlElement.rs",
    HTMLHtmlElement
);

web_feature!(
    "HTMLIFrameElement",
    htmli_frame_element,
    "HTMLIFrameElement.rs",
    HTMLIFrameElement
);

web_feature!(
    "HTMLImageElement",
    html_image_element,
    "HTMLImageElement.rs",
    HTMLImageElement
);

web_feature!(
    "HTMLInputElement",
    html_input_element,
    "HTMLInputElement.rs",
    HTMLInputElement
);

web_feature!(
    "HTMLLIElement",
    htmlli_element,
    "HTMLLIElement.rs",
    HTMLLIElement
);

web_feature!(
    "HTMLLabelElement",
    html_label_element,
    "HTMLLabelElement.rs",
    HTMLLabelElement
);

web_feature!(
    "HTMLLegendElement",
    html_legend_element,
    "HTMLLegendElement.rs",
    HTMLLegendElement
);

web_feature!(
    "HTMLLinkElement",
    html_link_element,
    "HTMLLinkElement.rs",
    HTMLLinkElement
);

web_feature!(
    "HTMLMapElement",
    html_map_element,
    "HTMLMapElement.rs",
    HTMLMapElement
);

web_feature!(
    "HTMLMarqueeElement",
    html_marquee_element,
    "HTMLMarqueeElement.rs",
    HTMLMarqueeElement
);

web_feature!(
    "HTMLMediaElement",
    html_media_element,
    "HTMLMediaElement.rs",
    HTMLMediaElement
);

web_feature!(
    "HTMLMenuElement",
    html_menu_element,
    "HTMLMenuElement.rs",
    HTMLMenuElement
);

web_feature!(
    "HTMLMetaElement",
    html_meta_element,
    "HTMLMetaElement.rs",
    HTMLMetaElement
);

web_feature!(
    "HTMLMeterElement",
    html_meter_element,
    "HTMLMeterElement.rs",
    HTMLMeterElement
);

web_feature!(
    "HTMLModElement",
    html_mod_element,
    "HTMLModElement.rs",
    HTMLModElement
);

web_feature!(
    "HTMLModelElement",
    html_model_element,
    "HTMLModelElement.rs",
    HTMLModelElement
);

web_feature!(
    "HTMLOListElement",
    htmlo_list_element,
    "HTMLOListElement.rs",
    HTMLOListElement
);

web_feature!(
    "HTMLObjectElement",
    html_object_element,
    "HTMLObjectElement.rs",
    HTMLObjectElement
);

web_feature!(
    "HTMLOptGroupElement",
    html_opt_group_element,
    "HTMLOptGroupElement.rs",
    HTMLOptGroupElement
);

web_feature!(
    "HTMLOptionElement",
    html_option_element,
    "HTMLOptionElement.rs",
    HTMLOptionElement
);

web_feature!(
    "HTMLOptionsCollection",
    html_options_collection,
    "HTMLOptionsCollection.rs",
    HTMLOptionsCollection
);

web_feature!(
    "HTMLOutputElement",
    html_output_element,
    "HTMLOutputElement.rs",
    HTMLOutputElement
);

web_feature!(
    "HTMLParagraphElement",
    html_paragraph_element,
    "HTMLParagraphElement.rs",
    HTMLParagraphElement
);

web_feature!(
    "HTMLParamElement",
    html_param_element,
    "HTMLParamElement.rs",
    HTMLParamElement
);

web_feature!(
    "HTMLPictureElement",
    html_picture_element,
    "HTMLPictureElement.rs",
    HTMLPictureElement
);

web_feature!(
    "HTMLPortalElement",
    html_portal_element,
    "HTMLPortalElement.rs",
    HTMLPortalElement
);

web_feature!(
    "HTMLPreElement",
    html_pre_element,
    "HTMLPreElement.rs",
    HTMLPreElement
);

web_feature!(
    "HTMLProgressElement",
    html_progress_element,
    "HTMLProgressElement.rs",
    HTMLProgressElement
);

web_feature!(
    "HTMLQuoteElement",
    html_quote_element,
    "HTMLQuoteElement.rs",
    HTMLQuoteElement
);

web_feature!(
    "HTMLScriptElement",
    html_script_element,
    "HTMLScriptElement.rs",
    HTMLScriptElement
);

web_feature!(
    "HTMLSelectElement",
    html_select_element,
    "HTMLSelectElement.rs",
    HTMLSelectElement
);

web_feature!(
    "HTMLSlotElement",
    html_slot_element,
    "HTMLSlotElement.rs",
    HTMLSlotElement
);

web_feature!(
    "HTMLSourceElement",
    html_source_element,
    "HTMLSourceElement.rs",
    HTMLSourceElement
);

web_feature!(
    "HTMLSpanElement",
    html_span_element,
    "HTMLSpanElement.rs",
    HTMLSpanElement
);

web_feature!(
    "HTMLStyleElement",
    html_style_element,
    "HTMLStyleElement.rs",
    HTMLStyleElement
);

web_feature!(
    "HTMLTableCaptionElement",
    html_table_caption_element,
    "HTMLTableCaptionElement.rs",
    HTMLTableCaptionElement
);

web_feature!(
    "HTMLTableCellElement",
    html_table_cell_element,
    "HTMLTableCellElement.rs",
    HTMLTableCellElement
);

web_feature!(
    "HTMLTableColElement",
    html_table_col_element,
    "HTMLTableColElement.rs",
    HTMLTableColElement
);

web_feature!(
    "HTMLTableElement",
    html_table_element,
    "HTMLTableElement.rs",
    HTMLTableElement
);

web_feature!(
    "HTMLTableRowElement",
    html_table_row_element,
    "HTMLTableRowElement.rs",
    HTMLTableRowElement
);

web_feature!(
    "HTMLTableSectionElement",
    html_table_section_element,
    "HTMLTableSectionElement.rs",
    HTMLTableSectionElement
);

web_feature!(
    "HTMLTemplateElement",
    html_template_element,
    "HTMLTemplateElement.rs",
    HTMLTemplateElement
);

web_feature!(
    "HTMLTextAreaElement",
    html_text_area_element,
    "HTMLTextAreaElement.rs",
    HTMLTextAreaElement
);

web_feature!(
    "HTMLTimeElement",
    html_time_element,
    "HTMLTimeElement.rs",
    HTMLTimeElement
);

web_feature!(
    "HTMLTitleElement",
    html_title_element,
    "HTMLTitleElement.rs",
    HTMLTitleElement
);

web_feature!(
    "HTMLTrackElement",
    html_track_element,
    "HTMLTrackElement.rs",
    HTMLTrackElement
);

web_feature!(
    "HTMLUListElement",
    htmlu_list_element,
    "HTMLUListElement.rs",
    HTMLUListElement
);

web_feature!(
    "HTMLUnknownElement",
    html_unknown_element,
    "HTMLUnknownElement.rs",
    HTMLUnknownElement
);

web_feature!(
    "HTMLVideoElement",
    html_video_element,
    "HTMLVideoElement.rs",
    HTMLVideoElement
);

web_feature!(
    "HandwritingDrawing",
    handwriting_drawing,
    "HandwritingDrawing.rs",
    HandwritingDrawing
);

web_feature!(
    "HandwritingDrawingSegment",
    handwriting_drawing_segment,
    "HandwritingDrawingSegment.rs",
    HandwritingDrawingSegment
);

web_feature!(
    "HandwritingHints",
    handwriting_hints,
    "HandwritingHints.rs",
    HandwritingHints
);

web_feature!(
    "HandwritingHintsQueryResult",
    handwriting_hints_query_result,
    "HandwritingHintsQueryResult.rs",
    HandwritingHintsQueryResult
);

web_feature!(
    "HandwritingModelConstraint",
    handwriting_model_constraint,
    "HandwritingModelConstraint.rs",
    HandwritingModelConstraint
);

web_feature!(
    "HandwritingPoint",
    handwriting_point,
    "HandwritingPoint.rs",
    HandwritingPoint
);

web_feature!(
    "HandwritingPrediction",
    handwriting_prediction,
    "HandwritingPrediction.rs",
    HandwritingPrediction
);

web_feature!(
    "HandwritingRecognizer",
    handwriting_recognizer,
    "HandwritingRecognizer.rs",
    HandwritingRecognizer
);

web_feature!(
    "HandwritingRecognizerQueryResult",
    handwriting_recognizer_query_result,
    "HandwritingRecognizerQueryResult.rs",
    HandwritingRecognizerQueryResult
);

web_feature!(
    "HandwritingSegment",
    handwriting_segment,
    "HandwritingSegment.rs",
    HandwritingSegment
);

web_feature!(
    "HandwritingStroke",
    handwriting_stroke,
    "HandwritingStroke.rs",
    HandwritingStroke
);

web_feature!(
    "HashChangeEvent",
    hash_change_event,
    "HashChangeEvent.rs",
    HashChangeEvent
);

web_feature!(
    "HashChangeEventInit",
    hash_change_event_init,
    "HashChangeEventInit.rs",
    HashChangeEventInit
);

web_feature!("Headers", headers, "Headers.rs", Headers);

web_feature!(
    "HevcEncoderConfig",
    hevc_encoder_config,
    "HevcEncoderConfig.rs",
    HevcEncoderConfig
);

web_feature!("Highlight", highlight, "Highlight.rs", Highlight);

web_feature!(
    "HighlightHitResult",
    highlight_hit_result,
    "HighlightHitResult.rs",
    HighlightHitResult
);

web_feature!(
    "HighlightRegistry",
    highlight_registry,
    "HighlightRegistry.rs",
    HighlightRegistry
);

web_feature!(
    "HighlightsFromPointOptions",
    highlights_from_point_options,
    "HighlightsFromPointOptions.rs",
    HighlightsFromPointOptions
);

web_feature!("History", history, "History.rs", History);

web_feature!("HkdfParams", hkdf_params, "HkdfParams.rs", HkdfParams);

web_feature!(
    "HmacImportParams",
    hmac_import_params,
    "HmacImportParams.rs",
    HmacImportParams
);

web_feature!(
    "HmacKeyAlgorithm",
    hmac_key_algorithm,
    "HmacKeyAlgorithm.rs",
    HmacKeyAlgorithm
);

web_feature!(
    "HmacKeyGenParams",
    hmac_key_gen_params,
    "HmacKeyGenParams.rs",
    HmacKeyGenParams
);

web_feature!("IDBCursor", idb_cursor, "IDBCursor.rs", IDBCursor);

web_feature!(
    "IDBCursorWithValue",
    idb_cursor_with_value,
    "IDBCursorWithValue.rs",
    IDBCursorWithValue
);

web_feature!("IDBDatabase", idb_database, "IDBDatabase.rs", IDBDatabase);

web_feature!(
    "IDBDatabaseInfo",
    idb_database_info,
    "IDBDatabaseInfo.rs",
    IDBDatabaseInfo
);

web_feature!("IDBFactory", idb_factory, "IDBFactory.rs", IDBFactory);

web_feature!("IDBIndex", idb_index, "IDBIndex.rs", IDBIndex);

web_feature!(
    "IDBIndexParameters",
    idb_index_parameters,
    "IDBIndexParameters.rs",
    IDBIndexParameters
);

web_feature!("IDBKeyRange", idb_key_range, "IDBKeyRange.rs", IDBKeyRange);

web_feature!(
    "IDBObjectStore",
    idb_object_store,
    "IDBObjectStore.rs",
    IDBObjectStore
);

web_feature!(
    "IDBObjectStoreParameters",
    idb_object_store_parameters,
    "IDBObjectStoreParameters.rs",
    IDBObjectStoreParameters
);

web_feature!(
    "IDBOpenDBRequest",
    idb_open_db_request,
    "IDBOpenDBRequest.rs",
    IDBOpenDBRequest
);

web_feature!("IDBRequest", idb_request, "IDBRequest.rs", IDBRequest);

web_feature!(
    "IDBTransaction",
    idb_transaction,
    "IDBTransaction.rs",
    IDBTransaction
);

web_feature!(
    "IDBTransactionOptions",
    idb_transaction_options,
    "IDBTransactionOptions.rs",
    IDBTransactionOptions
);

web_feature!(
    "IDBVersionChangeEvent",
    idb_version_change_event,
    "IDBVersionChangeEvent.rs",
    IDBVersionChangeEvent
);

web_feature!(
    "IDBVersionChangeEventInit",
    idb_version_change_event_init,
    "IDBVersionChangeEventInit.rs",
    IDBVersionChangeEventInit
);

web_feature!(
    "IIRFilterNode",
    iir_filter_node,
    "IIRFilterNode.rs",
    IIRFilterNode
);

web_feature!(
    "IIRFilterOptions",
    iir_filter_options,
    "IIRFilterOptions.rs",
    IIRFilterOptions
);

web_feature!(
    "IdentityAssertionResponse",
    identity_assertion_response,
    "IdentityAssertionResponse.rs",
    IdentityAssertionResponse
);

web_feature!(
    "IdentityCredential",
    identity_credential,
    "IdentityCredential.rs",
    IdentityCredential
);

web_feature!(
    "IdentityCredentialDisconnectOptions",
    identity_credential_disconnect_options,
    "IdentityCredentialDisconnectOptions.rs",
    IdentityCredentialDisconnectOptions
);

web_feature!(
    "IdentityCredentialError",
    identity_credential_error,
    "IdentityCredentialError.rs",
    IdentityCredentialError
);

web_feature!(
    "IdentityCredentialErrorInit",
    identity_credential_error_init,
    "IdentityCredentialErrorInit.rs",
    IdentityCredentialErrorInit
);

web_feature!(
    "IdentityCredentialRequestOptions",
    identity_credential_request_options,
    "IdentityCredentialRequestOptions.rs",
    IdentityCredentialRequestOptions
);

web_feature!(
    "IdentityProvider",
    identity_provider,
    "IdentityProvider.rs",
    IdentityProvider
);

web_feature!(
    "IdentityProviderAPIConfig",
    identity_provider_api_config,
    "IdentityProviderAPIConfig.rs",
    IdentityProviderAPIConfig
);

web_feature!(
    "IdentityProviderAccount",
    identity_provider_account,
    "IdentityProviderAccount.rs",
    IdentityProviderAccount
);

web_feature!(
    "IdentityProviderAccountList",
    identity_provider_account_list,
    "IdentityProviderAccountList.rs",
    IdentityProviderAccountList
);

web_feature!(
    "IdentityProviderBranding",
    identity_provider_branding,
    "IdentityProviderBranding.rs",
    IdentityProviderBranding
);

web_feature!(
    "IdentityProviderClientMetadata",
    identity_provider_client_metadata,
    "IdentityProviderClientMetadata.rs",
    IdentityProviderClientMetadata
);

web_feature!(
    "IdentityProviderConfig",
    identity_provider_config,
    "IdentityProviderConfig.rs",
    IdentityProviderConfig
);

web_feature!(
    "IdentityProviderIcon",
    identity_provider_icon,
    "IdentityProviderIcon.rs",
    IdentityProviderIcon
);

web_feature!(
    "IdentityProviderRequestOptions",
    identity_provider_request_options,
    "IdentityProviderRequestOptions.rs",
    IdentityProviderRequestOptions
);

web_feature!(
    "IdentityProviderWellKnown",
    identity_provider_well_known,
    "IdentityProviderWellKnown.rs",
    IdentityProviderWellKnown
);

web_feature!(
    "IdentityResolveOptions",
    identity_resolve_options,
    "IdentityResolveOptions.rs",
    IdentityResolveOptions
);

web_feature!(
    "IdentityUserInfo",
    identity_user_info,
    "IdentityUserInfo.rs",
    IdentityUserInfo
);

web_feature!(
    "IdleDeadline",
    idle_deadline,
    "IdleDeadline.rs",
    IdleDeadline
);

web_feature!(
    "IdleDetector",
    idle_detector,
    "IdleDetector.rs",
    IdleDetector
);

web_feature!("IdleOptions", idle_options, "IdleOptions.rs", IdleOptions);

web_feature!(
    "IdleRequestOptions",
    idle_request_options,
    "IdleRequestOptions.rs",
    IdleRequestOptions
);

web_feature!("ImageBitmap", image_bitmap, "ImageBitmap.rs", ImageBitmap);

web_feature!(
    "ImageBitmapOptions",
    image_bitmap_options,
    "ImageBitmapOptions.rs",
    ImageBitmapOptions
);

web_feature!(
    "ImageBitmapRenderingContext",
    image_bitmap_rendering_context,
    "ImageBitmapRenderingContext.rs",
    ImageBitmapRenderingContext
);

web_feature!(
    "ImageBitmapRenderingContextSettings",
    image_bitmap_rendering_context_settings,
    "ImageBitmapRenderingContextSettings.rs",
    ImageBitmapRenderingContextSettings
);

web_feature!(
    "ImageCapture",
    image_capture,
    "ImageCapture.rs",
    ImageCapture
);

web_feature!("ImageData", image_data, "ImageData.rs", ImageData);

web_feature!(
    "ImageDataSettings",
    image_data_settings,
    "ImageDataSettings.rs",
    ImageDataSettings
);

web_feature!(
    "ImageDecodeOptions",
    image_decode_options,
    "ImageDecodeOptions.rs",
    ImageDecodeOptions
);

web_feature!(
    "ImageDecodeResult",
    image_decode_result,
    "ImageDecodeResult.rs",
    ImageDecodeResult
);

web_feature!(
    "ImageDecoder",
    image_decoder,
    "ImageDecoder.rs",
    ImageDecoder
);

web_feature!(
    "ImageDecoderInit",
    image_decoder_init,
    "ImageDecoderInit.rs",
    ImageDecoderInit
);

web_feature!(
    "ImageEncodeOptions",
    image_encode_options,
    "ImageEncodeOptions.rs",
    ImageEncodeOptions
);

web_feature!(
    "ImageResource",
    image_resource,
    "ImageResource.rs",
    ImageResource
);

web_feature!("ImageTrack", image_track, "ImageTrack.rs", ImageTrack);

web_feature!(
    "ImageTrackList",
    image_track_list,
    "ImageTrackList.rs",
    ImageTrackList
);

web_feature!(
    "ImportNodeOptions",
    import_node_options,
    "ImportNodeOptions.rs",
    ImportNodeOptions
);

web_feature!("Ink", ink, "Ink.rs", Ink);

web_feature!(
    "InkPresenterParam",
    ink_presenter_param,
    "InkPresenterParam.rs",
    InkPresenterParam
);

web_feature!(
    "InkTrailStyle",
    ink_trail_style,
    "InkTrailStyle.rs",
    InkTrailStyle
);

web_feature!(
    "InputDeviceCapabilities",
    input_device_capabilities,
    "InputDeviceCapabilities.rs",
    InputDeviceCapabilities
);

web_feature!(
    "InputDeviceCapabilitiesInit",
    input_device_capabilities_init,
    "InputDeviceCapabilitiesInit.rs",
    InputDeviceCapabilitiesInit
);

web_feature!(
    "InputDeviceInfo",
    input_device_info,
    "InputDeviceInfo.rs",
    InputDeviceInfo
);

web_feature!("InputEvent", input_event, "InputEvent.rs", InputEvent);

web_feature!(
    "InputEventInit",
    input_event_init,
    "InputEventInit.rs",
    InputEventInit
);

web_feature!(
    "InstallEvent",
    install_event,
    "InstallEvent.rs",
    InstallEvent
);

web_feature!("Instance", instance, "Instance.rs", Instance);

web_feature!(
    "IntegrityViolationReportBody",
    integrity_violation_report_body,
    "IntegrityViolationReportBody.rs",
    IntegrityViolationReportBody
);

web_feature!(
    "InterestGroupBiddingAndScoringScriptRunnerGlobalScope",
    interest_group_bidding_and_scoring_script_runner_global_scope,
    "InterestGroupBiddingAndScoringScriptRunnerGlobalScope.rs",
    InterestGroupBiddingAndScoringScriptRunnerGlobalScope
);

web_feature!(
    "InterestGroupBiddingScriptRunnerGlobalScope",
    interest_group_bidding_script_runner_global_scope,
    "InterestGroupBiddingScriptRunnerGlobalScope.rs",
    InterestGroupBiddingScriptRunnerGlobalScope
);

web_feature!(
    "InterestGroupReportingScriptRunnerGlobalScope",
    interest_group_reporting_script_runner_global_scope,
    "InterestGroupReportingScriptRunnerGlobalScope.rs",
    InterestGroupReportingScriptRunnerGlobalScope
);

web_feature!(
    "InterestGroupScoringScriptRunnerGlobalScope",
    interest_group_scoring_script_runner_global_scope,
    "InterestGroupScoringScriptRunnerGlobalScope.rs",
    InterestGroupScoringScriptRunnerGlobalScope
);

web_feature!(
    "InterestGroupScriptRunnerGlobalScope",
    interest_group_script_runner_global_scope,
    "InterestGroupScriptRunnerGlobalScope.rs",
    InterestGroupScriptRunnerGlobalScope
);

web_feature!(
    "IntersectionObserver",
    intersection_observer,
    "IntersectionObserver.rs",
    IntersectionObserver
);

web_feature!(
    "IntersectionObserverEntry",
    intersection_observer_entry,
    "IntersectionObserverEntry.rs",
    IntersectionObserverEntry
);

web_feature!(
    "IntersectionObserverEntryInit",
    intersection_observer_entry_init,
    "IntersectionObserverEntryInit.rs",
    IntersectionObserverEntryInit
);

web_feature!(
    "IntersectionObserverInit",
    intersection_observer_init,
    "IntersectionObserverInit.rs",
    IntersectionObserverInit
);

web_feature!(
    "InterventionReportBody",
    intervention_report_body,
    "InterventionReportBody.rs",
    InterventionReportBody
);

web_feature!(
    "IntrinsicSizes",
    intrinsic_sizes,
    "IntrinsicSizes.rs",
    IntrinsicSizes
);

web_feature!(
    "IntrinsicSizesResultOptions",
    intrinsic_sizes_result_options,
    "IntrinsicSizesResultOptions.rs",
    IntrinsicSizesResultOptions
);

web_feature!(
    "IsInputPendingOptions",
    is_input_pending_options,
    "IsInputPendingOptions.rs",
    IsInputPendingOptions
);

web_feature!("ItemDetails", item_details, "ItemDetails.rs", ItemDetails);

web_feature!("JsonWebKey", json_web_key, "JsonWebKey.rs", JsonWebKey);

web_feature!(
    "KHR_parallel_shader_compile",
    khr_parallel_shader_compile,
    "KHR_parallel_shader_compile.rs",
    KHR_parallel_shader_compile
);

web_feature!(
    "KeyAlgorithm",
    key_algorithm,
    "KeyAlgorithm.rs",
    KeyAlgorithm
);

web_feature!(
    "KeyFrameRequestEvent",
    key_frame_request_event,
    "KeyFrameRequestEvent.rs",
    KeyFrameRequestEvent
);

web_feature!(
    "KeySystemTrackConfiguration",
    key_system_track_configuration,
    "KeySystemTrackConfiguration.rs",
    KeySystemTrackConfiguration
);

web_feature!("Keyboard", keyboard, "Keyboard.rs", Keyboard);

web_feature!(
    "KeyboardEvent",
    keyboard_event,
    "KeyboardEvent.rs",
    KeyboardEvent
);

web_feature!(
    "KeyboardEventInit",
    keyboard_event_init,
    "KeyboardEventInit.rs",
    KeyboardEventInit
);

web_feature!(
    "KeyboardLayoutMap",
    keyboard_layout_map,
    "KeyboardLayoutMap.rs",
    KeyboardLayoutMap
);

web_feature!(
    "KeyframeAnimationOptions",
    keyframe_animation_options,
    "KeyframeAnimationOptions.rs",
    KeyframeAnimationOptions
);

web_feature!(
    "KeyframeEffect",
    keyframe_effect,
    "KeyframeEffect.rs",
    KeyframeEffect
);

web_feature!(
    "KeyframeEffectOptions",
    keyframe_effect_options,
    "KeyframeEffectOptions.rs",
    KeyframeEffectOptions
);

web_feature!("Landmark", landmark, "Landmark.rs", Landmark);

web_feature!(
    "LanguageDetectionResult",
    language_detection_result,
    "LanguageDetectionResult.rs",
    LanguageDetectionResult
);

web_feature!(
    "LanguageDetector",
    language_detector,
    "LanguageDetector.rs",
    LanguageDetector
);

web_feature!(
    "LanguageDetectorCreateCoreOptions",
    language_detector_create_core_options,
    "LanguageDetectorCreateCoreOptions.rs",
    LanguageDetectorCreateCoreOptions
);

web_feature!(
    "LanguageDetectorCreateOptions",
    language_detector_create_options,
    "LanguageDetectorCreateOptions.rs",
    LanguageDetectorCreateOptions
);

web_feature!(
    "LanguageDetectorDetectOptions",
    language_detector_detect_options,
    "LanguageDetectorDetectOptions.rs",
    LanguageDetectorDetectOptions
);

web_feature!(
    "LargestContentfulPaint",
    largest_contentful_paint,
    "LargestContentfulPaint.rs",
    LargestContentfulPaint
);

web_feature!(
    "LaunchParams",
    launch_params,
    "LaunchParams.rs",
    LaunchParams
);

web_feature!("LaunchQueue", launch_queue, "LaunchQueue.rs", LaunchQueue);

web_feature!("LayoutChild", layout_child, "LayoutChild.rs", LayoutChild);

web_feature!(
    "LayoutConstraints",
    layout_constraints,
    "LayoutConstraints.rs",
    LayoutConstraints
);

web_feature!(
    "LayoutConstraintsOptions",
    layout_constraints_options,
    "LayoutConstraintsOptions.rs",
    LayoutConstraintsOptions
);

web_feature!("LayoutEdges", layout_edges, "LayoutEdges.rs", LayoutEdges);

web_feature!(
    "LayoutFragment",
    layout_fragment,
    "LayoutFragment.rs",
    LayoutFragment
);

web_feature!(
    "LayoutOptions",
    layout_options,
    "LayoutOptions.rs",
    LayoutOptions
);

web_feature!("LayoutShift", layout_shift, "LayoutShift.rs", LayoutShift);

web_feature!(
    "LayoutShiftAttribution",
    layout_shift_attribution,
    "LayoutShiftAttribution.rs",
    LayoutShiftAttribution
);

web_feature!(
    "LayoutWorkletGlobalScope",
    layout_worklet_global_scope,
    "LayoutWorkletGlobalScope.rs",
    LayoutWorkletGlobalScope
);

web_feature!(
    "LinearAccelerationSensor",
    linear_acceleration_sensor,
    "LinearAccelerationSensor.rs",
    LinearAccelerationSensor
);

web_feature!("Location", location, "Location.rs", Location);

web_feature!("Lock", lock, "Lock.rs", Lock);

web_feature!("LockInfo", lock_info, "LockInfo.rs", LockInfo);

web_feature!("LockManager", lock_manager, "LockManager.rs", LockManager);

web_feature!(
    "LockManagerSnapshot",
    lock_manager_snapshot,
    "LockManagerSnapshot.rs",
    LockManagerSnapshot
);

web_feature!("LockOptions", lock_options, "LockOptions.rs", LockOptions);

web_feature!("MIDIAccess", midi_access, "MIDIAccess.rs", MIDIAccess);

web_feature!(
    "MIDIConnectionEvent",
    midi_connection_event,
    "MIDIConnectionEvent.rs",
    MIDIConnectionEvent
);

web_feature!(
    "MIDIConnectionEventInit",
    midi_connection_event_init,
    "MIDIConnectionEventInit.rs",
    MIDIConnectionEventInit
);

web_feature!("MIDIInput", midi_input, "MIDIInput.rs", MIDIInput);

web_feature!(
    "MIDIInputMap",
    midi_input_map,
    "MIDIInputMap.rs",
    MIDIInputMap
);

web_feature!(
    "MIDIMessageEvent",
    midi_message_event,
    "MIDIMessageEvent.rs",
    MIDIMessageEvent
);

web_feature!(
    "MIDIMessageEventInit",
    midi_message_event_init,
    "MIDIMessageEventInit.rs",
    MIDIMessageEventInit
);

web_feature!("MIDIOptions", midi_options, "MIDIOptions.rs", MIDIOptions);

web_feature!("MIDIOutput", midi_output, "MIDIOutput.rs", MIDIOutput);

web_feature!(
    "MIDIOutputMap",
    midi_output_map,
    "MIDIOutputMap.rs",
    MIDIOutputMap
);

web_feature!("MIDIPort", midi_port, "MIDIPort.rs", MIDIPort);

web_feature!("ML", ml, "ML.rs", ML);

web_feature!(
    "MLArgMinMaxOptions",
    ml_arg_min_max_options,
    "MLArgMinMaxOptions.rs",
    MLArgMinMaxOptions
);

web_feature!(
    "MLBatchNormalizationOptions",
    ml_batch_normalization_options,
    "MLBatchNormalizationOptions.rs",
    MLBatchNormalizationOptions
);

web_feature!(
    "MLBatchNormalizationSupportLimits",
    ml_batch_normalization_support_limits,
    "MLBatchNormalizationSupportLimits.rs",
    MLBatchNormalizationSupportLimits
);

web_feature!(
    "MLBinarySupportLimits",
    ml_binary_support_limits,
    "MLBinarySupportLimits.rs",
    MLBinarySupportLimits
);

web_feature!(
    "MLClampOptions",
    ml_clamp_options,
    "MLClampOptions.rs",
    MLClampOptions
);

web_feature!(
    "MLConcatSupportLimits",
    ml_concat_support_limits,
    "MLConcatSupportLimits.rs",
    MLConcatSupportLimits
);

web_feature!("MLContext", ml_context, "MLContext.rs", MLContext);

web_feature!(
    "MLContextLostInfo",
    ml_context_lost_info,
    "MLContextLostInfo.rs",
    MLContextLostInfo
);

web_feature!(
    "MLContextOptions",
    ml_context_options,
    "MLContextOptions.rs",
    MLContextOptions
);

web_feature!(
    "MLConv2dOptions",
    ml_conv2d_options,
    "MLConv2dOptions.rs",
    MLConv2dOptions
);

web_feature!(
    "MLConv2dSupportLimits",
    ml_conv2d_support_limits,
    "MLConv2dSupportLimits.rs",
    MLConv2dSupportLimits
);

web_feature!(
    "MLConvTranspose2dOptions",
    ml_conv_transpose2d_options,
    "MLConvTranspose2dOptions.rs",
    MLConvTranspose2dOptions
);

web_feature!(
    "MLCumulativeSumOptions",
    ml_cumulative_sum_options,
    "MLCumulativeSumOptions.rs",
    MLCumulativeSumOptions
);

web_feature!(
    "MLDataTypeLimits",
    ml_data_type_limits,
    "MLDataTypeLimits.rs",
    MLDataTypeLimits
);

web_feature!(
    "MLEluOptions",
    ml_elu_options,
    "MLEluOptions.rs",
    MLEluOptions
);

web_feature!(
    "MLGatherOptions",
    ml_gather_options,
    "MLGatherOptions.rs",
    MLGatherOptions
);

web_feature!(
    "MLGatherSupportLimits",
    ml_gather_support_limits,
    "MLGatherSupportLimits.rs",
    MLGatherSupportLimits
);

web_feature!(
    "MLGemmOptions",
    ml_gemm_options,
    "MLGemmOptions.rs",
    MLGemmOptions
);

web_feature!(
    "MLGemmSupportLimits",
    ml_gemm_support_limits,
    "MLGemmSupportLimits.rs",
    MLGemmSupportLimits
);

web_feature!("MLGraph", ml_graph, "MLGraph.rs", MLGraph);

web_feature!(
    "MLGraphBuilder",
    ml_graph_builder,
    "MLGraphBuilder.rs",
    MLGraphBuilder
);

web_feature!(
    "MLGruCellOptions",
    ml_gru_cell_options,
    "MLGruCellOptions.rs",
    MLGruCellOptions
);

web_feature!(
    "MLGruCellSupportLimits",
    ml_gru_cell_support_limits,
    "MLGruCellSupportLimits.rs",
    MLGruCellSupportLimits
);

web_feature!(
    "MLGruOptions",
    ml_gru_options,
    "MLGruOptions.rs",
    MLGruOptions
);

web_feature!(
    "MLGruSupportLimits",
    ml_gru_support_limits,
    "MLGruSupportLimits.rs",
    MLGruSupportLimits
);

web_feature!(
    "MLHardSigmoidOptions",
    ml_hard_sigmoid_options,
    "MLHardSigmoidOptions.rs",
    MLHardSigmoidOptions
);

web_feature!(
    "MLInstanceNormalizationOptions",
    ml_instance_normalization_options,
    "MLInstanceNormalizationOptions.rs",
    MLInstanceNormalizationOptions
);

web_feature!(
    "MLLayerNormalizationOptions",
    ml_layer_normalization_options,
    "MLLayerNormalizationOptions.rs",
    MLLayerNormalizationOptions
);

web_feature!(
    "MLLeakyReluOptions",
    ml_leaky_relu_options,
    "MLLeakyReluOptions.rs",
    MLLeakyReluOptions
);

web_feature!(
    "MLLinearOptions",
    ml_linear_options,
    "MLLinearOptions.rs",
    MLLinearOptions
);

web_feature!(
    "MLLogicalNotSupportLimits",
    ml_logical_not_support_limits,
    "MLLogicalNotSupportLimits.rs",
    MLLogicalNotSupportLimits
);

web_feature!(
    "MLLstmCellOptions",
    ml_lstm_cell_options,
    "MLLstmCellOptions.rs",
    MLLstmCellOptions
);

web_feature!(
    "MLLstmCellSupportLimits",
    ml_lstm_cell_support_limits,
    "MLLstmCellSupportLimits.rs",
    MLLstmCellSupportLimits
);

web_feature!(
    "MLLstmOptions",
    ml_lstm_options,
    "MLLstmOptions.rs",
    MLLstmOptions
);

web_feature!(
    "MLLstmSupportLimits",
    ml_lstm_support_limits,
    "MLLstmSupportLimits.rs",
    MLLstmSupportLimits
);

web_feature!(
    "MLNormalizationSupportLimits",
    ml_normalization_support_limits,
    "MLNormalizationSupportLimits.rs",
    MLNormalizationSupportLimits
);

web_feature!(
    "MLOpSupportLimits",
    ml_op_support_limits,
    "MLOpSupportLimits.rs",
    MLOpSupportLimits
);

web_feature!("MLOperand", ml_operand, "MLOperand.rs", MLOperand);

web_feature!(
    "MLOperandDescriptor",
    ml_operand_descriptor,
    "MLOperandDescriptor.rs",
    MLOperandDescriptor
);

web_feature!(
    "MLOperatorOptions",
    ml_operator_options,
    "MLOperatorOptions.rs",
    MLOperatorOptions
);

web_feature!(
    "MLPadOptions",
    ml_pad_options,
    "MLPadOptions.rs",
    MLPadOptions
);

web_feature!(
    "MLPool2dOptions",
    ml_pool2d_options,
    "MLPool2dOptions.rs",
    MLPool2dOptions
);

web_feature!(
    "MLPreluSupportLimits",
    ml_prelu_support_limits,
    "MLPreluSupportLimits.rs",
    MLPreluSupportLimits
);

web_feature!(
    "MLQuantizeDequantizeLinearSupportLimits",
    ml_quantize_dequantize_linear_support_limits,
    "MLQuantizeDequantizeLinearSupportLimits.rs",
    MLQuantizeDequantizeLinearSupportLimits
);

web_feature!("MLRankRange", ml_rank_range, "MLRankRange.rs", MLRankRange);

web_feature!(
    "MLReduceOptions",
    ml_reduce_options,
    "MLReduceOptions.rs",
    MLReduceOptions
);

web_feature!(
    "MLResample2dOptions",
    ml_resample2d_options,
    "MLResample2dOptions.rs",
    MLResample2dOptions
);

web_feature!(
    "MLReverseOptions",
    ml_reverse_options,
    "MLReverseOptions.rs",
    MLReverseOptions
);

web_feature!(
    "MLScatterOptions",
    ml_scatter_options,
    "MLScatterOptions.rs",
    MLScatterOptions
);

web_feature!(
    "MLScatterSupportLimits",
    ml_scatter_support_limits,
    "MLScatterSupportLimits.rs",
    MLScatterSupportLimits
);

web_feature!(
    "MLSingleInputSupportLimits",
    ml_single_input_support_limits,
    "MLSingleInputSupportLimits.rs",
    MLSingleInputSupportLimits
);

web_feature!(
    "MLSliceOptions",
    ml_slice_options,
    "MLSliceOptions.rs",
    MLSliceOptions
);

web_feature!(
    "MLSplitOptions",
    ml_split_options,
    "MLSplitOptions.rs",
    MLSplitOptions
);

web_feature!(
    "MLSplitSupportLimits",
    ml_split_support_limits,
    "MLSplitSupportLimits.rs",
    MLSplitSupportLimits
);

web_feature!("MLTensor", ml_tensor, "MLTensor.rs", MLTensor);

web_feature!(
    "MLTensorDescriptor",
    ml_tensor_descriptor,
    "MLTensorDescriptor.rs",
    MLTensorDescriptor
);

web_feature!(
    "MLTensorLimits",
    ml_tensor_limits,
    "MLTensorLimits.rs",
    MLTensorLimits
);

web_feature!(
    "MLTransposeOptions",
    ml_transpose_options,
    "MLTransposeOptions.rs",
    MLTransposeOptions
);

web_feature!(
    "MLTriangularOptions",
    ml_triangular_options,
    "MLTriangularOptions.rs",
    MLTriangularOptions
);

web_feature!(
    "MLWhereSupportLimits",
    ml_where_support_limits,
    "MLWhereSupportLimits.rs",
    MLWhereSupportLimits
);

web_feature!(
    "Magnetometer",
    magnetometer,
    "Magnetometer.rs",
    Magnetometer
);

web_feature!(
    "MagnetometerSensorOptions",
    magnetometer_sensor_options,
    "MagnetometerSensorOptions.rs",
    MagnetometerSensorOptions
);

web_feature!(
    "ManagedMediaSource",
    managed_media_source,
    "ManagedMediaSource.rs",
    ManagedMediaSource
);

web_feature!(
    "ManagedSourceBuffer",
    managed_source_buffer,
    "ManagedSourceBuffer.rs",
    ManagedSourceBuffer
);

web_feature!(
    "MathMLElement",
    math_ml_element,
    "MathMLElement.rs",
    MathMLElement
);

web_feature!(
    "MediaCapabilities",
    media_capabilities,
    "MediaCapabilities.rs",
    MediaCapabilities
);

web_feature!(
    "MediaCapabilitiesDecodingInfo",
    media_capabilities_decoding_info,
    "MediaCapabilitiesDecodingInfo.rs",
    MediaCapabilitiesDecodingInfo
);

web_feature!(
    "MediaCapabilitiesEncodingInfo",
    media_capabilities_encoding_info,
    "MediaCapabilitiesEncodingInfo.rs",
    MediaCapabilitiesEncodingInfo
);

web_feature!(
    "MediaCapabilitiesInfo",
    media_capabilities_info,
    "MediaCapabilitiesInfo.rs",
    MediaCapabilitiesInfo
);

web_feature!(
    "MediaCapabilitiesKeySystemConfiguration",
    media_capabilities_key_system_configuration,
    "MediaCapabilitiesKeySystemConfiguration.rs",
    MediaCapabilitiesKeySystemConfiguration
);

web_feature!(
    "MediaConfiguration",
    media_configuration,
    "MediaConfiguration.rs",
    MediaConfiguration
);

web_feature!(
    "MediaDecodingConfiguration",
    media_decoding_configuration,
    "MediaDecodingConfiguration.rs",
    MediaDecodingConfiguration
);

web_feature!(
    "MediaDeviceInfo",
    media_device_info,
    "MediaDeviceInfo.rs",
    MediaDeviceInfo
);

web_feature!(
    "MediaDevices",
    media_devices,
    "MediaDevices.rs",
    MediaDevices
);

web_feature!(
    "MediaElementAudioSourceNode",
    media_element_audio_source_node,
    "MediaElementAudioSourceNode.rs",
    MediaElementAudioSourceNode
);

web_feature!(
    "MediaElementAudioSourceOptions",
    media_element_audio_source_options,
    "MediaElementAudioSourceOptions.rs",
    MediaElementAudioSourceOptions
);

web_feature!(
    "MediaEncodingConfiguration",
    media_encoding_configuration,
    "MediaEncodingConfiguration.rs",
    MediaEncodingConfiguration
);

web_feature!(
    "MediaEncryptedEvent",
    media_encrypted_event,
    "MediaEncryptedEvent.rs",
    MediaEncryptedEvent
);

web_feature!(
    "MediaEncryptedEventInit",
    media_encrypted_event_init,
    "MediaEncryptedEventInit.rs",
    MediaEncryptedEventInit
);

web_feature!("MediaError", media_error, "MediaError.rs", MediaError);

web_feature!("MediaImage", media_image, "MediaImage.rs", MediaImage);

web_feature!(
    "MediaKeyMessageEvent",
    media_key_message_event,
    "MediaKeyMessageEvent.rs",
    MediaKeyMessageEvent
);

web_feature!(
    "MediaKeyMessageEventInit",
    media_key_message_event_init,
    "MediaKeyMessageEventInit.rs",
    MediaKeyMessageEventInit
);

web_feature!(
    "MediaKeySession",
    media_key_session,
    "MediaKeySession.rs",
    MediaKeySession
);

web_feature!(
    "MediaKeyStatusMap",
    media_key_status_map,
    "MediaKeyStatusMap.rs",
    MediaKeyStatusMap
);

web_feature!(
    "MediaKeySystemAccess",
    media_key_system_access,
    "MediaKeySystemAccess.rs",
    MediaKeySystemAccess
);

web_feature!(
    "MediaKeySystemConfiguration",
    media_key_system_configuration,
    "MediaKeySystemConfiguration.rs",
    MediaKeySystemConfiguration
);

web_feature!(
    "MediaKeySystemMediaCapability",
    media_key_system_media_capability,
    "MediaKeySystemMediaCapability.rs",
    MediaKeySystemMediaCapability
);

web_feature!("MediaKeys", media_keys, "MediaKeys.rs", MediaKeys);

web_feature!(
    "MediaKeysPolicy",
    media_keys_policy,
    "MediaKeysPolicy.rs",
    MediaKeysPolicy
);

web_feature!("MediaList", media_list, "MediaList.rs", MediaList);

web_feature!(
    "MediaMetadata",
    media_metadata,
    "MediaMetadata.rs",
    MediaMetadata
);

web_feature!(
    "MediaMetadataInit",
    media_metadata_init,
    "MediaMetadataInit.rs",
    MediaMetadataInit
);

web_feature!(
    "MediaPositionState",
    media_position_state,
    "MediaPositionState.rs",
    MediaPositionState
);

web_feature!(
    "MediaQueryList",
    media_query_list,
    "MediaQueryList.rs",
    MediaQueryList
);

web_feature!(
    "MediaQueryListEvent",
    media_query_list_event,
    "MediaQueryListEvent.rs",
    MediaQueryListEvent
);

web_feature!(
    "MediaQueryListEventInit",
    media_query_list_event_init,
    "MediaQueryListEventInit.rs",
    MediaQueryListEventInit
);

web_feature!(
    "MediaRecorder",
    media_recorder,
    "MediaRecorder.rs",
    MediaRecorder
);

web_feature!(
    "MediaRecorderOptions",
    media_recorder_options,
    "MediaRecorderOptions.rs",
    MediaRecorderOptions
);

web_feature!(
    "MediaSession",
    media_session,
    "MediaSession.rs",
    MediaSession
);

web_feature!(
    "MediaSessionActionDetails",
    media_session_action_details,
    "MediaSessionActionDetails.rs",
    MediaSessionActionDetails
);

web_feature!(
    "MediaSettingsRange",
    media_settings_range,
    "MediaSettingsRange.rs",
    MediaSettingsRange
);

web_feature!("MediaSource", media_source, "MediaSource.rs", MediaSource);

web_feature!(
    "MediaSourceHandle",
    media_source_handle,
    "MediaSourceHandle.rs",
    MediaSourceHandle
);

web_feature!("MediaStream", media_stream, "MediaStream.rs", MediaStream);

web_feature!(
    "MediaStreamAudioDestinationNode",
    media_stream_audio_destination_node,
    "MediaStreamAudioDestinationNode.rs",
    MediaStreamAudioDestinationNode
);

web_feature!(
    "MediaStreamAudioSourceNode",
    media_stream_audio_source_node,
    "MediaStreamAudioSourceNode.rs",
    MediaStreamAudioSourceNode
);

web_feature!(
    "MediaStreamAudioSourceOptions",
    media_stream_audio_source_options,
    "MediaStreamAudioSourceOptions.rs",
    MediaStreamAudioSourceOptions
);

web_feature!(
    "MediaStreamConstraints",
    media_stream_constraints,
    "MediaStreamConstraints.rs",
    MediaStreamConstraints
);

web_feature!(
    "MediaStreamTrack",
    media_stream_track,
    "MediaStreamTrack.rs",
    MediaStreamTrack
);

web_feature!(
    "MediaStreamTrackAudioSourceNode",
    media_stream_track_audio_source_node,
    "MediaStreamTrackAudioSourceNode.rs",
    MediaStreamTrackAudioSourceNode
);

web_feature!(
    "MediaStreamTrackAudioSourceOptions",
    media_stream_track_audio_source_options,
    "MediaStreamTrackAudioSourceOptions.rs",
    MediaStreamTrackAudioSourceOptions
);

web_feature!(
    "MediaStreamTrackEvent",
    media_stream_track_event,
    "MediaStreamTrackEvent.rs",
    MediaStreamTrackEvent
);

web_feature!(
    "MediaStreamTrackEventInit",
    media_stream_track_event_init,
    "MediaStreamTrackEventInit.rs",
    MediaStreamTrackEventInit
);

web_feature!(
    "MediaStreamTrackProcessor",
    media_stream_track_processor,
    "MediaStreamTrackProcessor.rs",
    MediaStreamTrackProcessor
);

web_feature!(
    "MediaStreamTrackProcessorInit",
    media_stream_track_processor_init,
    "MediaStreamTrackProcessorInit.rs",
    MediaStreamTrackProcessorInit
);

web_feature!(
    "MediaTrackCapabilities",
    media_track_capabilities,
    "MediaTrackCapabilities.rs",
    MediaTrackCapabilities
);

web_feature!(
    "MediaTrackConstraintSet",
    media_track_constraint_set,
    "MediaTrackConstraintSet.rs",
    MediaTrackConstraintSet
);

web_feature!(
    "MediaTrackConstraints",
    media_track_constraints,
    "MediaTrackConstraints.rs",
    MediaTrackConstraints
);

web_feature!(
    "MediaTrackSettings",
    media_track_settings,
    "MediaTrackSettings.rs",
    MediaTrackSettings
);

web_feature!(
    "MediaTrackSupportedConstraints",
    media_track_supported_constraints,
    "MediaTrackSupportedConstraints.rs",
    MediaTrackSupportedConstraints
);

web_feature!("Memory", memory, "Memory.rs", Memory);

web_feature!(
    "MemoryAttribution",
    memory_attribution,
    "MemoryAttribution.rs",
    MemoryAttribution
);

web_feature!(
    "MemoryAttributionContainer",
    memory_attribution_container,
    "MemoryAttributionContainer.rs",
    MemoryAttributionContainer
);

web_feature!(
    "MemoryBreakdownEntry",
    memory_breakdown_entry,
    "MemoryBreakdownEntry.rs",
    MemoryBreakdownEntry
);

web_feature!(
    "MemoryDescriptor",
    memory_descriptor,
    "MemoryDescriptor.rs",
    MemoryDescriptor
);

web_feature!(
    "MemoryMeasurement",
    memory_measurement,
    "MemoryMeasurement.rs",
    MemoryMeasurement
);

web_feature!(
    "MessageChannel",
    message_channel,
    "MessageChannel.rs",
    MessageChannel
);

web_feature!(
    "MessageEvent",
    message_event,
    "MessageEvent.rs",
    MessageEvent
);

web_feature!(
    "MessageEventInit",
    message_event_init,
    "MessageEventInit.rs",
    MessageEventInit
);

web_feature!("MessagePort", message_port, "MessagePort.rs", MessagePort);

web_feature!(
    "MidiPermissionDescriptor",
    midi_permission_descriptor,
    "MidiPermissionDescriptor.rs",
    MidiPermissionDescriptor
);

web_feature!("MimeType", mime_type, "MimeType.rs", MimeType);

web_feature!(
    "MimeTypeArray",
    mime_type_array,
    "MimeTypeArray.rs",
    MimeTypeArray
);

web_feature!(
    "MockCameraConfiguration",
    mock_camera_configuration,
    "MockCameraConfiguration.rs",
    MockCameraConfiguration
);

web_feature!(
    "MockCaptureDeviceConfiguration",
    mock_capture_device_configuration,
    "MockCaptureDeviceConfiguration.rs",
    MockCaptureDeviceConfiguration
);

web_feature!(
    "MockCapturePromptResultConfiguration",
    mock_capture_prompt_result_configuration,
    "MockCapturePromptResultConfiguration.rs",
    MockCapturePromptResultConfiguration
);

web_feature!(
    "MockMicrophoneConfiguration",
    mock_microphone_configuration,
    "MockMicrophoneConfiguration.rs",
    MockMicrophoneConfiguration
);

web_feature!("Module", module, "Module.rs", Module);

web_feature!(
    "ModuleExportDescriptor",
    module_export_descriptor,
    "ModuleExportDescriptor.rs",
    ModuleExportDescriptor
);

web_feature!(
    "ModuleImportDescriptor",
    module_import_descriptor,
    "ModuleImportDescriptor.rs",
    ModuleImportDescriptor
);

web_feature!("MouseEvent", mouse_event, "MouseEvent.rs", MouseEvent);

web_feature!(
    "MouseEventInit",
    mouse_event_init,
    "MouseEventInit.rs",
    MouseEventInit
);

web_feature!(
    "MultiCacheQueryOptions",
    multi_cache_query_options,
    "MultiCacheQueryOptions.rs",
    MultiCacheQueryOptions
);

web_feature!(
    "MutationObserver",
    mutation_observer,
    "MutationObserver.rs",
    MutationObserver
);

web_feature!(
    "MutationObserverInit",
    mutation_observer_init,
    "MutationObserverInit.rs",
    MutationObserverInit
);

web_feature!(
    "MutationRecord",
    mutation_record,
    "MutationRecord.rs",
    MutationRecord
);

web_feature!(
    "NDEFMakeReadOnlyOptions",
    ndef_make_read_only_options,
    "NDEFMakeReadOnlyOptions.rs",
    NDEFMakeReadOnlyOptions
);

web_feature!("NDEFMessage", ndef_message, "NDEFMessage.rs", NDEFMessage);

web_feature!(
    "NDEFMessageInit",
    ndef_message_init,
    "NDEFMessageInit.rs",
    NDEFMessageInit
);

web_feature!("NDEFReader", ndef_reader, "NDEFReader.rs", NDEFReader);

web_feature!(
    "NDEFReadingEvent",
    ndef_reading_event,
    "NDEFReadingEvent.rs",
    NDEFReadingEvent
);

web_feature!(
    "NDEFReadingEventInit",
    ndef_reading_event_init,
    "NDEFReadingEventInit.rs",
    NDEFReadingEventInit
);

web_feature!("NDEFRecord", ndef_record, "NDEFRecord.rs", NDEFRecord);

web_feature!(
    "NDEFRecordInit",
    ndef_record_init,
    "NDEFRecordInit.rs",
    NDEFRecordInit
);

web_feature!(
    "NDEFScanOptions",
    ndef_scan_options,
    "NDEFScanOptions.rs",
    NDEFScanOptions
);

web_feature!(
    "NDEFWriteOptions",
    ndef_write_options,
    "NDEFWriteOptions.rs",
    NDEFWriteOptions
);

web_feature!("NamedFlow", named_flow, "NamedFlow.rs", NamedFlow);

web_feature!(
    "NamedFlowMap",
    named_flow_map,
    "NamedFlowMap.rs",
    NamedFlowMap
);

web_feature!(
    "NamedNodeMap",
    named_node_map,
    "NamedNodeMap.rs",
    NamedNodeMap
);

web_feature!(
    "NavigateEvent",
    navigate_event,
    "NavigateEvent.rs",
    NavigateEvent
);

web_feature!(
    "NavigateEventInit",
    navigate_event_init,
    "NavigateEventInit.rs",
    NavigateEventInit
);

web_feature!("Navigation", navigation, "Navigation.rs", Navigation);

web_feature!(
    "NavigationActivation",
    navigation_activation,
    "NavigationActivation.rs",
    NavigationActivation
);

web_feature!(
    "NavigationCurrentEntryChangeEvent",
    navigation_current_entry_change_event,
    "NavigationCurrentEntryChangeEvent.rs",
    NavigationCurrentEntryChangeEvent
);

web_feature!(
    "NavigationCurrentEntryChangeEventInit",
    navigation_current_entry_change_event_init,
    "NavigationCurrentEntryChangeEventInit.rs",
    NavigationCurrentEntryChangeEventInit
);

web_feature!(
    "NavigationDestination",
    navigation_destination,
    "NavigationDestination.rs",
    NavigationDestination
);

web_feature!(
    "NavigationEvent",
    navigation_event,
    "NavigationEvent.rs",
    NavigationEvent
);

web_feature!(
    "NavigationEventInit",
    navigation_event_init,
    "NavigationEventInit.rs",
    NavigationEventInit
);

web_feature!(
    "NavigationHistoryEntry",
    navigation_history_entry,
    "NavigationHistoryEntry.rs",
    NavigationHistoryEntry
);

web_feature!(
    "NavigationInterceptOptions",
    navigation_intercept_options,
    "NavigationInterceptOptions.rs",
    NavigationInterceptOptions
);

web_feature!(
    "NavigationNavigateOptions",
    navigation_navigate_options,
    "NavigationNavigateOptions.rs",
    NavigationNavigateOptions
);

web_feature!(
    "NavigationOptions",
    navigation_options,
    "NavigationOptions.rs",
    NavigationOptions
);

web_feature!(
    "NavigationPreloadManager",
    navigation_preload_manager,
    "NavigationPreloadManager.rs",
    NavigationPreloadManager
);

web_feature!(
    "NavigationPreloadState",
    navigation_preload_state,
    "NavigationPreloadState.rs",
    NavigationPreloadState
);

web_feature!(
    "NavigationReloadOptions",
    navigation_reload_options,
    "NavigationReloadOptions.rs",
    NavigationReloadOptions
);

web_feature!(
    "NavigationResult",
    navigation_result,
    "NavigationResult.rs",
    NavigationResult
);

web_feature!(
    "NavigationTransition",
    navigation_transition,
    "NavigationTransition.rs",
    NavigationTransition
);

web_feature!(
    "NavigationUpdateCurrentEntryOptions",
    navigation_update_current_entry_options,
    "NavigationUpdateCurrentEntryOptions.rs",
    NavigationUpdateCurrentEntryOptions
);

web_feature!("Navigator", navigator, "Navigator.rs", Navigator);

web_feature!(
    "NavigatorLogin",
    navigator_login,
    "NavigatorLogin.rs",
    NavigatorLogin
);

web_feature!(
    "NavigatorManagedData",
    navigator_managed_data,
    "NavigatorManagedData.rs",
    NavigatorManagedData
);

web_feature!(
    "NavigatorUABrandVersion",
    navigator_ua_brand_version,
    "NavigatorUABrandVersion.rs",
    NavigatorUABrandVersion
);

web_feature!(
    "NavigatorUAData",
    navigator_ua_data,
    "NavigatorUAData.rs",
    NavigatorUAData
);

web_feature!(
    "NetworkInformation",
    network_information,
    "NetworkInformation.rs",
    NetworkInformation
);

web_feature!("Node", node, "Node.rs", Node);

web_feature!(
    "NodeIterator",
    node_iterator,
    "NodeIterator.rs",
    NodeIterator
);

web_feature!("NodeList", node_list, "NodeList.rs", NodeList);

web_feature!(
    "NotRestoredReasonDetails",
    not_restored_reason_details,
    "NotRestoredReasonDetails.rs",
    NotRestoredReasonDetails
);

web_feature!(
    "NotRestoredReasons",
    not_restored_reasons,
    "NotRestoredReasons.rs",
    NotRestoredReasons
);

web_feature!(
    "Notification",
    notification,
    "Notification.rs",
    Notification
);

web_feature!(
    "NotificationAction",
    notification_action,
    "NotificationAction.rs",
    NotificationAction
);

web_feature!(
    "NotificationEvent",
    notification_event,
    "NotificationEvent.rs",
    NotificationEvent
);

web_feature!(
    "NotificationEventInit",
    notification_event_init,
    "NotificationEventInit.rs",
    NotificationEventInit
);

web_feature!(
    "NotificationOptions",
    notification_options,
    "NotificationOptions.rs",
    NotificationOptions
);

web_feature!(
    "OES_draw_buffers_indexed",
    oes_draw_buffers_indexed,
    "OES_draw_buffers_indexed.rs",
    OES_draw_buffers_indexed
);

web_feature!(
    "OES_element_index_uint",
    oes_element_index_uint,
    "OES_element_index_uint.rs",
    OES_element_index_uint
);

web_feature!(
    "OES_fbo_render_mipmap",
    oes_fbo_render_mipmap,
    "OES_fbo_render_mipmap.rs",
    OES_fbo_render_mipmap
);

web_feature!(
    "OES_standard_derivatives",
    oes_standard_derivatives,
    "OES_standard_derivatives.rs",
    OES_standard_derivatives
);

web_feature!(
    "OES_texture_float",
    oes_texture_float,
    "OES_texture_float.rs",
    OES_texture_float
);

web_feature!(
    "OES_texture_float_linear",
    oes_texture_float_linear,
    "OES_texture_float_linear.rs",
    OES_texture_float_linear
);

web_feature!(
    "OES_texture_half_float",
    oes_texture_half_float,
    "OES_texture_half_float.rs",
    OES_texture_half_float
);

web_feature!(
    "OES_texture_half_float_linear",
    oes_texture_half_float_linear,
    "OES_texture_half_float_linear.rs",
    OES_texture_half_float_linear
);

web_feature!(
    "OES_vertex_array_object",
    oes_vertex_array_object,
    "OES_vertex_array_object.rs",
    OES_vertex_array_object
);

web_feature!(
    "OTPCredential",
    otp_credential,
    "OTPCredential.rs",
    OTPCredential
);

web_feature!(
    "OTPCredentialRequestOptions",
    otp_credential_request_options,
    "OTPCredentialRequestOptions.rs",
    OTPCredentialRequestOptions
);

web_feature!(
    "OVR_multiview2",
    ovr_multiview2,
    "OVR_multiview2.rs",
    OVR_multiview2
);

web_feature!("Observable", observable, "Observable.rs", Observable);

web_feature!(
    "ObservableEventListenerOptions",
    observable_event_listener_options,
    "ObservableEventListenerOptions.rs",
    ObservableEventListenerOptions
);

web_feature!(
    "ObservableInspector",
    observable_inspector,
    "ObservableInspector.rs",
    ObservableInspector
);

web_feature!(
    "OfflineAudioCompletionEvent",
    offline_audio_completion_event,
    "OfflineAudioCompletionEvent.rs",
    OfflineAudioCompletionEvent
);

web_feature!(
    "OfflineAudioCompletionEventInit",
    offline_audio_completion_event_init,
    "OfflineAudioCompletionEventInit.rs",
    OfflineAudioCompletionEventInit
);

web_feature!(
    "OfflineAudioContext",
    offline_audio_context,
    "OfflineAudioContext.rs",
    OfflineAudioContext
);

web_feature!(
    "OfflineAudioContextOptions",
    offline_audio_context_options,
    "OfflineAudioContextOptions.rs",
    OfflineAudioContextOptions
);

web_feature!(
    "OffscreenCanvas",
    offscreen_canvas,
    "OffscreenCanvas.rs",
    OffscreenCanvas
);

web_feature!(
    "OffscreenCanvasRenderingContext2D",
    offscreen_canvas_rendering_context2d,
    "OffscreenCanvasRenderingContext2D.rs",
    OffscreenCanvasRenderingContext2D
);

web_feature!(
    "OpenFilePickerOptions",
    open_file_picker_options,
    "OpenFilePickerOptions.rs",
    OpenFilePickerOptions
);

web_feature!(
    "OptionalEffectTiming",
    optional_effect_timing,
    "OptionalEffectTiming.rs",
    OptionalEffectTiming
);

web_feature!(
    "OpusEncoderConfig",
    opus_encoder_config,
    "OpusEncoderConfig.rs",
    OpusEncoderConfig
);

web_feature!(
    "OrientationSensor",
    orientation_sensor,
    "OrientationSensor.rs",
    OrientationSensor
);

web_feature!(
    "OrientationSensorOptions",
    orientation_sensor_options,
    "OrientationSensorOptions.rs",
    OrientationSensorOptions
);

web_feature!(
    "OscillatorNode",
    oscillator_node,
    "OscillatorNode.rs",
    OscillatorNode
);

web_feature!(
    "OscillatorOptions",
    oscillator_options,
    "OscillatorOptions.rs",
    OscillatorOptions
);

web_feature!(
    "OverconstrainedError",
    overconstrained_error,
    "OverconstrainedError.rs",
    OverconstrainedError
);

web_feature!(
    "PADebugModeOptions",
    pa_debug_mode_options,
    "PADebugModeOptions.rs",
    PADebugModeOptions
);

web_feature!(
    "PAExtendedHistogramContribution",
    pa_extended_histogram_contribution,
    "PAExtendedHistogramContribution.rs",
    PAExtendedHistogramContribution
);

web_feature!(
    "PAHistogramContribution",
    pa_histogram_contribution,
    "PAHistogramContribution.rs",
    PAHistogramContribution
);

web_feature!(
    "PASignalValue",
    pa_signal_value,
    "PASignalValue.rs",
    PASignalValue
);

web_feature!(
    "PageRevealEvent",
    page_reveal_event,
    "PageRevealEvent.rs",
    PageRevealEvent
);

web_feature!(
    "PageRevealEventInit",
    page_reveal_event_init,
    "PageRevealEventInit.rs",
    PageRevealEventInit
);

web_feature!(
    "PageSwapEvent",
    page_swap_event,
    "PageSwapEvent.rs",
    PageSwapEvent
);

web_feature!(
    "PageSwapEventInit",
    page_swap_event_init,
    "PageSwapEventInit.rs",
    PageSwapEventInit
);

web_feature!(
    "PageTransitionEvent",
    page_transition_event,
    "PageTransitionEvent.rs",
    PageTransitionEvent
);

web_feature!(
    "PageTransitionEventInit",
    page_transition_event_init,
    "PageTransitionEventInit.rs",
    PageTransitionEventInit
);

web_feature!(
    "PaintRenderingContext2D",
    paint_rendering_context2d,
    "PaintRenderingContext2D.rs",
    PaintRenderingContext2D
);

web_feature!(
    "PaintRenderingContext2DSettings",
    paint_rendering_context2d_settings,
    "PaintRenderingContext2DSettings.rs",
    PaintRenderingContext2DSettings
);

web_feature!("PaintSize", paint_size, "PaintSize.rs", PaintSize);

web_feature!(
    "PaintWorkletGlobalScope",
    paint_worklet_global_scope,
    "PaintWorkletGlobalScope.rs",
    PaintWorkletGlobalScope
);

web_feature!("PannerNode", panner_node, "PannerNode.rs", PannerNode);

web_feature!(
    "PannerOptions",
    panner_options,
    "PannerOptions.rs",
    PannerOptions
);

web_feature!(
    "PasswordCredential",
    password_credential,
    "PasswordCredential.rs",
    PasswordCredential
);

web_feature!(
    "PasswordCredentialData",
    password_credential_data,
    "PasswordCredentialData.rs",
    PasswordCredentialData
);

web_feature!("Path2D", path2d, "Path2D.rs", Path2D);

web_feature!("PayerErrors", payer_errors, "PayerErrors.rs", PayerErrors);

web_feature!(
    "PaymentCompleteDetails",
    payment_complete_details,
    "PaymentCompleteDetails.rs",
    PaymentCompleteDetails
);

web_feature!(
    "PaymentCredentialInstrument",
    payment_credential_instrument,
    "PaymentCredentialInstrument.rs",
    PaymentCredentialInstrument
);

web_feature!(
    "PaymentCurrencyAmount",
    payment_currency_amount,
    "PaymentCurrencyAmount.rs",
    PaymentCurrencyAmount
);

web_feature!(
    "PaymentDetailsBase",
    payment_details_base,
    "PaymentDetailsBase.rs",
    PaymentDetailsBase
);

web_feature!(
    "PaymentDetailsInit",
    payment_details_init,
    "PaymentDetailsInit.rs",
    PaymentDetailsInit
);

web_feature!(
    "PaymentDetailsModifier",
    payment_details_modifier,
    "PaymentDetailsModifier.rs",
    PaymentDetailsModifier
);

web_feature!(
    "PaymentDetailsUpdate",
    payment_details_update,
    "PaymentDetailsUpdate.rs",
    PaymentDetailsUpdate
);

web_feature!(
    "PaymentEntityLogo",
    payment_entity_logo,
    "PaymentEntityLogo.rs",
    PaymentEntityLogo
);

web_feature!(
    "PaymentHandlerResponse",
    payment_handler_response,
    "PaymentHandlerResponse.rs",
    PaymentHandlerResponse
);

web_feature!("PaymentItem", payment_item, "PaymentItem.rs", PaymentItem);

web_feature!(
    "PaymentManager",
    payment_manager,
    "PaymentManager.rs",
    PaymentManager
);

web_feature!(
    "PaymentMethodChangeEvent",
    payment_method_change_event,
    "PaymentMethodChangeEvent.rs",
    PaymentMethodChangeEvent
);

web_feature!(
    "PaymentMethodChangeEventInit",
    payment_method_change_event_init,
    "PaymentMethodChangeEventInit.rs",
    PaymentMethodChangeEventInit
);

web_feature!(
    "PaymentMethodData",
    payment_method_data,
    "PaymentMethodData.rs",
    PaymentMethodData
);

web_feature!(
    "PaymentOptions",
    payment_options,
    "PaymentOptions.rs",
    PaymentOptions
);

web_feature!(
    "PaymentRequest",
    payment_request,
    "PaymentRequest.rs",
    PaymentRequest
);

web_feature!(
    "PaymentRequestDetailsUpdate",
    payment_request_details_update,
    "PaymentRequestDetailsUpdate.rs",
    PaymentRequestDetailsUpdate
);

web_feature!(
    "PaymentRequestEvent",
    payment_request_event,
    "PaymentRequestEvent.rs",
    PaymentRequestEvent
);

web_feature!(
    "PaymentRequestEventInit",
    payment_request_event_init,
    "PaymentRequestEventInit.rs",
    PaymentRequestEventInit
);

web_feature!(
    "PaymentRequestUpdateEvent",
    payment_request_update_event,
    "PaymentRequestUpdateEvent.rs",
    PaymentRequestUpdateEvent
);

web_feature!(
    "PaymentRequestUpdateEventInit",
    payment_request_update_event_init,
    "PaymentRequestUpdateEventInit.rs",
    PaymentRequestUpdateEventInit
);

web_feature!(
    "PaymentResponse",
    payment_response,
    "PaymentResponse.rs",
    PaymentResponse
);

web_feature!(
    "PaymentShippingOption",
    payment_shipping_option,
    "PaymentShippingOption.rs",
    PaymentShippingOption
);

web_feature!(
    "PaymentValidationErrors",
    payment_validation_errors,
    "PaymentValidationErrors.rs",
    PaymentValidationErrors
);

web_feature!(
    "Pbkdf2Params",
    pbkdf2_params,
    "Pbkdf2Params.rs",
    Pbkdf2Params
);

web_feature!("Performance", performance, "Performance.rs", Performance);

web_feature!(
    "PerformanceElementTiming",
    performance_element_timing,
    "PerformanceElementTiming.rs",
    PerformanceElementTiming
);

web_feature!(
    "PerformanceEntry",
    performance_entry,
    "PerformanceEntry.rs",
    PerformanceEntry
);

web_feature!(
    "PerformanceEventTiming",
    performance_event_timing,
    "PerformanceEventTiming.rs",
    PerformanceEventTiming
);

web_feature!(
    "PerformanceLongAnimationFrameTiming",
    performance_long_animation_frame_timing,
    "PerformanceLongAnimationFrameTiming.rs",
    PerformanceLongAnimationFrameTiming
);

web_feature!(
    "PerformanceLongTaskTiming",
    performance_long_task_timing,
    "PerformanceLongTaskTiming.rs",
    PerformanceLongTaskTiming
);

web_feature!(
    "PerformanceMark",
    performance_mark,
    "PerformanceMark.rs",
    PerformanceMark
);

web_feature!(
    "PerformanceMarkOptions",
    performance_mark_options,
    "PerformanceMarkOptions.rs",
    PerformanceMarkOptions
);

web_feature!(
    "PerformanceMeasure",
    performance_measure,
    "PerformanceMeasure.rs",
    PerformanceMeasure
);

web_feature!(
    "PerformanceMeasureOptions",
    performance_measure_options,
    "PerformanceMeasureOptions.rs",
    PerformanceMeasureOptions
);

web_feature!(
    "PerformanceNavigation",
    performance_navigation,
    "PerformanceNavigation.rs",
    PerformanceNavigation
);

web_feature!(
    "PerformanceNavigationTiming",
    performance_navigation_timing,
    "PerformanceNavigationTiming.rs",
    PerformanceNavigationTiming
);

web_feature!(
    "PerformanceObserver",
    performance_observer,
    "PerformanceObserver.rs",
    PerformanceObserver
);

web_feature!(
    "PerformanceObserverCallbackOptions",
    performance_observer_callback_options,
    "PerformanceObserverCallbackOptions.rs",
    PerformanceObserverCallbackOptions
);

web_feature!(
    "PerformanceObserverEntryList",
    performance_observer_entry_list,
    "PerformanceObserverEntryList.rs",
    PerformanceObserverEntryList
);

web_feature!(
    "PerformanceObserverInit",
    performance_observer_init,
    "PerformanceObserverInit.rs",
    PerformanceObserverInit
);

web_feature!(
    "PerformancePaintTiming",
    performance_paint_timing,
    "PerformancePaintTiming.rs",
    PerformancePaintTiming
);

web_feature!(
    "PerformanceResourceTiming",
    performance_resource_timing,
    "PerformanceResourceTiming.rs",
    PerformanceResourceTiming
);

web_feature!(
    "PerformanceScriptTiming",
    performance_script_timing,
    "PerformanceScriptTiming.rs",
    PerformanceScriptTiming
);

web_feature!(
    "PerformanceServerTiming",
    performance_server_timing,
    "PerformanceServerTiming.rs",
    PerformanceServerTiming
);

web_feature!(
    "PerformanceTiming",
    performance_timing,
    "PerformanceTiming.rs",
    PerformanceTiming
);

web_feature!(
    "PeriodicSyncEvent",
    periodic_sync_event,
    "PeriodicSyncEvent.rs",
    PeriodicSyncEvent
);

web_feature!(
    "PeriodicSyncEventInit",
    periodic_sync_event_init,
    "PeriodicSyncEventInit.rs",
    PeriodicSyncEventInit
);

web_feature!(
    "PeriodicSyncManager",
    periodic_sync_manager,
    "PeriodicSyncManager.rs",
    PeriodicSyncManager
);

web_feature!(
    "PeriodicWave",
    periodic_wave,
    "PeriodicWave.rs",
    PeriodicWave
);

web_feature!(
    "PeriodicWaveConstraints",
    periodic_wave_constraints,
    "PeriodicWaveConstraints.rs",
    PeriodicWaveConstraints
);

web_feature!(
    "PeriodicWaveOptions",
    periodic_wave_options,
    "PeriodicWaveOptions.rs",
    PeriodicWaveOptions
);

web_feature!(
    "PermissionDescriptor",
    permission_descriptor,
    "PermissionDescriptor.rs",
    PermissionDescriptor
);

web_feature!(
    "PermissionSetParameters",
    permission_set_parameters,
    "PermissionSetParameters.rs",
    PermissionSetParameters
);

web_feature!(
    "PermissionStatus",
    permission_status,
    "PermissionStatus.rs",
    PermissionStatus
);

web_feature!("Permissions", permissions, "Permissions.rs", Permissions);

web_feature!(
    "PermissionsPolicy",
    permissions_policy,
    "PermissionsPolicy.rs",
    PermissionsPolicy
);

web_feature!(
    "PermissionsPolicyViolationReportBody",
    permissions_policy_violation_report_body,
    "PermissionsPolicyViolationReportBody.rs",
    PermissionsPolicyViolationReportBody
);

web_feature!(
    "PhotoCapabilities",
    photo_capabilities,
    "PhotoCapabilities.rs",
    PhotoCapabilities
);

web_feature!(
    "PhotoSettings",
    photo_settings,
    "PhotoSettings.rs",
    PhotoSettings
);

web_feature!(
    "PictureInPictureEvent",
    picture_in_picture_event,
    "PictureInPictureEvent.rs",
    PictureInPictureEvent
);

web_feature!(
    "PictureInPictureEventInit",
    picture_in_picture_event_init,
    "PictureInPictureEventInit.rs",
    PictureInPictureEventInit
);

web_feature!(
    "PictureInPictureWindow",
    picture_in_picture_window,
    "PictureInPictureWindow.rs",
    PictureInPictureWindow
);

web_feature!("PlaneLayout", plane_layout, "PlaneLayout.rs", PlaneLayout);

web_feature!("Plugin", plugin, "Plugin.rs", Plugin);

web_feature!("PluginArray", plugin_array, "PluginArray.rs", PluginArray);

web_feature!("Point2D", point2d, "Point2D.rs", Point2D);

web_feature!(
    "PointerEvent",
    pointer_event,
    "PointerEvent.rs",
    PointerEvent
);

web_feature!(
    "PointerEventInit",
    pointer_event_init,
    "PointerEventInit.rs",
    PointerEventInit
);

web_feature!(
    "PointerLockOptions",
    pointer_lock_options,
    "PointerLockOptions.rs",
    PointerLockOptions
);

web_feature!(
    "PointerTimeline",
    pointer_timeline,
    "PointerTimeline.rs",
    PointerTimeline
);

web_feature!(
    "PointerTimelineOptions",
    pointer_timeline_options,
    "PointerTimelineOptions.rs",
    PointerTimelineOptions
);

web_feature!(
    "PopStateEvent",
    pop_state_event,
    "PopStateEvent.rs",
    PopStateEvent
);

web_feature!(
    "PopStateEventInit",
    pop_state_event_init,
    "PopStateEventInit.rs",
    PopStateEventInit
);

web_feature!(
    "PortalActivateEvent",
    portal_activate_event,
    "PortalActivateEvent.rs",
    PortalActivateEvent
);

web_feature!(
    "PortalActivateEventInit",
    portal_activate_event_init,
    "PortalActivateEventInit.rs",
    PortalActivateEventInit
);

web_feature!(
    "PortalActivateOptions",
    portal_activate_options,
    "PortalActivateOptions.rs",
    PortalActivateOptions
);

web_feature!("PortalHost", portal_host, "PortalHost.rs", PortalHost);

web_feature!(
    "PositionOptions",
    position_options,
    "PositionOptions.rs",
    PositionOptions
);

web_feature!(
    "PreferenceManager",
    preference_manager,
    "PreferenceManager.rs",
    PreferenceManager
);

web_feature!(
    "PreferenceObject",
    preference_object,
    "PreferenceObject.rs",
    PreferenceObject
);

web_feature!(
    "Presentation",
    presentation,
    "Presentation.rs",
    Presentation
);

web_feature!(
    "PresentationAvailability",
    presentation_availability,
    "PresentationAvailability.rs",
    PresentationAvailability
);

web_feature!(
    "PresentationConnection",
    presentation_connection,
    "PresentationConnection.rs",
    PresentationConnection
);

web_feature!(
    "PresentationConnectionAvailableEvent",
    presentation_connection_available_event,
    "PresentationConnectionAvailableEvent.rs",
    PresentationConnectionAvailableEvent
);

web_feature!(
    "PresentationConnectionAvailableEventInit",
    presentation_connection_available_event_init,
    "PresentationConnectionAvailableEventInit.rs",
    PresentationConnectionAvailableEventInit
);

web_feature!(
    "PresentationConnectionCloseEvent",
    presentation_connection_close_event,
    "PresentationConnectionCloseEvent.rs",
    PresentationConnectionCloseEvent
);

web_feature!(
    "PresentationConnectionCloseEventInit",
    presentation_connection_close_event_init,
    "PresentationConnectionCloseEventInit.rs",
    PresentationConnectionCloseEventInit
);

web_feature!(
    "PresentationConnectionList",
    presentation_connection_list,
    "PresentationConnectionList.rs",
    PresentationConnectionList
);

web_feature!(
    "PresentationReceiver",
    presentation_receiver,
    "PresentationReceiver.rs",
    PresentationReceiver
);

web_feature!(
    "PresentationRequest",
    presentation_request,
    "PresentationRequest.rs",
    PresentationRequest
);

web_feature!(
    "PressureObserver",
    pressure_observer,
    "PressureObserver.rs",
    PressureObserver
);

web_feature!(
    "PressureObserverOptions",
    pressure_observer_options,
    "PressureObserverOptions.rs",
    PressureObserverOptions
);

web_feature!(
    "PressureRecord",
    pressure_record,
    "PressureRecord.rs",
    PressureRecord
);

web_feature!(
    "PrivateAggregation",
    private_aggregation,
    "PrivateAggregation.rs",
    PrivateAggregation
);

web_feature!(
    "PrivateNetworkAccessPermissionDescriptor",
    private_network_access_permission_descriptor,
    "PrivateNetworkAccessPermissionDescriptor.rs",
    PrivateNetworkAccessPermissionDescriptor
);

web_feature!(
    "PrivateToken",
    private_token,
    "PrivateToken.rs",
    PrivateToken
);

web_feature!(
    "ProcessingInstruction",
    processing_instruction,
    "ProcessingInstruction.rs",
    ProcessingInstruction
);

web_feature!("Profiler", profiler, "Profiler.rs", Profiler);

web_feature!(
    "ProfilerFrame",
    profiler_frame,
    "ProfilerFrame.rs",
    ProfilerFrame
);

web_feature!(
    "ProfilerInitOptions",
    profiler_init_options,
    "ProfilerInitOptions.rs",
    ProfilerInitOptions
);

web_feature!(
    "ProfilerSample",
    profiler_sample,
    "ProfilerSample.rs",
    ProfilerSample
);

web_feature!(
    "ProfilerStack",
    profiler_stack,
    "ProfilerStack.rs",
    ProfilerStack
);

web_feature!(
    "ProfilerTrace",
    profiler_trace,
    "ProfilerTrace.rs",
    ProfilerTrace
);

web_feature!(
    "ProgressEvent",
    progress_event,
    "ProgressEvent.rs",
    ProgressEvent
);

web_feature!(
    "ProgressEventInit",
    progress_event_init,
    "ProgressEventInit.rs",
    ProgressEventInit
);

web_feature!(
    "PromiseRejectionEvent",
    promise_rejection_event,
    "PromiseRejectionEvent.rs",
    PromiseRejectionEvent
);

web_feature!(
    "PromiseRejectionEventInit",
    promise_rejection_event_init,
    "PromiseRejectionEventInit.rs",
    PromiseRejectionEventInit
);

web_feature!(
    "PromptResponseObject",
    prompt_response_object,
    "PromptResponseObject.rs",
    PromptResponseObject
);

web_feature!(
    "PropertyDefinition",
    property_definition,
    "PropertyDefinition.rs",
    PropertyDefinition
);

web_feature!(
    "ProtectedAudience",
    protected_audience,
    "ProtectedAudience.rs",
    ProtectedAudience
);

web_feature!(
    "ProtectedAudiencePrivateAggregationConfig",
    protected_audience_private_aggregation_config,
    "ProtectedAudiencePrivateAggregationConfig.rs",
    ProtectedAudiencePrivateAggregationConfig
);

web_feature!(
    "ProtectedAudienceUtilities",
    protected_audience_utilities,
    "ProtectedAudienceUtilities.rs",
    ProtectedAudienceUtilities
);

web_feature!(
    "ProximitySensor",
    proximity_sensor,
    "ProximitySensor.rs",
    ProximitySensor
);

web_feature!(
    "PublicKeyCredential",
    public_key_credential,
    "PublicKeyCredential.rs",
    PublicKeyCredential
);

web_feature!(
    "PublicKeyCredentialCreationOptions",
    public_key_credential_creation_options,
    "PublicKeyCredentialCreationOptions.rs",
    PublicKeyCredentialCreationOptions
);

web_feature!(
    "PublicKeyCredentialCreationOptionsJSON",
    public_key_credential_creation_options_json,
    "PublicKeyCredentialCreationOptionsJSON.rs",
    PublicKeyCredentialCreationOptionsJSON
);

web_feature!(
    "PublicKeyCredentialDescriptor",
    public_key_credential_descriptor,
    "PublicKeyCredentialDescriptor.rs",
    PublicKeyCredentialDescriptor
);

web_feature!(
    "PublicKeyCredentialDescriptorJSON",
    public_key_credential_descriptor_json,
    "PublicKeyCredentialDescriptorJSON.rs",
    PublicKeyCredentialDescriptorJSON
);

web_feature!(
    "PublicKeyCredentialEntity",
    public_key_credential_entity,
    "PublicKeyCredentialEntity.rs",
    PublicKeyCredentialEntity
);

web_feature!(
    "PublicKeyCredentialParameters",
    public_key_credential_parameters,
    "PublicKeyCredentialParameters.rs",
    PublicKeyCredentialParameters
);

web_feature!(
    "PublicKeyCredentialRequestOptions",
    public_key_credential_request_options,
    "PublicKeyCredentialRequestOptions.rs",
    PublicKeyCredentialRequestOptions
);

web_feature!(
    "PublicKeyCredentialRequestOptionsJSON",
    public_key_credential_request_options_json,
    "PublicKeyCredentialRequestOptionsJSON.rs",
    PublicKeyCredentialRequestOptionsJSON
);

web_feature!(
    "PublicKeyCredentialRpEntity",
    public_key_credential_rp_entity,
    "PublicKeyCredentialRpEntity.rs",
    PublicKeyCredentialRpEntity
);

web_feature!(
    "PublicKeyCredentialUserEntity",
    public_key_credential_user_entity,
    "PublicKeyCredentialUserEntity.rs",
    PublicKeyCredentialUserEntity
);

web_feature!(
    "PublicKeyCredentialUserEntityJSON",
    public_key_credential_user_entity_json,
    "PublicKeyCredentialUserEntityJSON.rs",
    PublicKeyCredentialUserEntityJSON
);

web_feature!(
    "PurchaseDetails",
    purchase_details,
    "PurchaseDetails.rs",
    PurchaseDetails
);

web_feature!("PushEvent", push_event, "PushEvent.rs", PushEvent);

web_feature!(
    "PushEventInit",
    push_event_init,
    "PushEventInit.rs",
    PushEventInit
);

web_feature!("PushManager", push_manager, "PushManager.rs", PushManager);

web_feature!(
    "PushMessageData",
    push_message_data,
    "PushMessageData.rs",
    PushMessageData
);

web_feature!(
    "PushPermissionDescriptor",
    push_permission_descriptor,
    "PushPermissionDescriptor.rs",
    PushPermissionDescriptor
);

web_feature!(
    "PushSubscription",
    push_subscription,
    "PushSubscription.rs",
    PushSubscription
);

web_feature!(
    "PushSubscriptionChangeEvent",
    push_subscription_change_event,
    "PushSubscriptionChangeEvent.rs",
    PushSubscriptionChangeEvent
);

web_feature!(
    "PushSubscriptionChangeEventInit",
    push_subscription_change_event_init,
    "PushSubscriptionChangeEventInit.rs",
    PushSubscriptionChangeEventInit
);

web_feature!(
    "PushSubscriptionJSON",
    push_subscription_json,
    "PushSubscriptionJSON.rs",
    PushSubscriptionJSON
);

web_feature!(
    "PushSubscriptionOptions",
    push_subscription_options,
    "PushSubscriptionOptions.rs",
    PushSubscriptionOptions
);

web_feature!(
    "PushSubscriptionOptionsInit",
    push_subscription_options_init,
    "PushSubscriptionOptionsInit.rs",
    PushSubscriptionOptionsInit
);

web_feature!(
    "QueryOptions",
    query_options,
    "QueryOptions.rs",
    QueryOptions
);

web_feature!(
    "QueuingStrategy",
    queuing_strategy,
    "QueuingStrategy.rs",
    QueuingStrategy
);

web_feature!(
    "QueuingStrategyInit",
    queuing_strategy_init,
    "QueuingStrategyInit.rs",
    QueuingStrategyInit
);

web_feature!(
    "RTCAnswerOptions",
    rtc_answer_options,
    "RTCAnswerOptions.rs",
    RTCAnswerOptions
);

web_feature!(
    "RTCAudioPlayoutStats",
    rtc_audio_playout_stats,
    "RTCAudioPlayoutStats.rs",
    RTCAudioPlayoutStats
);

web_feature!(
    "RTCAudioSourceStats",
    rtc_audio_source_stats,
    "RTCAudioSourceStats.rs",
    RTCAudioSourceStats
);

web_feature!(
    "RTCCertificate",
    rtc_certificate,
    "RTCCertificate.rs",
    RTCCertificate
);

web_feature!(
    "RTCCertificateExpiration",
    rtc_certificate_expiration,
    "RTCCertificateExpiration.rs",
    RTCCertificateExpiration
);

web_feature!(
    "RTCCertificateStats",
    rtc_certificate_stats,
    "RTCCertificateStats.rs",
    RTCCertificateStats
);

web_feature!(
    "RTCCodecStats",
    rtc_codec_stats,
    "RTCCodecStats.rs",
    RTCCodecStats
);

web_feature!(
    "RTCConfiguration",
    rtc_configuration,
    "RTCConfiguration.rs",
    RTCConfiguration
);

web_feature!(
    "RTCDTMFSender",
    rtcdtmf_sender,
    "RTCDTMFSender.rs",
    RTCDTMFSender
);

web_feature!(
    "RTCDTMFToneChangeEvent",
    rtcdtmf_tone_change_event,
    "RTCDTMFToneChangeEvent.rs",
    RTCDTMFToneChangeEvent
);

web_feature!(
    "RTCDTMFToneChangeEventInit",
    rtcdtmf_tone_change_event_init,
    "RTCDTMFToneChangeEventInit.rs",
    RTCDTMFToneChangeEventInit
);

web_feature!(
    "RTCDataChannel",
    rtc_data_channel,
    "RTCDataChannel.rs",
    RTCDataChannel
);

web_feature!(
    "RTCDataChannelEvent",
    rtc_data_channel_event,
    "RTCDataChannelEvent.rs",
    RTCDataChannelEvent
);

web_feature!(
    "RTCDataChannelEventInit",
    rtc_data_channel_event_init,
    "RTCDataChannelEventInit.rs",
    RTCDataChannelEventInit
);

web_feature!(
    "RTCDataChannelInit",
    rtc_data_channel_init,
    "RTCDataChannelInit.rs",
    RTCDataChannelInit
);

web_feature!(
    "RTCDataChannelStats",
    rtc_data_channel_stats,
    "RTCDataChannelStats.rs",
    RTCDataChannelStats
);

web_feature!(
    "RTCDtlsFingerprint",
    rtc_dtls_fingerprint,
    "RTCDtlsFingerprint.rs",
    RTCDtlsFingerprint
);

web_feature!(
    "RTCDtlsTransport",
    rtc_dtls_transport,
    "RTCDtlsTransport.rs",
    RTCDtlsTransport
);

web_feature!(
    "RTCEncodedAudioFrame",
    rtc_encoded_audio_frame,
    "RTCEncodedAudioFrame.rs",
    RTCEncodedAudioFrame
);

web_feature!(
    "RTCEncodedAudioFrameMetadata",
    rtc_encoded_audio_frame_metadata,
    "RTCEncodedAudioFrameMetadata.rs",
    RTCEncodedAudioFrameMetadata
);

web_feature!(
    "RTCEncodedAudioFrameOptions",
    rtc_encoded_audio_frame_options,
    "RTCEncodedAudioFrameOptions.rs",
    RTCEncodedAudioFrameOptions
);

web_feature!(
    "RTCEncodedFrameMetadata",
    rtc_encoded_frame_metadata,
    "RTCEncodedFrameMetadata.rs",
    RTCEncodedFrameMetadata
);

web_feature!(
    "RTCEncodedVideoFrame",
    rtc_encoded_video_frame,
    "RTCEncodedVideoFrame.rs",
    RTCEncodedVideoFrame
);

web_feature!(
    "RTCEncodedVideoFrameMetadata",
    rtc_encoded_video_frame_metadata,
    "RTCEncodedVideoFrameMetadata.rs",
    RTCEncodedVideoFrameMetadata
);

web_feature!(
    "RTCEncodedVideoFrameOptions",
    rtc_encoded_video_frame_options,
    "RTCEncodedVideoFrameOptions.rs",
    RTCEncodedVideoFrameOptions
);

web_feature!("RTCError", rtc_error, "RTCError.rs", RTCError);

web_feature!(
    "RTCErrorEvent",
    rtc_error_event,
    "RTCErrorEvent.rs",
    RTCErrorEvent
);

web_feature!(
    "RTCErrorEventInit",
    rtc_error_event_init,
    "RTCErrorEventInit.rs",
    RTCErrorEventInit
);

web_feature!(
    "RTCErrorInit",
    rtc_error_init,
    "RTCErrorInit.rs",
    RTCErrorInit
);

web_feature!(
    "RTCIceCandidate",
    rtc_ice_candidate,
    "RTCIceCandidate.rs",
    RTCIceCandidate
);

web_feature!(
    "RTCIceCandidateInit",
    rtc_ice_candidate_init,
    "RTCIceCandidateInit.rs",
    RTCIceCandidateInit
);

web_feature!(
    "RTCIceCandidatePair",
    rtc_ice_candidate_pair,
    "RTCIceCandidatePair.rs",
    RTCIceCandidatePair
);

web_feature!(
    "RTCIceCandidatePairStats",
    rtc_ice_candidate_pair_stats,
    "RTCIceCandidatePairStats.rs",
    RTCIceCandidatePairStats
);

web_feature!(
    "RTCIceCandidateStats",
    rtc_ice_candidate_stats,
    "RTCIceCandidateStats.rs",
    RTCIceCandidateStats
);

web_feature!(
    "RTCIceGatherOptions",
    rtc_ice_gather_options,
    "RTCIceGatherOptions.rs",
    RTCIceGatherOptions
);

web_feature!(
    "RTCIceParameters",
    rtc_ice_parameters,
    "RTCIceParameters.rs",
    RTCIceParameters
);

web_feature!(
    "RTCIceServer",
    rtc_ice_server,
    "RTCIceServer.rs",
    RTCIceServer
);

web_feature!(
    "RTCIceTransport",
    rtc_ice_transport,
    "RTCIceTransport.rs",
    RTCIceTransport
);

web_feature!(
    "RTCIdentityAssertion",
    rtc_identity_assertion,
    "RTCIdentityAssertion.rs",
    RTCIdentityAssertion
);

web_feature!(
    "RTCIdentityAssertionResult",
    rtc_identity_assertion_result,
    "RTCIdentityAssertionResult.rs",
    RTCIdentityAssertionResult
);

web_feature!(
    "RTCIdentityProvider",
    rtc_identity_provider,
    "RTCIdentityProvider.rs",
    RTCIdentityProvider
);

web_feature!(
    "RTCIdentityProviderDetails",
    rtc_identity_provider_details,
    "RTCIdentityProviderDetails.rs",
    RTCIdentityProviderDetails
);

web_feature!(
    "RTCIdentityProviderGlobalScope",
    rtc_identity_provider_global_scope,
    "RTCIdentityProviderGlobalScope.rs",
    RTCIdentityProviderGlobalScope
);

web_feature!(
    "RTCIdentityProviderOptions",
    rtc_identity_provider_options,
    "RTCIdentityProviderOptions.rs",
    RTCIdentityProviderOptions
);

web_feature!(
    "RTCIdentityProviderRegistrar",
    rtc_identity_provider_registrar,
    "RTCIdentityProviderRegistrar.rs",
    RTCIdentityProviderRegistrar
);

web_feature!(
    "RTCIdentityValidationResult",
    rtc_identity_validation_result,
    "RTCIdentityValidationResult.rs",
    RTCIdentityValidationResult
);

web_feature!(
    "RTCInboundRtpStreamStats",
    rtc_inbound_rtp_stream_stats,
    "RTCInboundRtpStreamStats.rs",
    RTCInboundRtpStreamStats
);

web_feature!(
    "RTCLocalIceCandidateInit",
    rtc_local_ice_candidate_init,
    "RTCLocalIceCandidateInit.rs",
    RTCLocalIceCandidateInit
);

web_feature!(
    "RTCLocalSessionDescriptionInit",
    rtc_local_session_description_init,
    "RTCLocalSessionDescriptionInit.rs",
    RTCLocalSessionDescriptionInit
);

web_feature!(
    "RTCMediaSourceStats",
    rtc_media_source_stats,
    "RTCMediaSourceStats.rs",
    RTCMediaSourceStats
);

web_feature!(
    "RTCOfferAnswerOptions",
    rtc_offer_answer_options,
    "RTCOfferAnswerOptions.rs",
    RTCOfferAnswerOptions
);

web_feature!(
    "RTCOfferOptions",
    rtc_offer_options,
    "RTCOfferOptions.rs",
    RTCOfferOptions
);

web_feature!(
    "RTCOutboundRtpStreamStats",
    rtc_outbound_rtp_stream_stats,
    "RTCOutboundRtpStreamStats.rs",
    RTCOutboundRtpStreamStats
);

web_feature!(
    "RTCPeerConnection",
    rtc_peer_connection,
    "RTCPeerConnection.rs",
    RTCPeerConnection
);

web_feature!(
    "RTCPeerConnectionIceErrorEvent",
    rtc_peer_connection_ice_error_event,
    "RTCPeerConnectionIceErrorEvent.rs",
    RTCPeerConnectionIceErrorEvent
);

web_feature!(
    "RTCPeerConnectionIceErrorEventInit",
    rtc_peer_connection_ice_error_event_init,
    "RTCPeerConnectionIceErrorEventInit.rs",
    RTCPeerConnectionIceErrorEventInit
);

web_feature!(
    "RTCPeerConnectionIceEvent",
    rtc_peer_connection_ice_event,
    "RTCPeerConnectionIceEvent.rs",
    RTCPeerConnectionIceEvent
);

web_feature!(
    "RTCPeerConnectionIceEventInit",
    rtc_peer_connection_ice_event_init,
    "RTCPeerConnectionIceEventInit.rs",
    RTCPeerConnectionIceEventInit
);

web_feature!(
    "RTCPeerConnectionStats",
    rtc_peer_connection_stats,
    "RTCPeerConnectionStats.rs",
    RTCPeerConnectionStats
);

web_feature!(
    "RTCReceivedRtpStreamStats",
    rtc_received_rtp_stream_stats,
    "RTCReceivedRtpStreamStats.rs",
    RTCReceivedRtpStreamStats
);

web_feature!(
    "RTCRemoteInboundRtpStreamStats",
    rtc_remote_inbound_rtp_stream_stats,
    "RTCRemoteInboundRtpStreamStats.rs",
    RTCRemoteInboundRtpStreamStats
);

web_feature!(
    "RTCRemoteOutboundRtpStreamStats",
    rtc_remote_outbound_rtp_stream_stats,
    "RTCRemoteOutboundRtpStreamStats.rs",
    RTCRemoteOutboundRtpStreamStats
);

web_feature!(
    "RTCRtcpParameters",
    rtc_rtcp_parameters,
    "RTCRtcpParameters.rs",
    RTCRtcpParameters
);

web_feature!(
    "RTCRtpCapabilities",
    rtc_rtp_capabilities,
    "RTCRtpCapabilities.rs",
    RTCRtpCapabilities
);

web_feature!("RTCRtpCodec", rtc_rtp_codec, "RTCRtpCodec.rs", RTCRtpCodec);

web_feature!(
    "RTCRtpCodecParameters",
    rtc_rtp_codec_parameters,
    "RTCRtpCodecParameters.rs",
    RTCRtpCodecParameters
);

web_feature!(
    "RTCRtpCodingParameters",
    rtc_rtp_coding_parameters,
    "RTCRtpCodingParameters.rs",
    RTCRtpCodingParameters
);

web_feature!(
    "RTCRtpContributingSource",
    rtc_rtp_contributing_source,
    "RTCRtpContributingSource.rs",
    RTCRtpContributingSource
);

web_feature!(
    "RTCRtpEncodingParameters",
    rtc_rtp_encoding_parameters,
    "RTCRtpEncodingParameters.rs",
    RTCRtpEncodingParameters
);

web_feature!(
    "RTCRtpHeaderExtensionCapability",
    rtc_rtp_header_extension_capability,
    "RTCRtpHeaderExtensionCapability.rs",
    RTCRtpHeaderExtensionCapability
);

web_feature!(
    "RTCRtpHeaderExtensionParameters",
    rtc_rtp_header_extension_parameters,
    "RTCRtpHeaderExtensionParameters.rs",
    RTCRtpHeaderExtensionParameters
);

web_feature!(
    "RTCRtpParameters",
    rtc_rtp_parameters,
    "RTCRtpParameters.rs",
    RTCRtpParameters
);

web_feature!(
    "RTCRtpReceiveParameters",
    rtc_rtp_receive_parameters,
    "RTCRtpReceiveParameters.rs",
    RTCRtpReceiveParameters
);

web_feature!(
    "RTCRtpReceiver",
    rtc_rtp_receiver,
    "RTCRtpReceiver.rs",
    RTCRtpReceiver
);

web_feature!(
    "RTCRtpScriptTransform",
    rtc_rtp_script_transform,
    "RTCRtpScriptTransform.rs",
    RTCRtpScriptTransform
);

web_feature!(
    "RTCRtpScriptTransformer",
    rtc_rtp_script_transformer,
    "RTCRtpScriptTransformer.rs",
    RTCRtpScriptTransformer
);

web_feature!(
    "RTCRtpSendParameters",
    rtc_rtp_send_parameters,
    "RTCRtpSendParameters.rs",
    RTCRtpSendParameters
);

web_feature!(
    "RTCRtpSender",
    rtc_rtp_sender,
    "RTCRtpSender.rs",
    RTCRtpSender
);

web_feature!(
    "RTCRtpStreamStats",
    rtc_rtp_stream_stats,
    "RTCRtpStreamStats.rs",
    RTCRtpStreamStats
);

web_feature!(
    "RTCRtpSynchronizationSource",
    rtc_rtp_synchronization_source,
    "RTCRtpSynchronizationSource.rs",
    RTCRtpSynchronizationSource
);

web_feature!(
    "RTCRtpTransceiver",
    rtc_rtp_transceiver,
    "RTCRtpTransceiver.rs",
    RTCRtpTransceiver
);

web_feature!(
    "RTCRtpTransceiverInit",
    rtc_rtp_transceiver_init,
    "RTCRtpTransceiverInit.rs",
    RTCRtpTransceiverInit
);

web_feature!(
    "RTCSctpTransport",
    rtc_sctp_transport,
    "RTCSctpTransport.rs",
    RTCSctpTransport
);

web_feature!(
    "RTCSentRtpStreamStats",
    rtc_sent_rtp_stream_stats,
    "RTCSentRtpStreamStats.rs",
    RTCSentRtpStreamStats
);

web_feature!(
    "RTCSessionDescription",
    rtc_session_description,
    "RTCSessionDescription.rs",
    RTCSessionDescription
);

web_feature!(
    "RTCSessionDescriptionInit",
    rtc_session_description_init,
    "RTCSessionDescriptionInit.rs",
    RTCSessionDescriptionInit
);

web_feature!(
    "RTCSetParameterOptions",
    rtc_set_parameter_options,
    "RTCSetParameterOptions.rs",
    RTCSetParameterOptions
);

web_feature!("RTCStats", rtc_stats, "RTCStats.rs", RTCStats);

web_feature!(
    "RTCStatsReport",
    rtc_stats_report,
    "RTCStatsReport.rs",
    RTCStatsReport
);

web_feature!(
    "RTCTrackEvent",
    rtc_track_event,
    "RTCTrackEvent.rs",
    RTCTrackEvent
);

web_feature!(
    "RTCTrackEventInit",
    rtc_track_event_init,
    "RTCTrackEventInit.rs",
    RTCTrackEventInit
);

web_feature!(
    "RTCTransformEvent",
    rtc_transform_event,
    "RTCTransformEvent.rs",
    RTCTransformEvent
);

web_feature!(
    "RTCTransportStats",
    rtc_transport_stats,
    "RTCTransportStats.rs",
    RTCTransportStats
);

web_feature!(
    "RTCVideoSourceStats",
    rtc_video_source_stats,
    "RTCVideoSourceStats.rs",
    RTCVideoSourceStats
);

web_feature!(
    "RadioNodeList",
    radio_node_list,
    "RadioNodeList.rs",
    RadioNodeList
);

web_feature!("Range", range, "Range.rs", Range);

web_feature!("ReadOptions", read_options, "ReadOptions.rs", ReadOptions);

web_feature!(
    "ReadableByteStreamController",
    readable_byte_stream_controller,
    "ReadableByteStreamController.rs",
    ReadableByteStreamController
);

web_feature!(
    "ReadableStream",
    readable_stream,
    "ReadableStream.rs",
    ReadableStream
);

web_feature!(
    "ReadableStreamBYOBReader",
    readable_stream_byob_reader,
    "ReadableStreamBYOBReader.rs",
    ReadableStreamBYOBReader
);

web_feature!(
    "ReadableStreamBYOBReaderReadOptions",
    readable_stream_byob_reader_read_options,
    "ReadableStreamBYOBReaderReadOptions.rs",
    ReadableStreamBYOBReaderReadOptions
);

web_feature!(
    "ReadableStreamBYOBRequest",
    readable_stream_byob_request,
    "ReadableStreamBYOBRequest.rs",
    ReadableStreamBYOBRequest
);

web_feature!(
    "ReadableStreamDefaultController",
    readable_stream_default_controller,
    "ReadableStreamDefaultController.rs",
    ReadableStreamDefaultController
);

web_feature!(
    "ReadableStreamDefaultReader",
    readable_stream_default_reader,
    "ReadableStreamDefaultReader.rs",
    ReadableStreamDefaultReader
);

web_feature!(
    "ReadableStreamGetReaderOptions",
    readable_stream_get_reader_options,
    "ReadableStreamGetReaderOptions.rs",
    ReadableStreamGetReaderOptions
);

web_feature!(
    "ReadableStreamIteratorOptions",
    readable_stream_iterator_options,
    "ReadableStreamIteratorOptions.rs",
    ReadableStreamIteratorOptions
);

web_feature!(
    "ReadableStreamReadResult",
    readable_stream_read_result,
    "ReadableStreamReadResult.rs",
    ReadableStreamReadResult
);

web_feature!(
    "ReadableWritablePair",
    readable_writable_pair,
    "ReadableWritablePair.rs",
    ReadableWritablePair
);

web_feature!(
    "RealTimeContribution",
    real_time_contribution,
    "RealTimeContribution.rs",
    RealTimeContribution
);

web_feature!(
    "RealTimeReporting",
    real_time_reporting,
    "RealTimeReporting.rs",
    RealTimeReporting
);

web_feature!(
    "RegistrationOptions",
    registration_options,
    "RegistrationOptions.rs",
    RegistrationOptions
);

web_feature!(
    "RegistrationResponseJSON",
    registration_response_json,
    "RegistrationResponseJSON.rs",
    RegistrationResponseJSON
);

web_feature!(
    "RelatedApplication",
    related_application,
    "RelatedApplication.rs",
    RelatedApplication
);

web_feature!(
    "RelativeOrientationSensor",
    relative_orientation_sensor,
    "RelativeOrientationSensor.rs",
    RelativeOrientationSensor
);

web_feature!(
    "RemotePlayback",
    remote_playback,
    "RemotePlayback.rs",
    RemotePlayback
);

web_feature!("Report", report, "Report.rs", Report);

web_feature!("ReportBody", report_body, "ReportBody.rs", ReportBody);

web_feature!(
    "ReportResultBrowserSignals",
    report_result_browser_signals,
    "ReportResultBrowserSignals.rs",
    ReportResultBrowserSignals
);

web_feature!(
    "ReportWinBrowserSignals",
    report_win_browser_signals,
    "ReportWinBrowserSignals.rs",
    ReportWinBrowserSignals
);

web_feature!(
    "ReportingBrowserSignals",
    reporting_browser_signals,
    "ReportingBrowserSignals.rs",
    ReportingBrowserSignals
);

web_feature!(
    "ReportingObserver",
    reporting_observer,
    "ReportingObserver.rs",
    ReportingObserver
);

web_feature!(
    "ReportingObserverOptions",
    reporting_observer_options,
    "ReportingObserverOptions.rs",
    ReportingObserverOptions
);

web_feature!("Request", request, "Request.rs", Request);

web_feature!(
    "RequestDeviceOptions",
    request_device_options,
    "RequestDeviceOptions.rs",
    RequestDeviceOptions
);

web_feature!("RequestInit", request_init, "RequestInit.rs", RequestInit);

web_feature!(
    "ResizeObserver",
    resize_observer,
    "ResizeObserver.rs",
    ResizeObserver
);

web_feature!(
    "ResizeObserverEntry",
    resize_observer_entry,
    "ResizeObserverEntry.rs",
    ResizeObserverEntry
);

web_feature!(
    "ResizeObserverOptions",
    resize_observer_options,
    "ResizeObserverOptions.rs",
    ResizeObserverOptions
);

web_feature!(
    "ResizeObserverSize",
    resize_observer_size,
    "ResizeObserverSize.rs",
    ResizeObserverSize
);

web_feature!("Response", response, "Response.rs", Response);

web_feature!(
    "ResponseInit",
    response_init,
    "ResponseInit.rs",
    ResponseInit
);

web_feature!(
    "RestrictionTarget",
    restriction_target,
    "RestrictionTarget.rs",
    RestrictionTarget
);

web_feature!("Rewriter", rewriter, "Rewriter.rs", Rewriter);

web_feature!(
    "RewriterCreateCoreOptions",
    rewriter_create_core_options,
    "RewriterCreateCoreOptions.rs",
    RewriterCreateCoreOptions
);

web_feature!(
    "RewriterCreateOptions",
    rewriter_create_options,
    "RewriterCreateOptions.rs",
    RewriterCreateOptions
);

web_feature!(
    "RewriterRewriteOptions",
    rewriter_rewrite_options,
    "RewriterRewriteOptions.rs",
    RewriterRewriteOptions
);

web_feature!(
    "RouterCondition",
    router_condition,
    "RouterCondition.rs",
    RouterCondition
);

web_feature!("RouterRule", router_rule, "RouterRule.rs", RouterRule);

web_feature!(
    "RouterSourceDict",
    router_source_dict,
    "RouterSourceDict.rs",
    RouterSourceDict
);

web_feature!(
    "RsaHashedImportParams",
    rsa_hashed_import_params,
    "RsaHashedImportParams.rs",
    RsaHashedImportParams
);

web_feature!(
    "RsaHashedKeyAlgorithm",
    rsa_hashed_key_algorithm,
    "RsaHashedKeyAlgorithm.rs",
    RsaHashedKeyAlgorithm
);

web_feature!(
    "RsaHashedKeyGenParams",
    rsa_hashed_key_gen_params,
    "RsaHashedKeyGenParams.rs",
    RsaHashedKeyGenParams
);

web_feature!(
    "RsaKeyAlgorithm",
    rsa_key_algorithm,
    "RsaKeyAlgorithm.rs",
    RsaKeyAlgorithm
);

web_feature!(
    "RsaKeyGenParams",
    rsa_key_gen_params,
    "RsaKeyGenParams.rs",
    RsaKeyGenParams
);

web_feature!(
    "RsaOaepParams",
    rsa_oaep_params,
    "RsaOaepParams.rs",
    RsaOaepParams
);

web_feature!(
    "RsaOtherPrimesInfo",
    rsa_other_primes_info,
    "RsaOtherPrimesInfo.rs",
    RsaOtherPrimesInfo
);

web_feature!(
    "RsaPssParams",
    rsa_pss_params,
    "RsaPssParams.rs",
    RsaPssParams
);

web_feature!(
    "SFrameTransform",
    s_frame_transform,
    "SFrameTransform.rs",
    SFrameTransform
);

web_feature!(
    "SFrameTransformErrorEvent",
    s_frame_transform_error_event,
    "SFrameTransformErrorEvent.rs",
    SFrameTransformErrorEvent
);

web_feature!(
    "SFrameTransformErrorEventInit",
    s_frame_transform_error_event_init,
    "SFrameTransformErrorEventInit.rs",
    SFrameTransformErrorEventInit
);

web_feature!(
    "SFrameTransformOptions",
    s_frame_transform_options,
    "SFrameTransformOptions.rs",
    SFrameTransformOptions
);

web_feature!("SVGAElement", svga_element, "SVGAElement.rs", SVGAElement);

web_feature!("SVGAngle", svg_angle, "SVGAngle.rs", SVGAngle);

web_feature!(
    "SVGAnimateElement",
    svg_animate_element,
    "SVGAnimateElement.rs",
    SVGAnimateElement
);

web_feature!(
    "SVGAnimateMotionElement",
    svg_animate_motion_element,
    "SVGAnimateMotionElement.rs",
    SVGAnimateMotionElement
);

web_feature!(
    "SVGAnimateTransformElement",
    svg_animate_transform_element,
    "SVGAnimateTransformElement.rs",
    SVGAnimateTransformElement
);

web_feature!(
    "SVGAnimatedAngle",
    svg_animated_angle,
    "SVGAnimatedAngle.rs",
    SVGAnimatedAngle
);

web_feature!(
    "SVGAnimatedBoolean",
    svg_animated_boolean,
    "SVGAnimatedBoolean.rs",
    SVGAnimatedBoolean
);

web_feature!(
    "SVGAnimatedEnumeration",
    svg_animated_enumeration,
    "SVGAnimatedEnumeration.rs",
    SVGAnimatedEnumeration
);

web_feature!(
    "SVGAnimatedInteger",
    svg_animated_integer,
    "SVGAnimatedInteger.rs",
    SVGAnimatedInteger
);

web_feature!(
    "SVGAnimatedLength",
    svg_animated_length,
    "SVGAnimatedLength.rs",
    SVGAnimatedLength
);

web_feature!(
    "SVGAnimatedLengthList",
    svg_animated_length_list,
    "SVGAnimatedLengthList.rs",
    SVGAnimatedLengthList
);

web_feature!(
    "SVGAnimatedNumber",
    svg_animated_number,
    "SVGAnimatedNumber.rs",
    SVGAnimatedNumber
);

web_feature!(
    "SVGAnimatedNumberList",
    svg_animated_number_list,
    "SVGAnimatedNumberList.rs",
    SVGAnimatedNumberList
);

web_feature!(
    "SVGAnimatedPreserveAspectRatio",
    svg_animated_preserve_aspect_ratio,
    "SVGAnimatedPreserveAspectRatio.rs",
    SVGAnimatedPreserveAspectRatio
);

web_feature!(
    "SVGAnimatedRect",
    svg_animated_rect,
    "SVGAnimatedRect.rs",
    SVGAnimatedRect
);

web_feature!(
    "SVGAnimatedString",
    svg_animated_string,
    "SVGAnimatedString.rs",
    SVGAnimatedString
);

web_feature!(
    "SVGAnimatedTransformList",
    svg_animated_transform_list,
    "SVGAnimatedTransformList.rs",
    SVGAnimatedTransformList
);

web_feature!(
    "SVGAnimationElement",
    svg_animation_element,
    "SVGAnimationElement.rs",
    SVGAnimationElement
);

web_feature!(
    "SVGBoundingBoxOptions",
    svg_bounding_box_options,
    "SVGBoundingBoxOptions.rs",
    SVGBoundingBoxOptions
);

web_feature!(
    "SVGCircleElement",
    svg_circle_element,
    "SVGCircleElement.rs",
    SVGCircleElement
);

web_feature!(
    "SVGClipPathElement",
    svg_clip_path_element,
    "SVGClipPathElement.rs",
    SVGClipPathElement
);

web_feature!(
    "SVGComponentTransferFunctionElement",
    svg_component_transfer_function_element,
    "SVGComponentTransferFunctionElement.rs",
    SVGComponentTransferFunctionElement
);

web_feature!(
    "SVGDefsElement",
    svg_defs_element,
    "SVGDefsElement.rs",
    SVGDefsElement
);

web_feature!(
    "SVGDescElement",
    svg_desc_element,
    "SVGDescElement.rs",
    SVGDescElement
);

web_feature!(
    "SVGDiscardElement",
    svg_discard_element,
    "SVGDiscardElement.rs",
    SVGDiscardElement
);

web_feature!("SVGElement", svg_element, "SVGElement.rs", SVGElement);

web_feature!(
    "SVGEllipseElement",
    svg_ellipse_element,
    "SVGEllipseElement.rs",
    SVGEllipseElement
);

web_feature!(
    "SVGFEBlendElement",
    svgfe_blend_element,
    "SVGFEBlendElement.rs",
    SVGFEBlendElement
);

web_feature!(
    "SVGFEColorMatrixElement",
    svgfe_color_matrix_element,
    "SVGFEColorMatrixElement.rs",
    SVGFEColorMatrixElement
);

web_feature!(
    "SVGFEComponentTransferElement",
    svgfe_component_transfer_element,
    "SVGFEComponentTransferElement.rs",
    SVGFEComponentTransferElement
);

web_feature!(
    "SVGFECompositeElement",
    svgfe_composite_element,
    "SVGFECompositeElement.rs",
    SVGFECompositeElement
);

web_feature!(
    "SVGFEConvolveMatrixElement",
    svgfe_convolve_matrix_element,
    "SVGFEConvolveMatrixElement.rs",
    SVGFEConvolveMatrixElement
);

web_feature!(
    "SVGFEDiffuseLightingElement",
    svgfe_diffuse_lighting_element,
    "SVGFEDiffuseLightingElement.rs",
    SVGFEDiffuseLightingElement
);

web_feature!(
    "SVGFEDisplacementMapElement",
    svgfe_displacement_map_element,
    "SVGFEDisplacementMapElement.rs",
    SVGFEDisplacementMapElement
);

web_feature!(
    "SVGFEDistantLightElement",
    svgfe_distant_light_element,
    "SVGFEDistantLightElement.rs",
    SVGFEDistantLightElement
);

web_feature!(
    "SVGFEDropShadowElement",
    svgfe_drop_shadow_element,
    "SVGFEDropShadowElement.rs",
    SVGFEDropShadowElement
);

web_feature!(
    "SVGFEFloodElement",
    svgfe_flood_element,
    "SVGFEFloodElement.rs",
    SVGFEFloodElement
);

web_feature!(
    "SVGFEFuncAElement",
    svgfe_func_a_element,
    "SVGFEFuncAElement.rs",
    SVGFEFuncAElement
);

web_feature!(
    "SVGFEFuncBElement",
    svgfe_func_b_element,
    "SVGFEFuncBElement.rs",
    SVGFEFuncBElement
);

web_feature!(
    "SVGFEFuncGElement",
    svgfe_func_g_element,
    "SVGFEFuncGElement.rs",
    SVGFEFuncGElement
);

web_feature!(
    "SVGFEFuncRElement",
    svgfe_func_r_element,
    "SVGFEFuncRElement.rs",
    SVGFEFuncRElement
);

web_feature!(
    "SVGFEGaussianBlurElement",
    svgfe_gaussian_blur_element,
    "SVGFEGaussianBlurElement.rs",
    SVGFEGaussianBlurElement
);

web_feature!(
    "SVGFEImageElement",
    svgfe_image_element,
    "SVGFEImageElement.rs",
    SVGFEImageElement
);

web_feature!(
    "SVGFEMergeElement",
    svgfe_merge_element,
    "SVGFEMergeElement.rs",
    SVGFEMergeElement
);

web_feature!(
    "SVGFEMergeNodeElement",
    svgfe_merge_node_element,
    "SVGFEMergeNodeElement.rs",
    SVGFEMergeNodeElement
);

web_feature!(
    "SVGFEMorphologyElement",
    svgfe_morphology_element,
    "SVGFEMorphologyElement.rs",
    SVGFEMorphologyElement
);

web_feature!(
    "SVGFEOffsetElement",
    svgfe_offset_element,
    "SVGFEOffsetElement.rs",
    SVGFEOffsetElement
);

web_feature!(
    "SVGFEPointLightElement",
    svgfe_point_light_element,
    "SVGFEPointLightElement.rs",
    SVGFEPointLightElement
);

web_feature!(
    "SVGFESpecularLightingElement",
    svgfe_specular_lighting_element,
    "SVGFESpecularLightingElement.rs",
    SVGFESpecularLightingElement
);

web_feature!(
    "SVGFESpotLightElement",
    svgfe_spot_light_element,
    "SVGFESpotLightElement.rs",
    SVGFESpotLightElement
);

web_feature!(
    "SVGFETileElement",
    svgfe_tile_element,
    "SVGFETileElement.rs",
    SVGFETileElement
);

web_feature!(
    "SVGFETurbulenceElement",
    svgfe_turbulence_element,
    "SVGFETurbulenceElement.rs",
    SVGFETurbulenceElement
);

web_feature!(
    "SVGFilterElement",
    svg_filter_element,
    "SVGFilterElement.rs",
    SVGFilterElement
);

web_feature!(
    "SVGForeignObjectElement",
    svg_foreign_object_element,
    "SVGForeignObjectElement.rs",
    SVGForeignObjectElement
);

web_feature!("SVGGElement", svgg_element, "SVGGElement.rs", SVGGElement);

web_feature!(
    "SVGGeometryElement",
    svg_geometry_element,
    "SVGGeometryElement.rs",
    SVGGeometryElement
);

web_feature!(
    "SVGGradientElement",
    svg_gradient_element,
    "SVGGradientElement.rs",
    SVGGradientElement
);

web_feature!(
    "SVGGraphicsElement",
    svg_graphics_element,
    "SVGGraphicsElement.rs",
    SVGGraphicsElement
);

web_feature!(
    "SVGImageElement",
    svg_image_element,
    "SVGImageElement.rs",
    SVGImageElement
);

web_feature!("SVGLength", svg_length, "SVGLength.rs", SVGLength);

web_feature!(
    "SVGLengthList",
    svg_length_list,
    "SVGLengthList.rs",
    SVGLengthList
);

web_feature!(
    "SVGLineElement",
    svg_line_element,
    "SVGLineElement.rs",
    SVGLineElement
);

web_feature!(
    "SVGLinearGradientElement",
    svg_linear_gradient_element,
    "SVGLinearGradientElement.rs",
    SVGLinearGradientElement
);

web_feature!(
    "SVGMPathElement",
    svgm_path_element,
    "SVGMPathElement.rs",
    SVGMPathElement
);

web_feature!(
    "SVGMarkerElement",
    svg_marker_element,
    "SVGMarkerElement.rs",
    SVGMarkerElement
);

web_feature!(
    "SVGMaskElement",
    svg_mask_element,
    "SVGMaskElement.rs",
    SVGMaskElement
);

web_feature!(
    "SVGMetadataElement",
    svg_metadata_element,
    "SVGMetadataElement.rs",
    SVGMetadataElement
);

web_feature!("SVGNumber", svg_number, "SVGNumber.rs", SVGNumber);

web_feature!(
    "SVGNumberList",
    svg_number_list,
    "SVGNumberList.rs",
    SVGNumberList
);

web_feature!(
    "SVGPathDataSettings",
    svg_path_data_settings,
    "SVGPathDataSettings.rs",
    SVGPathDataSettings
);

web_feature!(
    "SVGPathElement",
    svg_path_element,
    "SVGPathElement.rs",
    SVGPathElement
);

web_feature!(
    "SVGPathSegment",
    svg_path_segment,
    "SVGPathSegment.rs",
    SVGPathSegment
);

web_feature!(
    "SVGPatternElement",
    svg_pattern_element,
    "SVGPatternElement.rs",
    SVGPatternElement
);

web_feature!(
    "SVGPointList",
    svg_point_list,
    "SVGPointList.rs",
    SVGPointList
);

web_feature!(
    "SVGPolygonElement",
    svg_polygon_element,
    "SVGPolygonElement.rs",
    SVGPolygonElement
);

web_feature!(
    "SVGPolylineElement",
    svg_polyline_element,
    "SVGPolylineElement.rs",
    SVGPolylineElement
);

web_feature!(
    "SVGPreserveAspectRatio",
    svg_preserve_aspect_ratio,
    "SVGPreserveAspectRatio.rs",
    SVGPreserveAspectRatio
);

web_feature!(
    "SVGRadialGradientElement",
    svg_radial_gradient_element,
    "SVGRadialGradientElement.rs",
    SVGRadialGradientElement
);

web_feature!(
    "SVGRectElement",
    svg_rect_element,
    "SVGRectElement.rs",
    SVGRectElement
);

web_feature!(
    "SVGSVGElement",
    svgsvg_element,
    "SVGSVGElement.rs",
    SVGSVGElement
);

web_feature!(
    "SVGScriptElement",
    svg_script_element,
    "SVGScriptElement.rs",
    SVGScriptElement
);

web_feature!(
    "SVGSetElement",
    svg_set_element,
    "SVGSetElement.rs",
    SVGSetElement
);

web_feature!(
    "SVGStopElement",
    svg_stop_element,
    "SVGStopElement.rs",
    SVGStopElement
);

web_feature!(
    "SVGStringList",
    svg_string_list,
    "SVGStringList.rs",
    SVGStringList
);

web_feature!(
    "SVGStyleElement",
    svg_style_element,
    "SVGStyleElement.rs",
    SVGStyleElement
);

web_feature!(
    "SVGSwitchElement",
    svg_switch_element,
    "SVGSwitchElement.rs",
    SVGSwitchElement
);

web_feature!(
    "SVGSymbolElement",
    svg_symbol_element,
    "SVGSymbolElement.rs",
    SVGSymbolElement
);

web_feature!(
    "SVGTSpanElement",
    svgt_span_element,
    "SVGTSpanElement.rs",
    SVGTSpanElement
);

web_feature!(
    "SVGTextContentElement",
    svg_text_content_element,
    "SVGTextContentElement.rs",
    SVGTextContentElement
);

web_feature!(
    "SVGTextElement",
    svg_text_element,
    "SVGTextElement.rs",
    SVGTextElement
);

web_feature!(
    "SVGTextPathElement",
    svg_text_path_element,
    "SVGTextPathElement.rs",
    SVGTextPathElement
);

web_feature!(
    "SVGTextPositioningElement",
    svg_text_positioning_element,
    "SVGTextPositioningElement.rs",
    SVGTextPositioningElement
);

web_feature!(
    "SVGTitleElement",
    svg_title_element,
    "SVGTitleElement.rs",
    SVGTitleElement
);

web_feature!(
    "SVGTransform",
    svg_transform,
    "SVGTransform.rs",
    SVGTransform
);

web_feature!(
    "SVGTransformList",
    svg_transform_list,
    "SVGTransformList.rs",
    SVGTransformList
);

web_feature!(
    "SVGUnitTypes",
    svg_unit_types,
    "SVGUnitTypes.rs",
    SVGUnitTypes
);

web_feature!(
    "SVGUseElement",
    svg_use_element,
    "SVGUseElement.rs",
    SVGUseElement
);

web_feature!(
    "SVGUseElementShadowRoot",
    svg_use_element_shadow_root,
    "SVGUseElementShadowRoot.rs",
    SVGUseElementShadowRoot
);

web_feature!(
    "SVGViewElement",
    svg_view_element,
    "SVGViewElement.rs",
    SVGViewElement
);

web_feature!("Sanitizer", sanitizer, "Sanitizer.rs", Sanitizer);

web_feature!(
    "SanitizerAttributeNamespace",
    sanitizer_attribute_namespace,
    "SanitizerAttributeNamespace.rs",
    SanitizerAttributeNamespace
);

web_feature!(
    "SanitizerConfig",
    sanitizer_config,
    "SanitizerConfig.rs",
    SanitizerConfig
);

web_feature!(
    "SanitizerElementNamespace",
    sanitizer_element_namespace,
    "SanitizerElementNamespace.rs",
    SanitizerElementNamespace
);

web_feature!(
    "SanitizerElementNamespaceWithAttributes",
    sanitizer_element_namespace_with_attributes,
    "SanitizerElementNamespaceWithAttributes.rs",
    SanitizerElementNamespaceWithAttributes
);

web_feature!(
    "SaveFilePickerOptions",
    save_file_picker_options,
    "SaveFilePickerOptions.rs",
    SaveFilePickerOptions
);

web_feature!("Scheduler", scheduler, "Scheduler.rs", Scheduler);

web_feature!(
    "SchedulerPostTaskOptions",
    scheduler_post_task_options,
    "SchedulerPostTaskOptions.rs",
    SchedulerPostTaskOptions
);

web_feature!("Scheduling", scheduling, "Scheduling.rs", Scheduling);

web_feature!(
    "ScoreAdOutput",
    score_ad_output,
    "ScoreAdOutput.rs",
    ScoreAdOutput
);

web_feature!(
    "ScoringBrowserSignals",
    scoring_browser_signals,
    "ScoringBrowserSignals.rs",
    ScoringBrowserSignals
);

web_feature!("Screen", screen, "Screen.rs", Screen);

web_feature!(
    "ScreenDetailed",
    screen_detailed,
    "ScreenDetailed.rs",
    ScreenDetailed
);

web_feature!(
    "ScreenDetails",
    screen_details,
    "ScreenDetails.rs",
    ScreenDetails
);

web_feature!(
    "ScreenOrientation",
    screen_orientation,
    "ScreenOrientation.rs",
    ScreenOrientation
);

web_feature!(
    "ScriptProcessorNode",
    script_processor_node,
    "ScriptProcessorNode.rs",
    ScriptProcessorNode
);

web_feature!(
    "ScriptingPolicyReportBody",
    scripting_policy_report_body,
    "ScriptingPolicyReportBody.rs",
    ScriptingPolicyReportBody
);

web_feature!(
    "ScrollIntoViewOptions",
    scroll_into_view_options,
    "ScrollIntoViewOptions.rs",
    ScrollIntoViewOptions
);

web_feature!(
    "ScrollOptions",
    scroll_options,
    "ScrollOptions.rs",
    ScrollOptions
);

web_feature!(
    "ScrollTimeline",
    scroll_timeline,
    "ScrollTimeline.rs",
    ScrollTimeline
);

web_feature!(
    "ScrollTimelineOptions",
    scroll_timeline_options,
    "ScrollTimelineOptions.rs",
    ScrollTimelineOptions
);

web_feature!(
    "ScrollToOptions",
    scroll_to_options,
    "ScrollToOptions.rs",
    ScrollToOptions
);

web_feature!(
    "SecurePaymentConfirmationRequest",
    secure_payment_confirmation_request,
    "SecurePaymentConfirmationRequest.rs",
    SecurePaymentConfirmationRequest
);

web_feature!(
    "SecurityPolicyViolationEvent",
    security_policy_violation_event,
    "SecurityPolicyViolationEvent.rs",
    SecurityPolicyViolationEvent
);

web_feature!(
    "SecurityPolicyViolationEventInit",
    security_policy_violation_event_init,
    "SecurityPolicyViolationEventInit.rs",
    SecurityPolicyViolationEventInit
);

web_feature!("Selection", selection, "Selection.rs", Selection);

web_feature!("Sensor", sensor, "Sensor.rs", Sensor);

web_feature!(
    "SensorErrorEvent",
    sensor_error_event,
    "SensorErrorEvent.rs",
    SensorErrorEvent
);

web_feature!(
    "SensorErrorEventInit",
    sensor_error_event_init,
    "SensorErrorEventInit.rs",
    SensorErrorEventInit
);

web_feature!(
    "SensorOptions",
    sensor_options,
    "SensorOptions.rs",
    SensorOptions
);

web_feature!(
    "SequenceEffect",
    sequence_effect,
    "SequenceEffect.rs",
    SequenceEffect
);

web_feature!("Serial", serial, "Serial.rs", Serial);

web_feature!(
    "SerialInputSignals",
    serial_input_signals,
    "SerialInputSignals.rs",
    SerialInputSignals
);

web_feature!(
    "SerialOptions",
    serial_options,
    "SerialOptions.rs",
    SerialOptions
);

web_feature!(
    "SerialOutputSignals",
    serial_output_signals,
    "SerialOutputSignals.rs",
    SerialOutputSignals
);

web_feature!("SerialPort", serial_port, "SerialPort.rs", SerialPort);

web_feature!(
    "SerialPortFilter",
    serial_port_filter,
    "SerialPortFilter.rs",
    SerialPortFilter
);

web_feature!(
    "SerialPortInfo",
    serial_port_info,
    "SerialPortInfo.rs",
    SerialPortInfo
);

web_feature!(
    "SerialPortRequestOptions",
    serial_port_request_options,
    "SerialPortRequestOptions.rs",
    SerialPortRequestOptions
);

web_feature!(
    "ServiceWorker",
    service_worker,
    "ServiceWorker.rs",
    ServiceWorker
);

web_feature!(
    "ServiceWorkerContainer",
    service_worker_container,
    "ServiceWorkerContainer.rs",
    ServiceWorkerContainer
);

web_feature!(
    "ServiceWorkerGlobalScope",
    service_worker_global_scope,
    "ServiceWorkerGlobalScope.rs",
    ServiceWorkerGlobalScope
);

web_feature!(
    "ServiceWorkerRegistration",
    service_worker_registration,
    "ServiceWorkerRegistration.rs",
    ServiceWorkerRegistration
);

web_feature!(
    "SetHTMLOptions",
    set_html_options,
    "SetHTMLOptions.rs",
    SetHTMLOptions
);

web_feature!(
    "SetHTMLUnsafeOptions",
    set_html_unsafe_options,
    "SetHTMLUnsafeOptions.rs",
    SetHTMLUnsafeOptions
);

web_feature!(
    "ShadowAnimation",
    shadow_animation,
    "ShadowAnimation.rs",
    ShadowAnimation
);

web_feature!("ShadowRoot", shadow_root, "ShadowRoot.rs", ShadowRoot);

web_feature!(
    "ShadowRootInit",
    shadow_root_init,
    "ShadowRootInit.rs",
    ShadowRootInit
);

web_feature!("ShareData", share_data, "ShareData.rs", ShareData);

web_feature!(
    "SharedStorage",
    shared_storage,
    "SharedStorage.rs",
    SharedStorage
);

web_feature!(
    "SharedStorageAppendMethod",
    shared_storage_append_method,
    "SharedStorageAppendMethod.rs",
    SharedStorageAppendMethod
);

web_feature!(
    "SharedStorageClearMethod",
    shared_storage_clear_method,
    "SharedStorageClearMethod.rs",
    SharedStorageClearMethod
);

web_feature!(
    "SharedStorageDeleteMethod",
    shared_storage_delete_method,
    "SharedStorageDeleteMethod.rs",
    SharedStorageDeleteMethod
);

web_feature!(
    "SharedStorageModifierMethod",
    shared_storage_modifier_method,
    "SharedStorageModifierMethod.rs",
    SharedStorageModifierMethod
);

web_feature!(
    "SharedStorageModifierMethodOptions",
    shared_storage_modifier_method_options,
    "SharedStorageModifierMethodOptions.rs",
    SharedStorageModifierMethodOptions
);

web_feature!(
    "SharedStoragePrivateAggregationConfig",
    shared_storage_private_aggregation_config,
    "SharedStoragePrivateAggregationConfig.rs",
    SharedStoragePrivateAggregationConfig
);

web_feature!(
    "SharedStorageRunOperationMethodOptions",
    shared_storage_run_operation_method_options,
    "SharedStorageRunOperationMethodOptions.rs",
    SharedStorageRunOperationMethodOptions
);

web_feature!(
    "SharedStorageSetMethod",
    shared_storage_set_method,
    "SharedStorageSetMethod.rs",
    SharedStorageSetMethod
);

web_feature!(
    "SharedStorageSetMethodOptions",
    shared_storage_set_method_options,
    "SharedStorageSetMethodOptions.rs",
    SharedStorageSetMethodOptions
);

web_feature!(
    "SharedStorageUrlWithMetadata",
    shared_storage_url_with_metadata,
    "SharedStorageUrlWithMetadata.rs",
    SharedStorageUrlWithMetadata
);

web_feature!(
    "SharedStorageWorklet",
    shared_storage_worklet,
    "SharedStorageWorklet.rs",
    SharedStorageWorklet
);

web_feature!(
    "SharedStorageWorkletGlobalScope",
    shared_storage_worklet_global_scope,
    "SharedStorageWorkletGlobalScope.rs",
    SharedStorageWorkletGlobalScope
);

web_feature!(
    "SharedStorageWorkletNavigator",
    shared_storage_worklet_navigator,
    "SharedStorageWorkletNavigator.rs",
    SharedStorageWorkletNavigator
);

web_feature!(
    "SharedStorageWorkletOptions",
    shared_storage_worklet_options,
    "SharedStorageWorkletOptions.rs",
    SharedStorageWorkletOptions
);

web_feature!(
    "SharedWorker",
    shared_worker,
    "SharedWorker.rs",
    SharedWorker
);

web_feature!(
    "SharedWorkerGlobalScope",
    shared_worker_global_scope,
    "SharedWorkerGlobalScope.rs",
    SharedWorkerGlobalScope
);

web_feature!(
    "SharedWorkerOptions",
    shared_worker_options,
    "SharedWorkerOptions.rs",
    SharedWorkerOptions
);

web_feature!(
    "ShowPopoverOptions",
    show_popover_options,
    "ShowPopoverOptions.rs",
    ShowPopoverOptions
);

web_feature!("SnapEvent", snap_event, "SnapEvent.rs", SnapEvent);

web_feature!(
    "SnapEventInit",
    snap_event_init,
    "SnapEventInit.rs",
    SnapEventInit
);

web_feature!(
    "SourceBuffer",
    source_buffer,
    "SourceBuffer.rs",
    SourceBuffer
);

web_feature!(
    "SourceBufferList",
    source_buffer_list,
    "SourceBufferList.rs",
    SourceBufferList
);

web_feature!(
    "SpatialNavigationSearchOptions",
    spatial_navigation_search_options,
    "SpatialNavigationSearchOptions.rs",
    SpatialNavigationSearchOptions
);

web_feature!(
    "SpeechGrammar",
    speech_grammar,
    "SpeechGrammar.rs",
    SpeechGrammar
);

web_feature!(
    "SpeechGrammarList",
    speech_grammar_list,
    "SpeechGrammarList.rs",
    SpeechGrammarList
);

web_feature!(
    "SpeechRecognition",
    speech_recognition,
    "SpeechRecognition.rs",
    SpeechRecognition
);

web_feature!(
    "SpeechRecognitionAlternative",
    speech_recognition_alternative,
    "SpeechRecognitionAlternative.rs",
    SpeechRecognitionAlternative
);

web_feature!(
    "SpeechRecognitionErrorEvent",
    speech_recognition_error_event,
    "SpeechRecognitionErrorEvent.rs",
    SpeechRecognitionErrorEvent
);

web_feature!(
    "SpeechRecognitionErrorEventInit",
    speech_recognition_error_event_init,
    "SpeechRecognitionErrorEventInit.rs",
    SpeechRecognitionErrorEventInit
);

web_feature!(
    "SpeechRecognitionEvent",
    speech_recognition_event,
    "SpeechRecognitionEvent.rs",
    SpeechRecognitionEvent
);

web_feature!(
    "SpeechRecognitionEventInit",
    speech_recognition_event_init,
    "SpeechRecognitionEventInit.rs",
    SpeechRecognitionEventInit
);

web_feature!(
    "SpeechRecognitionOptions",
    speech_recognition_options,
    "SpeechRecognitionOptions.rs",
    SpeechRecognitionOptions
);

web_feature!(
    "SpeechRecognitionPhrase",
    speech_recognition_phrase,
    "SpeechRecognitionPhrase.rs",
    SpeechRecognitionPhrase
);

web_feature!(
    "SpeechRecognitionPhraseList",
    speech_recognition_phrase_list,
    "SpeechRecognitionPhraseList.rs",
    SpeechRecognitionPhraseList
);

web_feature!(
    "SpeechRecognitionResult",
    speech_recognition_result,
    "SpeechRecognitionResult.rs",
    SpeechRecognitionResult
);

web_feature!(
    "SpeechRecognitionResultList",
    speech_recognition_result_list,
    "SpeechRecognitionResultList.rs",
    SpeechRecognitionResultList
);

web_feature!(
    "SpeechSynthesis",
    speech_synthesis,
    "SpeechSynthesis.rs",
    SpeechSynthesis
);

web_feature!(
    "SpeechSynthesisErrorEvent",
    speech_synthesis_error_event,
    "SpeechSynthesisErrorEvent.rs",
    SpeechSynthesisErrorEvent
);

web_feature!(
    "SpeechSynthesisErrorEventInit",
    speech_synthesis_error_event_init,
    "SpeechSynthesisErrorEventInit.rs",
    SpeechSynthesisErrorEventInit
);

web_feature!(
    "SpeechSynthesisEvent",
    speech_synthesis_event,
    "SpeechSynthesisEvent.rs",
    SpeechSynthesisEvent
);

web_feature!(
    "SpeechSynthesisEventInit",
    speech_synthesis_event_init,
    "SpeechSynthesisEventInit.rs",
    SpeechSynthesisEventInit
);

web_feature!(
    "SpeechSynthesisUtterance",
    speech_synthesis_utterance,
    "SpeechSynthesisUtterance.rs",
    SpeechSynthesisUtterance
);

web_feature!(
    "SpeechSynthesisVoice",
    speech_synthesis_voice,
    "SpeechSynthesisVoice.rs",
    SpeechSynthesisVoice
);

web_feature!(
    "StartViewTransitionOptions",
    start_view_transition_options,
    "StartViewTransitionOptions.rs",
    StartViewTransitionOptions
);

web_feature!("StaticRange", static_range, "StaticRange.rs", StaticRange);

web_feature!(
    "StaticRangeInit",
    static_range_init,
    "StaticRangeInit.rs",
    StaticRangeInit
);

web_feature!(
    "StereoPannerNode",
    stereo_panner_node,
    "StereoPannerNode.rs",
    StereoPannerNode
);

web_feature!(
    "StereoPannerOptions",
    stereo_panner_options,
    "StereoPannerOptions.rs",
    StereoPannerOptions
);

web_feature!("Storage", storage, "Storage.rs", Storage);

web_feature!(
    "StorageAccessHandle",
    storage_access_handle,
    "StorageAccessHandle.rs",
    StorageAccessHandle
);

web_feature!(
    "StorageAccessTypes",
    storage_access_types,
    "StorageAccessTypes.rs",
    StorageAccessTypes
);

web_feature!(
    "StorageBucket",
    storage_bucket,
    "StorageBucket.rs",
    StorageBucket
);

web_feature!(
    "StorageBucketManager",
    storage_bucket_manager,
    "StorageBucketManager.rs",
    StorageBucketManager
);

web_feature!(
    "StorageBucketOptions",
    storage_bucket_options,
    "StorageBucketOptions.rs",
    StorageBucketOptions
);

web_feature!(
    "StorageEstimate",
    storage_estimate,
    "StorageEstimate.rs",
    StorageEstimate
);

web_feature!(
    "StorageEvent",
    storage_event,
    "StorageEvent.rs",
    StorageEvent
);

web_feature!(
    "StorageEventInit",
    storage_event_init,
    "StorageEventInit.rs",
    StorageEventInit
);

web_feature!(
    "StorageInterestGroup",
    storage_interest_group,
    "StorageInterestGroup.rs",
    StorageInterestGroup
);

web_feature!(
    "StorageManager",
    storage_manager,
    "StorageManager.rs",
    StorageManager
);

web_feature!(
    "StreamPipeOptions",
    stream_pipe_options,
    "StreamPipeOptions.rs",
    StreamPipeOptions
);

web_feature!(
    "StructuredSerializeOptions",
    structured_serialize_options,
    "StructuredSerializeOptions.rs",
    StructuredSerializeOptions
);

web_feature!(
    "StylePropertyMap",
    style_property_map,
    "StylePropertyMap.rs",
    StylePropertyMap
);

web_feature!(
    "StylePropertyMapReadOnly",
    style_property_map_read_only,
    "StylePropertyMapReadOnly.rs",
    StylePropertyMapReadOnly
);

web_feature!("StyleSheet", style_sheet, "StyleSheet.rs", StyleSheet);

web_feature!(
    "StyleSheetList",
    style_sheet_list,
    "StyleSheetList.rs",
    StyleSheetList
);

web_feature!("SubmitEvent", submit_event, "SubmitEvent.rs", SubmitEvent);

web_feature!(
    "SubmitEventInit",
    submit_event_init,
    "SubmitEventInit.rs",
    SubmitEventInit
);

web_feature!(
    "SubscribeOptions",
    subscribe_options,
    "SubscribeOptions.rs",
    SubscribeOptions
);

web_feature!("Subscriber", subscriber, "Subscriber.rs", Subscriber);

web_feature!(
    "SubscriptionObserver",
    subscription_observer,
    "SubscriptionObserver.rs",
    SubscriptionObserver
);

web_feature!(
    "SubtleCrypto",
    subtle_crypto,
    "SubtleCrypto.rs",
    SubtleCrypto
);

web_feature!("Summarizer", summarizer, "Summarizer.rs", Summarizer);

web_feature!(
    "SummarizerCreateCoreOptions",
    summarizer_create_core_options,
    "SummarizerCreateCoreOptions.rs",
    SummarizerCreateCoreOptions
);

web_feature!(
    "SummarizerCreateOptions",
    summarizer_create_options,
    "SummarizerCreateOptions.rs",
    SummarizerCreateOptions
);

web_feature!(
    "SummarizerSummarizeOptions",
    summarizer_summarize_options,
    "SummarizerSummarizeOptions.rs",
    SummarizerSummarizeOptions
);

web_feature!(
    "SvcOutputMetadata",
    svc_output_metadata,
    "SvcOutputMetadata.rs",
    SvcOutputMetadata
);

web_feature!("SyncEvent", sync_event, "SyncEvent.rs", SyncEvent);

web_feature!(
    "SyncEventInit",
    sync_event_init,
    "SyncEventInit.rs",
    SyncEventInit
);

web_feature!("SyncManager", sync_manager, "SyncManager.rs", SyncManager);

web_feature!("Table", table, "Table.rs", Table);

web_feature!(
    "TableDescriptor",
    table_descriptor,
    "TableDescriptor.rs",
    TableDescriptor
);

web_feature!(
    "TaskAttributionTiming",
    task_attribution_timing,
    "TaskAttributionTiming.rs",
    TaskAttributionTiming
);

web_feature!(
    "TaskController",
    task_controller,
    "TaskController.rs",
    TaskController
);

web_feature!(
    "TaskControllerInit",
    task_controller_init,
    "TaskControllerInit.rs",
    TaskControllerInit
);

web_feature!(
    "TaskPriorityChangeEvent",
    task_priority_change_event,
    "TaskPriorityChangeEvent.rs",
    TaskPriorityChangeEvent
);

web_feature!(
    "TaskPriorityChangeEventInit",
    task_priority_change_event_init,
    "TaskPriorityChangeEventInit.rs",
    TaskPriorityChangeEventInit
);

web_feature!("TaskSignal", task_signal, "TaskSignal.rs", TaskSignal);

web_feature!(
    "TaskSignalAnyInit",
    task_signal_any_init,
    "TaskSignalAnyInit.rs",
    TaskSignalAnyInit
);

web_feature!("TestUtils", test_utils, "TestUtils.rs", TestUtils);

web_feature!("Text", text, "Text.rs", Text);

web_feature!(
    "TextDecodeOptions",
    text_decode_options,
    "TextDecodeOptions.rs",
    TextDecodeOptions
);

web_feature!("TextDecoder", text_decoder, "TextDecoder.rs", TextDecoder);

web_feature!(
    "TextDecoderOptions",
    text_decoder_options,
    "TextDecoderOptions.rs",
    TextDecoderOptions
);

web_feature!(
    "TextDecoderStream",
    text_decoder_stream,
    "TextDecoderStream.rs",
    TextDecoderStream
);

web_feature!(
    "TextDetector",
    text_detector,
    "TextDetector.rs",
    TextDetector
);

web_feature!("TextEncoder", text_encoder, "TextEncoder.rs", TextEncoder);

web_feature!(
    "TextEncoderEncodeIntoResult",
    text_encoder_encode_into_result,
    "TextEncoderEncodeIntoResult.rs",
    TextEncoderEncodeIntoResult
);

web_feature!(
    "TextEncoderStream",
    text_encoder_stream,
    "TextEncoderStream.rs",
    TextEncoderStream
);

web_feature!("TextEvent", text_event, "TextEvent.rs", TextEvent);

web_feature!("TextFormat", text_format, "TextFormat.rs", TextFormat);

web_feature!(
    "TextFormatInit",
    text_format_init,
    "TextFormatInit.rs",
    TextFormatInit
);

web_feature!(
    "TextFormatUpdateEvent",
    text_format_update_event,
    "TextFormatUpdateEvent.rs",
    TextFormatUpdateEvent
);

web_feature!(
    "TextFormatUpdateEventInit",
    text_format_update_event_init,
    "TextFormatUpdateEventInit.rs",
    TextFormatUpdateEventInit
);

web_feature!("TextMetrics", text_metrics, "TextMetrics.rs", TextMetrics);

web_feature!("TextTrack", text_track, "TextTrack.rs", TextTrack);

web_feature!(
    "TextTrackCue",
    text_track_cue,
    "TextTrackCue.rs",
    TextTrackCue
);

web_feature!(
    "TextTrackCueList",
    text_track_cue_list,
    "TextTrackCueList.rs",
    TextTrackCueList
);

web_feature!(
    "TextTrackList",
    text_track_list,
    "TextTrackList.rs",
    TextTrackList
);

web_feature!(
    "TextUpdateEvent",
    text_update_event,
    "TextUpdateEvent.rs",
    TextUpdateEvent
);

web_feature!(
    "TextUpdateEventInit",
    text_update_event_init,
    "TextUpdateEventInit.rs",
    TextUpdateEventInit
);

web_feature!("TimeEvent", time_event, "TimeEvent.rs", TimeEvent);

web_feature!("TimeRanges", time_ranges, "TimeRanges.rs", TimeRanges);

web_feature!(
    "TimelineRangeOffset",
    timeline_range_offset,
    "TimelineRangeOffset.rs",
    TimelineRangeOffset
);

web_feature!("ToggleEvent", toggle_event, "ToggleEvent.rs", ToggleEvent);

web_feature!(
    "ToggleEventInit",
    toggle_event_init,
    "ToggleEventInit.rs",
    ToggleEventInit
);

web_feature!(
    "TogglePopoverOptions",
    toggle_popover_options,
    "TogglePopoverOptions.rs",
    TogglePopoverOptions
);

web_feature!(
    "TokenBinding",
    token_binding,
    "TokenBinding.rs",
    TokenBinding
);

web_feature!(
    "TopLevelStorageAccessPermissionDescriptor",
    top_level_storage_access_permission_descriptor,
    "TopLevelStorageAccessPermissionDescriptor.rs",
    TopLevelStorageAccessPermissionDescriptor
);

web_feature!("Touch", touch, "Touch.rs", Touch);

web_feature!("TouchEvent", touch_event, "TouchEvent.rs", TouchEvent);

web_feature!(
    "TouchEventInit",
    touch_event_init,
    "TouchEventInit.rs",
    TouchEventInit
);

web_feature!("TouchInit", touch_init, "TouchInit.rs", TouchInit);

web_feature!("TouchList", touch_list, "TouchList.rs", TouchList);

web_feature!("TrackEvent", track_event, "TrackEvent.rs", TrackEvent);

web_feature!(
    "TrackEventInit",
    track_event_init,
    "TrackEventInit.rs",
    TrackEventInit
);

web_feature!(
    "TransformStream",
    transform_stream,
    "TransformStream.rs",
    TransformStream
);

web_feature!(
    "TransformStreamDefaultController",
    transform_stream_default_controller,
    "TransformStreamDefaultController.rs",
    TransformStreamDefaultController
);

web_feature!("Transformer", transformer, "Transformer.rs", Transformer);

web_feature!(
    "TransitionEvent",
    transition_event,
    "TransitionEvent.rs",
    TransitionEvent
);

web_feature!(
    "TransitionEventInit",
    transition_event_init,
    "TransitionEventInit.rs",
    TransitionEventInit
);

web_feature!("Translator", translator, "Translator.rs", Translator);

web_feature!(
    "TranslatorCreateCoreOptions",
    translator_create_core_options,
    "TranslatorCreateCoreOptions.rs",
    TranslatorCreateCoreOptions
);

web_feature!(
    "TranslatorCreateOptions",
    translator_create_options,
    "TranslatorCreateOptions.rs",
    TranslatorCreateOptions
);

web_feature!(
    "TranslatorTranslateOptions",
    translator_translate_options,
    "TranslatorTranslateOptions.rs",
    TranslatorTranslateOptions
);

web_feature!("TreeWalker", tree_walker, "TreeWalker.rs", TreeWalker);

web_feature!("TrustedHTML", trusted_html, "TrustedHTML.rs", TrustedHTML);

web_feature!(
    "TrustedScript",
    trusted_script,
    "TrustedScript.rs",
    TrustedScript
);

web_feature!(
    "TrustedScriptURL",
    trusted_script_url,
    "TrustedScriptURL.rs",
    TrustedScriptURL
);

web_feature!(
    "TrustedTypePolicy",
    trusted_type_policy,
    "TrustedTypePolicy.rs",
    TrustedTypePolicy
);

web_feature!(
    "TrustedTypePolicyFactory",
    trusted_type_policy_factory,
    "TrustedTypePolicyFactory.rs",
    TrustedTypePolicyFactory
);

web_feature!(
    "TrustedTypePolicyOptions",
    trusted_type_policy_options,
    "TrustedTypePolicyOptions.rs",
    TrustedTypePolicyOptions
);

web_feature!(
    "UADataValues",
    ua_data_values,
    "UADataValues.rs",
    UADataValues
);

web_feature!(
    "UALowEntropyJSON",
    ua_low_entropy_json,
    "UALowEntropyJSON.rs",
    UALowEntropyJSON
);

web_feature!("UIEvent", ui_event, "UIEvent.rs", UIEvent);

web_feature!("UIEventInit", ui_event_init, "UIEventInit.rs", UIEventInit);

web_feature!("ULongRange", u_long_range, "ULongRange.rs", ULongRange);

web_feature!("URL", url, "URL.rs", URL);

web_feature!("URLPattern", url_pattern, "URLPattern.rs", URLPattern);

web_feature!(
    "URLPatternComponentResult",
    url_pattern_component_result,
    "URLPatternComponentResult.rs",
    URLPatternComponentResult
);

web_feature!(
    "URLPatternInit",
    url_pattern_init,
    "URLPatternInit.rs",
    URLPatternInit
);

web_feature!(
    "URLPatternOptions",
    url_pattern_options,
    "URLPatternOptions.rs",
    URLPatternOptions
);

web_feature!(
    "URLPatternResult",
    url_pattern_result,
    "URLPatternResult.rs",
    URLPatternResult
);

web_feature!(
    "URLSearchParams",
    url_search_params,
    "URLSearchParams.rs",
    URLSearchParams
);

web_feature!("USB", usb, "USB.rs", USB);

web_feature!(
    "USBAlternateInterface",
    usb_alternate_interface,
    "USBAlternateInterface.rs",
    USBAlternateInterface
);

web_feature!(
    "USBBlocklistEntry",
    usb_blocklist_entry,
    "USBBlocklistEntry.rs",
    USBBlocklistEntry
);

web_feature!(
    "USBConfiguration",
    usb_configuration,
    "USBConfiguration.rs",
    USBConfiguration
);

web_feature!(
    "USBConnectionEvent",
    usb_connection_event,
    "USBConnectionEvent.rs",
    USBConnectionEvent
);

web_feature!(
    "USBConnectionEventInit",
    usb_connection_event_init,
    "USBConnectionEventInit.rs",
    USBConnectionEventInit
);

web_feature!(
    "USBControlTransferParameters",
    usb_control_transfer_parameters,
    "USBControlTransferParameters.rs",
    USBControlTransferParameters
);

web_feature!("USBDevice", usb_device, "USBDevice.rs", USBDevice);

web_feature!(
    "USBDeviceFilter",
    usb_device_filter,
    "USBDeviceFilter.rs",
    USBDeviceFilter
);

web_feature!(
    "USBDeviceRequestOptions",
    usb_device_request_options,
    "USBDeviceRequestOptions.rs",
    USBDeviceRequestOptions
);

web_feature!("USBEndpoint", usb_endpoint, "USBEndpoint.rs", USBEndpoint);

web_feature!(
    "USBInTransferResult",
    usb_in_transfer_result,
    "USBInTransferResult.rs",
    USBInTransferResult
);

web_feature!(
    "USBInterface",
    usb_interface,
    "USBInterface.rs",
    USBInterface
);

web_feature!(
    "USBIsochronousInTransferPacket",
    usb_isochronous_in_transfer_packet,
    "USBIsochronousInTransferPacket.rs",
    USBIsochronousInTransferPacket
);

web_feature!(
    "USBIsochronousInTransferResult",
    usb_isochronous_in_transfer_result,
    "USBIsochronousInTransferResult.rs",
    USBIsochronousInTransferResult
);

web_feature!(
    "USBIsochronousOutTransferPacket",
    usb_isochronous_out_transfer_packet,
    "USBIsochronousOutTransferPacket.rs",
    USBIsochronousOutTransferPacket
);

web_feature!(
    "USBIsochronousOutTransferResult",
    usb_isochronous_out_transfer_result,
    "USBIsochronousOutTransferResult.rs",
    USBIsochronousOutTransferResult
);

web_feature!(
    "USBOutTransferResult",
    usb_out_transfer_result,
    "USBOutTransferResult.rs",
    USBOutTransferResult
);

web_feature!(
    "USBPermissionDescriptor",
    usb_permission_descriptor,
    "USBPermissionDescriptor.rs",
    USBPermissionDescriptor
);

web_feature!(
    "USBPermissionResult",
    usb_permission_result,
    "USBPermissionResult.rs",
    USBPermissionResult
);

web_feature!(
    "USBPermissionStorage",
    usb_permission_storage,
    "USBPermissionStorage.rs",
    USBPermissionStorage
);

web_feature!(
    "UncalibratedMagnetometer",
    uncalibrated_magnetometer,
    "UncalibratedMagnetometer.rs",
    UncalibratedMagnetometer
);

web_feature!(
    "UnderlyingSink",
    underlying_sink,
    "UnderlyingSink.rs",
    UnderlyingSink
);

web_feature!(
    "UnderlyingSource",
    underlying_source,
    "UnderlyingSource.rs",
    UnderlyingSource
);

web_feature!(
    "UnknownCredentialOptions",
    unknown_credential_options,
    "UnknownCredentialOptions.rs",
    UnknownCredentialOptions
);

web_feature!(
    "UserActivation",
    user_activation,
    "UserActivation.rs",
    UserActivation
);

web_feature!("VTTCue", vtt_cue, "VTTCue.rs", VTTCue);

web_feature!("VTTRegion", vtt_region, "VTTRegion.rs", VTTRegion);

web_feature!(
    "ValidityState",
    validity_state,
    "ValidityState.rs",
    ValidityState
);

web_feature!(
    "ValidityStateFlags",
    validity_state_flags,
    "ValidityStateFlags.rs",
    ValidityStateFlags
);

web_feature!("ValueEvent", value_event, "ValueEvent.rs", ValueEvent);

web_feature!(
    "ValueEventInit",
    value_event_init,
    "ValueEventInit.rs",
    ValueEventInit
);

web_feature!(
    "VideoColorSpace",
    video_color_space,
    "VideoColorSpace.rs",
    VideoColorSpace
);

web_feature!(
    "VideoColorSpaceInit",
    video_color_space_init,
    "VideoColorSpaceInit.rs",
    VideoColorSpaceInit
);

web_feature!(
    "VideoConfiguration",
    video_configuration,
    "VideoConfiguration.rs",
    VideoConfiguration
);

web_feature!(
    "VideoDecoder",
    video_decoder,
    "VideoDecoder.rs",
    VideoDecoder
);

web_feature!(
    "VideoDecoderConfig",
    video_decoder_config,
    "VideoDecoderConfig.rs",
    VideoDecoderConfig
);

web_feature!(
    "VideoDecoderInit",
    video_decoder_init,
    "VideoDecoderInit.rs",
    VideoDecoderInit
);

web_feature!(
    "VideoDecoderSupport",
    video_decoder_support,
    "VideoDecoderSupport.rs",
    VideoDecoderSupport
);

web_feature!(
    "VideoEncoder",
    video_encoder,
    "VideoEncoder.rs",
    VideoEncoder
);

web_feature!(
    "VideoEncoderConfig",
    video_encoder_config,
    "VideoEncoderConfig.rs",
    VideoEncoderConfig
);

web_feature!(
    "VideoEncoderEncodeOptions",
    video_encoder_encode_options,
    "VideoEncoderEncodeOptions.rs",
    VideoEncoderEncodeOptions
);

web_feature!(
    "VideoEncoderEncodeOptionsForAv1",
    video_encoder_encode_options_for_av1,
    "VideoEncoderEncodeOptionsForAv1.rs",
    VideoEncoderEncodeOptionsForAv1
);

web_feature!(
    "VideoEncoderEncodeOptionsForAvc",
    video_encoder_encode_options_for_avc,
    "VideoEncoderEncodeOptionsForAvc.rs",
    VideoEncoderEncodeOptionsForAvc
);

web_feature!(
    "VideoEncoderEncodeOptionsForHevc",
    video_encoder_encode_options_for_hevc,
    "VideoEncoderEncodeOptionsForHevc.rs",
    VideoEncoderEncodeOptionsForHevc
);

web_feature!(
    "VideoEncoderEncodeOptionsForVp9",
    video_encoder_encode_options_for_vp9,
    "VideoEncoderEncodeOptionsForVp9.rs",
    VideoEncoderEncodeOptionsForVp9
);

web_feature!(
    "VideoEncoderInit",
    video_encoder_init,
    "VideoEncoderInit.rs",
    VideoEncoderInit
);

web_feature!(
    "VideoEncoderSupport",
    video_encoder_support,
    "VideoEncoderSupport.rs",
    VideoEncoderSupport
);

web_feature!("VideoFrame", video_frame, "VideoFrame.rs", VideoFrame);

web_feature!(
    "VideoFrameBufferInit",
    video_frame_buffer_init,
    "VideoFrameBufferInit.rs",
    VideoFrameBufferInit
);

web_feature!(
    "VideoFrameCallbackMetadata",
    video_frame_callback_metadata,
    "VideoFrameCallbackMetadata.rs",
    VideoFrameCallbackMetadata
);

web_feature!(
    "VideoFrameCopyToOptions",
    video_frame_copy_to_options,
    "VideoFrameCopyToOptions.rs",
    VideoFrameCopyToOptions
);

web_feature!(
    "VideoFrameInit",
    video_frame_init,
    "VideoFrameInit.rs",
    VideoFrameInit
);

web_feature!(
    "VideoFrameMetadata",
    video_frame_metadata,
    "VideoFrameMetadata.rs",
    VideoFrameMetadata
);

web_feature!(
    "VideoPlaybackQuality",
    video_playback_quality,
    "VideoPlaybackQuality.rs",
    VideoPlaybackQuality
);

web_feature!("VideoTrack", video_track, "VideoTrack.rs", VideoTrack);

web_feature!(
    "VideoTrackGenerator",
    video_track_generator,
    "VideoTrackGenerator.rs",
    VideoTrackGenerator
);

web_feature!(
    "VideoTrackList",
    video_track_list,
    "VideoTrackList.rs",
    VideoTrackList
);

web_feature!(
    "ViewTimeline",
    view_timeline,
    "ViewTimeline.rs",
    ViewTimeline
);

web_feature!(
    "ViewTimelineOptions",
    view_timeline_options,
    "ViewTimelineOptions.rs",
    ViewTimelineOptions
);

web_feature!(
    "ViewTransition",
    view_transition,
    "ViewTransition.rs",
    ViewTransition
);

web_feature!(
    "ViewTransitionTypeSet",
    view_transition_type_set,
    "ViewTransitionTypeSet.rs",
    ViewTransitionTypeSet
);

web_feature!("Viewport", viewport, "Viewport.rs", Viewport);

web_feature!(
    "VirtualKeyboard",
    virtual_keyboard,
    "VirtualKeyboard.rs",
    VirtualKeyboard
);

web_feature!(
    "VisibilityStateEntry",
    visibility_state_entry,
    "VisibilityStateEntry.rs",
    VisibilityStateEntry
);

web_feature!(
    "VisualViewport",
    visual_viewport,
    "VisualViewport.rs",
    VisualViewport
);

web_feature!(
    "WEBGL_blend_equation_advanced_coherent",
    webgl_blend_equation_advanced_coherent,
    "WEBGL_blend_equation_advanced_coherent.rs",
    WEBGL_blend_equation_advanced_coherent
);

web_feature!(
    "WEBGL_clip_cull_distance",
    webgl_clip_cull_distance,
    "WEBGL_clip_cull_distance.rs",
    WEBGL_clip_cull_distance
);

web_feature!(
    "WEBGL_color_buffer_float",
    webgl_color_buffer_float,
    "WEBGL_color_buffer_float.rs",
    WEBGL_color_buffer_float
);

web_feature!(
    "WEBGL_compressed_texture_astc",
    webgl_compressed_texture_astc,
    "WEBGL_compressed_texture_astc.rs",
    WEBGL_compressed_texture_astc
);

web_feature!(
    "WEBGL_compressed_texture_etc",
    webgl_compressed_texture_etc,
    "WEBGL_compressed_texture_etc.rs",
    WEBGL_compressed_texture_etc
);

web_feature!(
    "WEBGL_compressed_texture_etc1",
    webgl_compressed_texture_etc1,
    "WEBGL_compressed_texture_etc1.rs",
    WEBGL_compressed_texture_etc1
);

web_feature!(
    "WEBGL_compressed_texture_pvrtc",
    webgl_compressed_texture_pvrtc,
    "WEBGL_compressed_texture_pvrtc.rs",
    WEBGL_compressed_texture_pvrtc
);

web_feature!(
    "WEBGL_compressed_texture_s3tc",
    webgl_compressed_texture_s3tc,
    "WEBGL_compressed_texture_s3tc.rs",
    WEBGL_compressed_texture_s3tc
);

web_feature!(
    "WEBGL_compressed_texture_s3tc_srgb",
    webgl_compressed_texture_s3tc_srgb,
    "WEBGL_compressed_texture_s3tc_srgb.rs",
    WEBGL_compressed_texture_s3tc_srgb
);

web_feature!(
    "WEBGL_debug_renderer_info",
    webgl_debug_renderer_info,
    "WEBGL_debug_renderer_info.rs",
    WEBGL_debug_renderer_info
);

web_feature!(
    "WEBGL_debug_shaders",
    webgl_debug_shaders,
    "WEBGL_debug_shaders.rs",
    WEBGL_debug_shaders
);

web_feature!(
    "WEBGL_depth_texture",
    webgl_depth_texture,
    "WEBGL_depth_texture.rs",
    WEBGL_depth_texture
);

web_feature!(
    "WEBGL_draw_buffers",
    webgl_draw_buffers,
    "WEBGL_draw_buffers.rs",
    WEBGL_draw_buffers
);

web_feature!(
    "WEBGL_draw_instanced_base_vertex_base_instance",
    webgl_draw_instanced_base_vertex_base_instance,
    "WEBGL_draw_instanced_base_vertex_base_instance.rs",
    WEBGL_draw_instanced_base_vertex_base_instance
);

web_feature!(
    "WEBGL_lose_context",
    webgl_lose_context,
    "WEBGL_lose_context.rs",
    WEBGL_lose_context
);

web_feature!(
    "WEBGL_multi_draw",
    webgl_multi_draw,
    "WEBGL_multi_draw.rs",
    WEBGL_multi_draw
);

web_feature!(
    "WEBGL_multi_draw_instanced_base_vertex_base_instance",
    webgl_multi_draw_instanced_base_vertex_base_instance,
    "WEBGL_multi_draw_instanced_base_vertex_base_instance.rs",
    WEBGL_multi_draw_instanced_base_vertex_base_instance
);

web_feature!(
    "WEBGL_provoking_vertex",
    webgl_provoking_vertex,
    "WEBGL_provoking_vertex.rs",
    WEBGL_provoking_vertex
);

web_feature!(
    "WGSLLanguageFeatures",
    wgsl_language_features,
    "WGSLLanguageFeatures.rs",
    WGSLLanguageFeatures
);

web_feature!("WakeLock", wake_lock, "WakeLock.rs", WakeLock);

web_feature!(
    "WakeLockSentinel",
    wake_lock_sentinel,
    "WakeLockSentinel.rs",
    WakeLockSentinel
);

web_feature!(
    "WatchAdvertisementsOptions",
    watch_advertisements_options,
    "WatchAdvertisementsOptions.rs",
    WatchAdvertisementsOptions
);

web_feature!(
    "WaveShaperNode",
    wave_shaper_node,
    "WaveShaperNode.rs",
    WaveShaperNode
);

web_feature!(
    "WaveShaperOptions",
    wave_shaper_options,
    "WaveShaperOptions.rs",
    WaveShaperOptions
);

web_feature!("WebAssembly", web_assembly, "WebAssembly.rs", WebAssembly);

web_feature!(
    "WebAssemblyInstantiatedSource",
    web_assembly_instantiated_source,
    "WebAssemblyInstantiatedSource.rs",
    WebAssemblyInstantiatedSource
);

web_feature!(
    "WebGL2RenderingContext",
    web_gl2_rendering_context,
    "WebGL2RenderingContext.rs",
    WebGL2RenderingContext
);

web_feature!(
    "WebGLActiveInfo",
    web_gl_active_info,
    "WebGLActiveInfo.rs",
    WebGLActiveInfo
);

web_feature!("WebGLBuffer", web_gl_buffer, "WebGLBuffer.rs", WebGLBuffer);

web_feature!(
    "WebGLContextAttributes",
    web_gl_context_attributes,
    "WebGLContextAttributes.rs",
    WebGLContextAttributes
);

web_feature!(
    "WebGLContextEvent",
    web_gl_context_event,
    "WebGLContextEvent.rs",
    WebGLContextEvent
);

web_feature!(
    "WebGLContextEventInit",
    web_gl_context_event_init,
    "WebGLContextEventInit.rs",
    WebGLContextEventInit
);

web_feature!(
    "WebGLFramebuffer",
    web_gl_framebuffer,
    "WebGLFramebuffer.rs",
    WebGLFramebuffer
);

web_feature!("WebGLObject", web_gl_object, "WebGLObject.rs", WebGLObject);

web_feature!(
    "WebGLProgram",
    web_gl_program,
    "WebGLProgram.rs",
    WebGLProgram
);

web_feature!("WebGLQuery", web_gl_query, "WebGLQuery.rs", WebGLQuery);

web_feature!(
    "WebGLRenderbuffer",
    web_gl_renderbuffer,
    "WebGLRenderbuffer.rs",
    WebGLRenderbuffer
);

web_feature!(
    "WebGLRenderingContext",
    web_gl_rendering_context,
    "WebGLRenderingContext.rs",
    WebGLRenderingContext
);

web_feature!(
    "WebGLSampler",
    web_gl_sampler,
    "WebGLSampler.rs",
    WebGLSampler
);

web_feature!("WebGLShader", web_gl_shader, "WebGLShader.rs", WebGLShader);

web_feature!(
    "WebGLShaderPrecisionFormat",
    web_gl_shader_precision_format,
    "WebGLShaderPrecisionFormat.rs",
    WebGLShaderPrecisionFormat
);

web_feature!("WebGLSync", web_gl_sync, "WebGLSync.rs", WebGLSync);

web_feature!(
    "WebGLTexture",
    web_gl_texture,
    "WebGLTexture.rs",
    WebGLTexture
);

web_feature!(
    "WebGLTimerQueryEXT",
    web_gl_timer_query_ext,
    "WebGLTimerQueryEXT.rs",
    WebGLTimerQueryEXT
);

web_feature!(
    "WebGLTransformFeedback",
    web_gl_transform_feedback,
    "WebGLTransformFeedback.rs",
    WebGLTransformFeedback
);

web_feature!(
    "WebGLUniformLocation",
    web_gl_uniform_location,
    "WebGLUniformLocation.rs",
    WebGLUniformLocation
);

web_feature!(
    "WebGLVertexArrayObject",
    web_gl_vertex_array_object,
    "WebGLVertexArrayObject.rs",
    WebGLVertexArrayObject
);

web_feature!(
    "WebGLVertexArrayObjectOES",
    web_gl_vertex_array_object_oes,
    "WebGLVertexArrayObjectOES.rs",
    WebGLVertexArrayObjectOES
);

web_feature!("WebSocket", web_socket, "WebSocket.rs", WebSocket);

web_feature!(
    "WebTransport",
    web_transport,
    "WebTransport.rs",
    WebTransport
);

web_feature!(
    "WebTransportBidirectionalStream",
    web_transport_bidirectional_stream,
    "WebTransportBidirectionalStream.rs",
    WebTransportBidirectionalStream
);

web_feature!(
    "WebTransportCloseInfo",
    web_transport_close_info,
    "WebTransportCloseInfo.rs",
    WebTransportCloseInfo
);

web_feature!(
    "WebTransportConnectionStats",
    web_transport_connection_stats,
    "WebTransportConnectionStats.rs",
    WebTransportConnectionStats
);

web_feature!(
    "WebTransportDatagramDuplexStream",
    web_transport_datagram_duplex_stream,
    "WebTransportDatagramDuplexStream.rs",
    WebTransportDatagramDuplexStream
);

web_feature!(
    "WebTransportDatagramStats",
    web_transport_datagram_stats,
    "WebTransportDatagramStats.rs",
    WebTransportDatagramStats
);

web_feature!(
    "WebTransportDatagramsWritable",
    web_transport_datagrams_writable,
    "WebTransportDatagramsWritable.rs",
    WebTransportDatagramsWritable
);

web_feature!(
    "WebTransportError",
    web_transport_error,
    "WebTransportError.rs",
    WebTransportError
);

web_feature!(
    "WebTransportErrorOptions",
    web_transport_error_options,
    "WebTransportErrorOptions.rs",
    WebTransportErrorOptions
);

web_feature!(
    "WebTransportHash",
    web_transport_hash,
    "WebTransportHash.rs",
    WebTransportHash
);

web_feature!(
    "WebTransportOptions",
    web_transport_options,
    "WebTransportOptions.rs",
    WebTransportOptions
);

web_feature!(
    "WebTransportReceiveStream",
    web_transport_receive_stream,
    "WebTransportReceiveStream.rs",
    WebTransportReceiveStream
);

web_feature!(
    "WebTransportReceiveStreamStats",
    web_transport_receive_stream_stats,
    "WebTransportReceiveStreamStats.rs",
    WebTransportReceiveStreamStats
);

web_feature!(
    "WebTransportSendGroup",
    web_transport_send_group,
    "WebTransportSendGroup.rs",
    WebTransportSendGroup
);

web_feature!(
    "WebTransportSendOptions",
    web_transport_send_options,
    "WebTransportSendOptions.rs",
    WebTransportSendOptions
);

web_feature!(
    "WebTransportSendStream",
    web_transport_send_stream,
    "WebTransportSendStream.rs",
    WebTransportSendStream
);

web_feature!(
    "WebTransportSendStreamOptions",
    web_transport_send_stream_options,
    "WebTransportSendStreamOptions.rs",
    WebTransportSendStreamOptions
);

web_feature!(
    "WebTransportSendStreamStats",
    web_transport_send_stream_stats,
    "WebTransportSendStreamStats.rs",
    WebTransportSendStreamStats
);

web_feature!(
    "WebTransportWriter",
    web_transport_writer,
    "WebTransportWriter.rs",
    WebTransportWriter
);

web_feature!("WheelEvent", wheel_event, "WheelEvent.rs", WheelEvent);

web_feature!(
    "WheelEventInit",
    wheel_event_init,
    "WheelEventInit.rs",
    WheelEventInit
);

web_feature!("Window", window, "Window.rs", Window);

web_feature!(
    "WindowClient",
    window_client,
    "WindowClient.rs",
    WindowClient
);

web_feature!(
    "WindowControlsOverlay",
    window_controls_overlay,
    "WindowControlsOverlay.rs",
    WindowControlsOverlay
);

web_feature!(
    "WindowControlsOverlayGeometryChangeEvent",
    window_controls_overlay_geometry_change_event,
    "WindowControlsOverlayGeometryChangeEvent.rs",
    WindowControlsOverlayGeometryChangeEvent
);

web_feature!(
    "WindowControlsOverlayGeometryChangeEventInit",
    window_controls_overlay_geometry_change_event_init,
    "WindowControlsOverlayGeometryChangeEventInit.rs",
    WindowControlsOverlayGeometryChangeEventInit
);

web_feature!(
    "WindowPostMessageOptions",
    window_post_message_options,
    "WindowPostMessageOptions.rs",
    WindowPostMessageOptions
);

web_feature!("Worker", worker, "Worker.rs", Worker);

web_feature!(
    "WorkerGlobalScope",
    worker_global_scope,
    "WorkerGlobalScope.rs",
    WorkerGlobalScope
);

web_feature!(
    "WorkerLocation",
    worker_location,
    "WorkerLocation.rs",
    WorkerLocation
);

web_feature!(
    "WorkerNavigator",
    worker_navigator,
    "WorkerNavigator.rs",
    WorkerNavigator
);

web_feature!(
    "WorkerOptions",
    worker_options,
    "WorkerOptions.rs",
    WorkerOptions
);

web_feature!("Worklet", worklet, "Worklet.rs", Worklet);

web_feature!(
    "WorkletAnimation",
    worklet_animation,
    "WorkletAnimation.rs",
    WorkletAnimation
);

web_feature!(
    "WorkletAnimationEffect",
    worklet_animation_effect,
    "WorkletAnimationEffect.rs",
    WorkletAnimationEffect
);

web_feature!(
    "WorkletGlobalScope",
    worklet_global_scope,
    "WorkletGlobalScope.rs",
    WorkletGlobalScope
);

web_feature!(
    "WorkletGroupEffect",
    worklet_group_effect,
    "WorkletGroupEffect.rs",
    WorkletGroupEffect
);

web_feature!(
    "WorkletOptions",
    worklet_options,
    "WorkletOptions.rs",
    WorkletOptions
);

web_feature!(
    "WritableStream",
    writable_stream,
    "WritableStream.rs",
    WritableStream
);

web_feature!(
    "WritableStreamDefaultController",
    writable_stream_default_controller,
    "WritableStreamDefaultController.rs",
    WritableStreamDefaultController
);

web_feature!(
    "WritableStreamDefaultWriter",
    writable_stream_default_writer,
    "WritableStreamDefaultWriter.rs",
    WritableStreamDefaultWriter
);

web_feature!("WriteParams", write_params, "WriteParams.rs", WriteParams);

web_feature!("Writer", writer, "Writer.rs", Writer);

web_feature!(
    "WriterCreateCoreOptions",
    writer_create_core_options,
    "WriterCreateCoreOptions.rs",
    WriterCreateCoreOptions
);

web_feature!(
    "WriterCreateOptions",
    writer_create_options,
    "WriterCreateOptions.rs",
    WriterCreateOptions
);

web_feature!(
    "WriterWriteOptions",
    writer_write_options,
    "WriterWriteOptions.rs",
    WriterWriteOptions
);

web_feature!("XMLDocument", xml_document, "XMLDocument.rs", XMLDocument);

web_feature!(
    "XMLHttpRequest",
    xml_http_request,
    "XMLHttpRequest.rs",
    XMLHttpRequest
);

web_feature!(
    "XMLHttpRequestEventTarget",
    xml_http_request_event_target,
    "XMLHttpRequestEventTarget.rs",
    XMLHttpRequestEventTarget
);

web_feature!(
    "XMLHttpRequestUpload",
    xml_http_request_upload,
    "XMLHttpRequestUpload.rs",
    XMLHttpRequestUpload
);

web_feature!(
    "XMLSerializer",
    xml_serializer,
    "XMLSerializer.rs",
    XMLSerializer
);

web_feature!(
    "XPathEvaluator",
    x_path_evaluator,
    "XPathEvaluator.rs",
    XPathEvaluator
);

web_feature!(
    "XPathExpression",
    x_path_expression,
    "XPathExpression.rs",
    XPathExpression
);

web_feature!("XPathResult", x_path_result, "XPathResult.rs", XPathResult);

web_feature!("XRAnchor", xr_anchor, "XRAnchor.rs", XRAnchor);

web_feature!("XRAnchorSet", xr_anchor_set, "XRAnchorSet.rs", XRAnchorSet);

web_feature!(
    "XRBoundedReferenceSpace",
    xr_bounded_reference_space,
    "XRBoundedReferenceSpace.rs",
    XRBoundedReferenceSpace
);

web_feature!(
    "XRCPUDepthInformation",
    xrcpu_depth_information,
    "XRCPUDepthInformation.rs",
    XRCPUDepthInformation
);

web_feature!("XRCamera", xr_camera, "XRCamera.rs", XRCamera);

web_feature!(
    "XRCompositionLayer",
    xr_composition_layer,
    "XRCompositionLayer.rs",
    XRCompositionLayer
);

web_feature!("XRCubeLayer", xr_cube_layer, "XRCubeLayer.rs", XRCubeLayer);

web_feature!(
    "XRCubeLayerInit",
    xr_cube_layer_init,
    "XRCubeLayerInit.rs",
    XRCubeLayerInit
);

web_feature!(
    "XRCylinderLayer",
    xr_cylinder_layer,
    "XRCylinderLayer.rs",
    XRCylinderLayer
);

web_feature!(
    "XRCylinderLayerInit",
    xr_cylinder_layer_init,
    "XRCylinderLayerInit.rs",
    XRCylinderLayerInit
);

web_feature!(
    "XRDOMOverlayInit",
    xrdom_overlay_init,
    "XRDOMOverlayInit.rs",
    XRDOMOverlayInit
);

web_feature!(
    "XRDOMOverlayState",
    xrdom_overlay_state,
    "XRDOMOverlayState.rs",
    XRDOMOverlayState
);

web_feature!(
    "XRDepthInformation",
    xr_depth_information,
    "XRDepthInformation.rs",
    XRDepthInformation
);

web_feature!(
    "XRDepthStateInit",
    xr_depth_state_init,
    "XRDepthStateInit.rs",
    XRDepthStateInit
);

web_feature!(
    "XREquirectLayer",
    xr_equirect_layer,
    "XREquirectLayer.rs",
    XREquirectLayer
);

web_feature!(
    "XREquirectLayerInit",
    xr_equirect_layer_init,
    "XREquirectLayerInit.rs",
    XREquirectLayerInit
);

web_feature!("XRFrame", xr_frame, "XRFrame.rs", XRFrame);

web_feature!("XRHand", xr_hand, "XRHand.rs", XRHand);

web_feature!(
    "XRHitTestOptionsInit",
    xr_hit_test_options_init,
    "XRHitTestOptionsInit.rs",
    XRHitTestOptionsInit
);

web_feature!(
    "XRHitTestResult",
    xr_hit_test_result,
    "XRHitTestResult.rs",
    XRHitTestResult
);

web_feature!(
    "XRHitTestSource",
    xr_hit_test_source,
    "XRHitTestSource.rs",
    XRHitTestSource
);

web_feature!(
    "XRInputSource",
    xr_input_source,
    "XRInputSource.rs",
    XRInputSource
);

web_feature!(
    "XRInputSourceArray",
    xr_input_source_array,
    "XRInputSourceArray.rs",
    XRInputSourceArray
);

web_feature!(
    "XRInputSourceEvent",
    xr_input_source_event,
    "XRInputSourceEvent.rs",
    XRInputSourceEvent
);

web_feature!(
    "XRInputSourceEventInit",
    xr_input_source_event_init,
    "XRInputSourceEventInit.rs",
    XRInputSourceEventInit
);

web_feature!(
    "XRInputSourcesChangeEvent",
    xr_input_sources_change_event,
    "XRInputSourcesChangeEvent.rs",
    XRInputSourcesChangeEvent
);

web_feature!(
    "XRInputSourcesChangeEventInit",
    xr_input_sources_change_event_init,
    "XRInputSourcesChangeEventInit.rs",
    XRInputSourcesChangeEventInit
);

web_feature!("XRJointPose", xr_joint_pose, "XRJointPose.rs", XRJointPose);

web_feature!(
    "XRJointSpace",
    xr_joint_space,
    "XRJointSpace.rs",
    XRJointSpace
);

web_feature!("XRLayer", xr_layer, "XRLayer.rs", XRLayer);

web_feature!(
    "XRLayerEvent",
    xr_layer_event,
    "XRLayerEvent.rs",
    XRLayerEvent
);

web_feature!(
    "XRLayerEventInit",
    xr_layer_event_init,
    "XRLayerEventInit.rs",
    XRLayerEventInit
);

web_feature!("XRLayerInit", xr_layer_init, "XRLayerInit.rs", XRLayerInit);

web_feature!(
    "XRLightEstimate",
    xr_light_estimate,
    "XRLightEstimate.rs",
    XRLightEstimate
);

web_feature!(
    "XRLightProbe",
    xr_light_probe,
    "XRLightProbe.rs",
    XRLightProbe
);

web_feature!(
    "XRLightProbeInit",
    xr_light_probe_init,
    "XRLightProbeInit.rs",
    XRLightProbeInit
);

web_feature!(
    "XRMediaBinding",
    xr_media_binding,
    "XRMediaBinding.rs",
    XRMediaBinding
);

web_feature!(
    "XRMediaCylinderLayerInit",
    xr_media_cylinder_layer_init,
    "XRMediaCylinderLayerInit.rs",
    XRMediaCylinderLayerInit
);

web_feature!(
    "XRMediaEquirectLayerInit",
    xr_media_equirect_layer_init,
    "XRMediaEquirectLayerInit.rs",
    XRMediaEquirectLayerInit
);

web_feature!(
    "XRMediaLayerInit",
    xr_media_layer_init,
    "XRMediaLayerInit.rs",
    XRMediaLayerInit
);

web_feature!(
    "XRMediaQuadLayerInit",
    xr_media_quad_layer_init,
    "XRMediaQuadLayerInit.rs",
    XRMediaQuadLayerInit
);

web_feature!("XRMesh", xr_mesh, "XRMesh.rs", XRMesh);

web_feature!("XRMeshSet", xr_mesh_set, "XRMeshSet.rs", XRMeshSet);

web_feature!(
    "XRPermissionDescriptor",
    xr_permission_descriptor,
    "XRPermissionDescriptor.rs",
    XRPermissionDescriptor
);

web_feature!(
    "XRPermissionStatus",
    xr_permission_status,
    "XRPermissionStatus.rs",
    XRPermissionStatus
);

web_feature!("XRPlane", xr_plane, "XRPlane.rs", XRPlane);

web_feature!("XRPlaneSet", xr_plane_set, "XRPlaneSet.rs", XRPlaneSet);

web_feature!("XRPose", xr_pose, "XRPose.rs", XRPose);

web_feature!(
    "XRProjectionLayer",
    xr_projection_layer,
    "XRProjectionLayer.rs",
    XRProjectionLayer
);

web_feature!(
    "XRProjectionLayerInit",
    xr_projection_layer_init,
    "XRProjectionLayerInit.rs",
    XRProjectionLayerInit
);

web_feature!("XRQuadLayer", xr_quad_layer, "XRQuadLayer.rs", XRQuadLayer);

web_feature!(
    "XRQuadLayerInit",
    xr_quad_layer_init,
    "XRQuadLayerInit.rs",
    XRQuadLayerInit
);

web_feature!("XRRay", xr_ray, "XRRay.rs", XRRay);

web_feature!(
    "XRRayDirectionInit",
    xr_ray_direction_init,
    "XRRayDirectionInit.rs",
    XRRayDirectionInit
);

web_feature!(
    "XRReferenceSpace",
    xr_reference_space,
    "XRReferenceSpace.rs",
    XRReferenceSpace
);

web_feature!(
    "XRReferenceSpaceEvent",
    xr_reference_space_event,
    "XRReferenceSpaceEvent.rs",
    XRReferenceSpaceEvent
);

web_feature!(
    "XRReferenceSpaceEventInit",
    xr_reference_space_event_init,
    "XRReferenceSpaceEventInit.rs",
    XRReferenceSpaceEventInit
);

web_feature!(
    "XRRenderState",
    xr_render_state,
    "XRRenderState.rs",
    XRRenderState
);

web_feature!(
    "XRRenderStateInit",
    xr_render_state_init,
    "XRRenderStateInit.rs",
    XRRenderStateInit
);

web_feature!(
    "XRRigidTransform",
    xr_rigid_transform,
    "XRRigidTransform.rs",
    XRRigidTransform
);

web_feature!("XRSession", xr_session, "XRSession.rs", XRSession);

web_feature!(
    "XRSessionEvent",
    xr_session_event,
    "XRSessionEvent.rs",
    XRSessionEvent
);

web_feature!(
    "XRSessionEventInit",
    xr_session_event_init,
    "XRSessionEventInit.rs",
    XRSessionEventInit
);

web_feature!(
    "XRSessionInit",
    xr_session_init,
    "XRSessionInit.rs",
    XRSessionInit
);

web_feature!(
    "XRSessionSupportedPermissionDescriptor",
    xr_session_supported_permission_descriptor,
    "XRSessionSupportedPermissionDescriptor.rs",
    XRSessionSupportedPermissionDescriptor
);

web_feature!("XRSpace", xr_space, "XRSpace.rs", XRSpace);

web_feature!("XRSubImage", xr_sub_image, "XRSubImage.rs", XRSubImage);

web_feature!("XRSystem", xr_system, "XRSystem.rs", XRSystem);

web_feature!(
    "XRTransientInputHitTestOptionsInit",
    xr_transient_input_hit_test_options_init,
    "XRTransientInputHitTestOptionsInit.rs",
    XRTransientInputHitTestOptionsInit
);

web_feature!(
    "XRTransientInputHitTestResult",
    xr_transient_input_hit_test_result,
    "XRTransientInputHitTestResult.rs",
    XRTransientInputHitTestResult
);

web_feature!(
    "XRTransientInputHitTestSource",
    xr_transient_input_hit_test_source,
    "XRTransientInputHitTestSource.rs",
    XRTransientInputHitTestSource
);

web_feature!("XRView", xr_view, "XRView.rs", XRView);

web_feature!(
    "XRViewerPose",
    xr_viewer_pose,
    "XRViewerPose.rs",
    XRViewerPose
);

web_feature!("XRViewport", xr_viewport, "XRViewport.rs", XRViewport);

web_feature!(
    "XRWebGLBinding",
    xr_web_gl_binding,
    "XRWebGLBinding.rs",
    XRWebGLBinding
);

web_feature!(
    "XRWebGLDepthInformation",
    xr_web_gl_depth_information,
    "XRWebGLDepthInformation.rs",
    XRWebGLDepthInformation
);

web_feature!(
    "XRWebGLLayer",
    xr_web_gl_layer,
    "XRWebGLLayer.rs",
    XRWebGLLayer
);

web_feature!(
    "XRWebGLLayerInit",
    xr_web_gl_layer_init,
    "XRWebGLLayerInit.rs",
    XRWebGLLayerInit
);

web_feature!(
    "XRWebGLSubImage",
    xr_web_gl_sub_image,
    "XRWebGLSubImage.rs",
    XRWebGLSubImage
);

web_feature!(
    "XSLTProcessor",
    xslt_processor,
    "XSLTProcessor.rs",
    XSLTProcessor
);

web_feature!("console", console, "console.rs", console);

web_feature!("enums", enums, "enums.rs", enums);

pub fn window() -> Window {
    Any::global("window").as_::<Window>()
}

pub use jsbind::prelude::atob;
pub use jsbind::prelude::btoa;
pub use jsbind::prelude::is_nan;
pub use jsbind::prelude::parse_float;
pub use jsbind::prelude::parse_int;
pub use jsbind::prelude::queue_microtask;
pub use jsbind::prelude::{JsStructuredSerializeOptions, structured_clone};

pub fn report_error(error: &jsbind::prelude::JsError) {
    Any::global("reportError").invoke(&[error.into()]);
}

pub fn caches() -> CacheStorage {
    Any::global("caches").as_()
}

pub fn cross_origin_isolated() -> bool {
    Any::global("crossOriginIsolated").as_::<bool>()
}

pub fn crypto() -> Crypto {
    Any::global("crypto").as_()
}

pub fn indexed_db() -> IDBFactory {
    Any::global("indexedDB").as_()
}

pub fn is_secure_context() -> bool {
    Any::global("isSecureContext").as_()
}

pub fn origin() -> jsbind::prelude::JsString {
    Any::global("origin").as_()
}

// #[cfg(feature = "Performance")]
pub fn performance() -> self::performance::Performance {
    Any::global("performance").as_()
}

pub fn create_image_bitmap0(
    image: &jsbind::prelude::Any,
) -> jsbind::prelude::Promise<Result<ImageBitmap, JsError>> {
    Any::global("createImageBitmap")
        .invoke(&[image.clone()])
        .as_()
}

pub fn create_image_bitmap1(
    image: &jsbind::prelude::Any,
    options: &jsbind::prelude::Object,
) -> jsbind::prelude::Promise<Result<ImageBitmap, JsError>> {
    Any::global("createImageBitmap")
        .invoke(&[image.clone(), options.into()])
        .as_()
}

pub fn create_image_bitmap2(
    image: &jsbind::prelude::Any,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
) -> jsbind::prelude::Promise<Result<ImageBitmap, JsError>> {
    Any::global("createImageBitmap")
        .invoke(&[image.clone(), sx.into(), sy.into(), sw.into(), sh.into()])
        .as_()
}

pub fn create_image_bitmap3(
    image: &jsbind::prelude::Any,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
    options: &jsbind::prelude::Object,
) -> jsbind::prelude::Promise<Result<ImageBitmap, JsError>> {
    Any::global("createImageBitmap")
        .invoke(&[
            image.clone(),
            sx.into(),
            sy.into(),
            sw.into(),
            sh.into(),
            options.into(),
        ])
        .as_()
}
