#pragma once
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
#include "autocxxgen_alt_bridge.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

namespace {
template <typename T>
class impl;
} // namespace

class String;

#ifndef CXXBRIDGE1_RUST_STR
#define CXXBRIDGE1_RUST_STR
class Str final {
public:
  Str() noexcept;
  Str(const String &) noexcept;
  Str(const std::string &);
  Str(const char *);
  Str(const char *, std::size_t);

  Str &operator=(const Str &) &noexcept = default;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  Str(const Str &) noexcept = default;
  ~Str() noexcept = default;

  using iterator = const char *;
  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const Str &) const noexcept;
  bool operator!=(const Str &) const noexcept;
  bool operator<(const Str &) const noexcept;
  bool operator<=(const Str &) const noexcept;
  bool operator>(const Str &) const noexcept;
  bool operator>=(const Str &) const noexcept;

  void swap(Str &) noexcept;

private:
  class uninit;
  Str(uninit) noexcept;
  friend impl<Str>;

  std::array<std::uintptr_t, 2> repr;
};
#endif // CXXBRIDGE1_RUST_STR
} // namespace cxxbridge1
} // namespace rust

using Config_internal_ValueWrapper_Config_Value_AutocxxConcrete = ::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete;
namespace alt {
  using IBaseObject = ::alt::IBaseObject;
  using IWorldObject = ::alt::IWorldObject;
  using IVirtualEntity = ::alt::IVirtualEntity;
  using IVirtualEntityGroup = ::alt::IVirtualEntityGroup;
  using IEntity = ::alt::IEntity;
  using IVehicle = ::alt::IVehicle;
  using IPlayer = ::alt::IPlayer;
  using IPed = ::alt::IPed;
  using IObject = ::alt::IObject;
  using IColShape = ::alt::IColShape;
  using IBlip = ::alt::IBlip;
  using IVoiceChannel = ::alt::IVoiceChannel;
  using IMarker = ::alt::IMarker;
  using ICheckpoint = ::alt::ICheckpoint;
  using IConnectionInfo = ::alt::IConnectionInfo;
  using IResource = ::alt::IResource;
  using ICore = ::alt::ICore;
  using IScriptRuntime = ::alt::IScriptRuntime;
}
