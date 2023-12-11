#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppGetFloat {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppSetFloat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppSetBlock {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppSetString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppDeleteAppData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppClearBlock {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppSetInt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppHasLinkedSocialClubAccount {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppGetString {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppDataValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppSaveData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppGetDeletedFileStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppHasSyncedData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppSetApp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppGetInt {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppCloseApp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAppCloseBlock {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartAudioScene {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnrequestTennisBanks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleAudioBodyDamageFactor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAmbientZoneEnabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleDefaultHorn {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPrepareSynchronizedAudioEventForScene {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioPositionAudioMute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnlockRadioStationTrackList {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanVehicleReceiveCbRadio {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleHornSoundIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartAlarm {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPortalSettingsOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInCurrentConversation {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsRadioFadedOut {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForcePedPanicWalla {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedFootstepsEventsEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptUpdateDoorAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetConversationAudioControlledByAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedVoiceGroupFromRaceToPvg {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleRadioOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseMissionAudioBank {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesPlayerVehHaveRadio {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMobileRadioEnabledDuringGameplay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnhintNamedScriptAudioBank {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAmbientZoneListState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAudioSpecialEffectMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideTrevorRage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedWallaDensity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddEntityToAudioMixGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGlobalRadioSignalLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearCustomRadioTrackList {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScriptedConversationOngoing {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveEntityFromAudioMixGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMissionCompletePlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnhintAmbientAudioBank {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUserRadioControlEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedRaceAndVoiceGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBlipSiren {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehRadioStation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableVehicleFanbeltDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientZoneStatePersistent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPrepareMusicEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedRingtonePlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadStream {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSirenWithNoDriver {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayStreamFromPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAmbientZoneState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAlarmPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartPreloadedConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartScriptPhoneConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedClothEventsEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnblockSpeechContextGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsRadioStationFavourited {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableVehicleExhaustPops {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioFrontendFadeTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioTrackWithStartOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAudioIsScriptedMusicPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNetworkIdFromSoundId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopAllAlarms {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestScriptAudioBank {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVariableOnStream {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnyPositionalSpeechPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPositionForNullConvPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFreezeRadioStation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentTrackSoundName {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayPedAmbientSpeechWithVoiceNative {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseSoundId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAggressiveHorns {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetStaticEmitterEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMusicVolSlider {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayVehicleDoorOpenSound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPreloadScriptConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutsceneAudioOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleRadioEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGpsActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideVehHorn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehHasNormalRadio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentTrackPlayTime {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHintMissionAudioBank {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedVoiceFull {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSoundId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSirenCanBeControlledByAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLockRadioStation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateUnlockableDjRadioTracks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentScriptedConversationLine {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesContextExistForThisPed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleBoostActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestTennisBanks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioStationAsFavourite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceMusicTrackList {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCustomRadioTrackList {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStreamPlayTime {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceUseAudioGameObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAudibleMusicTrackTextId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasLoadedMpDataSet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDistantCopCarSirens {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNextRadioTrack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleConversationsPersist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayStreamFrontend {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadStreamWithStartOffset {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleAudioEngineDamageFactor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCancelMusicEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasLoadedSpDataSet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaySoundFromEntityHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRefreshClosestOceanShoreline {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleAudiblyDamaged {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAmbientVoiceNameHash {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerVehRadioEnable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPreloadScriptPhoneConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetConversationAudioPlaceholder {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayVehicleDoorCloseSound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLinkStaticEmitterToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseAmbientAudioBank {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerSirenAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMissionNewsStoryUnlocked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaySoundFrontend {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartScriptConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopPedRingtone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientVoiceName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsGameInControlOfMusic {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSkipRadioForward {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMissionCompleteReadyForUi {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerVehicleAlarmAudioActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerMusicEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScriptOverridesWindElevation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnySpeechPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVariableOnUnderWaterStream {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestMissionAudioBank {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMobilePhoneCallOngoing {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSoundIdFromNetworkId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideMicrophoneSettings {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHornEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioStationMusicOnly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseNamedScriptAudioBank {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseScriptAudioBank {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCurrentPlayingSpeech {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedVoiceGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateSoundCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaySound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCutsceneAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAudioIsMusicPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseScriptedConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInitialPlayerStation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayStreamFromPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityForNullConvPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInterruptConversationAndPause {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaySynchronizedAudioEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedInteriorWallaDensity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFindRadioStationIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaySoundFromCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayPedAmbientSpeechNative {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHintAmbientAudioBank {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAmbientSpeechPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopSynchronizedAudioEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAmbientSpeechDisabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInitSynchSceneAudioWithEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedIsDrunk {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPedToConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSkipToNextScriptedConversationLine {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientZoneListState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleForceReverseWarning {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientVoiceNameHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnhintScriptAudioBank {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRestartScriptedConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleConversationsPersistNew {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHornPermanentlyOn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHornPermanentlyOnTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopPedSpeaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHornActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPrepareAlarm {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInterruptConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMusicOneshotPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsRadioRetuning {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopAlarm {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopSound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopStream {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedGender {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerRadioStationGenre {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAudioScriptCleanupTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioToStationIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnregisterScriptWithAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBlockSpeechContextGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisablePedPainAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVariationChosenForScriptedLine {
    pub success: bool,
    pub ret: i32,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopPedSpeakingSynced {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleDefaultHornIgnoreMods {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEmitterRadioStation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVariableOnSound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayMissionCompleteAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnlockMissionNewsStory {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRadioStationName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAllBrokenGlass {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMobilePhoneRadioActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioTrack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePortalSettingsOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCancelAllPoliceReports {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNoDuckingForConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAudioSceneActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMicrophonePosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayStreamFromVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableStuntJumpAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCurrentPlayingAmbientSpeech {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAudioFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopAudioScenes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleRadioLoud {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayPain {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVariableOnSynchSceneAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientZoneState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSkipMinigunSpinUpAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMobilePhoneRadioState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseFootstepScriptSweeteners {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableStallWarningSounds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehForcedRadioThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioAutoUnfreeze {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnimalVocalizationPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddLineToConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNextAudibleBeat {
    pub success: bool,
    pub ret: bool,
    pub out1_: f32,
pub out2_: f32,
pub out3_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayPedAmbientSpeechAndCloneNative {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioToStationName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterScriptWithAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPrepareSynchronizedAudioEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMobileInterferenceActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInitSynchSceneAudioWithPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPreloadVehicleAudioBank {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayDeferredSoundFrontend {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAnimalMood {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScriptedSpeechPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayEndCreditsMusic {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseWeaponAudio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivateAudioSlowmoMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsStreamPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateNewScriptedConversation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverridePlayerGroundMaterial {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetVehicleStartupRevSound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleHornSoundIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFreezeMicrophone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopScriptedConversation {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPositionedPlayerVehicleRadioEmitterEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioRetuneDown {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeactivateAudioSlowmoMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScriptedConversationLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopAudioScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayPoliceReport {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopSmokeGrenadeExplosionSounds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAudioVehiclePriority {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaySoundFromEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsPreloadedConversationReady {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetTrevorRage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMusicPlaytime {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerRadioStationIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerAngry {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayStreamFromObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayAmbientSpeechFromPositionNative {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayAnimalVocalization {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAudioSceneVariable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBlockDeathJingle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumUnlockedRadioStations {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleStartupRevSound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideUnderwaterStream {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleMissileWarningEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientZoneListStatePersistent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPedAudioFlags {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSirenBypassMpDriverCheck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerRadioStationName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFrontendRadioActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBlockAllSpeechFromPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayPedRingtone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseSirenAsHorn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHintScriptAudioBank {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRecordBrokenGlass {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnfreezeRadioStation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasSoundFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestAmbientAudioBank {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadioRetuneUp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLockRadioStationTrackList {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReactivateAllWorldBrainsThatAreWaitingTillOutOfRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterObjectScriptBrain {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableScriptBrainSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterWorldPointScriptBrain {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReactivateAllObjectBrainsThatAreWaitingTillOutOfRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddScriptToRandomPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableScriptBrainSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReactivateNamedWorldBrainsWaitingTillOutOfRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReactivateNamedObjectBrainsWaitingTillOutOfRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsWorldPointWithinBrainActivationRange {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsObjectWithinBrainActivationRange {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsGameplayCamShaking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayCamMaxMotionBlurStrengthThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCamSplinePaused {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCamRendering {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCamInterpolating {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamDofStrength {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRenderScriptCams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddCamSplineNodeUsingCameraFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceVehicleCamStuntSettingsThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFirstPersonAimCamNearClipThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopGameplayCamShaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddCamSplineNodeUsingCamera {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFirstPersonShooterCameraHeading {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFirstPersonFlashEffectVehicleModelHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutsceneCamFarClipThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamSplineDuration {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHardAttachCamToPedBone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGameplayCamCoord {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFinalRenderedCamMotionBlurStrength {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamUseShallowDofMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableCinematicSlowMoThisUpdate {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayEntityHint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFollowVehicleCamZoomLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamActiveViewModeContext {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableAimCamThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDebugName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopScriptGlobalShaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsInVehicleMobilePhoneCameraRendering {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHardAttachCamToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFirstPersonFlashEffectVehicleModelName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCinematicCamShaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamSplinePhase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCodeGameplayHint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamFarDof {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFinalRenderedRemotePlayerCamRot {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowMotionBlurDecay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseScriptCamForAmbientPopulationOriginThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceBonnetCameraRelativeHeadingAndPitch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamViewModeForContext {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayCamIgnoreEntityCollisionThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayPedHint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDofMaxNearInFocusDistanceBlendLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFirstPersonAimCamRelativeHeadingLimitsThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsInterpolatingFromScriptCams {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCutsceneCamShaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFollowPedCamZoomLevel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceTightspaceCustomFramingThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsGameplayCamRendering {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGameplayCamRelativePitch {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDofPlanes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamNearDof {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllowCustomVehicleDriveByCamThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideCamSplineVelocity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamAnimCurrentPhase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetThirdPersonAimCamNearClipThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseDedicatedStuntCameraThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFollowPedCamThisUpdate {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamInheritRollVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptedCameraIsFirstPersonThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDofFocalLengthMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceCameraRelativeHeadingAndPitch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreWidescreenBordersActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayCamMotionBlurScalingThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableCamCollisionForObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCinematicFirstPersonVehicleInteriorCamRendering {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFlyCamHorizontalResponse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayHintFov {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCinematicButtonActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRenderingCam {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPointCamAtEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableFirstPersonFlashEffectThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableNearClipScanThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFollowPedCamViewMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScreenFadedIn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFinalRenderedCamRot {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFirstPersonFlashEffectType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWasFlyCamConstrainedOnPreviousUdpate {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScreenFadingIn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFollowVehicleCamSeatThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayHintCameraRelativeSideOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerVehiclePartBrokenCameraShake {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsFirstPersonAimCamActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateCamera {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDofStrength {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFinalRenderedRemotePlayerCamFov {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddCamSplineNodeUsingGameplayFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachCamToPedBone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCameraPreventCollisionSettingsForTripleheadInInteriorsThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableCinematicVehicleIdleModeThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseVehicleCamStuntSettingsThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGameplayCamFov {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamControlsMiniMapHeading {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPointCamAtPedBone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAimCamActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShakeCam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateCameraWithParams {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCamShaking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayCamRelativePitch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamMotionBlurStrength {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsInterpolatingToScriptCams {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFirstPersonAimCamZoomFactor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsGameplayCamLookingBehind {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetGameplayCamFullAttachParentTransformTimer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUseHiDofOnSyncedSceneThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateCinematicShot {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGameplayCamRelativeHeading {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAimCamActiveInAccurateMode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFirstPersonShooterCameraPitch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCinematicShot {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDebugCam {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScreenFadingOut {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTableGamesCameraThisUpdate {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIgnoreMenuPreferenceForBonnetCameraThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamSplineNodeExtraFlags {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamRot {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideCamSplineMotionBlur {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDofFnumberOfLens {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFirstPersonAimCamZoomFactor {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDeathFailEffectState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFinalRenderedCamFov {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGameplayCamRot {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamSplineNodeEase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayObjectHint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamRot {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddCamSplineNode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDestroyCam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoScreenFadeOut {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFocusPedOnScreen {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayCamFollowPedThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplayGetMaxDistanceAllowedFromPlayer {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamAffectsAiming {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFollowPedCamViewMode {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachCamToVehicleBone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDestroyAllCams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetThirdPersonCamRelativeHeadingLimitsThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFollowVehicleCamHighAngleModeThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFinalRenderedCamFarDof {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayCamAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFollowVehicleCamHighAngleModeEveryUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInvalidateCinematicVehicleIdleMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBlockFirstPersonOrientationResetThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamActiveWithInterp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFinalRenderedCamNearDof {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamAnimCurrentPhase {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUseHiDof {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFinalRenderedCamCoord {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayVehicleHint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAnimatedShakeCam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamIsInsideVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachCam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceCinematicRenderingThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFollowVehicleCamViewMode {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetThirdPersonCamRelativePitchLimitsThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamSplineNodeVelocityScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBypassCameraCollisionBuoyancyTestThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesCamExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayCamShakeAmplitude {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceCamFarClip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFollowVehicleCamViewMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableCinematicBonnetCameraThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamFarClip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableFirstPersonCameraWaterClippingTestThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamFov {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCinematicCamRendering {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScreenFadedOut {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamSplineNodeIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayCamRelativeHeading {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateCamWithParams {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamSplinePhase {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamFarClip {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamCoord {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCinematicCamShaking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFirstPersonAimCamRelativePitchLimitsThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCamShaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCodeGameplayHintActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamParams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamNearDof {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAnimatedShakeScriptGlobal {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamFov {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDofMaxNearInFocusDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateCam {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamNearClip {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDofFocusDistanceBias {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsFollowPedCamActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCinematicCamShakeAmplitude {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamNearClip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopRenderingScriptCamsUsingCatchUp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFollowPedCamLadderAlignThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFlyCamVerticalControlsThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCamPlayingAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScriptGlobalShaking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFlyCamCoordAndConstrain {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayHintCameraRelativeVerticalOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCinematicIdleCamRendering {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsFollowVehicleCamActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCinematicShotActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopGameplayHintBeingCancelledThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFirstPersonAimCamZoomFactorLimitsThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFinalRenderedCamNearClip {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamSplineSmoothingStyle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayHintBaseOrbitPitchOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoScreenFadeIn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayCoordHint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsBonnetCinematicCamRendering {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamShakeAmplitude {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamSplineNodePhase {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBypassCutsceneCamRenderingThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayCamAltitudeFovScalingState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCinematicNewsChannelActiveThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWidescreenBorders {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShakeCinematicCam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCinematicModeActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFollowCamIgnoreAttachParentMovementThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableOnFootFirstPersonViewThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetThirdPersonCamOrbitDistanceLimitsThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCamActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFinalRenderedCamFarClip {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDofOverriddenFocusDistanceBlendLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaySynchronizedCamAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSphereVisible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayHintCameraBlendToFollowPedMediumViewMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsGameplayHintActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFlyCamVerticalResponse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInVehicleCamStateThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableGameplayCamAltitudeFovScalingThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAllowedIndependentCameraModes {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamFarDof {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCamViewModeForContext {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFollowVehicleCamZoomLevel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCamPointing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopGameplayHint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShakeScriptGlobal {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInvalidateIdleCam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCamDofOverriddenFocusDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCinematicCamInputActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPointCamAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayHintFollowDistanceScalar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFlyCamMaxHeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGameplayCamEntityToLimitFocusOverBoundingSphereThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShakeGameplayCam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachCamToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClockMinutes {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClockHours {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMillisecondsPerGameMinute {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClockDayOfMonth {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseClock {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetClockTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClockSeconds {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLocalTime {
    pub success: bool,
    pub ret: (),
    pub year_: i32,
pub month_: i32,
pub day_: i32,
pub hour_: i32,
pub minute_: i32,
pub second_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetUtcTime {
    pub success: bool,
    pub ret: (),
    pub year_: i32,
pub month_: i32,
pub day_: i32,
pub hour_: i32,
pub minute_: i32,
pub second_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClockYear {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetClockDate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClockMonth {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAdvanceClockTimeTo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddToClockTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClockDayOfWeek {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPosixTime {
    pub success: bool,
    pub ret: (),
    pub year_: i32,
pub month_: i32,
pub day_: i32,
pub hour_: i32,
pub minute_: i32,
pub second_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutsceneOriginAndOrientation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutscenePedPropVariation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestCutFile {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutsceneMultiheadFadeManual {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityIndexOfCutsceneEntity {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCutFileConcatCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartCutsceneAtCoords {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutsceneMultiheadFade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCanDisplayMinimapDuringCutsceneThisUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasThisCutsceneLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutscenePedComponentVariationFromPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetMocapCutsceneCanBeSkipped {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWasCutsceneSkipped {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutsceneCanBeSkipped {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCutsceneSectionPlaying {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesCutsceneEntityExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutsceneEntityStreamingFlags {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanSetExitStateForRegisteredEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCutsceneAuthorized {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesCutsceneHandleExist {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCutsceneConcatSectionPlaying {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCutscenePlayDuration {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanUseMobilePhoneDuringCutscene {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanSetEnterStateForRegisteredEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasCutsceneCutThisFrame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCutscenePlaybackFlagSet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasCutsceneFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleModelPlayerWillExitScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutsceneFadeValues {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptCanStartCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCutsceneEndTime {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutsceneTriggerArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCutsceneActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMultiheadFadeUp {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasCutFileLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanSetExitStateForCamera {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanRequestAssetsForCutsceneEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutsceneOrigin {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutscenePedComponentVariation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityIndexOfRegisteredEntity {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestCutsceneWithPlaybackList {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasCutsceneLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPadCanShakeDuringCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveCutFile {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopCutsceneImmediately {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCutscenePlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCarGeneratorsCanUpdateDuringCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterEntityForCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCutsceneTime {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCutsceneTotalDuration {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileSelectCreatorStats {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictGetType {
    pub success: bool,
    pub ret: i32,
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayGetCount {
    pub success: bool,
    pub ret: i32,
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictGetFloat {
    pub success: bool,
    pub ret: f32,
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictGetBool {
    pub success: bool,
    pub ret: bool,
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileHasLoadedFileData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileSelectActiveFile {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileStoreMissionHeader {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayAddString {
    pub success: bool,
    pub ret: (),
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictSetBool {
    pub success: bool,
    pub ret: (),
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayGetType {
    pub success: bool,
    pub ret: i32,
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictGetString {
    pub success: bool,
    pub ret: Option<String>,
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayGetInt {
    pub success: bool,
    pub ret: i32,
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayAddVector {
    pub success: bool,
    pub ret: (),
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcUpdateMission {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictGetVector {
    pub success: bool,
    pub ret: shared::Vector3,
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictSetVector {
    pub success: bool,
    pub ret: (),
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileUpdateSaveToCloud {
    pub success: bool,
    pub ret: bool,
    pub p0_: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayGetBool {
    pub success: bool,
    pub ret: bool,
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileSelectUgcPlayerData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayAddFloat {
    pub success: bool,
    pub ret: (),
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictCreateArray {
    pub success: bool,
    pub ret: i32,
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcUpdateContent {
    pub success: bool,
    pub ret: bool,
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayAddDict {
    pub success: bool,
    pub ret: i32,
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcSetPlayerData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileDeleteForAdditionalDataFile {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileClearWatchList {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictGetInt {
    pub success: bool,
    pub ret: i32,
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictGetArray {
    pub success: bool,
    pub ret: i32,
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileStartSaveToCloud {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayGetDict {
    pub success: bool,
    pub ret: i32,
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayGetVector {
    pub success: bool,
    pub ret: shared::Vector3,
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileDeleteRequestedFile {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictSetString {
    pub success: bool,
    pub ret: (),
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileGetFileDict {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileDelete {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileSelectUgcStats {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictCreateDict {
    pub success: bool,
    pub ret: i32,
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcCreateMission {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileSelectUgcData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileLoadOfflineUgcForAdditionalDataFile {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileWatchRequestId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictGetDict {
    pub success: bool,
    pub ret: i32,
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileIsSavePending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayGetFloat {
    pub success: bool,
    pub ret: f32,
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictSetFloat {
    pub success: bool,
    pub ret: (),
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileLoadOfflineUgc {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileFlushMissionHeader {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcCreateContent {
    pub success: bool,
    pub ret: bool,
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayAddInt {
    pub success: bool,
    pub ret: (),
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileCreate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayGetString {
    pub success: bool,
    pub ret: Option<String>,
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileGetFileDictForAdditionalDataFile {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatadictSetInt {
    pub success: bool,
    pub ret: (),
    pub object_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDataarrayAddBool {
    pub success: bool,
    pub ret: (),
    pub array_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileHasValidFileData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDatafileIsValidRequestId {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorRemove {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorExistOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorSetInt {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorSetFloat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorIsRegisteredAsType {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorGetFloat {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorSetBool {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorSetTime {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorRegister {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorGetInt {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorRegisterLock {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDecorGetBool {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOnEnterMp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsLoadingScreenActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreAnyCcsPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasCloudRequestsFinished {
    pub success: bool,
    pub ret: bool,
    pub p0_: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDlcPresent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEverHadBadPackOrder {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetExtracontentCloudResult {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDlcCheckCompatPackConfiguration {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsInitialLoadingScreenActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOnEnterSp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDlcCheckCloudDataCorrect {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasEntityClearLosToEntityInFront {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveModelSwap {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedIndexFromEntityIndex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesEntityHaveDrawable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCoords {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFindAnimEventPhase {
    pub success: bool,
    pub ret: bool,
    pub p3_: shared::MemoryBufferId,
pub p4_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityRecordsCollisions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityForwardVector {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAmissionEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityLoadCollisionFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityMaxSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityTouchingModel {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopSynchronizedMapEntityAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityStatic {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateForcedObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityMaxHealth {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityMaxHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityDynamic {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCanBeDamaged {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityTouchingEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetOffsetFromEntityInWorldCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfApplyForceToEntityCenterOfMass {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityUseMaxDistanceForWaterReflection {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCollision {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityVelocity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityUpsidedown {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityHeightAboveGround {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityPlayingAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityOfTypeAttachedToEntity {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAtCoord {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasEntityAnimFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityRotationVelocity {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesEntityHaveAnimDirector {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetOffsetFromEntityGivenWorldCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCoordsNoOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAsNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    pub ped_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAttachedToAnyVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopEntityAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityAnimSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityMotionBlur {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityNoweapondecals {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityAnimCurrentTime {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCanOnlyBeDamagedByScriptParticipants {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllowMigrateToSpectator {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityInvincible {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityShouldFreezeWaitingOnCollision {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasEntityClearLosToEntityAdjustForCover {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateModelHideExcludingScriptObjects {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectAsNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    pub object_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceEntityAiAndAnimationUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityLodDist {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFreezeEntityPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopSynchronizedEntityAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityAnimCurrentTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityAlpha {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWorldPositionOfEntityBone {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityBonePostion {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityVisible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityVelocity {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityAttachedTo {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPickupEntityGlow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityHasGravity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleIndexFromEntityIndex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNearestPlayerToEntityOnTeam {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityAnimTotalTime {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityInAngledArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAped {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityUpright {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityTrafficlightOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityLodDist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityAlpha {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityHeight {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntitySortBias {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLastMaterialHitByEntity {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachEntityBoneToEntityBone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityDead {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasEntityBeenDamagedByAnyPed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveForcedObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCoordsWithoutPlantsReset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleAsNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    pub vehicle_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCantCauseCollisionDamagedEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityRequiresMoreExpensiveRiverCheck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAvehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachEntityToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableEntityBulletCollision {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityOnlyDamagedByRelationshipGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNearestPlayerToEntity {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesEntityExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityRenderScorched {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAtEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesEntityHaveSkeleton {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityQuaternion {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityIsInVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityOnlyDamagedByPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityQuaternion {
    pub success: bool,
    pub ret: (),
    pub x_: f32,
pub y_: f32,
pub z_: f32,
pub w_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityLights {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayEntityAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityRoll {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityAngularVelocity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityHeadingFromEulers {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityRotation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityForwardY {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityInAir {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateModelHide {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasEntityCollidedWithAnything {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityForwardX {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAnObject {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityHeading {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateModelSwap {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasEntityBeenDamagedByAnyObject {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityUprightValue {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntitySpeedVector {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetEntityAlpha {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCompletelyDisableCollision {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityModel {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityNoCollisionEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityScript {
    pub success: bool,
    pub ret: Option<String>,
    pub script_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearEntityLastDamageEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLastEntityHitByEntity {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCanClimbOnEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityAlwaysPrerender {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityAsMissionEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteEntity {
    pub success: bool,
    pub ret: (),
    pub entity_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityRotation {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAttachedToAnyPed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCanOnlyBeDamagedByEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityBoneCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAttached {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityInZone {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityAsNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    pub entity_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaySynchronizedMapEntityAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityBoneObjectRotation {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityProofs {
    pub success: bool,
    pub ret: bool,
    pub bullet_proof_: bool,
pub fire_proof_: bool,
pub explosion_proof_: bool,
pub collision_proof_: bool,
pub melee_proof_: bool,
pub steam_proof_: bool,
pub p7_: bool,
pub drown_proof_: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityWaterReflectionFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachEntityToEntityPhysically {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfApplyForceToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaySynchronizedEntityAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasEntityBeenDamagedByEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityCollisionDisabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityBoneRotation {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupCollidesWithProjectiles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityBoneObjectPostion {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAttachedToAnyObject {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityInWater {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityWaitingForWorldCollision {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCanBeTargetedWithoutLos {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityPitch {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntitySpeed {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityVisibleToScript {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickUpByCargobobDisabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetObjectIndexFromEntityIndex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityCanBeDamaged {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveModelHide {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesEntityHavePhysics {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWaitForCollisionsBeforeProbe {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesEntityBelongToThisScript {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasEntityBeenDamagedByAnyVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCanAutoVaultOnEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityCanBeDamagedByRelationshipGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityOccluded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCollisionNormalOfLastHitForEntity {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityOnScreen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityMirrorReflectionFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntitySubmergedLevel {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityHeading {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasCollisionLoadedAroundEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityIsTargetPriority {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityVisible {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasAnimEventFired {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityMatrix {
    pub success: bool,
    pub ret: (),
    pub forward_vector_: shared::Vector3,
pub right_vector_: shared::Vector3,
pub up_vector_: shared::Vector3,
pub position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWouldEntityBeOccluded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityHealth {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAttachedToEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfProcessEntityAttachments {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityPopulationType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityProofs {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityBoneIndexByName {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasEntityClearLosToEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachEntityBoneToEntityBoneYforward {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAnimDuration {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsShockingEventInSphere {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveShockingEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressShockingEventsNextFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveShockingEventSpawnBlockingAreas {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressShockingEventTypeNextFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearDecisionMakerEventResponse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressAgitationEventsNextFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddShockingEventForEntity {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDecisionMaker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnblockDecisionMakerEvent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddShockingEventAtPosition {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBlockDecisionMakerEvent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveAllShockingEvents {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedApparelForcedPropCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHashNameForComponent {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDlcVehicleMod {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedOutfitLocate {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTattooShopDlcItemIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedOutfitComponentVariant {
    pub success: bool,
    pub ret: bool,
    pub out_component_variant_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInitShopPedComponent {
    pub success: bool,
    pub ret: (),
    pub out_component_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedQueryComponent {
    pub success: bool,
    pub ret: (),
    pub out_component_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumTattooShopDlcItems {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDlcWeaponDataSp {
    pub success: bool,
    pub ret: bool,
    pub out_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDlcWeaponComponentDataSp {
    pub success: bool,
    pub ret: bool,
    pub component_data_ptr_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDlcVehicleData {
    pub success: bool,
    pub ret: bool,
    pub out_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesShopPedApparelHaveRestrictionTag {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRevertContentChangesetGroupForAll {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumDlcWeaponComponents {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumDlcWeaponsSp {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetupShopPedApparelQuery {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDlcVehicleFlags {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedProp {
    pub success: bool,
    pub ret: (),
    pub out_prop_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHashNameForProp {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfExecuteContentChangesetGroupForAll {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetForcedComponent {
    pub success: bool,
    pub ret: (),
    pub name_hash_: u32,
pub enum_value_: i32,
pub component_type_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedQueryPropIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDlcWeaponComponentData {
    pub success: bool,
    pub ret: bool,
    pub component_data_ptr_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedQueryOutfit {
    pub success: bool,
    pub ret: (),
    pub outfit_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVariantComponent {
    pub success: bool,
    pub ret: (),
    pub name_hash_: u32,
pub enum_value_: i32,
pub component_type_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedComponent {
    pub success: bool,
    pub ret: (),
    pub out_component_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesCurrentPedComponentHaveRestrictionTag {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDlcWeaponData {
    pub success: bool,
    pub ret: bool,
    pub out_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedQueryComponentIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetupShopPedApparelQueryTu {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumDlcVehicles {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedOutfitPropVariant {
    pub success: bool,
    pub ret: bool,
    pub out_prop_variant_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumDlcWeaponComponentsSp {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedOutfit {
    pub success: bool,
    pub ret: (),
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDlcVehicleModLockHash {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedApparelVariantComponentCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedApparelForcedComponentCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedApparelVariantPropCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsContentItemLocked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesCurrentPedPropHaveRestrictionTag {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVariantProp {
    pub success: bool,
    pub ret: (),
    pub name_hash_: u32,
pub enum_value_: i32,
pub anchor_point_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShopPedQueryProp {
    pub success: bool,
    pub ret: (),
    pub out_prop_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetForcedProp {
    pub success: bool,
    pub ret: (),
    pub name_hash_: u32,
pub enum_value_: i32,
pub anchor_point_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInitShopPedProp {
    pub success: bool,
    pub ret: (),
    pub out_prop_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDlcVehicleModel {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumDlcWeapons {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetupShopPedOutfitQuery {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTattooShopDlcItemData {
    pub success: bool,
    pub ret: bool,
    pub out_component_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopFireInRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetOwnerOfExplosionInAngledArea {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddOwnedExplosion {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityOnFire {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsExplosionInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClosestFirePos {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddExplosionWithUserVfx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfFiresInRange {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsExplosionActiveInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartScriptFire {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopEntityFire {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveScriptFire {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFlammabilityMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsExplosionInAngledArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsExplosionInSphere {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetOwnerOfExplosionInSphere {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddExplosion {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartEntityFire {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateCheckpoint {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasStreamedTextureDictLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfProcgrassDisableAmbscalescan {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableScuffDecals {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsSetDynamicDepthValue {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartParticleFxNonLoopedOnEntityBone {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRenderShadowedLightsWithNoShadows {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsInitSession {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseBinkMovie {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesVehicleHaveCrewEmblem {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAnimpostfxStop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailSetFacing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawDebugBox {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndPetrolTrailDecals {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFreeMemoryForMissionCreatorPhoto {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFlash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTvshowCurrentlyPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsEnableFreezer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStatusOfSaveHighQualityPhoto {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasScaleformMovieFilenameLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughSetMaxThickness {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBinkMovieTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartParticleFxNonLoopedOnEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStatusOfTakeHighQualityPhoto {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawScaleformMovieFullscreen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableCompositeShotgunDecals {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartParticleFxNonLoopedOnPedBone {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearTimecycleModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawLowQualityPhotoToPhone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughSetColorNear {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUi3dsceneMakePushedPresetPersistent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTvAudioFrontend {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestScaleformMovie {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetArtificialLightsState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddOilDecal {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailSetColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfProcgrassEnableAmbscalescan {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAllTcmodifierOverrides {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideInteriorSmokeLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableProcobjCreation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughSetHighlightNoise {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTrackedPointInfo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStatusOfLoadMissionCreatorPhoto {
    pub success: bool,
    pub ret: i32,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterPostfxBulletImpact {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDebugLinesAndSpheresDrawingActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNightvision {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughSetHilightIntensity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTcmodifierOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartParticleFxLoopedOnEntity {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTakenPhotoIsMugshot {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTerraingridSetParams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTransitionOutOfTimecycleModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawScaleformMovie3dSolid {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScaleformMovieAsNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    pub scaleform_handle_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityIconColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTakeMissionCreatorPhoto {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxNonLoopedEmitterSize {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlaylistOnChannel {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTvVolume {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTvChannelPlaylistAtHour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetUsingnightvision {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAnimpostfxPlay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDontRenderInGameUi {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBackfaceculling {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailSetRadius {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartParticleFxNonLoopedAtCoord {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsSetBoundPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsSetScreenSizeCheckEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxNonLoopedColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckpointCylinderHeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsClearShadowSampleType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisablePetrolDecalsRecyclingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxBulletImpactScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPhonephotoeditorSetFrameTxd {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawMarker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawTexturedPoly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTvVolume {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearParticleFxShootoutBoat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideInteriorSmokeName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfQueueOperationToCreateSortedListOfPhotos {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxSlipstreamLodrangeScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawSpriteNamedRendertarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableMoonCycleOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableMoonCycleOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfProcgrassIsCullsphereEnabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTimecycleModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawSpriteArx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScaleformMovieMethodReturnValueInt {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsPetrolDecalInRange {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsActiveScaleformMovieDeleting {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGrassbatchDisableFlattening {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentTvClipNamehash {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsWidescreen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailSetPath {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDecalWashLevel {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScaleformMovieToUseLargeRt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBinkMovie {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFreeMemoryForMissionCreatorPhotoPreview {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaximumNumberOfPhotos {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScreenCoordFromWorldCoord {
    pub success: bool,
    pub ret: bool,
    pub screen_x_: f32,
pub screen_y_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTextureResolution {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRequestingnightvision {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandScaleformString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableOcclusionThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAnimpostfxIsRunning {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsSetSplitZexpWeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawDebugText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLightOverrideMaxIntensityScale {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawRect {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTransitionTimecycleModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckpointDirection {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPopTimecycleModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSaveHighQualityPhoto {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLoadHighQualityPhotoStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddVehicleCrewEmblem {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughGetMaxThickness {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideNightvisionLightRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckpointInsideCylinderScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetUsingseethrough {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimecycleTransitionModifierIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDecalBulletImpactRangeScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentNumberOfCloudPhotos {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadMissionCreatorPhoto {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearStatusOfSortedListOperation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckpointInsideCylinderHeightScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableDecalRenderingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseSnowWheelVfxWhenUnsheltered {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPetrolDecal {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetExtraTcmodifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCallScaleformMovieMethodWithString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawScaleformMovie {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxBangScrapeLodrangeScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPushTimecycleModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStatusOfCreateMissionCreatorPhotoPreview {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWashDecalsFromVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawShadowedSpotLight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScreenblurFadeCurrentTime {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTerraingridSetColours {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveDecalsInRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSkidmarkRangeScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableProcobjCreation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesLatestBriefStringExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsSetEntityTrackerScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxLoopedEvolution {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsSetCascadeBoundsScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableDownwashPtfx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckpointDecalRotAlignedToCameraRot {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptGfxDrawOrder {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveGrassCullSphere {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopBinkMovie {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfProcgrassDisableCullsphere {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestScaleformMovieWithIgnoreSuperWidescreen {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBinkShouldSkip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFreeMemoryForLowQualityPhoto {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForceMotionblur {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawLine {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseParticleFxAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScaleformMovieToUseSystemTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGrassbatchEnableFlatteningInSphere {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScriptGfxAlignPosition {
    pub success: bool,
    pub ret: (),
    pub calculated_x_: f32,
pub calculated_y_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsSetAircraftMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartNetworkedParticleFxLoopedOnEntity {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughReset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayBinkMovie {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawBinkMovie {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckpointRgba {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxLoopedAlpha {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawTexturedPolyWithThreeColours {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawDebugCross {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesParticleFxLoopedExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableMovieKeyframeWait {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginCreateLowQualityCopyOfPhoto {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScaleformMovieMethodReturnValueReady {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxNonLoopedAlpha {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScaleformMovieMethodAddParamLiteralString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawMarkerSphere {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUi3dsceneClearPatchedData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPhonephotoeditorToggle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScreenblurFadeRunning {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSeethrough {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxLoopedColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginCreateMissionCreatorPhotoPreview {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawDebugLine {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandScaleformString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsEnableEntityTracker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceExposureReadback {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasScaleformContainerMovieLoadedIntoParent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAbortVehicleCrewEmblemRequest {
    pub success: bool,
    pub ret: bool,
    pub p0_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTimecycleModifierStrength {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachTvAudioToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfMoveVehicleDecals {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsHidef {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGrabPausemenuOwnership {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasScaleformMovieLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScaleformMovieDeleting {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetActualScreenResolution {
    pub success: bool,
    pub ret: (),
    pub x_: i32,
pub y_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableMovieSubtitles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawScaleformMovie3d {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScreenResolution {
    pub success: bool,
    pub ret: (),
    pub x_: i32,
pub y_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetParticleFxOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPatchDecalDiffuseMap {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxForceVehicleInterior {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBinkMovieTime {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopParticleFxLooped {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxBloodScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStatusOfTakeMissionCreatorPhoto {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearExtraTcmodifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestScaleformScriptHudMovie {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxFootLodrangeScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawSpriteArxWithUv {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverridePedCrewLogoTexture {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLightOverrideMaxIntensityScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPetrolTrailDecalInfo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxShootoutBoat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginScaleformScriptHudMovieMethod {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUi3dsceneAssignPedToSlot {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsTimecycleTransitioningOut {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestEarlyLightCheck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartPetrolTrailDecals {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForcePostfxBulletImpactsAfterHud {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfQueryMovieMeshSetState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWashDecalsInRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddEntityIcon {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailSetShaderParams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughSetFadeEnddistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableAlienBloodVfx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerScreenblurFadeIn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTerraingridActivate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawDebugText2d {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartNetworkedParticleFxNonLoopedOnPedBone {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterNoirLensEffect {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailGetVisualControlPoint {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeatherPtfxUseOverrideSettings {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailGetMaxHeight {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailSetEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTakeHighQualityPhoto {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveDecalsFromObjectFacing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughSetFadeStartdistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDrawOrigin {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawDebugSphere {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGrassbatchEnableFlatteningExtInSphere {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginScaleformMovieMethodOnFrontend {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawPoly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxCamInsideNonplayerVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetOnIslandXforTakenPhoto {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandUnparsedScaleformString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfProcgrassEnableCullsphere {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseSnowFootVfxWhenUnsheltered {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBinkMovieVolume {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsSetShadowSampleType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailSetFixedControlPoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDestroyTrackedPoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScreenAspectRatio {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddDecal {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMotionblurMaxVelScaler {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxLoopedScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAnimpostfxStopAll {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLockAdaptiveDofDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadMovieMeshSet {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnpatchDecalDiffuseMap {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxNonLoopedScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptGfxAlign {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveParticleFxFromEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginScaleformMovieMethodOnFrontendHeader {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckpointRgba2 {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceParticleFxInVehicleInterior {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxFootOverrideName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHidofOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScaleformMovieMethodAddParamTextureNameString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTvChannel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSafeZoneSize {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetExtraTcmodifier {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxBulletImpactLodrangeScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCurrentPlayerTcmodifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPhonephotoeditorIsActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestScaleformMovieSkipRenderWhilePaused {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerTcmodifierTransition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGrassCullSphere {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetStreamedTextureDictAsNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearTvChannelPlaylist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNextPlayerTcmodifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailSetFixedControlPointEnable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableHdtexThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScaleformMovieMethodAddParamInt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveParticleFx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTrackedPointVisible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndScaleformMovieMethodReturnValue {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestScaleformMovieInstance {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScaleformMovieMethodAddParamBool {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDepthwriting {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptGfxDrawBehindPausemenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPtfxForceVehicleInteriorFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndScaleformMovieMethod {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDecalAlive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartParticleFxLoopedOnEntityBone {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartNetworkedParticleFxNonLoopedOnEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFadeUpPedLight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableVehicleDistantlights {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterReflectionSetScriptObjectVisibility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxBulletTraceNoAngleReject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNoisinessoveride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStatusOfCreateLowQualityCopyOfPhoto {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveDecalsFromObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawScaleformMovieFullscreenMasked {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableInWaterPtfx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCallScaleformMovieMethodWithNumber {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawSpotLight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTvPlayerWatchingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPassKeyboardInputToScaleform {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAnimpostfxStopAndFlushRequests {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveVehicleCrewEmblem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsSetCascadeBounds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCascadeShadowsSetDynamicDepthMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUi3dsceneIsAvailable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawBox {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScaleformMovieMethodAddParamFloat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPresetInteriorAmbientCache {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFadeDecalsInRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughSetHeatscale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFreeMemoryForHighQualityPhoto {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScaleformMovieMethodReturnValueBool {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableClownBloodVfx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawDebugLineWithTwoColours {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisablePetrolDecalsIgnitingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckpointForceDirection {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGolfTrailSetTessellation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceRenderInGameUi {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaximumNumberOfCloudPhotos {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxLoopedFarClipDist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveParticleFxInRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartNetworkedParticleFxLoopedOnEntityBone {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableScreenblurFade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateLightsOnEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasScaleformScriptHudMovieLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestStreamedTextureDict {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTogglePausedRenderphases {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityIconVisibility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartParticleFxLoopedAtCoord {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPausedRenderphases {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScaleformMovieMethodReturnValueString {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDistanceBlurStrengthOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetArtificialVehicleLightsState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateTrackedPoint {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAnimpostfxGetCurrentTime {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetScriptGfxAlign {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetAdaptation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMotionblurMaxVelScaler {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTogglePlayerDamageOverlay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScaleformMovieToUseSuperLargeRt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNoiseoveride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesThisPhotoSlotContainAvalidPhoto {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawSprite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawMarkerEx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScaleformMovieMethodAddParamPlayerNameString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveDecalsFromVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseMovieMeshSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTogglePausedRenderphasesStatus {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScaleformMovieMethodAddParamLatestBriefString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadHighQualityPhoto {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveDecal {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxCamInsideVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetExposuretweak {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCallScaleformMovieMethodWithNumberAndString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAdjustNextPosSizeAsNormalized169 {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerScreenblurFadeOut {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideInteriorSmokeEnd {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableRegionVfx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAspectRatio {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUi3dscenePushPreset {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartParticleFxLoopedOnPedBone {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawLightWithRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetArenaThemeAndVariationForTakenPhoto {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveScaleformScriptHudMovie {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawLightWithRangeex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckpointClipplaneWithPosNorm {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartNetworkedParticleFxNonLoopedAtCoord {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptGfxAlignParams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStatusOfSortedListOperation {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteCheckpoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginScaleformMovieMethod {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeatherPtfxOverrideCurrLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTvChannelPlaylist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParticleFxLoopedOffsets {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBinkMovieAudioFrontend {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCallScaleformMovieMethod {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTvChannel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckpointForceOldArrowPointing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawTvChannel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimecycleModifierIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleCrewEmblemRequestState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughSetNoiseMax {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearDrawOrigin {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSeethroughSetNoiseMin {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipRotation {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentWebpageId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasAdditionalTextLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFlashAbilityBar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextProportional {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentInteger {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDirectorModeAvailable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMenuPedBoolStat {
    pub success: bool,
    pub ret: bool,
    pub out_value_: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMaxArmourHudDisplay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfChangeFakeMpCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinimapFowRevealCoordinate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadarZoom {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScriptedHudComponentHiddenThisFrame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandIsThisHelpMessageBeingDisplayed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHudSuppressWeaponWheelResultsThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowHudComponentThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAiBlipForcedOn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWarningMessageOptionItems {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseVehicleTargetingReticule {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSavegameListUniqueId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentFormattedInteger {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRestartFrontendMenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandIsThisHelpMessageBeingDisplayed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBusyspinnerOff {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentSubstringTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceSonarBlipsThisFrame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNamedRendertargetLinked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextInputBoxEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetReticuleValues {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLockMinimapPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipNameToPlayerName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipShowCone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostCrewtagWithGameName {
    pub success: bool,
    pub ret: i32,
    pub p2_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuGetMouseHoverUniqueId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseControlOfFrontend {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipSecondaryColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHudShowingCharacterSwitchSelection {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNextBlipInfoId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagsPointHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsRadarHidden {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWarningMessageWithHeaderAndSubstringFlagsExtended {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesPedHaveAiBlip {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedShow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCharacterFromAudioConversationFilename {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSecondScriptVariableHudColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseFakeMpCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentSubstringTextLabelHashKey {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedSetRgbaParameterForNextMessage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedSetFlashDurationParameterForNextMessage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWaypointBlipEnumId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHudPreferenceSwitchedOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowForSaleIconOnBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinimapInSpectatorMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNamedRendertargetRenderId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloseMpTextChat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFirstBlipInfoId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPauseMenuRestarting {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesTextBlockExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextDropShadow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplaceHudColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostMessagetext {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfMpTextChatDisable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsWaypointActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAbilityBarVisibility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipInfoIdDisplay {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostMessagetextTu {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRaceTrackRender {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCustomMinimapSetBlipObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipSprite {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandThefeedPost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideMinimapInteriorMapThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawFrontendBackgroundThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasScriptHiddenHelpThisFrame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceScriptedGfxWhenFrontendActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearGpsFlags {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHudComponentPosition {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentFrontendMenuVersion {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBigmapActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipCategory {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandDisplayHelp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowFriendIndicatorOnBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandAddDirectlyToPreviousBriefs {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsFloatingHelpTextOnScreen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideHudmarkersThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCharacterMenuPedMaskedIntStat {
    pub success: bool,
    pub ret: bool,
    pub out_value_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipAsMissionCreatorBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextOutline {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadiusBlipEdge {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedHideThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandDisplayText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDirectorModeLaunchedByScript {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMissionCreatorBlip {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCustomMinimapClearBlips {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPauseMenuState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayAreaName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForceShowGps {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReloadMapMenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLockMinimapAngle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAdditionalText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuIsContextMenuActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCustomMpHudColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipFade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipAsMinimalOnEdge {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostStats {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipFadeDirection {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipUseHeightIndicatorOnEdge {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearSmallPrints {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseToggleFullscreenMap {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasMenuLayoutChangedEventOccurred {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipFlashesAlternate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostTicker {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsStorePendingNetworkShutdownToOpen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPointToGpsCustomRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagHealthBarColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveMpGamerTag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedUpdateItemTexture {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGpsFlashes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHelpMessageFadingOut {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedHide {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostUnlock {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuGetMouseHoverIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveFakeConeData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinimapGolfCourseOff {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMenuTriggerEventDetails {
    pub success: bool,
    pub ret: (),
    pub last_item_menu_id_: i32,
pub selected_item_unique_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostTickerWithTokens {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWarningMessageWithHeaderExtended {
    pub success: bool,
    pub ret: (),
    pub p6_: shared::MemoryBufferId,
pub p7_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetColourOfNextTextComponent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsFrontendReadyForControl {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPauseMenuPedLighting {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartGpsMultiRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMouseRolledOverInstructionalButtons {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGpsMultiRouteRender {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnlockMinimapPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAiBlipHasCone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNorthBlidIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHealthHudDisplayValues {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayHudWhenPausedThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPmWarningscreenActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHoveringOverMissionCreatorBlip {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLengthOfLiteralStringInBytes {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextEdge {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuDeactivateContext {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostTickerForced {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetHudComponentValues {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipAlpha {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextDropshadow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddBlipForRadius {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHudForceSpecialVehicleWeaponWheel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuRedrawInstructionalButtons {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedSetVibrateParameterForNextMessage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStandardBlipEnumId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideLoadingOnFadeThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipShortHeightThreshold {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipInfoIdEntityIndex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHelpMessageBeingDisplayed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextJustification {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNavigatingMenuContent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMpGamerTagActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowScriptedHudComponentThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearFloatingHelp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInsideVerySmallInterior {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandGetNumberOfLinesForString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOpenReportugcMenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDefaultScriptRendertargetRenderId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostMessagetextWithCrewTagAndAdditionalIcon {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideNumberOnBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCustomMinimapSetActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipHiddenOnLegend {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandGetScreenWidthOfDisplayText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCustomMinimapCreateBlip {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedSetScriptedMenuHeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDontZoomMinimapWhenSnipingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAiPedVehicleBlipIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedAutoPostGametipsOn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllowCommaOnTextInput {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceNextMessageToPreviousBriefsList {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterNamedRendertarget {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedForceRenderOff {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinimapBlockWaypoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScreenCodeWantsScriptToControl {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMpGamerTagFree {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadarAsInteriorThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddBlipForCoord {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGpsFlags {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPauseMenuPosition {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNewSelectedMissionCreatorBlip {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostMessagetextWithCrewTag {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddBlipForEntity {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUseIslandMap {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedInPauseMenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextRenderId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMissionName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentSubstringKeyboardDisplay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideMinimapExteriorMapThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowHeadingIndicatorOnBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMenuPedFloatStat {
    pub success: bool,
    pub ret: bool,
    pub out_value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestAdditionalTextForDlc {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddNextMessageToPreviousBriefs {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowSonarBlips {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowAccountPicker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAllHelpMessages {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinimapFowDoNotUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextWrap {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMouseEvent {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId,
pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagVisibility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextFont {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCodeWantsScriptToTakeControl {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearGpsMultiRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideHudComponentThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideMpTextChatTeamString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfToggleStealthRadar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFlashMinimapDisplayWithColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextRightJustify {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinimapSonarSweep {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentSubstringPlayerName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUseSetDestinationInPauseMap {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDontTiltMinimapThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableFrontendThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateMpGamerTagWithCrewColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMpGamerTagMovieActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMinimapFowCoordinateIsRevealed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveWarningMessageOptionItems {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedOnlyShowTooltips {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipAsFriendly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsOnlinePoliciesMenuActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWarningMessageWithHeaderAndSubstringFlags {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideHudAndRadarThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestAdditionalText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinimapGolfCourse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipHudColour {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHudSetWeaponWheelTopSlot {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerSonarBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplaySniperScopeThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPulseBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowTickOnBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFrontendActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowHeightOnBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinimapComponent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOpenSocialClubMenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentSubstringPhoneNumber {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayHudWhenNotInStateOfPlayThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFloatingHelpTextScreenPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearDynamicPauseMenuErrorMessage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFakePausemapPlayerPositionThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuceptionGoDeeper {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFloatingHelpTextWorldPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFloatingHelpTextStyle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNamedRendertargetRegistered {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMessageBeingDisplayed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearGpsRaceTrack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostUnlockTuWithColor {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWarningMessage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerIconColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFilenameForAudioConversation {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagBigText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfMpTextChatIsTeamJob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHudColour {
    pub success: bool,
    pub ret: (),
    pub r_: i32,
pub g_: i32,
pub b_: i32,
pub a_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAiPedPedBlipIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMenuLayoutChangedEventDetails {
    pub success: bool,
    pub ret: (),
    pub last_item_menu_id_: i32,
pub selected_item_menu_id_: i32,
pub selected_item_unique_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInsideVeryLargeInterior {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsImeInProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLengthOfStringWithThisTextLabel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOpenOnlinePoliciesMenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerIsInDirectorMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentSubstringBlipName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedClearFrozenPost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDescriptionForUgcMissionEightStrings {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnlockMinimapAngle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWarningScreenMessageHash {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRefreshWaypoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedGetLastShownPhoneActivatableFeedId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayPlayerNameTagsOnBlips {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipRouteColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearFakeConeArray {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuIsContextActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandDisplayHelp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandIsMessageDisplayed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandGetScreenWidthOfDisplayText {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveBlip {
    pub success: bool,
    pub ret: (),
    pub blip_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceCloseTextInputBox {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllowAbilityBar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDontZoomMinimapWhenRunningThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandIsMessageDisplayed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsStreamingAdditionalText {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMouseCursorStyle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearHelp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostCrewRankupWithLiteralFlag {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCharacterMenuPedFloatStat {
    pub success: bool,
    pub ret: bool,
    pub out_value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandOverrideButtonText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGpsCustomRouteRender {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipDisplay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandGetNumberOfLinesForString {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPausemapInInteriorMode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMenuPedMaskedIntStat {
    pub success: bool,
    pub ret: bool,
    pub out_value_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinimapInPrologue {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsReportugcMenuOpen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisablePausemenuSpinner {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedSetBackgroundColorForNextPost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentSubstringWebsite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveMultiplayerWalletCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayHelpTextThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveMultiplayerHudCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipAlpha {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMaxHealthHudDisplay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAiBlipNoticeRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostCrewtag {
    pub success: bool,
    pub ret: i32,
    pub p2_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentWebsiteId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMouseCursorVisible {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFirstNcharactersOfLiteralString {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAbilityBarValue {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfActiveBlips {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipInfoIdPickupIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagNumPackages {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearBrief {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandPrint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSocialClubTour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsRadarPreferenceSwitchedOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCopBlipSprite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayRadar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedForceRenderOn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHudGetWeaponWheelTopSlot {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFakeGpsPlayerPositionThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFlashWantedDisplay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPmPlayerCrewColor {
    pub success: bool,
    pub ret: bool,
    pub r_: i32,
pub g_: i32,
pub b_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasDirectorModeBeenLaunchedByCode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowNumberOnBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHudGetWeaponWheelCurrentlyHighlighted {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideStreetAndCarNamesThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextLeading {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsBlipFlashing {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayAmmoThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayHud {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagsShouldUseVehicleHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesBlipExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWaypointOff {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHudHidden {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandOverrideButtonText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipRotationWithFloat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedFlushQueue {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPointToGpsMultiRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedIsPaused {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostAward {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipFlashInterval {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHudComponentPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMouseCursorThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandBusyspinnerOn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesTextLabelExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGivePedToPauseMenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSubtitlePreferenceSwitchedOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasThisAdditionalTextLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedAutoPostGametipsOff {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipCoords {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipPriority {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsWarningMessageReadyForControl {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMinimapRendering {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPauseMenuActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFloatingHelpTextToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMpTextChatTyping {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHasAiBlipWithColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipFlashes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipBright {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCharacterFromAudioConversationFilenameWithByteLimit {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBusyspinnerIsDisplaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipMarkerLongDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearReminderMessage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostVersusTu {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedReportLogoOff {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCopBlipSpriteAsStandard {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowOutlineIndicatorOnBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandPrint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetGlobalActionscriptFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHelpMessageStyle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressFrontendRenderingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceOffWantedStarFlash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedSetSnapFeedItemPositions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandSetBlipName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHudComponentActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipFromEntity {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadarZoomPrecise {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandBusyspinnerOn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddBlipForPickup {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedRemoveItem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipAsShortRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipInfoIdType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDrawHudOverFadeThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateFakeMpGamerTag {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTextCentre {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMultiplayerWalletCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFakeSpectatorMode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWidescreenFormat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSocialClubActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipExtendedHeightThreshold {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveCopBlipFromPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentSubstringTextLabel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPreloadBusyspinner {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostMessagetextSubtitleLabel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowContactInstructionalButton {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuSetBusySpinner {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveMultiplayerBankCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuGetMouseClickEvent {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostUnlockTu {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCharacterMenuPedIntStat {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowGoldTickOnBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadarZoomToDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPrints {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowPauseWhenNotInStateOfPlayThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandDisplayText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipScale2d {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFakeSpectatorMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuceptionTheKick {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddBlipForArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCharacterFromAudioConversationFilenameBytes {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFlagPlayerContextInTournament {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagWantedLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearThisPrint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandAddDirectlyToPreviousBriefs {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStreetNameFromHashKey {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAllBlipRoutes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlockWantedFlash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFakeMinimapMaxAltimeterHeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostReplay {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateRadarZoomToBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagsShouldUsePointsHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloseSocialClubMenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHasAiBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipFlashTimer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBusyspinnerIsOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedReportLogoOn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideHelpTextThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClosestBlipInfoId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagAlpha {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptVariableHudColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteWaypointsFromThisPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsBlipShortRange {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHelpMessageOnScreen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWarningMessageOptionHighlight {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartGpsCustomRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRenderedCharacterHeight {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWarningMessageWithHeader {
    pub success: bool,
    pub ret: (),
    pub show_background_: shared::MemoryBufferId,
pub p7_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMainPlayerBlipId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowCrewIndicatorOnBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScriptedHudComponentActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMultiplayerBankCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesBlipHaveGpsRoute {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuActivateContext {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostReplayInput {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuGetHairColourIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMpGamerTagName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPauseMenuActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipColour {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipSprite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMinimapFowDiscoveryRatio {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandClearPrint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsWarningMessageActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedResume {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipHighDetail {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideScriptedHudComponentThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGlobalActionscriptFlag {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsBlipOnMinimap {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMissionNameForUgcMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddValidVehicleHitHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAiBlipGangId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowDisplayOfMultiplayerCashText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearGpsCustomRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTextComponentFloat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadarAsExteriorThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseNamedRendertarget {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipNameFromTextFile {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHudForceWeaponWheel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsUpdatingMpGamerTagNameAndCrewDetails {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearValidVehicleHitHashes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTakeControlOfFrontend {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPauseMenuPedSleepState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceCloseReportugcMenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllMpGamerTagsVisibility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivateFrontendMenu {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMenuPedIntStat {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandThefeedPostMpticker {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLengthOfLiteralString {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseMenuSetWarnOnTabChange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreOnlinePoliciesUpToDate {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShowStartMissionInstructionalButton {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasMenuTriggerEventOccurred {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFlashMinimapDisplay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplaceHudColourWithRgba {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideMpTextChatColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLinkNamedRendertarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetupFakeConeData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlipRotation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinimapHideFow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginTextCommandSetBlipName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRadarZoomToBlip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHudScreenPositionFromWorldPosition {
    pub success: bool,
    pub ret: i32,
    pub screen_x_: f32,
pub screen_y_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBlipInfoIdCoord {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndTextCommandClearPrint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAiBlipSprite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMultiplayerHudCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedPause {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedResetAllParameters {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfThefeedFreezeNextPost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNewWaypoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearGpsPlayerWaypoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetInteriorAtCoordsWithType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetInteriorFromEntity {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearRoomForGameViewport {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetInteriorLocationAndNamehash {
    pub success: bool,
    pub ret: (),
    pub position_: shared::Vector3,
pub name_hash_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnpinInterior {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsValidInterior {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPinInteriorInMemory {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsInteriorEntitySetActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceActivatingTrackingOnEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetKeyForEntityInRoom {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPickupToInteriorRoomByName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRoomForGameViewportByKey {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRefreshInterior {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeactivateInteriorEntitySet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRoomKeyFromEntity {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivateInteriorGroupsUsingCamera {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInteriorInUse {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableShadowCullModelThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceRoomForEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivateInteriorEntitySet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableInterior {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsInteriorReady {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIsExteriorOnly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableStadiumProbesThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRetainEntityInInterior {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearInteriorStateOfEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceRoomForGameViewport {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsInteriorCapped {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetOffsetFromInteriorInWorldCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableMetroSystem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRoomKeyForGameViewport {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableExteriorCullModelThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRoomForGameViewportByName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetInteriorAtCoords {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearRoomForEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsInteriorDisabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsInteriorScene {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInteriorEntitySetTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCapInterior {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetInteriorGroupId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetInteriorFromPrimaryView {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetInteriorFromCollision {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCollisionMarkedOutside {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetInteriorAtCoordsWithTypehash {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetInteriorHeading {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveFromItemset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsInItemset {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateItemset {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCleanItemset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIndexedItemInItemset {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsItemsetValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetItemsetSize {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDestroyItemset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddToItemset {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLobbyAutoMultiplayerEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLobbySetAutoMultiplayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLobbyAutoMultiplayerRandomJob {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLobbySetAutoMpRandomJob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLobbyAutoMultiplayerFreemode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLobbyAutoMultiplayerMenu {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShutdownSessionClearsAutoMultiplayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLobbySetAutoMultiplayerEvent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentLanguage {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLocalizationGetSystemLanguage {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLocalizationGetSystemDateType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayOnscreenKeyboard {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAreaOfVehicles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCloudSettingsOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetModelDimensions {
    pub success: bool,
    pub ret: (),
    pub minimum_: shared::Vector3,
pub maximum_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopSaveArray {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAreaOfCops {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateIncidentWithEntity {
    pub success: bool,
    pub ret: bool,
    pub out_incident_i_d_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearCodeRequestedAutosave {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasCheatWithHashBeenActivated {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHaveCreditsReachedEnd {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAreaOfProjectiles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSteamVersion {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScriptRaceInit {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreStringsEqual {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateOnscreenKeyboard {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearWeatherTypeNowPersistNetwork {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterEnumToSave {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideSaveHouse {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetOverrideWeatherex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFireAmmoThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPreloadCloudHat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAngledAreaOfVehicles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPopMultiplierAreaNetworked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesPopMultiplierAreaExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsXboxPlatform {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFakeWantedLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasAsyncInstallFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFrameTime {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesPopMultiplierSphereExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTennisSwingAnimComplete {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAngleBetween2dVectors {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTennisSwingAnimCanBeInterrupted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMinigameInProgress {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsBulletInAngledArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddStuntJump {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearReplayStats {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBenchmarkPass {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScriptRacePlayerHitCheckpoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAcos {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTimeScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFadeOutAfterArrest {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCompareStrings {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSupressRandomEventThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWindDirection {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddHospitalRestart {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScriptRaceShutdown {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCloudsAlpha {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasCodeRequestedAutosave {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCopyScriptStruct {
    pub success: bool,
    pub ret: (),
    pub dst_: shared::MemoryBufferId,
pub src_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClosestPointOnLine {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIgnoreNextRestart {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsStuntJumpMessageShowing {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableScreenDimmingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisablePoliceRestart {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetContentToLoad {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRiotModeEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveDispatchSpawnBlockingArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableTennisMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeatherTypeNow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMinigameInProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStatusOfMissionRepeatSave {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetReplayStatMissionType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPauseDeathArrestRestart {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddDispatchSpawnSphereBlockingArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsProjectileTypeInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNextWeatherType {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHeadingFromVector2d {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivityFeedAddSubstringToCaption {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRandomFloatInRange {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetOceannoiseminamplitude {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPopMultiplierSphere {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearOverrideWeather {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsProjectileTypeWithinDistance {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterIntToSave {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSaveEndUserBenchmark {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAreaOccupiedSlow {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStatusOfManualSave {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLandingScreenStartedEndUserBenchmark {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCoordsOfProjectileTypeInAngledArea {
    pub success: bool,
    pub ret: bool,
    pub position_out_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNextOnscreenKeyboardResultWillDisplayUsingTheseFonts {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsBulletInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRealWorldTime {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateIncident {
    pub success: bool,
    pub ret: bool,
    pub out_incident_i_d_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetOceanwaveamplitude {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetEndUserBenchmark {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBeastJumpThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRandomSeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfQueueMissionRepeatSave {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPrevWeatherType {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDispatchTimeBetweenSpawnAttempts {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPoliceRestart {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBenchmarkIterations {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDispatchTimeBetweenSpawnAttemptsMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPcVersion {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterTextLabel23ToSave {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFadeOutAfterDeath {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetContentIdIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFakeWantedLevel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDurangoVersion {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivityFeedCreate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerIsInAnimalForm {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSaveHouse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoAutoSave {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsProjectileInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBitsInRange {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTennisMoveNetworkSignalFloat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteIncident {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPcCheatWithHashBeenActivated {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPrevWeatherTypeHashName {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGamePaused {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCurrWeatherState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSuperJumpThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetDispatchSpawnLocation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCancelOnscreenKeyboard {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStringToInt {
    pub success: bool,
    pub ret: bool,
    pub out_integer_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreProfileSettingsValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetReplayStatMissionId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTennisMode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartSaveArrayWithSize {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSniperInverted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetRippleminbumpiness {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTan {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRain {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptHighPrio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPopMultiplierArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlmIsInConstrainedMode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTotalSuccessfulStuntJumps {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFindSpawnPointInDirection {
    pub success: bool,
    pub ret: bool,
    pub spawn_point_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAutoSaveInProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseActiveCameraForTimeslicingCentre {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddReplayStatValue {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsAutoSaveOff {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetThisIsAtriggerScript {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterTextLabel15ToSave {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasGameInstalledThisSession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDispatchIdealSpawnDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivityFeedActionStartWithCommandLineAdd {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeatherTypePersist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRestartCoordOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNextWeatherTypeHashName {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfQueueMissionRepeatLoad {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAbsf {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGravityLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPopulateNow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopSaveData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetDispatchIdealSpawnDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsStuntJumpInProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisAminigameScript {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetRipplebumpiness {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterFloatToSave {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsFrontendFading {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearScenarioSpawnHistory {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSnow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRatioOfClosestPointOnLine {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsProsperoVersion {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetReplayStatAtIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterTextLabel31ToSave {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetProjectileOfProjectileTypeWithinDistance {
    pub success: bool,
    pub ret: bool,
    pub out_coords_: shared::Vector3,
pub out_projectile_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetOnscreenKeyboardResult {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShootSingleBulletBetweenCoords {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAtan2 {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivityFeedPost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMemoryCardInUse {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRandomWeatherType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAllocatedStackSize {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGroundZandNormalFor3dCoord {
    pub success: bool,
    pub ret: bool,
    pub ground_z_: f32,
pub normal_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInformCodeOfContentIdOfCurrentUgcMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCoordsOfProjectileTypeInArea {
    pub success: bool,
    pub ret: bool,
    pub projectile_pos_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBitsInRange {
    pub success: bool,
    pub ret: (),
    pub unk_var_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScriptRaceGetPlayerSplitTime {
    pub success: bool,
    pub ret: bool,
    pub p1_: i32,
pub p2_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayTennisDiveAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivityFeedLargeImageUrl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddDispatchSpawnAngledBlockingArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetScriptIsSafeForNetworkGame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartEndUserBenchmark {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBit {
    pub success: bool,
    pub ret: (),
    pub address_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAreaLeaveVehicleHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnloadAllCloudHats {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRainLevel {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsPlayerInAnimalForm {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRandomEventFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivityFeedAddIntToCaption {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasBulletImpactedInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumSuccessfulStuntJumps {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBlockDispatchServiceResourceCreation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInstancePriorityMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGameTimer {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerIsRepeatingAmission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTerminateAllScriptsWithThisName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGroundZexcludingObjectsFor3dCoord {
    pub success: bool,
    pub ret: bool,
    pub ground_z_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAussieVersion {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetRipplemaxbumpiness {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCommandlineEndUserBenchmark {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSizeOfSaveData {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPointAreaOverlap {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForcedJumpThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndReplayStats {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearRestartCoordOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMissionFlag {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetOverrideWeather {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSaveHouseDetailsAfterSuccessfulLoad {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::Vector3,
pub p1_: f32,
pub fade_in_after_load_: bool,
pub p3_: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableStuntJumpSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAreaOccupied {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetExplosiveAmmoThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActionManagerEnableAction {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsOrbisVersion {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterInt64ToSave {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnloadCloudHat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetShorewavemaxamplitude {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideFadeIn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWindSpeed {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartSaveData {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAtan {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetThisScriptCanBePaused {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlmGetConstrainedDurationMs {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWind {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetDispatchSpawnBlockingAreas {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPositionOccupied {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIncidentRequestedUnits {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePopMultiplierArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBaseElementLocationFromMetadataBlock {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearTacticalNavMeshPoints {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetOceanwavemaxamplitude {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCreditsFadeOutWithScreen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddTacticalNavMeshPoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsJapaneseVersion {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetShorewaveamplitude {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCreditsActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetThisScriptCanRemoveBlipsCreatedByAnyScript {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetRippledisturb {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivityFeedOnlinePlayedWithPost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddStuntJumpAngled {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAreaOfPeds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartSaveStructWithSize {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShootSingleBulletBetweenCoordsIgnoreEntityNew {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterSaveHouse {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceGameStatePlaying {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideFadeOut {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetShorewaveminamplitude {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMissionFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetProfileSetting {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScarlettVersion {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetStrength {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSnowLevel {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInstancePriorityHint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCleanupAsyncInstall {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopEndUserBenchmark {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAsin {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableHospitalRestart {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsIncidentValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterBoolToSave {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGroundZfor3dCoord {
    pub success: bool,
    pub ret: bool,
    pub ground_z_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSaveMenuActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsStringNullOrEmpty {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplayOnscreenKeyboardWithLongerInitialString {
    pub success: bool,
    pub ret: (),
    pub p2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIndexOfCurrentLevel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPs3Version {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearWeatherTypePersist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCityDensity {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDispatchSpawnLocation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreCreditsRunning {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHashKey {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIdealSpawnDistanceForIncident {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRandomEventFlag {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShouldUseMetricMeasurements {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRandomIntInRange {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHaveReplayStatsBeenStored {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetStuntJumpsCanTrigger {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetWantedResponseNumPedsToSpawn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFadeInAfterDeathArrest {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableDispatchService {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteStuntJump {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasBulletImpactedInBox {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetReplayStatCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAreaOfObjects {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsBulletInBox {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowMissionCreatorWarp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCoordsOfProjectileTypeWithinDistance {
    pub success: bool,
    pub ret: bool,
    pub out_coords_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginReplayStats {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayTennisSwingAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableStuntJumpSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShootSingleBulletBetweenCoordsIgnoreEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPreventArrestStateThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWantedResponseNumPedsToSpawn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPointObscuredByAmissionEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRestartGame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSystemTimeStep {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePopMultiplierSphere {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCancelStuntJump {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearBit {
    pub success: bool,
    pub ret: (),
    pub address_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasResumedFromSuspend {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTennisSwingAnimSwung {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUiStartedEndUserBenchmark {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivityFeedActionStartWithCommandLine {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWindDirection {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopSaveStruct {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfQueueMissionRepeatSaveForBenchmarkTest {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetDispatchTimeBetweenSpawnAttempts {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberResourcesAllocatedToWantedLevel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfQuitGame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivityFeedAddLiteralSubstringToCaption {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetContentIdIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeatherTypeNowPersist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterTextLabelToSave {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWindSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsProjectileTypeInAngledArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAbsi {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUsingMissionCreator {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDistanceBetweenCoords {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsStringNull {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRandomMwcIntInRange {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSlerpNearQuaternion {
    pub success: bool,
    pub ret: (),
    pub out_x_: f32,
pub out_y_: f32,
pub out_z_: f32,
pub out_w_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCloudsAlpha {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrWeatherState {
    pub success: bool,
    pub ret: (),
    pub weather_type1_: u32,
pub weather_type2_: u32,
pub percent_weather2_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFadeInAfterLoad {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLinePlaneIntersection {
    pub success: bool,
    pub ret: bool,
    pub p12_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceLightningFlash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsXbox360Version {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaterOverrideSetOceanwaveminamplitude {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScePlatform {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideFreezeFlags {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterTextLabel63ToSave {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTickerJohnmarstonIsDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeatherTypeOvertimePersist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfToggleShowOptionalStuntJumpCamera {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadCloudHat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFrameCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfFreeStacksOfThisSize {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSniperBulletInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetExplosiveMeleeThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamActivateSelfieMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamSetSelfieModeRollOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamSetSelfieModeSideOffsetScaling {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMobilePhoneRotation {
    pub success: bool,
    pub ret: (),
    pub rotation_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamSetSelfieModeVertPanOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMobilePhoneDofState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDestroyMobilePhone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamIsCharVisibleNoFaceCheck {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellHorizontalModeToggle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamSetSelfieModeHeadPitchOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamSetSelfieModeHorzPanOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMobilePhonePosition {
    pub success: bool,
    pub ret: (),
    pub position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMobilePhonePosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellSetInput {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamActivateShallowDofMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateMobilePhone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamSetSelfieModeDistanceScaling {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMobilePhoneRenderId {
    pub success: bool,
    pub ret: (),
    pub render_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMobilePhoneRotation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanPhoneBeSeenOnScreen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMobilePhoneScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamSetSelfieModeHeadYawOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamSetSelfieModeHeadRollOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScriptIsMovingMobilePhoneOffscreen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCellCamActivate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPayBusinessSupplies {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnWagePaymentBonus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromRockstar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendVehicleRequested {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeArena {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDeleteCharacter {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentOrderWarehouseVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCasinoTimeTrialWin {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendGoon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnBoss {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetCanTransferCash {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCasinoMissionParticipation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromBusinessHubSell {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpradeBunker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAgency {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBuyTiltrotor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPaServiceSnack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnChallenge {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnSellBase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnArenaCareerProgression {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPaHeliPickup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeAcidLabEquipment {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPaServiceHeli {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFirstTimeBonus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendApartmentUtilities {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnDoomsdayFinaleBonus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBuyBunker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromBounty {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnSourceParticipationAcidLab {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPvcTransferBalance {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendCarClubMembership {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAwardContract {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentArenaJoinSpectator {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCasinoCanBet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnGangopsWagesBonus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeTiltrotor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentTaxi {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDeductCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendAgency {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkServiceEarnGangopsRivalDelivery {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBountyHunterMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnGangopsFinale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanShareJobCash {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBetting {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeNightclubAndWarehouse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromFmbbBossWork {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfProcessCashGift {
    pub success: bool,
    pub ret: Option<String>,
    pub p0_: i32,
pub p1_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeHackerTruck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFixerRivalDelivery {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnGangopsElite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentCashDrop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentJobSkip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeAcidLabScoop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBounty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendGangopsRepairCost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPurchaseHackerTruck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeOfficeGarage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromChallengeWin {
    pub success: bool,
    pub ret: (),
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnBikerShop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendGunrunning {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnGangopsWages {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentTradeImpexpWarehouseProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBuyContrabandMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromHangarTrade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPurchaseImpexpWarehouseProperty {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBuyCasino {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnWagePayment {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeTruck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAutoshopBusiness {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAgencyContract {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCasinoCanBetPvc {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanBet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCasinoBuyChips {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkInitializeCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeBase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromWarehouse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCasinoHeistAwards {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromPersonalVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBuyArena {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnJuggaloStoryMissionParticipation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeImpexpWarehouseProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromBusinessBattle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromNotBadsport {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendArcade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendRenameAcidProduct {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnSightseeingReward {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromHoldups {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeCasino {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendCasinoHeistSkipMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeAcidLabMines {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnUpgradeArcade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBuyBase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPvcBalance {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnDailyObjectiveEvent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromAiTargetKill {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBoatPickup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnDailyVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBeachParty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCollectablesActionFigures {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeArcade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnPurchaseClubHouse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCasinoMissionReward {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkReceivePlayerJobshareCash {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkYohanSourceGoods {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAwardShortTrip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnTargetRefund {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentEmployAssassins {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentJukebox {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCollectableCompletedCollection {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetVcBalance {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanReceivePlayerCash {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetEvcBalance {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBallisticEquipment {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnArenaWarAssassinateTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendGangopsTripSkip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentChangeAppearance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPayEmployeeWage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromBendJob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeHangar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendArenaPremium {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendSuvFstTrvl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFixerPrep {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnArenaWar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBuyProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendNightclubAndWarehouse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAgencySafe {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeBusinessProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnSpinTheWheelCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromJobBonus {
    pub success: bool,
    pub ret: (),
    pub p1_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPurchaseOfficeProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentFromRockstar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentCinema {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromSmugglerWork {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendSubmarine {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeAgency {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBuyPassiveMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBuyRevealPlayers {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromDailyObjectives {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWasVcWithdrawalSuccessful {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPurchaseBusinessProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetStringBankWalletBalance {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendArenaSpectatorBox {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromCashingOut {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnBiker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCasinoHeist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanSpendMoney2 {
    pub success: bool,
    pub ret: bool,
    pub p4_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAwardPhone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBuySmokes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBuyAirstrike {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetVcBankBalance {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendGangopsCannon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnSellAcid {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnRdrBonus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBuyBounty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentHeliPickup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEconomyHasFixedCrazyNumbers {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPlayerHealthcare {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendSetDiscount {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentTelescope {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentArrestBail {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkMoneyCanBet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBuyHeliStrike {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentRequestJob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkManualDeleteCharacter {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromBetting {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCasinoCollectableCompletedCollection {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendRenameAcidLab {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromDestroyingContraband {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCollectableItem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromBusinessProduct {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBuyArcade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentNightclubEntryFee {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendCasinoGeneric {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAwardFixerMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeSub {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCasinoCanBuyChipsPvc {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBuyFairgroundRide {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnSellPrizeVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeOfficeProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBuySub {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBikeShop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromSellBunker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendCarClubBar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBusinessPropertyFees {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPayMatchEntryFee {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendInteractionMenuAbility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentCargoSourcing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCasinoAward {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanPayAmountToBoss {
    pub success: bool,
    pub ret: bool,
    pub p3_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnTaxiJob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentRobbedByMugger {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPurchaseClubHouse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendSpinTheWheelPayment {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendEarnedFromBankAndWallets {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendAutoshopModify {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnDailyStashHouseParticipation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentRequestHeist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnHeistAward {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPayVehicleInsurancePremium {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromGangattackPickup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendGangopsStartStrand {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBuyMfgarage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetCanSpendFromBank {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBuyBackupGang {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetVcWalletBalance {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnBeachPartyLostFound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentTradeBusinessProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBuyOfftheradar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBullShark {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetStringBankBalance {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentVehicleExportMods {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromClubManagementParticipation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeAcidLabArmor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnStreetDealer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnArenaWarEventCargo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAwardTaxi {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnGangopsSetup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearCharacterWallet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCollectables {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeWarehouseProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnGangopsAward {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanSpendMoney {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBuyTruck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCasinoStoryMissionReward {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentCallPlayer {
    pub success: bool,
    pub ret: (),
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentAmmoDrop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentHangarUtilityCharges {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromCrateDrop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentHangarStaffCharges {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentProstitutes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromJob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBuySupplies {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPaServiceDancer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPurchaseOfficeGarage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnNightclubDancing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendArcadeMgmt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnTunerAward {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendSetCommonFields {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFixerFinale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnCarclubMembership {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnTunerRobbery {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPurchaseWarehouseProperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAwardRandomEvent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendHidden {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnUpgradeAutoshop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentImportExportRepair {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAwardDailyStash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFooliganJobParticipation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromFmbbPhonecallMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAutoshopIncome {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnNightclubAndWarehouse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromPremiumJob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendCasinoClub {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBankInterest {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnSellParticipationAcidLab {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnDailyStashHouseCompleted {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnDarChallenge {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPurchaseHangar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDeferCashTransactionsUntilShopSave {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentMoveSubmarine {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnGoon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFooliganJob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeMfgarage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAwardDeadDrop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnUpgradeAgency {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAwardAcidLab {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendCarClubTakeover {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnIslandHeist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnArcade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendCasinoHeist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentNoCops {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentMcAbility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendCompSuv {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentWager {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBuyHealthcare {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentHoldups {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendSourceBike {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendGangopsSkipMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBoss {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetCanSpendFromBankAndWallet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendNightclubBarDrink {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendUpgradeAutoshop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAwardJuggaloMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnYatchMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromJobx2 {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnSmugglerAgency {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnRcTimeTrial {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnJuggaloStoryMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnArenaSkillLevelProgression {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnNightclub {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasVcWithdrawalCompleted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBuyWantedlevel {
    pub success: bool,
    pub ret: (),
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPaServiceImpound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDepositVc {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentRdrHatchetBonus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAgencyPhone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromIslandHeistDjMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnDispatchCall {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnSetupParticipationAcidLab {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentHireMugger {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendMakeItRain {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentHireMercenary {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentMoveYacht {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendIslandHeist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnHackerTruck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentOrderBodyguardVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetRemainingTransferBalance {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBuyAgency {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendPlayArcade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendSupply {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentCarwash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromContraband {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromPickup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnGangopsPrepParticipation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCasinoSellChips {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetCanSpendFromWallet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPaServiceVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentPurchaseNightclubAndWarehouse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromVehicleExport {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentInStripclub {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBuyAutoshop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBuyItem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendBuyAcidLab {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFixerAgencyShortTrip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromRobArmoredCars {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCasinoCanBetAmount {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnBountyHunterReward {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentRehireDj {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWithdrawVc {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromImportExport {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetStringWalletBalance {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRefundCash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromCriminalMastermind {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentUpgradeClubHouse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnAssassinateTargetKilled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGivePlayerJobshareCash {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPlayerIsHighEarner {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFromAmbientJob {
    pub success: bool,
    pub ret: (),
    pub p2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpendCasinoMembership {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentRenameOrganization {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnBbEventBonus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnDailyVehicleBonus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSpentBossGoon {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEarnFmbbWageBonus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverRetrieveInitSessionStatus {
    pub success: bool,
    pub ret: bool,
    pub p0_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverDeleteCharacterGetStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverDeleteSetTelemetryNonceSeed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverRetrieveStartSessionStatus {
    pub success: bool,
    pub ret: bool,
    pub p0_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverTransferBankToWalletGetStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverCatalogItemKeyIsValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverBasketStart {
    pub success: bool,
    pub ret: bool,
    pub transaction_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverBasketIsFull {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverIsCatalogCurrent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverSessionApplyReceivedData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverTransferWalletToBankGetStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverRefreshServerCatalog {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverStartSessionRestart {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverCheckoutStart {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverCatalogIsValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverBeginService {
    pub success: bool,
    pub ret: bool,
    pub transaction_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverTransferCashSetTelemetryNonceSeed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverDeleteCharacter {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverTransactionInProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverStartSessionPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverClearSession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverUseServerTransactions {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverIsSessionRefreshPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverGetCatalogCloudCrc {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverGetSessionStateAndStatus {
    pub success: bool,
    pub ret: bool,
    pub p0_: i32,
pub p1_: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverSetTelemetryNonceSeed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverStartSession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverBasketIsActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverIsSessionValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverCatalogItemIsValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverRetrieveSessionErrorCode {
    pub success: bool,
    pub ret: bool,
    pub p0_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverGetPrice {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverTransferWalletToBank {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverRetrieveCatalogRefreshStatus {
    pub success: bool,
    pub ret: bool,
    pub state_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverTransferBankToWallet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverBasketApplyServerData {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverEndService {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverInitSession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverBasketAddItem {
    pub success: bool,
    pub ret: bool,
    pub item_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetGameserverBasketEnd {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimeOffset {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPlatformPartyMemberCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasControlOfEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanSendLocalInvite {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCancelTransitionMatchmaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkShouldShowPromotionAlertScreen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetGamerStatusResult {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcHasGetFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetLocalSceneFromNetworkId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerTalking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetObjectCanBlendWhenFixed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearInvalidObjectModels {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionReserveSlotsTransition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPrimaryClanDataCancel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTunableCloudRequestPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsInSpectatorMode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTransitionFinish {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetRandomFloatRanged {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsActivitySession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsSignedIn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearFollowers {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRegisterEntityAsNetworked {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanPlayMultiplayerWithGamer {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCheckTextCommunicationPrivileges {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFacebookPostCompletedHeist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetEntityIsLocal {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanRegisterMissionPickups {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFacebookPostCompletedMilestone {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaxNumNetworkVehicles {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloudGetAvailabilityCheckResult {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTitleTextureDownloadRequest {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionHost {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPendingFriend {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaxNumNetworkPeds {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetTransitionVisibilityLock {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumCreatedMissionVehicles {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCheckPrivileges {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetInviteFailedMessageForInviteMenu {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionClosedCrew {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetAveragePing {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetActivityPlayerMax {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetAttributeDamageToPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPedToNet {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsGamerInMySession {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAllowCloningWhileInTutorial {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOverrideReceiveRestrictionsAll {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsSignedOnline {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetTunableCloudCrc {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsGameInProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanReleaseEmblem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetScriptValidateJoin {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSendImportantTransitionInviteViaPresence {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPlatformPartyMembers {
    pub success: bool,
    pub ret: i32,
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsActivitySpectator {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetLastPlayerPosReceivedOverNetwork {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerInvisibleLocally {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumCreatedMissionObjects {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanRequestEmblem {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasAgeRestrictions {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanGamerPlayMultiplayerWithMe {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTransitionSetInProgress {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNonParticipantsOfThisScriptAsGhosts {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearOfflineInvitePending {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAllowRemoteSyncedSceneLocalPlayerRequests {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsQueuingForSessionJoin {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcCopyContent {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAmIblockedByGamer {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetUserStarterAccess {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcQueryByContentId {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTextureDownloadRequest {
    pub success: bool,
    pub ret: i32,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcDidDescriptionRequestSucceed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkConcealEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsConnectedViaRelay {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcRequestContentDataFromIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPlayerGetCheaterReason {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetGhostAlpha {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcClearCreateResult {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetPropertyId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkKeepEntityCollisionDisabledAfterAnimScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkStartSoloTutorialSession {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetContentModifierListId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearGroupActivity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoesEntityExistWithNetworkId {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNumParticipants {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAccessTunableFloatModificationDetectionRegistrationHash {
    pub success: bool,
    pub ret: bool,
    pub value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPlayerIsBadsport {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsFriend {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentRating {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerAparticipantOnScript {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetParticipantIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetPlayerIsPassive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetThisScriptIsNetworkScript {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCommerceDataFetchInProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentHasLoResPhoto {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetHostOfScript {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetRichPresence {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcPublish {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumReservedMissionPeds {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanGetLocalMembershipsCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkFadeInEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetPrivilegeCheckResultNotNeeded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTransitionSetContentCreator {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetFriendCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPlayerLoudness {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAghost {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPreventScriptHostMigration {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddFollowers {
    pub success: bool,
    pub ret: (),
    pub p0_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDisableRealtimeMultiplayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasCachedPlayerHeadBlendData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionWasInvited {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityLocallyVisible {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPresenceInviteContentId {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcDidCreateSucceed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPlayerIndex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetSignallingInfo {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPositionHashOfThisScript {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddClientEntityArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHaveOnlinePrivileges {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRemoveAllQueuedJoinRequests {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAreTransitionDetailsValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkMarkAsPreferredActivity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetScriptControllingTeams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDelayMpStoreOpen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAllowRemoteAttachmentModification {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPresenceInviteSessionId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasAutomuteOverride {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionMarkVisible {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcSetBookmarked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsActivitySpectatorFromHandle {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOnReturnToSinglePlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetThisScriptIsNetworkScript {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionMatchmaking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkIdCanMigrate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcHasModifyFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkExplodeHeli {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCommerceItemNumCats {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkUseHighPrecisionBlending {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddClientEntityAngledArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOpenTransitionMatchmaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanCrewinfoGetCrewranktitle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanQueueForPreviousSessionJoin {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetCurrentPublicContentId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReserveLocalNetworkMissionPeds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetGamerStatusFromQueue {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetInProgressFinishTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionDoFriendMatchmaking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcIsDescriptionRequestInProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetLocalPlayerInvincibleTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetKillerOfPlayer {
    pub success: bool,
    pub ret: u32,
    pub weapon_hash_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkLaunchTransition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasBoneBeenHitByKiller {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsAnyPlayerNear {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsFriendInSameTitle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCommerceStoreOpen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionCancelInvite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsInPlatformParty {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkExplodeVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOverrideChatRestrictions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentIsPublished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcTextureDownloadRequest {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetTransitionActivityId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetMyContent {
    pub success: bool,
    pub ret: bool,
    pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSendTransitionGamerInstruction {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentLanguage {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkIdVisibleInCutsceneHack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionDoFreeroamQuickmatch {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRegisterPlayerBroadcastVariables {
    pub success: bool,
    pub ret: (),
    pub unk_vars_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetLastVelReceivedOverNetwork {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionLeaveSinglePlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTextureDownloadGetName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBlockInvites {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetAveragePacketLoss {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTutorialSessionChangePending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkStartCommunicationPermissionsCheck {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNumScriptParticipants {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetToVeh {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetPlayerMentalState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentFileVersion {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetRespawnResult {
    pub success: bool,
    pub ret: (),
    pub coordinates_: shared::Vector3,
pub heading_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetUnreliableResendCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddEntityAngledArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionOpenToMatchmaking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetEntityFromObjectId {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetInviteReplyStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHandleFromPlayer {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTriggerDamageEventForZeroWeaponHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoesNetworkIdExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPresenceInviteHandle {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkMarkAsWaitingAsync {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentHash {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSendTextMessage {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloudDidRequestSucceed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAccessTunableIntModificationDetectionRegistrationHash {
    pub success: bool,
    pub ret: bool,
    pub value_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoTransitionToFreemode {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPlayerTutorialSessionInstance {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetMissionFinished {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetOverrideTutorialSessionChat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkQueryRespawnResults {
    pub success: bool,
    pub ret: i32,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerAparticipant {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetCurrentChatOption {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPresenceInviteFromAdmin {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetRichPresenceString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRegisterHostBroadcastVariables {
    pub success: bool,
    pub ret: (),
    pub unk_vars_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoTransitionToGame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetMatchmakingPropertyId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasTransitionInviteBeenAcked {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkIdPassControlInTutorial {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPlayerHasHeadset {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDisableProximityMigration {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetCachedDescription {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAccessTunableIntHash {
    pub success: bool,
    pub ret: bool,
    pub value_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetFriendDisplayName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFadeOutLocalPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetInSpectatorModeExtended {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanReceiveLocalInvite {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHaveRosLeaderboardWritePriv {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsEntityFading {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkWasGameSuspended {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetInSpectatorMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsFriendOnline {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReserveLocalNetworkMissionVehicles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetGamertagFromHandle {
    pub success: bool,
    pub ret: Option<String>,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetEntityKillerOfPlayer {
    pub success: bool,
    pub ret: u32,
    pub weapon_hash_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOverrideClockRate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRequestCloudTunables {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetSameTeamAsLocalPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFacebookCanPostToFacebook {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearFollowInvite {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCloseTransitionMatchmaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetStoreNetworkGameTracking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCheckDataManagerSucceededForHandle {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkQuitMpToDesktop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcIsModifying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddMapEntityToSynchronisedScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoTransitionToNewGame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasNetworkTimeStarted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerPlayerCrcHackerCheck {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoteCheaterPlayerDetected {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAttachSynchronisedSceneToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetUniqueCrewLimitTransition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTextureDownloadRelease {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanGetMembershipValid {
    pub success: bool,
    pub ret: bool,
    pub p0_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanGetMembershipDesc {
    pub success: bool,
    pub ret: bool,
    pub member_desc_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPlayerGetUserid {
    pub success: bool,
    pub ret: Option<String>,
    pub user_i_d_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddEntityArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPresenceInviteInviter {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetMatchmakingGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEntityAreaIsOccupied {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkInviteGamersToTransition {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetPresenceSessionInvitesBlocked {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCheckDataManagerForHandle {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityGhostedForGhostPlayers {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkStartRespawnSearchInAngledAreaForPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIgnoreRemoteWaypoints {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloudHasRequestCompleted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetGameMode {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetAssistedDamageOfEntity {
    pub success: bool,
    pub ret: bool,
    pub p2_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcDidQueryCreatorsSucceed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasControlOfNetworkId {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasInvitedGamer {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEntityAreaHaveAllReplied {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentRatingNegativeCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReserveNetworkMissionObjects {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanSessionEnd {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloudCheckAvailability {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionBusy {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkApplyTransitionParameter {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetLocalPlayerSyncLookAt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetHighestReliableResendCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetTopRatedContent {
    pub success: bool,
    pub ret: bool,
    pub p2_: shared::MemoryBufferId,
pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionGetPrivateSlots {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkNeedToStartNewGameButBlocked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionStarted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPlayerIsRockstarDev {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetUniqueCrewOnlyCrewsTransition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPermissionsHasGamerRecord {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentId {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsThreadAnetworkScript {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkReportCodeTamper {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPlayerIndexIsCheater {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionGetMatchmakingGroupFree {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsFriendInMultiplayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkMarkTransitionGamerAsFullyJoined {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTextureDownloadHasFailed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanServiceIsValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfParticipantIdToInt {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerBlockedByMe {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOverrideSendRestrictionsAll {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetScriptStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAreHandlesTheSame {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle1_: shared::MemoryBufferId,
pub gamer_handle2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanBail {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanGetEmblemTxdName {
    pub success: bool,
    pub ret: bool,
    pub net_handle_: shared::MemoryBufferId,
pub txd_name_: Option<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentCreatorGamerHandle {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHashFromGamerHandle {
    pub success: bool,
    pub ret: u32,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOpenCommerceStore {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetDisplaynamesFromHandles {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsUserOldEnoughToAccessStore {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasRosPrivilegePlayedLastGen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShutdownAndLaunchSinglePlayerGame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCheckUserContentPrivileges {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetCreatorNum {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetRandomInt {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetNumBosses {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkJoinPreviouslyFailedSession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetModifyResult {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcReleaseCachedDescription {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionPrivate {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkStartRespawnSearchForPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDidGetGamerStatusSucceed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPrimaryClanDataSuccess {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionVoiceSetTimeout {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanDownloadMembershipPending {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasControlOfPickup {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkShowPsnUgcRestriction {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetAntagonisticToPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcLoadOfflineQuery {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasReceivedHostBroadcastData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionSolo {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcRequestCachedDescription {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcHasCreateFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEnableVoiceBandwidthRestriction {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHavePlatformSubscription {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetTimeoutTime {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHaveRosMultiplayerPriv {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTextChatIsTyping {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLocalPlayerAsGhost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetGamemode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHaveRosSocialClubPriv {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCheckRosLinkWentdownNotNet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNumTransitionNonAsyncGamers {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcClearOfflineQuery {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkVehicleAsGhost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRequestInviteConfirmedEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFacebookHasPostCompleted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerFading {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetInstanceIdOfThisScript {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcHasQueryCreatorsFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerInMpCutscene {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetLastEntityPosReceivedOverNetwork {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPrivilegeCheckInProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkFinishBroadcastingData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetTransitionHost {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionClosedFriends {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPlayerIsCheater {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGhostAlpha {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSkipRadioWarning {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCommerceItemId {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHaveScsPrivateMsgPriv {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreCutsceneEntitiesNetworked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetInviteOnCallForInviteMenu {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetGetByCategory {
    pub success: bool,
    pub ret: bool,
    pub p4_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionVoiceLeave {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasSocialClubAccount {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsLiveAreaLaunchWithContent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsInTransition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcReleaseAllCachedDescriptions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetCanReceiveRsInvites {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcQueryByCategory {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanRegisterMissionEntities {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAccessTunableBoolModificationDetectionRegistrationHash {
    pub success: bool,
    pub ret: bool,
    pub value_: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetIgnoreSpectatorChatLimitsSameTeam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAllowInviteProcessInPlayerSwitch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionHostFromHandle {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkShowChatRestrictionMsc {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPlayerIndexFromPed {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetRespawnResultFlags {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsInMpCutscene {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearQueuedJoinRequest {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetGlobalMultiplayerClock {
    pub success: bool,
    pub ret: (),
    pub hours_: i32,
pub minutes_: i32,
pub seconds_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearFoundGamers {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcQueryMostRecentlyCreatedContent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDamageTrackerActiveOnNetworkId {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsAddingFriend {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionHost {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCommerceItemCat {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOverrideTeamRestrictions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsHandleValid {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTitleUpdateRequired {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNetStatisticsInfo {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsParticipantActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetCrewLimitMaxMembersTransition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentUserName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetOverrideSpectatorMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentHasPlayerRecord {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsEntityConcealed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveAllStickyBombsFromEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFilloutPmPlayerListWithNames {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsGamerTalking {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasMadeInviteDecision {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoTransitionQuickmatch {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetCurrentlySelectedGamerHandleFromInviteMenu {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCommerceItemTexturename {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsNetworkIdRemotelyControlled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkResetBodyTracker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRemoveAllTransitionInvite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanRegisterMissionVehicles {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasInvitedGamerToTransition {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPresenceInvitePlaylistCurrent {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanCrewinfoGetStringValue {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseAllCommerceItemImages {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHaveUserContentPrivileges {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkUnregisterNetworkedEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetTransitionMembers {
    pub success: bool,
    pub ret: i32,
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetActivityPlayerNum {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRequestToBeHostOfThisScript {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddPedToSynchronisedScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPresenceInviteIndexById {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsOfflineInvitePending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionIsClosedCrew {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetCurrentlySelectedGamerHandleFromInviteMenu {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNpUnavailableReason {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetActivitySpectator {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRemoveTransitionInvite {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanIsRockstarClan {
    pub success: bool,
    pub ret: bool,
    pub clan_desc_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetUserPremiumAccess {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentRatingCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLocalPlayerVisibleLocally {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentTotal {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReserveNetworkMissionVehicles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkIdVisibleInCutsceneRemainHack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasSocialNetworkingSharingPriv {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasFollowInvite {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPlayerGetName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUsePlayerColourInsteadOfTeamColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsShowingSystemUiOrRecentlyRequestedUpsell {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsConnetedToNpPresence {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCheckOnlinePrivileges {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRemoveInvalidObjectModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcDidModifySucceed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetCurrentDataManagerHandle {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReserveLocalNetworkMissionObjects {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetDestroyerOfNetworkId {
    pub success: bool,
    pub ret: u32,
    pub weapon_hash_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNetworkTime {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetScriptReadyForEvents {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCreateSynchronisedScene {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentDescriptionHash {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetLookAtTalkers {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetBoneIdOfFatalHit {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsInactiveProfile {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanEnterMultiplayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityInGhostCollision {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddInvalidObjectModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionVoiceRespondToRequest {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentHasHiResPhoto {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcRequestContentDataFromParams {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanRegisterMissionObjects {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHaveRosBannedPriv {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsCloudBackgroundScriptRequestPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetGetByContentId {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEntityGetObjectId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPlayerOwnsWaypoint {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetHostPlayerIndex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkShouldShowStrictNatWarning {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetAssistedKillOfEntity {
    pub success: bool,
    pub ret: bool,
    pub p2_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkEnableHighSpeedEdgeFallDetection {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsHostOfThisScript {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCheckCommunicationPrivileges {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkShowAccountUpgradeUi {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetTalkerProximity {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasValidRosCredentials {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionIsInVoiceSession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkShowProfileUi {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkQueueGamerForStatus {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoesTunableExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearGetGamerStatus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRequestControlOfDoor {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentRatingPositiveCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsFriendHandleOnline {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAmIblockedByPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPresenceInviteIsTournament {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsStoreAvailableToUser {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsLaunchFromLiveArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNetworkTimeAccurate {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTriggerDamageEventForZeroDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsLocalPlayerInvincible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetContentToLoadType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTransitionStart {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetMatchmakingGroupMax {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStatusOfTextureDownload {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAccessTunableInt {
    pub success: bool,
    pub ret: bool,
    pub value_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetVehicleDrivenInTestDrive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerMutedByMe {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentCreatedByLocalPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsRefreshingRosCredentials {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsHost {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsChattingInPlatformParty {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddFriend {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetSpectatorToNonSpectatorTextChat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanCommunicateWithGamer {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsUsingOnlinePromotion {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfParticipantId {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerConcealed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasRosPrivilegeSpecialEditionContent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRequestCloudBackgroundScripts {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRemoveEntityArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerConnected {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcDidGetSucceed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetMinimumRankForMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSkipRadioResetNextClose {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsGettingGamerStatus {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionDoCrewMatchmaking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBail {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEntityUseHighPrecisionRotation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetAgeGroup {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRemotePlayerVisibleInCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetStoreEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearServiceEventArguments {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAccessTunableFloatHash {
    pub success: bool,
    pub ret: bool,
    pub value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTransitionBlockJoinRequests {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsMultiplayerDisabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkLeavePedBehindBeforeWarp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetMostRecentlyPlayedContent {
    pub success: bool,
    pub ret: bool,
    pub p2_: shared::MemoryBufferId,
pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOverrideSendRestrictions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentHasPlayerBookmarked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkApplyCachedPlayerHeadBlendData {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfObjToNet {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkStartSynchronisedScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsCloudAvailable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCloudTimeAsInt {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPrimaryClanDataClear {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasPlayerStartedTransition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcQueryMyContent {
    pub success: bool,
    pub ret: bool,
    pub p2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionVoiceHost {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoTransitionQuickmatchWithGroup {
    pub success: bool,
    pub ret: bool,
    pub p4_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetInMpCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkJoinTransition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetActivitySpectatorMax {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsScriptActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAmImutedByPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkIdCanBeReassigned {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionToGame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetProximityAffectsTeam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkInviteGamers {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetFoundGamer {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDisableInvincibleFlashing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsSessionStarted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkArePlayersInSameTutorialSession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimeAsString {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoTransitionToNewFreemode {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShutdownAndLoadMostRecentSave {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGamertagFromHandleStart {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetCrewContent {
    pub success: bool,
    pub ret: bool,
    pub p4_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanJoin {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcIsCreating {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionEnd {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkJoinGroupActivity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSuppressInvite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoTransitionQuickmatchAsync {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHaveRosCreateTicketPriv {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsLoggedInToPsn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHandleFromMemberId {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNetworkIdFromEntity {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerTuningCrcHackerCheck {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanIsEmblemReady {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanTextChatWithGamer {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNetworkIdOwnedByParticipant {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNumFoundGamers {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcClearModifyResult {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionForceCancelInvite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkVehicleMaxPositionDeltaMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimeDifference {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetDoNotLaunchFromJoinAsMigratedHost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestCommerceItemImage {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNumConnectedPlayers {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddPedToSynchronisedSceneWithIk {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHostTransition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRequestControlOfNetworkId {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkIdVisibleInCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasRosPrivilege {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetMaxNumParticipants {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetCustomArenaBallParams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaxNumNetworkPickups {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionBlockJoinRequests {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFacebookDidPostSucceed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetMostRecentlyCreatedContent {
    pub success: bool,
    pub ret: bool,
    pub p2_: shared::MemoryBufferId,
pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentCategory {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRemotePlayerAsGhost {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOverrideCoordsAndHeading {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkIdAlwaysExistsForPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlatformSubscriptionCheckPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentIsVerified {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanDownloadMembership {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPredictedVelocity {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAccessTunableBool {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetCurrentSpawnLocationOption {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumReservedMissionObjects {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkCutsceneEntities {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanGetMembershipCount {
    pub success: bool,
    pub ret: i32,
    pub p0_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionVoiceConnectToPlayer {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasPendingInvite {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfConvertPosixTime {
    pub success: bool,
    pub ret: (),
    pub time_structure_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerOnBlocklist {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsInTutorialSession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetMuteCountForPlayer {
    pub success: bool,
    pub ret: (),
    pub p1_: f32,
pub p2_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentIsUsingScNickname {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDumpNetIfConfig {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHaveCommunicationPrivileges {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanAccessMultiplayer {
    pub success: bool,
    pub ret: bool,
    pub loading_state_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOverrideTransitionChat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetMaxFriends {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGamertagFromHandlePending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasEntityBeenRegisteredWithThisThread {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanPlayerIsActive {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRetainActivityGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDamageTrackerActiveOnPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetScriptAutomuted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkWaitingPopClearTutorialSession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanAnyDownloadMembershipPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCommerceItemName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionChangeSlots {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehToNet {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPrimaryClanDataPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanViewGamerUserContent {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionIsVoiceSessionActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerCommerceDataFetch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReserveNetworkMissionPeds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRequestControlOfEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetCreatorsByUserId {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkForceLocalPlayerScarSync {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetGetByContentIds {
    pub success: bool,
    pub ret: bool,
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPlayerActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionLeave {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionHostFriendsOnly {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionIsVisible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetObjectScopeDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcClearQueryResults {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAreSocialClubPoliciesCurrent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetVoiceActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsFriendIndexOnline {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentPath {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanRemoteMembershipsAreInCache {
    pub success: bool,
    pub ret: bool,
    pub p0_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkConcealPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHashFromPlayerHandle {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanRegisterMissionPeds {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCacheLocalPlayerHeadBlendData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsNpAvailable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionIsDisplayingInviteConfirmation {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetToPed {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionDoActivityQuickmatch {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivateDamageTrackerOnPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkLeavePedBehindBeforeCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetToEnt {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetRootContentId {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsDoorNetworked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPrimaryClanDataNew {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsPushToTalkActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSendTransitionInviteViaPresence {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkUgcNav {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionValidateJoin {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasRosPrivilegeEndDate {
    pub success: bool,
    pub ret: bool,
    pub ban_type_: i32,
pub time_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkStopSynchronisedScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearPropertyId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanHasCrewinfoMetadataBeenReceived {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTransitionAddStage {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSendInviteViaPresence {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasConfirmedInvite {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetDestroyerOfEntity {
    pub success: bool,
    pub ret: u32,
    pub weapon_hash_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDisableLeaveRemotePedBehind {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetCreateContentId {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionLeavePostponed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloudDeleteMemberFile {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionJoinInvite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetTaskCutsceneInscopeMultipler {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcQueryByContentIds {
    pub success: bool,
    pub ret: bool,
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTryAccessTunableBoolHash {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionHostSinglePlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetEntityIsNetworked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloudIsCheckingAvailability {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetHostOfThisScript {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaxNumNetworkObjects {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkMemberIdFromGamerHandle {
    pub success: bool,
    pub ret: Option<String>,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcWasQueryForceCancelled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkActionFollowInvite {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanGetMembership {
    pub success: bool,
    pub ret: bool,
    pub p0_: i32,
pub clan_membership_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCanSetWaypoint {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkForceLocalUseOfSyncedSceneCamera {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetGamerInvitedToTransition {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDisableVoiceBandwidthRestriction {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSkipRadioResetNextOpen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCommerceProductPrice {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsInSession {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionAddActiveMatchmakingGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumCreatedMissionPeds {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTimeLessThan {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasControlOfDoor {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFilloutPmPlayerList {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetTalkerProximity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasViewGamerUserContentResult {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentUserId {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkUseLogarithmicBlendingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetEntityFromNetworkId {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPlayerFromGamerHandle {
    pub success: bool,
    pub ret: u32,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsGamerMutedByMe {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPrimaryClanDataStart {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionIsPrivate {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNumPresenceInvites {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumReservedMissionVehicles {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetTotalNumPlayers {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddSynchronisedSceneCamera {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentUpdatedDate {
    pub success: bool,
    pub ret: (),
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRemainInGameChat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBlockJoinQueueInvites {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcSetDeleted {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsTransitionVisibilityLocked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEndTutorialSession {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLocalPlayerVisibleInCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTryToSetThisScriptIsNetworkScript {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkLeaveTransition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionIsAwaitingInviteResponse {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNumberBodyTrackerHits {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPresenceInvitePlaylistLength {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetAverageLatency {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivateDamageTrackerOnNetworkId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHandleFromFriend {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcIsGetting {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetBookmarkedContent {
    pub success: bool,
    pub ret: bool,
    pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetTeamOnlyChat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDisplaynamesFromHandlesStart {
    pub success: bool,
    pub ret: i32,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionGetKickVote {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRegisterHighFrequencyPlayerBroadcastVariables {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInvertGhosting {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsClockTimeOverridden {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSphereVisibleToAnotherMachine {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetEntityCanBlend {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsSessionActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetToObj {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearClockTimeOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsScriptActiveByHash {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPlayerAccountId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkApplyVoiceProximityOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSphereVisibleToPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFacebookPostCreateCharacter {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHandleFromUserId {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNetworkVehicleRunningRespotTimer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsFindingGamers {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOverrideReceiveRestrictions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTimeMoreThan {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkFadeOutEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkResolvePrivilegeUserContent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkStartUserContentPermissionsCheck {
    pub success: bool,
    pub ret: i32,
    pub net_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAmImutedByGamer {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetPresenceInviteId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionGetHostAimPreference {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityVisibleInCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearVoiceChannel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkIdExistsOnAllMachines {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetContentNum {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetFriendName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityLocallyInvisible {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanRegisterMissionDoors {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRefreshPlayerListStats {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetRandomIntRanged {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetReservedMissionEntitiesInArea {
    pub success: bool,
    pub ret: (),
    pub out1_: shared::MemoryBufferId,
pub out2_: shared::MemoryBufferId,
pub out3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDoesTunableExistHash {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkFindGamersInCrew {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAccessTunableFloat {
    pub success: bool,
    pub ret: bool,
    pub value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionGetInviter {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLocalPlayerInvisibleLocally {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEntityAreaDoesExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkApplyPedScarData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEnableExtraVehicleOrientationBlendChecks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkOverrideClockTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerInCutscene {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetLocalHandle {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHasHeadset {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsGamerBlockedByMe {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcCancelQuery {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCommerceDataValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAccessTunableBoolHash {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkResurrectLocalPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRegisterHighFrequencyHostBroadcastVariables {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBailTransition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsNpPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkApplyTransitionParameterString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRemoveAndCancelAllInvites {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcHasDescriptionRequestFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNetworkVehicleRespotTimer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBadSportPlayerLeftDetected {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionHostClosed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetQueryResult {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanPlayerGetDesc {
    pub success: bool,
    pub ret: bool,
    pub clan_desc_: shared::MemoryBufferId,
pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkChangeTransitionSlots {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionIsVoiceSessionBusy {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetTransitionCreatorHandle {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetVoiceChannel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsCableConnected {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRemovePresenceInvite {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearVoiceProximityOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkRemoveAndCancelAllTransitionInvites {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPatchPostCutsceneHs4fTunEnt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCloudTimeAsString {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSeedRandomNumberGenerator {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetEntityOnlyExistsForParticipants {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetMatchmakingMentalState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAddEntityToSynchronisedScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerFileCrcHackerCheck {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumCommerceItems {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGamerHasHeadset {
    pub success: bool,
    pub ret: bool,
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionIsSolo {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsSessionBusy {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClanGetUiFormattedTag {
    pub success: bool,
    pub ret: (),
    pub clan_desc_: shared::MemoryBufferId,
pub formatted_tag_: Option<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetNoSpectatorChat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionSetUniqueCrewLimit {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcIsLanguageSupported {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTimeEqualTo {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkTransitionSetActivityIsland {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkFindMatchedGamers {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetFriendlyFireOption {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkStoreInviteThroughRestart {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcSetQueryDataFromOffline {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkDidFindGamersSucceed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetFriendContent {
    pub success: bool,
    pub ret: bool,
    pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSendQueuedJoinRequest {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionKickPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAcceptPresenceInvite {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerVisibleLocally {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkHideProjectileInCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLastViewedShopItem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAccessTunableModificationDetectionClear {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkFindLargestBunchOfPlayers {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkClearTransitionCreatorHandle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkAllowGangToJoinTutorialSession {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCancelRespawnSearch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcGetCreateResult {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSessionIsClosedFriends {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkSetInFreeCamMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetOnlineVersion {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGamertagFromHandleSucceeded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUgcSetUsingOfflineContent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkIsInPlatformPartyChat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkBlockProxyMigrationBetweenTutorialSessions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkGetNumUnackedReliables {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkJoinPreviouslyFailedTransition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayObjectAutoStartAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerEntirelyInsideGarage {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsObjectAportablePickup {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemSetAutomaticRate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateMoneyPickups {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupGlowOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponTypeFromPickupType {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMaxNumPortablePickupsCarriedByPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPickupWeaponObjectValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateNonNetworkedPortablePickup {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemGetDoorState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetOffsetFromCoordAndHeadingInWorldCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCoordsAndRotationOfClosestObjectOfType {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3,
pub out_rotation_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerPartiallyInsideGarage {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearObjectsInsideGarage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeaponImpactsApplyGreaterForce {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupUncollectable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIsObjectArticulated {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBlockPlayersForAmbientPickup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPickupCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHasObjectBeenCompletelyDestroyed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRayfireMapObjectAnimPhase {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupObjectGlowWhenUncollectable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveAllPickupsOfType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPointInAngledArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreatePortablePickup {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSlideObject {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPropTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupGenerationRangeMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetOnlyAllowAmmoCollectionWhenLow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePickup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRenderFakePickupGlow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsObjectEntirelyInsideGarage {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForcePickupRotateFaceUp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnyObjectNearPoint {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupObjectArrowMarker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectIsVisibleInMirrors {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsArticulatedJointAtMaxAngle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupHiddenWhenUncollectable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetActivateObjectPhysicsAsSoonAsItIsUnfrozen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsArticulatedJointAtMinAngle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasClosestObjectOfTypeBeenCompletelyDestroyed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveDoorFromSystem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPortablePickupPersist {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveObjectHighDetailModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemGetDoorPendingState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceActivatePhysicsOnUnfixedPickup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectAllowLowLodBuoyancy {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPickupObject {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateObject {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesRayfireMapObjectExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteObject {
    pub success: bool,
    pub ret: (),
    pub object_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTeamPickupObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemFindExistingDoor {
    pub success: bool,
    pub ret: bool,
    pub out_door_hash_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaceObjectOnGroundProperly {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetStateOfRayfireMapObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForcePortablePickupLastAccessiblePositionSetting {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfConvertOldPickupTypeToNew {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPropLightColor {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerPermittedToCollectPickupsOfType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectGlowInSameTeam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetProjectilesShouldExplodeOnContact {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowPortablePickupToMigrateToNonParticipants {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemGetOpenRatio {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableTidyingUpInGarage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloseSafehouseGarages {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateAmbientPickup {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnyEntityEntirelyInsideGarage {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemSetDoorState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSafePickupCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddDoorToSystem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloseAllBarriersForRace {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectIsApressurePlate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForcePickupRegenerate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasClosestObjectOfTypeBeenBroken {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPickupRewardTypeSuppression {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectForceVehiclesToAvoid {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupRegenerationTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupObjectCollectableInVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLocalPlayerCanCollectPortablePickups {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPickupBeenCollected {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCustomPickupWeaponHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowPickupArrowMarkerWhenUncollectable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupTransparentWhenUncollectable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreEntitiesEntirelyInsideGarage {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHidePortablePickupWhenDetached {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupObjectTransparentWhenUncollectable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLocalPlayerPermittedToCollectPickupsWithModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreatePickupRotate {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStateOfRayfireMapObject {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectTargettable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasObjectBeenBroken {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsObjectVisible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsObjectNearPoint {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableCollisionsBetweenCarsAndCarParachute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupObjectAlphaWhenTransparent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachPortablePickupToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressPickupSoundForPickup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsGarageEmpty {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDriveArticulatedJoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPreventCollectionOfPortablePickup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectSpeedBoostAmount {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateObjectNoOffset {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLockedUnstreamedInDoorOfType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemSetAutomaticDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateNonNetworkedAmbientPickup {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupObjectGlowOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAllPickupRewardTypeSuppression {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemSetDoorOpenForRaces {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowPickupByNoneParticipant {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowDamageEventsForNonNetworkedObjects {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOnlyCleanUpObjectWhenOutOfRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPropLightOverriden {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesPickupExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRotateObject {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDriveArticulatedJointWithInflictor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTrackObjectVisibility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityFlagRenderSmallShadow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPickupGenerationRangeMultiplier {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRayfireMapObject {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIsObjectBall {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemSetOpenRatio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetObjectFragmentDamageHealth {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearExtendedPickupProbeAreas {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCutscenesWeaponFlashlightOnThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesObjectOfTypeExistAtCoords {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupTrackDamageEvents {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDoorRegisteredWithSystem {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemSetSpringRemoved {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDoorClosed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectIsSpecialGolfball {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOpenAllBarriersForRace {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachPortablePickupFromPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEntityFlagSuppressShadow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddExtendedPickupProbeArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPickupTypeFromWeaponHash {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaceObjectOnGroundOrObjectProperly {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemSetHoldOpen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesPickupObjectExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearGarage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDefaultAmmoForWeaponPickup {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectSpeedBoostDuration {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemGetIsPhysicsLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDamageObjectFragmentChild {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClosestObjectOfType {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBreakObjectFragmentChild {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetObjectTintIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoorSystemGetAutomaticDistance {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectTakesDamageFromCollidingWithBuildings {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStateOfClosestDoorOfType {
    pub success: bool,
    pub ret: (),
    pub locked_: bool,
pub heading_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsObjectPartiallyInsideGarage {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTintIndexClosestBuildingOfType {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableSavingInGarage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForceObjectThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetObjectPhysicsParams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetStateOfClosestDoorOfType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressPickupRewardType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFixObjectFragment {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesPickupOfTypeExistInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreatePickup {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsObjectApickup {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowAllPlayersToCollectPickupsOfType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetControlInstructionalButtonsString {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsUsingAlternateDriveby {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDisabledControlNormal {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsUsingCursor {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetControlTriggerShake {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsControlEnabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsUsingRemotePlay {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsUsingAlternateHandbrake {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDisabledControlJustReleased {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableControlAction {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopControlShake {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInitPcScriptedControls {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSwitchPcScriptedControls {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetControlShake {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDisabledControlUnboundNormal {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsControlJustReleased {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsControlJustPressed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLocalPlayerGamepadAimState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUseAdjustedMouseCoords {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetControlUnboundNormal {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableAllControlActions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShutdownPcScriptedControls {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsControlReleased {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHaveControlsChanged {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsLookInverted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerpadShakesWhenControllerDisabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowAlternativeScriptControlsLayout {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetControlGroupInstructionalButtonsString {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetControlLightEffectColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDisabledControlJustPressed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearControlShakeSuppressedId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsUsingKeyboardAndMouse {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableAllControlActions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLocalPlayerAimState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearControlLightEffect {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetControlHowLongAgo {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetControlValue {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMouseLookInverted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDisabledControlPressed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetControlValueNextFrame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetControlNormal {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInputExclusive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetControlShakeSuppressedId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsControlPressed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDisabledControlReleased {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCursorPosition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAllowMovementWhileZoomed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableControlAction {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRoadsBackToOriginalInAngledArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumNavmeshesExistingInArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNodeProperties {
    pub success: bool,
    pub ret: bool,
    pub density_: i32,
pub flags_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestPathNodesInAreaThisFrame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientPedRangeMultiplierThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesNavmeshBlockingObjectExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateNavmeshBlockingObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPointOnRoad {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClosestRoad {
    pub success: bool,
    pub ret: bool,
    pub p5_: shared::Vector3,
pub p6_: shared::Vector3,
pub p7_: shared::MemoryBufferId,
pub p8_: shared::MemoryBufferId,
pub p9_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPositionBySideOfRoad {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRoadsInAngledArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleNodeIdValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRoadsBackToOriginal {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIgnoreNoGpsFlagUntilFirstNormalNode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllowStreamPrologueNodes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNthClosestVehicleNodeId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClosestVehicleNode {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearGpsDisabledZoneAtIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetApproxHeightForPoint {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClosestMajorVehicleNode {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetStreetNameAtCoord {
    pub success: bool,
    pub ret: (),
    pub street_name_: u32,
pub crossing_road_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetApproxFloorForPoint {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPathsInArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetApproxFloorForArea {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddNavmeshRequiredRegion {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNthClosestVehicleNodeFavourDirection {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3,
pub out_heading_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveNavmeshBlockingObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableNavmeshInArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNodeIsSwitchedOff {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNthClosestVehicleNodeIdWithHeading {
    pub success: bool,
    pub ret: i32,
    pub out_position_: shared::Vector3,
pub out_heading_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNodePosition {
    pub success: bool,
    pub ret: (),
    pub out_position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNavmeshRequiredRegionInUse {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIgnoreNoGpsFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNthClosestVehicleNodeWithHeading {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3,
pub out_heading_: f32,
pub out_num_lanes_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreAllNavmeshRegionsLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGpsBlipRouteFound {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetApproxHeightForArea {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveNavmeshRequiredRegions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRandomVehicleNode {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3,
pub node_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRoadBoundaryUsingHeading {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNodeIsGpsAllowed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAdjustAmbientPedSpawnDensitiesThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCalculateTravelDistanceBetweenPoints {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSafeCoordForPed {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGpsBlipRouteLength {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRoadsInArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadAllPathNodes {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGpsDisabledZoneAtIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNextGpsDisabledZoneIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGpsDisabledZone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPathsBackToOriginal {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNthClosestVehicleNode {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPosAlongGpsTypeRoute {
    pub success: bool,
    pub ret: bool,
    pub result_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllowStreamHeistIslandNodes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreNodesLoadedForArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNavmeshLoadedInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGenerateDirectionsToCoord {
    pub success: bool,
    pub ret: i32,
    pub direction_: i32,
pub p5_: f32,
pub dist_to_nx_junction_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddNavmeshBlockingObject {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClosestVehicleNodeWithHeading {
    pub success: bool,
    pub ret: bool,
    pub out_position_: shared::Vector3,
pub out_heading_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetFacialIdleAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedUsingActionMode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedMakeupTintColor {
    pub success: bool,
    pub ret: (),
    pub out_r_: i32,
pub out_g_: i32,
pub out_b_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMinMoveBlendRatio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedRagdollForceFall {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedOnFoot {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAsEnemy {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDucking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCanPedBeGrabbedByScript {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedTextureVariation {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedLipstickTintForBarber {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTargetPedInPerceptionArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllowLockonToPedIfFriendly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedTargetLossResponse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnyPedNearPoint {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMoveRateOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedProp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedBlushFacepaintTintForBarber {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMotionBlur {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMoveRateInWaterOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInAnyPoliceVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedGroupMemberPassengerIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedDecorations {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanPlayAmbientBaseAnims {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedTreatedAsFriendly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCreateRandomCops {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedEnveffCpvAdd {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedToInformRespectedFriends {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedVaulting {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWasPedSkeletonUpdated {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestRagdollBoundsUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedAplayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedParachutePackVariation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanPedRagdoll {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForceFootstepUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedBlendFromParents {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveActionModeAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedTakingOffHelmet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedEmissiveScale {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClonePedToTargetAlt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScenarioPedsToBeReturnedByNextCommand {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedSteersAroundObjects {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForcePedToOpenParachute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCopPedInArea3d {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedCauseOfDeath {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedBoneCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMeleeTargetForPed {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedConfigFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLadderClimbInputState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAiWeaponDamageModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddScenarioBlockingArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedUsingScenario {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedHangingOnToVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanSmashGlass {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGroupFormationSpacing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFmMaleShopPedApparelItemIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedTimeOfDeath {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedSteersAroundDeadBodies {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPedStrafeClipset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedRagdollBoneIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForcePedAiAndAnimationUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPedInVehicleContext {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedNearbyPeds {
    pub success: bool,
    pub ret: i32,
    pub size_and_peds_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedReloading {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerPedScenarioPanicexittoflee {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSynchronizedSceneRunning {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedWeaponMovementClipset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedComponentVariation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRagdollBlockingFlags {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedOpeningDoor {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHelmetPropIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachSynchronizedSceneToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCorpseRagdollFriction {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedHeadBlendData {
    pub success: bool,
    pub ret: bool,
    pub head_blend_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfPedDrawableVariations {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedSweat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpawnpointsGetSearchResult {
    pub success: bool,
    pub ret: (),
    pub x_: f32,
pub y_: f32,
pub z_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScenarioPedsSpawnInSphereArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedSteerBias {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestActionModeAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInAnyHeli {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedStrafeClipset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestStealthModeAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDiesInVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAsGroupLeader {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedRunningMobilePhoneTask {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPreloadPropData {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAoBlobRendering {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfMarkPedDecorationsAsClonedFromLocalPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestPedVehicleVisibilityTracking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfExplodePedHead {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedVisualFieldMinAngle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpawnpointsStartSearch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanPedShuffleToOrFromExtraSeat {
    pub success: bool,
    pub ret: bool,
    pub p1_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedNonCreationArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInAnyBoat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanTeleportToGroupLeader {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCopPerceptionOverrides {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTreatAsAmbientPedForDriverLockon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveScenarioBlockingArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedTargetFromCombatPed {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfApplyPedBloodByZone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedDeadOrDying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedToLoadCover {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedParachuteTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCountPedsInCombatWithTargetWithinRadius {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedIsIgnoredByAutoOpenDoors {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHearingRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedShooting {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCapsule {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimePedDamagedByWeapon {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedScubaGearVariation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedSwitchingWeapon {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedAccuracy {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSynchronizedSceneHoldLastFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfApplyPedDamageDecal {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedAimingFromCover {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPreloadVariationData {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPedVisibleDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedVisualFieldCenterAngle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAllowVehiclesOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedDefensiveAreaPosition {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCombatRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpawnpointsIsSearchActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedNeverLeavesGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedLipstickTintForCreator {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanCreateRandomPed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedWillOnlyAttackWantedPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedBoneIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedMoney {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHelmetVisorPropIndices {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGroupSeparationRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedLanding {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDefensiveAreaDirection {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedEvasiveDiving {
    pub success: bool,
    pub ret: bool,
    pub evading_entity_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateNmMessage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedShouldIgnoreScenarioExitCollisionChecks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedRelationshipGroupDefaultHash {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanBeTargetedWithoutLos {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMaxMoveBlendRatio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedJumpingOutOfVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMaxTimeInWater {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCreateRandomCopsOnScenarios {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterPedheadshot {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedWetnessHeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedTryingToEnterAlockedVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedHelmetStoredHatPropIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedDoingAbeastJump {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfKnockPedOffVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDefaultComponentVariation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedOnMount {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFinalizeHeadBlend {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPedHasSexinessFlagSet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfApplyPedDamagePack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetAiMeleeWeaponDamageModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedSteersAroundPeds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedMaxHealth {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInstantlyFillPedPopulation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedRagdoll {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedHairTintColor {
    pub success: bool,
    pub ret: (),
    pub out_r_: i32,
pub out_g_: i32,
pub out_b_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInCombat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHeadOverlay {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHeadOverlayTint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAllowMinorReactionsAsMissionPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedJacking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedDriveByClipsetOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAnimInitialOffsetRotation {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedTracked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHairTint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCombatMovement {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInMeleeCombat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedEmissiveScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDefensiveAreaAttachedToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedBoundsOrientation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedBeingStunned {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptedConversionCoordThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeadBlendEyeColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTrackedPedPixelcount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedAsGroupMember {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanKnockPedOffVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedDamageDecalByZone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHighlyPerceptive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCombatFloat {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedInVehicleContext {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedStopped {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedClimbing {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCountPedsInCombatWithTarget {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetJackTarget {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGivePedHelmet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedDiving {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHelmet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRelationshipGroupAffectsWantedLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPedDecorationFromHashesInCorona {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFacialClipset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDiesInWater {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedBloodDamageByZone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedIncreasedAvoidanceRadius {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedBlocksPathingWhenDead {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedUsingAnyScenario {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInGroup {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptedAnimSeatOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedHurt {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressAmbientPedAggressiveCleanupThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleasePedPreloadVariationData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedShouldIgnoreScenarioNavChecks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddArmourToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDiesWhenInjured {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedAsGroupLeader {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedHeadtrackingPed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleasePedheadshotImgUpload {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearRelationshipBetweenGroups {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanCreateRandomCops {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedHeadBlendNumHeads {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPedDecorationFromHashes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfPedPropDrawableVariations {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInAnyPlane {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisablePedHeatscaleOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedBlushTintForBarber {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehiclePedIsUsing {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInCover {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedShootRate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWasPedKnockedOut {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSynchronizedSceneLooped {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedRespondingToEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHidePedBloodDamageByZone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateSynchronizedSceneAtMapObject {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanPlayAmbientAnims {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearCoverPointForPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanBeTargetedWhenInjured {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetGroupFormationDefaultSpacing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanBeTargetted {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPedHeadBlendFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedDiesInWater {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedEnvDirt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAiMeleeWeaponDamageModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanTorsoVehicleIk {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPedPreloadVariationDataFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClonePedAlt {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanBeTargettedByPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedOnVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedDrawableVariation {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnyHostilePedNearPoint {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedStoredHatProp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedHeadBlendFirstIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfApplyDamageToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInHighCover {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerPedIsFollowing {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSynchronizedSceneOrigin {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientPedsDropMoney {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanEvasiveDive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMaxTimeUnderwater {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanArmIk {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAlternateWalkAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanPedSeeHatedPed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachSynchronizedScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedMale {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInAnyTaxi {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScriptedScenarioPedUsingConditionalAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSeatPedIsTryingToEnter {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInAnyTrain {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfKnockOffPedProp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedVisualFieldMaxAngle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedheadshotReady {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedFleeAttributes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableHighFallDeath {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMicroMorph {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResurrectPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedDecorationsState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdatePedHeadBlendData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearFacialIdleAnimOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisablePedInjuredOnGroundBehaviour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSynchronizedScenePhase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHaveAllStreamingRequestsCompleted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanLegIk {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePedDefensiveArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestPedUseSmallBboxVisibilityTracking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHeadBlendEyeColor {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMovementModeOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPedPreloadPropDataFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedClothPinFrames {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedVisualFieldMaxElevationAngle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInModel {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedParachuteState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedVisualFieldMinElevationAngle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScenarioPedDensityMultiplierThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanBeKnockedOffVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAccuracy {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedStealthMovement {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesGroupExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestPedVisibilityTracking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedRelationshipGroupHash {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInParachuteFreeFall {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreatePedInsideVehicle {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedShootingInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedConfigFlag {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWasPedKilledByTakedown {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSynchronizedSceneHoldLastFrame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedGeneratesDeadBodyEvents {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedFallUpperBodyClipsetOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedHeadtrackingEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehiclePedIsTryingToEnter {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfApplyPedBloodDamageByZone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedShaderReady {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanPlayInCarIdles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedSittingInAnyVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedClothPackageIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPhonePaletteIdx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfApplyPedBlood {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPreferredCoverSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInCoverFacingLeft {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInjured {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPedReceivedEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedBeingStealthKilled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCoordsNoGang {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRandomPedAtCoord {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPedheadshotImgUploadFailed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientLawPedAccuracyModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMpLightEnabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedAlternateWalkAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedStealthMovement {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedPropIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesScenarioBlockingAreaExists {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCreateRandomCopsNotOnScenarios {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedParachuteLandingType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateSynchronizedScene {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateParachuteBagObject {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReviveInjuredPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGroupSize {
    pub success: bool,
    pub ret: (),
    pub p1_: shared::MemoryBufferId,
pub size_in_members_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedLastDamageBone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfPedTextureVariations {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedBloodDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateGroup {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAlternateMovementAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInFlyingVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTrackedPedVisible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveStealthModeAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterHatedTargetsAroundPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPropIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedSourceOfDeath {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHeadBlendData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedOnAnyBike {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedArmour {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPlaysHeadOnHornAnimWhenDiesInVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedVehicleForcedSeatUsage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterPedheadshotTransparent {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDensityMultiplierThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeletePed {
    pub success: bool,
    pub ret: (),
    pub ped_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedShootsAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnregisterPedheadshot {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedKeepTask {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedEnableWeaponBlocking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPedWeaponMovementClipset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedNameDebug {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlockingOfNonTemporaryEventsForAmbientPedsThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInAnyVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedBeingJacked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfToggleScenarioPedCowerInPlace {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehiclePedIsIn {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedFiringPattern {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCoordsKeepVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedsJacker {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateRandomPedAsDriver {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedGroupMember {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedEnveffScale {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanPedShuffleToOrFromTurretSeat {
    pub success: bool,
    pub ret: bool,
    pub p1_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedWetness {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedVisualFieldPeripheralRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedSphereDefensiveArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedHelmetStoredHatTexIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMotionInCoverClipsetOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedSwimming {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMpOutfitDataFromMetadata {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRelationshipBetweenGroups {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMoveAnimsBlendOut {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAsGroupMember {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedGoingIntoCover {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCombatAttributes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBlockingOfNonTemporaryEvents {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPedRagdollTimer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedDecorationZoneFromHashes {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedGravity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedheadshotValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnyPedShootingInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableHeadBlendPaletteColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedShouldPlayNormalScenarioExit {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMobilePhoneToPedEar {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedNoTimeDelayBeforeShot {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCowerHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpawnpointsIsSearchComplete {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedHeadOverlay {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpawnpointsGetNumSearchResults {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedClothProne {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfPedPropTextureVariations {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDriverAggressiveness {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePedHelmet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedSittingInVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedIsAvoidedByOthers {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMoney {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDesiredHeading {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPedMovementClipset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDefaultSecondaryTintForBarber {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPinnedDown {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedWetness {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePedElegantly {
    pub success: bool,
    pub ret: (),
    pub ped_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTellGroupPedsInAreaToAttack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedRelationshipGroupDefaultHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedToRagdoll {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMovementClipset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedResetFlag {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedUpperBodyDamageOnly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDropAmbientProp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanRagdoll {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGivePedNmMessage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDriverAbility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanBeKnockedOffBike {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpawnpointsStartSearchInAngledArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedDoingDriveby {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHealthPendingLastDamageEventOverrideFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCanAttackFriendly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopAnyPedModelBeingSuppressed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateRandomPed {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedScubaGearVariation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedWetnessEnabledThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveRelationshipGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSynchronizedSceneRate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpawnpointsGetSearchResultFlags {
    pub success: bool,
    pub ret: (),
    pub p1_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedSheltered {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanCreateRandomDriver {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedHelmetVisorUp {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedHuman {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedDefensiveAreaActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPosFromFiredEvent {
    pub success: bool,
    pub ret: bool,
    pub out_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterPedheadshotHires {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanPlayGestureAnims {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAsCop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedGettingIntoAvehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPedLastVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedFleeing {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedGetOutUpsideDownVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAnimInitialOffsetPosition {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanBeTargettedByTeam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRelationshipBetweenGroups {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedEnveffScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedSwimmingUnderWater {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHelmetFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanHeadIk {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopPedWeaponFiringWhenDropped {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanBeDraggedOut {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedResetFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHeatscaleOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerIdleAnimationOnPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedGesturing {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIkTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClosestPed {
    pub success: bool,
    pub ret: bool,
    pub out_ped_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedLegIkMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedRandomProps {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanPeekInCover {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEnableBoundAnkles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTintIndexForLastGenHairTexture {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedPlantingBomb {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCombatAbility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedMotionInCoverClipsetOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanBeShotInVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAngledDefensiveArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedRelationshipGroupHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedRandomComponentVariation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedModel {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanCowerInCover {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForceStepType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesRelationshipGroupExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeadBlendPaletteColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestPedRestrictedVehicleVisibilityTracking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPrimaryLookat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDeadPedPickupCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAllPedProps {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTakeOwnershipOfSynchronizedScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGroupFormation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedArmour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedShouldProbeForScenarioExitsInOneFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedJumping {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedHeadOverlayNum {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedNearbyVehicles {
    pub success: bool,
    pub ret: i32,
    pub size_and_vehs_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedDucking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedRunningMeleeTask {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumPedMakeupTints {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEnablePedEnveffScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceZeroMassInCollisions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveScenarioBlockingAreas {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreatePed {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedEnveffColorModulator {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedProne {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedFacingPed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDiesInSinkingVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedLastDamageBone {
    pub success: bool,
    pub ret: bool,
    pub out_bone_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedUsingActionMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedToRagdollWithFall {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSynchronizedSceneRate {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedFatallyInjured {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearRagdollBlockingFlags {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPopControlSphereThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedAlternateMovementAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSynchronizedSceneLooped {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAllowedToDuck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedheadshotTxdString {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAlertness {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedLodMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedPerformingMeleeAction {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedGestureGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedCombatMovement {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDriverRacingModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEnableHandcuffs {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanRagdollFromPlayerImpact {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisablePedMapCollision {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsConversationPedDead {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedExtractedDisplacement {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedHairTintForBarber {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedPropTextureIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedModelIsSuppressed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayFacialAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedDecorationsLeaveScars {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedRunningRagdollTask {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedPaletteVariation {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBlockPedFromGeneratingDeadBodyEventsWhenDead {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDefensiveSphereAttachedToVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSynchronizedScenePhase {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasActionModeAssetLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumPedHairTints {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearAllPedVehicleForcedSeatUsage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMount {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedComponentVariationValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanLosePropsOnDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedReserveParachuteTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPedheadshotImgUploadSucceeded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedEnableCrewEmblem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClonePedToTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasStealthModeAssetLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetAiWeaponDamageModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDefaultSecondaryTintForCreator {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanCreateRandomBikeRider {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanPedInCombatSeeTarget {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedParachuteTintIndex {
    pub success: bool,
    pub ret: (),
    pub out_tint_index_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedSteersAroundVehicles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRelationshipBetweenPeds {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedheadshotImgUploadAvailable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedPerformingAcounterAttack {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedSuffersCriticalHits {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanUseAutoConversationLookat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedInjuredOnGroundBehaviour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedOnSpecificVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedShouldPlayDirectedNormalScenarioExit {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDriveByClipsetOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceInstantLegIkSetup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedHairTintForCreator {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePedFromGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanSwitchWeapon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedStayInVehicleWhenJacked {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedNonCreationArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableMpLight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDiesInstantlyInWater {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedShouldPlayFleeScenarioExit {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfApplyPedBloodSpecific {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClonePed {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedVisualFieldCenterAngle {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFmFemaleShopPedApparelItemIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedRagdollOnCollision {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestPedheadshotImgUpload {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedIdRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedHelmetTextureIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedGroupIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedShouldPlayImmediateScenarioExit {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCurrentHeadPropAhelmet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForcePedMotionState {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedSeeingRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanTorsoIk {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAllowHurtCombatForAllMissionPeds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedWearingHelmet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddRelationshipGroup {
    pub success: bool,
    pub ret: bool,
    pub group_hash_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedBlushTintForCreator {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpawnpointsIsSearchFailed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanTorsoReactIk {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMaxHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedCurrentMoveBlendRatio {
    pub success: bool,
    pub ret: bool,
    pub speed_x_: f32,
pub speed_y_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedAlertness {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedIntoVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleasePedPreloadPropData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanPlayVisemeAnims {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehiclePedIsEntering {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWasPedKilledByStealth {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEnableScuba {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialFunctionDoNotUse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDefensiveSphereAttachedToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedCombatRange {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedMinGroundTimeForStungun {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllowStuntJumpCamera {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedFalling {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInAnySub {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedHeadingTowardsPosition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUseCameraHeadingForDesiredDirectionLockOnTest {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedPerformingStealthKill {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePedPreferredCoverSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPanicExitScenario {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsUsingPedScubaGearVariation {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpawnpointsCancelSearch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceAllHeadingValuesToAlign {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCombatFloat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHealthSnacksCarriedByAllNewPeds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFacialIdleAnimOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableFragDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsEntityAfrag {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartRopeWinding {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUseKinematicPhysics {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRopeLastVertexCoord {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesScriptOwnRope {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPinRopeVertex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBreakEntityGlass {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRopeVertexCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeSetSmoothReelin {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachEntitiesToRope {
    pub success: bool,
    pub ret: (),
    pub p12_: shared::MemoryBufferId,
pub p13_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachRopeToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnpinRopeVertex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteRope {
    pub success: bool,
    pub ret: (),
    pub rope_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeConvertToSimple {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartRopeUnwindingFront {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableBreaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeUnloadTextures {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivatePhysics {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeGetDistanceBetweenEnds {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCgoffset {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsRopeAttachedAtBothEnds {
    pub success: bool,
    pub ret: bool,
    pub rope_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeLoadTextures {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInStuntMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeDrawEnabled {
    pub success: bool,
    pub ret: (),
    pub rope_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteChildRope {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInArenaMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeChangeScriptOwner {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeSetRefframevelocityColliderorder {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeAttachVirtualBoundGeom {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachRopeFromEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCgAtBoundcenter {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeResetLength {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeSetUpdatePinverts {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopRopeWinding {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadRopeData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetDisableBreaking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeForceLength {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCgoffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeSetUpdateOrder {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfApplyImpulseToCloth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddRope {
    pub success: bool,
    pub ret: i32,
    pub unk_ptr_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRopeVertexCoord {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDamping {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeDrawShadowEnabled {
    pub success: bool,
    pub ret: (),
    pub rope_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRopeAreTexturesLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesRopeExist {
    pub success: bool,
    pub ret: bool,
    pub rope_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopRopeUnwindingFront {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetLawResponseDelayOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWantedLevelMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerTeam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerTeleportActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfChangePlayerPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllRandomPedsFlee {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSpecialAbilityMeterFull {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerReserveParachuteModelOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWantedLevelRadius {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfArePlayerStarsGreyedOut {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerWantedCentrePosition {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerGroup {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPlayerParachuteVariationOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerForcedAim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPlayerParachutePackModelOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerPhonePaletteIdx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerTargetEntity {
    pub success: bool,
    pub ret: bool,
    pub entity_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerClothLockCounter {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityDeactivateMp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableSpecialAbility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerSprintTimeRemaining {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPlayerInputGait {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAchievementProgress {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityDepleteMeter {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayerDetachVirtualBound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerMayNotEnterAnyVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfPlayersInTeam {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPlayerDamagedAtLeastOnePed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerVehicleWeaponToNonHoming {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetApplyWaypointOfPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerWantedLevelGreater {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerInvincible {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPlayerReserveParachuteModelOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityPlayerIsFreeAimingAt {
    pub success: bool,
    pub ret: bool,
    pub entity_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerLockonRangeOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPlayerArrestState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerWeaponDefenseModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerFreeAiming {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityChargeSmall {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableAmbientMeleeMove {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerCurrentStealthNoise {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowEvasionHudIfDisablingHiddenEvasionThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerSpectatedVehicleRadioOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerWeaponTakedownDefenseModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPoliceIgnorePlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerWantedLevelNoDrop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressWitnessesCallingPoliceThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerTeam {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityReset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerReserveParachuteModelOverride {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerBeingArrested {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerBattleAware {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerWantedLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerFreeAimingAtEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGivePlayerRagdollControl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityFillMeter {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSpecialAbilityActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerSprintStaminaRemaining {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfPlayers {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIntToPlayerindex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerDead {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPoliceRadarBlips {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerPed {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaxWantedLevel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressLosingWantedLevelIfHiddenThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllRandomPedsFleeThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSimulatePlayerInputGait {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWantedLevelHiddenEscapeTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerControlOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerMeleeWeaponDamageModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPlayerHasDamagedAtLeastOneNonAnimalPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerVehicleDefenseModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceCleanupForAllThreadsWithThisName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerStealthPerceptionModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerRidingTrain {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayerId {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfExtendWorldBoundaryForPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimeSinceLastArrest {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerPedScriptIndex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerWantedCentrePosition {
    pub success: bool,
    pub ret: (),
    pub position_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableCameraViewModeCycle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerCanDamagePlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerFakeWantedLevel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerTargetLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIgnoreLowPriorityShockingEvents {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerLockon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimeSincePlayerHitVehicle {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSystemUiBeingDisplayed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerHealthRechargeMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerBluetoothState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerHasReserveParachute {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisablePlayerFiring {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerPlaying {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsPlayerDrivingOnHighway {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerBluetoothEnable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsRemotePlayerInNonClonedVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityLock {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerInvincibleButHasReactions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRunSprintMultiplierForPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerVehicleWeaponToggledToNonHoming {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerCanDoDriveBy {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerParachutePackTintIndex {
    pub success: bool,
    pub ret: (),
    pub tint_index_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptFirePosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveScriptFirePosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerLoggingInNp {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerClothPinFrames {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerParachuteTintIndex {
    pub success: bool,
    pub ret: (),
    pub tint_index_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerForcedZoom {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerForceSkipAimIntro {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerMaxArmour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerTargettingAnything {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerTargettingEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerPreviousVariationData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAreCameraControlsDisabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerHasReserveParachute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsWantedAndHasBeenSeenByCops {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerMayOnlyEnterThisVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerParachuteSmokeTrailColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityActivate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetWantedLevelHiddenEscapeTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAssistedMovementFlushRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasAchievementBeenPassed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPlayerParachuteModelOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerScriptControlOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerHealthRechargeMaxPercent {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerControl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerMaxExplosiveDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEveryoneIgnorePlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerReadyForCutscene {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPlayerTargetableEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerMaxArmour {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerParachutePackTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisplaySystemSigninUi {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerClimbing {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerParachuteModelOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCauseOfMostRecentForceCleanup {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressCrimeThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWantedLevelDifficulty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityDeactivateFast {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIntToParticipantindex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIncreasePlayerJumpSuppressionRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePlayerTargetableEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAutoGiveParachuteWhenEnterPlane {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerClothPackageIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerSprint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityChargeNormalized {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerUnderwaterBreathPercentRemaining {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerUnderwaterTimeRemaining {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRestorePlayerStamina {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerParachuteTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSpecialAbilityMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerVehicleDamageModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerIndex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetPlayerStamina {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWantedLevelTimeToEscape {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSwimMultiplierForPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMaxWantedLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartPlayerTeleport {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceStartHiddenEvasion {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerMeleeWeaponDefenseModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAssistedMovementCloseRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerReserveParachuteTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfArePlayerFlashingStarsAboutToDrop {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerTargetingMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSpecialAbilityEnabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSpecialAbilityMp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerSneakingNoiseMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPlayerWantedLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLawResponseDelayOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayersLastVehicle {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerInvincible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityChargeAbsolute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisablePlayerThrowGrenadeWhileUsingGun {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsUsingFpsThirdPersonCover {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetWantedLevelDifficulty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerReceivedBattleEventRecently {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceCleanup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateWantedPositionThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisablePlayerHealthRecharge {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerWeaponMinigunDefenseModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGiveAchievementToPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartFiringAmnesty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisablePlayerVehicleRewards {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerParachuteModelOverride {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAchievementProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllNeutralRandomPedsFleeThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerHealthRechargeMaxPercent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopPlayerTeleport {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerSimulateAiming {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSpecialAbilityUnlocked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimeSinceLastDeath {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasForceCleanupOccurred {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityChargeOnMissionFailed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAirDragMultiplierForPlayersVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerCanCollectDroppedMoney {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsUsingHoodCamera {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerWeaponDamageModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAutoGiveScubaGearWhenExitVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerCanUseCover {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimeSincePlayerDroveOnPavement {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPlayerLeftTheWorld {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerReserveParachuteTintIndex {
    pub success: bool,
    pub ret: (),
    pub index_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerCanBeHassledByGangs {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityDeactivate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPlayerBeenSpottedInStolenVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayerPedId {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerExplosiveDamageModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerParachuteVariationOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetWorldBoundaryForPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDispatchCopsForPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimeSincePlayerDroveAgainstTraffic {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerNoiseMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReportPoliceSpottedPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerParachutePackModelOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerDebugInvincible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerFreeForAmbientTask {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsMoppingAreaFreeInFrontOfPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllNeutralRandomPedsFlee {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanPlayerStartMission {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerWantedLevelNow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdatePlayerTeleport {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerWantedLevel {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimeSincePlayerHitPed {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPlayerDamagedAtLeastOneNonAnimalPed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerRgbColour {
    pub success: bool,
    pub ret: (),
    pub r_: i32,
pub g_: i32,
pub b_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReportCrime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityChargeContinuous {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayerAttachVirtualBound {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerHomingDisabledForAllVehicleWeapons {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkPlayerIdToInt {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerParachuteSmokeTrailColor {
    pub success: bool,
    pub ret: (),
    pub r_: i32,
pub g_: i32,
pub b_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerFallDistanceToTriggerRagdollOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPlayerHasDamagedAtLeastOnePed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsPlayerDrivingWreckless {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityChargeMedium {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityUnlock {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerOnline {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanPedHearPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePlayerHelmet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerCanLeaveParachuteSmokeTrail {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSpecialAbilityChargeLarge {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceCleanupForThreadWithThisId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerPressingHorn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLawPedsCanAttackNonWantedPlayerThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWantedLevelThreshold {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerLeavePedBehind {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateSpecialAbilityFromStat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopReplayRecording {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplayCancelEvent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsReplayRecording {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplayCheckForEventThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplayRecordBackForTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsReplayRecordSpaceAvailable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsReplayAvailable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplayStartEvent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSaveReplayRecording {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRecordGreatestMoment {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplayStopEvent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCancelReplayRecording {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplayDisableCameraMovementThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartReplayRecording {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsReplayInitialized {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplayPreventRecordingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplayResetEventInfo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplayControlShutdown {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfActivateRockstarEditor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptsHaveCleanedUpForReplaySystem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterEffectForReplayEditor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReplaySystemHasRequestedAscriptCleanup {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetReplaySystemPausedForSave {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSavemigrationMpGetStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSavemigrationMpNumAccounts {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSavemigrationIsMpEnabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSavemigrationMpRequestAccounts {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSavemigrationMpGetAccountsStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSavemigrationMpRequestStatus {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSavemigrationMpGetAccount {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNameOfScriptWithThisId {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShutdownLoadingScreen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBgDoesLaunchParamExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBgEndContextHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTerminateThisThread {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNoLoadingScreen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBgGetLaunchParamValue {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEventData {
    pub success: bool,
    pub ret: bool,
    pub event_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfThreadsRunningTheScriptWithThisHash {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScriptThreadIteratorGetNextThreadId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetThisScriptName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThreadActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNoLoadingScreen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTriggerScriptEvent {
    pub success: bool,
    pub ret: (),
    pub event_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasScriptWithNameHashLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfEvents {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestScript {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBgStartContextHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBgSetExitflagResponse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBgGetScriptIdFromNameHash {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBgIsExitflagSet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHashOfThisScriptName {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEventExists {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBgStartContext {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSendTuScriptEvent {
    pub success: bool,
    pub ret: (),
    pub event_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCommitToLoadingscreenSelction {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIdOfThisThread {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptWithNameHashAsNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTerminateThread {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptAsNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestScriptWithNameHash {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEventAtIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScriptThreadIteratorReset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBgEndContext {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasScriptLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesScriptWithNameHashExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesScriptExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnregisterScriptVariable {
    pub success: bool,
    pub ret: (),
    pub unk_variable_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRegisterScriptVariable {
    pub success: bool,
    pub ret: (),
    pub unk_variable_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceCheckScriptVariables {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartShapeTestBoundingBox {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartShapeTestCapsule {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleaseScriptGuidFromEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartShapeTestBound {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartExpensiveSynchronousShapeTestLosProbe {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShapeTestResult {
    pub success: bool,
    pub ret: i32,
    pub hit_: bool,
pub end_coords_: shared::Vector3,
pub surface_normal_: shared::Vector3,
pub entity_hit_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetShapeTestResultIncludingMaterial {
    pub success: bool,
    pub ret: i32,
    pub hit_: bool,
pub end_coords_: shared::Vector3,
pub surface_normal_: shared::Vector3,
pub material_hash_: u32,
pub entity_hit_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartShapeTestLosProbe {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartShapeTestSweptSphere {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartShapeTestBox {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartShapeTestMouseCursorLosProbe {
    pub success: bool,
    pub ret: i32,
    pub p_vec1_: shared::Vector3,
pub p_vec2_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxGetTotalNumMessages {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScEmailRetrieveEmails {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateGetAddIsPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScEmailSetCurrentEmailTag {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateIsvalid {
    pub success: bool,
    pub ret: bool,
    pub token_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScEmailSendEmail {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScEmailGetRetrievalStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScProfanityGetCheckIsValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetDisplayNameById {
    pub success: bool,
    pub ret: bool,
    pub p1_: Option<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateAdd {
    pub success: bool,
    pub ret: bool,
    pub plate_data_: shared::MemoryBufferId,
pub token_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScAccountInfoGetNickname {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetExtraDataStringForType {
    pub success: bool,
    pub ret: bool,
    pub p1_: Option<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateGetPlate {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScPresenceAttrSetInt {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScAchievementInfoStatus {
    pub success: bool,
    pub ret: bool,
    pub p0_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScEmailMessagePushGamerToRecipList {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetExtraDataFloatForType {
    pub success: bool,
    pub ret: bool,
    pub p1_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScPresenceAttrSetString {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxSetMessageAsReadAtIndex {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScGamerdataGetActiveXpBonus {
    pub success: bool,
    pub ret: bool,
    pub value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateGetPlateData {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScTransitionNewsHasExtraDataTu {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetDisplayNameForType {
    pub success: bool,
    pub ret: bool,
    pub p0_: Option<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScHasAchievementBeenPassed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScEmailDeleteEmails {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventIsActiveForType {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScEmailGetEmailAtIndex {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScPresenceSetActivityRating {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetEventIdForType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetEventId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetExtraDataFloat {
    pub success: bool,
    pub ret: bool,
    pub p1_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScEmailMessageClearRecipList {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateGetIsvalidStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScTransitionNewsEnd {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetExtraDataStringById {
    pub success: bool,
    pub ret: bool,
    pub p2_: Option<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxMessageGetUgcdata {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxSendBountyToRecipList {
    pub success: bool,
    pub ret: bool,
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScTransitionNewsShow {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateGetCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetExtraDataInt {
    pub success: bool,
    pub ret: bool,
    pub p1_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScProfanityCheckString {
    pub success: bool,
    pub ret: bool,
    pub token_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxMessageGetDataString {
    pub success: bool,
    pub ret: bool,
    pub out_: Option<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScEmailGetNumRetrievedEmails {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScGamerdataGetString {
    pub success: bool,
    pub ret: bool,
    pub value_: Option<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateGetAddStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScProfanityGetCheckIsPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScGamerdataGetBool {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScProfanityGetStringPassed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxGetBountyDataAtIndex {
    pub success: bool,
    pub ret: bool,
    pub out_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScPauseNewsGetPendingStory {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetExtraDataIntById {
    pub success: bool,
    pub ret: bool,
    pub p2_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateGetCheckIsPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScTransitionNewsGetExtraDataIntTu {
    pub success: bool,
    pub ret: bool,
    pub p1_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxGetMessageIsReadAtIndex {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScProfanityGetStringStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxMessageDoApply {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetExtraDataString {
    pub success: bool,
    pub ret: bool,
    pub p1_: Option<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxMessageGetDataInt {
    pub success: bool,
    pub ret: bool,
    pub out_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventIsActiveById {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxSendUgcstatupdateToRecipList {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScGamerdataGetFloat {
    pub success: bool,
    pub ret: bool,
    pub value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxGetMessageTypeAtIndex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScHasNewRockstarMsg {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCacheNewRockstarMsgs {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetDisplayName {
    pub success: bool,
    pub ret: bool,
    pub p0_: Option<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScPresenceAttrSetFloat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetExtraDataFloatById {
    pub success: bool,
    pub ret: bool,
    pub p2_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScGamerdataGetInt {
    pub success: bool,
    pub ret: bool,
    pub value_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateSetPlateData {
    pub success: bool,
    pub ret: bool,
    pub plate_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateGetIsvalidIsPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScTransitionNewsShowNextItem {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxMessagePushGamerT0RecipList {
    pub success: bool,
    pub ret: (),
    pub gamer_handle_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScGetNewRockstarMsg {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScPauseNewsInitStarterPack {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventGetExtraDataIntForType {
    pub success: bool,
    pub ret: bool,
    pub p1_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScPauseNewsShutdown {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScProfanityCheckStringUgc {
    pub success: bool,
    pub ret: bool,
    pub token_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateGetCheckIsValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxMessageGetRawTypeAtIndex {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScLicenseplateCheckString {
    pub success: bool,
    pub ret: bool,
    pub p1_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScTransitionNewsShowTimed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScCommunityEventIsActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfScInboxMessageGetDataBool {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsNpcPhone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSendMetricPunishBodyguard {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMcFormationEnds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetCheatIsActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoInsideTrack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsRecoverContrabandMission {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBcSmashAndGrab {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsStartedSessionInOfflinemode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoChip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsSwitchMcEmblem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventDeaddrop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsChangeMcRole {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetVehicleBailDistance {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsDarCheckpoint {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPackedStatIntCode {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsWriteAddColumn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoLuckySeven {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFreemodePrologueDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSlotIsLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsGunrunningMissionEnded {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFriendActivity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsGetNumberOfColumns {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCarclubChallenge {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetBool {
    pub success: bool,
    pub ret: bool,
    pub out_value_: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPresenceEventUpdatestatInt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsRosBet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsJobLtsRoundEnd {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId,
pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBcProtectionRacket {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPackedStatIntCode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventKingofthecastle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsShopItem {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetGxtLabel {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFreemodeCasinoMissionEnded {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsRobberyPrep {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventPennedin {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetFlyingAltitude {
    pub success: bool,
    pub ret: bool,
    pub out_value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCrateDropMissionDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsInstancedHeistEnded {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsPimenuHideOptions {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetUserId {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoInsideTrackLight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsDefendContrabandMission {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatResetAllOnlineCharacterStats {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsRivalBehavior {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHubEntry {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBwFragileGoods {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsChangeMcOutfit {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPackedNgIntStatKey {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsImportExportMissionDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetDate {
    pub success: bool,
    pub ret: bool,
    pub value_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventHuntbeast {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetNumberOfSeconds {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsWarehouseMissionEnded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHeist3Finale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsWriteAddColumnLong {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHeist4Hack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsReadSuccessful {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPresenceEventUpdatestatFloat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBusinessBattleEnded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsSmugglerMissionEnded {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSaveMigrationConsumeContent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetCurrentDrivingReverseDistance {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatStartRecordStat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadGetRowDataInfo {
    pub success: bool,
    pub ret: bool,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsClothChange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetPos {
    pub success: bool,
    pub ret: bool,
    pub out_x_: f32,
pub out_y_: f32,
pub out_z_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsStoneHatchetEnded {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsSwitchPassiveMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadGetRowDataFloat {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetProfileSettingCreatorDmDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventCheckpointcollection {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoBlackjack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartBeingBoss {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventPasstheparcel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatNetworkIncrementOnSuicide {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfChangeGoonLookingForWork {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsAppendDirectorMetric {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmMissionEnd {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsAwardXp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsAwardBadSport {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetFloat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsPlayerStyle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatDeleteSlot {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsKillYourself {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetBool {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatMigrateCheckAlreadyDone {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFastTrvl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsArcadeLoveMatch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSaveMigrationCancelPendingOperation {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBackgroundScriptAction {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsEarnedMcPoints {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBanAlert {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsArcadeGame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHeist3Prep {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBcCashing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetLicensePlate {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetProfileSettingCreatorCtfDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetCurrentSpeed {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetCancelSaveMigrationStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetOpenSavetypeInJob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsGetCacheNumberOfRows {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHubExit {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatCommunityStartSynch {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatMigrateCheckGetIsPlatformAvailable {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsQuitMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsNjvsVote {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadByRadius {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsIdleKick {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerHasDrivenAllVehicles {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBwHeadHunter {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCheatApplied {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHitContrabandDestroyLimit {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPackedIntStatKey {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatDisableStatsTracking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPresenceEventUpdatestatIntWithString {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetMaskedInt {
    pub success: bool,
    pub ret: bool,
    pub out_value_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventCompetitiveurbanwarfare {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoRouletteLight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsDroneUsage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsSpinWheel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBwAssault {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetProfileSettingValue {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCarclubPrize {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsOddjobDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetLicensePlate {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventChallenges {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetBlockSaves {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBossGoonUuid {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndBeingGoon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCreateMatchHistoryId2 {
    pub success: bool,
    pub ret: bool,
    pub player_account_id_: i32,
pub posix_time_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetChallengeFlyingDist {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceCloudMpStatsDownloadAndOverwriteLocalSave {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsArenaWarsSpectator {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSendMetricGhostingToPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsAwardNav {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsRandomMissionDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadGetRowDataEnd {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetJobActivityIdStarted {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBcPointToPoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetNumberOfMinutes {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetInt {
    pub success: bool,
    pub ret: bool,
    pub out_value_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHiredLimo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsAcquiredHiddenPackage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFreemodeStrandProgressionStatus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBcCarJacking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetMaskedInt {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMissionOver {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsReadClear {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBcSalvage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSavePending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsImpexpMissionEnded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatCloudSlotSaveFailed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadByScoreInt {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatCloudSlotLoadFailed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMasterControl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsNightclubMissionEnded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsDupeDetected {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetCurrentFrontWheelDistance {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventHotproperty {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsEnterSessionPack {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBwHuntTheBoss {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadGetRowDataInt {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetSaveMigrationStatus {
    pub success: bool,
    pub ret: i32,
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsInventory {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMcClubhouseActivity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsAcidMissionEnd {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsStopTrackingStunts {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetDate {
    pub success: bool,
    pub ret: bool,
    pub out_value_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatIsRecordingStat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventUrbanwarfare {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetUserId {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBwBossonbossdeathmatch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsClearCacheDataId {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsQuickfixTool {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsGetCacheDataRow {
    pub success: bool,
    pub ret: bool,
    pub p2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadFriendsByRow {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsStartTrackingStunts {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHeist3Hack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsNpcInvite {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBcMostWanted {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPackedStatGetIntStatIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoRoulette {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsPegasusAsPersonalAircraft {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartBeingGoon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatEnableStatsTracking {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatMigrateSavegameGetStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatClearDirtyReadDetected {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatIncrement {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsRaceCheckpoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsGetCacheExists {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetCurrentJumpDistance {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsActivityDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadGetRowDataStart {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatLoadPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsReadAnyPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsReadClearAll {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndBeingBoss {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatMigrateSavegameStart {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatLoad {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBwBellyOfTheBeast {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsJobLtsEnd {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId,
pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatStopRecordStat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatLocalResetAllOnlineCharacterStats {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatClearPendingSaves {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetString {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetCurrentRearWheelDistance {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadByRow {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId,
pub p3_: shared::MemoryBufferId,
pub p5_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSendMetricVipPoach {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsReadPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsRaceToPointMissionDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2WriteData {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCrateCreated {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetCurrentSkydivingDistance {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatCommunitySynchIsPending {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsDjUsage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetInt {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatRollbackSaveMigration {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetProfileSettingPrologueComplete {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsArenaWarsEnded {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCopyRankIntoNewSlot {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsCacheDataRow {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadByRank {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsPropChange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetCurrentDriveNocrashDistance {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventAtob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsRobberyFinale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSavePendingOrRequested {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMatchStarted {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatIsStatsTrackingEnabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsSpentPiCustomLoadout {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatCommunityGetHistory {
    pub success: bool,
    pub ret: bool,
    pub out_value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMissionEnded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsGetColumnType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventVehicletarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatsCompletedCharacterCreation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMissionVote {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetLoadSafeToProgressToMpFromSp {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHasPostedAllVehiclesDriven {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMinigameUsage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMissionStarted {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHeist4Finale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetCurrentPosixTime {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadByHandle {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadRankPrediction {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsGetColumnId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsLeaveJobChain {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetProfileSettingSpChopMissionComplete {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatMigrateCheckStart {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsSellContrabandMission {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsRankUp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatMigrateClearForRestart {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoThreeCardPokerLight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMissionCheckpoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2WriteDataForEventType {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHoldUpMissionDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMcRequestBike {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCollectiblePickedUp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetSaveMigrationConsumeContentStatus {
    pub success: bool,
    pub ret: i32,
    pub p0_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOrderBossVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsAcidRnd {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsSetJoinType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPackedTuIntStatKey {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBwYatchattack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfChangeUniform {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsSubWeap {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsClearCacheData {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoBlackjackLight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsAbandonedMc {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBuyContrabandMission {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsDjMissionEnded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetFloat {
    pub success: bool,
    pub ret: bool,
    pub out_value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmHeistPrepEnded {
    pub success: bool,
    pub ret: (),
    pub data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPackedStatBoolCode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHasSpecialeditionContent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsGunrunningRnd {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetPos {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPackedStatBoolCode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsWebsiteVisited {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatMigrateCheckGetPlatformStatus {
    pub success: bool,
    pub ret: i32,
    pub p1_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHeist3Drone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHeist4Prep {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSave {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetNumberOfDays {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBcFindersKeepers {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatCloudSlotLoadFailedCode {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetString {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoSlotMachineLight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadByScoreFloat {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetCurrentNearMissNocrashPrecise {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsWeaponModeChange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatClearSlotForReload {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsIdle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatLoadDirtyReadDetected {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsFmEventCriminaldamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoSlotMachine {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboardsGetCacheTime {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBwAirFreight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetRecordedValue {
    pub success: bool,
    pub ret: bool,
    pub value_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetProfileSettingCreatorRacesDone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLeaderboards2ReadByPlaform {
    pub success: bool,
    pub ret: bool,
    pub p0_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatGetNumberOfHours {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatSetBlockSaves {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsHeistSaveCheat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsMcKilledRivalMcMember {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsJobBend {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId,
pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSaveMigrationTransactionIdWarning {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoThreeCardPoker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsJobActivityEnd {
    pub success: bool,
    pub ret: (),
    pub p0_: shared::MemoryBufferId,
pub p1_: shared::MemoryBufferId,
pub p2_: shared::MemoryBufferId,
pub p3_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsArcadeCabinet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsShopmenuNav {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStatIsPlayerVehicleAboveOcean {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsExtraEvent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsBwSightseer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCasinoStoryMissionEnded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsInstMissionEnd {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlaystatsCarclubPoints {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNewLoadSceneLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveClipSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceAllowTimeBasedFadingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestCollisionAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStreamvolIsValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRestoreFocusEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerSwitchInterpOutDuration {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEndSrl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStreamvolCreateLine {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddModelToCreatorBudget {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLodscale {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerSwitchEstablishingShot {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveAnimSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFocusEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsModelAvehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSceneStreamingTracksCamPosThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStreamvolDelete {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStreamvolCreateFrustum {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSrlLongJumpMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerShortSwitchState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNewLoadSceneStart {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStreamvolCreateSphere {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasCollisionForModelLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesAnimDictExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityFocus {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasClipSetLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearFocus {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsModelInCdimage {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPrefetchSrl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetUsedCreatorBudget {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfStreamingRequests {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRenderHdOnly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestIpl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNetworkLoadingScene {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDitchPoliceModels {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowPlayerSwitchPan {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerSwitchState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressHdMapStreamingThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllMapdataCulled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIplGroupSwapIsActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerSwitchInterpOutCurrentTime {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSwitchSkippingDescent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerShortSwitchStyle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveNamedPtfxAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIplGroupSwapCancel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetStreaming {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestAnimSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGamePausesForStreaming {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSafeToStartPlayerSwitch {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowPlayerSwitchOutro {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsModelAped {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetReducePedModelBudget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerSwitchJumpCutIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStreamvolHasLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadGlobalWaterFile {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetReduceVehicleModelBudget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasNamedPtfxAssetLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsIplActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePtfxAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestModelsInRoom {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPopulationBudget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowPlayerSwitchAscent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestCollisionForModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSwitchToMultiFirstpartFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestPtfxAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIplGroupSwapStart {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopPlayerSwitch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasModelLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIslandEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBeginSrl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestMenuPedModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsNewLoadSceneActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSrlTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverrideLodscaleThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSwitchToMultiFirstpart {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNewLoadSceneStartSphere {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowPlayerSwitchDescent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMapdatacullboxEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPlayerSwitchType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInitCreatorBudget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIdealPlayerSwitchType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestNamedPtfxAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHdArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFocusPosAndVel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsStreamvolActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableSwitchOutroFx {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLoadAllObjectsNow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSrlReadaheadTimes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemapLodscaleRangeThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsModelValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNewLoadSceneStop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerSwitchOutro {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkUpdateLoadScene {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasAnimSetLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestAdditionalCollisionAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPtfxAssetLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehiclePopulationBudget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShutdownCreatorBudget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearHdArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSrlLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasAnimDictLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestClipSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestAnimDict {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableSwitchPauseBeforeDescent {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSwitchToMultiSecondpart {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayerSwitchInProgress {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSwitchReadyForDescent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInteriorActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetModelAsNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveIpl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSrlPostCutsceneCamera {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveModelFromCreatorBudget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIplGroupSwapFinish {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveAnimDict {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetGlobalWaterFile {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSrlForcePrestream {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartPlayerSwitch {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIplGroupSwapIsReady {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTimestep {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSin {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCeil {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVdist {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetThisThreadPriority {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWait {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSettimerb {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVmag {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSqrt {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTimera {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShiftRight {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVmag2 {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVdist2 {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartNewScriptWithArgs {
    pub success: bool,
    pub ret: i32,
    pub args_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfToFloat {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSettimera {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartNewScriptWithNameHashAndArgs {
    pub success: bool,
    pub ret: i32,
    pub args_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTimerb {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCos {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPow {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartNewScript {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLog10 {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartNewScriptWithNameHash {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfShiftLeft {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRound {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFloor {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointRecordingGetSpeedAtPoint {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSequenceProgress {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScenarioGroupEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAnimRate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoToEntityWhileAimingAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointPlaybackStartShootingAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParachuteTaskThrust {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskFollowWaypointRecording {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskShootAtEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskRappelFromHeli {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesScenarioOfTypeExistInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskJump {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetScenarioTypesEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointPlaybackPause {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehiclePark {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskClearLookAt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleEscort {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTaskMoveNetworkEnableCollisionOnNetworkCloneWhenFixed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleShootAtPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoToCoordWhileAimingAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleWaypointPlaybackOverrideSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskScriptedAnimation {
    pub success: bool,
    pub ret: (),
    pub priority_low_data_: i32,
pub priority_mid_data_: i32,
pub priority_high_data_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAssistedMovementOverrideLoadDistanceThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskStartScenarioInPlace {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleDriveToCoordLongrange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskBoatMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskFollowNavMeshToCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedSecondaryTask {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskFollowNavMeshToCoordAdvanced {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleGotoNavmesh {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskAgitatedActionConfrontResponse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPutPedDirectlyIntoMelee {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoToCoordAnyMeansExtraParams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskTurnPedToFaceCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleHeliProtect {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskExtendRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDesiredMoveBlendRatio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPhoneGestureAnimTotalTime {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveSpecificCoverBlockingAreas {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSweepAimEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAnimWeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointPlaybackStartAimingAtPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSmartFleePed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPatrolRouteLink {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPlaneMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointPlaybackResume {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMoveBlendRatioSprinting {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedWaypointProgress {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskUseNearestScenarioToCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddScriptedCoverArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPedHasUseScenarioTask {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskMoveNetworkAdvancedByNameWithInitParams {
    pub success: bool,
    pub ret: (),
    pub initial_parameters_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedGettingUp {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskCombatHatedTargetsAroundPedTimed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPlaneChase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskMoveNetworkByName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskDriveBy {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointRecordingGetCoord {
    pub success: bool,
    pub ret: bool,
    pub coord_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskFollowToOffsetOfEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTaskMoveNetworkReadyForTransition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleFollowWaypointRecording {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGotoEntityOffsetXy {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMoveBlendRatioStill {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAssistedMovementRemoveRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScenarioGroupEnabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTaskMoveNetworkSignalLocalFloat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearSequenceTask {
    pub success: bool,
    pub ret: (),
    pub task_sequence_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPathPreferToAvoidWater {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSeekCoverToCoords {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloseSequenceTask {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScenarioTypeEnabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClipSetForScriptedGunTask {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleChase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskMoveNetworkByNameWithInitParams {
    pub success: bool,
    pub ret: (),
    pub initial_parameters_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedRunningArrestTask {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsControlledVehicleUnableToGetToRoad {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskCower {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskStopPhoneGestureAnimation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDriveTaskMaxCruiseSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleWaypointTargetPoint {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPutPedDirectlyIntoCover {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetExclusiveScenarioGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPathAvoidFire {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleAimAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTaskMoveNetworkSignalFloat {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskShockingEventReact {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddCoverBlockingArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskShootAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPhoneGestureAnimCurrentTime {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointPlaybackStopAimingOrShooting {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleDriveWander {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGuardCurrentPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskCombatHatedTargetsInArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskForceMotionState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskLeaveAnyVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleShootAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPatrolTaskInfo {
    pub success: bool,
    pub ret: bool,
    pub time_left_at_node_: i32,
pub node_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointRecordingGetNumPoints {
    pub success: bool,
    pub ret: bool,
    pub points_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetActiveVehicleMissionType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetExclusiveScenarioGroup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearVehicleCrashTask {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedSprinting {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSequenceToRepeat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskUseNearestScenarioToCoordWarp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScriptedCoverPointCoords {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskFollowPointRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUseWaypointRecordingAsAssistedMovementRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPedSlideToCoordHdgRate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesScenarioExistInArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPerformSequence {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskTurnPedToFaceEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoToCoordAnyMeans {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDriveTaskCruiseSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleWaypointPlaybackUseDefaultSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddVehicleSubtaskAttackCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskUseMobilePhoneTimed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSkyDive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAssistedMovementIsRouteLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearDefaultPrimaryTask {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoStraightToCoordRelativeToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedPlayingBaseClipInScenario {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskReloadWeapon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNavmeshRouteResult {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTaskVehicleChaseIdealPursuitDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointPlaybackUseDefaultSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskAimGunAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUncuffPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskWanderSpecific {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskLookAtEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehiclePlayAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoToEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGlobalMinBirdFlightHeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskWarpPedDirectlyIntoCover {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskLookAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAnimLooped {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointPlaybackGetIsPaused {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTaskMoveNetworkState {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskThrowProjectile {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskReactAndFleePed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayAnimOnRunningScenario {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedCuffed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSeekCoverFromPos {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeletePatrolRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPlayEntityScriptedAnim {
    pub success: bool,
    pub ret: (),
    pub priority_low_data_: i32,
pub priority_mid_data_: i32,
pub priority_high_data_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPathCanUseLadders {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetScriptTaskStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsScenarioOccupied {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskExitCover {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskAimGunScripted {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskShuffleToNextVehicleSeat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSweepAimPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskCombatHatedTargetsAroundPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointPlaybackOverrideSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskEveryoneLeaveVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAssistedMovementRequestRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPlayAnimAdvanced {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskFlushRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTaskMoveNetworkAnimSet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSeekCoverFromPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedDesiredMoveBlendRatio {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddVehicleSubtaskAttackPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskAimGunScriptedWithTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTaskMoveNetworkSignalFloatLerpRate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsDrivebyTaskUnderneathDrivingTask {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPathClimbCostModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPerformSequenceFromProgress {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointPlaybackStartAimingAtCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskClimb {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleWaypointPlaybackPause {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPerformSequenceLocally {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskChatToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHighFallTask {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPathCanUseClimbovers {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPatrolRouteNode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPlayPhoneGestureAnimation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCanPlayAmbientIdles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedBeingArrested {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSetBlockingOfNonTemporaryEvents {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskStandStill {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTaskMoveNetworkActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPlaneTaxi {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSetSphereDefensiveArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskAchieveHeading {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskCombatPedTimed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleMissionPedTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSmartFleeCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskClearDefensiveArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskOpenVehicleDoor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPlantBomb {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateTaskAimGunScriptedTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoToEntityWhileAimingAtEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskUseNearestScenarioChainToCoordWarp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopAnimTask {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleWaypointProgress {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskWarpPedIntoVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskAimGunAtEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGetOffBoat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTaskRappelDownWallState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestWaypointRecording {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskUseNearestScenarioChainToCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSwapWeapon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMountedWeaponTaskUnderneathDrivingTask {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOpenPatrolRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoToCoordAndAimAtHatedEntitiesNearCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWaypointDistanceAlongRoute {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTaskMoveNetworkSignalBool {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesScriptedCoverPointExistAtCoords {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateTaskHandsUpDuration {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGotoEntityAiming {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedActiveInScenario {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskStealthKill {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedTasksImmediately {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetExpectedCloneNextTaskMoveNetworkState {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedStill {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskHeliChase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskToggleDuck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskStandGuard {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveCoverPoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreatePatrolRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClosePatrolRoute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsTaskActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTaskMoveNetworkSignalBool {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoToCoordWhileAimingAtEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskParachuteToTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskHeliEscortHeli {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTaskMoveNetworkEvent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointRecordingGetClosestWaypoint {
    pub success: bool,
    pub ret: bool,
    pub point_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskClimbLadder {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlayingPhoneGestureAnim {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoToCoordAnyMeansExtraParamsWithCruiseSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateTaskSweepAimPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskWanderStandard {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskUseMobilePhone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPatrol {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPlaneLand {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskEnterVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSubmarineGotoAndStop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParachuteTaskTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearDrivebyTaskUnderneathDrivingTask {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleTempAction {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedRunning {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNavmeshRouteDistanceRemaining {
    pub success: bool,
    pub ret: i32,
    pub distance_remaining_: f32,
pub is_path_ready_: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGuardSphereDefensiveArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsWaypointRecordingLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTaskVehicleChaseBehaviorFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMountedWeaponTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskWrithe {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestTaskMoveNetworkStateTransition {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPedSlideToCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGuardAssignedDefensiveArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskParachute {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskLeaveVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSeekCoverToCoverPoint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMoveBlendRatioRunning {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAssistedMovementSetRouteProperties {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskMoveNetworkAdvancedByName {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTaskMoveNetworkSignalFloat {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddCoverPoint {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGoStraightToCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDriveTaskDrivingStyle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskHeliMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveAllCoverBlockingAreas {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPrimaryVehicleTask {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleWaypointPlaybackResume {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfControlMountedWeapon {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetScenarioGroupsEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAnimPhase {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedWalking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedInWrithe {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsWaypointPlaybackGoingOnForPed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskWanderInArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedTasks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleDriveToCoord {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPathCanDropFromHeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskGotoEntityOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleAimAtPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedStrafing {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUpdateTaskSweepAimEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDrivebyTaskTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskStayInCover {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedWaypointDistance {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfWaypointPlaybackStartShootingAtPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPause {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOpenSequenceTask {
    pub success: bool,
    pub ret: (),
    pub task_sequence_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPlayAnim {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskRappelDownWallUsingClipsetOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScenarioTypeEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSetDecisionMaker {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedWaypointRouteOffset {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopAnimPlayback {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskSynchronizedScene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleMissionCoorsTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMoveBlendRatioWalking {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskCombatPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNextDesiredMoveState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskHandsUp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedPathMayEnterWater {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskArrestPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsWaypointPlaybackGoingOnForVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskPlaneGotoPreciseVtol {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesScenarioGroupExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskStartScenarioAtPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveCoverBlockingAreasAtPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTaskVehicleFollow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveWaypointRecording {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveVehicleHighDetailModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleModGen9Exclusive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleClassEstimatedMaxSpeed {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTyreWearRate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetArriveDistanceOverrideForVehiclePersuitAttack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleHasBeenDrivenFlag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleSetOverrideExtenableSideRatio {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLastShuntVehicle {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsVehicleDisabledByEmp {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRandomVehicleModelInMemory {
    pub success: bool,
    pub ret: (),
    pub model_hash_: u32,
pub success_indicator_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTurretTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetShouldLerpFromAiToFullRecording {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleInactiveDuringPlayback {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCanEjectPassengersIfLocked {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDropsMoneyWhenBlownUp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPreloadModsFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleIsRacing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTrainCarriage {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSubmarineNumberOfAirLeaks {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleRudderBroken {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeliCombatOffset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearVehicleGeneratorAreaOfInterest {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBoatDisableAvoidance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleInfluencesWantedLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHydraulicSuspensionRaiseFactor {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlaneSectionDamageScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleStartParachuting {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCanDeformWheels {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupRopeType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachVehicleFromCargobob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTotalDurationOfVehicleRecording {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleWindowTint {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowBoatBoomToAnimate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopAllGarageActivity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleModelIsSuppressed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTotalDurationOfVehicleRecordingId {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveRoadNodeSpeedZone {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleHandlingOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetShortSlowdownForLanding {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDontProcessVehicleGlass {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTurretSpeedThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleFixed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreAllVehicleWindowsIntact {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCanUseHydraulics {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesExtraExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleInBurnout {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWheelieEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableVehicleExplosionsDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleAttachedToTowTruck {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleSearchlight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTrainCruiseSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachVehicleOnToTrailer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesCargobobHavePickUpRope {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCustomPathNodeStreamingRadius {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleFrictionOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCanLeakPetrol {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetConvertibleRoofLatchState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleHasUnbreakableLights {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetOpenRearDoorsOnExplosion {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleSetEnableRampCarSideImpulse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasVehicleAssetLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlaybackGoingOnForVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleTrailerVehicle {
    pub success: bool,
    pub ret: bool,
    pub trailer_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleStaysFrozenWhenCleanedUp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleUseAlternateHandling {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsWheelsRetracted {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleAllowHomingMissleLockonSynced {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleHighDetail {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleModKit {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForcePlaybackRecordedVehicleUpdate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobExcludeFromPickupEntity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleActAsIfHighSpeedForFragSmashing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleHeadlightShadows {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleExtraColours {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorsLockedForAllTeams {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRotationOfVehicleRecordingAtTime {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCanEngineMissfire {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleActiveForPedNavigation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableWeaponBladeForces {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleRecordingId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedUsingVehicleDoor {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTrainTrackSpawnFrequency {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetVehicleWheels {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteScriptVehicleGenerator {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleReduceGrip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleSeatFree {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowTrainToBeRemovedByPopulation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleRespectsLocksWhenHasDriver {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisablePlaneAileron {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDensityMultiplierThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleEngineOn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNumberOfPassengers {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanAnchorBoatHereIgnorePlayers {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisablePretendOccupants {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleDoorLockStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesVehicleHaveWeapons {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfBringVehicleToHalt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFarDrawVehicles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanAnchorBoatHere {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableBmxExtraTrickForces {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableExplodeFromBodyDamageOnCollision {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleGeneratesEngineShockingEvents {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleBumperBouncing {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHydraulicsControl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleSetRampAndRammingCarsTakeDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleLayoutHash {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnyPedRappellingFromHeli {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleClass {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleStoppedAtTrafficLights {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIncreaseWheelCrushDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachVehicleToTowTruck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleWheelsCanBreak {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartPlaybackRecordedVehicleUsingAi {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfToggleVehicleMod {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTransformToCar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlaneEngineHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTrailerInverseMassScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleNeonEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModelNumberOfSeats {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGarbageTrucks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleHasBeenOwnedByPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDisableTowing {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleLivery {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanCargobobPickUpEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleWillTellOthersToHurry {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNumOfBrokenLoosenParts {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddRoadNodeSpeedZone {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreAnyVehicleSeatsFree {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableHoverModeFlight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPositionInRecording {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTimedExplosion {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDriftTyresSet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleClassMaxAcceleration {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDoorAllowedToBeBrokenOff {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTrailerAttachmentEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddVehicleStuckCheckWithWarp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCanBeUsedByFleeingPeds {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasVehicleRecordingBeenLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanShuffleSeat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleFlightNozzlePosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRenderTrainAsDerailed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleIsConsideredByPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableVehicleTurretMovementThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceSubmarineSurfaceMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleColourCombination {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumModKits {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllowVehicleExplodesOnContact {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllVehicleGeneratorsActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleLights {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCloseBombBayDoors {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleOccupantsTakeExplosiveDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientVehicleNeonEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleCurrentTimeInSlipStream {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCustomSecondaryColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHasRocketBoost {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCombatMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStabiliseEntityAttachedToHeli {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCanBeTargetted {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFindHandlerVehicleContainerIsAttachedTo {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableVehiclePetrolTankDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForkliftForkHeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRandomVehicleInSphere {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTyreWearRateScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAquadbike {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetReducedSuspensionForce {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetOutriggersDeployed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleEnveffScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorAutoLock {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfVehicleColours {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleExtraColours {
    pub success: bool,
    pub ret: (),
    pub pearlescent_color_: i32,
pub wheel_color_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachVehicleToTrailer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsRocketBoostActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleParachuteDeployed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleXenonLightColorIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSubmarineIsUnderDesignDepth {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleStrong {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleDoorFullyOpen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeliTailBoomCanBreakOff {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartPlaybackRecordedVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeliMainRotorHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleHomingLockedontoState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleAiCanUseExclusiveSeats {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachVehicleToCargobob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleUseBoostButtonForWheelRetract {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlaneLandingGearIntact {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTransformToSubmarineUsesAlternateInput {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleModel {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFormationLeader {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCanSaveInGarage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNumOfBrokenOffParts {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleSteerBias {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentPlaybackForVehicle {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGroundEffectReducesDrag {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleAlarmActivated {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSpecialFlightModeTargetRatio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleModColor1 {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearVehiclePetroltankFireCulprit {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfReleasePreloadMods {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleWeaponRestrictedAmmo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModIdentifierHash {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleWeaponDamageScale {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAboat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleEngineHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableVehiclePetrolTankFires {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleBumperBrokenOff {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveVehiclesFromGeneratorsInArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleWindowIntact {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleWheelType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfInstantlyFillVehiclePopulation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleProducingSlipStream {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModColor2Name {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleOnGroundProperly {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTransformRateForAnimation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableWantedConesResponse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleStolen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleClassMaxBraking {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleDriveable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfVehicleNumberPlates {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCanBeVisiblyDamaged {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlanePropellerHealth {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleSirenOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleSetParachuteModelOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForceVehicleEngineDamageByBullet {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDamageScale {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesVehicleAllowRappel {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPoliceFocusWillTrackVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleDeformationAtPos {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleColours {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleClassMaxAgility {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableIndividualPlanePropeller {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleHasKers {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorsLockedForPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCanLeakOil {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkEnableEmptyCrowdingVehiclesRemoval {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetModSlotName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCanVehicleBePlacedHere {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCanResprayVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleAconvertible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWheelsRetractedInstantly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModelAccelerationMaxMods {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsExtraBrokenOff {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFakeSuspensionLoweringAmount {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModelMaxTraction {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleEstimatedMaxSpeed {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGliderActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopPlaybackRecordedVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddVehicleCombatAngledAvoidanceArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearVehicleCustomPrimaryColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTyreHealth {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleWeaponDisabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTankTurretPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupMagnetEnsurePickupEntityUpright {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobForceDontDetachVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleStopped {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTimePositionInRecording {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntityAttachedToHandlerFrame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleWindowTint {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesVehicleHaveStuckVehicleCheck {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTailLights {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAdditionalRotationForRecordedVehiclePlayback {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModelValue {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMissionTrainCoords {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTaxiLights {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreWingsOfPlaneIntact {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCanBreak {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedsCanFallOffThisVehicleFromLargeFallDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBoatVehicleModelAgility {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDriftTyres {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleProvidesCover {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteMissionTrain {
    pub success: bool,
    pub ret: (),
    pub train_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetInvertVehicleControls {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasVehiclePetroltankSetOnFireByEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUseDoubleClickForCarJump {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleAllowNoPassengersLockon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleAcceleration {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleNoExplosionDamageFromDriver {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleLivery2Count {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverridePlaneDamageThrehsold {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsLeftVehicleHeadlightDamaged {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearVehicleCustomSecondaryColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleLivery2 {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRollUpWindow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllLowPriorityVehicleGeneratorsActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleExtraColour6 {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleLivery {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnyVehicleNearPoint {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAnyEntityAttachedToHandlerFrame {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModKit {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPausePlaybackRecordedVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAnAmphibiousCar {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHeliLandingAreaBlocked {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsEntryPointForSeatClear {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateMissionTrain {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTrackVehicleVisibility {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsDoorValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleForwardSpeedXy {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSlowDownEffectDisabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBoatBoomPositionRatio {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlaybackSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupMagnetReducedStrength {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetOverrideVehicleDoorTorque {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleTyresCanBurst {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleStuckTimerUp {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleIsStolen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleHandbrake {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupMagnetPullRopeLength {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleColourCombination {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleImpatienceTimer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachContainerToHandlerFrameWhenLinedUp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasVehiclePhoneExplosiveDevice {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleMod {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPopOutVehicleWindscreen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearVehicleRouteHistory {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupMagnetReducedFalloff {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleReduceGripLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeliControlLaggingRateScalar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesCargobobHavePickupMagnet {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTyreFixed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetTyreWearRate {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlaybackToUseAiTryToRevertBackLater {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleHomingLockedontoState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleStopInstantlyWhenPlayerInactive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHoverModeWingRatio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehiclePetrolTankHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCustomPrimaryColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDoesVehicleHaveTombstone {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleExplodesOnHighExplosionDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobExtaPickupRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBikeEasyToLand {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteAllTrains {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTankStationary {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTyreHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleShootAtTarget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTaxiLightOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfArePlanePropellersIntact {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableMapCollision {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleSetEnableNormaliseRampCarVerticalVeloctiy {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPreloadVehicleMod {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBoatAnchor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNeonColour {
    pub success: bool,
    pub ret: (),
    pub r_: i32,
pub g_: i32,
pub b_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDontTerminateTaskWhenAchieved {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFixVehicleWindow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleMod {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorsShut {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfExplodeVehicleInCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetShouldResetTurretInScriptedCameras {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPositionOffsetForRecordedVehiclePlayback {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDirtLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleExtendedRemovalRange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRollDownWindow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSpeedBoostEffectDisabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreatePickUpRopeForCargobob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachContainerFromHandlerFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStopBringingVehicleToHalt {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorOpen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNumberPlateText {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleExtraColour5 {
    pub success: bool,
    pub ret: (),
    pub color_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehiclePetrolTankHealth {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleAllowHomingMissleLockon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartPlaybackRecordedVehicleWithFlags {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleExtra {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsCopVehicleInArea3d {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAcar {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeliResistToExplosion {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleTrailerParentVehicle {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRandomTrains {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllowRammingSoopOrRamp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModColor2 {
    pub success: bool,
    pub ret: (),
    pub paint_type_: i32,
pub color_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleModColor2 {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleWeaponRestrictedAmmo {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestVehicleAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRocketBoostActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisablePedStandOnTop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDetonationMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveVehicleStuckCheck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleCustomSecondaryColour {
    pub success: bool,
    pub ret: (),
    pub r_: i32,
pub g_: i32,
pub b_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSuppressNeonsOnVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLastPedInVehicleSeat {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRandomBoats {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsToggleModOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHydraulicSuspensionRaiseFactor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCarHighSpeedBumpSeverityMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleIsDummy {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRollDownWindows {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBoundsAffectWaterProbes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetIgnorePlanesSmallPitchChange {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleWeaponCanTargetObjects {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleBobbleheadVelocity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleAttachedToCargobob {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupRopeLengthForCargobob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTrailerLegsLowered {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleLiveryCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOpenBombBayDoors {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleForceInteriorlight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfUnpausePlaybackRecordedVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForceFixLinkMatrices {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleUseMoreRestrictiveSpawnChecks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetModTextLabel {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHandlerFrameLinedUpWithContainer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleGravity {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDisableHeightMapAvoidance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleUndriveable {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesVehicleHaveRoof {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleFullbeam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleAutomaticallyAttaches {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModelAcceleration {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNeonEnabled {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleSprayable {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFindSpawnCoordinatesForHeli {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleNeonColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHydraulicVehicleState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableRetractingWeaponBlades {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleDirtLevel {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleLockOnTarget {
    pub success: bool,
    pub ret: bool,
    pub entity_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRaiseConvertibleRoof {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBoatSinksWhenWrecked {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleSteeringBiasScalar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachVehicleFromTrailer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCarHasJump {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleNumberPlateTextIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModModifierValue {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmbientVehicleRangeMultiplierThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsVehicleSecondaryColourCustom {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableVehicleEngineFires {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasInstantFillVehiclePopulationFinished {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLowerForkliftForks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPositionOfVehicleRecordingIdAtTime {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumberOfVehicleDoors {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleBrakeLights {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveVehicleMod {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleHasStrongAxles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfModifyVehicleTopSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleLodMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorShut {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSkipTimeInPlaybackRecordedVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeliCanPickupEntityThatHasPickUpDisabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAjetski {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDeformationFixed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesVehicleExistWithDecorator {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleNumberPlateText {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTrailerLegsRaised {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDamageScales {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorsLockedForNonScriptPlayers {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemovePickUpRopeForCargobob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableAutomaticCrashTask {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForceHdVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleEngineCanDegrade {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCausesSwerving {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesVehicleHaveSearchlight {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityAttachedToCargobob {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleShuntOnStick {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddVehiclePhoneExplosiveDevice {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleKersAllowed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceSubThrottleForTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupMagnetActive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleGeneratorAreaOfInterest {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHaveVehicleModsStreamedIn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleFlightNozzlePositionImmediate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLandingGearState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAllVehicles {
    pub success: bool,
    pub ret: i32,
    pub vehs_struct_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCountermeasureAmmo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupMagnetSetAmbientMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleActAsIfHasSirenOn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartVehicleHorn {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehiclePlateType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBikeOnStand {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumVehicleWindowTints {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleSetJetWashForceEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleInCarModShop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateScriptVehicleGenerator {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSmashVehicleWindow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsBigVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleBlipThrottleRandomly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetInVehicleClipsetHashForSeat {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAplane {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleMaxTraction {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeliBladesFullSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupMagnetFalloff {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleColours {
    pub success: bool,
    pub ret: (),
    pub color_primary_: i32,
pub color_secondary_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAnAmphibiousQuadbike {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAttachEntityToCargobob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsVehicleShunting {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFleeingVehiclesUseSwitchedOffNodes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorsLockedForAllPlayers {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleWheelsCanBreakOffWhenBlowUp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCeilingHeight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGenerateVehicleCreationPosFromPaths {
    pub success: bool,
    pub ret: bool,
    pub out_vec_: shared::Vector3,
pub out_vec1_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkCapEmptyCrowdingVehiclesRemoval {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlaybackToUseAi {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumModColors {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorLatched {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleLivery2 {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestVehicleHighDetailModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveVehicleWindow {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleSetParachuteModelTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleInSubmarineMode {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleMaxNumberOfPassengers {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCanAdjustGroundClearance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsRightVehicleHeadlightDamaged {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleEnveffScale {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleCanDeployParachute {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleVisible {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTrainSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearVehiclePhoneExplosiveDevice {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDipStraightDownWhenCrashingPlane {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleNotStealableAmbiently {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGoonBossVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleForwardSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSkipToEndAndStopPlaybackRecordedVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAtrain {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHeliTailBoomHealth {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveVehicleAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLastDrivenVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlaneTurbulenceMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsMissionTrain {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleMaxBraking {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachVehicleFromAnyCargobob {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsVehicleEngineRunning {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHideTombstone {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHeliTailRotorHealth {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPlaybackUsingAiGoingOnForVehicle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAreFoldingWingsDeployed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachEntityFromCargobob {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateVehicle {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestVehicleRecording {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDisableCollisionUponCreation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleForceAfterburner {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableSuperdummy {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedExclusiveDriverOfVehicle {
    pub success: bool,
    pub ret: bool,
    pub out_index_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsBoatAnchored {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleOnAllWheels {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDisplayNameFromVehicleModel {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDeployFoldingWings {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAllowAmbientVehiclesToAvoidAdverseConditions {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetForceLowLodAnchorMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLastDrivenVehicle {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetClearFreezeWaitingOnCollisionOncePlayerEnters {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleLightMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModVariation {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRandomVehicleDensityMultiplierThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleWheelType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModColor1Name {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleStuckOnRoof {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLiveryName {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRandomVehicleBackBumperInSphere {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAbike {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleCheatPowerIncrease {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTyreSmokeColor {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleExclusiveDriver {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleSirenAudioOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleIndicatorLights {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleTyreSmokeColor {
    pub success: bool,
    pub ret: (),
    pub r_: i32,
pub g_: i32,
pub b_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleCustomPrimaryColour {
    pub success: bool,
    pub ret: (),
    pub r_: i32,
pub g_: i32,
pub b_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorsLocked {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUseDesiredZcruiseSpeedForLanding {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddVehicleUpsidedownCheck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleExtraColour6 {
    pub success: bool,
    pub ret: (),
    pub color_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleBodyHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorsLockedForTeam {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTaskVehicleGotoPlaneMinHeightAboveTerrain {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleDoorDamaged {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleHealthPercentage {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleKeepEngineOnWhenAbandoned {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfStartVehicleAlarm {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleLightsState {
    pub success: bool,
    pub ret: bool,
    pub lights_on_: bool,
pub highbeams_on_: bool
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleNeonIndexColour {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCarjackMissionRemovalParameters {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleTyreBurst {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfExplodeVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsBoatCapsized {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleMaxSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleUsesMpPlayerDamageMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedInVehicleSeat {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetMissionTrainAsNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    pub train_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleInteriorlight {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetLightsCutoffDistanceTweak {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsHeliPartBroken {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleHasParachute {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupMagnetStrength {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDoesVehicleHaveDamageDecals {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayersLastVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBoatWrecked {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTransformToSubmarine {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleWillForceOtherVehiclesToStop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleIndividualDoorsLocked {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAbicycle {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModelMaxBrakingMaxMods {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleNameDebug {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntryPointPosition {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupRopeLengthWithoutCreatingRopeForCargobob {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleSearchlightOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAllVehicleGeneratorsActiveInArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSwingBoatBoomFreely {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHydraulicWheelState {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachVehicleFromTowTruck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleBrokenPartsDontAffectAiHandling {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleUsePlayerLightSettings {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleEngineHealth {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleRemoveAggressiveCarjackMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleLimitSpeedWhenPlayerInactive {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveVehicleUpsidedownCheck {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRandomVehicleFrontBumperInSphere {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSubmarineCrushDepths {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTurretHidden {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfForceSubmarineNeurtalBuoyancy {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleBeingBroughtToHalt {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetFlyingVehicleModelAgility {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetOverrideNitrousLevel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetTyreMaximumGripDifferenceDueToWearRate {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleIndividualDoorLockStatus {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetNumberOfParkedVehicles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleUsesLargeRearRamp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAttachedPickUpHookPosition {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleAlarm {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableVerticalFlightModeTransition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleInGarageArea {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupRopeDampingMultiplier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleReadyForCleanup {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfControlLandingGear {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLockDoorsWhenNoLongerNeeded {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAreBombBayDoorsOpen {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetachVehicleFromAnyTowTruck {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSpecialFlightModeRatio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPositionOfVehicleRecordingAtTime {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleExtraTurnedOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleAvoidPlayerVehicleRiotVanMission {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleSetOverrideSideRatio {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleAttachedToCargobob {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableDamageWithPickedUpEntity {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableRandomTrainsThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleIsMercenary {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorBroken {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleExplodesOnExplosionDamageAtZeroBodyHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetVehicleStuckTimer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleHasMutedSirens {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptVehicleGenerator {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRandomBoatsMp {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleFlightNozzlePosition {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestVehicleDial {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDontAllowPlayerToEnterVehicleIfLockedForPlayer {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleClassMaxTraction {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModelMaxBraking {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHasRetractableWheels {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsThisModelAheli {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleSteerForBuildings {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfLowerConvertibleRoof {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleClassFromName {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleSize {
    pub success: bool,
    pub ret: (),
    pub out1_: shared::Vector3,
pub out2_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleActiveDuringPlayback {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptRocketBoostRechargeTime {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearLastDrivenVehicle {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleUseCutsceneWheelCompression {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleUsedForPilotSchool {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlaneResistToExplosion {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetFormationLeader {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupMagnetSetTargetedMode {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveVehicleCombatAvoidanceArea {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsTurretSeat {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfPopOffVehicleRoofWithImpulse {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNumVehicleMods {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBoatRemainsAnchoredWhilePlayerIsDriver {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleXenonLightColorIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleHasLandingGear {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCopyVehicleDamages {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleCauseOfDestruction {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHeliMainRotorHealth {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleBrake {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAircraftPilotSkillNoiseScalar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableTurretMovement {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleHomingLockonState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEnableVehicleSlipstreaming {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeliTurbulenceScalar {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsVehicleAttachedToTrailer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBoatLowLodAnchorDistance {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleMayBeUsedByGotoPointAnyMeans {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHasVehicleBeenHitByShunt {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModColor1 {
    pub success: bool,
    pub ret: (),
    pub paint_type_: i32,
pub color_: i32,
pub pearlescent_color_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleBombAmmo {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDeleteVehicle {
    pub success: bool,
    pub ret: (),
    pub vehicle_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetParkedVehicleDensityMultiplierThisFrame {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTyresCanBurst {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfNetworkUseHighPrecisionTrainBlending {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBothVehicleHeadlightsDamaged {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTyreBurst {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetBoatIgnoreLandProbes {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCargobobPickupMagnetPullStrength {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDisableHeliExplodeFromBodyDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleColoursWhichCanBeSet {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDetonateVehiclePhoneExplosiveDevice {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCheckForEnoughRoomForPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetScriptRampImpulseScale {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetEntityAttachedToTowTruck {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleSlipstreamingShouldTimeOut {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetUseHigherCarJump {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsVehiclePrimaryColourCustom {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableVehcileDynamicAmbientScales {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetRotationOfVehicleRecordingIdAtTime {
    pub success: bool,
    pub ret: shared::Vector3,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveVehicleRecording {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleNumberPlateTextIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetSpecialFlightModeAllowed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleOutOfControl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableVehicleExplosionBreakOffParts {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleBodyHealth {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleDoorControl {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetConvertibleRoof {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfVehicleSetExtenableSideTargetRatio {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleColor {
    pub success: bool,
    pub ret: (),
    pub r_: i32,
pub g_: i32,
pub b_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleExtraColour5 {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModelEstimatedMaxSpeed {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSwingBoatBoomToRatio {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleSiren {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleBombAmmo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDisableVehicleWeapon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesScriptVehicleGeneratorExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWheelsExtendedInstantly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleDoorsLockedForPlayer {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetClosestVehicle {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfArePlaneControlPanelsIntact {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDistantCarsEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMakeNameFromVehicleModel {
    pub success: bool,
    pub ret: Option<String>,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleIsWanted {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsSeatWarpOnly {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleCountermeasureAmmo {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableVehicleDynamicAmbientScales {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAircraftIgnoreHightmapOptimisation {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetConvertibleRoofState {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleBulldozerArmPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetGlobalPositionOffsetForRecordedVehiclePlayback {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleBurnout {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleNeedsToBeHotwired {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleModKitType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCarBootOpen {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeliBladesSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSwitchTrainTrack {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetHeliTailRotorHealth {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetVehicleDoorAngleRatio {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetVehicleTowTruckArmPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetRocketBoostFill {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDeepOceanScaler {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTestVerticalProbeAgainstAllWater {
    pub success: bool,
    pub ret: i32,
    pub water_height_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCalmedWaveHeightScaler {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfResetDeepOceanScaler {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTestProbeAgainstAllWater {
    pub success: bool,
    pub ret: i32,
    pub water_height_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWaterHeightNoWaves {
    pub success: bool,
    pub ret: bool,
    pub height_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveExtraCalmingQuad {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetDeepOceanScaler {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfModifyWater {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWaterHeight {
    pub success: bool,
    pub ret: bool,
    pub height_: f32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddExtraCalmingQuad {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfTestProbeAgainstWater {
    pub success: bool,
    pub ret: bool,
    pub result_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAmmoInPedWeapon {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponTimeBetweenShots {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCurrentWeaponVisible {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetSelectedPedWeapon {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveAirDefenceSphere {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedWeaponComponentActive {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponComponentTypeModel {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPedLastWeaponDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentPedVehicleWeapon {
    pub success: bool,
    pub ret: bool,
    pub weapon_hash_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeaponAnimationOverride {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasEntityBeenDamagedByWeapon {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAmmo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedInfiniteAmmoClip {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveAllAirDefenceSpheres {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveWeaponComponentFromPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDropsInventoryWeapon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfMakePedReload {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddPedAmmoByType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedStunGunFiniteAmmo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedWeaponTintIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPedBeenDamagedByWeapon {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetAmmoInClip {
    pub success: bool,
    pub ret: bool,
    pub ammo_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponDamage {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGiveWeaponComponentToWeaponObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasWeaponAssetLoaded {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedAmmoByType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentPedWeapon {
    pub success: bool,
    pub ret: bool,
    pub weapon_hash_: u32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetCurrentPedWeaponEntityIndex {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponDamageType {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedInfiniteAmmo {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeapontypeSlot {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfFireAirDefenceSphereWeaponAtPosition {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedArmed {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeaponDamageModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDropsWeaponsWhenDead {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestWeaponHighDetailModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveWeaponFromPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeaponAoeModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsFlashLightOn {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponComponentVariantExtraModel {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedCycleVehicleWeaponsOnly {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedWeaponTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRequestWeaponAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponClipSize {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaxAmmoByType {
    pub success: bool,
    pub ret: bool,
    pub ammo_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesWeaponTakeWeaponComponent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeaponObjectComponentTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponTintCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAmmoByType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponComponentVariantExtraCount {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedCurrentWeaponSilenced {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGiveLoadoutToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedDropsWeapon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedLastWeaponImpactCoord {
    pub success: bool,
    pub ret: bool,
    pub coords_: shared::Vector3
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHidePedWeaponForScriptedCutscene {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasVehicleGotProjectileAttached {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCurrentPedVehicleWeapon {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasWeaponGotWeaponComponent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfAddAmmoToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedAmmoTypeFromWeapon {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaxRangeOfCurrentPedWeapon {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedChanceOfFiringBlanks {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetLockonDistanceOfCurrentPedWeapon {
    pub success: bool,
    pub ret: f32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetBestPedWeapon {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRefillAmmoInstantly {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPedGotWeapon {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateAirDefenceSphere {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsWeaponValid {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateWeaponObject {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeaponObjectCamoIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFlashLightActiveHistory {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCreateAirDefenceAngledArea {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedWeaponComponentTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedWeaponCamoIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaxAmmoInClip {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedAmmoToDrop {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveWeaponAsset {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearEntityLastWeaponDamage {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCurrentPedWeapon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGiveWeaponObjectToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGiveDelayedWeaponToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponComponentHudStats {
    pub success: bool,
    pub ret: bool,
    pub out_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponObjectComponentTintIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCanPedSelectInventoryWeapon {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedShootOrdnanceWeapon {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsPedWeaponReadyToShoot {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfCanUseWeaponOnParachute {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGiveWeaponToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeapontypeGroup {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfHasPedGotWeaponComponent {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfEnableLaserSightRendering {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponObjectFromPed {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponObjectTintIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfDoesAirDefenceSphereExist {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetFlashLightFadeDistance {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPedGadget {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeaponHudStats {
    pub success: bool,
    pub ret: bool,
    pub out_data_: shared::MemoryBufferId
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGiveWeaponComponentToPed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfIsAirDefenceSphereInArea {
    pub success: bool,
    pub ret: bool,
    pub out_zone_id_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetMaxAmmo {
    pub success: bool,
    pub ret: bool,
    pub ammo_: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetAmmoInClip {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetEqippedWeaponStartSpinningAtFullSpeed {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPickupAmmoAmountScaler {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeaponEffectDurationModifier {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetPlayerTargettableForAirDefenceSphere {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetCanPedSelectAllWeapons {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedWeapontypeInSlot {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedWeaponComponentTintIndex {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveAllPedWeapons {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetWeapontypeModel {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetPedOriginalAmmoTypeFromWeapon {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetIsPedGadgetEquipped {
    pub success: bool,
    pub ret: bool,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveWeaponComponentFromWeaponObject {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetWeaponObjectTintIndex {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfExplodeProjectiles {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfRemoveAllProjectilesOfType {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetZoneAtCoords {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetZonePopschedule {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfClearPopscheduleOverrideVehicleModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetZoneScumminess {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfOverridePopscheduleVehicleModel {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetHashOfMapAreaAtCoords {
    pub success: bool,
    pub ret: u32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetZoneFromNameId {
    pub success: bool,
    pub ret: i32,
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfSetZoneEnabled {
    pub success: bool,
    pub ret: (),
    
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetNameOfZone {
    pub success: bool,
    pub ret: Option<String>,
    
}
