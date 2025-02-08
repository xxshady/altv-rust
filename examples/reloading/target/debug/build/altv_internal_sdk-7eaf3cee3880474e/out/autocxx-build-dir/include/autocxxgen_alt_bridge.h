#ifndef __AUTOCXXGEN_H__
#define __AUTOCXXGEN_H__

#include <memory>
#include <string>
#include "cxx.h"
#include <stddef.h>
#ifndef AUTOCXX_NEW_AND_DELETE_PRELUDE
#define AUTOCXX_NEW_AND_DELETE_PRELUDE
// Mechanics to call custom operator new and delete
template <typename T>
auto delete_imp(T *ptr, int) -> decltype((void)T::operator delete(ptr)) {
  T::operator delete(ptr);
}
template <typename T> void delete_imp(T *ptr, long) { ::operator delete(ptr); }
template <typename T> void delete_appropriately(T *obj) {
  // 0 is a better match for the first 'delete_imp' so will match
  // preferentially.
  delete_imp(obj, 0);
}
template <typename T>
auto new_imp(size_t count, int) -> decltype(T::operator new(count)) {
  return T::operator new(count);
}
template <typename T> void *new_imp(size_t count, long) {
  return ::operator new(count);
}
template <typename T> T *new_appropriately() {
  // 0 is a better match for the first 'delete_imp' so will match
  // preferentially.
  return static_cast<T *>(new_imp<T>(sizeof(T), 0));
}
#endif // AUTOCXX_NEW_AND_DELETE_PRELUDE
#include "alt_bridge.h"
#include "alt_classes/ICore.h"
#include "alt_classes/IBaseObject.h"
#include "alt_classes/IWorldObject.h"
#include "alt_classes/IEntity.h"
#include "alt_classes/IPlayer.h"
#include "alt_classes/IVehicle.h"
#include "alt_classes/IPed.h"
#include "alt_classes/IObject.h"
#include "alt_classes/IColShape.h"
#include "alt_classes/IBlip.h"
#include "alt_classes/IMarker.h"
#include "alt_classes/ICheckpoint.h"
#include "alt_classes/CEvent.h"
#include "alt_classes/CCancellableEvent.h"
#include "alt_classes/IResource.h"
#include "alt_classes/IVoiceChannel.h"
#include "alt_classes/CConsoleCommandEvent.h"
#include "alt_classes/CServerScriptEvent.h"
#include "alt_classes/CClientScriptEvent.h"
#include "alt_classes/CPlayerConnectEvent.h"
#include "alt_classes/CPlayerDisconnectEvent.h"
#include "alt_classes/CColShapeEvent.h"
#include "alt_classes/IVirtualEntity.h"
#include "alt_classes/IVirtualEntityGroup.h"
#include "alt_classes/CWeaponDamageEvent.h"
#include "alt_classes/CNetOwnerChangeEvent.h"
#include "alt_classes/CPlayerDeathEvent.h"
#include "alt_classes/CPlayerDamageEvent.h"
#include "alt_classes/CPlayerEnteringVehicleEvent.h"
#include "alt_classes/CPlayerEnterVehicleEvent.h"
#include "alt_classes/CPlayerLeaveVehicleEvent.h"
#include "alt_classes/CPlayerChangeAnimationEvent.h"
#include "alt_classes/CPlayerChangeVehicleSeatEvent.h"
#include "alt_classes/CPlayerWeaponChangeEvent.h"
#include "alt_classes/CPlayerConnectDeniedEvent.h"
#include "alt_classes/CPlayerSpawnEvent.h"
#include "alt_classes/CPlayerRequestControlEvent.h"
#include "alt_classes/CPlayerDimensionChangeEvent.h"
#include "alt_classes/CPlayerChangeInteriorEvent.h"
#include "alt_classes/CConnectionQueueAddEvent.h"
#include "alt_classes/CConnectionQueueRemoveEvent.h"
#include "alt_classes/CPlayerHealEvent.h"
#include "alt_classes/CVehicleAttachEvent.h"
#include "alt_classes/CVehicleDetachEvent.h"
#include "alt_classes/CVehicleDestroyEvent.h"
#include "alt_classes/CVehicleDamageEvent.h"
#include "alt_classes/CVehicleHornEvent.h"
#include "alt_classes/CVehicleSirenEvent.h"
#include "alt_classes/CStartProjectileEvent.h"
#include "alt_classes/CFireEvent.h"
#include "alt_classes/CExplosionEvent.h"
#include "alt_classes/IConnectionInfo.h"
#include "alt_classes/VehicleModelInfo.h"
#include "alt_classes/PedModelInfo.h"
#include "alt_classes/CMetaChangeEvent.h"
#include "alt_classes/CGlobalMetaDataChangeEvent.h"
#include "alt_classes/CGlobalSyncedMetaDataChangeEvent.h"
#include "alt_classes/CSyncedMetaDataChangeEvent.h"
#include "alt_classes/CStreamSyncedMetaDataChangeEvent.h"
#include "alt_classes/CLocalMetaDataChangeEvent.h"
#include "alt_classes/CResourceStopEvent.h"
#include "alt_classes/CResourceStartEvent.h"
#include "alt_classes/CResourceStartEvent.h"
#include "alt_classes/CVoiceConnectionEvent.h"
#include "alt_classes/CRequestSyncedSceneEvent.h"
#include "alt_classes/CStartSyncedSceneEvent.h"
#include "alt_classes/CStopSyncedSceneEvent.h"
#include "alt_classes/CUpdateSyncedSceneEvent.h"
#include "alt_classes/CClientDeleteObjectEvent.h"
#include "alt_classes/CClientRequestObjectEvent.h"
#include "alt_classes/CGivePedScriptedTaskEvent.h"
#include "alt_classes/CPedDeathEvent.h"
#include "alt_classes/CPedDamageEvent.h"
#include "alt_classes/CPedHealEvent.h"

typedef Config::internal::ValueWrapper<Config::Value> Config_internal_ValueWrapper_Config_Value_AutocxxConcrete;
typedef int c_int;

inline std::unique_ptr<std::string> autocxx_make_string_0xd5d0abec981e3e3a(::rust::Str str) { return std::make_unique<std::string>(std::string(str)); }
inline ConstMValueWrapper* ConstMValueWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<ConstMValueWrapper>();; }
inline void ConstMValueWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { delete_appropriately<ConstMValueWrapper>(arg0);; }
inline ConfigDictPairWrapper* ConfigDictPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<ConfigDictPairWrapper>();; }
inline void ConfigDictPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(ConfigDictPairWrapper* arg0)  { delete_appropriately<ConfigDictPairWrapper>(arg0);; }
inline Vector3Wrapper* Vector3Wrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<Vector3Wrapper>();; }
inline void Vector3Wrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(Vector3Wrapper* arg0)  { delete_appropriately<Vector3Wrapper>(arg0);; }
inline Vector2Wrapper* Vector2Wrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<Vector2Wrapper>();; }
inline void Vector2Wrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(Vector2Wrapper* arg0)  { delete_appropriately<Vector2Wrapper>(arg0);; }
inline RGBAWrapper* RGBAWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<RGBAWrapper>();; }
inline void RGBAWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(RGBAWrapper* arg0)  { delete_appropriately<RGBAWrapper>(arg0);; }
inline WeaponWrapper* WeaponWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<WeaponWrapper>();; }
inline void WeaponWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(WeaponWrapper* arg0)  { delete_appropriately<WeaponWrapper>(arg0);; }
inline FireInfoWrapper* FireInfoWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<FireInfoWrapper>();; }
inline void FireInfoWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(FireInfoWrapper* arg0)  { delete_appropriately<FireInfoWrapper>(arg0);; }
inline bool read_bool_autocxx_wrapper_0xd5d0abec981e3e3a(Config_internal_ValueWrapper_Config_Value_AutocxxConcrete* arg0)  { return config_node::read_bool(std::move(*arg0)); }
inline double read_f64_autocxx_wrapper_0xd5d0abec981e3e3a(Config_internal_ValueWrapper_Config_Value_AutocxxConcrete* arg0)  { return config_node::read_f64(std::move(*arg0)); }
inline std::unique_ptr<std::string> read_string_autocxx_wrapper_0xd5d0abec981e3e3a(Config_internal_ValueWrapper_Config_Value_AutocxxConcrete* arg0)  { return std::make_unique<std::string>(config_node::read_string(std::move(*arg0))); }
inline std::unique_ptr<std::vector<Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>> read_list_autocxx_wrapper_0xd5d0abec981e3e3a(Config_internal_ValueWrapper_Config_Value_AutocxxConcrete* arg0)  { return std::make_unique<std::vector<Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>>(config_node::read_list(std::move(*arg0))); }
inline std::unique_ptr<Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> copy_value_ptr_autocxx_wrapper_0xd5d0abec981e3e3a(const Config_internal_ValueWrapper_Config_Value_AutocxxConcrete& arg0)  { return std::make_unique<Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>(config_node::copy_value_ptr(arg0)); }
inline std::unique_ptr<std::vector<ConfigDictPairWrapper>> read_dict_autocxx_wrapper_0xd5d0abec981e3e3a(Config_internal_ValueWrapper_Config_Value_AutocxxConcrete* arg0)  { return std::make_unique<std::vector<ConfigDictPairWrapper>>(config_node::read_dict(std::move(*arg0))); }
inline std::unique_ptr<std::string> read_dict_pair_key_autocxx_wrapper_0xd5d0abec981e3e3a(const ConfigDictPairWrapper& arg0)  { return std::make_unique<std::string>(config_node::read_dict_pair_key(arg0)); }
inline std::unique_ptr<Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> read_dict_pair_value_autocxx_wrapper_0xd5d0abec981e3e3a(const ConfigDictPairWrapper& arg0)  { return std::make_unique<Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>(config_node::read_dict_pair_value(arg0)); }
inline std::unique_ptr<std::string> ICore_GetVersion_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<std::string>(ICore::GetVersion()); }
inline std::unique_ptr<std::string> ICore_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<std::string>(ICore::GetBranch()); }
inline void ICore_LogInfo_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, std::unique_ptr<std::string> arg1, alt::IResource* arg2)  { ICore::LogInfo(std::move(*arg0), std::move(*arg1), arg2); }
inline void ICore_LogDebug_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, std::unique_ptr<std::string> arg1, alt::IResource* arg2)  { ICore::LogDebug(std::move(*arg0), std::move(*arg1), arg2); }
inline void ICore_LogWarning_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, std::unique_ptr<std::string> arg1, alt::IResource* arg2)  { ICore::LogWarning(std::move(*arg0), std::move(*arg1), arg2); }
inline void ICore_LogError_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, std::unique_ptr<std::string> arg1, alt::IResource* arg2)  { ICore::LogError(std::move(*arg0), std::move(*arg1), arg2); }
inline void ICore_LogColored_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, std::unique_ptr<std::string> arg1, alt::IResource* arg2)  { ICore::LogColored(std::move(*arg0), std::move(*arg1), arg2); }
inline alt::IVirtualEntity* ICore_CreateVirtualEntity_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IVirtualEntityGroup* arg0, float arg1, float arg2, float arg3, uint32_t arg4, MValueUnorderedMapWrapper* arg5)  { return ICore::CreateVirtualEntity(arg0, arg1, arg2, arg3, arg4, std::move(*arg5)); }
inline alt::IColShape* ICore_CreateColShapePolygon_autocxx_wrapper_0xd5d0abec981e3e3a(float arg0, float arg1, Vector2Vec* arg2)  { return ICore::CreateColShapePolygon(arg0, arg1, std::move(*arg2)); }
inline uint32_t ICore_Hash_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { return ICore::Hash(std::move(*arg0)); }
inline bool ICore_FileExists_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { return ICore::FileExists(std::move(*arg0)); }
inline std::unique_ptr<std::string> ICore_FileRead_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { return std::make_unique<std::string>(ICore::FileRead(std::move(*arg0))); }
inline alt::IResource* ICore_GetResource_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { return ICore::GetResource(std::move(*arg0)); }
inline bool ICore_HasMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { return ICore::HasMetaData(std::move(*arg0)); }
inline void ICore_GetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(ICore::GetMetaData(std::move(*arg0))); }
inline void ICore_SetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, MValueMutWrapper* arg1)  { ICore::SetMetaData(std::move(*arg0), std::move(*arg1)); }
inline void ICore_DeleteMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { ICore::DeleteMetaData(std::move(*arg0)); }
inline std::unique_ptr<std::vector<std::string>> ICore_GetMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<std::vector<std::string>>(ICore::GetMetaDataKeys()); }
inline bool ICore_HasSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { return ICore::HasSyncedMetaData(std::move(*arg0)); }
inline void ICore_GetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(ICore::GetSyncedMetaData(std::move(*arg0))); }
inline std::unique_ptr<std::vector<std::string>> ICore_GetSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<std::vector<std::string>>(ICore::GetSyncedMetaDataKeys()); }
inline std::unique_ptr<std::vector<ResourcePtrWrapper>> ICore_GetAllResources_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<std::vector<ResourcePtrWrapper>>(ICore::GetAllResources()); }
inline std::unique_ptr<std::string> ICore_StringToSHA256_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { return std::make_unique<std::string>(ICore::StringToSHA256(std::move(*arg0))); }
inline std::unique_ptr<std::string> ICore_GetRootDirectory_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<std::string>(ICore::GetRootDirectory()); }
inline alt::IResource* ICore_StartResource_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { return ICore::StartResource(std::move(*arg0)); }
inline void ICore_StopResource_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { ICore::StopResource(std::move(*arg0)); }
inline void ICore_RestartResource_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { ICore::RestartResource(std::move(*arg0)); }
inline void ICore_AddClientConfigKey_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { ICore::AddClientConfigKey(std::move(*arg0)); }
inline void ICore_TriggerClientRPCAnswer_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IPlayer* arg0, uint16_t arg1, MValueMutWrapper* arg2, std::unique_ptr<std::string> arg3)  { ICore::TriggerClientRPCAnswer(arg0, arg1, std::move(*arg2), std::move(*arg3)); }
inline void ICore_SetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, MValueMutWrapper* arg1)  { ICore::SetSyncedMetaData(std::move(*arg0), std::move(*arg1)); }
inline void ICore_DeleteSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { ICore::DeleteSyncedMetaData(std::move(*arg0)); }
inline alt::IBlip* ICore_CreateBlip_autocxx_wrapper_0xd5d0abec981e3e3a(bool arg0, uint8_t arg1, float arg2, float arg3, float arg4, std::vector<PlayerPtrWrapper>* arg5)  { return ICore::CreateBlip(arg0, arg1, arg2, arg3, arg4, std::move(*arg5)); }
inline alt::IBlip* ICore_CreateBlip1_autocxx_wrapper_0xd5d0abec981e3e3a(bool arg0, uint8_t arg1, alt::IEntity* arg2, std::vector<PlayerPtrWrapper>* arg3)  { return ICore::CreateBlip(arg0, arg1, arg2, std::move(*arg3)); }
inline std::unique_ptr<std::vector<PlayerPtrWrapper>> ICore_GetPlayersByName_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { return std::make_unique<std::vector<PlayerPtrWrapper>>(ICore::GetPlayersByName(std::move(*arg0))); }
inline void ICore_SetPassword_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { ICore::SetPassword(std::move(*arg0)); }
inline uint64_t ICore_HashServerPassword_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0)  { return ICore::HashServerPassword(std::move(*arg0)); }
inline std::unique_ptr<std::vector<uint32_t>> ICore_GetLoadedVehicleModels_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<std::vector<uint32_t>>(ICore::GetLoadedVehicleModels()); }
inline std::unique_ptr<Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> ICore_GetServerConfig_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>(ICore::GetServerConfig()); }
inline std::unique_ptr<std::vector<BaseObjectPtrWrapper>> ICore_GetEntitiesInDimension_autocxx_wrapper_0xd5d0abec981e3e3a(int32_t arg0, uint64_t arg1)  { return std::make_unique<std::vector<BaseObjectPtrWrapper>>(ICore::GetEntitiesInDimension(arg0, arg1)); }
inline std::unique_ptr<std::vector<BaseObjectPtrWrapper>> ICore_GetEntitiesInRange_autocxx_wrapper_0xd5d0abec981e3e3a(float arg0, float arg1, float arg2, int32_t arg3, int32_t arg4, uint64_t arg5)  { return std::make_unique<std::vector<BaseObjectPtrWrapper>>(ICore::GetEntitiesInRange(arg0, arg1, arg2, arg3, arg4, arg5)); }
inline std::unique_ptr<std::vector<BaseObjectPtrWrapper>> ICore_GetClosestEntities_autocxx_wrapper_0xd5d0abec981e3e3a(float arg0, float arg1, float arg2, int32_t arg3, int32_t arg4, int32_t arg5, uint64_t arg6, uint8_t arg7)  { return std::make_unique<std::vector<BaseObjectPtrWrapper>>(ICore::GetClosestEntities(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)); }
inline void ICore_SetVoiceExternalPublic_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, uint16_t arg1)  { ICore::SetVoiceExternalPublic(std::move(*arg0), arg1); }
inline void ICore_SetVoiceExternal_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, uint16_t arg1)  { ICore::SetVoiceExternal(std::move(*arg0), arg1); }
inline bool IBaseObject_HasMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBaseObject* arg0, std::unique_ptr<std::string> arg1)  { return IBaseObject::HasMetaData(arg0, std::move(*arg1)); }
inline void IBaseObject_GetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBaseObject* arg0, std::unique_ptr<std::string> arg1, ConstMValueWrapper* arg2)  { new(arg2) ConstMValueWrapper(IBaseObject::GetMetaData(arg0, std::move(*arg1))); }
inline void IBaseObject_SetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IBaseObject* arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { IBaseObject::SetMetaData(arg0, std::move(*arg1), std::move(*arg2)); }
inline void IBaseObject_DeleteMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IBaseObject* arg0, std::unique_ptr<std::string> arg1)  { IBaseObject::DeleteMetaData(arg0, std::move(*arg1)); }
inline std::unique_ptr<std::vector<std::string>> IBaseObject_GetMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBaseObject* arg0)  { return std::make_unique<std::vector<std::string>>(IBaseObject::GetMetaDataKeys(arg0)); }
inline bool IBaseObject_HasSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBaseObject* arg0, std::unique_ptr<std::string> arg1)  { return IBaseObject::HasSyncedMetaData(arg0, std::move(*arg1)); }
inline void IBaseObject_GetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBaseObject* arg0, std::unique_ptr<std::string> arg1, ConstMValueWrapper* arg2)  { new(arg2) ConstMValueWrapper(IBaseObject::GetSyncedMetaData(arg0, std::move(*arg1))); }
inline std::unique_ptr<std::vector<std::string>> IBaseObject_GetSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBaseObject* arg0)  { return std::make_unique<std::vector<std::string>>(IBaseObject::GetSyncedMetaDataKeys(arg0)); }
inline void IBaseObject_SetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IBaseObject* arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { IBaseObject::SetSyncedMetaData(arg0, std::move(*arg1), std::move(*arg2)); }
inline void IBaseObject_DeleteSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IBaseObject* arg0, std::unique_ptr<std::string> arg1)  { IBaseObject::DeleteSyncedMetaData(arg0, std::move(*arg1)); }
inline void IWorldObject_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IWorldObject* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(IWorldObject::GetPosition(arg0)); }
inline void IEntity_GetRotation_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IEntity* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(IEntity::GetRotation(arg0)); }
inline bool IEntity_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IEntity* arg0, std::unique_ptr<std::string> arg1)  { return IEntity::HasStreamSyncedMetaData(arg0, std::move(*arg1)); }
inline void IEntity_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IEntity* arg0, std::unique_ptr<std::string> arg1, ConstMValueWrapper* arg2)  { new(arg2) ConstMValueWrapper(IEntity::GetStreamSyncedMetaData(arg0, std::move(*arg1))); }
inline std::unique_ptr<std::vector<std::string>> IEntity_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IEntity* arg0)  { return std::make_unique<std::vector<std::string>>(IEntity::GetStreamSyncedMetaDataKeys(arg0)); }
inline void IEntity_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IEntity* arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { IEntity::SetStreamSyncedMetaData(arg0, std::move(*arg1), std::move(*arg2)); }
inline void IEntity_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IEntity* arg0, std::unique_ptr<std::string> arg1)  { IEntity::DeleteStreamSyncedMetaData(arg0, std::move(*arg1)); }
inline void IEntity_AttachToEntity1_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IEntity* arg0, alt::IEntity* arg1, std::unique_ptr<std::string> arg2, std::unique_ptr<std::string> arg3, float arg4, float arg5, float arg6, float arg7, float arg8, float arg9, bool arg10, bool arg11)  { IEntity::AttachToEntity(arg0, arg1, std::move(*arg2), std::move(*arg3), arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11); }
inline std::unique_ptr<std::string> IPlayer_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::string>(IPlayer::GetName(arg0)); }
inline std::unique_ptr<std::vector<uint32_t>> IPlayer_GetCurrentWeaponComponents_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::vector<uint32_t>>(IPlayer::GetCurrentWeaponComponents(arg0)); }
inline void IPlayer_GetAimPos_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(IPlayer::GetAimPos(arg0)); }
inline void IPlayer_GetHeadRotation_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(IPlayer::GetHeadRotation(arg0)); }
inline void IPlayer_GetEntityAimOffset_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(IPlayer::GetEntityAimOffset(arg0)); }
inline std::unique_ptr<std::string> IPlayer_GetIP_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::string>(IPlayer::GetIP(arg0)); }
inline std::unique_ptr<std::string> IPlayer_GetSocialClubName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::string>(IPlayer::GetSocialClubName(arg0)); }
inline std::unique_ptr<std::string> IPlayer_GetHwid3_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::string>(IPlayer::GetHwid3(arg0)); }
inline std::unique_ptr<std::string> IPlayer_GetAuthToken_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::string>(IPlayer::GetAuthToken(arg0)); }
inline void IPlayer_Kick_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IPlayer* arg0, std::unique_ptr<std::string> arg1)  { IPlayer::Kick(arg0, std::move(*arg1)); }
inline void IPlayer_GetClothes_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, uint8_t arg1, alt::Cloth* arg2)  { new(arg2) alt::Cloth(IPlayer::GetClothes(arg0, arg1)); }
inline void IPlayer_GetDlcClothes_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, uint8_t arg1, alt::DlcCloth* arg2)  { new(arg2) alt::DlcCloth(IPlayer::GetDlcClothes(arg0, arg1)); }
inline void IPlayer_GetProps_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, uint8_t arg1, alt::Prop* arg2)  { new(arg2) alt::Prop(IPlayer::GetProps(arg0, arg1)); }
inline void IPlayer_GetDlcProps_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, uint8_t arg1, alt::DlcProp* arg2)  { new(arg2) alt::DlcProp(IPlayer::GetDlcProps(arg0, arg1)); }
inline void IPlayer_PlayAmbientSpeech_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IPlayer* arg0, std::unique_ptr<std::string> arg1, std::unique_ptr<std::string> arg2, uint32_t arg3)  { IPlayer::PlayAmbientSpeech(arg0, std::move(*arg1), std::move(*arg2), arg3); }
inline void IPlayer_GetHeadOverlay_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, uint8_t arg1, alt::HeadOverlay* arg2)  { new(arg2) alt::HeadOverlay(IPlayer::GetHeadOverlay(arg0, arg1)); }
inline void IPlayer_GetHeadBlendPaletteColor_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, uint8_t arg1, RGBAWrapper* arg2)  { new(arg2) RGBAWrapper(IPlayer::GetHeadBlendPaletteColor(arg0, arg1)); }
inline void IPlayer_GetHeadBlendData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, alt::HeadBlendData* arg1)  { new(arg1) alt::HeadBlendData(IPlayer::GetHeadBlendData(arg0)); }
inline std::unique_ptr<std::vector<WeaponWrapper>> IPlayer_GetWeapons_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::vector<WeaponWrapper>>(IPlayer::GetWeapons(arg0)); }
inline bool IPlayer_HasLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, std::unique_ptr<std::string> arg1)  { return IPlayer::HasLocalMetaData(arg0, std::move(*arg1)); }
inline void IPlayer_SetLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IPlayer* arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { IPlayer::SetLocalMetaData(arg0, std::move(*arg1), std::move(*arg2)); }
inline void IPlayer_GetLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, std::unique_ptr<std::string> arg1, ConstMValueWrapper* arg2)  { new(arg2) ConstMValueWrapper(IPlayer::GetLocalMetaData(arg0, std::move(*arg1))); }
inline void IPlayer_DeleteLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IPlayer* arg0, std::unique_ptr<std::string> arg1)  { IPlayer::DeleteLocalMetaData(arg0, std::move(*arg1)); }
inline std::unique_ptr<std::vector<std::string>> IPlayer_GetLocalMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::vector<std::string>>(IPlayer::GetLocalMetaDataKeys(arg0)); }
inline void IPlayer_PlayAnimation_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IPlayer* arg0, std::unique_ptr<std::string> arg1, std::unique_ptr<std::string> arg2, float arg3, float arg4, int arg5, int arg6, float arg7, bool arg8, bool arg9, bool arg10)  { IPlayer::PlayAnimation(arg0, std::move(*arg1), std::move(*arg2), arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10); }
inline void IPlayer_PlayScenario_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IPlayer* arg0, std::unique_ptr<std::string> arg1)  { IPlayer::PlayScenario(arg0, std::move(*arg1)); }
inline std::unique_ptr<std::vector<StreamedEntityWrapper>> IPlayer_GetStreamedEntities_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::vector<StreamedEntityWrapper>>(IPlayer::GetStreamedEntities(arg0)); }
inline void IPlayer_GetAmmoFlags_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0, uint32_t arg1, alt::AmmoFlags* arg2)  { new(arg2) alt::AmmoFlags(IPlayer::GetAmmoFlags(arg0, arg1)); }
inline std::unique_ptr<std::vector<alt::CDecoration>> IPlayer_GetDecorations_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::vector<alt::CDecoration>>(IPlayer::GetDecorations(arg0)); }
inline std::unique_ptr<std::string> IPlayer_GetCloudID_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::string>(IPlayer::GetCloudID(arg0)); }
inline std::unique_ptr<std::string> IPlayer_GetBloodDamageBase64_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IPlayer* arg0)  { return std::make_unique<std::string>(IPlayer::GetBloodDamageBase64(arg0)); }
inline void IPlayer_SetBloodDamageBase64_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IPlayer* arg0, std::unique_ptr<std::string> arg1)  { IPlayer::SetBloodDamageBase64(arg0, std::move(*arg1)); }
inline void IVehicle_GetPrimaryColorRGB_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0, RGBAWrapper* arg1)  { new(arg1) RGBAWrapper(IVehicle::GetPrimaryColorRGB(arg0)); }
inline void IVehicle_GetSecondaryColorRGB_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0, RGBAWrapper* arg1)  { new(arg1) RGBAWrapper(IVehicle::GetSecondaryColorRGB(arg0)); }
inline void IVehicle_GetTireSmokeColor_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0, RGBAWrapper* arg1)  { new(arg1) RGBAWrapper(IVehicle::GetTireSmokeColor(arg0)); }
inline std::unique_ptr<std::string> IVehicle_GetNumberplateText_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0)  { return std::make_unique<std::string>(IVehicle::GetNumberplateText(arg0)); }
inline void IVehicle_GetNeonColor_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0, RGBAWrapper* arg1)  { new(arg1) RGBAWrapper(IVehicle::GetNeonColor(arg0)); }
inline std::unique_ptr<std::string> IVehicle_GetAppearanceDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0)  { return std::make_unique<std::string>(IVehicle::GetAppearanceDataBase64(arg0)); }
inline std::unique_ptr<std::string> IVehicle_GetGameStateBase64_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0)  { return std::make_unique<std::string>(IVehicle::GetGameStateBase64(arg0)); }
inline std::unique_ptr<std::string> IVehicle_GetHealthDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0)  { return std::make_unique<std::string>(IVehicle::GetHealthDataBase64(arg0)); }
inline std::unique_ptr<std::string> IVehicle_GetDamageDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0)  { return std::make_unique<std::string>(IVehicle::GetDamageDataBase64(arg0)); }
inline std::unique_ptr<std::string> IVehicle_GetScriptDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0)  { return std::make_unique<std::string>(IVehicle::GetScriptDataBase64(arg0)); }
inline void IVehicle_GetVelocity_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(IVehicle::GetVelocity(arg0)); }
inline void IVehicle_SetNumberplateText_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IVehicle* arg0, std::unique_ptr<std::string> arg1)  { IVehicle::SetNumberplateText(arg0, std::move(*arg1)); }
inline void IVehicle_LoadAppearanceDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IVehicle* arg0, std::unique_ptr<std::string> arg1)  { IVehicle::LoadAppearanceDataFromBase64(arg0, std::move(*arg1)); }
inline void IVehicle_LoadDamageDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IVehicle* arg0, std::unique_ptr<std::string> arg1)  { IVehicle::LoadDamageDataFromBase64(arg0, std::move(*arg1)); }
inline void IVehicle_LoadScriptDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IVehicle* arg0, std::unique_ptr<std::string> arg1)  { IVehicle::LoadScriptDataFromBase64(arg0, std::move(*arg1)); }
inline void IVehicle_LoadGameStateFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IVehicle* arg0, std::unique_ptr<std::string> arg1)  { IVehicle::LoadGameStateFromBase64(arg0, std::move(*arg1)); }
inline void IVehicle_LoadHealthDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IVehicle* arg0, std::unique_ptr<std::string> arg1)  { IVehicle::LoadHealthDataFromBase64(arg0, std::move(*arg1)); }
inline void IVehicle_GetQuaternion_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVehicle* arg0, alt::Quaternion* arg1)  { new(arg1) alt::Quaternion(IVehicle::GetQuaternion(arg0)); }
inline void IBlip_GetScaleXY_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBlip* arg0, Vector2Wrapper* arg1)  { new(arg1) Vector2Wrapper(IBlip::GetScaleXY(arg0)); }
inline void IBlip_GetSecondaryColor_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBlip* arg0, RGBAWrapper* arg1)  { new(arg1) RGBAWrapper(IBlip::GetSecondaryColor(arg0)); }
inline std::unique_ptr<std::string> IBlip_GetGxtName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBlip* arg0)  { return std::make_unique<std::string>(IBlip::GetGxtName(arg0)); }
inline std::unique_ptr<std::string> IBlip_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBlip* arg0)  { return std::make_unique<std::string>(IBlip::GetName(arg0)); }
inline void IBlip_GetRouteColor_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBlip* arg0, RGBAWrapper* arg1)  { new(arg1) RGBAWrapper(IBlip::GetRouteColor(arg0)); }
inline std::unique_ptr<std::vector<PlayerPtrWrapper>> IBlip_GetTargets_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IBlip* arg0)  { return std::make_unique<std::vector<PlayerPtrWrapper>>(IBlip::GetTargets(arg0)); }
inline void IBlip_SetGxtName_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IBlip* arg0, std::unique_ptr<std::string> arg1)  { IBlip::SetGxtName(arg0, std::move(*arg1)); }
inline void IBlip_SetName_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IBlip* arg0, std::unique_ptr<std::string> arg1)  { IBlip::SetName(arg0, std::move(*arg1)); }
inline void IMarker_GetColor_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IMarker* arg0, RGBAWrapper* arg1)  { new(arg1) RGBAWrapper(IMarker::GetColor(arg0)); }
inline void IMarker_GetScale_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IMarker* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(IMarker::GetScale(arg0)); }
inline void IMarker_GetRotation_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IMarker* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(IMarker::GetRotation(arg0)); }
inline void IMarker_GetDirection_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IMarker* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(IMarker::GetDirection(arg0)); }
inline void ICheckpoint_GetColor_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::ICheckpoint* arg0, RGBAWrapper* arg1)  { new(arg1) RGBAWrapper(ICheckpoint::GetColor(arg0)); }
inline void ICheckpoint_GetIconColor_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::ICheckpoint* arg0, RGBAWrapper* arg1)  { new(arg1) RGBAWrapper(ICheckpoint::GetIconColor(arg0)); }
inline void ICheckpoint_GetNextPosition_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::ICheckpoint* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(ICheckpoint::GetNextPosition(arg0)); }
inline bool ICheckpoint_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::ICheckpoint* arg0, std::unique_ptr<std::string> arg1)  { return ICheckpoint::HasStreamSyncedMetaData(arg0, std::move(*arg1)); }
inline void ICheckpoint_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::ICheckpoint* arg0, std::unique_ptr<std::string> arg1, ConstMValueWrapper* arg2)  { new(arg2) ConstMValueWrapper(ICheckpoint::GetStreamSyncedMetaData(arg0, std::move(*arg1))); }
inline std::unique_ptr<std::vector<std::string>> ICheckpoint_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::ICheckpoint* arg0)  { return std::make_unique<std::vector<std::string>>(ICheckpoint::GetStreamSyncedMetaDataKeys(arg0)); }
inline void ICheckpoint_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::ICheckpoint* arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { ICheckpoint::SetStreamSyncedMetaData(arg0, std::move(*arg1), std::move(*arg2)); }
inline void ICheckpoint_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::ICheckpoint* arg0, std::unique_ptr<std::string> arg1)  { ICheckpoint::DeleteStreamSyncedMetaData(arg0, std::move(*arg1)); }
inline std::unique_ptr<std::string> IResource_GetType_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IResource* arg0)  { return std::make_unique<std::string>(IResource::GetType(arg0)); }
inline std::unique_ptr<std::string> IResource_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IResource* arg0)  { return std::make_unique<std::string>(IResource::GetName(arg0)); }
inline std::unique_ptr<std::string> IResource_GetPath_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IResource* arg0)  { return std::make_unique<std::string>(IResource::GetPath(arg0)); }
inline std::unique_ptr<std::string> IResource_GetMain_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IResource* arg0)  { return std::make_unique<std::string>(IResource::GetMain(arg0)); }
inline std::unique_ptr<std::vector<std::string>> IResource_GetDependencies_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IResource* arg0)  { return std::make_unique<std::vector<std::string>>(IResource::GetDependencies(arg0)); }
inline std::unique_ptr<std::vector<std::string>> IResource_GetDependants_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IResource* arg0)  { return std::make_unique<std::vector<std::string>>(IResource::GetDependants(arg0)); }
inline std::unique_ptr<std::string> IResource_GetClientType_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IResource* arg0)  { return std::make_unique<std::string>(IResource::GetClientType(arg0)); }
inline std::unique_ptr<std::string> IResource_GetClientMain_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IResource* arg0)  { return std::make_unique<std::string>(IResource::GetClientMain(arg0)); }
inline std::unique_ptr<std::vector<std::string>> IResource_GetClientFiles_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IResource* arg0)  { return std::make_unique<std::vector<std::string>>(IResource::GetClientFiles(arg0)); }
inline std::unique_ptr<Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> IResource_GetConfig_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IResource* arg0)  { return std::make_unique<Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>(IResource::GetConfig(arg0)); }
inline std::unique_ptr<std::vector<PlayerPtrWrapper>> IVoiceChannel_GetPlayers_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVoiceChannel* arg0)  { return std::make_unique<std::vector<PlayerPtrWrapper>>(IVoiceChannel::GetPlayers(arg0)); }
inline std::unique_ptr<std::string> CConsoleCommandEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CConsoleCommandEvent* arg0)  { return std::make_unique<std::string>(CConsoleCommandEvent::GetName(arg0)); }
inline std::unique_ptr<std::vector<std::string>> CConsoleCommandEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CConsoleCommandEvent* arg0)  { return std::make_unique<std::vector<std::string>>(CConsoleCommandEvent::GetArgs(arg0)); }
inline std::unique_ptr<std::string> CServerScriptEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CServerScriptEvent* arg0)  { return std::make_unique<std::string>(CServerScriptEvent::GetName(arg0)); }
inline std::unique_ptr<std::vector<ConstMValueWrapper>> CServerScriptEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CServerScriptEvent* arg0)  { return std::make_unique<std::vector<ConstMValueWrapper>>(CServerScriptEvent::GetArgs(arg0)); }
inline std::unique_ptr<std::string> CClientScriptEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CClientScriptEvent* arg0)  { return std::make_unique<std::string>(CClientScriptEvent::GetName(arg0)); }
inline std::unique_ptr<std::vector<ConstMValueWrapper>> CClientScriptEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CClientScriptEvent* arg0)  { return std::make_unique<std::vector<ConstMValueWrapper>>(CClientScriptEvent::GetArgs(arg0)); }
inline std::unique_ptr<std::string> CPlayerConnectEvent_GetReason_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CPlayerConnectEvent* arg0)  { return std::make_unique<std::string>(CPlayerConnectEvent::GetReason(arg0)); }
inline void CPlayerConnectEvent_Cancel_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerConnectEvent* arg0, std::unique_ptr<std::string> arg1)  { CPlayerConnectEvent::Cancel(arg0, std::move(*arg1)); }
inline std::unique_ptr<std::string> CPlayerDisconnectEvent_GetReason_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CPlayerDisconnectEvent* arg0)  { return std::make_unique<std::string>(CPlayerDisconnectEvent::GetReason(arg0)); }
inline bool IVirtualEntity_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVirtualEntity* arg0, std::unique_ptr<std::string> arg1)  { return IVirtualEntity::HasStreamSyncedMetaData(arg0, std::move(*arg1)); }
inline void IVirtualEntity_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVirtualEntity* arg0, std::unique_ptr<std::string> arg1, ConstMValueWrapper* arg2)  { new(arg2) ConstMValueWrapper(IVirtualEntity::GetStreamSyncedMetaData(arg0, std::move(*arg1))); }
inline std::unique_ptr<std::vector<std::string>> IVirtualEntity_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IVirtualEntity* arg0)  { return std::make_unique<std::vector<std::string>>(IVirtualEntity::GetStreamSyncedMetaDataKeys(arg0)); }
inline void IVirtualEntity_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IVirtualEntity* arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { IVirtualEntity::SetStreamSyncedMetaData(arg0, std::move(*arg1), std::move(*arg2)); }
inline void IVirtualEntity_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IVirtualEntity* arg0, std::unique_ptr<std::string> arg1)  { IVirtualEntity::DeleteStreamSyncedMetaData(arg0, std::move(*arg1)); }
inline void CWeaponDamageEvent_GetShotOffset_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CWeaponDamageEvent* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(CWeaponDamageEvent::GetShotOffset(arg0)); }
inline std::unique_ptr<std::string> CPlayerConnectDeniedEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CPlayerConnectDeniedEvent* arg0)  { return std::make_unique<std::string>(CPlayerConnectDeniedEvent::GetName(arg0)); }
inline std::unique_ptr<std::string> CPlayerConnectDeniedEvent_GetIp_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CPlayerConnectDeniedEvent* arg0)  { return std::make_unique<std::string>(CPlayerConnectDeniedEvent::GetIp(arg0)); }
inline std::unique_ptr<std::string> CPlayerConnectDeniedEvent_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CPlayerConnectDeniedEvent* arg0)  { return std::make_unique<std::string>(CPlayerConnectDeniedEvent::GetBranch(arg0)); }
inline std::unique_ptr<std::string> CPlayerConnectDeniedEvent_GetCdnUrl_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CPlayerConnectDeniedEvent* arg0)  { return std::make_unique<std::string>(CPlayerConnectDeniedEvent::GetCdnUrl(arg0)); }
inline void CStartProjectileEvent_GetStartPosition_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CStartProjectileEvent* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(CStartProjectileEvent::GetStartPosition(arg0)); }
inline void CStartProjectileEvent_GetDirection_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CStartProjectileEvent* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(CStartProjectileEvent::GetDirection(arg0)); }
inline std::unique_ptr<std::vector<FireInfoWrapper>> CFireEvent_GetFires_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CFireEvent* arg0)  { return std::make_unique<std::vector<FireInfoWrapper>>(CFireEvent::GetFires(arg0)); }
inline void CExplosionEvent_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CExplosionEvent* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(CExplosionEvent::GetPosition(arg0)); }
inline std::unique_ptr<std::string> IConnectionInfo_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IConnectionInfo* arg0)  { return std::make_unique<std::string>(IConnectionInfo::GetName(arg0)); }
inline std::unique_ptr<std::string> IConnectionInfo_GetSocialName_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IConnectionInfo* arg0)  { return std::make_unique<std::string>(IConnectionInfo::GetSocialName(arg0)); }
inline std::unique_ptr<std::string> IConnectionInfo_GetHwid3_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IConnectionInfo* arg0)  { return std::make_unique<std::string>(IConnectionInfo::GetHwid3(arg0)); }
inline std::unique_ptr<std::string> IConnectionInfo_GetAuthToken_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IConnectionInfo* arg0)  { return std::make_unique<std::string>(IConnectionInfo::GetAuthToken(arg0)); }
inline std::unique_ptr<std::string> IConnectionInfo_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IConnectionInfo* arg0)  { return std::make_unique<std::string>(IConnectionInfo::GetBranch(arg0)); }
inline std::unique_ptr<std::string> IConnectionInfo_GetCdnUrl_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IConnectionInfo* arg0)  { return std::make_unique<std::string>(IConnectionInfo::GetCdnUrl(arg0)); }
inline std::unique_ptr<std::string> IConnectionInfo_GetIp_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IConnectionInfo* arg0)  { return std::make_unique<std::string>(IConnectionInfo::GetIp(arg0)); }
inline std::unique_ptr<std::string> IConnectionInfo_GetText_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IConnectionInfo* arg0)  { return std::make_unique<std::string>(IConnectionInfo::GetText(arg0)); }
inline std::unique_ptr<std::string> IConnectionInfo_GetCloudID_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::IConnectionInfo* arg0)  { return std::make_unique<std::string>(IConnectionInfo::GetCloudID(arg0)); }
inline void IConnectionInfo_Decline_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IConnectionInfo* arg0, std::unique_ptr<std::string> arg1)  { IConnectionInfo::Decline(arg0, std::move(*arg1)); }
inline void IConnectionInfo_SetText_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IConnectionInfo* arg0, std::unique_ptr<std::string> arg1)  { IConnectionInfo::SetText(arg0, std::move(*arg1)); }
inline std::unique_ptr<std::string> CMetaChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CMetaChangeEvent* arg0)  { return std::make_unique<std::string>(CMetaChangeEvent::GetKey(arg0)); }
inline void CMetaChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CMetaChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CMetaChangeEvent::GetVal(arg0)); }
inline void CMetaChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CMetaChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CMetaChangeEvent::GetOldVal(arg0)); }
inline std::unique_ptr<std::string> CGlobalMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CGlobalMetaDataChangeEvent* arg0)  { return std::make_unique<std::string>(CGlobalMetaDataChangeEvent::GetKey(arg0)); }
inline void CGlobalMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CGlobalMetaDataChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CGlobalMetaDataChangeEvent::GetVal(arg0)); }
inline void CGlobalMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CGlobalMetaDataChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CGlobalMetaDataChangeEvent::GetOldVal(arg0)); }
inline std::unique_ptr<std::string> CGlobalSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CGlobalSyncedMetaDataChangeEvent* arg0)  { return std::make_unique<std::string>(CGlobalSyncedMetaDataChangeEvent::GetKey(arg0)); }
inline void CGlobalSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CGlobalSyncedMetaDataChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CGlobalSyncedMetaDataChangeEvent::GetVal(arg0)); }
inline void CGlobalSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CGlobalSyncedMetaDataChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CGlobalSyncedMetaDataChangeEvent::GetOldVal(arg0)); }
inline std::unique_ptr<std::string> CSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CSyncedMetaDataChangeEvent* arg0)  { return std::make_unique<std::string>(CSyncedMetaDataChangeEvent::GetKey(arg0)); }
inline void CSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CSyncedMetaDataChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CSyncedMetaDataChangeEvent::GetVal(arg0)); }
inline void CSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CSyncedMetaDataChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CSyncedMetaDataChangeEvent::GetOldVal(arg0)); }
inline std::unique_ptr<std::string> CStreamSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CStreamSyncedMetaDataChangeEvent* arg0)  { return std::make_unique<std::string>(CStreamSyncedMetaDataChangeEvent::GetKey(arg0)); }
inline void CStreamSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CStreamSyncedMetaDataChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CStreamSyncedMetaDataChangeEvent::GetVal(arg0)); }
inline void CStreamSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CStreamSyncedMetaDataChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CStreamSyncedMetaDataChangeEvent::GetOldVal(arg0)); }
inline std::unique_ptr<std::string> CLocalMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CLocalMetaDataChangeEvent* arg0)  { return std::make_unique<std::string>(CLocalMetaDataChangeEvent::GetKey(arg0)); }
inline void CLocalMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CLocalMetaDataChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CLocalMetaDataChangeEvent::GetVal(arg0)); }
inline void CLocalMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CLocalMetaDataChangeEvent* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(CLocalMetaDataChangeEvent::GetOldVal(arg0)); }
inline void CStartSyncedSceneEvent_GetStartPosition_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CStartSyncedSceneEvent* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(CStartSyncedSceneEvent::GetStartPosition(arg0)); }
inline void CStartSyncedSceneEvent_GetStartRotation_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CStartSyncedSceneEvent* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(CStartSyncedSceneEvent::GetStartRotation(arg0)); }
inline void CStartSyncedSceneEvent_GetEntityAndAnimHashPairs_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CStartSyncedSceneEvent* arg0, EntityAnimHashPairsWrapper* arg1)  { new(arg1) EntityAnimHashPairsWrapper(CStartSyncedSceneEvent::GetEntityAndAnimHashPairs(arg0)); }
inline void CClientRequestObjectEvent_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::CClientRequestObjectEvent* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(CClientRequestObjectEvent::GetPosition(arg0)); }
inline void register_script_runtime_autocxx_wrapper_0xd5d0abec981e3e3a(alt::ICore* arg0, std::unique_ptr<std::string> arg1, alt::IScriptRuntime* arg2)  { register_script_runtime(arg0, std::move(*arg1), arg2); }
inline void clone_autocxx_wrapper_0xd5d0abec981e3e3a(const ConstMValueWrapper& autocxx_gen_this, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(autocxx_gen_this.clone()); }
inline void copy_const_mvalue_autocxx_wrapper_0xd5d0abec981e3e3a(const ConstMValueWrapper& arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(copy_const_mvalue(arg0)); }
inline void copy_mut_mvalue_autocxx_wrapper_0xd5d0abec981e3e3a(const MValueMutWrapper& arg0, MValueMutWrapper* arg1)  { new(arg1) MValueMutWrapper(copy_mut_mvalue(arg0)); }
inline void convert_mvalue_mut_wrapper_to_const_autocxx_wrapper_0xd5d0abec981e3e3a(MValueMutWrapper* arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(convert_mvalue_mut_wrapper_to_const(std::move(*arg0))); }
inline void copy_mvalue_dict_pair_autocxx_wrapper_0xd5d0abec981e3e3a(const MValueDictPairWrapper& arg0, MValueDictPairWrapper* arg1)  { new(arg1) MValueDictPairWrapper(copy_mvalue_dict_pair(arg0)); }
inline void ConfigDictPairWrapper_clone_autocxx_wrapper_0xd5d0abec981e3e3a(ConfigDictPairWrapper& autocxx_gen_this, ConfigDictPairWrapper* arg1)  { new(arg1) ConfigDictPairWrapper(autocxx_gen_this.clone()); }
inline std::unique_ptr<std::vector<BaseObjectPtrWrapper>> create_base_object_vec_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<std::vector<BaseObjectPtrWrapper>>(create_base_object_vec()); }
inline std::unique_ptr<std::vector<PlayerPtrWrapper>> create_player_vec_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<std::vector<PlayerPtrWrapper>>(create_player_vec()); }
inline void Vector3Wrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(Vector3Wrapper* autocxx_gen_this, float arg1, float arg2, float arg3)  { new (autocxx_gen_this) Vector3Wrapper(arg1, arg2, arg3); }
inline void Vector2Wrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(Vector2Wrapper* autocxx_gen_this, float arg1, float arg2)  { new (autocxx_gen_this) Vector2Wrapper(arg1, arg2); }
inline void create_vector2_vec_autocxx_wrapper_0xd5d0abec981e3e3a(Vector2Vec* arg0)  { new(arg0) Vector2Vec(create_vector2_vec()); }
inline void RGBAWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(RGBAWrapper* autocxx_gen_this, uint8_t arg1, uint8_t arg2, uint8_t arg3, uint8_t arg4)  { new (autocxx_gen_this) RGBAWrapper(arg1, arg2, arg3, arg4); }
inline std::unique_ptr<std::vector<uint32_t>> read_weapon_components_autocxx_wrapper_0xd5d0abec981e3e3a(const WeaponWrapper& arg0)  { return std::make_unique<std::vector<uint32_t>>(read_weapon_components(arg0)); }
inline void FireInfoWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(FireInfoWrapper* autocxx_gen_this, Vector3Wrapper* arg1, uint32_t arg2)  { new (autocxx_gen_this) FireInfoWrapper(std::move(*arg1), arg2); }
inline void read_fire_info_pos_autocxx_wrapper_0xd5d0abec981e3e3a(const FireInfoWrapper& arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(read_fire_info_pos(arg0)); }
inline void create_mvalue_unordered_map_autocxx_wrapper_0xd5d0abec981e3e3a(MValueUnorderedMapWrapper* arg0)  { new(arg0) MValueUnorderedMapWrapper(create_mvalue_unordered_map()); }
inline void push_to_mvalue_unordered_map_autocxx_wrapper_0xd5d0abec981e3e3a(MValueUnorderedMapWrapper& arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { push_to_mvalue_unordered_map(arg0, std::move(*arg1), std::move(*arg2)); }
inline std::unique_ptr<std::vector<EntityAnimHashPairWrapper>> read_entity_anim_hash_pairs_autocxx_wrapper_0xd5d0abec981e3e3a(const EntityAnimHashPairsWrapper& arg0)  { return std::make_unique<std::vector<EntityAnimHashPairWrapper>>(read_entity_anim_hash_pairs(arg0)); }
inline std::unique_ptr<std::vector<ConstMValueWrapper>> create_mvalue_vec_autocxx_wrapper_0xd5d0abec981e3e3a()  { return std::make_unique<std::vector<ConstMValueWrapper>>(create_mvalue_vec()); }
inline uint8_t read_mvalue_type_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { return read_mvalue_type(std::move(*arg0)); }
inline bool read_mvalue_bool_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { return read_mvalue_bool(std::move(*arg0)); }
inline double read_mvalue_double_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { return read_mvalue_double(std::move(*arg0)); }
inline std::unique_ptr<std::string> read_mvalue_string_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { return std::make_unique<std::string>(read_mvalue_string(std::move(*arg0))); }
inline int64_t read_mvalue_int_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { return read_mvalue_int(std::move(*arg0)); }
inline uint64_t read_mvalue_uint_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { return read_mvalue_uint(std::move(*arg0)); }
inline std::unique_ptr<std::vector<ConstMValueWrapper>> read_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { return std::make_unique<std::vector<ConstMValueWrapper>>(read_mvalue_list(std::move(*arg0))); }
inline std::unique_ptr<std::vector<MValueDictPairWrapper>> read_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { return std::make_unique<std::vector<MValueDictPairWrapper>>(read_mvalue_dict(std::move(*arg0))); }
inline std::unique_ptr<std::string> read_mvalue_dict_pair_key_autocxx_wrapper_0xd5d0abec981e3e3a(const MValueDictPairWrapper& arg0)  { return std::make_unique<std::string>(read_mvalue_dict_pair_key(arg0)); }
inline void read_mvalue_dict_pair_value_autocxx_wrapper_0xd5d0abec981e3e3a(const MValueDictPairWrapper& arg0, ConstMValueWrapper* arg1)  { new(arg1) ConstMValueWrapper(read_mvalue_dict_pair_value(arg0)); }
inline alt::IBaseObject* read_mvalue_base_object_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { return read_mvalue_base_object(std::move(*arg0)); }
inline void read_mvalue_vector3_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0, Vector3Wrapper* arg1)  { new(arg1) Vector3Wrapper(read_mvalue_vector3(std::move(*arg0))); }
inline void read_mvalue_vector2_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0, Vector2Wrapper* arg1)  { new(arg1) Vector2Wrapper(read_mvalue_vector2(std::move(*arg0))); }
inline size_t read_mvalue_byte_array_size_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { return read_mvalue_byte_array_size(std::move(*arg0)); }
inline void read_mvalue_byte_array_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0, uint8_t* arg1)  { read_mvalue_byte_array(std::move(*arg0), arg1); }
inline void read_mvalue_rgba_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0, RGBAWrapper* arg1)  { new(arg1) RGBAWrapper(read_mvalue_rgba(std::move(*arg0))); }
inline void create_mvalue_bool_autocxx_wrapper_0xd5d0abec981e3e3a(bool arg0, MValueMutWrapper* arg1)  { new(arg1) MValueMutWrapper(create_mvalue_bool(arg0)); }
inline void create_mvalue_double_autocxx_wrapper_0xd5d0abec981e3e3a(double arg0, MValueMutWrapper* arg1)  { new(arg1) MValueMutWrapper(create_mvalue_double(arg0)); }
inline void create_mvalue_string_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, MValueMutWrapper* arg1)  { new(arg1) MValueMutWrapper(create_mvalue_string(std::move(*arg0))); }
inline void create_mvalue_nil_autocxx_wrapper_0xd5d0abec981e3e3a(MValueMutWrapper* arg0)  { new(arg0) MValueMutWrapper(create_mvalue_nil()); }
inline void create_mvalue_int_autocxx_wrapper_0xd5d0abec981e3e3a(int64_t arg0, MValueMutWrapper* arg1)  { new(arg1) MValueMutWrapper(create_mvalue_int(arg0)); }
inline void create_mvalue_uint_autocxx_wrapper_0xd5d0abec981e3e3a(uint64_t arg0, MValueMutWrapper* arg1)  { new(arg1) MValueMutWrapper(create_mvalue_uint(arg0)); }
inline void create_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a(MValueMutWrapper* arg0)  { new(arg0) MValueMutWrapper(create_mvalue_list()); }
inline void push_to_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a(MValueMutWrapper& arg0, ConstMValueWrapper* arg1)  { push_to_mvalue_list(arg0, std::move(*arg1)); }
inline void create_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a(MValueMutWrapper* arg0)  { new(arg0) MValueMutWrapper(create_mvalue_dict()); }
inline void push_to_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a(MValueMutWrapper& arg0, MValueMutWrapper* arg1, MValueMutWrapper* arg2)  { push_to_mvalue_dict(arg0, std::move(*arg1), std::move(*arg2)); }
inline void create_mvalue_base_object_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IBaseObject* arg0, MValueMutWrapper* arg1)  { new(arg1) MValueMutWrapper(create_mvalue_base_object(arg0)); }
inline void create_mvalue_vector3_autocxx_wrapper_0xd5d0abec981e3e3a(float arg0, float arg1, float arg2, MValueMutWrapper* arg3)  { new(arg3) MValueMutWrapper(create_mvalue_vector3(arg0, arg1, arg2)); }
inline void create_mvalue_vector2_autocxx_wrapper_0xd5d0abec981e3e3a(float arg0, float arg1, MValueMutWrapper* arg2)  { new(arg2) MValueMutWrapper(create_mvalue_vector2(arg0, arg1)); }
inline void create_mvalue_byte_array_autocxx_wrapper_0xd5d0abec981e3e3a(const uint8_t* arg0, size_t arg1, MValueMutWrapper* arg2)  { new(arg2) MValueMutWrapper(create_mvalue_byte_array(arg0, arg1)); }
inline void create_mvalue_rgba_autocxx_wrapper_0xd5d0abec981e3e3a(uint8_t arg0, uint8_t arg1, uint8_t arg2, uint8_t arg3, MValueMutWrapper* arg4)  { new(arg4) MValueMutWrapper(create_mvalue_rgba(arg0, arg1, arg2, arg3)); }
inline void trigger_local_event_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, MValueMutWrapper* arg1)  { trigger_local_event(std::move(*arg0), std::move(*arg1)); }
inline void trigger_client_event_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IPlayer* arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { trigger_client_event(arg0, std::move(*arg1), std::move(*arg2)); }
inline void trigger_client_event_unreliable_autocxx_wrapper_0xd5d0abec981e3e3a(alt::IPlayer* arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { trigger_client_event_unreliable(arg0, std::move(*arg1), std::move(*arg2)); }
inline void trigger_client_event_for_some_autocxx_wrapper_0xd5d0abec981e3e3a(std::vector<PlayerPtrWrapper>* arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { trigger_client_event_for_some(std::move(*arg0), std::move(*arg1), std::move(*arg2)); }
inline void trigger_client_event_unreliable_for_some_autocxx_wrapper_0xd5d0abec981e3e3a(std::vector<PlayerPtrWrapper>* arg0, std::unique_ptr<std::string> arg1, MValueMutWrapper* arg2)  { trigger_client_event_unreliable_for_some(std::move(*arg0), std::move(*arg1), std::move(*arg2)); }
inline void trigger_client_event_for_all_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, MValueMutWrapper* arg1)  { trigger_client_event_for_all(std::move(*arg0), std::move(*arg1)); }
inline void trigger_client_event_unreliable_for_all_autocxx_wrapper_0xd5d0abec981e3e3a(std::unique_ptr<std::string> arg0, MValueMutWrapper* arg1)  { trigger_client_event_unreliable_for_all(std::move(*arg0), std::move(*arg1)); }
inline std::unique_ptr<std::string> read_bone_info_name_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::BoneInfo& arg0)  { return std::make_unique<std::string>(read_bone_info_name(arg0)); }
inline std::unique_ptr<std::string> read_vehicle_model_info_title_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::VehicleModelInfo* arg0)  { return std::make_unique<std::string>(read_vehicle_model_info_title(arg0)); }
inline std::unique_ptr<std::vector<alt::BoneInfo>> read_vehicle_model_info_bones_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::VehicleModelInfo* arg0)  { return std::make_unique<std::vector<alt::BoneInfo>>(read_vehicle_model_info_bones(arg0)); }
inline std::unique_ptr<std::vector<alt::BoneInfo>> read_ped_model_info_bones_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::PedModelInfo* arg0)  { return std::make_unique<std::vector<alt::BoneInfo>>(read_ped_model_info_bones(arg0)); }
inline std::unique_ptr<std::string> read_ped_model_info_name_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::PedModelInfo* arg0)  { return std::make_unique<std::string>(read_ped_model_info_name(arg0)); }
inline std::unique_ptr<std::string> read_ped_model_info_type_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::PedModelInfo* arg0)  { return std::make_unique<std::string>(read_ped_model_info_type(arg0)); }
inline std::unique_ptr<std::string> read_ped_model_info_dlc_name_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::PedModelInfo* arg0)  { return std::make_unique<std::string>(read_ped_model_info_dlc_name(arg0)); }
inline std::unique_ptr<std::string> read_ped_model_info_movement_clip_set_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::PedModelInfo* arg0)  { return std::make_unique<std::string>(read_ped_model_info_movement_clip_set(arg0)); }
inline std::unique_ptr<std::string> read_ped_model_info_default_unarmed_weapon_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::PedModelInfo* arg0)  { return std::make_unique<std::string>(read_ped_model_info_default_unarmed_weapon(arg0)); }
inline std::unique_ptr<std::string> read_weapon_model_info_name_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::WeaponModelInfo* arg0)  { return std::make_unique<std::string>(read_weapon_model_info_name(arg0)); }
inline std::unique_ptr<std::string> read_weapon_model_info_ammo_type_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::WeaponModelInfo* arg0)  { return std::make_unique<std::string>(read_weapon_model_info_ammo_type(arg0)); }
inline std::unique_ptr<std::string> read_weapon_model_info_model_name_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::WeaponModelInfo* arg0)  { return std::make_unique<std::string>(read_weapon_model_info_model_name(arg0)); }
inline std::unique_ptr<std::string> read_weapon_model_info_ammo_model_name_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::WeaponModelInfo* arg0)  { return std::make_unique<std::string>(read_weapon_model_info_ammo_model_name(arg0)); }
inline std::unique_ptr<std::string> read_weapon_model_info_damage_type_autocxx_wrapper_0xd5d0abec981e3e3a(const alt::WeaponModelInfo* arg0)  { return std::make_unique<std::string>(read_weapon_model_info_damage_type(arg0)); }
inline void create_ammo_flags_from_params_autocxx_wrapper_0xd5d0abec981e3e3a(bool arg0, bool arg1, bool arg2, bool arg3, alt::AmmoFlags* arg4)  { new(arg4) alt::AmmoFlags(create_ammo_flags_from_params(arg0, arg1, arg2, arg3)); }
inline void ConstMValueWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* autocxx_gen_this)  { new (autocxx_gen_this) ConstMValueWrapper(); }
inline void ConstMValueWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* autocxx_gen_this, ConstMValueWrapper* arg1)  { new (autocxx_gen_this) ConstMValueWrapper(std::move(*arg1)); }
inline void ConstMValueWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* autocxx_gen_this, const ConstMValueWrapper& arg1)  { new (autocxx_gen_this) ConstMValueWrapper(arg1); }
inline void ConstMValueWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(ConstMValueWrapper* arg0)  { arg0->~ConstMValueWrapper(); }
inline void ConfigDictPairWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(ConfigDictPairWrapper* autocxx_gen_this)  { new (autocxx_gen_this) ConfigDictPairWrapper(); }
inline void ConfigDictPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(ConfigDictPairWrapper* autocxx_gen_this, ConfigDictPairWrapper* arg1)  { new (autocxx_gen_this) ConfigDictPairWrapper(std::move(*arg1)); }
inline void ConfigDictPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(ConfigDictPairWrapper* autocxx_gen_this, const ConfigDictPairWrapper& arg1)  { new (autocxx_gen_this) ConfigDictPairWrapper(arg1); }
inline void ConfigDictPairWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(ConfigDictPairWrapper* arg0)  { arg0->~ConfigDictPairWrapper(); }
inline void Vector3Wrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(Vector3Wrapper* autocxx_gen_this, Vector3Wrapper* arg1)  { new (autocxx_gen_this) Vector3Wrapper(std::move(*arg1)); }
inline void Vector3Wrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(Vector3Wrapper* autocxx_gen_this, const Vector3Wrapper& arg1)  { new (autocxx_gen_this) Vector3Wrapper(arg1); }
inline void Vector3Wrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(Vector3Wrapper* arg0)  { arg0->~Vector3Wrapper(); }
inline void Vector2Wrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(Vector2Wrapper* autocxx_gen_this, Vector2Wrapper* arg1)  { new (autocxx_gen_this) Vector2Wrapper(std::move(*arg1)); }
inline void Vector2Wrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(Vector2Wrapper* autocxx_gen_this, const Vector2Wrapper& arg1)  { new (autocxx_gen_this) Vector2Wrapper(arg1); }
inline void Vector2Wrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(Vector2Wrapper* arg0)  { arg0->~Vector2Wrapper(); }
inline void RGBAWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(RGBAWrapper* autocxx_gen_this, RGBAWrapper* arg1)  { new (autocxx_gen_this) RGBAWrapper(std::move(*arg1)); }
inline void RGBAWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(RGBAWrapper* autocxx_gen_this, const RGBAWrapper& arg1)  { new (autocxx_gen_this) RGBAWrapper(arg1); }
inline void RGBAWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(RGBAWrapper* arg0)  { arg0->~RGBAWrapper(); }
inline void WeaponWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(WeaponWrapper* autocxx_gen_this, WeaponWrapper* arg1)  { new (autocxx_gen_this) WeaponWrapper(std::move(*arg1)); }
inline void WeaponWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(WeaponWrapper* arg0)  { arg0->~WeaponWrapper(); }
inline void FireInfoWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(FireInfoWrapper* autocxx_gen_this, FireInfoWrapper* arg1)  { new (autocxx_gen_this) FireInfoWrapper(std::move(*arg1)); }
inline void FireInfoWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(FireInfoWrapper* autocxx_gen_this, const FireInfoWrapper& arg1)  { new (autocxx_gen_this) FireInfoWrapper(arg1); }
inline void FireInfoWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(FireInfoWrapper* arg0)  { arg0->~FireInfoWrapper(); }
inline alt::CEvent* CEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CEvent>();; }
inline void CEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CEvent* arg0)  { delete_appropriately<alt::CEvent>(arg0);; }
inline void alt_CEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CEvent* autocxx_gen_this, const alt::CEvent& arg1)  { new (autocxx_gen_this) alt::CEvent(arg1); }
inline alt::CCancellableEvent* CCancellableEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CCancellableEvent>();; }
inline void CCancellableEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CCancellableEvent* arg0)  { delete_appropriately<alt::CCancellableEvent>(arg0);; }
inline void alt_CCancellableEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CCancellableEvent* autocxx_gen_this, const alt::CCancellableEvent& arg1)  { new (autocxx_gen_this) alt::CCancellableEvent(arg1); }
inline alt::CConsoleCommandEvent* CConsoleCommandEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CConsoleCommandEvent>();; }
inline void CConsoleCommandEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CConsoleCommandEvent* arg0)  { delete_appropriately<alt::CConsoleCommandEvent>(arg0);; }
inline alt::CServerScriptEvent* CServerScriptEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CServerScriptEvent>();; }
inline void CServerScriptEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CServerScriptEvent* arg0)  { delete_appropriately<alt::CServerScriptEvent>(arg0);; }
inline void alt_CServerScriptEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CServerScriptEvent* autocxx_gen_this, const alt::CServerScriptEvent& arg1)  { new (autocxx_gen_this) alt::CServerScriptEvent(arg1); }
inline alt::CClientScriptEvent* CClientScriptEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CClientScriptEvent>();; }
inline void CClientScriptEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CClientScriptEvent* arg0)  { delete_appropriately<alt::CClientScriptEvent>(arg0);; }
inline void alt_CClientScriptEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CClientScriptEvent* autocxx_gen_this, const alt::CClientScriptEvent& arg1)  { new (autocxx_gen_this) alt::CClientScriptEvent(arg1); }
inline alt::CPlayerDisconnectEvent* CPlayerDisconnectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerDisconnectEvent>();; }
inline void CPlayerDisconnectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerDisconnectEvent* arg0)  { delete_appropriately<alt::CPlayerDisconnectEvent>(arg0);; }
inline void alt_CPlayerDisconnectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerDisconnectEvent* autocxx_gen_this, const alt::CPlayerDisconnectEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerDisconnectEvent(arg1); }
inline alt::CPlayerConnectEvent* CPlayerConnectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerConnectEvent>();; }
inline void CPlayerConnectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerConnectEvent* arg0)  { delete_appropriately<alt::CPlayerConnectEvent>(arg0);; }
inline void alt_CPlayerConnectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerConnectEvent* autocxx_gen_this, const alt::CPlayerConnectEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerConnectEvent(arg1); }
inline alt::CColShapeEvent* CColShapeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CColShapeEvent>();; }
inline void CColShapeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CColShapeEvent* arg0)  { delete_appropriately<alt::CColShapeEvent>(arg0);; }
inline void alt_CColShapeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CColShapeEvent* autocxx_gen_this, const alt::CColShapeEvent& arg1)  { new (autocxx_gen_this) alt::CColShapeEvent(arg1); }
inline alt::CWeaponDamageEvent* CWeaponDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CWeaponDamageEvent>();; }
inline void CWeaponDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CWeaponDamageEvent* arg0)  { delete_appropriately<alt::CWeaponDamageEvent>(arg0);; }
inline void alt_CWeaponDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CWeaponDamageEvent* autocxx_gen_this, const alt::CWeaponDamageEvent& arg1)  { new (autocxx_gen_this) alt::CWeaponDamageEvent(arg1); }
inline alt::CPlayerDeathEvent* CPlayerDeathEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerDeathEvent>();; }
inline void CPlayerDeathEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerDeathEvent* arg0)  { delete_appropriately<alt::CPlayerDeathEvent>(arg0);; }
inline void alt_CPlayerDeathEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerDeathEvent* autocxx_gen_this, const alt::CPlayerDeathEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerDeathEvent(arg1); }
inline alt::CPlayerDamageEvent* CPlayerDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerDamageEvent>();; }
inline void CPlayerDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerDamageEvent* arg0)  { delete_appropriately<alt::CPlayerDamageEvent>(arg0);; }
inline void alt_CPlayerDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerDamageEvent* autocxx_gen_this, const alt::CPlayerDamageEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerDamageEvent(arg1); }
inline alt::CPlayerEnteringVehicleEvent* CPlayerEnteringVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerEnteringVehicleEvent>();; }
inline void CPlayerEnteringVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerEnteringVehicleEvent* arg0)  { delete_appropriately<alt::CPlayerEnteringVehicleEvent>(arg0);; }
inline void alt_CPlayerEnteringVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerEnteringVehicleEvent* autocxx_gen_this, const alt::CPlayerEnteringVehicleEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerEnteringVehicleEvent(arg1); }
inline alt::CPlayerEnterVehicleEvent* CPlayerEnterVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerEnterVehicleEvent>();; }
inline void CPlayerEnterVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerEnterVehicleEvent* arg0)  { delete_appropriately<alt::CPlayerEnterVehicleEvent>(arg0);; }
inline void alt_CPlayerEnterVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerEnterVehicleEvent* autocxx_gen_this, const alt::CPlayerEnterVehicleEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerEnterVehicleEvent(arg1); }
inline alt::CPlayerLeaveVehicleEvent* CPlayerLeaveVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerLeaveVehicleEvent>();; }
inline void CPlayerLeaveVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerLeaveVehicleEvent* arg0)  { delete_appropriately<alt::CPlayerLeaveVehicleEvent>(arg0);; }
inline void alt_CPlayerLeaveVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerLeaveVehicleEvent* autocxx_gen_this, const alt::CPlayerLeaveVehicleEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerLeaveVehicleEvent(arg1); }
inline alt::CPlayerChangeAnimationEvent* CPlayerChangeAnimationEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerChangeAnimationEvent>();; }
inline void CPlayerChangeAnimationEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerChangeAnimationEvent* arg0)  { delete_appropriately<alt::CPlayerChangeAnimationEvent>(arg0);; }
inline void alt_CPlayerChangeAnimationEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerChangeAnimationEvent* autocxx_gen_this, const alt::CPlayerChangeAnimationEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerChangeAnimationEvent(arg1); }
inline alt::CPlayerChangeVehicleSeatEvent* CPlayerChangeVehicleSeatEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerChangeVehicleSeatEvent>();; }
inline void CPlayerChangeVehicleSeatEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerChangeVehicleSeatEvent* arg0)  { delete_appropriately<alt::CPlayerChangeVehicleSeatEvent>(arg0);; }
inline void alt_CPlayerChangeVehicleSeatEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerChangeVehicleSeatEvent* autocxx_gen_this, const alt::CPlayerChangeVehicleSeatEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerChangeVehicleSeatEvent(arg1); }
inline alt::CPlayerWeaponChangeEvent* CPlayerWeaponChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerWeaponChangeEvent>();; }
inline void CPlayerWeaponChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerWeaponChangeEvent* arg0)  { delete_appropriately<alt::CPlayerWeaponChangeEvent>(arg0);; }
inline void alt_CPlayerWeaponChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerWeaponChangeEvent* autocxx_gen_this, const alt::CPlayerWeaponChangeEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerWeaponChangeEvent(arg1); }
inline alt::CPlayerConnectDeniedEvent* CPlayerConnectDeniedEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerConnectDeniedEvent>();; }
inline void CPlayerConnectDeniedEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerConnectDeniedEvent* arg0)  { delete_appropriately<alt::CPlayerConnectDeniedEvent>(arg0);; }
inline void alt_CPlayerConnectDeniedEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerConnectDeniedEvent* autocxx_gen_this, const alt::CPlayerConnectDeniedEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerConnectDeniedEvent(arg1); }
inline alt::CPlayerSpawnEvent* CPlayerSpawnEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerSpawnEvent>();; }
inline void CPlayerSpawnEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerSpawnEvent* arg0)  { delete_appropriately<alt::CPlayerSpawnEvent>(arg0);; }
inline void alt_CPlayerSpawnEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerSpawnEvent* autocxx_gen_this, const alt::CPlayerSpawnEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerSpawnEvent(arg1); }
inline alt::CStartProjectileEvent* CStartProjectileEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CStartProjectileEvent>();; }
inline void CStartProjectileEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CStartProjectileEvent* arg0)  { delete_appropriately<alt::CStartProjectileEvent>(arg0);; }
inline void alt_CStartProjectileEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CStartProjectileEvent* autocxx_gen_this, const alt::CStartProjectileEvent& arg1)  { new (autocxx_gen_this) alt::CStartProjectileEvent(arg1); }
inline alt::CPlayerRequestControlEvent* CPlayerRequestControlEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerRequestControlEvent>();; }
inline void CPlayerRequestControlEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerRequestControlEvent* arg0)  { delete_appropriately<alt::CPlayerRequestControlEvent>(arg0);; }
inline void alt_CPlayerRequestControlEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerRequestControlEvent* autocxx_gen_this, const alt::CPlayerRequestControlEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerRequestControlEvent(arg1); }
inline alt::CPlayerDimensionChangeEvent* CPlayerDimensionChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerDimensionChangeEvent>();; }
inline void CPlayerDimensionChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerDimensionChangeEvent* arg0)  { delete_appropriately<alt::CPlayerDimensionChangeEvent>(arg0);; }
inline void alt_CPlayerDimensionChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerDimensionChangeEvent* autocxx_gen_this, const alt::CPlayerDimensionChangeEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerDimensionChangeEvent(arg1); }
inline alt::CPlayerChangeInteriorEvent* CPlayerChangeInteriorEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerChangeInteriorEvent>();; }
inline void CPlayerChangeInteriorEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerChangeInteriorEvent* arg0)  { delete_appropriately<alt::CPlayerChangeInteriorEvent>(arg0);; }
inline void alt_CPlayerChangeInteriorEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerChangeInteriorEvent* autocxx_gen_this, const alt::CPlayerChangeInteriorEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerChangeInteriorEvent(arg1); }
inline alt::CExplosionEvent* CExplosionEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CExplosionEvent>();; }
inline void CExplosionEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CExplosionEvent* arg0)  { delete_appropriately<alt::CExplosionEvent>(arg0);; }
inline void alt_CExplosionEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CExplosionEvent* autocxx_gen_this, const alt::CExplosionEvent& arg1)  { new (autocxx_gen_this) alt::CExplosionEvent(arg1); }
inline alt::CFireEvent* CFireEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CFireEvent>();; }
inline void CFireEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CFireEvent* arg0)  { delete_appropriately<alt::CFireEvent>(arg0);; }
inline alt::CConnectionQueueAddEvent* CConnectionQueueAddEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CConnectionQueueAddEvent>();; }
inline void CConnectionQueueAddEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CConnectionQueueAddEvent* arg0)  { delete_appropriately<alt::CConnectionQueueAddEvent>(arg0);; }
inline void alt_CConnectionQueueAddEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CConnectionQueueAddEvent* autocxx_gen_this, const alt::CConnectionQueueAddEvent& arg1)  { new (autocxx_gen_this) alt::CConnectionQueueAddEvent(arg1); }
inline alt::CConnectionQueueRemoveEvent* CConnectionQueueRemoveEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CConnectionQueueRemoveEvent>();; }
inline void CConnectionQueueRemoveEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CConnectionQueueRemoveEvent* arg0)  { delete_appropriately<alt::CConnectionQueueRemoveEvent>(arg0);; }
inline void alt_CConnectionQueueRemoveEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CConnectionQueueRemoveEvent* autocxx_gen_this, const alt::CConnectionQueueRemoveEvent& arg1)  { new (autocxx_gen_this) alt::CConnectionQueueRemoveEvent(arg1); }
inline alt::CPlayerHealEvent* CPlayerHealEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPlayerHealEvent>();; }
inline void CPlayerHealEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerHealEvent* arg0)  { delete_appropriately<alt::CPlayerHealEvent>(arg0);; }
inline void alt_CPlayerHealEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPlayerHealEvent* autocxx_gen_this, const alt::CPlayerHealEvent& arg1)  { new (autocxx_gen_this) alt::CPlayerHealEvent(arg1); }
inline alt::CVehicleAttachEvent* CVehicleAttachEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CVehicleAttachEvent>();; }
inline void CVehicleAttachEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleAttachEvent* arg0)  { delete_appropriately<alt::CVehicleAttachEvent>(arg0);; }
inline void alt_CVehicleAttachEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleAttachEvent* autocxx_gen_this, const alt::CVehicleAttachEvent& arg1)  { new (autocxx_gen_this) alt::CVehicleAttachEvent(arg1); }
inline alt::CVehicleDetachEvent* CVehicleDetachEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CVehicleDetachEvent>();; }
inline void CVehicleDetachEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleDetachEvent* arg0)  { delete_appropriately<alt::CVehicleDetachEvent>(arg0);; }
inline void alt_CVehicleDetachEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleDetachEvent* autocxx_gen_this, const alt::CVehicleDetachEvent& arg1)  { new (autocxx_gen_this) alt::CVehicleDetachEvent(arg1); }
inline alt::CVehicleDestroyEvent* CVehicleDestroyEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CVehicleDestroyEvent>();; }
inline void CVehicleDestroyEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleDestroyEvent* arg0)  { delete_appropriately<alt::CVehicleDestroyEvent>(arg0);; }
inline void alt_CVehicleDestroyEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleDestroyEvent* autocxx_gen_this, const alt::CVehicleDestroyEvent& arg1)  { new (autocxx_gen_this) alt::CVehicleDestroyEvent(arg1); }
inline alt::CVehicleDamageEvent* CVehicleDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CVehicleDamageEvent>();; }
inline void CVehicleDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleDamageEvent* arg0)  { delete_appropriately<alt::CVehicleDamageEvent>(arg0);; }
inline void alt_CVehicleDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleDamageEvent* autocxx_gen_this, const alt::CVehicleDamageEvent& arg1)  { new (autocxx_gen_this) alt::CVehicleDamageEvent(arg1); }
inline alt::CVehicleHornEvent* CVehicleHornEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CVehicleHornEvent>();; }
inline void CVehicleHornEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleHornEvent* arg0)  { delete_appropriately<alt::CVehicleHornEvent>(arg0);; }
inline void alt_CVehicleHornEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleHornEvent* autocxx_gen_this, const alt::CVehicleHornEvent& arg1)  { new (autocxx_gen_this) alt::CVehicleHornEvent(arg1); }
inline alt::CVehicleSirenEvent* CVehicleSirenEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CVehicleSirenEvent>();; }
inline void CVehicleSirenEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleSirenEvent* arg0)  { delete_appropriately<alt::CVehicleSirenEvent>(arg0);; }
inline void alt_CVehicleSirenEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVehicleSirenEvent* autocxx_gen_this, const alt::CVehicleSirenEvent& arg1)  { new (autocxx_gen_this) alt::CVehicleSirenEvent(arg1); }
inline alt::CNetOwnerChangeEvent* CNetOwnerChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CNetOwnerChangeEvent>();; }
inline void CNetOwnerChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CNetOwnerChangeEvent* arg0)  { delete_appropriately<alt::CNetOwnerChangeEvent>(arg0);; }
inline void alt_CNetOwnerChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CNetOwnerChangeEvent* autocxx_gen_this, const alt::CNetOwnerChangeEvent& arg1)  { new (autocxx_gen_this) alt::CNetOwnerChangeEvent(arg1); }
inline alt::CMetaChangeEvent* CMetaChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CMetaChangeEvent>();; }
inline void CMetaChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CMetaChangeEvent* arg0)  { delete_appropriately<alt::CMetaChangeEvent>(arg0);; }
inline void alt_CMetaChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CMetaChangeEvent* autocxx_gen_this, const alt::CMetaChangeEvent& arg1)  { new (autocxx_gen_this) alt::CMetaChangeEvent(arg1); }
inline alt::CGlobalMetaDataChangeEvent* CGlobalMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CGlobalMetaDataChangeEvent>();; }
inline void CGlobalMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CGlobalMetaDataChangeEvent* arg0)  { delete_appropriately<alt::CGlobalMetaDataChangeEvent>(arg0);; }
inline void alt_CGlobalMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CGlobalMetaDataChangeEvent* autocxx_gen_this, const alt::CGlobalMetaDataChangeEvent& arg1)  { new (autocxx_gen_this) alt::CGlobalMetaDataChangeEvent(arg1); }
inline alt::CGlobalSyncedMetaDataChangeEvent* CGlobalSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CGlobalSyncedMetaDataChangeEvent>();; }
inline void CGlobalSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CGlobalSyncedMetaDataChangeEvent* arg0)  { delete_appropriately<alt::CGlobalSyncedMetaDataChangeEvent>(arg0);; }
inline void alt_CGlobalSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CGlobalSyncedMetaDataChangeEvent* autocxx_gen_this, const alt::CGlobalSyncedMetaDataChangeEvent& arg1)  { new (autocxx_gen_this) alt::CGlobalSyncedMetaDataChangeEvent(arg1); }
inline alt::CSyncedMetaDataChangeEvent* CSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CSyncedMetaDataChangeEvent>();; }
inline void CSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CSyncedMetaDataChangeEvent* arg0)  { delete_appropriately<alt::CSyncedMetaDataChangeEvent>(arg0);; }
inline void alt_CSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CSyncedMetaDataChangeEvent* autocxx_gen_this, const alt::CSyncedMetaDataChangeEvent& arg1)  { new (autocxx_gen_this) alt::CSyncedMetaDataChangeEvent(arg1); }
inline alt::CStreamSyncedMetaDataChangeEvent* CStreamSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CStreamSyncedMetaDataChangeEvent>();; }
inline void CStreamSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CStreamSyncedMetaDataChangeEvent* arg0)  { delete_appropriately<alt::CStreamSyncedMetaDataChangeEvent>(arg0);; }
inline void alt_CStreamSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CStreamSyncedMetaDataChangeEvent* autocxx_gen_this, const alt::CStreamSyncedMetaDataChangeEvent& arg1)  { new (autocxx_gen_this) alt::CStreamSyncedMetaDataChangeEvent(arg1); }
inline alt::CLocalMetaDataChangeEvent* CLocalMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CLocalMetaDataChangeEvent>();; }
inline void CLocalMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CLocalMetaDataChangeEvent* arg0)  { delete_appropriately<alt::CLocalMetaDataChangeEvent>(arg0);; }
inline void alt_CLocalMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CLocalMetaDataChangeEvent* autocxx_gen_this, const alt::CLocalMetaDataChangeEvent& arg1)  { new (autocxx_gen_this) alt::CLocalMetaDataChangeEvent(arg1); }
inline alt::CResourceStopEvent* CResourceStopEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CResourceStopEvent>();; }
inline void CResourceStopEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CResourceStopEvent* arg0)  { delete_appropriately<alt::CResourceStopEvent>(arg0);; }
inline void alt_CResourceStopEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CResourceStopEvent* autocxx_gen_this, const alt::CResourceStopEvent& arg1)  { new (autocxx_gen_this) alt::CResourceStopEvent(arg1); }
inline alt::CResourceStartEvent* CResourceStartEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CResourceStartEvent>();; }
inline void CResourceStartEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CResourceStartEvent* arg0)  { delete_appropriately<alt::CResourceStartEvent>(arg0);; }
inline void alt_CResourceStartEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CResourceStartEvent* autocxx_gen_this, const alt::CResourceStartEvent& arg1)  { new (autocxx_gen_this) alt::CResourceStartEvent(arg1); }
inline alt::CVoiceConnectionEvent* CVoiceConnectionEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CVoiceConnectionEvent>();; }
inline void CVoiceConnectionEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVoiceConnectionEvent* arg0)  { delete_appropriately<alt::CVoiceConnectionEvent>(arg0);; }
inline void alt_CVoiceConnectionEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CVoiceConnectionEvent* autocxx_gen_this, const alt::CVoiceConnectionEvent& arg1)  { new (autocxx_gen_this) alt::CVoiceConnectionEvent(arg1); }
inline alt::CRequestSyncedSceneEvent* CRequestSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CRequestSyncedSceneEvent>();; }
inline void CRequestSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CRequestSyncedSceneEvent* arg0)  { delete_appropriately<alt::CRequestSyncedSceneEvent>(arg0);; }
inline void alt_CRequestSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CRequestSyncedSceneEvent* autocxx_gen_this, const alt::CRequestSyncedSceneEvent& arg1)  { new (autocxx_gen_this) alt::CRequestSyncedSceneEvent(arg1); }
inline alt::CStartSyncedSceneEvent* CStartSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CStartSyncedSceneEvent>();; }
inline void CStartSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CStartSyncedSceneEvent* arg0)  { delete_appropriately<alt::CStartSyncedSceneEvent>(arg0);; }
inline alt::CStopSyncedSceneEvent* CStopSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CStopSyncedSceneEvent>();; }
inline void CStopSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CStopSyncedSceneEvent* arg0)  { delete_appropriately<alt::CStopSyncedSceneEvent>(arg0);; }
inline void alt_CStopSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CStopSyncedSceneEvent* autocxx_gen_this, const alt::CStopSyncedSceneEvent& arg1)  { new (autocxx_gen_this) alt::CStopSyncedSceneEvent(arg1); }
inline alt::CUpdateSyncedSceneEvent* CUpdateSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CUpdateSyncedSceneEvent>();; }
inline void CUpdateSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CUpdateSyncedSceneEvent* arg0)  { delete_appropriately<alt::CUpdateSyncedSceneEvent>(arg0);; }
inline void alt_CUpdateSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CUpdateSyncedSceneEvent* autocxx_gen_this, const alt::CUpdateSyncedSceneEvent& arg1)  { new (autocxx_gen_this) alt::CUpdateSyncedSceneEvent(arg1); }
inline alt::CClientDeleteObjectEvent* CClientDeleteObjectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CClientDeleteObjectEvent>();; }
inline void CClientDeleteObjectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CClientDeleteObjectEvent* arg0)  { delete_appropriately<alt::CClientDeleteObjectEvent>(arg0);; }
inline void alt_CClientDeleteObjectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CClientDeleteObjectEvent* autocxx_gen_this, const alt::CClientDeleteObjectEvent& arg1)  { new (autocxx_gen_this) alt::CClientDeleteObjectEvent(arg1); }
inline alt::CClientRequestObjectEvent* CClientRequestObjectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CClientRequestObjectEvent>();; }
inline void CClientRequestObjectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CClientRequestObjectEvent* arg0)  { delete_appropriately<alt::CClientRequestObjectEvent>(arg0);; }
inline void alt_CClientRequestObjectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CClientRequestObjectEvent* autocxx_gen_this, const alt::CClientRequestObjectEvent& arg1)  { new (autocxx_gen_this) alt::CClientRequestObjectEvent(arg1); }
inline alt::CGivePedScriptedTaskEvent* CGivePedScriptedTaskEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CGivePedScriptedTaskEvent>();; }
inline void CGivePedScriptedTaskEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CGivePedScriptedTaskEvent* arg0)  { delete_appropriately<alt::CGivePedScriptedTaskEvent>(arg0);; }
inline void alt_CGivePedScriptedTaskEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CGivePedScriptedTaskEvent* autocxx_gen_this, const alt::CGivePedScriptedTaskEvent& arg1)  { new (autocxx_gen_this) alt::CGivePedScriptedTaskEvent(arg1); }
inline alt::CPedDeathEvent* CPedDeathEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPedDeathEvent>();; }
inline void CPedDeathEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPedDeathEvent* arg0)  { delete_appropriately<alt::CPedDeathEvent>(arg0);; }
inline void alt_CPedDeathEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPedDeathEvent* autocxx_gen_this, const alt::CPedDeathEvent& arg1)  { new (autocxx_gen_this) alt::CPedDeathEvent(arg1); }
inline alt::CPedDamageEvent* CPedDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPedDamageEvent>();; }
inline void CPedDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPedDamageEvent* arg0)  { delete_appropriately<alt::CPedDamageEvent>(arg0);; }
inline void alt_CPedDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPedDamageEvent* autocxx_gen_this, const alt::CPedDamageEvent& arg1)  { new (autocxx_gen_this) alt::CPedDamageEvent(arg1); }
inline alt::CPedHealEvent* CPedHealEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CPedHealEvent>();; }
inline void CPedHealEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPedHealEvent* arg0)  { delete_appropriately<alt::CPedHealEvent>(arg0);; }
inline void alt_CPedHealEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CPedHealEvent* autocxx_gen_this, const alt::CPedHealEvent& arg1)  { new (autocxx_gen_this) alt::CPedHealEvent(arg1); }
inline MValueUnorderedMapWrapper* MValueUnorderedMapWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<MValueUnorderedMapWrapper>();; }
inline void MValueUnorderedMapWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(MValueUnorderedMapWrapper* arg0)  { delete_appropriately<MValueUnorderedMapWrapper>(arg0);; }
inline Vector2Vec* Vector2Vec_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<Vector2Vec>();; }
inline void Vector2Vec_free_autocxx_wrapper_0xd5d0abec981e3e3a(Vector2Vec* arg0)  { delete_appropriately<Vector2Vec>(arg0);; }
inline void Vector2Vec_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(Vector2Vec* autocxx_gen_this, Vector2Vec* arg1)  { new (autocxx_gen_this) Vector2Vec(std::move(*arg1)); }
inline MValueMutWrapper* MValueMutWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<MValueMutWrapper>();; }
inline void MValueMutWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(MValueMutWrapper* arg0)  { delete_appropriately<MValueMutWrapper>(arg0);; }
inline void MValueMutWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(MValueMutWrapper* autocxx_gen_this, MValueMutWrapper* arg1)  { new (autocxx_gen_this) MValueMutWrapper(std::move(*arg1)); }
inline void MValueMutWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(MValueMutWrapper* autocxx_gen_this, const MValueMutWrapper& arg1)  { new (autocxx_gen_this) MValueMutWrapper(arg1); }
inline ResourcePtrWrapper* ResourcePtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<ResourcePtrWrapper>();; }
inline void ResourcePtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(ResourcePtrWrapper* arg0)  { delete_appropriately<ResourcePtrWrapper>(arg0);; }
inline void ResourcePtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(ResourcePtrWrapper* autocxx_gen_this, ResourcePtrWrapper* arg1)  { new (autocxx_gen_this) ResourcePtrWrapper(std::move(*arg1)); }
inline void ResourcePtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(ResourcePtrWrapper* autocxx_gen_this, const ResourcePtrWrapper& arg1)  { new (autocxx_gen_this) ResourcePtrWrapper(arg1); }
inline PlayerPtrWrapper* PlayerPtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<PlayerPtrWrapper>();; }
inline void PlayerPtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(PlayerPtrWrapper* arg0)  { delete_appropriately<PlayerPtrWrapper>(arg0);; }
inline void PlayerPtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(PlayerPtrWrapper* autocxx_gen_this, PlayerPtrWrapper* arg1)  { new (autocxx_gen_this) PlayerPtrWrapper(std::move(*arg1)); }
inline void PlayerPtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(PlayerPtrWrapper* autocxx_gen_this, const PlayerPtrWrapper& arg1)  { new (autocxx_gen_this) PlayerPtrWrapper(arg1); }
inline alt::VehicleModelInfo* VehicleModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::VehicleModelInfo>();; }
inline void VehicleModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::VehicleModelInfo* arg0)  { delete_appropriately<alt::VehicleModelInfo>(arg0);; }
inline void alt_VehicleModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::VehicleModelInfo* autocxx_gen_this, alt::VehicleModelInfo* arg1)  { new (autocxx_gen_this) alt::VehicleModelInfo(std::move(*arg1)); }
inline alt::PedModelInfo* PedModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::PedModelInfo>();; }
inline void PedModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::PedModelInfo* arg0)  { delete_appropriately<alt::PedModelInfo>(arg0);; }
inline void alt_PedModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::PedModelInfo* autocxx_gen_this, alt::PedModelInfo* arg1)  { new (autocxx_gen_this) alt::PedModelInfo(std::move(*arg1)); }
inline alt::WeaponModelInfo* WeaponModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::WeaponModelInfo>();; }
inline void WeaponModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::WeaponModelInfo* arg0)  { delete_appropriately<alt::WeaponModelInfo>(arg0);; }
inline void alt_WeaponModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::WeaponModelInfo* autocxx_gen_this, alt::WeaponModelInfo* arg1)  { new (autocxx_gen_this) alt::WeaponModelInfo(std::move(*arg1)); }
inline void alt_WeaponModelInfo_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::WeaponModelInfo* autocxx_gen_this, const alt::WeaponModelInfo& arg1)  { new (autocxx_gen_this) alt::WeaponModelInfo(arg1); }
inline BaseObjectPtrWrapper* BaseObjectPtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<BaseObjectPtrWrapper>();; }
inline void BaseObjectPtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(BaseObjectPtrWrapper* arg0)  { delete_appropriately<BaseObjectPtrWrapper>(arg0);; }
inline void BaseObjectPtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(BaseObjectPtrWrapper* autocxx_gen_this, BaseObjectPtrWrapper* arg1)  { new (autocxx_gen_this) BaseObjectPtrWrapper(std::move(*arg1)); }
inline void BaseObjectPtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(BaseObjectPtrWrapper* autocxx_gen_this, const BaseObjectPtrWrapper& arg1)  { new (autocxx_gen_this) BaseObjectPtrWrapper(arg1); }
inline alt::Cloth* Cloth_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::Cloth>();; }
inline void Cloth_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::Cloth* arg0)  { delete_appropriately<alt::Cloth>(arg0);; }
inline void alt_Cloth_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::Cloth* autocxx_gen_this, alt::Cloth* arg1)  { new (autocxx_gen_this) alt::Cloth(std::move(*arg1)); }
inline void alt_Cloth_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::Cloth* autocxx_gen_this, const alt::Cloth& arg1)  { new (autocxx_gen_this) alt::Cloth(arg1); }
inline alt::DlcCloth* DlcCloth_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::DlcCloth>();; }
inline void DlcCloth_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::DlcCloth* arg0)  { delete_appropriately<alt::DlcCloth>(arg0);; }
inline void alt_DlcCloth_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::DlcCloth* autocxx_gen_this, alt::DlcCloth* arg1)  { new (autocxx_gen_this) alt::DlcCloth(std::move(*arg1)); }
inline void alt_DlcCloth_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::DlcCloth* autocxx_gen_this, const alt::DlcCloth& arg1)  { new (autocxx_gen_this) alt::DlcCloth(arg1); }
inline alt::Prop* Prop_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::Prop>();; }
inline void Prop_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::Prop* arg0)  { delete_appropriately<alt::Prop>(arg0);; }
inline void alt_Prop_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::Prop* autocxx_gen_this, alt::Prop* arg1)  { new (autocxx_gen_this) alt::Prop(std::move(*arg1)); }
inline void alt_Prop_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::Prop* autocxx_gen_this, const alt::Prop& arg1)  { new (autocxx_gen_this) alt::Prop(arg1); }
inline alt::DlcProp* DlcProp_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::DlcProp>();; }
inline void DlcProp_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::DlcProp* arg0)  { delete_appropriately<alt::DlcProp>(arg0);; }
inline void alt_DlcProp_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::DlcProp* autocxx_gen_this, alt::DlcProp* arg1)  { new (autocxx_gen_this) alt::DlcProp(std::move(*arg1)); }
inline void alt_DlcProp_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::DlcProp* autocxx_gen_this, const alt::DlcProp& arg1)  { new (autocxx_gen_this) alt::DlcProp(arg1); }
inline alt::HeadOverlay* HeadOverlay_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::HeadOverlay>();; }
inline void HeadOverlay_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::HeadOverlay* arg0)  { delete_appropriately<alt::HeadOverlay>(arg0);; }
inline void alt_HeadOverlay_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::HeadOverlay* autocxx_gen_this, alt::HeadOverlay* arg1)  { new (autocxx_gen_this) alt::HeadOverlay(std::move(*arg1)); }
inline void alt_HeadOverlay_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::HeadOverlay* autocxx_gen_this, const alt::HeadOverlay& arg1)  { new (autocxx_gen_this) alt::HeadOverlay(arg1); }
inline alt::HeadBlendData* HeadBlendData_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::HeadBlendData>();; }
inline void HeadBlendData_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::HeadBlendData* arg0)  { delete_appropriately<alt::HeadBlendData>(arg0);; }
inline void alt_HeadBlendData_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::HeadBlendData* autocxx_gen_this, alt::HeadBlendData* arg1)  { new (autocxx_gen_this) alt::HeadBlendData(std::move(*arg1)); }
inline void alt_HeadBlendData_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::HeadBlendData* autocxx_gen_this, const alt::HeadBlendData& arg1)  { new (autocxx_gen_this) alt::HeadBlendData(arg1); }
inline StreamedEntityWrapper* StreamedEntityWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<StreamedEntityWrapper>();; }
inline void StreamedEntityWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(StreamedEntityWrapper* arg0)  { delete_appropriately<StreamedEntityWrapper>(arg0);; }
inline void StreamedEntityWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(StreamedEntityWrapper* autocxx_gen_this, StreamedEntityWrapper* arg1)  { new (autocxx_gen_this) StreamedEntityWrapper(std::move(*arg1)); }
inline void StreamedEntityWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(StreamedEntityWrapper* autocxx_gen_this, const StreamedEntityWrapper& arg1)  { new (autocxx_gen_this) StreamedEntityWrapper(arg1); }
inline alt::AmmoFlags* AmmoFlags_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::AmmoFlags>();; }
inline void AmmoFlags_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::AmmoFlags* arg0)  { delete_appropriately<alt::AmmoFlags>(arg0);; }
inline void alt_AmmoFlags_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::AmmoFlags* autocxx_gen_this, alt::AmmoFlags* arg1)  { new (autocxx_gen_this) alt::AmmoFlags(std::move(*arg1)); }
inline void alt_AmmoFlags_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::AmmoFlags* autocxx_gen_this, const alt::AmmoFlags& arg1)  { new (autocxx_gen_this) alt::AmmoFlags(arg1); }
inline alt::CDecoration* CDecoration_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::CDecoration>();; }
inline void CDecoration_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CDecoration* arg0)  { delete_appropriately<alt::CDecoration>(arg0);; }
inline void alt_CDecoration_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CDecoration* autocxx_gen_this, alt::CDecoration* arg1)  { new (autocxx_gen_this) alt::CDecoration(std::move(*arg1)); }
inline void alt_CDecoration_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::CDecoration* autocxx_gen_this, const alt::CDecoration& arg1)  { new (autocxx_gen_this) alt::CDecoration(arg1); }
inline alt::Quaternion* Quaternion_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::Quaternion>();; }
inline void Quaternion_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::Quaternion* arg0)  { delete_appropriately<alt::Quaternion>(arg0);; }
inline void alt_Quaternion_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::Quaternion* autocxx_gen_this, alt::Quaternion* arg1)  { new (autocxx_gen_this) alt::Quaternion(std::move(*arg1)); }
inline void alt_Quaternion_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::Quaternion* autocxx_gen_this, const alt::Quaternion& arg1)  { new (autocxx_gen_this) alt::Quaternion(arg1); }
inline EntityAnimHashPairsWrapper* EntityAnimHashPairsWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<EntityAnimHashPairsWrapper>();; }
inline void EntityAnimHashPairsWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(EntityAnimHashPairsWrapper* arg0)  { delete_appropriately<EntityAnimHashPairsWrapper>(arg0);; }
inline MValueDictPairWrapper* MValueDictPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<MValueDictPairWrapper>();; }
inline void MValueDictPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(MValueDictPairWrapper* arg0)  { delete_appropriately<MValueDictPairWrapper>(arg0);; }
inline void MValueDictPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(MValueDictPairWrapper* autocxx_gen_this, MValueDictPairWrapper* arg1)  { new (autocxx_gen_this) MValueDictPairWrapper(std::move(*arg1)); }
inline void MValueDictPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(MValueDictPairWrapper* autocxx_gen_this, const MValueDictPairWrapper& arg1)  { new (autocxx_gen_this) MValueDictPairWrapper(arg1); }
inline EntityAnimHashPairWrapper* EntityAnimHashPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<EntityAnimHashPairWrapper>();; }
inline void EntityAnimHashPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(EntityAnimHashPairWrapper* arg0)  { delete_appropriately<EntityAnimHashPairWrapper>(arg0);; }
inline void EntityAnimHashPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(EntityAnimHashPairWrapper* autocxx_gen_this, EntityAnimHashPairWrapper* arg1)  { new (autocxx_gen_this) EntityAnimHashPairWrapper(std::move(*arg1)); }
inline void EntityAnimHashPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(EntityAnimHashPairWrapper* autocxx_gen_this, const EntityAnimHashPairWrapper& arg1)  { new (autocxx_gen_this) EntityAnimHashPairWrapper(arg1); }
inline alt::BoneInfo* BoneInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a()  { return new_appropriately<alt::BoneInfo>();; }
inline void BoneInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a(alt::BoneInfo* arg0)  { delete_appropriately<alt::BoneInfo>(arg0);; }
inline void alt_BoneInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::BoneInfo* autocxx_gen_this, alt::BoneInfo* arg1)  { new (autocxx_gen_this) alt::BoneInfo(std::move(*arg1)); }
inline void alt_BoneInfo_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(alt::BoneInfo* autocxx_gen_this, const alt::BoneInfo& arg1)  { new (autocxx_gen_this) alt::BoneInfo(arg1); }
#endif // __AUTOCXXGEN_H__
