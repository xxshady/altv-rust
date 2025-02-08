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
#include <new>
#include <string>
#include <type_traits>
#include <utility>
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

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

#ifndef CXXBRIDGE1_RELOCATABLE
#define CXXBRIDGE1_RELOCATABLE
namespace detail {
template <typename... Ts>
struct make_void {
  using type = void;
};

template <typename... Ts>
using void_t = typename make_void<Ts...>::type;

template <typename Void, template <typename...> class, typename...>
struct detect : std::false_type {};
template <template <typename...> class T, typename... A>
struct detect<void_t<T<A...>>, T, A...> : std::true_type {};

template <template <typename...> class T, typename... A>
using is_detected = detect<void, T, A...>;

template <typename T>
using detect_IsRelocatable = typename T::IsRelocatable;

template <typename T>
struct get_IsRelocatable
    : std::is_same<typename T::IsRelocatable, std::true_type> {};
} // namespace detail

template <typename T>
struct IsRelocatable
    : std::conditional<
          detail::is_detected<detail::detect_IsRelocatable, T>::value,
          detail::get_IsRelocatable<T>,
          std::integral_constant<
              bool, std::is_trivially_move_constructible<T>::value &&
                        std::is_trivially_destructible<T>::value>>::type {};
#endif // CXXBRIDGE1_RELOCATABLE

namespace detail {
template <typename T, typename = void *>
struct operator_new {
  void *operator()(::std::size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(::std::size_t sz) { return T::operator new(sz); }
};
} // namespace detail

template <typename T>
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};

namespace {
template <typename T>
void destroy(T *ptr) {
  ptr->~T();
}

template <bool> struct deleter_if {
  template <typename T> void operator()(T *) {}
};

template <> struct deleter_if<true> {
  template <typename T> void operator()(T *ptr) { ptr->~T(); }
};
} // namespace
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

static_assert(
    ::rust::IsRelocatable<::c_int>::value,
    "type c_int should be trivially move constructible and trivially destructible in C++ to be used as an argument of `IPlayer_SetDateTime`, `IPlayer_PlayAnimation_autocxx_wrapper_0xd5d0abec981e3e3a`, `IBlip_SetFlashTimer` or return value of `IBlip_GetFlashTimer`, `IBlip_GetFlashInterval`, `IBlip_GetNumber` in Rust");

extern "C" {
::std::string *cxxbridge1$autocxx_make_string_0xd5d0abec981e3e3a(::rust::Str str_) noexcept {
  ::std::unique_ptr<::std::string> (*autocxx_make_string_0xd5d0abec981e3e3a$)(::rust::Str) = ::autocxx_make_string_0xd5d0abec981e3e3a;
  return autocxx_make_string_0xd5d0abec981e3e3a$(str_).release();
}

::ConstMValueWrapper *cxxbridge1$ConstMValueWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::ConstMValueWrapper *(*ConstMValueWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ConstMValueWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ConstMValueWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$ConstMValueWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *arg0) noexcept {
  void (*ConstMValueWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::ConstMValueWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConstMValueWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::ConfigDictPairWrapper *cxxbridge1$ConfigDictPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::ConfigDictPairWrapper *(*ConfigDictPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ConfigDictPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ConfigDictPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$ConfigDictPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::ConfigDictPairWrapper *arg0) noexcept {
  void (*ConfigDictPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConfigDictPairWrapper *) = ::ConfigDictPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConfigDictPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::Vector3Wrapper *cxxbridge1$Vector3Wrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::Vector3Wrapper *(*Vector3Wrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::Vector3Wrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return Vector3Wrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$Vector3Wrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector3Wrapper *arg0) noexcept {
  void (*Vector3Wrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector3Wrapper *) = ::Vector3Wrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector3Wrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::Vector2Wrapper *cxxbridge1$Vector2Wrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::Vector2Wrapper *(*Vector2Wrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::Vector2Wrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return Vector2Wrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$Vector2Wrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector2Wrapper *arg0) noexcept {
  void (*Vector2Wrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector2Wrapper *) = ::Vector2Wrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector2Wrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::RGBAWrapper *cxxbridge1$RGBAWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::RGBAWrapper *(*RGBAWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::RGBAWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return RGBAWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$RGBAWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::RGBAWrapper *arg0) noexcept {
  void (*RGBAWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::RGBAWrapper *) = ::RGBAWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  RGBAWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::WeaponWrapper *cxxbridge1$WeaponWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::WeaponWrapper *(*WeaponWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::WeaponWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return WeaponWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$WeaponWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::WeaponWrapper *arg0) noexcept {
  void (*WeaponWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::WeaponWrapper *) = ::WeaponWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  WeaponWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::FireInfoWrapper *cxxbridge1$FireInfoWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::FireInfoWrapper *(*FireInfoWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::FireInfoWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return FireInfoWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$FireInfoWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::FireInfoWrapper *arg0) noexcept {
  void (*FireInfoWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::FireInfoWrapper *) = ::FireInfoWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  FireInfoWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}
} // extern "C"

namespace base_object {
extern "C" {
::alt::IWorldObject *base_object$cxxbridge1$to_world_object(::alt::IBaseObject *base_object) noexcept {
  ::alt::IWorldObject *(*to_world_object$)(::alt::IBaseObject *) = ::base_object::to_world_object;
  return to_world_object$(base_object);
}

::alt::IVirtualEntity *base_object$cxxbridge1$to_virtual_entity(::alt::IBaseObject *base_object) noexcept {
  ::alt::IVirtualEntity *(*to_virtual_entity$)(::alt::IBaseObject *) = ::base_object::to_virtual_entity;
  return to_virtual_entity$(base_object);
}

::alt::IVirtualEntityGroup *base_object$cxxbridge1$to_virtual_entity_group(::alt::IBaseObject *base_object) noexcept {
  ::alt::IVirtualEntityGroup *(*to_virtual_entity_group$)(::alt::IBaseObject *) = ::base_object::to_virtual_entity_group;
  return to_virtual_entity_group$(base_object);
}

::alt::IEntity *base_object$cxxbridge1$to_entity(::alt::IBaseObject *base_object) noexcept {
  ::alt::IEntity *(*to_entity$)(::alt::IBaseObject *) = ::base_object::to_entity;
  return to_entity$(base_object);
}

::alt::IVehicle *base_object$cxxbridge1$to_vehicle(::alt::IBaseObject *base_object) noexcept {
  ::alt::IVehicle *(*to_vehicle$)(::alt::IBaseObject *) = ::base_object::to_vehicle;
  return to_vehicle$(base_object);
}

::alt::IPlayer *base_object$cxxbridge1$to_player(::alt::IBaseObject *base_object) noexcept {
  ::alt::IPlayer *(*to_player$)(::alt::IBaseObject *) = ::base_object::to_player;
  return to_player$(base_object);
}

::alt::IPed *base_object$cxxbridge1$to_ped(::alt::IBaseObject *base_object) noexcept {
  ::alt::IPed *(*to_ped$)(::alt::IBaseObject *) = ::base_object::to_ped;
  return to_ped$(base_object);
}

::alt::IObject *base_object$cxxbridge1$to_object(::alt::IBaseObject *base_object) noexcept {
  ::alt::IObject *(*to_object$)(::alt::IBaseObject *) = ::base_object::to_object;
  return to_object$(base_object);
}

::alt::IColShape *base_object$cxxbridge1$to_col_shape(::alt::IBaseObject *base_object) noexcept {
  ::alt::IColShape *(*to_col_shape$)(::alt::IBaseObject *) = ::base_object::to_col_shape;
  return to_col_shape$(base_object);
}

::alt::IBlip *base_object$cxxbridge1$to_blip(::alt::IBaseObject *base_object) noexcept {
  ::alt::IBlip *(*to_blip$)(::alt::IBaseObject *) = ::base_object::to_blip;
  return to_blip$(base_object);
}

::alt::IVoiceChannel *base_object$cxxbridge1$to_voice_channel(::alt::IBaseObject *base_object) noexcept {
  ::alt::IVoiceChannel *(*to_voice_channel$)(::alt::IBaseObject *) = ::base_object::to_voice_channel;
  return to_voice_channel$(base_object);
}

::alt::IMarker *base_object$cxxbridge1$to_marker(::alt::IBaseObject *base_object) noexcept {
  ::alt::IMarker *(*to_marker$)(::alt::IBaseObject *) = ::base_object::to_marker;
  return to_marker$(base_object);
}

::alt::ICheckpoint *base_object$cxxbridge1$to_checkpoint(::alt::IBaseObject *base_object) noexcept {
  ::alt::ICheckpoint *(*to_checkpoint$)(::alt::IBaseObject *) = ::base_object::to_checkpoint;
  return to_checkpoint$(base_object);
}

::alt::IConnectionInfo *base_object$cxxbridge1$to_connection_info(::alt::IBaseObject *base_object) noexcept {
  ::alt::IConnectionInfo *(*to_connection_info$)(::alt::IBaseObject *) = ::base_object::to_connection_info;
  return to_connection_info$(base_object);
}
} // extern "C"
} // namespace base_object

namespace world_object {
extern "C" {
::alt::IBaseObject *world_object$cxxbridge1$to_base_object(::alt::IWorldObject *world_object) noexcept {
  ::alt::IBaseObject *(*to_base_object$)(::alt::IWorldObject *) = ::world_object::to_base_object;
  return to_base_object$(world_object);
}
} // extern "C"
} // namespace world_object

namespace entity {
extern "C" {
::alt::IBaseObject *entity$cxxbridge1$entity_to_base_object(::alt::IEntity *entity) noexcept {
  ::alt::IBaseObject *(*entity_to_base_object$)(::alt::IEntity *) = ::entity::to_base_object;
  return entity_to_base_object$(entity);
}
} // extern "C"
} // namespace entity

namespace player {
extern "C" {
::alt::IEntity *player$cxxbridge1$player_to_entity(::alt::IPlayer *player) noexcept {
  ::alt::IEntity *(*player_to_entity$)(::alt::IPlayer *) = ::player::to_entity;
  return player_to_entity$(player);
}

::alt::IBaseObject *player$cxxbridge1$player_to_base_object(::alt::IPlayer *player) noexcept {
  ::alt::IBaseObject *(*player_to_base_object$)(::alt::IPlayer *) = ::player::to_base_object;
  return player_to_base_object$(player);
}
} // extern "C"
} // namespace player

namespace col_shape {
extern "C" {
::alt::IBaseObject *col_shape$cxxbridge1$col_shape_to_base_object(::alt::IColShape *col_shape) noexcept {
  ::alt::IBaseObject *(*col_shape_to_base_object$)(::alt::IColShape *) = ::col_shape::to_base_object;
  return col_shape_to_base_object$(col_shape);
}
} // extern "C"
} // namespace col_shape

namespace vehicle {
extern "C" {
::alt::IBaseObject *vehicle$cxxbridge1$vehicle_to_base_object(::alt::IVehicle *vehicle) noexcept {
  ::alt::IBaseObject *(*vehicle_to_base_object$)(::alt::IVehicle *) = ::vehicle::to_base_object;
  return vehicle_to_base_object$(vehicle);
}

::alt::IEntity *vehicle$cxxbridge1$vehicle_to_entity(::alt::IVehicle *vehicle) noexcept {
  ::alt::IEntity *(*vehicle_to_entity$)(::alt::IVehicle *) = ::vehicle::to_entity;
  return vehicle_to_entity$(vehicle);
}
} // extern "C"
} // namespace vehicle

namespace ped {
extern "C" {
::alt::IBaseObject *ped$cxxbridge1$ped_to_base_object(::alt::IPed *ped) noexcept {
  ::alt::IBaseObject *(*ped_to_base_object$)(::alt::IPed *) = ::ped::to_base_object;
  return ped_to_base_object$(ped);
}

::alt::IEntity *ped$cxxbridge1$ped_to_entity(::alt::IPed *ped) noexcept {
  ::alt::IEntity *(*ped_to_entity$)(::alt::IPed *) = ::ped::to_entity;
  return ped_to_entity$(ped);
}
} // extern "C"
} // namespace ped

namespace object {
extern "C" {
::alt::IBaseObject *object$cxxbridge1$object_to_base_object(::alt::IObject *object) noexcept {
  ::alt::IBaseObject *(*object_to_base_object$)(::alt::IObject *) = ::object::to_base_object;
  return object_to_base_object$(object);
}

::alt::IEntity *object$cxxbridge1$object_to_entity(::alt::IObject *object) noexcept {
  ::alt::IEntity *(*object_to_entity$)(::alt::IObject *) = ::object::to_entity;
  return object_to_entity$(object);
}
} // extern "C"
} // namespace object

namespace virtual_entity {
extern "C" {
::alt::IBaseObject *virtual_entity$cxxbridge1$virtual_entity_to_base_object(::alt::IVirtualEntity *vehicle) noexcept {
  ::alt::IBaseObject *(*virtual_entity_to_base_object$)(::alt::IVirtualEntity *) = ::virtual_entity::to_base_object;
  return virtual_entity_to_base_object$(vehicle);
}
} // extern "C"
} // namespace virtual_entity

namespace virtual_entity_group {
extern "C" {
::alt::IBaseObject *virtual_entity_group$cxxbridge1$virtual_entity_group_to_base_object(::alt::IVirtualEntityGroup *vehicle) noexcept {
  ::alt::IBaseObject *(*virtual_entity_group_to_base_object$)(::alt::IVirtualEntityGroup *) = ::virtual_entity_group::to_base_object;
  return virtual_entity_group_to_base_object$(vehicle);
}
} // extern "C"
} // namespace virtual_entity_group

namespace blip {
extern "C" {
::alt::IBaseObject *blip$cxxbridge1$blip_to_base_object(::alt::IBlip *blip) noexcept {
  ::alt::IBaseObject *(*blip_to_base_object$)(::alt::IBlip *) = ::blip::to_base_object;
  return blip_to_base_object$(blip);
}
} // extern "C"
} // namespace blip

namespace voice_channel {
extern "C" {
::alt::IBaseObject *voice_channel$cxxbridge1$voice_channel_to_base_object(::alt::IVoiceChannel *voice_channel) noexcept {
  ::alt::IBaseObject *(*voice_channel_to_base_object$)(::alt::IVoiceChannel *) = ::voice_channel::to_base_object;
  return voice_channel_to_base_object$(voice_channel);
}
} // extern "C"
} // namespace voice_channel

namespace marker {
extern "C" {
::alt::IBaseObject *marker$cxxbridge1$marker_to_base_object(::alt::IMarker *marker) noexcept {
  ::alt::IBaseObject *(*marker_to_base_object$)(::alt::IMarker *) = ::marker::to_base_object;
  return marker_to_base_object$(marker);
}
} // extern "C"
} // namespace marker

namespace checkpoint {
extern "C" {
::alt::IBaseObject *checkpoint$cxxbridge1$checkpoint_to_base_object(::alt::ICheckpoint *checkpoint) noexcept {
  ::alt::IBaseObject *(*checkpoint_to_base_object$)(::alt::ICheckpoint *) = ::checkpoint::to_base_object;
  return checkpoint_to_base_object$(checkpoint);
}
} // extern "C"
} // namespace checkpoint

namespace connection_info {
extern "C" {
::alt::IBaseObject *connection_info$cxxbridge1$connection_info_to_base_object(::alt::IConnectionInfo *connection_info) noexcept {
  ::alt::IBaseObject *(*connection_info_to_base_object$)(::alt::IConnectionInfo *) = ::connection_info::to_base_object;
  return connection_info_to_base_object$(connection_info);
}
} // extern "C"
} // namespace connection_info

namespace events {
extern "C" {
::alt::CCancellableEvent const *events$cxxbridge1$to_cancellable(::alt::CEvent const *event) noexcept {
  ::alt::CCancellableEvent const *(*to_cancellable$)(::alt::CEvent const *) = ::events::to_cancellable;
  return to_cancellable$(event);
}

::alt::CConsoleCommandEvent const *events$cxxbridge1$to_CConsoleCommandEvent(::alt::CEvent const *event) noexcept {
  ::alt::CConsoleCommandEvent const *(*to_CConsoleCommandEvent$)(::alt::CEvent const *) = ::events::to_CConsoleCommandEvent;
  return to_CConsoleCommandEvent$(event);
}

::alt::CServerScriptEvent const *events$cxxbridge1$to_CServerScriptEvent(::alt::CEvent const *event) noexcept {
  ::alt::CServerScriptEvent const *(*to_CServerScriptEvent$)(::alt::CEvent const *) = ::events::to_CServerScriptEvent;
  return to_CServerScriptEvent$(event);
}

::alt::CClientScriptEvent const *events$cxxbridge1$to_CClientScriptEvent(::alt::CEvent const *event) noexcept {
  ::alt::CClientScriptEvent const *(*to_CClientScriptEvent$)(::alt::CEvent const *) = ::events::to_CClientScriptEvent;
  return to_CClientScriptEvent$(event);
}

::alt::CPlayerDisconnectEvent const *events$cxxbridge1$to_CPlayerDisconnectEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerDisconnectEvent const *(*to_CPlayerDisconnectEvent$)(::alt::CEvent const *) = ::events::to_CPlayerDisconnectEvent;
  return to_CPlayerDisconnectEvent$(event);
}

::alt::CPlayerConnectEvent const *events$cxxbridge1$to_CPlayerConnectEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerConnectEvent const *(*to_CPlayerConnectEvent$)(::alt::CEvent const *) = ::events::to_CPlayerConnectEvent;
  return to_CPlayerConnectEvent$(event);
}

::alt::CColShapeEvent const *events$cxxbridge1$to_CColShapeEvent(::alt::CEvent const *event) noexcept {
  ::alt::CColShapeEvent const *(*to_CColShapeEvent$)(::alt::CEvent const *) = ::events::to_CColShapeEvent;
  return to_CColShapeEvent$(event);
}

::alt::CWeaponDamageEvent *events$cxxbridge1$to_CWeaponDamageEvent(::alt::CEvent const *event) noexcept {
  ::alt::CWeaponDamageEvent *(*to_CWeaponDamageEvent$)(::alt::CEvent const *) = ::events::to_CWeaponDamageEvent;
  return to_CWeaponDamageEvent$(event);
}

::alt::CPlayerDeathEvent const *events$cxxbridge1$to_CPlayerDeathEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerDeathEvent const *(*to_CPlayerDeathEvent$)(::alt::CEvent const *) = ::events::to_CPlayerDeathEvent;
  return to_CPlayerDeathEvent$(event);
}

::alt::CPlayerDamageEvent const *events$cxxbridge1$to_CPlayerDamageEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerDamageEvent const *(*to_CPlayerDamageEvent$)(::alt::CEvent const *) = ::events::to_CPlayerDamageEvent;
  return to_CPlayerDamageEvent$(event);
}

::alt::CPlayerEnteringVehicleEvent const *events$cxxbridge1$to_CPlayerEnteringVehicleEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerEnteringVehicleEvent const *(*to_CPlayerEnteringVehicleEvent$)(::alt::CEvent const *) = ::events::to_CPlayerEnteringVehicleEvent;
  return to_CPlayerEnteringVehicleEvent$(event);
}

::alt::CPlayerEnterVehicleEvent const *events$cxxbridge1$to_CPlayerEnterVehicleEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerEnterVehicleEvent const *(*to_CPlayerEnterVehicleEvent$)(::alt::CEvent const *) = ::events::to_CPlayerEnterVehicleEvent;
  return to_CPlayerEnterVehicleEvent$(event);
}

::alt::CPlayerLeaveVehicleEvent const *events$cxxbridge1$to_CPlayerLeaveVehicleEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerLeaveVehicleEvent const *(*to_CPlayerLeaveVehicleEvent$)(::alt::CEvent const *) = ::events::to_CPlayerLeaveVehicleEvent;
  return to_CPlayerLeaveVehicleEvent$(event);
}

::alt::CPlayerChangeAnimationEvent const *events$cxxbridge1$to_CPlayerChangeAnimationEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerChangeAnimationEvent const *(*to_CPlayerChangeAnimationEvent$)(::alt::CEvent const *) = ::events::to_CPlayerChangeAnimationEvent;
  return to_CPlayerChangeAnimationEvent$(event);
}

::alt::CPlayerChangeVehicleSeatEvent const *events$cxxbridge1$to_CPlayerChangeVehicleSeatEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerChangeVehicleSeatEvent const *(*to_CPlayerChangeVehicleSeatEvent$)(::alt::CEvent const *) = ::events::to_CPlayerChangeVehicleSeatEvent;
  return to_CPlayerChangeVehicleSeatEvent$(event);
}

::alt::CPlayerWeaponChangeEvent const *events$cxxbridge1$to_CPlayerWeaponChangeEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerWeaponChangeEvent const *(*to_CPlayerWeaponChangeEvent$)(::alt::CEvent const *) = ::events::to_CPlayerWeaponChangeEvent;
  return to_CPlayerWeaponChangeEvent$(event);
}

::alt::CPlayerConnectDeniedEvent const *events$cxxbridge1$to_CPlayerConnectDeniedEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerConnectDeniedEvent const *(*to_CPlayerConnectDeniedEvent$)(::alt::CEvent const *) = ::events::to_CPlayerConnectDeniedEvent;
  return to_CPlayerConnectDeniedEvent$(event);
}

::alt::CPlayerSpawnEvent const *events$cxxbridge1$to_CPlayerSpawnEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerSpawnEvent const *(*to_CPlayerSpawnEvent$)(::alt::CEvent const *) = ::events::to_CPlayerSpawnEvent;
  return to_CPlayerSpawnEvent$(event);
}

::alt::CStartProjectileEvent const *events$cxxbridge1$to_CStartProjectileEvent(::alt::CEvent const *event) noexcept {
  ::alt::CStartProjectileEvent const *(*to_CStartProjectileEvent$)(::alt::CEvent const *) = ::events::to_CStartProjectileEvent;
  return to_CStartProjectileEvent$(event);
}

::alt::CPlayerRequestControlEvent const *events$cxxbridge1$to_CPlayerRequestControlEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerRequestControlEvent const *(*to_CPlayerRequestControlEvent$)(::alt::CEvent const *) = ::events::to_CPlayerRequestControlEvent;
  return to_CPlayerRequestControlEvent$(event);
}

::alt::CPlayerDimensionChangeEvent const *events$cxxbridge1$to_CPlayerDimensionChangeEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerDimensionChangeEvent const *(*to_CPlayerDimensionChangeEvent$)(::alt::CEvent const *) = ::events::to_CPlayerDimensionChangeEvent;
  return to_CPlayerDimensionChangeEvent$(event);
}

::alt::CPlayerChangeInteriorEvent const *events$cxxbridge1$to_CPlayerChangeInteriorEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerChangeInteriorEvent const *(*to_CPlayerChangeInteriorEvent$)(::alt::CEvent const *) = ::events::to_CPlayerChangeInteriorEvent;
  return to_CPlayerChangeInteriorEvent$(event);
}

::alt::CExplosionEvent const *events$cxxbridge1$to_CExplosionEvent(::alt::CEvent const *event) noexcept {
  ::alt::CExplosionEvent const *(*to_CExplosionEvent$)(::alt::CEvent const *) = ::events::to_CExplosionEvent;
  return to_CExplosionEvent$(event);
}

::alt::CFireEvent const *events$cxxbridge1$to_CFireEvent(::alt::CEvent const *event) noexcept {
  ::alt::CFireEvent const *(*to_CFireEvent$)(::alt::CEvent const *) = ::events::to_CFireEvent;
  return to_CFireEvent$(event);
}

::alt::CConnectionQueueAddEvent const *events$cxxbridge1$to_CConnectionQueueAddEvent(::alt::CEvent const *event) noexcept {
  ::alt::CConnectionQueueAddEvent const *(*to_CConnectionQueueAddEvent$)(::alt::CEvent const *) = ::events::to_CConnectionQueueAddEvent;
  return to_CConnectionQueueAddEvent$(event);
}

::alt::CConnectionQueueRemoveEvent const *events$cxxbridge1$to_CConnectionQueueRemoveEvent(::alt::CEvent const *event) noexcept {
  ::alt::CConnectionQueueRemoveEvent const *(*to_CConnectionQueueRemoveEvent$)(::alt::CEvent const *) = ::events::to_CConnectionQueueRemoveEvent;
  return to_CConnectionQueueRemoveEvent$(event);
}

::alt::CPlayerHealEvent const *events$cxxbridge1$to_CPlayerHealEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPlayerHealEvent const *(*to_CPlayerHealEvent$)(::alt::CEvent const *) = ::events::to_CPlayerHealEvent;
  return to_CPlayerHealEvent$(event);
}

::alt::CVehicleAttachEvent const *events$cxxbridge1$to_CVehicleAttachEvent(::alt::CEvent const *event) noexcept {
  ::alt::CVehicleAttachEvent const *(*to_CVehicleAttachEvent$)(::alt::CEvent const *) = ::events::to_CVehicleAttachEvent;
  return to_CVehicleAttachEvent$(event);
}

::alt::CVehicleDetachEvent const *events$cxxbridge1$to_CVehicleDetachEvent(::alt::CEvent const *event) noexcept {
  ::alt::CVehicleDetachEvent const *(*to_CVehicleDetachEvent$)(::alt::CEvent const *) = ::events::to_CVehicleDetachEvent;
  return to_CVehicleDetachEvent$(event);
}

::alt::CVehicleDestroyEvent const *events$cxxbridge1$to_CVehicleDestroyEvent(::alt::CEvent const *event) noexcept {
  ::alt::CVehicleDestroyEvent const *(*to_CVehicleDestroyEvent$)(::alt::CEvent const *) = ::events::to_CVehicleDestroyEvent;
  return to_CVehicleDestroyEvent$(event);
}

::alt::CVehicleDamageEvent const *events$cxxbridge1$to_CVehicleDamageEvent(::alt::CEvent const *event) noexcept {
  ::alt::CVehicleDamageEvent const *(*to_CVehicleDamageEvent$)(::alt::CEvent const *) = ::events::to_CVehicleDamageEvent;
  return to_CVehicleDamageEvent$(event);
}

::alt::CVehicleHornEvent const *events$cxxbridge1$to_CVehicleHornEvent(::alt::CEvent const *event) noexcept {
  ::alt::CVehicleHornEvent const *(*to_CVehicleHornEvent$)(::alt::CEvent const *) = ::events::to_CVehicleHornEvent;
  return to_CVehicleHornEvent$(event);
}

::alt::CVehicleSirenEvent const *events$cxxbridge1$to_CVehicleSirenEvent(::alt::CEvent const *event) noexcept {
  ::alt::CVehicleSirenEvent const *(*to_CVehicleSirenEvent$)(::alt::CEvent const *) = ::events::to_CVehicleSirenEvent;
  return to_CVehicleSirenEvent$(event);
}

::alt::CNetOwnerChangeEvent const *events$cxxbridge1$to_CNetOwnerChangeEvent(::alt::CEvent const *event) noexcept {
  ::alt::CNetOwnerChangeEvent const *(*to_CNetOwnerChangeEvent$)(::alt::CEvent const *) = ::events::to_CNetOwnerChangeEvent;
  return to_CNetOwnerChangeEvent$(event);
}

::alt::CMetaChangeEvent const *events$cxxbridge1$to_CMetaChangeEvent(::alt::CEvent const *event) noexcept {
  ::alt::CMetaChangeEvent const *(*to_CMetaChangeEvent$)(::alt::CEvent const *) = ::events::to_CMetaChangeEvent;
  return to_CMetaChangeEvent$(event);
}

::alt::CGlobalMetaDataChangeEvent const *events$cxxbridge1$to_CGlobalMetaDataChangeEvent(::alt::CEvent const *event) noexcept {
  ::alt::CGlobalMetaDataChangeEvent const *(*to_CGlobalMetaDataChangeEvent$)(::alt::CEvent const *) = ::events::to_CGlobalMetaDataChangeEvent;
  return to_CGlobalMetaDataChangeEvent$(event);
}

::alt::CGlobalSyncedMetaDataChangeEvent const *events$cxxbridge1$to_CGlobalSyncedMetaDataChangeEvent(::alt::CEvent const *event) noexcept {
  ::alt::CGlobalSyncedMetaDataChangeEvent const *(*to_CGlobalSyncedMetaDataChangeEvent$)(::alt::CEvent const *) = ::events::to_CGlobalSyncedMetaDataChangeEvent;
  return to_CGlobalSyncedMetaDataChangeEvent$(event);
}

::alt::CSyncedMetaDataChangeEvent const *events$cxxbridge1$to_CSyncedMetaDataChangeEvent(::alt::CEvent const *event) noexcept {
  ::alt::CSyncedMetaDataChangeEvent const *(*to_CSyncedMetaDataChangeEvent$)(::alt::CEvent const *) = ::events::to_CSyncedMetaDataChangeEvent;
  return to_CSyncedMetaDataChangeEvent$(event);
}

::alt::CStreamSyncedMetaDataChangeEvent const *events$cxxbridge1$to_CStreamSyncedMetaDataChangeEvent(::alt::CEvent const *event) noexcept {
  ::alt::CStreamSyncedMetaDataChangeEvent const *(*to_CStreamSyncedMetaDataChangeEvent$)(::alt::CEvent const *) = ::events::to_CStreamSyncedMetaDataChangeEvent;
  return to_CStreamSyncedMetaDataChangeEvent$(event);
}

::alt::CLocalMetaDataChangeEvent const *events$cxxbridge1$to_CLocalMetaDataChangeEvent(::alt::CEvent const *event) noexcept {
  ::alt::CLocalMetaDataChangeEvent const *(*to_CLocalMetaDataChangeEvent$)(::alt::CEvent const *) = ::events::to_CLocalMetaDataChangeEvent;
  return to_CLocalMetaDataChangeEvent$(event);
}

::alt::CResourceStopEvent const *events$cxxbridge1$to_CResourceStopEvent(::alt::CEvent const *event) noexcept {
  ::alt::CResourceStopEvent const *(*to_CResourceStopEvent$)(::alt::CEvent const *) = ::events::to_CResourceStopEvent;
  return to_CResourceStopEvent$(event);
}

::alt::CResourceStartEvent const *events$cxxbridge1$to_CResourceStartEvent(::alt::CEvent const *event) noexcept {
  ::alt::CResourceStartEvent const *(*to_CResourceStartEvent$)(::alt::CEvent const *) = ::events::to_CResourceStartEvent;
  return to_CResourceStartEvent$(event);
}

::alt::CVoiceConnectionEvent const *events$cxxbridge1$to_CVoiceConnectionEvent(::alt::CEvent const *event) noexcept {
  ::alt::CVoiceConnectionEvent const *(*to_CVoiceConnectionEvent$)(::alt::CEvent const *) = ::events::to_CVoiceConnectionEvent;
  return to_CVoiceConnectionEvent$(event);
}

::alt::CRequestSyncedSceneEvent const *events$cxxbridge1$to_CRequestSyncedSceneEvent(::alt::CEvent const *event) noexcept {
  ::alt::CRequestSyncedSceneEvent const *(*to_CRequestSyncedSceneEvent$)(::alt::CEvent const *) = ::events::to_CRequestSyncedSceneEvent;
  return to_CRequestSyncedSceneEvent$(event);
}

::alt::CStartSyncedSceneEvent const *events$cxxbridge1$to_CStartSyncedSceneEvent(::alt::CEvent const *event) noexcept {
  ::alt::CStartSyncedSceneEvent const *(*to_CStartSyncedSceneEvent$)(::alt::CEvent const *) = ::events::to_CStartSyncedSceneEvent;
  return to_CStartSyncedSceneEvent$(event);
}

::alt::CStopSyncedSceneEvent const *events$cxxbridge1$to_CStopSyncedSceneEvent(::alt::CEvent const *event) noexcept {
  ::alt::CStopSyncedSceneEvent const *(*to_CStopSyncedSceneEvent$)(::alt::CEvent const *) = ::events::to_CStopSyncedSceneEvent;
  return to_CStopSyncedSceneEvent$(event);
}

::alt::CUpdateSyncedSceneEvent const *events$cxxbridge1$to_CUpdateSyncedSceneEvent(::alt::CEvent const *event) noexcept {
  ::alt::CUpdateSyncedSceneEvent const *(*to_CUpdateSyncedSceneEvent$)(::alt::CEvent const *) = ::events::to_CUpdateSyncedSceneEvent;
  return to_CUpdateSyncedSceneEvent$(event);
}

::alt::CClientDeleteObjectEvent const *events$cxxbridge1$to_CClientDeleteObjectEvent(::alt::CEvent const *event) noexcept {
  ::alt::CClientDeleteObjectEvent const *(*to_CClientDeleteObjectEvent$)(::alt::CEvent const *) = ::events::to_CClientDeleteObjectEvent;
  return to_CClientDeleteObjectEvent$(event);
}

::alt::CClientRequestObjectEvent const *events$cxxbridge1$to_CClientRequestObjectEvent(::alt::CEvent const *event) noexcept {
  ::alt::CClientRequestObjectEvent const *(*to_CClientRequestObjectEvent$)(::alt::CEvent const *) = ::events::to_CClientRequestObjectEvent;
  return to_CClientRequestObjectEvent$(event);
}

::alt::CGivePedScriptedTaskEvent const *events$cxxbridge1$to_CGivePedScriptedTaskEvent(::alt::CEvent const *event) noexcept {
  ::alt::CGivePedScriptedTaskEvent const *(*to_CGivePedScriptedTaskEvent$)(::alt::CEvent const *) = ::events::to_CGivePedScriptedTaskEvent;
  return to_CGivePedScriptedTaskEvent$(event);
}

::alt::CPedDeathEvent const *events$cxxbridge1$to_CPedDeathEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPedDeathEvent const *(*to_CPedDeathEvent$)(::alt::CEvent const *) = ::events::to_CPedDeathEvent;
  return to_CPedDeathEvent$(event);
}

::alt::CPedDamageEvent const *events$cxxbridge1$to_CPedDamageEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPedDamageEvent const *(*to_CPedDamageEvent$)(::alt::CEvent const *) = ::events::to_CPedDamageEvent;
  return to_CPedDamageEvent$(event);
}

::alt::CPedHealEvent const *events$cxxbridge1$to_CPedHealEvent(::alt::CEvent const *event) noexcept {
  ::alt::CPedHealEvent const *(*to_CPedHealEvent$)(::alt::CEvent const *) = ::events::to_CPedHealEvent;
  return to_CPedHealEvent$(event);
}
} // extern "C"
} // namespace events

namespace config_node {
extern "C" {
::std::uint8_t config_node$cxxbridge1$get_type(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete const &node) noexcept {
  ::std::uint8_t (*get_type$)(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete const &) = ::config_node::get_type;
  return get_type$(node);
}
} // extern "C"
} // namespace config_node

extern "C" {
bool cxxbridge1$read_bool_autocxx_wrapper_0xd5d0abec981e3e3a(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *node) noexcept {
  bool (*read_bool_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *) = ::read_bool_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_bool_autocxx_wrapper_0xd5d0abec981e3e3a$(node);
}

double cxxbridge1$read_f64_autocxx_wrapper_0xd5d0abec981e3e3a(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *node) noexcept {
  double (*read_f64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *) = ::read_f64_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_f64_autocxx_wrapper_0xd5d0abec981e3e3a$(node);
}

::std::string *cxxbridge1$read_string_autocxx_wrapper_0xd5d0abec981e3e3a(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *node) noexcept {
  ::std::unique_ptr<::std::string> (*read_string_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *) = ::read_string_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_string_autocxx_wrapper_0xd5d0abec981e3e3a$(node).release();
}

::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *cxxbridge1$read_list_autocxx_wrapper_0xd5d0abec981e3e3a(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *node) noexcept {
  ::std::unique_ptr<::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>> (*read_list_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *) = ::read_list_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_list_autocxx_wrapper_0xd5d0abec981e3e3a$(node).release();
}

::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *cxxbridge1$copy_value_ptr_autocxx_wrapper_0xd5d0abec981e3e3a(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete const &node) noexcept {
  ::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> (*copy_value_ptr_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete const &) = ::copy_value_ptr_autocxx_wrapper_0xd5d0abec981e3e3a;
  return copy_value_ptr_autocxx_wrapper_0xd5d0abec981e3e3a$(node).release();
}

::std::vector<::ConfigDictPairWrapper> *cxxbridge1$read_dict_autocxx_wrapper_0xd5d0abec981e3e3a(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *node) noexcept {
  ::std::unique_ptr<::std::vector<::ConfigDictPairWrapper>> (*read_dict_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *) = ::read_dict_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_dict_autocxx_wrapper_0xd5d0abec981e3e3a$(node).release();
}

::std::string *cxxbridge1$read_dict_pair_key_autocxx_wrapper_0xd5d0abec981e3e3a(::ConfigDictPairWrapper const &pair) noexcept {
  ::std::unique_ptr<::std::string> (*read_dict_pair_key_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConfigDictPairWrapper const &) = ::read_dict_pair_key_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_dict_pair_key_autocxx_wrapper_0xd5d0abec981e3e3a$(pair).release();
}

::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *cxxbridge1$read_dict_pair_value_autocxx_wrapper_0xd5d0abec981e3e3a(::ConfigDictPairWrapper const &pair) noexcept {
  ::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> (*read_dict_pair_value_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConfigDictPairWrapper const &) = ::read_dict_pair_value_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_dict_pair_value_autocxx_wrapper_0xd5d0abec981e3e3a$(pair).release();
}

::std::string *cxxbridge1$ICore_GetVersion_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::std::string> (*ICore_GetVersion_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ICore_GetVersion_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetVersion_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}

::std::string *cxxbridge1$ICore_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::std::string> (*ICore_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ICore_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}

void cxxbridge1$ICore_LogInfo_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *prefix, ::std::string *str_, ::alt::IResource *resource) noexcept {
  void (*ICore_LogInfo_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::std::unique_ptr<::std::string>, ::alt::IResource *) = ::ICore_LogInfo_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_LogInfo_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(prefix), ::std::unique_ptr<::std::string>(str_), resource);
}

void cxxbridge1$ICore_LogDebug_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *prefix, ::std::string *str_, ::alt::IResource *resource) noexcept {
  void (*ICore_LogDebug_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::std::unique_ptr<::std::string>, ::alt::IResource *) = ::ICore_LogDebug_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_LogDebug_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(prefix), ::std::unique_ptr<::std::string>(str_), resource);
}

void cxxbridge1$ICore_LogWarning_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *prefix, ::std::string *str_, ::alt::IResource *resource) noexcept {
  void (*ICore_LogWarning_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::std::unique_ptr<::std::string>, ::alt::IResource *) = ::ICore_LogWarning_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_LogWarning_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(prefix), ::std::unique_ptr<::std::string>(str_), resource);
}

void cxxbridge1$ICore_LogError_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *prefix, ::std::string *str_, ::alt::IResource *resource) noexcept {
  void (*ICore_LogError_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::std::unique_ptr<::std::string>, ::alt::IResource *) = ::ICore_LogError_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_LogError_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(prefix), ::std::unique_ptr<::std::string>(str_), resource);
}

void cxxbridge1$ICore_LogColored_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *prefix, ::std::string *str_, ::alt::IResource *resource) noexcept {
  void (*ICore_LogColored_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::std::unique_ptr<::std::string>, ::alt::IResource *) = ::ICore_LogColored_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_LogColored_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(prefix), ::std::unique_ptr<::std::string>(str_), resource);
}

::alt::IVirtualEntity *cxxbridge1$ICore_CreateVirtualEntity_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVirtualEntityGroup *group, float pos_x, float pos_y, float pos_z, ::std::uint32_t streamingDistance, ::MValueUnorderedMapWrapper *data) noexcept {
  ::alt::IVirtualEntity *(*ICore_CreateVirtualEntity_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVirtualEntityGroup *, float, float, float, ::std::uint32_t, ::MValueUnorderedMapWrapper *) = ::ICore_CreateVirtualEntity_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_CreateVirtualEntity_autocxx_wrapper_0xd5d0abec981e3e3a$(group, pos_x, pos_y, pos_z, streamingDistance, data);
}
} // extern "C"

namespace ICore {
extern "C" {
::alt::IVirtualEntityGroup *ICore$cxxbridge1$ICore_CreateVirtualEntityGroup(::std::uint32_t maxEntitiesInStream) noexcept {
  ::alt::IVirtualEntityGroup *(*ICore_CreateVirtualEntityGroup$)(::std::uint32_t) = ::ICore::CreateVirtualEntityGroup;
  return ICore_CreateVirtualEntityGroup$(maxEntitiesInStream);
}

::alt::IColShape *ICore$cxxbridge1$ICore_CreateColShapeCylinder(float pos_x, float pos_y, float pos_z, float radius, float height) noexcept {
  ::alt::IColShape *(*ICore_CreateColShapeCylinder$)(float, float, float, float, float) = ::ICore::CreateColShapeCylinder;
  return ICore_CreateColShapeCylinder$(pos_x, pos_y, pos_z, radius, height);
}

::alt::IColShape *ICore$cxxbridge1$ICore_CreateColShapeSphere(float pos_x, float pos_y, float pos_z, float radius) noexcept {
  ::alt::IColShape *(*ICore_CreateColShapeSphere$)(float, float, float, float) = ::ICore::CreateColShapeSphere;
  return ICore_CreateColShapeSphere$(pos_x, pos_y, pos_z, radius);
}

::alt::IColShape *ICore$cxxbridge1$ICore_CreateColShapeCircle(float pos_x, float pos_y, float pos_z, float radius) noexcept {
  ::alt::IColShape *(*ICore_CreateColShapeCircle$)(float, float, float, float) = ::ICore::CreateColShapeCircle;
  return ICore_CreateColShapeCircle$(pos_x, pos_y, pos_z, radius);
}

::alt::IColShape *ICore$cxxbridge1$ICore_CreateColShapeCube(float pos_x, float pos_y, float pos_z, float pos2_x, float pos2_y, float pos2_z) noexcept {
  ::alt::IColShape *(*ICore_CreateColShapeCube$)(float, float, float, float, float, float) = ::ICore::CreateColShapeCube;
  return ICore_CreateColShapeCube$(pos_x, pos_y, pos_z, pos2_x, pos2_y, pos2_z);
}

::alt::IColShape *ICore$cxxbridge1$ICore_CreateColShapeRectangle(float x1, float y1, float x2, float y2, float z) noexcept {
  ::alt::IColShape *(*ICore_CreateColShapeRectangle$)(float, float, float, float, float) = ::ICore::CreateColShapeRectangle;
  return ICore_CreateColShapeRectangle$(x1, y1, x2, y2, z);
}
} // extern "C"
} // namespace ICore

extern "C" {
::alt::IColShape *cxxbridge1$ICore_CreateColShapePolygon_autocxx_wrapper_0xd5d0abec981e3e3a(float minZ, float maxZ, ::Vector2Vec *points) noexcept {
  ::alt::IColShape *(*ICore_CreateColShapePolygon_autocxx_wrapper_0xd5d0abec981e3e3a$)(float, float, ::Vector2Vec *) = ::ICore_CreateColShapePolygon_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_CreateColShapePolygon_autocxx_wrapper_0xd5d0abec981e3e3a$(minZ, maxZ, points);
}
} // extern "C"

namespace ICore {
extern "C" {
bool ICore$cxxbridge1$ICore_IsDebug() noexcept {
  bool (*ICore_IsDebug$)() = ::ICore::IsDebug;
  return ICore_IsDebug$();
}
} // extern "C"
} // namespace ICore

extern "C" {
::std::uint32_t cxxbridge1$ICore_Hash_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *str_) noexcept {
  ::std::uint32_t (*ICore_Hash_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_Hash_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_Hash_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(str_));
}

bool cxxbridge1$ICore_FileExists_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *path) noexcept {
  bool (*ICore_FileExists_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_FileExists_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_FileExists_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(path));
}

::std::string *cxxbridge1$ICore_FileRead_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *path) noexcept {
  ::std::unique_ptr<::std::string> (*ICore_FileRead_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_FileRead_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_FileRead_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(path)).release();
}

::alt::IResource *cxxbridge1$ICore_GetResource_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *name) noexcept {
  ::alt::IResource *(*ICore_GetResource_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_GetResource_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetResource_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(name));
}
} // extern "C"

namespace ICore {
extern "C" {
::alt::IEntity *ICore$cxxbridge1$ICore_GetEntityBySyncID(::std::uint16_t id) noexcept {
  ::alt::IEntity *(*ICore_GetEntityBySyncID$)(::std::uint16_t) = ::ICore::GetEntityBySyncID;
  return ICore_GetEntityBySyncID$(id);
}
} // extern "C"
} // namespace ICore

extern "C" {
bool cxxbridge1$ICore_HasMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *key) noexcept {
  bool (*ICore_HasMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_HasMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_HasMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(key));
}

void cxxbridge1$ICore_GetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *key, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*ICore_GetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::ConstMValueWrapper *) = ::ICore_GetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_GetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(key), placement_return_type);
}

void cxxbridge1$ICore_SetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *key, ::MValueMutWrapper *val) noexcept {
  void (*ICore_SetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::ICore_SetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_SetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(key), val);
}

void cxxbridge1$ICore_DeleteMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *key) noexcept {
  void (*ICore_DeleteMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_DeleteMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_DeleteMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(key));
}

::std::vector<::std::string> *cxxbridge1$ICore_GetMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*ICore_GetMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ICore_GetMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}

bool cxxbridge1$ICore_HasSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *key) noexcept {
  bool (*ICore_HasSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_HasSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_HasSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(key));
}

void cxxbridge1$ICore_GetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *key, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*ICore_GetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::ConstMValueWrapper *) = ::ICore_GetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_GetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(key), placement_return_type);
}

::std::vector<::std::string> *cxxbridge1$ICore_GetSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*ICore_GetSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ICore_GetSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}
} // extern "C"

namespace ICore {
extern "C" {
void ICore$cxxbridge1$ICore_DestroyBaseObject(::alt::IBaseObject *handle) noexcept {
  void (*ICore_DestroyBaseObject$)(::alt::IBaseObject *) = ::ICore::DestroyBaseObject;
  ICore_DestroyBaseObject$(handle);
}
} // extern "C"
} // namespace ICore

extern "C" {
::std::vector<::ResourcePtrWrapper> *cxxbridge1$ICore_GetAllResources_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::std::vector<::ResourcePtrWrapper>> (*ICore_GetAllResources_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ICore_GetAllResources_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetAllResources_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}

::std::string *cxxbridge1$ICore_StringToSHA256_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *str_) noexcept {
  ::std::unique_ptr<::std::string> (*ICore_StringToSHA256_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_StringToSHA256_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_StringToSHA256_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(str_)).release();
}
} // extern "C"

namespace ICore {
extern "C" {
bool ICore$cxxbridge1$ICore_IsEventEnabled(::std::uint16_t type_) noexcept {
  bool (*ICore_IsEventEnabled$)(::std::uint16_t) = ::ICore::IsEventEnabled;
  return ICore_IsEventEnabled$(type_);
}

void ICore$cxxbridge1$ICore_ToggleEvent(::std::uint16_t type_, bool state) noexcept {
  void (*ICore_ToggleEvent$)(::std::uint16_t, bool) = ::ICore::ToggleEvent;
  ICore_ToggleEvent$(type_, state);
}

::std::uint8_t ICore$cxxbridge1$ICore_GetVoiceConnectionState() noexcept {
  ::std::uint8_t (*ICore_GetVoiceConnectionState$)() = ::ICore::GetVoiceConnectionState;
  return ICore_GetVoiceConnectionState$();
}

::std::uint32_t ICore$cxxbridge1$ICore_GetNetTime() noexcept {
  ::std::uint32_t (*ICore_GetNetTime$)() = ::ICore::GetNetTime;
  return ICore_GetNetTime$();
}
} // extern "C"
} // namespace ICore

extern "C" {
::std::string *cxxbridge1$ICore_GetRootDirectory_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::std::string> (*ICore_GetRootDirectory_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ICore_GetRootDirectory_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetRootDirectory_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}

::alt::IResource *cxxbridge1$ICore_StartResource_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *name) noexcept {
  ::alt::IResource *(*ICore_StartResource_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_StartResource_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_StartResource_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(name));
}

void cxxbridge1$ICore_StopResource_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *name) noexcept {
  void (*ICore_StopResource_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_StopResource_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_StopResource_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(name));
}

void cxxbridge1$ICore_RestartResource_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *name) noexcept {
  void (*ICore_RestartResource_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_RestartResource_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_RestartResource_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(name));
}

void cxxbridge1$ICore_AddClientConfigKey_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *key) noexcept {
  void (*ICore_AddClientConfigKey_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_AddClientConfigKey_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_AddClientConfigKey_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(key));
}

void cxxbridge1$ICore_TriggerClientRPCAnswer_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer *target, ::std::uint16_t answerID, ::MValueMutWrapper *args, ::std::string *error) noexcept {
  void (*ICore_TriggerClientRPCAnswer_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer *, ::std::uint16_t, ::MValueMutWrapper *, ::std::unique_ptr<::std::string>) = ::ICore_TriggerClientRPCAnswer_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_TriggerClientRPCAnswer_autocxx_wrapper_0xd5d0abec981e3e3a$(target, answerID, args, ::std::unique_ptr<::std::string>(error));
}

void cxxbridge1$ICore_SetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *key, ::MValueMutWrapper *val) noexcept {
  void (*ICore_SetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::ICore_SetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_SetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(key), val);
}

void cxxbridge1$ICore_DeleteSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *key) noexcept {
  void (*ICore_DeleteSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_DeleteSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_DeleteSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(key));
}
} // extern "C"

namespace ICore {
extern "C" {
::alt::IVehicle *ICore$cxxbridge1$ICore_CreateVehicle(::std::uint32_t model, float pos_x, float pos_y, float pos_z, float rot_x, float rot_y, float rot_z, ::std::uint32_t streamingDistance) noexcept {
  ::alt::IVehicle *(*ICore_CreateVehicle$)(::std::uint32_t, float, float, float, float, float, float, ::std::uint32_t) = ::ICore::CreateVehicle;
  return ICore_CreateVehicle$(model, pos_x, pos_y, pos_z, rot_x, rot_y, rot_z, streamingDistance);
}

::alt::ICheckpoint *ICore$cxxbridge1$ICore_CreateCheckpoint(::std::uint8_t type_, float pos_x, float pos_y, float pos_z, float radius, float height, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a, ::std::uint32_t streamingDistance) noexcept {
  ::alt::ICheckpoint *(*ICore_CreateCheckpoint$)(::std::uint8_t, float, float, float, float, float, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint32_t) = ::ICore::CreateCheckpoint;
  return ICore_CreateCheckpoint$(type_, pos_x, pos_y, pos_z, radius, height, color_r, color_g, color_b, color_a, streamingDistance);
}
} // extern "C"
} // namespace ICore

extern "C" {
::alt::IBlip *cxxbridge1$ICore_CreateBlip_autocxx_wrapper_0xd5d0abec981e3e3a(bool global, ::std::uint8_t type_, float pos_x, float pos_y, float pos_z, ::std::vector<::PlayerPtrWrapper> *targets) noexcept {
  ::alt::IBlip *(*ICore_CreateBlip_autocxx_wrapper_0xd5d0abec981e3e3a$)(bool, ::std::uint8_t, float, float, float, ::std::vector<::PlayerPtrWrapper> *) = ::ICore_CreateBlip_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_CreateBlip_autocxx_wrapper_0xd5d0abec981e3e3a$(global, type_, pos_x, pos_y, pos_z, targets);
}

::alt::IBlip *cxxbridge1$ICore_CreateBlip1_autocxx_wrapper_0xd5d0abec981e3e3a(bool global, ::std::uint8_t type_, ::alt::IEntity *attachTo, ::std::vector<::PlayerPtrWrapper> *targets) noexcept {
  ::alt::IBlip *(*ICore_CreateBlip1_autocxx_wrapper_0xd5d0abec981e3e3a$)(bool, ::std::uint8_t, ::alt::IEntity *, ::std::vector<::PlayerPtrWrapper> *) = ::ICore_CreateBlip1_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_CreateBlip1_autocxx_wrapper_0xd5d0abec981e3e3a$(global, type_, attachTo, targets);
}
} // extern "C"

namespace ICore {
extern "C" {
::alt::IMarker *ICore$cxxbridge1$ICore_CreateMarker(::alt::IPlayer *target, ::std::uint32_t type_, float position_x, float position_y, float position_z, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a, ::alt::IResource *res) noexcept {
  ::alt::IMarker *(*ICore_CreateMarker$)(::alt::IPlayer *, ::std::uint32_t, float, float, float, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::alt::IResource *) = ::ICore::CreateMarker;
  return ICore_CreateMarker$(target, type_, position_x, position_y, position_z, color_r, color_g, color_b, color_a, res);
}

::alt::IVoiceChannel *ICore$cxxbridge1$ICore_CreateVoiceChannel(bool spatial, float maxDistance) noexcept {
  ::alt::IVoiceChannel *(*ICore_CreateVoiceChannel$)(bool, float) = ::ICore::CreateVoiceChannel;
  return ICore_CreateVoiceChannel$(spatial, maxDistance);
}
} // extern "C"
} // namespace ICore

extern "C" {
::std::vector<::PlayerPtrWrapper> *cxxbridge1$ICore_GetPlayersByName_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *name) noexcept {
  ::std::unique_ptr<::std::vector<::PlayerPtrWrapper>> (*ICore_GetPlayersByName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_GetPlayersByName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetPlayersByName_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(name)).release();
}

void cxxbridge1$ICore_SetPassword_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *password) noexcept {
  void (*ICore_SetPassword_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_SetPassword_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_SetPassword_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(password));
}

::std::uint64_t cxxbridge1$ICore_HashServerPassword_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *password) noexcept {
  ::std::uint64_t (*ICore_HashServerPassword_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>) = ::ICore_HashServerPassword_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_HashServerPassword_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(password));
}
} // extern "C"

namespace ICore {
extern "C" {
void ICore$cxxbridge1$ICore_StopServer() noexcept {
  void (*ICore_StopServer$)() = ::ICore::StopServer;
  ICore_StopServer$();
}

::alt::VehicleModelInfo const *ICore$cxxbridge1$ICore_GetVehicleModelByHash(::std::uint32_t hash) noexcept {
  ::alt::VehicleModelInfo const *(*ICore_GetVehicleModelByHash$)(::std::uint32_t) = ::ICore::GetVehicleModelByHash;
  return ICore_GetVehicleModelByHash$(hash);
}
} // extern "C"
} // namespace ICore

extern "C" {
::std::vector<::std::uint32_t> *cxxbridge1$ICore_GetLoadedVehicleModels_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::std::vector<::std::uint32_t>> (*ICore_GetLoadedVehicleModels_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ICore_GetLoadedVehicleModels_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetLoadedVehicleModels_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}
} // extern "C"

namespace ICore {
extern "C" {
::alt::PedModelInfo const *ICore$cxxbridge1$ICore_GetPedModelByHash(::std::uint32_t hash) noexcept {
  ::alt::PedModelInfo const *(*ICore_GetPedModelByHash$)(::std::uint32_t) = ::ICore::GetPedModelByHash;
  return ICore_GetPedModelByHash$(hash);
}

::alt::WeaponModelInfo const *ICore$cxxbridge1$ICore_GetWeaponModelByHash(::std::uint32_t hash) noexcept {
  ::alt::WeaponModelInfo const *(*ICore_GetWeaponModelByHash$)(::std::uint32_t) = ::ICore::GetWeaponModelByHash;
  return ICore_GetWeaponModelByHash$(hash);
}
} // extern "C"
} // namespace ICore

extern "C" {
::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *cxxbridge1$ICore_GetServerConfig_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> (*ICore_GetServerConfig_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ICore_GetServerConfig_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetServerConfig_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}
} // extern "C"

namespace ICore {
extern "C" {
void ICore$cxxbridge1$ICore_SetWorldProfiler(bool state) noexcept {
  void (*ICore_SetWorldProfiler$)(bool) = ::ICore::SetWorldProfiler;
  ICore_SetWorldProfiler$(state);
}

::alt::IPed *ICore$cxxbridge1$ICore_CreatePed(::std::uint32_t model, float pos_x, float pos_y, float pos_z, float rot_x, float rot_y, float rot_z, ::std::uint32_t streamingDistance) noexcept {
  ::alt::IPed *(*ICore_CreatePed$)(::std::uint32_t, float, float, float, float, float, float, ::std::uint32_t) = ::ICore::CreatePed;
  return ICore_CreatePed$(model, pos_x, pos_y, pos_z, rot_x, rot_y, rot_z, streamingDistance);
}
} // extern "C"
} // namespace ICore

extern "C" {
::std::vector<::BaseObjectPtrWrapper> *cxxbridge1$ICore_GetEntitiesInDimension_autocxx_wrapper_0xd5d0abec981e3e3a(::std::int32_t dimension, ::std::uint64_t allowedTypes) noexcept {
  ::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>> (*ICore_GetEntitiesInDimension_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::int32_t, ::std::uint64_t) = ::ICore_GetEntitiesInDimension_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetEntitiesInDimension_autocxx_wrapper_0xd5d0abec981e3e3a$(dimension, allowedTypes).release();
}

::std::vector<::BaseObjectPtrWrapper> *cxxbridge1$ICore_GetEntitiesInRange_autocxx_wrapper_0xd5d0abec981e3e3a(float position_x, float position_y, float position_z, ::std::int32_t range, ::std::int32_t dimension, ::std::uint64_t allowedTypes) noexcept {
  ::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>> (*ICore_GetEntitiesInRange_autocxx_wrapper_0xd5d0abec981e3e3a$)(float, float, float, ::std::int32_t, ::std::int32_t, ::std::uint64_t) = ::ICore_GetEntitiesInRange_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetEntitiesInRange_autocxx_wrapper_0xd5d0abec981e3e3a$(position_x, position_y, position_z, range, dimension, allowedTypes).release();
}

::std::vector<::BaseObjectPtrWrapper> *cxxbridge1$ICore_GetClosestEntities_autocxx_wrapper_0xd5d0abec981e3e3a(float position_x, float position_y, float position_z, ::std::int32_t range, ::std::int32_t dimension, ::std::int32_t limit, ::std::uint64_t allowedTypes, ::std::uint8_t sortOrder) noexcept {
  ::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>> (*ICore_GetClosestEntities_autocxx_wrapper_0xd5d0abec981e3e3a$)(float, float, float, ::std::int32_t, ::std::int32_t, ::std::int32_t, ::std::uint64_t, ::std::uint8_t) = ::ICore_GetClosestEntities_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICore_GetClosestEntities_autocxx_wrapper_0xd5d0abec981e3e3a$(position_x, position_y, position_z, range, dimension, limit, allowedTypes, sortOrder).release();
}
} // extern "C"

namespace ICore {
extern "C" {
::alt::IObject *ICore$cxxbridge1$ICore_CreateObject(::std::uint32_t model, float pos_x, float pos_y, float pos_z, float rot_x, float rot_y, float rot_z, ::std::uint8_t alpha, ::std::uint8_t textureVariation, ::std::uint16_t lodDistance, ::std::uint32_t streamingDistance) noexcept {
  ::alt::IObject *(*ICore_CreateObject$)(::std::uint32_t, float, float, float, float, float, float, ::std::uint8_t, ::std::uint8_t, ::std::uint16_t, ::std::uint32_t) = ::ICore::CreateObject;
  return ICore_CreateObject$(model, pos_x, pos_y, pos_z, rot_x, rot_y, rot_z, alpha, textureVariation, lodDistance, streamingDistance);
}

::std::uint32_t ICore$cxxbridge1$ICore_GetAmmoHashForWeaponHash(::std::uint32_t weaponHash) noexcept {
  ::std::uint32_t (*ICore_GetAmmoHashForWeaponHash$)(::std::uint32_t) = ::ICore::GetAmmoHashForWeaponHash;
  return ICore_GetAmmoHashForWeaponHash$(weaponHash);
}
} // extern "C"
} // namespace ICore

extern "C" {
void cxxbridge1$ICore_SetVoiceExternalPublic_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *host, ::std::uint16_t port) noexcept {
  void (*ICore_SetVoiceExternalPublic_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::std::uint16_t) = ::ICore_SetVoiceExternalPublic_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_SetVoiceExternalPublic_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(host), port);
}

void cxxbridge1$ICore_SetVoiceExternal_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *host, ::std::uint16_t port) noexcept {
  void (*ICore_SetVoiceExternal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::std::uint16_t) = ::ICore_SetVoiceExternal_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICore_SetVoiceExternal_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(host), port);
}
} // extern "C"

namespace ICore {
extern "C" {
::std::uint16_t ICore$cxxbridge1$ICore_GetMaxStreamingPeds() noexcept {
  ::std::uint16_t (*ICore_GetMaxStreamingPeds$)() = ::ICore::GetMaxStreamingPeds;
  return ICore_GetMaxStreamingPeds$();
}

::std::uint16_t ICore$cxxbridge1$ICore_GetMaxStreamingObjects() noexcept {
  ::std::uint16_t (*ICore_GetMaxStreamingObjects$)() = ::ICore::GetMaxStreamingObjects;
  return ICore_GetMaxStreamingObjects$();
}

::std::uint16_t ICore$cxxbridge1$ICore_GetMaxStreamingVehicles() noexcept {
  ::std::uint16_t (*ICore_GetMaxStreamingVehicles$)() = ::ICore::GetMaxStreamingVehicles;
  return ICore_GetMaxStreamingVehicles$();
}

void ICore$cxxbridge1$ICore_SetMaxStreamingPeds(::std::uint16_t _limit) noexcept {
  void (*ICore_SetMaxStreamingPeds$)(::std::uint16_t) = ::ICore::SetMaxStreamingPeds;
  ICore_SetMaxStreamingPeds$(_limit);
}

void ICore$cxxbridge1$ICore_SetMaxStreamingObjects(::std::uint16_t _limit) noexcept {
  void (*ICore_SetMaxStreamingObjects$)(::std::uint16_t) = ::ICore::SetMaxStreamingObjects;
  ICore_SetMaxStreamingObjects$(_limit);
}

void ICore$cxxbridge1$ICore_SetMaxStreamingVehicles(::std::uint16_t _limit) noexcept {
  void (*ICore_SetMaxStreamingVehicles$)(::std::uint16_t) = ::ICore::SetMaxStreamingVehicles;
  ICore_SetMaxStreamingVehicles$(_limit);
}

::std::uint8_t ICore$cxxbridge1$ICore_GetStreamerThreadCount() noexcept {
  ::std::uint8_t (*ICore_GetStreamerThreadCount$)() = ::ICore::GetStreamerThreadCount;
  return ICore_GetStreamerThreadCount$();
}

::std::uint8_t ICore$cxxbridge1$ICore_GetMigrationThreadCount() noexcept {
  ::std::uint8_t (*ICore_GetMigrationThreadCount$)() = ::ICore::GetMigrationThreadCount;
  return ICore_GetMigrationThreadCount$();
}

::std::uint8_t ICore$cxxbridge1$ICore_GetSyncSendThreadCount() noexcept {
  ::std::uint8_t (*ICore_GetSyncSendThreadCount$)() = ::ICore::GetSyncSendThreadCount;
  return ICore_GetSyncSendThreadCount$();
}

::std::uint8_t ICore$cxxbridge1$ICore_GetSyncReceiveThreadCount() noexcept {
  ::std::uint8_t (*ICore_GetSyncReceiveThreadCount$)() = ::ICore::GetSyncReceiveThreadCount;
  return ICore_GetSyncReceiveThreadCount$();
}

void ICore$cxxbridge1$ICore_SetStreamerThreadCount(::std::uint8_t _count) noexcept {
  void (*ICore_SetStreamerThreadCount$)(::std::uint8_t) = ::ICore::SetStreamerThreadCount;
  ICore_SetStreamerThreadCount$(_count);
}

void ICore$cxxbridge1$ICore_SetMigrationThreadCount(::std::uint8_t _count) noexcept {
  void (*ICore_SetMigrationThreadCount$)(::std::uint8_t) = ::ICore::SetMigrationThreadCount;
  ICore_SetMigrationThreadCount$(_count);
}

void ICore$cxxbridge1$ICore_SetSyncSendThreadCount(::std::uint8_t _count) noexcept {
  void (*ICore_SetSyncSendThreadCount$)(::std::uint8_t) = ::ICore::SetSyncSendThreadCount;
  ICore_SetSyncSendThreadCount$(_count);
}

void ICore$cxxbridge1$ICore_SetSyncReceiveThreadCount(::std::uint8_t _count) noexcept {
  void (*ICore_SetSyncReceiveThreadCount$)(::std::uint8_t) = ::ICore::SetSyncReceiveThreadCount;
  ICore_SetSyncReceiveThreadCount$(_count);
}

::std::uint32_t ICore$cxxbridge1$ICore_GetStreamingTickRate() noexcept {
  ::std::uint32_t (*ICore_GetStreamingTickRate$)() = ::ICore::GetStreamingTickRate;
  return ICore_GetStreamingTickRate$();
}

::std::uint32_t ICore$cxxbridge1$ICore_GetMigrationTickRate() noexcept {
  ::std::uint32_t (*ICore_GetMigrationTickRate$)() = ::ICore::GetMigrationTickRate;
  return ICore_GetMigrationTickRate$();
}

::std::uint32_t ICore$cxxbridge1$ICore_GetColShapeTickRate() noexcept {
  ::std::uint32_t (*ICore_GetColShapeTickRate$)() = ::ICore::GetColShapeTickRate;
  return ICore_GetColShapeTickRate$();
}

void ICore$cxxbridge1$ICore_SetStreamingTickRate(::std::uint32_t _tickRate) noexcept {
  void (*ICore_SetStreamingTickRate$)(::std::uint32_t) = ::ICore::SetStreamingTickRate;
  ICore_SetStreamingTickRate$(_tickRate);
}

void ICore$cxxbridge1$ICore_SetMigrationTickRate(::std::uint32_t _tickRate) noexcept {
  void (*ICore_SetMigrationTickRate$)(::std::uint32_t) = ::ICore::SetMigrationTickRate;
  ICore_SetMigrationTickRate$(_tickRate);
}

void ICore$cxxbridge1$ICore_SetColShapeTickRate(::std::uint32_t _tickRate) noexcept {
  void (*ICore_SetColShapeTickRate$)(::std::uint32_t) = ::ICore::SetColShapeTickRate;
  ICore_SetColShapeTickRate$(_tickRate);
}

::std::uint32_t ICore$cxxbridge1$ICore_GetStreamingDistance() noexcept {
  ::std::uint32_t (*ICore_GetStreamingDistance$)() = ::ICore::GetStreamingDistance;
  return ICore_GetStreamingDistance$();
}

::std::uint32_t ICore$cxxbridge1$ICore_GetMigrationDistance() noexcept {
  ::std::uint32_t (*ICore_GetMigrationDistance$)() = ::ICore::GetMigrationDistance;
  return ICore_GetMigrationDistance$();
}

void ICore$cxxbridge1$ICore_SetStreamingDistance(::std::uint32_t _distance) noexcept {
  void (*ICore_SetStreamingDistance$)(::std::uint32_t) = ::ICore::SetStreamingDistance;
  ICore_SetStreamingDistance$(_distance);
}

void ICore$cxxbridge1$ICore_SetMigrationDistance(::std::uint32_t _distance) noexcept {
  void (*ICore_SetMigrationDistance$)(::std::uint32_t) = ::ICore::SetMigrationDistance;
  ICore_SetMigrationDistance$(_distance);
}

bool ICore$cxxbridge1$ICore_HasBenefit(::std::uint8_t benefit) noexcept {
  bool (*ICore_HasBenefit$)(::std::uint8_t) = ::ICore::HasBenefit;
  return ICore_HasBenefit$(benefit);
}
} // extern "C"
} // namespace ICore

namespace IBaseObject {
extern "C" {
::std::uint8_t IBaseObject$cxxbridge1$IBaseObject_GetType(::alt::IBaseObject const *ptr) noexcept {
  ::std::uint8_t (*IBaseObject_GetType$)(::alt::IBaseObject const *) = ::IBaseObject::GetType;
  return IBaseObject_GetType$(ptr);
}

::std::uint32_t IBaseObject$cxxbridge1$IBaseObject_GetID(::alt::IBaseObject const *ptr) noexcept {
  ::std::uint32_t (*IBaseObject_GetID$)(::alt::IBaseObject const *) = ::IBaseObject::GetID;
  return IBaseObject_GetID$(ptr);
}
} // extern "C"
} // namespace IBaseObject

extern "C" {
bool cxxbridge1$IBaseObject_HasMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject const *ptr, ::std::string *key) noexcept {
  bool (*IBaseObject_HasMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject const *, ::std::unique_ptr<::std::string>) = ::IBaseObject_HasMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IBaseObject_HasMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}

void cxxbridge1$IBaseObject_GetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject const *ptr, ::std::string *key, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*IBaseObject_GetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject const *, ::std::unique_ptr<::std::string>, ::ConstMValueWrapper *) = ::IBaseObject_GetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBaseObject_GetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), placement_return_type);
}

void cxxbridge1$IBaseObject_SetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject *ptr, ::std::string *key, ::MValueMutWrapper *val) noexcept {
  void (*IBaseObject_SetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject *, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::IBaseObject_SetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBaseObject_SetMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), val);
}
} // extern "C"

namespace IBaseObject {
extern "C" {
void IBaseObject$cxxbridge1$IBaseObject_SetMultipleMetaData(::alt::IBaseObject *ptr, ::MValueUnorderedMapWrapper const &values) noexcept {
  void (*IBaseObject_SetMultipleMetaData$)(::alt::IBaseObject *, ::MValueUnorderedMapWrapper const &) = ::IBaseObject::SetMultipleMetaData;
  IBaseObject_SetMultipleMetaData$(ptr, values);
}
} // extern "C"
} // namespace IBaseObject

extern "C" {
void cxxbridge1$IBaseObject_DeleteMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject *ptr, ::std::string *key) noexcept {
  void (*IBaseObject_DeleteMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject *, ::std::unique_ptr<::std::string>) = ::IBaseObject_DeleteMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBaseObject_DeleteMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}

::std::vector<::std::string> *cxxbridge1$IBaseObject_GetMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*IBaseObject_GetMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject const *) = ::IBaseObject_GetMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IBaseObject_GetMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

bool cxxbridge1$IBaseObject_HasSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject const *ptr, ::std::string *key) noexcept {
  bool (*IBaseObject_HasSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject const *, ::std::unique_ptr<::std::string>) = ::IBaseObject_HasSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IBaseObject_HasSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}

void cxxbridge1$IBaseObject_GetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject const *ptr, ::std::string *key, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*IBaseObject_GetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject const *, ::std::unique_ptr<::std::string>, ::ConstMValueWrapper *) = ::IBaseObject_GetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBaseObject_GetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), placement_return_type);
}

::std::vector<::std::string> *cxxbridge1$IBaseObject_GetSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*IBaseObject_GetSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject const *) = ::IBaseObject_GetSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IBaseObject_GetSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$IBaseObject_SetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject *ptr, ::std::string *key, ::MValueMutWrapper *val) noexcept {
  void (*IBaseObject_SetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject *, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::IBaseObject_SetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBaseObject_SetSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), val);
}
} // extern "C"

namespace IBaseObject {
extern "C" {
void IBaseObject$cxxbridge1$IBaseObject_SetMultipleSyncedMetaData(::alt::IBaseObject *ptr, ::MValueUnorderedMapWrapper const &values) noexcept {
  void (*IBaseObject_SetMultipleSyncedMetaData$)(::alt::IBaseObject *, ::MValueUnorderedMapWrapper const &) = ::IBaseObject::SetMultipleSyncedMetaData;
  IBaseObject_SetMultipleSyncedMetaData$(ptr, values);
}
} // extern "C"
} // namespace IBaseObject

extern "C" {
void cxxbridge1$IBaseObject_DeleteSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject *ptr, ::std::string *key) noexcept {
  void (*IBaseObject_DeleteSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject *, ::std::unique_ptr<::std::string>) = ::IBaseObject_DeleteSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBaseObject_DeleteSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}
} // extern "C"

namespace IBaseObject {
extern "C" {
bool IBaseObject$cxxbridge1$IBaseObject_IsRemoved(::alt::IBaseObject const *ptr) noexcept {
  bool (*IBaseObject_IsRemoved$)(::alt::IBaseObject const *) = ::IBaseObject::IsRemoved;
  return IBaseObject_IsRemoved$(ptr);
}
} // extern "C"
} // namespace IBaseObject

extern "C" {
void cxxbridge1$IWorldObject_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IWorldObject const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*IWorldObject_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IWorldObject const *, ::Vector3Wrapper *) = ::IWorldObject_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a;
  IWorldObject_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IWorldObject {
extern "C" {
void IWorldObject$cxxbridge1$IWorldObject_SetPosition(::alt::IWorldObject *ptr, float pos_x, float pos_y, float pos_z) noexcept {
  void (*IWorldObject_SetPosition$)(::alt::IWorldObject *, float, float, float) = ::IWorldObject::SetPosition;
  IWorldObject_SetPosition$(ptr, pos_x, pos_y, pos_z);
}

::std::int32_t IWorldObject$cxxbridge1$IWorldObject_GetDimension(::alt::IWorldObject const *ptr) noexcept {
  ::std::int32_t (*IWorldObject_GetDimension$)(::alt::IWorldObject const *) = ::IWorldObject::GetDimension;
  return IWorldObject_GetDimension$(ptr);
}

void IWorldObject$cxxbridge1$IWorldObject_SetDimension(::alt::IWorldObject *ptr, ::std::int32_t dimension) noexcept {
  void (*IWorldObject_SetDimension$)(::alt::IWorldObject *, ::std::int32_t) = ::IWorldObject::SetDimension;
  IWorldObject_SetDimension$(ptr, dimension);
}
} // extern "C"
} // namespace IWorldObject

namespace IEntity {
extern "C" {
::std::uint16_t IEntity$cxxbridge1$IEntity_GetSyncID(::alt::IEntity const *ptr) noexcept {
  ::std::uint16_t (*IEntity_GetSyncID$)(::alt::IEntity const *) = ::IEntity::GetSyncID;
  return IEntity_GetSyncID$(ptr);
}

::alt::IPlayer *IEntity$cxxbridge1$IEntity_GetNetworkOwner(::alt::IEntity const *ptr) noexcept {
  ::alt::IPlayer *(*IEntity_GetNetworkOwner$)(::alt::IEntity const *) = ::IEntity::GetNetworkOwner;
  return IEntity_GetNetworkOwner$(ptr);
}

::std::uint32_t IEntity$cxxbridge1$IEntity_GetModel(::alt::IEntity const *ptr) noexcept {
  ::std::uint32_t (*IEntity_GetModel$)(::alt::IEntity const *) = ::IEntity::GetModel;
  return IEntity_GetModel$(ptr);
}
} // extern "C"
} // namespace IEntity

extern "C" {
void cxxbridge1$IEntity_GetRotation_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IEntity const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*IEntity_GetRotation_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IEntity const *, ::Vector3Wrapper *) = ::IEntity_GetRotation_autocxx_wrapper_0xd5d0abec981e3e3a;
  IEntity_GetRotation_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IEntity {
extern "C" {
void IEntity$cxxbridge1$IEntity_SetRotation(::alt::IEntity *ptr, float rot_x, float rot_y, float rot_z) noexcept {
  void (*IEntity_SetRotation$)(::alt::IEntity *, float, float, float) = ::IEntity::SetRotation;
  IEntity_SetRotation$(ptr, rot_x, rot_y, rot_z);
}
} // extern "C"
} // namespace IEntity

extern "C" {
bool cxxbridge1$IEntity_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IEntity const *ptr, ::std::string *key) noexcept {
  bool (*IEntity_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IEntity const *, ::std::unique_ptr<::std::string>) = ::IEntity_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IEntity_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}

void cxxbridge1$IEntity_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IEntity const *ptr, ::std::string *key, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*IEntity_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IEntity const *, ::std::unique_ptr<::std::string>, ::ConstMValueWrapper *) = ::IEntity_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IEntity_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), placement_return_type);
}

::std::vector<::std::string> *cxxbridge1$IEntity_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IEntity const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*IEntity_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IEntity const *) = ::IEntity_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IEntity_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IEntity {
extern "C" {
bool IEntity$cxxbridge1$IEntity_GetVisible(::alt::IEntity const *ptr) noexcept {
  bool (*IEntity_GetVisible$)(::alt::IEntity const *) = ::IEntity::GetVisible;
  return IEntity_GetVisible$(ptr);
}

bool IEntity$cxxbridge1$IEntity_IsFrozen(::alt::IEntity const *ptr) noexcept {
  bool (*IEntity_IsFrozen$)(::alt::IEntity const *) = ::IEntity::IsFrozen;
  return IEntity_IsFrozen$(ptr);
}

void IEntity$cxxbridge1$IEntity_SetFrozen(::alt::IEntity *ptr, bool state) noexcept {
  void (*IEntity_SetFrozen$)(::alt::IEntity *, bool) = ::IEntity::SetFrozen;
  IEntity_SetFrozen$(ptr, state);
}

::std::uint32_t IEntity$cxxbridge1$IEntity_GetTimestamp(::alt::IEntity const *ptr) noexcept {
  ::std::uint32_t (*IEntity_GetTimestamp$)(::alt::IEntity const *) = ::IEntity::GetTimestamp;
  return IEntity_GetTimestamp$(ptr);
}

void IEntity$cxxbridge1$IEntity_SetNetworkOwner(::alt::IEntity *ptr, ::alt::IPlayer *player, bool disableMigration) noexcept {
  void (*IEntity_SetNetworkOwner$)(::alt::IEntity *, ::alt::IPlayer *, bool) = ::IEntity::SetNetworkOwner;
  IEntity_SetNetworkOwner$(ptr, player, disableMigration);
}
} // extern "C"
} // namespace IEntity

extern "C" {
void cxxbridge1$IEntity_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IEntity *ptr, ::std::string *key, ::MValueMutWrapper *val) noexcept {
  void (*IEntity_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IEntity *, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::IEntity_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IEntity_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), val);
}
} // extern "C"

namespace IEntity {
extern "C" {
void IEntity$cxxbridge1$IEntity_SetMultipleStreamSyncedMetaData(::alt::IEntity *ptr, ::MValueUnorderedMapWrapper const &values) noexcept {
  void (*IEntity_SetMultipleStreamSyncedMetaData$)(::alt::IEntity *, ::MValueUnorderedMapWrapper const &) = ::IEntity::SetMultipleStreamSyncedMetaData;
  IEntity_SetMultipleStreamSyncedMetaData$(ptr, values);
}
} // extern "C"
} // namespace IEntity

extern "C" {
void cxxbridge1$IEntity_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IEntity *ptr, ::std::string *key) noexcept {
  void (*IEntity_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IEntity *, ::std::unique_ptr<::std::string>) = ::IEntity_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IEntity_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}
} // extern "C"

namespace IEntity {
extern "C" {
void IEntity$cxxbridge1$IEntity_SetVisible(::alt::IEntity *ptr, bool toggle) noexcept {
  void (*IEntity_SetVisible$)(::alt::IEntity *, bool) = ::IEntity::SetVisible;
  IEntity_SetVisible$(ptr, toggle);
}

void IEntity$cxxbridge1$IEntity_AttachToEntity(::alt::IEntity *ptr, ::alt::IEntity *entity, ::std::uint16_t otherBoneId, ::std::uint16_t myBoneId, float position_x, float position_y, float position_z, float rotation_x, float rotation_y, float rotation_z, bool collision, bool noFixedRotation) noexcept {
  void (*IEntity_AttachToEntity$)(::alt::IEntity *, ::alt::IEntity *, ::std::uint16_t, ::std::uint16_t, float, float, float, float, float, float, bool, bool) = ::IEntity::AttachToEntity;
  IEntity_AttachToEntity$(ptr, entity, otherBoneId, myBoneId, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, collision, noFixedRotation);
}
} // extern "C"
} // namespace IEntity

extern "C" {
void cxxbridge1$IEntity_AttachToEntity1_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IEntity *ptr, ::alt::IEntity *entity, ::std::string *otherBoneName, ::std::string *myBoneName, float position_x, float position_y, float position_z, float rotation_x, float rotation_y, float rotation_z, bool collision, bool noFixedRotation) noexcept {
  void (*IEntity_AttachToEntity1_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IEntity *, ::alt::IEntity *, ::std::unique_ptr<::std::string>, ::std::unique_ptr<::std::string>, float, float, float, float, float, float, bool, bool) = ::IEntity_AttachToEntity1_autocxx_wrapper_0xd5d0abec981e3e3a;
  IEntity_AttachToEntity1_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, entity, ::std::unique_ptr<::std::string>(otherBoneName), ::std::unique_ptr<::std::string>(myBoneName), position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, collision, noFixedRotation);
}
} // extern "C"

namespace IEntity {
extern "C" {
void IEntity$cxxbridge1$IEntity_Detach(::alt::IEntity *ptr) noexcept {
  void (*IEntity_Detach$)(::alt::IEntity *) = ::IEntity::Detach;
  IEntity_Detach$(ptr);
}

void IEntity$cxxbridge1$IEntity_SetStreamed(::alt::IEntity *ptr, bool toggle) noexcept {
  void (*IEntity_SetStreamed$)(::alt::IEntity *, bool) = ::IEntity::SetStreamed;
  IEntity_SetStreamed$(ptr, toggle);
}

bool IEntity$cxxbridge1$IEntity_GetStreamed(::alt::IEntity const *ptr) noexcept {
  bool (*IEntity_GetStreamed$)(::alt::IEntity const *) = ::IEntity::GetStreamed;
  return IEntity_GetStreamed$(ptr);
}

bool IEntity$cxxbridge1$IEntity_HasCollision(::alt::IEntity const *ptr) noexcept {
  bool (*IEntity_HasCollision$)(::alt::IEntity const *) = ::IEntity::HasCollision;
  return IEntity_HasCollision$(ptr);
}

void IEntity$cxxbridge1$IEntity_SetCollision(::alt::IEntity *ptr, bool state) noexcept {
  void (*IEntity_SetCollision$)(::alt::IEntity *, bool) = ::IEntity::SetCollision;
  IEntity_SetCollision$(ptr, state);
}

::std::uint32_t IEntity$cxxbridge1$IEntity_GetStreamingDistance(::alt::IEntity const *ptr) noexcept {
  ::std::uint32_t (*IEntity_GetStreamingDistance$)(::alt::IEntity const *) = ::IEntity::GetStreamingDistance;
  return IEntity_GetStreamingDistance$(ptr);
}

void IEntity$cxxbridge1$IEntity_SetStreamingDistance(::alt::IEntity *ptr, ::std::uint32_t streamingDistance) noexcept {
  void (*IEntity_SetStreamingDistance$)(::alt::IEntity *, ::std::uint32_t) = ::IEntity::SetStreamingDistance;
  IEntity_SetStreamingDistance$(ptr, streamingDistance);
}
} // extern "C"
} // namespace IEntity

extern "C" {
::std::string *cxxbridge1$IPlayer_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IPlayer_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IPlayer {
extern "C" {
::std::uint16_t IPlayer$cxxbridge1$IPlayer_GetHealth(::alt::IPlayer const *ptr) noexcept {
  ::std::uint16_t (*IPlayer_GetHealth$)(::alt::IPlayer const *) = ::IPlayer::GetHealth;
  return IPlayer_GetHealth$(ptr);
}

::std::uint16_t IPlayer$cxxbridge1$IPlayer_GetMaxHealth(::alt::IPlayer const *ptr) noexcept {
  ::std::uint16_t (*IPlayer_GetMaxHealth$)(::alt::IPlayer const *) = ::IPlayer::GetMaxHealth;
  return IPlayer_GetMaxHealth$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_HasWeaponComponent(::alt::IPlayer const *ptr, ::std::uint32_t weapon, ::std::uint32_t component) noexcept {
  bool (*IPlayer_HasWeaponComponent$)(::alt::IPlayer const *, ::std::uint32_t, ::std::uint32_t) = ::IPlayer::HasWeaponComponent;
  return IPlayer_HasWeaponComponent$(ptr, weapon, component);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
::std::vector<::std::uint32_t> *cxxbridge1$IPlayer_GetCurrentWeaponComponents_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::uint32_t>> (*IPlayer_GetCurrentWeaponComponents_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetCurrentWeaponComponents_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetCurrentWeaponComponents_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IPlayer {
extern "C" {
::std::uint8_t IPlayer$cxxbridge1$IPlayer_GetWeaponTintIndex(::alt::IPlayer const *ptr, ::std::uint32_t weapon) noexcept {
  ::std::uint8_t (*IPlayer_GetWeaponTintIndex$)(::alt::IPlayer const *, ::std::uint32_t) = ::IPlayer::GetWeaponTintIndex;
  return IPlayer_GetWeaponTintIndex$(ptr, weapon);
}

::std::uint8_t IPlayer$cxxbridge1$IPlayer_GetCurrentWeaponTintIndex(::alt::IPlayer const *ptr) noexcept {
  ::std::uint8_t (*IPlayer_GetCurrentWeaponTintIndex$)(::alt::IPlayer const *) = ::IPlayer::GetCurrentWeaponTintIndex;
  return IPlayer_GetCurrentWeaponTintIndex$(ptr);
}

::std::uint32_t IPlayer$cxxbridge1$IPlayer_GetCurrentWeapon(::alt::IPlayer const *ptr) noexcept {
  ::std::uint32_t (*IPlayer_GetCurrentWeapon$)(::alt::IPlayer const *) = ::IPlayer::GetCurrentWeapon;
  return IPlayer_GetCurrentWeapon$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsDead(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsDead$)(::alt::IPlayer const *) = ::IPlayer::IsDead;
  return IPlayer_IsDead$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsJumping(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsJumping$)(::alt::IPlayer const *) = ::IPlayer::IsJumping;
  return IPlayer_IsJumping$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsInRagdoll(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsInRagdoll$)(::alt::IPlayer const *) = ::IPlayer::IsInRagdoll;
  return IPlayer_IsInRagdoll$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsAiming(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsAiming$)(::alt::IPlayer const *) = ::IPlayer::IsAiming;
  return IPlayer_IsAiming$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsShooting(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsShooting$)(::alt::IPlayer const *) = ::IPlayer::IsShooting;
  return IPlayer_IsShooting$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsReloading(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsReloading$)(::alt::IPlayer const *) = ::IPlayer::IsReloading;
  return IPlayer_IsReloading$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsEnteringVehicle(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsEnteringVehicle$)(::alt::IPlayer const *) = ::IPlayer::IsEnteringVehicle;
  return IPlayer_IsEnteringVehicle$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsLeavingVehicle(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsLeavingVehicle$)(::alt::IPlayer const *) = ::IPlayer::IsLeavingVehicle;
  return IPlayer_IsLeavingVehicle$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsOnLadder(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsOnLadder$)(::alt::IPlayer const *) = ::IPlayer::IsOnLadder;
  return IPlayer_IsOnLadder$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsInMelee(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsInMelee$)(::alt::IPlayer const *) = ::IPlayer::IsInMelee;
  return IPlayer_IsInMelee$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsInCover(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsInCover$)(::alt::IPlayer const *) = ::IPlayer::IsInCover;
  return IPlayer_IsInCover$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsParachuting(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsParachuting$)(::alt::IPlayer const *) = ::IPlayer::IsParachuting;
  return IPlayer_IsParachuting$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsInWater(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsInWater$)(::alt::IPlayer const *) = ::IPlayer::IsInWater;
  return IPlayer_IsInWater$(ptr);
}

::std::uint16_t IPlayer$cxxbridge1$IPlayer_GetArmour(::alt::IPlayer const *ptr) noexcept {
  ::std::uint16_t (*IPlayer_GetArmour$)(::alt::IPlayer const *) = ::IPlayer::GetArmour;
  return IPlayer_GetArmour$(ptr);
}

::std::uint16_t IPlayer$cxxbridge1$IPlayer_GetMaxArmour(::alt::IPlayer const *ptr) noexcept {
  ::std::uint16_t (*IPlayer_GetMaxArmour$)(::alt::IPlayer const *) = ::IPlayer::GetMaxArmour;
  return IPlayer_GetMaxArmour$(ptr);
}

float IPlayer$cxxbridge1$IPlayer_GetMoveSpeed(::alt::IPlayer const *ptr) noexcept {
  float (*IPlayer_GetMoveSpeed$)(::alt::IPlayer const *) = ::IPlayer::GetMoveSpeed;
  return IPlayer_GetMoveSpeed$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_GetAimPos_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*IPlayer_GetAimPos_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::Vector3Wrapper *) = ::IPlayer_GetAimPos_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetAimPos_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$IPlayer_GetHeadRotation_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*IPlayer_GetHeadRotation_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::Vector3Wrapper *) = ::IPlayer_GetHeadRotation_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetHeadRotation_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_IsInVehicle(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsInVehicle$)(::alt::IPlayer const *) = ::IPlayer::IsInVehicle;
  return IPlayer_IsInVehicle$(ptr);
}

::alt::IVehicle *IPlayer$cxxbridge1$IPlayer_GetVehicle(::alt::IPlayer const *ptr) noexcept {
  ::alt::IVehicle *(*IPlayer_GetVehicle$)(::alt::IPlayer const *) = ::IPlayer::GetVehicle;
  return IPlayer_GetVehicle$(ptr);
}

::std::uint8_t IPlayer$cxxbridge1$IPlayer_GetSeat(::alt::IPlayer const *ptr) noexcept {
  ::std::uint8_t (*IPlayer_GetSeat$)(::alt::IPlayer const *) = ::IPlayer::GetSeat;
  return IPlayer_GetSeat$(ptr);
}

::alt::IEntity *IPlayer$cxxbridge1$IPlayer_GetEntityAimingAt(::alt::IPlayer const *ptr) noexcept {
  ::alt::IEntity *(*IPlayer_GetEntityAimingAt$)(::alt::IPlayer const *) = ::IPlayer::GetEntityAimingAt;
  return IPlayer_GetEntityAimingAt$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_GetEntityAimOffset_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*IPlayer_GetEntityAimOffset_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::Vector3Wrapper *) = ::IPlayer_GetEntityAimOffset_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetEntityAimOffset_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_IsFlashlightActive(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsFlashlightActive$)(::alt::IPlayer const *) = ::IPlayer::IsFlashlightActive;
  return IPlayer_IsFlashlightActive$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsSuperJumpEnabled(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsSuperJumpEnabled$)(::alt::IPlayer const *) = ::IPlayer::IsSuperJumpEnabled;
  return IPlayer_IsSuperJumpEnabled$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsCrouching(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsCrouching$)(::alt::IPlayer const *) = ::IPlayer::IsCrouching;
  return IPlayer_IsCrouching$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsStealthy(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsStealthy$)(::alt::IPlayer const *) = ::IPlayer::IsStealthy;
  return IPlayer_IsStealthy$(ptr);
}

::std::uint32_t IPlayer$cxxbridge1$IPlayer_GetCurrentAnimationDict(::alt::IPlayer const *ptr) noexcept {
  ::std::uint32_t (*IPlayer_GetCurrentAnimationDict$)(::alt::IPlayer const *) = ::IPlayer::GetCurrentAnimationDict;
  return IPlayer_GetCurrentAnimationDict$(ptr);
}

::std::uint32_t IPlayer$cxxbridge1$IPlayer_GetCurrentAnimationName(::alt::IPlayer const *ptr) noexcept {
  ::std::uint32_t (*IPlayer_GetCurrentAnimationName$)(::alt::IPlayer const *) = ::IPlayer::GetCurrentAnimationName;
  return IPlayer_GetCurrentAnimationName$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsSpawned(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsSpawned$)(::alt::IPlayer const *) = ::IPlayer::IsSpawned;
  return IPlayer_IsSpawned$(ptr);
}

float IPlayer$cxxbridge1$IPlayer_GetForwardSpeed(::alt::IPlayer const *ptr) noexcept {
  float (*IPlayer_GetForwardSpeed$)(::alt::IPlayer const *) = ::IPlayer::GetForwardSpeed;
  return IPlayer_GetForwardSpeed$(ptr);
}

float IPlayer$cxxbridge1$IPlayer_GetStrafeSpeed(::alt::IPlayer const *ptr) noexcept {
  float (*IPlayer_GetStrafeSpeed$)(::alt::IPlayer const *) = ::IPlayer::GetStrafeSpeed;
  return IPlayer_GetStrafeSpeed$(ptr);
}

bool IPlayer$cxxbridge1$IPlayer_IsConnected(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsConnected$)(::alt::IPlayer const *) = ::IPlayer::IsConnected;
  return IPlayer_IsConnected$(ptr);
}

::std::uint32_t IPlayer$cxxbridge1$IPlayer_GetPing(::alt::IPlayer const *ptr) noexcept {
  ::std::uint32_t (*IPlayer_GetPing$)(::alt::IPlayer const *) = ::IPlayer::GetPing;
  return IPlayer_GetPing$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
::std::string *cxxbridge1$IPlayer_GetIP_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IPlayer_GetIP_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetIP_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetIP_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IPlayer {
extern "C" {
::std::uint64_t IPlayer$cxxbridge1$IPlayer_GetSocialID(::alt::IPlayer const *ptr) noexcept {
  ::std::uint64_t (*IPlayer_GetSocialID$)(::alt::IPlayer const *) = ::IPlayer::GetSocialID;
  return IPlayer_GetSocialID$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
::std::string *cxxbridge1$IPlayer_GetSocialClubName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IPlayer_GetSocialClubName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetSocialClubName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetSocialClubName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IPlayer {
extern "C" {
::std::uint64_t IPlayer$cxxbridge1$IPlayer_GetHwidHash(::alt::IPlayer const *ptr) noexcept {
  ::std::uint64_t (*IPlayer_GetHwidHash$)(::alt::IPlayer const *) = ::IPlayer::GetHwidHash;
  return IPlayer_GetHwidHash$(ptr);
}

::std::uint64_t IPlayer$cxxbridge1$IPlayer_GetHwidExHash(::alt::IPlayer const *ptr) noexcept {
  ::std::uint64_t (*IPlayer_GetHwidExHash$)(::alt::IPlayer const *) = ::IPlayer::GetHwidExHash;
  return IPlayer_GetHwidExHash$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
::std::string *cxxbridge1$IPlayer_GetHwid3_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IPlayer_GetHwid3_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetHwid3_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetHwid3_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$IPlayer_GetAuthToken_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IPlayer_GetAuthToken_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetAuthToken_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetAuthToken_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IPlayer {
extern "C" {
::std::int64_t IPlayer$cxxbridge1$IPlayer_GetDiscordId(::alt::IPlayer const *ptr) noexcept {
  ::std::int64_t (*IPlayer_GetDiscordId$)(::alt::IPlayer const *) = ::IPlayer::GetDiscordId;
  return IPlayer_GetDiscordId$(ptr);
}

void IPlayer$cxxbridge1$IPlayer_Spawn(::alt::IPlayer *ptr, float pos_x, float pos_y, float pos_z, ::std::uint32_t delayMs) noexcept {
  void (*IPlayer_Spawn$)(::alt::IPlayer *, float, float, float, ::std::uint32_t) = ::IPlayer::Spawn;
  IPlayer_Spawn$(ptr, pos_x, pos_y, pos_z, delayMs);
}

void IPlayer$cxxbridge1$IPlayer_Despawn(::alt::IPlayer *ptr) noexcept {
  void (*IPlayer_Despawn$)(::alt::IPlayer *) = ::IPlayer::Despawn;
  IPlayer_Despawn$(ptr);
}

void IPlayer$cxxbridge1$IPlayer_SetModel(::alt::IPlayer *ptr, ::std::uint32_t model) noexcept {
  void (*IPlayer_SetModel$)(::alt::IPlayer *, ::std::uint32_t) = ::IPlayer::SetModel;
  IPlayer_SetModel$(ptr, model);
}

void IPlayer$cxxbridge1$IPlayer_SetArmour(::alt::IPlayer *ptr, ::std::uint16_t armor) noexcept {
  void (*IPlayer_SetArmour$)(::alt::IPlayer *, ::std::uint16_t) = ::IPlayer::SetArmour;
  IPlayer_SetArmour$(ptr, armor);
}

void IPlayer$cxxbridge1$IPlayer_SetMaxArmour(::alt::IPlayer *ptr, ::std::uint16_t armor) noexcept {
  void (*IPlayer_SetMaxArmour$)(::alt::IPlayer *, ::std::uint16_t) = ::IPlayer::SetMaxArmour;
  IPlayer_SetMaxArmour$(ptr, armor);
}

void IPlayer$cxxbridge1$IPlayer_SetCurrentWeapon(::alt::IPlayer *ptr, ::std::uint32_t weapon) noexcept {
  void (*IPlayer_SetCurrentWeapon$)(::alt::IPlayer *, ::std::uint32_t) = ::IPlayer::SetCurrentWeapon;
  IPlayer_SetCurrentWeapon$(ptr, weapon);
}

void IPlayer$cxxbridge1$IPlayer_SetWeaponTintIndex(::alt::IPlayer *ptr, ::std::uint32_t weapon, ::std::uint8_t tintIndex) noexcept {
  void (*IPlayer_SetWeaponTintIndex$)(::alt::IPlayer *, ::std::uint32_t, ::std::uint8_t) = ::IPlayer::SetWeaponTintIndex;
  IPlayer_SetWeaponTintIndex$(ptr, weapon, tintIndex);
}

void IPlayer$cxxbridge1$IPlayer_AddWeaponComponent(::alt::IPlayer *ptr, ::std::uint32_t weapon, ::std::uint32_t component) noexcept {
  void (*IPlayer_AddWeaponComponent$)(::alt::IPlayer *, ::std::uint32_t, ::std::uint32_t) = ::IPlayer::AddWeaponComponent;
  IPlayer_AddWeaponComponent$(ptr, weapon, component);
}

void IPlayer$cxxbridge1$IPlayer_RemoveWeaponComponent(::alt::IPlayer *ptr, ::std::uint32_t weapon, ::std::uint32_t component) noexcept {
  void (*IPlayer_RemoveWeaponComponent$)(::alt::IPlayer *, ::std::uint32_t, ::std::uint32_t) = ::IPlayer::RemoveWeaponComponent;
  IPlayer_RemoveWeaponComponent$(ptr, weapon, component);
}

void IPlayer$cxxbridge1$IPlayer_ClearBloodDamage(::alt::IPlayer *ptr) noexcept {
  void (*IPlayer_ClearBloodDamage$)(::alt::IPlayer *) = ::IPlayer::ClearBloodDamage;
  IPlayer_ClearBloodDamage$(ptr);
}

void IPlayer$cxxbridge1$IPlayer_SetHealth(::alt::IPlayer *ptr, ::std::uint16_t health) noexcept {
  void (*IPlayer_SetHealth$)(::alt::IPlayer *, ::std::uint16_t) = ::IPlayer::SetHealth;
  IPlayer_SetHealth$(ptr, health);
}

void IPlayer$cxxbridge1$IPlayer_SetMaxHealth(::alt::IPlayer *ptr, ::std::uint16_t health) noexcept {
  void (*IPlayer_SetMaxHealth$)(::alt::IPlayer *, ::std::uint16_t) = ::IPlayer::SetMaxHealth;
  IPlayer_SetMaxHealth$(ptr, health);
}

void IPlayer$cxxbridge1$IPlayer_GiveWeapon(::alt::IPlayer *ptr, ::std::uint32_t weapon, ::std::int32_t ammo, bool selectWeapon) noexcept {
  void (*IPlayer_GiveWeapon$)(::alt::IPlayer *, ::std::uint32_t, ::std::int32_t, bool) = ::IPlayer::GiveWeapon;
  IPlayer_GiveWeapon$(ptr, weapon, ammo, selectWeapon);
}

bool IPlayer$cxxbridge1$IPlayer_RemoveWeapon(::alt::IPlayer *ptr, ::std::uint32_t weapon) noexcept {
  bool (*IPlayer_RemoveWeapon$)(::alt::IPlayer *, ::std::uint32_t) = ::IPlayer::RemoveWeapon;
  return IPlayer_RemoveWeapon$(ptr, weapon);
}

void IPlayer$cxxbridge1$IPlayer_RemoveAllWeapons(::alt::IPlayer *ptr, bool removeAllAmmo) noexcept {
  void (*IPlayer_RemoveAllWeapons$)(::alt::IPlayer *, bool) = ::IPlayer::RemoveAllWeapons;
  IPlayer_RemoveAllWeapons$(ptr, removeAllAmmo);
}

void IPlayer$cxxbridge1$IPlayer_SetDateTime(::alt::IPlayer *ptr, ::c_int *day, ::c_int *month, ::c_int *year, ::c_int *hour, ::c_int *minute, ::c_int *second) noexcept {
  void (*IPlayer_SetDateTime$)(::alt::IPlayer *, ::c_int, ::c_int, ::c_int, ::c_int, ::c_int, ::c_int) = ::IPlayer::SetDateTime;
  IPlayer_SetDateTime$(ptr, ::std::move(*day), ::std::move(*month), ::std::move(*year), ::std::move(*hour), ::std::move(*minute), ::std::move(*second));
}

void IPlayer$cxxbridge1$IPlayer_SetWeather(::alt::IPlayer *ptr, ::std::uint32_t weather) noexcept {
  void (*IPlayer_SetWeather$)(::alt::IPlayer *, ::std::uint32_t) = ::IPlayer::SetWeather;
  IPlayer_SetWeather$(ptr, weather);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_Kick_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer *ptr, ::std::string *reason) noexcept {
  void (*IPlayer_Kick_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer *, ::std::unique_ptr<::std::string>) = ::IPlayer_Kick_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_Kick_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(reason));
}

void cxxbridge1$IPlayer_GetClothes_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::std::uint8_t component, ::alt::Cloth *placement_return_type) noexcept {
  void (*IPlayer_GetClothes_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::std::uint8_t, ::alt::Cloth *) = ::IPlayer_GetClothes_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetClothes_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, component, placement_return_type);
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_SetClothes(::alt::IPlayer *ptr, ::std::uint8_t component, ::std::uint16_t drawable, ::std::uint8_t texture, ::std::uint8_t palette) noexcept {
  bool (*IPlayer_SetClothes$)(::alt::IPlayer *, ::std::uint8_t, ::std::uint16_t, ::std::uint8_t, ::std::uint8_t) = ::IPlayer::SetClothes;
  return IPlayer_SetClothes$(ptr, component, drawable, texture, palette);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_GetDlcClothes_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::std::uint8_t component, ::alt::DlcCloth *placement_return_type) noexcept {
  void (*IPlayer_GetDlcClothes_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::std::uint8_t, ::alt::DlcCloth *) = ::IPlayer_GetDlcClothes_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetDlcClothes_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, component, placement_return_type);
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_SetDlcClothes(::alt::IPlayer *ptr, ::std::uint8_t component, ::std::uint16_t drawable, ::std::uint8_t texture, ::std::uint8_t palette, ::std::uint32_t dlc) noexcept {
  bool (*IPlayer_SetDlcClothes$)(::alt::IPlayer *, ::std::uint8_t, ::std::uint16_t, ::std::uint8_t, ::std::uint8_t, ::std::uint32_t) = ::IPlayer::SetDlcClothes;
  return IPlayer_SetDlcClothes$(ptr, component, drawable, texture, palette, dlc);
}

bool IPlayer$cxxbridge1$IPlayer_ClearClothes(::alt::IPlayer *ptr, ::std::uint8_t component) noexcept {
  bool (*IPlayer_ClearClothes$)(::alt::IPlayer *, ::std::uint8_t) = ::IPlayer::ClearClothes;
  return IPlayer_ClearClothes$(ptr, component);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_GetProps_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::std::uint8_t component, ::alt::Prop *placement_return_type) noexcept {
  void (*IPlayer_GetProps_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::std::uint8_t, ::alt::Prop *) = ::IPlayer_GetProps_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetProps_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, component, placement_return_type);
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_SetProps(::alt::IPlayer *ptr, ::std::uint8_t component, ::std::uint16_t drawable, ::std::uint8_t texture) noexcept {
  bool (*IPlayer_SetProps$)(::alt::IPlayer *, ::std::uint8_t, ::std::uint16_t, ::std::uint8_t) = ::IPlayer::SetProps;
  return IPlayer_SetProps$(ptr, component, drawable, texture);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_GetDlcProps_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::std::uint8_t component, ::alt::DlcProp *placement_return_type) noexcept {
  void (*IPlayer_GetDlcProps_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::std::uint8_t, ::alt::DlcProp *) = ::IPlayer_GetDlcProps_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetDlcProps_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, component, placement_return_type);
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_SetDlcProps(::alt::IPlayer *ptr, ::std::uint8_t component, ::std::uint8_t drawable, ::std::uint8_t texture, ::std::uint32_t dlc) noexcept {
  bool (*IPlayer_SetDlcProps$)(::alt::IPlayer *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint32_t) = ::IPlayer::SetDlcProps;
  return IPlayer_SetDlcProps$(ptr, component, drawable, texture, dlc);
}

void IPlayer$cxxbridge1$IPlayer_ClearProps(::alt::IPlayer *ptr, ::std::uint8_t component) noexcept {
  void (*IPlayer_ClearProps$)(::alt::IPlayer *, ::std::uint8_t) = ::IPlayer::ClearProps;
  IPlayer_ClearProps$(ptr, component);
}

bool IPlayer$cxxbridge1$IPlayer_IsEntityInStreamingRange(::alt::IPlayer *ptr, ::std::uint16_t entityId) noexcept {
  bool (*IPlayer_IsEntityInStreamingRange$)(::alt::IPlayer *, ::std::uint16_t) = ::IPlayer::IsEntityInStreamingRange;
  return IPlayer_IsEntityInStreamingRange$(ptr, entityId);
}

void IPlayer$cxxbridge1$IPlayer_SetInvincible(::alt::IPlayer *ptr, bool toggle) noexcept {
  void (*IPlayer_SetInvincible$)(::alt::IPlayer *, bool) = ::IPlayer::SetInvincible;
  IPlayer_SetInvincible$(ptr, toggle);
}

bool IPlayer$cxxbridge1$IPlayer_GetInvincible(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_GetInvincible$)(::alt::IPlayer const *) = ::IPlayer::GetInvincible;
  return IPlayer_GetInvincible$(ptr);
}

void IPlayer$cxxbridge1$IPlayer_SetIntoVehicle(::alt::IPlayer *ptr, ::alt::IVehicle *vehicle, ::std::uint8_t seat) noexcept {
  void (*IPlayer_SetIntoVehicle$)(::alt::IPlayer *, ::alt::IVehicle *, ::std::uint8_t) = ::IPlayer::SetIntoVehicle;
  IPlayer_SetIntoVehicle$(ptr, vehicle, seat);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_PlayAmbientSpeech_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer *ptr, ::std::string *speechName, ::std::string *speechParam, ::std::uint32_t speechDictHash) noexcept {
  void (*IPlayer_PlayAmbientSpeech_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer *, ::std::unique_ptr<::std::string>, ::std::unique_ptr<::std::string>, ::std::uint32_t) = ::IPlayer_PlayAmbientSpeech_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_PlayAmbientSpeech_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(speechName), ::std::unique_ptr<::std::string>(speechParam), speechDictHash);
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_SetHeadOverlay(::alt::IPlayer *ptr, ::std::uint8_t overlayID, ::std::uint8_t index, float opacity) noexcept {
  bool (*IPlayer_SetHeadOverlay$)(::alt::IPlayer *, ::std::uint8_t, ::std::uint8_t, float) = ::IPlayer::SetHeadOverlay;
  return IPlayer_SetHeadOverlay$(ptr, overlayID, index, opacity);
}

bool IPlayer$cxxbridge1$IPlayer_RemoveHeadOverlay(::alt::IPlayer *ptr, ::std::uint8_t overlayID) noexcept {
  bool (*IPlayer_RemoveHeadOverlay$)(::alt::IPlayer *, ::std::uint8_t) = ::IPlayer::RemoveHeadOverlay;
  return IPlayer_RemoveHeadOverlay$(ptr, overlayID);
}

bool IPlayer$cxxbridge1$IPlayer_SetHeadOverlayColor(::alt::IPlayer *ptr, ::std::uint8_t overlayID, ::std::uint8_t colorType, ::std::uint8_t colorIndex, ::std::uint8_t secondColorIndex) noexcept {
  bool (*IPlayer_SetHeadOverlayColor$)(::alt::IPlayer *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::IPlayer::SetHeadOverlayColor;
  return IPlayer_SetHeadOverlayColor$(ptr, overlayID, colorType, colorIndex, secondColorIndex);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_GetHeadOverlay_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::std::uint8_t overlayID, ::alt::HeadOverlay *placement_return_type) noexcept {
  void (*IPlayer_GetHeadOverlay_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::std::uint8_t, ::alt::HeadOverlay *) = ::IPlayer_GetHeadOverlay_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetHeadOverlay_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, overlayID, placement_return_type);
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_SetFaceFeature(::alt::IPlayer *ptr, ::std::uint8_t index, float scale) noexcept {
  bool (*IPlayer_SetFaceFeature$)(::alt::IPlayer *, ::std::uint8_t, float) = ::IPlayer::SetFaceFeature;
  return IPlayer_SetFaceFeature$(ptr, index, scale);
}

float IPlayer$cxxbridge1$IPlayer_GetFaceFeatureScale(::alt::IPlayer const *ptr, ::std::uint8_t index) noexcept {
  float (*IPlayer_GetFaceFeatureScale$)(::alt::IPlayer const *, ::std::uint8_t) = ::IPlayer::GetFaceFeatureScale;
  return IPlayer_GetFaceFeatureScale$(ptr, index);
}

bool IPlayer$cxxbridge1$IPlayer_RemoveFaceFeature(::alt::IPlayer *ptr, ::std::uint8_t index) noexcept {
  bool (*IPlayer_RemoveFaceFeature$)(::alt::IPlayer *, ::std::uint8_t) = ::IPlayer::RemoveFaceFeature;
  return IPlayer_RemoveFaceFeature$(ptr, index);
}

bool IPlayer$cxxbridge1$IPlayer_SetHeadBlendPaletteColor(::alt::IPlayer *ptr, ::std::uint8_t id, ::std::uint8_t red, ::std::uint8_t green, ::std::uint8_t blue) noexcept {
  bool (*IPlayer_SetHeadBlendPaletteColor$)(::alt::IPlayer *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::IPlayer::SetHeadBlendPaletteColor;
  return IPlayer_SetHeadBlendPaletteColor$(ptr, id, red, green, blue);
}

void IPlayer$cxxbridge1$IPlayer_RemoveHeadBlendPaletteColor(::alt::IPlayer *ptr) noexcept {
  void (*IPlayer_RemoveHeadBlendPaletteColor$)(::alt::IPlayer *) = ::IPlayer::RemoveHeadBlendPaletteColor;
  IPlayer_RemoveHeadBlendPaletteColor$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_GetHeadBlendPaletteColor_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::std::uint8_t id, ::RGBAWrapper *placement_return_type) noexcept {
  void (*IPlayer_GetHeadBlendPaletteColor_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::std::uint8_t, ::RGBAWrapper *) = ::IPlayer_GetHeadBlendPaletteColor_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetHeadBlendPaletteColor_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, id, placement_return_type);
}
} // extern "C"

namespace IPlayer {
extern "C" {
void IPlayer$cxxbridge1$IPlayer_SetHeadBlendData(::alt::IPlayer *ptr, ::std::uint32_t shapeFirstID, ::std::uint32_t shapeSecondID, ::std::uint32_t shapeThirdID, ::std::uint32_t skinFirstID, ::std::uint32_t skinSecondID, ::std::uint32_t skinThirdID, float shapeMix, float skinMix, float thirdMix) noexcept {
  void (*IPlayer_SetHeadBlendData$)(::alt::IPlayer *, ::std::uint32_t, ::std::uint32_t, ::std::uint32_t, ::std::uint32_t, ::std::uint32_t, ::std::uint32_t, float, float, float) = ::IPlayer::SetHeadBlendData;
  IPlayer_SetHeadBlendData$(ptr, shapeFirstID, shapeSecondID, shapeThirdID, skinFirstID, skinSecondID, skinThirdID, shapeMix, skinMix, thirdMix);
}

void IPlayer$cxxbridge1$IPlayer_RemoveHeadBlendData(::alt::IPlayer *ptr) noexcept {
  void (*IPlayer_RemoveHeadBlendData$)(::alt::IPlayer *) = ::IPlayer::RemoveHeadBlendData;
  IPlayer_RemoveHeadBlendData$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_GetHeadBlendData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::alt::HeadBlendData *placement_return_type) noexcept {
  void (*IPlayer_GetHeadBlendData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::alt::HeadBlendData *) = ::IPlayer_GetHeadBlendData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetHeadBlendData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_SetEyeColor(::alt::IPlayer *ptr, ::std::int16_t eyeColor) noexcept {
  bool (*IPlayer_SetEyeColor$)(::alt::IPlayer *, ::std::int16_t) = ::IPlayer::SetEyeColor;
  return IPlayer_SetEyeColor$(ptr, eyeColor);
}

::std::int16_t IPlayer$cxxbridge1$IPlayer_GetEyeColor(::alt::IPlayer const *ptr) noexcept {
  ::std::int16_t (*IPlayer_GetEyeColor$)(::alt::IPlayer const *) = ::IPlayer::GetEyeColor;
  return IPlayer_GetEyeColor$(ptr);
}

void IPlayer$cxxbridge1$IPlayer_SetHairColor(::alt::IPlayer *ptr, ::std::uint8_t hairColor) noexcept {
  void (*IPlayer_SetHairColor$)(::alt::IPlayer *, ::std::uint8_t) = ::IPlayer::SetHairColor;
  IPlayer_SetHairColor$(ptr, hairColor);
}

::std::uint8_t IPlayer$cxxbridge1$IPlayer_GetHairColor(::alt::IPlayer const *ptr) noexcept {
  ::std::uint8_t (*IPlayer_GetHairColor$)(::alt::IPlayer const *) = ::IPlayer::GetHairColor;
  return IPlayer_GetHairColor$(ptr);
}

void IPlayer$cxxbridge1$IPlayer_SetHairHighlightColor(::alt::IPlayer *ptr, ::std::uint8_t hairHighlightColor) noexcept {
  void (*IPlayer_SetHairHighlightColor$)(::alt::IPlayer *, ::std::uint8_t) = ::IPlayer::SetHairHighlightColor;
  IPlayer_SetHairHighlightColor$(ptr, hairHighlightColor);
}

::std::uint8_t IPlayer$cxxbridge1$IPlayer_GetHairHighlightColor(::alt::IPlayer const *ptr) noexcept {
  ::std::uint8_t (*IPlayer_GetHairHighlightColor$)(::alt::IPlayer const *) = ::IPlayer::GetHairHighlightColor;
  return IPlayer_GetHairHighlightColor$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
::std::vector<::WeaponWrapper> *cxxbridge1$IPlayer_GetWeapons_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::WeaponWrapper>> (*IPlayer_GetWeapons_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetWeapons_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetWeapons_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_HasWeapon(::alt::IPlayer const *ptr, ::std::uint32_t weapon) noexcept {
  bool (*IPlayer_HasWeapon$)(::alt::IPlayer const *, ::std::uint32_t) = ::IPlayer::HasWeapon;
  return IPlayer_HasWeapon$(ptr, weapon);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
bool cxxbridge1$IPlayer_HasLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::std::string *key) noexcept {
  bool (*IPlayer_HasLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::std::unique_ptr<::std::string>) = ::IPlayer_HasLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_HasLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}

void cxxbridge1$IPlayer_SetLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer *ptr, ::std::string *key, ::MValueMutWrapper *val) noexcept {
  void (*IPlayer_SetLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer *, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::IPlayer_SetLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_SetLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), val);
}

void cxxbridge1$IPlayer_GetLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::std::string *key, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*IPlayer_GetLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::std::unique_ptr<::std::string>, ::ConstMValueWrapper *) = ::IPlayer_GetLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), placement_return_type);
}

void cxxbridge1$IPlayer_DeleteLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer *ptr, ::std::string *key) noexcept {
  void (*IPlayer_DeleteLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer *, ::std::unique_ptr<::std::string>) = ::IPlayer_DeleteLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_DeleteLocalMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}

::std::vector<::std::string> *cxxbridge1$IPlayer_GetLocalMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*IPlayer_GetLocalMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetLocalMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetLocalMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IPlayer {
extern "C" {
::std::uint32_t IPlayer$cxxbridge1$IPlayer_GetInteriorLocation(::alt::IPlayer const *ptr) noexcept {
  ::std::uint32_t (*IPlayer_GetInteriorLocation$)(::alt::IPlayer const *) = ::IPlayer::GetInteriorLocation;
  return IPlayer_GetInteriorLocation$(ptr);
}

::std::uint32_t IPlayer$cxxbridge1$IPlayer_GetLastDamagedBodyPart(::alt::IPlayer const *ptr) noexcept {
  ::std::uint32_t (*IPlayer_GetLastDamagedBodyPart$)(::alt::IPlayer const *) = ::IPlayer::GetLastDamagedBodyPart;
  return IPlayer_GetLastDamagedBodyPart$(ptr);
}

void IPlayer$cxxbridge1$IPlayer_SetLastDamagedBodyPart(::alt::IPlayer *ptr, ::std::uint32_t bodyPart) noexcept {
  void (*IPlayer_SetLastDamagedBodyPart$)(::alt::IPlayer *, ::std::uint32_t) = ::IPlayer::SetLastDamagedBodyPart;
  IPlayer_SetLastDamagedBodyPart$(ptr, bodyPart);
}

void IPlayer$cxxbridge1$IPlayer_SetSendNames(::alt::IPlayer *ptr, bool state) noexcept {
  void (*IPlayer_SetSendNames$)(::alt::IPlayer *, bool) = ::IPlayer::SetSendNames;
  IPlayer_SetSendNames$(ptr, state);
}

bool IPlayer$cxxbridge1$IPlayer_GetSendNames(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_GetSendNames$)(::alt::IPlayer const *) = ::IPlayer::GetSendNames;
  return IPlayer_GetSendNames$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_PlayAnimation_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer *ptr, ::std::string *animDict, ::std::string *animName, float blendInSpeed, float blendOutSpeed, ::c_int *duration, ::c_int *flags, float playbackRate, bool lockX, bool lockY, bool lockZ) noexcept {
  void (*IPlayer_PlayAnimation_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer *, ::std::unique_ptr<::std::string>, ::std::unique_ptr<::std::string>, float, float, ::c_int, ::c_int, float, bool, bool, bool) = ::IPlayer_PlayAnimation_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_PlayAnimation_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(animDict), ::std::unique_ptr<::std::string>(animName), blendInSpeed, blendOutSpeed, ::std::move(*duration), ::std::move(*flags), playbackRate, lockX, lockY, lockZ);
}
} // extern "C"

namespace IPlayer {
extern "C" {
void IPlayer$cxxbridge1$IPlayer_ClearTasks(::alt::IPlayer *ptr) noexcept {
  void (*IPlayer_ClearTasks$)(::alt::IPlayer *) = ::IPlayer::ClearTasks;
  IPlayer_ClearTasks$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_PlayScenario_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer *ptr, ::std::string *name) noexcept {
  void (*IPlayer_PlayScenario_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer *, ::std::unique_ptr<::std::string>) = ::IPlayer_PlayScenario_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_PlayScenario_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(name));
}

::std::vector<::StreamedEntityWrapper> *cxxbridge1$IPlayer_GetStreamedEntities_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::StreamedEntityWrapper>> (*IPlayer_GetStreamedEntities_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetStreamedEntities_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetStreamedEntities_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IPlayer {
extern "C" {
void IPlayer$cxxbridge1$IPlayer_SetAmmo(::alt::IPlayer *ptr, ::std::uint32_t ammoHash, ::std::uint16_t ammo) noexcept {
  void (*IPlayer_SetAmmo$)(::alt::IPlayer *, ::std::uint32_t, ::std::uint16_t) = ::IPlayer::SetAmmo;
  IPlayer_SetAmmo$(ptr, ammoHash, ammo);
}

::std::uint16_t IPlayer$cxxbridge1$IPlayer_GetAmmo(::alt::IPlayer const *ptr, ::std::uint32_t ammoHash) noexcept {
  ::std::uint16_t (*IPlayer_GetAmmo$)(::alt::IPlayer const *, ::std::uint32_t) = ::IPlayer::GetAmmo;
  return IPlayer_GetAmmo$(ptr, ammoHash);
}

void IPlayer$cxxbridge1$IPlayer_SetWeaponAmmo(::alt::IPlayer *ptr, ::std::uint32_t weaponHash, ::std::uint16_t ammo) noexcept {
  void (*IPlayer_SetWeaponAmmo$)(::alt::IPlayer *, ::std::uint32_t, ::std::uint16_t) = ::IPlayer::SetWeaponAmmo;
  IPlayer_SetWeaponAmmo$(ptr, weaponHash, ammo);
}

::std::uint16_t IPlayer$cxxbridge1$IPlayer_GetWeaponAmmo(::alt::IPlayer const *ptr, ::std::uint32_t weaponHash) noexcept {
  ::std::uint16_t (*IPlayer_GetWeaponAmmo$)(::alt::IPlayer const *, ::std::uint32_t) = ::IPlayer::GetWeaponAmmo;
  return IPlayer_GetWeaponAmmo$(ptr, weaponHash);
}

void IPlayer$cxxbridge1$IPlayer_SetAmmoSpecialType(::alt::IPlayer *ptr, ::std::uint32_t ammoHash, ::std::uint32_t ammoSpecialType) noexcept {
  void (*IPlayer_SetAmmoSpecialType$)(::alt::IPlayer *, ::std::uint32_t, ::std::uint32_t) = ::IPlayer::SetAmmoSpecialType;
  IPlayer_SetAmmoSpecialType$(ptr, ammoHash, ammoSpecialType);
}

::std::uint32_t IPlayer$cxxbridge1$IPlayer_GetAmmoSpecialType(::alt::IPlayer const *ptr, ::std::uint32_t ammoHash) noexcept {
  ::std::uint32_t (*IPlayer_GetAmmoSpecialType$)(::alt::IPlayer const *, ::std::uint32_t) = ::IPlayer::GetAmmoSpecialType;
  return IPlayer_GetAmmoSpecialType$(ptr, ammoHash);
}

void IPlayer$cxxbridge1$IPlayer_SetAmmoFlags(::alt::IPlayer *ptr, ::std::uint32_t ammoHash, bool ammoFlags_infiniteAmmo, bool ammoFlags_addSmokeOnExplosion, bool ammoFlags_fuse, bool ammoFlags_fixedAfterExplosion) noexcept {
  void (*IPlayer_SetAmmoFlags$)(::alt::IPlayer *, ::std::uint32_t, bool, bool, bool, bool) = ::IPlayer::SetAmmoFlags;
  IPlayer_SetAmmoFlags$(ptr, ammoHash, ammoFlags_infiniteAmmo, ammoFlags_addSmokeOnExplosion, ammoFlags_fuse, ammoFlags_fixedAfterExplosion);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
void cxxbridge1$IPlayer_GetAmmoFlags_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr, ::std::uint32_t ammoHash, ::alt::AmmoFlags *placement_return_type) noexcept {
  void (*IPlayer_GetAmmoFlags_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *, ::std::uint32_t, ::alt::AmmoFlags *) = ::IPlayer_GetAmmoFlags_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_GetAmmoFlags_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ammoHash, placement_return_type);
}
} // extern "C"

namespace IPlayer {
extern "C" {
void IPlayer$cxxbridge1$IPlayer_SetAmmoMax(::alt::IPlayer *ptr, ::std::uint32_t ammoHash, ::std::int32_t ammoMax) noexcept {
  void (*IPlayer_SetAmmoMax$)(::alt::IPlayer *, ::std::uint32_t, ::std::int32_t) = ::IPlayer::SetAmmoMax;
  IPlayer_SetAmmoMax$(ptr, ammoHash, ammoMax);
}

::std::int32_t IPlayer$cxxbridge1$IPlayer_GetAmmoMax(::alt::IPlayer const *ptr, ::std::uint32_t ammoHash) noexcept {
  ::std::int32_t (*IPlayer_GetAmmoMax$)(::alt::IPlayer const *, ::std::uint32_t) = ::IPlayer::GetAmmoMax;
  return IPlayer_GetAmmoMax$(ptr, ammoHash);
}

void IPlayer$cxxbridge1$IPlayer_SetAmmoMax50(::alt::IPlayer *ptr, ::std::uint32_t ammoHash, ::std::int32_t ammoMax50) noexcept {
  void (*IPlayer_SetAmmoMax50$)(::alt::IPlayer *, ::std::uint32_t, ::std::int32_t) = ::IPlayer::SetAmmoMax50;
  IPlayer_SetAmmoMax50$(ptr, ammoHash, ammoMax50);
}

::std::int32_t IPlayer$cxxbridge1$IPlayer_GetAmmoMax50(::alt::IPlayer const *ptr, ::std::uint32_t ammoHash) noexcept {
  ::std::int32_t (*IPlayer_GetAmmoMax50$)(::alt::IPlayer const *, ::std::uint32_t) = ::IPlayer::GetAmmoMax50;
  return IPlayer_GetAmmoMax50$(ptr, ammoHash);
}

void IPlayer$cxxbridge1$IPlayer_SetAmmoMax100(::alt::IPlayer *ptr, ::std::uint32_t ammoHash, ::std::int32_t ammoMax100) noexcept {
  void (*IPlayer_SetAmmoMax100$)(::alt::IPlayer *, ::std::uint32_t, ::std::int32_t) = ::IPlayer::SetAmmoMax100;
  IPlayer_SetAmmoMax100$(ptr, ammoHash, ammoMax100);
}

::std::int32_t IPlayer$cxxbridge1$IPlayer_GetAmmoMax100(::alt::IPlayer const *ptr, ::std::uint32_t ammoHash) noexcept {
  ::std::int32_t (*IPlayer_GetAmmoMax100$)(::alt::IPlayer const *, ::std::uint32_t) = ::IPlayer::GetAmmoMax100;
  return IPlayer_GetAmmoMax100$(ptr, ammoHash);
}

void IPlayer$cxxbridge1$IPlayer_AddDecoration(::alt::IPlayer *ptr, ::std::uint32_t collection, ::std::uint32_t overlay, ::std::uint8_t count) noexcept {
  void (*IPlayer_AddDecoration$)(::alt::IPlayer *, ::std::uint32_t, ::std::uint32_t, ::std::uint8_t) = ::IPlayer::AddDecoration;
  IPlayer_AddDecoration$(ptr, collection, overlay, count);
}

void IPlayer$cxxbridge1$IPlayer_RemoveDecoration(::alt::IPlayer *ptr, ::std::uint32_t collection, ::std::uint32_t overlay) noexcept {
  void (*IPlayer_RemoveDecoration$)(::alt::IPlayer *, ::std::uint32_t, ::std::uint32_t) = ::IPlayer::RemoveDecoration;
  IPlayer_RemoveDecoration$(ptr, collection, overlay);
}

void IPlayer$cxxbridge1$IPlayer_ClearDecorations(::alt::IPlayer *ptr) noexcept {
  void (*IPlayer_ClearDecorations$)(::alt::IPlayer *) = ::IPlayer::ClearDecorations;
  IPlayer_ClearDecorations$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
::std::vector<::alt::CDecoration> *cxxbridge1$IPlayer_GetDecorations_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::alt::CDecoration>> (*IPlayer_GetDecorations_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetDecorations_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetDecorations_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IPlayer {
extern "C" {
bool IPlayer$cxxbridge1$IPlayer_IsNetworkOwnershipDisabled(::alt::IPlayer const *ptr) noexcept {
  bool (*IPlayer_IsNetworkOwnershipDisabled$)(::alt::IPlayer const *) = ::IPlayer::IsNetworkOwnershipDisabled;
  return IPlayer_IsNetworkOwnershipDisabled$(ptr);
}

void IPlayer$cxxbridge1$IPlayer_SetNetworkOwnershipDisabled(::alt::IPlayer *ptr, bool disabled) noexcept {
  void (*IPlayer_SetNetworkOwnershipDisabled$)(::alt::IPlayer *, bool) = ::IPlayer::SetNetworkOwnershipDisabled;
  IPlayer_SetNetworkOwnershipDisabled$(ptr, disabled);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
::std::string *cxxbridge1$IPlayer_GetCloudID_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IPlayer_GetCloudID_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetCloudID_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetCloudID_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IPlayer {
extern "C" {
::std::uint8_t IPlayer$cxxbridge1$IPlayer_GetCloudAuthResult(::alt::IPlayer const *ptr) noexcept {
  ::std::uint8_t (*IPlayer_GetCloudAuthResult$)(::alt::IPlayer const *) = ::IPlayer::GetCloudAuthResult;
  return IPlayer_GetCloudAuthResult$(ptr);
}
} // extern "C"
} // namespace IPlayer

extern "C" {
::std::string *cxxbridge1$IPlayer_GetBloodDamageBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IPlayer_GetBloodDamageBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer const *) = ::IPlayer_GetBloodDamageBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IPlayer_GetBloodDamageBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$IPlayer_SetBloodDamageBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer *ptr, ::std::string *_base64) noexcept {
  void (*IPlayer_SetBloodDamageBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer *, ::std::unique_ptr<::std::string>) = ::IPlayer_SetBloodDamageBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  IPlayer_SetBloodDamageBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(_base64));
}
} // extern "C"

namespace IVehicle {
extern "C" {
::alt::IPlayer *IVehicle$cxxbridge1$IVehicle_GetDriver(::alt::IVehicle const *ptr) noexcept {
  ::alt::IPlayer *(*IVehicle_GetDriver$)(::alt::IVehicle const *) = ::IVehicle::GetDriver;
  return IVehicle_GetDriver$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsDestroyed(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsDestroyed$)(::alt::IVehicle const *) = ::IVehicle::IsDestroyed;
  return IVehicle_IsDestroyed$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetMod(::alt::IVehicle const *ptr, ::std::uint8_t category) noexcept {
  ::std::uint8_t (*IVehicle_GetMod$)(::alt::IVehicle const *, ::std::uint8_t) = ::IVehicle::GetMod;
  return IVehicle_GetMod$(ptr, category);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetModsCount(::alt::IVehicle const *ptr, ::std::uint8_t category) noexcept {
  ::std::uint8_t (*IVehicle_GetModsCount$)(::alt::IVehicle const *, ::std::uint8_t) = ::IVehicle::GetModsCount;
  return IVehicle_GetModsCount$(ptr, category);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetModKitsCount(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetModKitsCount$)(::alt::IVehicle const *) = ::IVehicle::GetModKitsCount;
  return IVehicle_GetModKitsCount$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetModKit(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetModKit$)(::alt::IVehicle const *) = ::IVehicle::GetModKit;
  return IVehicle_GetModKit$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsPrimaryColorRGB(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsPrimaryColorRGB$)(::alt::IVehicle const *) = ::IVehicle::IsPrimaryColorRGB;
  return IVehicle_IsPrimaryColorRGB$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetPrimaryColor(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetPrimaryColor$)(::alt::IVehicle const *) = ::IVehicle::GetPrimaryColor;
  return IVehicle_GetPrimaryColor$(ptr);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
void cxxbridge1$IVehicle_GetPrimaryColorRGB_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr, ::RGBAWrapper *placement_return_type) noexcept {
  void (*IVehicle_GetPrimaryColorRGB_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *, ::RGBAWrapper *) = ::IVehicle_GetPrimaryColorRGB_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_GetPrimaryColorRGB_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IVehicle {
extern "C" {
bool IVehicle$cxxbridge1$IVehicle_IsSecondaryColorRGB(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsSecondaryColorRGB$)(::alt::IVehicle const *) = ::IVehicle::IsSecondaryColorRGB;
  return IVehicle_IsSecondaryColorRGB$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetSecondaryColor(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetSecondaryColor$)(::alt::IVehicle const *) = ::IVehicle::GetSecondaryColor;
  return IVehicle_GetSecondaryColor$(ptr);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
void cxxbridge1$IVehicle_GetSecondaryColorRGB_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr, ::RGBAWrapper *placement_return_type) noexcept {
  void (*IVehicle_GetSecondaryColorRGB_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *, ::RGBAWrapper *) = ::IVehicle_GetSecondaryColorRGB_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_GetSecondaryColorRGB_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IVehicle {
extern "C" {
::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetPearlColor(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetPearlColor$)(::alt::IVehicle const *) = ::IVehicle::GetPearlColor;
  return IVehicle_GetPearlColor$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetWheelColor(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetWheelColor$)(::alt::IVehicle const *) = ::IVehicle::GetWheelColor;
  return IVehicle_GetWheelColor$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetInteriorColor(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetInteriorColor$)(::alt::IVehicle const *) = ::IVehicle::GetInteriorColor;
  return IVehicle_GetInteriorColor$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetDashboardColor(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetDashboardColor$)(::alt::IVehicle const *) = ::IVehicle::GetDashboardColor;
  return IVehicle_GetDashboardColor$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsTireSmokeColorCustom(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsTireSmokeColorCustom$)(::alt::IVehicle const *) = ::IVehicle::IsTireSmokeColorCustom;
  return IVehicle_IsTireSmokeColorCustom$(ptr);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
void cxxbridge1$IVehicle_GetTireSmokeColor_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr, ::RGBAWrapper *placement_return_type) noexcept {
  void (*IVehicle_GetTireSmokeColor_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *, ::RGBAWrapper *) = ::IVehicle_GetTireSmokeColor_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_GetTireSmokeColor_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IVehicle {
extern "C" {
::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetWheelType(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetWheelType$)(::alt::IVehicle const *) = ::IVehicle::GetWheelType;
  return IVehicle_GetWheelType$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetWheelVariation(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetWheelVariation$)(::alt::IVehicle const *) = ::IVehicle::GetWheelVariation;
  return IVehicle_GetWheelVariation$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetRearWheelVariation(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetRearWheelVariation$)(::alt::IVehicle const *) = ::IVehicle::GetRearWheelVariation;
  return IVehicle_GetRearWheelVariation$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_GetCustomTires(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_GetCustomTires$)(::alt::IVehicle const *) = ::IVehicle::GetCustomTires;
  return IVehicle_GetCustomTires$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetSpecialDarkness(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetSpecialDarkness$)(::alt::IVehicle const *) = ::IVehicle::GetSpecialDarkness;
  return IVehicle_GetSpecialDarkness$(ptr);
}

::std::uint32_t IVehicle$cxxbridge1$IVehicle_GetNumberplateIndex(::alt::IVehicle const *ptr) noexcept {
  ::std::uint32_t (*IVehicle_GetNumberplateIndex$)(::alt::IVehicle const *) = ::IVehicle::GetNumberplateIndex;
  return IVehicle_GetNumberplateIndex$(ptr);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
::std::string *cxxbridge1$IVehicle_GetNumberplateText_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IVehicle_GetNumberplateText_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *) = ::IVehicle_GetNumberplateText_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IVehicle_GetNumberplateText_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IVehicle {
extern "C" {
::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetWindowTint(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetWindowTint$)(::alt::IVehicle const *) = ::IVehicle::GetWindowTint;
  return IVehicle_GetWindowTint$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetDirtLevel(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetDirtLevel$)(::alt::IVehicle const *) = ::IVehicle::GetDirtLevel;
  return IVehicle_GetDirtLevel$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsExtraOn(::alt::IVehicle const *ptr, ::std::uint8_t extraID) noexcept {
  bool (*IVehicle_IsExtraOn$)(::alt::IVehicle const *, ::std::uint8_t) = ::IVehicle::IsExtraOn;
  return IVehicle_IsExtraOn$(ptr, extraID);
}

bool IVehicle$cxxbridge1$IVehicle_IsNeonActive(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsNeonActive$)(::alt::IVehicle const *) = ::IVehicle::IsNeonActive;
  return IVehicle_IsNeonActive$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_GetNeonActive(::alt::IVehicle const *ptr, bool *left, bool *right, bool *front, bool *back) noexcept {
  void (*IVehicle_GetNeonActive$)(::alt::IVehicle const *, bool *, bool *, bool *, bool *) = ::IVehicle::GetNeonActive;
  IVehicle_GetNeonActive$(ptr, left, right, front, back);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
void cxxbridge1$IVehicle_GetNeonColor_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr, ::RGBAWrapper *placement_return_type) noexcept {
  void (*IVehicle_GetNeonColor_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *, ::RGBAWrapper *) = ::IVehicle_GetNeonColor_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_GetNeonColor_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IVehicle {
extern "C" {
::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetLivery(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetLivery$)(::alt::IVehicle const *) = ::IVehicle::GetLivery;
  return IVehicle_GetLivery$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetRoofLivery(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetRoofLivery$)(::alt::IVehicle const *) = ::IVehicle::GetRoofLivery;
  return IVehicle_GetRoofLivery$(ptr);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
::std::string *cxxbridge1$IVehicle_GetAppearanceDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IVehicle_GetAppearanceDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *) = ::IVehicle_GetAppearanceDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IVehicle_GetAppearanceDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IVehicle {
extern "C" {
bool IVehicle$cxxbridge1$IVehicle_IsEngineOn(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsEngineOn$)(::alt::IVehicle const *) = ::IVehicle::IsEngineOn;
  return IVehicle_IsEngineOn$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsHandbrakeActive(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsHandbrakeActive$)(::alt::IVehicle const *) = ::IVehicle::IsHandbrakeActive;
  return IVehicle_IsHandbrakeActive$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetHeadlightColor(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetHeadlightColor$)(::alt::IVehicle const *) = ::IVehicle::GetHeadlightColor;
  return IVehicle_GetHeadlightColor$(ptr);
}

::std::uint32_t IVehicle$cxxbridge1$IVehicle_GetRadioStationIndex(::alt::IVehicle const *ptr) noexcept {
  ::std::uint32_t (*IVehicle_GetRadioStationIndex$)(::alt::IVehicle const *) = ::IVehicle::GetRadioStationIndex;
  return IVehicle_GetRadioStationIndex$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsSirenActive(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsSirenActive$)(::alt::IVehicle const *) = ::IVehicle::IsSirenActive;
  return IVehicle_IsSirenActive$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetLockState(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetLockState$)(::alt::IVehicle const *) = ::IVehicle::GetLockState;
  return IVehicle_GetLockState$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetDoorState(::alt::IVehicle const *ptr, ::std::uint8_t doorId) noexcept {
  ::std::uint8_t (*IVehicle_GetDoorState$)(::alt::IVehicle const *, ::std::uint8_t) = ::IVehicle::GetDoorState;
  return IVehicle_GetDoorState$(ptr, doorId);
}

bool IVehicle$cxxbridge1$IVehicle_IsWindowOpened(::alt::IVehicle const *ptr, ::std::uint8_t windowId) noexcept {
  bool (*IVehicle_IsWindowOpened$)(::alt::IVehicle const *, ::std::uint8_t) = ::IVehicle::IsWindowOpened;
  return IVehicle_IsWindowOpened$(ptr, windowId);
}

bool IVehicle$cxxbridge1$IVehicle_IsDaylightOn(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsDaylightOn$)(::alt::IVehicle const *) = ::IVehicle::IsDaylightOn;
  return IVehicle_IsDaylightOn$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsNightlightOn(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsNightlightOn$)(::alt::IVehicle const *) = ::IVehicle::IsNightlightOn;
  return IVehicle_IsNightlightOn$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetRoofState(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetRoofState$)(::alt::IVehicle const *) = ::IVehicle::GetRoofState;
  return IVehicle_GetRoofState$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsFlamethrowerActive(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsFlamethrowerActive$)(::alt::IVehicle const *) = ::IVehicle::IsFlamethrowerActive;
  return IVehicle_IsFlamethrowerActive$(ptr);
}

float IVehicle$cxxbridge1$IVehicle_GetLightsMultiplier(::alt::IVehicle const *ptr) noexcept {
  float (*IVehicle_GetLightsMultiplier$)(::alt::IVehicle const *) = ::IVehicle::GetLightsMultiplier;
  return IVehicle_GetLightsMultiplier$(ptr);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
::std::string *cxxbridge1$IVehicle_GetGameStateBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IVehicle_GetGameStateBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *) = ::IVehicle_GetGameStateBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IVehicle_GetGameStateBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IVehicle {
extern "C" {
::std::int32_t IVehicle$cxxbridge1$IVehicle_GetEngineHealth(::alt::IVehicle const *ptr) noexcept {
  ::std::int32_t (*IVehicle_GetEngineHealth$)(::alt::IVehicle const *) = ::IVehicle::GetEngineHealth;
  return IVehicle_GetEngineHealth$(ptr);
}

::std::int32_t IVehicle$cxxbridge1$IVehicle_GetPetrolTankHealth(::alt::IVehicle const *ptr) noexcept {
  ::std::int32_t (*IVehicle_GetPetrolTankHealth$)(::alt::IVehicle const *) = ::IVehicle::GetPetrolTankHealth;
  return IVehicle_GetPetrolTankHealth$(ptr);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetWheelsCount(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetWheelsCount$)(::alt::IVehicle const *) = ::IVehicle::GetWheelsCount;
  return IVehicle_GetWheelsCount$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsWheelBurst(::alt::IVehicle *ptr, ::std::uint8_t wheelId) noexcept {
  bool (*IVehicle_IsWheelBurst$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::IsWheelBurst;
  return IVehicle_IsWheelBurst$(ptr, wheelId);
}

bool IVehicle$cxxbridge1$IVehicle_DoesWheelHasTire(::alt::IVehicle *ptr, ::std::uint8_t wheelId) noexcept {
  bool (*IVehicle_DoesWheelHasTire$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::DoesWheelHasTire;
  return IVehicle_DoesWheelHasTire$(ptr, wheelId);
}

bool IVehicle$cxxbridge1$IVehicle_IsWheelDetached(::alt::IVehicle *ptr, ::std::uint8_t wheelId) noexcept {
  bool (*IVehicle_IsWheelDetached$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::IsWheelDetached;
  return IVehicle_IsWheelDetached$(ptr, wheelId);
}

bool IVehicle$cxxbridge1$IVehicle_IsWheelOnFire(::alt::IVehicle *ptr, ::std::uint8_t wheelId) noexcept {
  bool (*IVehicle_IsWheelOnFire$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::IsWheelOnFire;
  return IVehicle_IsWheelOnFire$(ptr, wheelId);
}

float IVehicle$cxxbridge1$IVehicle_GetWheelHealth(::alt::IVehicle *ptr, ::std::uint8_t wheelId) noexcept {
  float (*IVehicle_GetWheelHealth$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::GetWheelHealth;
  return IVehicle_GetWheelHealth$(ptr, wheelId);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetRepairsCount(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetRepairsCount$)(::alt::IVehicle const *) = ::IVehicle::GetRepairsCount;
  return IVehicle_GetRepairsCount$(ptr);
}

::std::uint32_t IVehicle$cxxbridge1$IVehicle_GetBodyHealth(::alt::IVehicle const *ptr) noexcept {
  ::std::uint32_t (*IVehicle_GetBodyHealth$)(::alt::IVehicle const *) = ::IVehicle::GetBodyHealth;
  return IVehicle_GetBodyHealth$(ptr);
}

::std::uint32_t IVehicle$cxxbridge1$IVehicle_GetBodyAdditionalHealth(::alt::IVehicle const *ptr) noexcept {
  ::std::uint32_t (*IVehicle_GetBodyAdditionalHealth$)(::alt::IVehicle const *) = ::IVehicle::GetBodyAdditionalHealth;
  return IVehicle_GetBodyAdditionalHealth$(ptr);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
::std::string *cxxbridge1$IVehicle_GetHealthDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IVehicle_GetHealthDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *) = ::IVehicle_GetHealthDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IVehicle_GetHealthDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IVehicle {
extern "C" {
::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetPartDamageLevel(::alt::IVehicle *ptr, ::std::uint8_t partId) noexcept {
  ::std::uint8_t (*IVehicle_GetPartDamageLevel$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::GetPartDamageLevel;
  return IVehicle_GetPartDamageLevel$(ptr, partId);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetPartBulletHoles(::alt::IVehicle *ptr, ::std::uint8_t partId) noexcept {
  ::std::uint8_t (*IVehicle_GetPartBulletHoles$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::GetPartBulletHoles;
  return IVehicle_GetPartBulletHoles$(ptr, partId);
}

bool IVehicle$cxxbridge1$IVehicle_IsLightDamaged(::alt::IVehicle *ptr, ::std::uint8_t lightId) noexcept {
  bool (*IVehicle_IsLightDamaged$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::IsLightDamaged;
  return IVehicle_IsLightDamaged$(ptr, lightId);
}

bool IVehicle$cxxbridge1$IVehicle_IsWindowDamaged(::alt::IVehicle *ptr, ::std::uint8_t windowId) noexcept {
  bool (*IVehicle_IsWindowDamaged$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::IsWindowDamaged;
  return IVehicle_IsWindowDamaged$(ptr, windowId);
}

bool IVehicle$cxxbridge1$IVehicle_IsSpecialLightDamaged(::alt::IVehicle *ptr, ::std::uint8_t specialLightId) noexcept {
  bool (*IVehicle_IsSpecialLightDamaged$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::IsSpecialLightDamaged;
  return IVehicle_IsSpecialLightDamaged$(ptr, specialLightId);
}

bool IVehicle$cxxbridge1$IVehicle_HasArmoredWindows(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_HasArmoredWindows$)(::alt::IVehicle const *) = ::IVehicle::HasArmoredWindows;
  return IVehicle_HasArmoredWindows$(ptr);
}

float IVehicle$cxxbridge1$IVehicle_GetArmoredWindowHealth(::alt::IVehicle *ptr, ::std::uint8_t windowId) noexcept {
  float (*IVehicle_GetArmoredWindowHealth$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::GetArmoredWindowHealth;
  return IVehicle_GetArmoredWindowHealth$(ptr, windowId);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetArmoredWindowShootCount(::alt::IVehicle *ptr, ::std::uint8_t windowId) noexcept {
  ::std::uint8_t (*IVehicle_GetArmoredWindowShootCount$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::GetArmoredWindowShootCount;
  return IVehicle_GetArmoredWindowShootCount$(ptr, windowId);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetBumperDamageLevel(::alt::IVehicle *ptr, ::std::uint8_t bumperId) noexcept {
  ::std::uint8_t (*IVehicle_GetBumperDamageLevel$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::GetBumperDamageLevel;
  return IVehicle_GetBumperDamageLevel$(ptr, bumperId);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
::std::string *cxxbridge1$IVehicle_GetDamageDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IVehicle_GetDamageDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *) = ::IVehicle_GetDamageDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IVehicle_GetDamageDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IVehicle {
extern "C" {
bool IVehicle$cxxbridge1$IVehicle_IsManualEngineControl(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsManualEngineControl$)(::alt::IVehicle const *) = ::IVehicle::IsManualEngineControl;
  return IVehicle_IsManualEngineControl$(ptr);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
::std::string *cxxbridge1$IVehicle_GetScriptDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IVehicle_GetScriptDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *) = ::IVehicle_GetScriptDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IVehicle_GetScriptDataBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IVehicle {
extern "C" {
void IVehicle$cxxbridge1$IVehicle_ToggleExtra(::alt::IVehicle *ptr, ::std::uint8_t extraID, bool state) noexcept {
  void (*IVehicle_ToggleExtra$)(::alt::IVehicle *, ::std::uint8_t, bool) = ::IVehicle::ToggleExtra;
  IVehicle_ToggleExtra$(ptr, extraID, state);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
void cxxbridge1$IVehicle_GetVelocity_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*IVehicle_GetVelocity_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *, ::Vector3Wrapper *) = ::IVehicle_GetVelocity_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_GetVelocity_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IVehicle {
extern "C" {
float IVehicle$cxxbridge1$IVehicle_GetSteeringAngle(::alt::IVehicle const *ptr) noexcept {
  float (*IVehicle_GetSteeringAngle$)(::alt::IVehicle const *) = ::IVehicle::GetSteeringAngle;
  return IVehicle_GetSteeringAngle$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetFixed(::alt::IVehicle *ptr) noexcept {
  void (*IVehicle_SetFixed$)(::alt::IVehicle *) = ::IVehicle::SetFixed;
  IVehicle_SetFixed$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_SetMod(::alt::IVehicle *ptr, ::std::uint8_t category, ::std::uint8_t id) noexcept {
  bool (*IVehicle_SetMod$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetMod;
  return IVehicle_SetMod$(ptr, category, id);
}

bool IVehicle$cxxbridge1$IVehicle_SetModKit(::alt::IVehicle *ptr, ::std::uint8_t id) noexcept {
  bool (*IVehicle_SetModKit$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetModKit;
  return IVehicle_SetModKit$(ptr, id);
}

void IVehicle$cxxbridge1$IVehicle_SetPrimaryColor(::alt::IVehicle *ptr, ::std::uint8_t color) noexcept {
  void (*IVehicle_SetPrimaryColor$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetPrimaryColor;
  IVehicle_SetPrimaryColor$(ptr, color);
}

void IVehicle$cxxbridge1$IVehicle_SetPrimaryColorRGB(::alt::IVehicle *ptr, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a) noexcept {
  void (*IVehicle_SetPrimaryColorRGB$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetPrimaryColorRGB;
  IVehicle_SetPrimaryColorRGB$(ptr, color_r, color_g, color_b, color_a);
}

void IVehicle$cxxbridge1$IVehicle_SetSecondaryColor(::alt::IVehicle *ptr, ::std::uint8_t color) noexcept {
  void (*IVehicle_SetSecondaryColor$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetSecondaryColor;
  IVehicle_SetSecondaryColor$(ptr, color);
}

void IVehicle$cxxbridge1$IVehicle_SetSecondaryColorRGB(::alt::IVehicle *ptr, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a) noexcept {
  void (*IVehicle_SetSecondaryColorRGB$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetSecondaryColorRGB;
  IVehicle_SetSecondaryColorRGB$(ptr, color_r, color_g, color_b, color_a);
}

void IVehicle$cxxbridge1$IVehicle_SetPearlColor(::alt::IVehicle *ptr, ::std::uint8_t color) noexcept {
  void (*IVehicle_SetPearlColor$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetPearlColor;
  IVehicle_SetPearlColor$(ptr, color);
}

void IVehicle$cxxbridge1$IVehicle_SetWheelColor(::alt::IVehicle *ptr, ::std::uint8_t color) noexcept {
  void (*IVehicle_SetWheelColor$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetWheelColor;
  IVehicle_SetWheelColor$(ptr, color);
}

void IVehicle$cxxbridge1$IVehicle_SetInteriorColor(::alt::IVehicle *ptr, ::std::uint8_t color) noexcept {
  void (*IVehicle_SetInteriorColor$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetInteriorColor;
  IVehicle_SetInteriorColor$(ptr, color);
}

void IVehicle$cxxbridge1$IVehicle_SetDashboardColor(::alt::IVehicle *ptr, ::std::uint8_t color) noexcept {
  void (*IVehicle_SetDashboardColor$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetDashboardColor;
  IVehicle_SetDashboardColor$(ptr, color);
}

void IVehicle$cxxbridge1$IVehicle_SetTireSmokeColor(::alt::IVehicle *ptr, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a) noexcept {
  void (*IVehicle_SetTireSmokeColor$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetTireSmokeColor;
  IVehicle_SetTireSmokeColor$(ptr, color_r, color_g, color_b, color_a);
}

void IVehicle$cxxbridge1$IVehicle_SetWheels(::alt::IVehicle *ptr, ::std::uint8_t type_, ::std::uint8_t variation) noexcept {
  void (*IVehicle_SetWheels$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetWheels;
  IVehicle_SetWheels$(ptr, type_, variation);
}

void IVehicle$cxxbridge1$IVehicle_SetRearWheels(::alt::IVehicle *ptr, ::std::uint8_t variation) noexcept {
  void (*IVehicle_SetRearWheels$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetRearWheels;
  IVehicle_SetRearWheels$(ptr, variation);
}

void IVehicle$cxxbridge1$IVehicle_SetCustomTires(::alt::IVehicle *ptr, bool state) noexcept {
  void (*IVehicle_SetCustomTires$)(::alt::IVehicle *, bool) = ::IVehicle::SetCustomTires;
  IVehicle_SetCustomTires$(ptr, state);
}

void IVehicle$cxxbridge1$IVehicle_SetSpecialDarkness(::alt::IVehicle *ptr, ::std::uint8_t value) noexcept {
  void (*IVehicle_SetSpecialDarkness$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetSpecialDarkness;
  IVehicle_SetSpecialDarkness$(ptr, value);
}

void IVehicle$cxxbridge1$IVehicle_SetNumberplateIndex(::alt::IVehicle *ptr, ::std::uint32_t index) noexcept {
  void (*IVehicle_SetNumberplateIndex$)(::alt::IVehicle *, ::std::uint32_t) = ::IVehicle::SetNumberplateIndex;
  IVehicle_SetNumberplateIndex$(ptr, index);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
void cxxbridge1$IVehicle_SetNumberplateText_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle *ptr, ::std::string *text) noexcept {
  void (*IVehicle_SetNumberplateText_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle *, ::std::unique_ptr<::std::string>) = ::IVehicle_SetNumberplateText_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_SetNumberplateText_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(text));
}
} // extern "C"

namespace IVehicle {
extern "C" {
void IVehicle$cxxbridge1$IVehicle_SetWindowTint(::alt::IVehicle *ptr, ::std::uint8_t tint) noexcept {
  void (*IVehicle_SetWindowTint$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetWindowTint;
  IVehicle_SetWindowTint$(ptr, tint);
}

void IVehicle$cxxbridge1$IVehicle_SetDirtLevel(::alt::IVehicle *ptr, ::std::uint8_t level) noexcept {
  void (*IVehicle_SetDirtLevel$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetDirtLevel;
  IVehicle_SetDirtLevel$(ptr, level);
}

void IVehicle$cxxbridge1$IVehicle_SetNeonActive(::alt::IVehicle *ptr, bool left, bool right, bool front, bool back) noexcept {
  void (*IVehicle_SetNeonActive$)(::alt::IVehicle *, bool, bool, bool, bool) = ::IVehicle::SetNeonActive;
  IVehicle_SetNeonActive$(ptr, left, right, front, back);
}

void IVehicle$cxxbridge1$IVehicle_SetNeonColor(::alt::IVehicle *ptr, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a) noexcept {
  void (*IVehicle_SetNeonColor$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetNeonColor;
  IVehicle_SetNeonColor$(ptr, color_r, color_g, color_b, color_a);
}

void IVehicle$cxxbridge1$IVehicle_SetLivery(::alt::IVehicle *ptr, ::std::uint8_t livery) noexcept {
  void (*IVehicle_SetLivery$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetLivery;
  IVehicle_SetLivery$(ptr, livery);
}

void IVehicle$cxxbridge1$IVehicle_SetRoofLivery(::alt::IVehicle *ptr, ::std::uint8_t roofLivery) noexcept {
  void (*IVehicle_SetRoofLivery$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetRoofLivery;
  IVehicle_SetRoofLivery$(ptr, roofLivery);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
void cxxbridge1$IVehicle_LoadAppearanceDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle *ptr, ::std::string *base64) noexcept {
  void (*IVehicle_LoadAppearanceDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle *, ::std::unique_ptr<::std::string>) = ::IVehicle_LoadAppearanceDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_LoadAppearanceDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(base64));
}
} // extern "C"

namespace IVehicle {
extern "C" {
void IVehicle$cxxbridge1$IVehicle_SetEngineOn(::alt::IVehicle *ptr, bool state) noexcept {
  void (*IVehicle_SetEngineOn$)(::alt::IVehicle *, bool) = ::IVehicle::SetEngineOn;
  IVehicle_SetEngineOn$(ptr, state);
}

void IVehicle$cxxbridge1$IVehicle_SetHeadlightColor(::alt::IVehicle *ptr, ::std::uint8_t color) noexcept {
  void (*IVehicle_SetHeadlightColor$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetHeadlightColor;
  IVehicle_SetHeadlightColor$(ptr, color);
}

void IVehicle$cxxbridge1$IVehicle_SetRadioStationIndex(::alt::IVehicle *ptr, ::std::uint32_t stationIndex) noexcept {
  void (*IVehicle_SetRadioStationIndex$)(::alt::IVehicle *, ::std::uint32_t) = ::IVehicle::SetRadioStationIndex;
  IVehicle_SetRadioStationIndex$(ptr, stationIndex);
}

void IVehicle$cxxbridge1$IVehicle_SetSirenActive(::alt::IVehicle *ptr, bool state) noexcept {
  void (*IVehicle_SetSirenActive$)(::alt::IVehicle *, bool) = ::IVehicle::SetSirenActive;
  IVehicle_SetSirenActive$(ptr, state);
}

void IVehicle$cxxbridge1$IVehicle_SetLockState(::alt::IVehicle *ptr, ::std::uint8_t state) noexcept {
  void (*IVehicle_SetLockState$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetLockState;
  IVehicle_SetLockState$(ptr, state);
}

void IVehicle$cxxbridge1$IVehicle_SetDoorState(::alt::IVehicle *ptr, ::std::uint8_t doorId, ::std::uint8_t state) noexcept {
  void (*IVehicle_SetDoorState$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetDoorState;
  IVehicle_SetDoorState$(ptr, doorId, state);
}

void IVehicle$cxxbridge1$IVehicle_SetWindowOpened(::alt::IVehicle *ptr, ::std::uint8_t windowId, bool state) noexcept {
  void (*IVehicle_SetWindowOpened$)(::alt::IVehicle *, ::std::uint8_t, bool) = ::IVehicle::SetWindowOpened;
  IVehicle_SetWindowOpened$(ptr, windowId, state);
}

void IVehicle$cxxbridge1$IVehicle_SetRoofState(::alt::IVehicle *ptr, ::std::uint8_t state) noexcept {
  void (*IVehicle_SetRoofState$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetRoofState;
  IVehicle_SetRoofState$(ptr, state);
}

void IVehicle$cxxbridge1$IVehicle_SetLightsMultiplier(::alt::IVehicle *ptr, float multiplier) noexcept {
  void (*IVehicle_SetLightsMultiplier$)(::alt::IVehicle *, float) = ::IVehicle::SetLightsMultiplier;
  IVehicle_SetLightsMultiplier$(ptr, multiplier);
}

void IVehicle$cxxbridge1$IVehicle_SetEngineHealth(::alt::IVehicle *ptr, ::std::int32_t health) noexcept {
  void (*IVehicle_SetEngineHealth$)(::alt::IVehicle *, ::std::int32_t) = ::IVehicle::SetEngineHealth;
  IVehicle_SetEngineHealth$(ptr, health);
}

void IVehicle$cxxbridge1$IVehicle_SetPetrolTankHealth(::alt::IVehicle *ptr, ::std::int32_t health) noexcept {
  void (*IVehicle_SetPetrolTankHealth$)(::alt::IVehicle *, ::std::int32_t) = ::IVehicle::SetPetrolTankHealth;
  IVehicle_SetPetrolTankHealth$(ptr, health);
}

void IVehicle$cxxbridge1$IVehicle_SetWheelBurst(::alt::IVehicle *ptr, ::std::uint8_t wheelId, bool state) noexcept {
  void (*IVehicle_SetWheelBurst$)(::alt::IVehicle *, ::std::uint8_t, bool) = ::IVehicle::SetWheelBurst;
  IVehicle_SetWheelBurst$(ptr, wheelId, state);
}

void IVehicle$cxxbridge1$IVehicle_SetWheelHasTire(::alt::IVehicle *ptr, ::std::uint8_t wheelId, bool state) noexcept {
  void (*IVehicle_SetWheelHasTire$)(::alt::IVehicle *, ::std::uint8_t, bool) = ::IVehicle::SetWheelHasTire;
  IVehicle_SetWheelHasTire$(ptr, wheelId, state);
}

void IVehicle$cxxbridge1$IVehicle_SetWheelDetached(::alt::IVehicle *ptr, ::std::uint8_t wheelId, bool state) noexcept {
  void (*IVehicle_SetWheelDetached$)(::alt::IVehicle *, ::std::uint8_t, bool) = ::IVehicle::SetWheelDetached;
  IVehicle_SetWheelDetached$(ptr, wheelId, state);
}

void IVehicle$cxxbridge1$IVehicle_SetWheelOnFire(::alt::IVehicle *ptr, ::std::uint8_t wheelId, bool state) noexcept {
  void (*IVehicle_SetWheelOnFire$)(::alt::IVehicle *, ::std::uint8_t, bool) = ::IVehicle::SetWheelOnFire;
  IVehicle_SetWheelOnFire$(ptr, wheelId, state);
}

void IVehicle$cxxbridge1$IVehicle_SetWheelHealth(::alt::IVehicle *ptr, ::std::uint8_t wheelId, float health) noexcept {
  void (*IVehicle_SetWheelHealth$)(::alt::IVehicle *, ::std::uint8_t, float) = ::IVehicle::SetWheelHealth;
  IVehicle_SetWheelHealth$(ptr, wheelId, health);
}

void IVehicle$cxxbridge1$IVehicle_SetWheelFixed(::alt::IVehicle *ptr, ::std::uint8_t wheelId) noexcept {
  void (*IVehicle_SetWheelFixed$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetWheelFixed;
  IVehicle_SetWheelFixed$(ptr, wheelId);
}

void IVehicle$cxxbridge1$IVehicle_SetBodyHealth(::alt::IVehicle *ptr, ::std::uint32_t health) noexcept {
  void (*IVehicle_SetBodyHealth$)(::alt::IVehicle *, ::std::uint32_t) = ::IVehicle::SetBodyHealth;
  IVehicle_SetBodyHealth$(ptr, health);
}

void IVehicle$cxxbridge1$IVehicle_SetBodyAdditionalHealth(::alt::IVehicle *ptr, ::std::uint32_t health) noexcept {
  void (*IVehicle_SetBodyAdditionalHealth$)(::alt::IVehicle *, ::std::uint32_t) = ::IVehicle::SetBodyAdditionalHealth;
  IVehicle_SetBodyAdditionalHealth$(ptr, health);
}

void IVehicle$cxxbridge1$IVehicle_SetPartDamageLevel(::alt::IVehicle *ptr, ::std::uint8_t partId, ::std::uint8_t damage) noexcept {
  void (*IVehicle_SetPartDamageLevel$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetPartDamageLevel;
  IVehicle_SetPartDamageLevel$(ptr, partId, damage);
}

void IVehicle$cxxbridge1$IVehicle_SetPartBulletHoles(::alt::IVehicle *ptr, ::std::uint8_t partId, ::std::uint8_t shootsCount) noexcept {
  void (*IVehicle_SetPartBulletHoles$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetPartBulletHoles;
  IVehicle_SetPartBulletHoles$(ptr, partId, shootsCount);
}

void IVehicle$cxxbridge1$IVehicle_SetLightDamaged(::alt::IVehicle *ptr, ::std::uint8_t lightId, bool isDamaged) noexcept {
  void (*IVehicle_SetLightDamaged$)(::alt::IVehicle *, ::std::uint8_t, bool) = ::IVehicle::SetLightDamaged;
  IVehicle_SetLightDamaged$(ptr, lightId, isDamaged);
}

void IVehicle$cxxbridge1$IVehicle_SetWindowDamaged(::alt::IVehicle *ptr, ::std::uint8_t windowId, bool isDamaged) noexcept {
  void (*IVehicle_SetWindowDamaged$)(::alt::IVehicle *, ::std::uint8_t, bool) = ::IVehicle::SetWindowDamaged;
  IVehicle_SetWindowDamaged$(ptr, windowId, isDamaged);
}

void IVehicle$cxxbridge1$IVehicle_SetSpecialLightDamaged(::alt::IVehicle *ptr, ::std::uint8_t specialLightId, bool isDamaged) noexcept {
  void (*IVehicle_SetSpecialLightDamaged$)(::alt::IVehicle *, ::std::uint8_t, bool) = ::IVehicle::SetSpecialLightDamaged;
  IVehicle_SetSpecialLightDamaged$(ptr, specialLightId, isDamaged);
}

void IVehicle$cxxbridge1$IVehicle_SetArmoredWindowHealth(::alt::IVehicle *ptr, ::std::uint8_t windowId, float health) noexcept {
  void (*IVehicle_SetArmoredWindowHealth$)(::alt::IVehicle *, ::std::uint8_t, float) = ::IVehicle::SetArmoredWindowHealth;
  IVehicle_SetArmoredWindowHealth$(ptr, windowId, health);
}

void IVehicle$cxxbridge1$IVehicle_SetArmoredWindowShootCount(::alt::IVehicle *ptr, ::std::uint8_t windowId, ::std::uint8_t count) noexcept {
  void (*IVehicle_SetArmoredWindowShootCount$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetArmoredWindowShootCount;
  IVehicle_SetArmoredWindowShootCount$(ptr, windowId, count);
}

void IVehicle$cxxbridge1$IVehicle_SetBumperDamageLevel(::alt::IVehicle *ptr, ::std::uint8_t bumperId, ::std::uint8_t damageLevel) noexcept {
  void (*IVehicle_SetBumperDamageLevel$)(::alt::IVehicle *, ::std::uint8_t, ::std::uint8_t) = ::IVehicle::SetBumperDamageLevel;
  IVehicle_SetBumperDamageLevel$(ptr, bumperId, damageLevel);
}

void IVehicle$cxxbridge1$IVehicle_SetManualEngineControl(::alt::IVehicle *ptr, bool state) noexcept {
  void (*IVehicle_SetManualEngineControl$)(::alt::IVehicle *, bool) = ::IVehicle::SetManualEngineControl;
  IVehicle_SetManualEngineControl$(ptr, state);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
void cxxbridge1$IVehicle_LoadDamageDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle *ptr, ::std::string *base64) noexcept {
  void (*IVehicle_LoadDamageDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle *, ::std::unique_ptr<::std::string>) = ::IVehicle_LoadDamageDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_LoadDamageDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(base64));
}

void cxxbridge1$IVehicle_LoadScriptDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle *ptr, ::std::string *base64) noexcept {
  void (*IVehicle_LoadScriptDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle *, ::std::unique_ptr<::std::string>) = ::IVehicle_LoadScriptDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_LoadScriptDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(base64));
}

void cxxbridge1$IVehicle_LoadGameStateFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle *ptr, ::std::string *base64) noexcept {
  void (*IVehicle_LoadGameStateFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle *, ::std::unique_ptr<::std::string>) = ::IVehicle_LoadGameStateFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_LoadGameStateFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(base64));
}

void cxxbridge1$IVehicle_LoadHealthDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle *ptr, ::std::string *base64) noexcept {
  void (*IVehicle_LoadHealthDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle *, ::std::unique_ptr<::std::string>) = ::IVehicle_LoadHealthDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_LoadHealthDataFromBase64_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(base64));
}
} // extern "C"

namespace IVehicle {
extern "C" {
::alt::IVehicle *IVehicle$cxxbridge1$IVehicle_GetAttached(::alt::IVehicle const *ptr) noexcept {
  ::alt::IVehicle *(*IVehicle_GetAttached$)(::alt::IVehicle const *) = ::IVehicle::GetAttached;
  return IVehicle_GetAttached$(ptr);
}

::alt::IVehicle *IVehicle$cxxbridge1$IVehicle_GetAttachedTo(::alt::IVehicle const *ptr) noexcept {
  ::alt::IVehicle *(*IVehicle_GetAttachedTo$)(::alt::IVehicle const *) = ::IVehicle::GetAttachedTo;
  return IVehicle_GetAttachedTo$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsDriftMode(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsDriftMode$)(::alt::IVehicle const *) = ::IVehicle::IsDriftMode;
  return IVehicle_IsDriftMode$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetDriftMode(::alt::IVehicle *ptr, bool state) noexcept {
  void (*IVehicle_SetDriftMode$)(::alt::IVehicle *, bool) = ::IVehicle::SetDriftMode;
  IVehicle_SetDriftMode$(ptr, state);
}

bool IVehicle$cxxbridge1$IVehicle_IsHornActive(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsHornActive$)(::alt::IVehicle const *) = ::IVehicle::IsHornActive;
  return IVehicle_IsHornActive$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsTrainMissionTrain(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsTrainMissionTrain$)(::alt::IVehicle const *) = ::IVehicle::IsTrainMissionTrain;
  return IVehicle_IsTrainMissionTrain$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainMissionTrain(::alt::IVehicle *ptr, bool value) noexcept {
  void (*IVehicle_SetTrainMissionTrain$)(::alt::IVehicle *, bool) = ::IVehicle::SetTrainMissionTrain;
  IVehicle_SetTrainMissionTrain$(ptr, value);
}

::std::int8_t IVehicle$cxxbridge1$IVehicle_GetTrainTrackId(::alt::IVehicle const *ptr) noexcept {
  ::std::int8_t (*IVehicle_GetTrainTrackId$)(::alt::IVehicle const *) = ::IVehicle::GetTrainTrackId;
  return IVehicle_GetTrainTrackId$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainTrackId(::alt::IVehicle *ptr, ::std::int8_t trackId) noexcept {
  void (*IVehicle_SetTrainTrackId$)(::alt::IVehicle *, ::std::int8_t) = ::IVehicle::SetTrainTrackId;
  IVehicle_SetTrainTrackId$(ptr, trackId);
}

::alt::IVehicle *IVehicle$cxxbridge1$IVehicle_GetTrainEngineId(::alt::IVehicle const *ptr) noexcept {
  ::alt::IVehicle *(*IVehicle_GetTrainEngineId$)(::alt::IVehicle const *) = ::IVehicle::GetTrainEngineId;
  return IVehicle_GetTrainEngineId$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainEngineId(::alt::IVehicle *ptr, ::alt::IVehicle *vehicle) noexcept {
  void (*IVehicle_SetTrainEngineId$)(::alt::IVehicle *, ::alt::IVehicle *) = ::IVehicle::SetTrainEngineId;
  IVehicle_SetTrainEngineId$(ptr, vehicle);
}

::std::int8_t IVehicle$cxxbridge1$IVehicle_GetTrainConfigIndex(::alt::IVehicle const *ptr) noexcept {
  ::std::int8_t (*IVehicle_GetTrainConfigIndex$)(::alt::IVehicle const *) = ::IVehicle::GetTrainConfigIndex;
  return IVehicle_GetTrainConfigIndex$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainConfigIndex(::alt::IVehicle *ptr, ::std::int8_t trainConfigIndex) noexcept {
  void (*IVehicle_SetTrainConfigIndex$)(::alt::IVehicle *, ::std::int8_t) = ::IVehicle::SetTrainConfigIndex;
  IVehicle_SetTrainConfigIndex$(ptr, trainConfigIndex);
}

float IVehicle$cxxbridge1$IVehicle_GetTrainDistanceFromEngine(::alt::IVehicle const *ptr) noexcept {
  float (*IVehicle_GetTrainDistanceFromEngine$)(::alt::IVehicle const *) = ::IVehicle::GetTrainDistanceFromEngine;
  return IVehicle_GetTrainDistanceFromEngine$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainDistanceFromEngine(::alt::IVehicle *ptr, float distanceFromEngine) noexcept {
  void (*IVehicle_SetTrainDistanceFromEngine$)(::alt::IVehicle *, float) = ::IVehicle::SetTrainDistanceFromEngine;
  IVehicle_SetTrainDistanceFromEngine$(ptr, distanceFromEngine);
}

bool IVehicle$cxxbridge1$IVehicle_IsTrainEngine(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsTrainEngine$)(::alt::IVehicle const *) = ::IVehicle::IsTrainEngine;
  return IVehicle_IsTrainEngine$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainIsEngine(::alt::IVehicle *ptr, bool isEngine) noexcept {
  void (*IVehicle_SetTrainIsEngine$)(::alt::IVehicle *, bool) = ::IVehicle::SetTrainIsEngine;
  IVehicle_SetTrainIsEngine$(ptr, isEngine);
}

bool IVehicle$cxxbridge1$IVehicle_IsTrainCaboose(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsTrainCaboose$)(::alt::IVehicle const *) = ::IVehicle::IsTrainCaboose;
  return IVehicle_IsTrainCaboose$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainIsCaboose(::alt::IVehicle *ptr, bool isCaboose) noexcept {
  void (*IVehicle_SetTrainIsCaboose$)(::alt::IVehicle *, bool) = ::IVehicle::SetTrainIsCaboose;
  IVehicle_SetTrainIsCaboose$(ptr, isCaboose);
}

bool IVehicle$cxxbridge1$IVehicle_GetTrainDirection(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_GetTrainDirection$)(::alt::IVehicle const *) = ::IVehicle::GetTrainDirection;
  return IVehicle_GetTrainDirection$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainDirection(::alt::IVehicle *ptr, bool direction) noexcept {
  void (*IVehicle_SetTrainDirection$)(::alt::IVehicle *, bool) = ::IVehicle::SetTrainDirection;
  IVehicle_SetTrainDirection$(ptr, direction);
}

bool IVehicle$cxxbridge1$IVehicle_HasTrainPassengerCarriages(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_HasTrainPassengerCarriages$)(::alt::IVehicle const *) = ::IVehicle::HasTrainPassengerCarriages;
  return IVehicle_HasTrainPassengerCarriages$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainHasPassengerCarriages(::alt::IVehicle *ptr, bool hasPassengerCarriages) noexcept {
  void (*IVehicle_SetTrainHasPassengerCarriages$)(::alt::IVehicle *, bool) = ::IVehicle::SetTrainHasPassengerCarriages;
  IVehicle_SetTrainHasPassengerCarriages$(ptr, hasPassengerCarriages);
}

bool IVehicle$cxxbridge1$IVehicle_GetTrainRenderDerailed(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_GetTrainRenderDerailed$)(::alt::IVehicle const *) = ::IVehicle::GetTrainRenderDerailed;
  return IVehicle_GetTrainRenderDerailed$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainRenderDerailed(::alt::IVehicle *ptr, bool renderDerailed) noexcept {
  void (*IVehicle_SetTrainRenderDerailed$)(::alt::IVehicle *, bool) = ::IVehicle::SetTrainRenderDerailed;
  IVehicle_SetTrainRenderDerailed$(ptr, renderDerailed);
}

bool IVehicle$cxxbridge1$IVehicle_GetTrainForceDoorsOpen(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_GetTrainForceDoorsOpen$)(::alt::IVehicle const *) = ::IVehicle::GetTrainForceDoorsOpen;
  return IVehicle_GetTrainForceDoorsOpen$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainForceDoorsOpen(::alt::IVehicle *ptr, bool forceDoorsOpen) noexcept {
  void (*IVehicle_SetTrainForceDoorsOpen$)(::alt::IVehicle *, bool) = ::IVehicle::SetTrainForceDoorsOpen;
  IVehicle_SetTrainForceDoorsOpen$(ptr, forceDoorsOpen);
}

float IVehicle$cxxbridge1$IVehicle_GetTrainCruiseSpeed(::alt::IVehicle const *ptr) noexcept {
  float (*IVehicle_GetTrainCruiseSpeed$)(::alt::IVehicle const *) = ::IVehicle::GetTrainCruiseSpeed;
  return IVehicle_GetTrainCruiseSpeed$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainCruiseSpeed(::alt::IVehicle *ptr, float cruiseSpeed) noexcept {
  void (*IVehicle_SetTrainCruiseSpeed$)(::alt::IVehicle *, float) = ::IVehicle::SetTrainCruiseSpeed;
  IVehicle_SetTrainCruiseSpeed$(ptr, cruiseSpeed);
}

::std::int8_t IVehicle$cxxbridge1$IVehicle_GetTrainCarriageConfigIndex(::alt::IVehicle const *ptr) noexcept {
  ::std::int8_t (*IVehicle_GetTrainCarriageConfigIndex$)(::alt::IVehicle const *) = ::IVehicle::GetTrainCarriageConfigIndex;
  return IVehicle_GetTrainCarriageConfigIndex$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainCarriageConfigIndex(::alt::IVehicle *ptr, ::std::int8_t carriageConfigIndex) noexcept {
  void (*IVehicle_SetTrainCarriageConfigIndex$)(::alt::IVehicle *, ::std::int8_t) = ::IVehicle::SetTrainCarriageConfigIndex;
  IVehicle_SetTrainCarriageConfigIndex$(ptr, carriageConfigIndex);
}

::alt::IVehicle *IVehicle$cxxbridge1$IVehicle_GetTrainLinkedToBackwardId(::alt::IVehicle const *ptr) noexcept {
  ::alt::IVehicle *(*IVehicle_GetTrainLinkedToBackwardId$)(::alt::IVehicle const *) = ::IVehicle::GetTrainLinkedToBackwardId;
  return IVehicle_GetTrainLinkedToBackwardId$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainLinkedToBackwardId(::alt::IVehicle *ptr, ::alt::IVehicle *vehicle) noexcept {
  void (*IVehicle_SetTrainLinkedToBackwardId$)(::alt::IVehicle *, ::alt::IVehicle *) = ::IVehicle::SetTrainLinkedToBackwardId;
  IVehicle_SetTrainLinkedToBackwardId$(ptr, vehicle);
}

::alt::IVehicle *IVehicle$cxxbridge1$IVehicle_GetTrainLinkedToForwardId(::alt::IVehicle const *ptr) noexcept {
  ::alt::IVehicle *(*IVehicle_GetTrainLinkedToForwardId$)(::alt::IVehicle const *) = ::IVehicle::GetTrainLinkedToForwardId;
  return IVehicle_GetTrainLinkedToForwardId$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainLinkedToForwardId(::alt::IVehicle *ptr, ::alt::IVehicle *vehicle) noexcept {
  void (*IVehicle_SetTrainLinkedToForwardId$)(::alt::IVehicle *, ::alt::IVehicle *) = ::IVehicle::SetTrainLinkedToForwardId;
  IVehicle_SetTrainLinkedToForwardId$(ptr, vehicle);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainUnk1(::alt::IVehicle *ptr, bool unk1) noexcept {
  void (*IVehicle_SetTrainUnk1$)(::alt::IVehicle *, bool) = ::IVehicle::SetTrainUnk1;
  IVehicle_SetTrainUnk1$(ptr, unk1);
}

bool IVehicle$cxxbridge1$IVehicle_GetTrainUnk1(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_GetTrainUnk1$)(::alt::IVehicle const *) = ::IVehicle::GetTrainUnk1;
  return IVehicle_GetTrainUnk1$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainUnk2(::alt::IVehicle *ptr, bool unk2) noexcept {
  void (*IVehicle_SetTrainUnk2$)(::alt::IVehicle *, bool) = ::IVehicle::SetTrainUnk2;
  IVehicle_SetTrainUnk2$(ptr, unk2);
}

bool IVehicle$cxxbridge1$IVehicle_GetTrainUnk2(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_GetTrainUnk2$)(::alt::IVehicle const *) = ::IVehicle::GetTrainUnk2;
  return IVehicle_GetTrainUnk2$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTrainUnk3(::alt::IVehicle *ptr, bool unk3) noexcept {
  void (*IVehicle_SetTrainUnk3$)(::alt::IVehicle *, bool) = ::IVehicle::SetTrainUnk3;
  IVehicle_SetTrainUnk3$(ptr, unk3);
}

bool IVehicle$cxxbridge1$IVehicle_GetTrainUnk3(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_GetTrainUnk3$)(::alt::IVehicle const *) = ::IVehicle::GetTrainUnk3;
  return IVehicle_GetTrainUnk3$(ptr);
}

bool IVehicle$cxxbridge1$IVehicle_IsBoatAnchorActive(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsBoatAnchorActive$)(::alt::IVehicle const *) = ::IVehicle::IsBoatAnchorActive;
  return IVehicle_IsBoatAnchorActive$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetBoatAnchorActive(::alt::IVehicle *ptr, bool state) noexcept {
  void (*IVehicle_SetBoatAnchorActive$)(::alt::IVehicle *, bool) = ::IVehicle::SetBoatAnchorActive;
  IVehicle_SetBoatAnchorActive$(ptr, state);
}

bool IVehicle$cxxbridge1$IVehicle_SetSearchLight(::alt::IVehicle *ptr, bool state, ::alt::IEntity *spottedEntity) noexcept {
  bool (*IVehicle_SetSearchLight$)(::alt::IVehicle *, bool, ::alt::IEntity *) = ::IVehicle::SetSearchLight;
  return IVehicle_SetSearchLight$(ptr, state, spottedEntity);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetLightState(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetLightState$)(::alt::IVehicle const *) = ::IVehicle::GetLightState;
  return IVehicle_GetLightState$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetLightState(::alt::IVehicle *ptr, ::std::uint8_t state) noexcept {
  void (*IVehicle_SetLightState$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetLightState;
  IVehicle_SetLightState$(ptr, state);
}

bool IVehicle$cxxbridge1$IVehicle_HasTimedExplosion(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_HasTimedExplosion$)(::alt::IVehicle const *) = ::IVehicle::HasTimedExplosion;
  return IVehicle_HasTimedExplosion$(ptr);
}

::alt::IPlayer *IVehicle$cxxbridge1$IVehicle_GetTimedExplosionCulprit(::alt::IVehicle const *ptr) noexcept {
  ::alt::IPlayer *(*IVehicle_GetTimedExplosionCulprit$)(::alt::IVehicle const *) = ::IVehicle::GetTimedExplosionCulprit;
  return IVehicle_GetTimedExplosionCulprit$(ptr);
}

::std::uint32_t IVehicle$cxxbridge1$IVehicle_GetTimedExplosionTime(::alt::IVehicle const *ptr) noexcept {
  ::std::uint32_t (*IVehicle_GetTimedExplosionTime$)(::alt::IVehicle const *) = ::IVehicle::GetTimedExplosionTime;
  return IVehicle_GetTimedExplosionTime$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetTimedExplosion(::alt::IVehicle *ptr, bool state, ::alt::IPlayer *culprit, ::std::uint32_t time) noexcept {
  void (*IVehicle_SetTimedExplosion$)(::alt::IVehicle *, bool, ::alt::IPlayer *, ::std::uint32_t) = ::IVehicle::SetTimedExplosion;
  IVehicle_SetTimedExplosion$(ptr, state, culprit, time);
}

bool IVehicle$cxxbridge1$IVehicle_IsTowingDisabled(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_IsTowingDisabled$)(::alt::IVehicle const *) = ::IVehicle::IsTowingDisabled;
  return IVehicle_IsTowingDisabled$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetDisableTowing(::alt::IVehicle *ptr, bool state) noexcept {
  void (*IVehicle_SetDisableTowing$)(::alt::IVehicle *, bool) = ::IVehicle::SetDisableTowing;
  IVehicle_SetDisableTowing$(ptr, state);
}

float IVehicle$cxxbridge1$IVehicle_GetRocketRefuelSpeed(::alt::IVehicle const *ptr) noexcept {
  float (*IVehicle_GetRocketRefuelSpeed$)(::alt::IVehicle const *) = ::IVehicle::GetRocketRefuelSpeed;
  return IVehicle_GetRocketRefuelSpeed$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetRocketRefuelSpeed(::alt::IVehicle *ptr, float rocketRefuelSpeed) noexcept {
  void (*IVehicle_SetRocketRefuelSpeed$)(::alt::IVehicle *, float) = ::IVehicle::SetRocketRefuelSpeed;
  IVehicle_SetRocketRefuelSpeed$(ptr, rocketRefuelSpeed);
}

::std::uint32_t IVehicle$cxxbridge1$IVehicle_GetCounterMeasureCount(::alt::IVehicle const *ptr) noexcept {
  ::std::uint32_t (*IVehicle_GetCounterMeasureCount$)(::alt::IVehicle const *) = ::IVehicle::GetCounterMeasureCount;
  return IVehicle_GetCounterMeasureCount$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetCounterMeasureCount(::alt::IVehicle *ptr, ::std::uint32_t counterMeasureCount) noexcept {
  void (*IVehicle_SetCounterMeasureCount$)(::alt::IVehicle *, ::std::uint32_t) = ::IVehicle::SetCounterMeasureCount;
  IVehicle_SetCounterMeasureCount$(ptr, counterMeasureCount);
}

float IVehicle$cxxbridge1$IVehicle_GetScriptMaxSpeed(::alt::IVehicle const *ptr) noexcept {
  float (*IVehicle_GetScriptMaxSpeed$)(::alt::IVehicle const *) = ::IVehicle::GetScriptMaxSpeed;
  return IVehicle_GetScriptMaxSpeed$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetScriptMaxSpeed(::alt::IVehicle *ptr, float scriptMaxSpeed) noexcept {
  void (*IVehicle_SetScriptMaxSpeed$)(::alt::IVehicle *, float) = ::IVehicle::SetScriptMaxSpeed;
  IVehicle_SetScriptMaxSpeed$(ptr, scriptMaxSpeed);
}

::std::int32_t IVehicle$cxxbridge1$IVehicle_GetWeaponCapacity(::alt::IVehicle const *ptr, ::std::uint8_t index) noexcept {
  ::std::int32_t (*IVehicle_GetWeaponCapacity$)(::alt::IVehicle const *, ::std::uint8_t) = ::IVehicle::GetWeaponCapacity;
  return IVehicle_GetWeaponCapacity$(ptr, index);
}

void IVehicle$cxxbridge1$IVehicle_SetWeaponCapacity(::alt::IVehicle *ptr, ::std::uint8_t index, ::std::int32_t state) noexcept {
  void (*IVehicle_SetWeaponCapacity$)(::alt::IVehicle *, ::std::uint8_t, ::std::int32_t) = ::IVehicle::SetWeaponCapacity;
  IVehicle_SetWeaponCapacity$(ptr, index, state);
}

bool IVehicle$cxxbridge1$IVehicle_GetHybridExtraActive(::alt::IVehicle const *ptr) noexcept {
  bool (*IVehicle_GetHybridExtraActive$)(::alt::IVehicle const *) = ::IVehicle::GetHybridExtraActive;
  return IVehicle_GetHybridExtraActive$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetHybridExtraActive(::alt::IVehicle *ptr, bool state) noexcept {
  void (*IVehicle_SetHybridExtraActive$)(::alt::IVehicle *, bool) = ::IVehicle::SetHybridExtraActive;
  IVehicle_SetHybridExtraActive$(ptr, state);
}

::std::uint8_t IVehicle$cxxbridge1$IVehicle_GetHybridExtraState(::alt::IVehicle const *ptr) noexcept {
  ::std::uint8_t (*IVehicle_GetHybridExtraState$)(::alt::IVehicle const *) = ::IVehicle::GetHybridExtraState;
  return IVehicle_GetHybridExtraState$(ptr);
}

void IVehicle$cxxbridge1$IVehicle_SetHybridExtraState(::alt::IVehicle *ptr, ::std::uint8_t state) noexcept {
  void (*IVehicle_SetHybridExtraState$)(::alt::IVehicle *, ::std::uint8_t) = ::IVehicle::SetHybridExtraState;
  IVehicle_SetHybridExtraState$(ptr, state);
}
} // extern "C"
} // namespace IVehicle

extern "C" {
void cxxbridge1$IVehicle_GetQuaternion_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVehicle const *ptr, ::alt::Quaternion *placement_return_type) noexcept {
  void (*IVehicle_GetQuaternion_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVehicle const *, ::alt::Quaternion *) = ::IVehicle_GetQuaternion_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVehicle_GetQuaternion_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IVehicle {
extern "C" {
void IVehicle$cxxbridge1$IVehicle_SetQuaternion(::alt::IVehicle *ptr, float quaternion_x, float quaternion_y, float quaternion_z, float quaternion_w) noexcept {
  void (*IVehicle_SetQuaternion$)(::alt::IVehicle *, float, float, float, float) = ::IVehicle::SetQuaternion;
  IVehicle_SetQuaternion$(ptr, quaternion_x, quaternion_y, quaternion_z, quaternion_w);
}

float IVehicle$cxxbridge1$IVehicle_GetAccelerationLevel(::alt::IVehicle const *ptr) noexcept {
  float (*IVehicle_GetAccelerationLevel$)(::alt::IVehicle const *) = ::IVehicle::GetAccelerationLevel;
  return IVehicle_GetAccelerationLevel$(ptr);
}

float IVehicle$cxxbridge1$IVehicle_GetBrakeLevel(::alt::IVehicle const *ptr) noexcept {
  float (*IVehicle_GetBrakeLevel$)(::alt::IVehicle const *) = ::IVehicle::GetBrakeLevel;
  return IVehicle_GetBrakeLevel$(ptr);
}
} // extern "C"
} // namespace IVehicle

namespace IPed {
extern "C" {
::std::uint16_t IPed$cxxbridge1$IPed_GetHealth(::alt::IPed const *ptr) noexcept {
  ::std::uint16_t (*IPed_GetHealth$)(::alt::IPed const *) = ::IPed::GetHealth;
  return IPed_GetHealth$(ptr);
}

::std::uint16_t IPed$cxxbridge1$IPed_GetMaxHealth(::alt::IPed const *ptr) noexcept {
  ::std::uint16_t (*IPed_GetMaxHealth$)(::alt::IPed const *) = ::IPed::GetMaxHealth;
  return IPed_GetMaxHealth$(ptr);
}

::std::uint16_t IPed$cxxbridge1$IPed_GetArmour(::alt::IPed const *ptr) noexcept {
  ::std::uint16_t (*IPed_GetArmour$)(::alt::IPed const *) = ::IPed::GetArmour;
  return IPed_GetArmour$(ptr);
}

::std::uint32_t IPed$cxxbridge1$IPed_GetCurrentWeapon(::alt::IPed const *ptr) noexcept {
  ::std::uint32_t (*IPed_GetCurrentWeapon$)(::alt::IPed const *) = ::IPed::GetCurrentWeapon;
  return IPed_GetCurrentWeapon$(ptr);
}

void IPed$cxxbridge1$IPed_SetHealth(::alt::IPed *ptr, ::std::uint16_t health) noexcept {
  void (*IPed_SetHealth$)(::alt::IPed *, ::std::uint16_t) = ::IPed::SetHealth;
  IPed_SetHealth$(ptr, health);
}

void IPed$cxxbridge1$IPed_SetMaxHealth(::alt::IPed *ptr, ::std::uint16_t health) noexcept {
  void (*IPed_SetMaxHealth$)(::alt::IPed *, ::std::uint16_t) = ::IPed::SetMaxHealth;
  IPed_SetMaxHealth$(ptr, health);
}

void IPed$cxxbridge1$IPed_SetArmour(::alt::IPed *ptr, ::std::uint16_t armor) noexcept {
  void (*IPed_SetArmour$)(::alt::IPed *, ::std::uint16_t) = ::IPed::SetArmour;
  IPed_SetArmour$(ptr, armor);
}

void IPed$cxxbridge1$IPed_SetCurrentWeapon(::alt::IPed *ptr, ::std::uint32_t weapon) noexcept {
  void (*IPed_SetCurrentWeapon$)(::alt::IPed *, ::std::uint32_t) = ::IPed::SetCurrentWeapon;
  IPed_SetCurrentWeapon$(ptr, weapon);
}
} // extern "C"
} // namespace IPed

namespace IObject {
extern "C" {
::std::uint8_t IObject$cxxbridge1$IObject_GetAlpha(::alt::IObject const *ptr) noexcept {
  ::std::uint8_t (*IObject_GetAlpha$)(::alt::IObject const *) = ::IObject::GetAlpha;
  return IObject_GetAlpha$(ptr);
}

::std::uint8_t IObject$cxxbridge1$IObject_GetTextureVariation(::alt::IObject const *ptr) noexcept {
  ::std::uint8_t (*IObject_GetTextureVariation$)(::alt::IObject const *) = ::IObject::GetTextureVariation;
  return IObject_GetTextureVariation$(ptr);
}

::std::uint16_t IObject$cxxbridge1$IObject_GetLodDistance(::alt::IObject const *ptr) noexcept {
  ::std::uint16_t (*IObject_GetLodDistance$)(::alt::IObject const *) = ::IObject::GetLodDistance;
  return IObject_GetLodDistance$(ptr);
}

void IObject$cxxbridge1$IObject_ActivatePhysics(::alt::IObject *ptr) noexcept {
  void (*IObject_ActivatePhysics$)(::alt::IObject *) = ::IObject::ActivatePhysics;
  IObject_ActivatePhysics$(ptr);
}

void IObject$cxxbridge1$IObject_PlaceOnGroundProperly(::alt::IObject *ptr) noexcept {
  void (*IObject_PlaceOnGroundProperly$)(::alt::IObject *) = ::IObject::PlaceOnGroundProperly;
  IObject_PlaceOnGroundProperly$(ptr);
}

void IObject$cxxbridge1$IObject_SetAlpha(::alt::IObject *ptr, ::std::uint8_t alpha) noexcept {
  void (*IObject_SetAlpha$)(::alt::IObject *, ::std::uint8_t) = ::IObject::SetAlpha;
  IObject_SetAlpha$(ptr, alpha);
}

void IObject$cxxbridge1$IObject_SetTextureVariation(::alt::IObject *ptr, ::std::uint8_t textureVariation) noexcept {
  void (*IObject_SetTextureVariation$)(::alt::IObject *, ::std::uint8_t) = ::IObject::SetTextureVariation;
  IObject_SetTextureVariation$(ptr, textureVariation);
}

void IObject$cxxbridge1$IObject_SetLodDistance(::alt::IObject *ptr, ::std::uint16_t lodDistance) noexcept {
  void (*IObject_SetLodDistance$)(::alt::IObject *, ::std::uint16_t) = ::IObject::SetLodDistance;
  IObject_SetLodDistance$(ptr, lodDistance);
}
} // extern "C"
} // namespace IObject

namespace IColShape {
extern "C" {
::std::uint8_t IColShape$cxxbridge1$IColShape_GetColshapeType(::alt::IColShape const *ptr) noexcept {
  ::std::uint8_t (*IColShape_GetColshapeType$)(::alt::IColShape const *) = ::IColShape::GetColshapeType;
  return IColShape_GetColshapeType$(ptr);
}

bool IColShape$cxxbridge1$IColShape_IsEntityIn(::alt::IColShape const *ptr, ::alt::IEntity *ent) noexcept {
  bool (*IColShape_IsEntityIn$)(::alt::IColShape const *, ::alt::IEntity *) = ::IColShape::IsEntityIn;
  return IColShape_IsEntityIn$(ptr, ent);
}

bool IColShape$cxxbridge1$IColShape_IsEntityIdIn(::alt::IColShape const *ptr, ::std::uint16_t id) noexcept {
  bool (*IColShape_IsEntityIdIn$)(::alt::IColShape const *, ::std::uint16_t) = ::IColShape::IsEntityIdIn;
  return IColShape_IsEntityIdIn$(ptr, id);
}

bool IColShape$cxxbridge1$IColShape_IsPointIn(::alt::IColShape const *ptr, float p_x, float p_y, float p_z) noexcept {
  bool (*IColShape_IsPointIn$)(::alt::IColShape const *, float, float, float) = ::IColShape::IsPointIn;
  return IColShape_IsPointIn$(ptr, p_x, p_y, p_z);
}

void IColShape$cxxbridge1$IColShape_SetPlayersOnly(::alt::IColShape *ptr, bool state) noexcept {
  void (*IColShape_SetPlayersOnly$)(::alt::IColShape *, bool) = ::IColShape::SetPlayersOnly;
  IColShape_SetPlayersOnly$(ptr, state);
}

bool IColShape$cxxbridge1$IColShape_IsPlayersOnly(::alt::IColShape const *ptr) noexcept {
  bool (*IColShape_IsPlayersOnly$)(::alt::IColShape const *) = ::IColShape::IsPlayersOnly;
  return IColShape_IsPlayersOnly$(ptr);
}
} // extern "C"
} // namespace IColShape

namespace IBlip {
extern "C" {
void IBlip$cxxbridge1$IBlip_SetVisible(::alt::IBlip *ptr, bool toggle) noexcept {
  void (*IBlip_SetVisible$)(::alt::IBlip *, bool) = ::IBlip::SetVisible;
  IBlip_SetVisible$(ptr, toggle);
}

bool IBlip$cxxbridge1$IBlip_IsVisible(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_IsVisible$)(::alt::IBlip const *) = ::IBlip::IsVisible;
  return IBlip_IsVisible$(ptr);
}

bool IBlip$cxxbridge1$IBlip_IsGlobal(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_IsGlobal$)(::alt::IBlip const *) = ::IBlip::IsGlobal;
  return IBlip_IsGlobal$(ptr);
}

bool IBlip$cxxbridge1$IBlip_IsAttached(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_IsAttached$)(::alt::IBlip const *) = ::IBlip::IsAttached;
  return IBlip_IsAttached$(ptr);
}

::alt::IEntity *IBlip$cxxbridge1$IBlip_AttachedTo(::alt::IBlip const *ptr) noexcept {
  ::alt::IEntity *(*IBlip_AttachedTo$)(::alt::IBlip const *) = ::IBlip::AttachedTo;
  return IBlip_AttachedTo$(ptr);
}

void IBlip$cxxbridge1$IBlip_AttachTo(::alt::IBlip *ptr, ::alt::IEntity *entity) noexcept {
  void (*IBlip_AttachTo$)(::alt::IBlip *, ::alt::IEntity *) = ::IBlip::AttachTo;
  IBlip_AttachTo$(ptr, entity);
}

::std::uint8_t IBlip$cxxbridge1$IBlip_GetBlipType(::alt::IBlip const *ptr) noexcept {
  ::std::uint8_t (*IBlip_GetBlipType$)(::alt::IBlip const *) = ::IBlip::GetBlipType;
  return IBlip_GetBlipType$(ptr);
}

void IBlip$cxxbridge1$IBlip_SetBlipType(::alt::IBlip *ptr, ::std::uint8_t blipType) noexcept {
  void (*IBlip_SetBlipType$)(::alt::IBlip *, ::std::uint8_t) = ::IBlip::SetBlipType;
  IBlip_SetBlipType$(ptr, blipType);
}
} // extern "C"
} // namespace IBlip

extern "C" {
void cxxbridge1$IBlip_GetScaleXY_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBlip const *ptr, ::Vector2Wrapper *placement_return_type) noexcept {
  void (*IBlip_GetScaleXY_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBlip const *, ::Vector2Wrapper *) = ::IBlip_GetScaleXY_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBlip_GetScaleXY_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IBlip {
extern "C" {
void IBlip$cxxbridge1$IBlip_SetScaleXY(::alt::IBlip *ptr, float scale_x, float scale_y) noexcept {
  void (*IBlip_SetScaleXY$)(::alt::IBlip *, float, float) = ::IBlip::SetScaleXY;
  IBlip_SetScaleXY$(ptr, scale_x, scale_y);
}

::std::uint32_t IBlip$cxxbridge1$IBlip_GetDisplay(::alt::IBlip const *ptr) noexcept {
  ::std::uint32_t (*IBlip_GetDisplay$)(::alt::IBlip const *) = ::IBlip::GetDisplay;
  return IBlip_GetDisplay$(ptr);
}

void IBlip$cxxbridge1$IBlip_SetDisplay(::alt::IBlip *ptr, ::std::uint32_t display) noexcept {
  void (*IBlip_SetDisplay$)(::alt::IBlip *, ::std::uint32_t) = ::IBlip::SetDisplay;
  IBlip_SetDisplay$(ptr, display);
}

::std::uint32_t IBlip$cxxbridge1$IBlip_GetSprite(::alt::IBlip const *ptr) noexcept {
  ::std::uint32_t (*IBlip_GetSprite$)(::alt::IBlip const *) = ::IBlip::GetSprite;
  return IBlip_GetSprite$(ptr);
}

::std::uint32_t IBlip$cxxbridge1$IBlip_GetColor(::alt::IBlip const *ptr) noexcept {
  ::std::uint32_t (*IBlip_GetColor$)(::alt::IBlip const *) = ::IBlip::GetColor;
  return IBlip_GetColor$(ptr);
}
} // extern "C"
} // namespace IBlip

extern "C" {
void cxxbridge1$IBlip_GetSecondaryColor_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBlip const *ptr, ::RGBAWrapper *placement_return_type) noexcept {
  void (*IBlip_GetSecondaryColor_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBlip const *, ::RGBAWrapper *) = ::IBlip_GetSecondaryColor_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBlip_GetSecondaryColor_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IBlip {
extern "C" {
::std::uint32_t IBlip$cxxbridge1$IBlip_GetAlpha(::alt::IBlip const *ptr) noexcept {
  ::std::uint32_t (*IBlip_GetAlpha$)(::alt::IBlip const *) = ::IBlip::GetAlpha;
  return IBlip_GetAlpha$(ptr);
}

void IBlip$cxxbridge1$IBlip_GetFlashTimer(::alt::IBlip const *ptr, ::c_int *return$) noexcept {
  ::c_int (*IBlip_GetFlashTimer$)(::alt::IBlip const *) = ::IBlip::GetFlashTimer;
  new (return$) ::c_int(IBlip_GetFlashTimer$(ptr));
}

void IBlip$cxxbridge1$IBlip_GetFlashInterval(::alt::IBlip const *ptr, ::c_int *return$) noexcept {
  ::c_int (*IBlip_GetFlashInterval$)(::alt::IBlip const *) = ::IBlip::GetFlashInterval;
  new (return$) ::c_int(IBlip_GetFlashInterval$(ptr));
}

bool IBlip$cxxbridge1$IBlip_GetAsFriendly(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetAsFriendly$)(::alt::IBlip const *) = ::IBlip::GetAsFriendly;
  return IBlip_GetAsFriendly$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetRoute(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetRoute$)(::alt::IBlip const *) = ::IBlip::GetRoute;
  return IBlip_GetRoute$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetBright(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetBright$)(::alt::IBlip const *) = ::IBlip::GetBright;
  return IBlip_GetBright$(ptr);
}

void IBlip$cxxbridge1$IBlip_GetNumber(::alt::IBlip const *ptr, ::c_int *return$) noexcept {
  ::c_int (*IBlip_GetNumber$)(::alt::IBlip const *) = ::IBlip::GetNumber;
  new (return$) ::c_int(IBlip_GetNumber$(ptr));
}

bool IBlip$cxxbridge1$IBlip_GetShowCone(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetShowCone$)(::alt::IBlip const *) = ::IBlip::GetShowCone;
  return IBlip_GetShowCone$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetFlashes(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetFlashes$)(::alt::IBlip const *) = ::IBlip::GetFlashes;
  return IBlip_GetFlashes$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetFlashesAlternate(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetFlashesAlternate$)(::alt::IBlip const *) = ::IBlip::GetFlashesAlternate;
  return IBlip_GetFlashesAlternate$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetAsShortRange(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetAsShortRange$)(::alt::IBlip const *) = ::IBlip::GetAsShortRange;
  return IBlip_GetAsShortRange$(ptr);
}

::std::uint32_t IBlip$cxxbridge1$IBlip_GetPriority(::alt::IBlip const *ptr) noexcept {
  ::std::uint32_t (*IBlip_GetPriority$)(::alt::IBlip const *) = ::IBlip::GetPriority;
  return IBlip_GetPriority$(ptr);
}

float IBlip$cxxbridge1$IBlip_GetRotation(::alt::IBlip const *ptr) noexcept {
  float (*IBlip_GetRotation$)(::alt::IBlip const *) = ::IBlip::GetRotation;
  return IBlip_GetRotation$(ptr);
}
} // extern "C"
} // namespace IBlip

extern "C" {
::std::string *cxxbridge1$IBlip_GetGxtName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBlip const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IBlip_GetGxtName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBlip const *) = ::IBlip_GetGxtName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IBlip_GetGxtName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$IBlip_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBlip const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IBlip_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBlip const *) = ::IBlip_GetName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IBlip_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$IBlip_GetRouteColor_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBlip const *ptr, ::RGBAWrapper *placement_return_type) noexcept {
  void (*IBlip_GetRouteColor_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBlip const *, ::RGBAWrapper *) = ::IBlip_GetRouteColor_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBlip_GetRouteColor_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IBlip {
extern "C" {
bool IBlip$cxxbridge1$IBlip_GetPulse(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetPulse$)(::alt::IBlip const *) = ::IBlip::GetPulse;
  return IBlip_GetPulse$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetAsMissionCreator(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetAsMissionCreator$)(::alt::IBlip const *) = ::IBlip::GetAsMissionCreator;
  return IBlip_GetAsMissionCreator$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetTickVisible(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetTickVisible$)(::alt::IBlip const *) = ::IBlip::GetTickVisible;
  return IBlip_GetTickVisible$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetHeadingIndicatorVisible(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetHeadingIndicatorVisible$)(::alt::IBlip const *) = ::IBlip::GetHeadingIndicatorVisible;
  return IBlip_GetHeadingIndicatorVisible$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetOutlineIndicatorVisible(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetOutlineIndicatorVisible$)(::alt::IBlip const *) = ::IBlip::GetOutlineIndicatorVisible;
  return IBlip_GetOutlineIndicatorVisible$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetFriendIndicatorVisible(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetFriendIndicatorVisible$)(::alt::IBlip const *) = ::IBlip::GetFriendIndicatorVisible;
  return IBlip_GetFriendIndicatorVisible$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetCrewIndicatorVisible(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetCrewIndicatorVisible$)(::alt::IBlip const *) = ::IBlip::GetCrewIndicatorVisible;
  return IBlip_GetCrewIndicatorVisible$(ptr);
}

::std::uint32_t IBlip$cxxbridge1$IBlip_GetCategory(::alt::IBlip const *ptr) noexcept {
  ::std::uint32_t (*IBlip_GetCategory$)(::alt::IBlip const *) = ::IBlip::GetCategory;
  return IBlip_GetCategory$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetAsHighDetail(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetAsHighDetail$)(::alt::IBlip const *) = ::IBlip::GetAsHighDetail;
  return IBlip_GetAsHighDetail$(ptr);
}

bool IBlip$cxxbridge1$IBlip_GetShrinked(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_GetShrinked$)(::alt::IBlip const *) = ::IBlip::GetShrinked;
  return IBlip_GetShrinked$(ptr);
}

void IBlip$cxxbridge1$IBlip_SetGlobal(::alt::IBlip *ptr, bool state) noexcept {
  void (*IBlip_SetGlobal$)(::alt::IBlip *, bool) = ::IBlip::SetGlobal;
  IBlip_SetGlobal$(ptr, state);
}

void IBlip$cxxbridge1$IBlip_AddTargetPlayer(::alt::IBlip *ptr, ::alt::IPlayer *player) noexcept {
  void (*IBlip_AddTargetPlayer$)(::alt::IBlip *, ::alt::IPlayer *) = ::IBlip::AddTargetPlayer;
  IBlip_AddTargetPlayer$(ptr, player);
}

void IBlip$cxxbridge1$IBlip_RemoveTargetPlayer(::alt::IBlip *ptr, ::alt::IPlayer *player) noexcept {
  void (*IBlip_RemoveTargetPlayer$)(::alt::IBlip *, ::alt::IPlayer *) = ::IBlip::RemoveTargetPlayer;
  IBlip_RemoveTargetPlayer$(ptr, player);
}
} // extern "C"
} // namespace IBlip

extern "C" {
::std::vector<::PlayerPtrWrapper> *cxxbridge1$IBlip_GetTargets_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBlip const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::PlayerPtrWrapper>> (*IBlip_GetTargets_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBlip const *) = ::IBlip_GetTargets_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IBlip_GetTargets_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IBlip {
extern "C" {
void IBlip$cxxbridge1$IBlip_SetSprite(::alt::IBlip *ptr, ::std::uint32_t sprite) noexcept {
  void (*IBlip_SetSprite$)(::alt::IBlip *, ::std::uint32_t) = ::IBlip::SetSprite;
  IBlip_SetSprite$(ptr, sprite);
}

void IBlip$cxxbridge1$IBlip_SetColor(::alt::IBlip *ptr, ::std::uint32_t color) noexcept {
  void (*IBlip_SetColor$)(::alt::IBlip *, ::std::uint32_t) = ::IBlip::SetColor;
  IBlip_SetColor$(ptr, color);
}

void IBlip$cxxbridge1$IBlip_SetRoute(::alt::IBlip *ptr, bool state) noexcept {
  void (*IBlip_SetRoute$)(::alt::IBlip *, bool) = ::IBlip::SetRoute;
  IBlip_SetRoute$(ptr, state);
}

void IBlip$cxxbridge1$IBlip_SetRouteColor(::alt::IBlip *ptr, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a) noexcept {
  void (*IBlip_SetRouteColor$)(::alt::IBlip *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::IBlip::SetRouteColor;
  IBlip_SetRouteColor$(ptr, color_r, color_g, color_b, color_a);
}

void IBlip$cxxbridge1$IBlip_SetSecondaryColor(::alt::IBlip *ptr, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a) noexcept {
  void (*IBlip_SetSecondaryColor$)(::alt::IBlip *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::IBlip::SetSecondaryColor;
  IBlip_SetSecondaryColor$(ptr, color_r, color_g, color_b, color_a);
}

void IBlip$cxxbridge1$IBlip_SetAlpha(::alt::IBlip *ptr, ::std::uint32_t alpha) noexcept {
  void (*IBlip_SetAlpha$)(::alt::IBlip *, ::std::uint32_t) = ::IBlip::SetAlpha;
  IBlip_SetAlpha$(ptr, alpha);
}

void IBlip$cxxbridge1$IBlip_SetFlashTimer(::alt::IBlip *ptr, ::c_int *timer) noexcept {
  void (*IBlip_SetFlashTimer$)(::alt::IBlip *, ::c_int) = ::IBlip::SetFlashTimer;
  IBlip_SetFlashTimer$(ptr, ::std::move(*timer));
}

void IBlip$cxxbridge1$IBlip_SetFlashInterval(::alt::IBlip *ptr, ::c_int *interval) noexcept {
  void (*IBlip_SetFlashInterval$)(::alt::IBlip *, ::c_int) = ::IBlip::SetFlashInterval;
  IBlip_SetFlashInterval$(ptr, ::std::move(*interval));
}

void IBlip$cxxbridge1$IBlip_SetAsFriendly(::alt::IBlip *ptr, bool friendly) noexcept {
  void (*IBlip_SetAsFriendly$)(::alt::IBlip *, bool) = ::IBlip::SetAsFriendly;
  IBlip_SetAsFriendly$(ptr, friendly);
}

void IBlip$cxxbridge1$IBlip_SetBright(::alt::IBlip *ptr, bool bright) noexcept {
  void (*IBlip_SetBright$)(::alt::IBlip *, bool) = ::IBlip::SetBright;
  IBlip_SetBright$(ptr, bright);
}

void IBlip$cxxbridge1$IBlip_SetNumber(::alt::IBlip *ptr, ::c_int *number) noexcept {
  void (*IBlip_SetNumber$)(::alt::IBlip *, ::c_int) = ::IBlip::SetNumber;
  IBlip_SetNumber$(ptr, ::std::move(*number));
}

void IBlip$cxxbridge1$IBlip_SetShowCone(::alt::IBlip *ptr, bool state) noexcept {
  void (*IBlip_SetShowCone$)(::alt::IBlip *, bool) = ::IBlip::SetShowCone;
  IBlip_SetShowCone$(ptr, state);
}

void IBlip$cxxbridge1$IBlip_SetFlashes(::alt::IBlip *ptr, bool state) noexcept {
  void (*IBlip_SetFlashes$)(::alt::IBlip *, bool) = ::IBlip::SetFlashes;
  IBlip_SetFlashes$(ptr, state);
}

void IBlip$cxxbridge1$IBlip_SetFlashesAlternate(::alt::IBlip *ptr, bool state) noexcept {
  void (*IBlip_SetFlashesAlternate$)(::alt::IBlip *, bool) = ::IBlip::SetFlashesAlternate;
  IBlip_SetFlashesAlternate$(ptr, state);
}

void IBlip$cxxbridge1$IBlip_SetAsShortRange(::alt::IBlip *ptr, bool state) noexcept {
  void (*IBlip_SetAsShortRange$)(::alt::IBlip *, bool) = ::IBlip::SetAsShortRange;
  IBlip_SetAsShortRange$(ptr, state);
}

void IBlip$cxxbridge1$IBlip_SetPriority(::alt::IBlip *ptr, ::std::uint32_t state) noexcept {
  void (*IBlip_SetPriority$)(::alt::IBlip *, ::std::uint32_t) = ::IBlip::SetPriority;
  IBlip_SetPriority$(ptr, state);
}

void IBlip$cxxbridge1$IBlip_SetRotation(::alt::IBlip *ptr, float rot) noexcept {
  void (*IBlip_SetRotation$)(::alt::IBlip *, float) = ::IBlip::SetRotation;
  IBlip_SetRotation$(ptr, rot);
}
} // extern "C"
} // namespace IBlip

extern "C" {
void cxxbridge1$IBlip_SetGxtName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBlip *ptr, ::std::string *name) noexcept {
  void (*IBlip_SetGxtName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBlip *, ::std::unique_ptr<::std::string>) = ::IBlip_SetGxtName_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBlip_SetGxtName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(name));
}

void cxxbridge1$IBlip_SetName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBlip *ptr, ::std::string *name) noexcept {
  void (*IBlip_SetName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBlip *, ::std::unique_ptr<::std::string>) = ::IBlip_SetName_autocxx_wrapper_0xd5d0abec981e3e3a;
  IBlip_SetName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(name));
}
} // extern "C"

namespace IBlip {
extern "C" {
void IBlip$cxxbridge1$IBlip_SetPulse(::alt::IBlip *ptr, bool val) noexcept {
  void (*IBlip_SetPulse$)(::alt::IBlip *, bool) = ::IBlip::SetPulse;
  IBlip_SetPulse$(ptr, val);
}

void IBlip$cxxbridge1$IBlip_SetAsMissionCreator(::alt::IBlip *ptr, bool val) noexcept {
  void (*IBlip_SetAsMissionCreator$)(::alt::IBlip *, bool) = ::IBlip::SetAsMissionCreator;
  IBlip_SetAsMissionCreator$(ptr, val);
}

void IBlip$cxxbridge1$IBlip_SetTickVisible(::alt::IBlip *ptr, bool val) noexcept {
  void (*IBlip_SetTickVisible$)(::alt::IBlip *, bool) = ::IBlip::SetTickVisible;
  IBlip_SetTickVisible$(ptr, val);
}

void IBlip$cxxbridge1$IBlip_SetHeadingIndicatorVisible(::alt::IBlip *ptr, bool val) noexcept {
  void (*IBlip_SetHeadingIndicatorVisible$)(::alt::IBlip *, bool) = ::IBlip::SetHeadingIndicatorVisible;
  IBlip_SetHeadingIndicatorVisible$(ptr, val);
}

void IBlip$cxxbridge1$IBlip_SetOutlineIndicatorVisible(::alt::IBlip *ptr, bool val) noexcept {
  void (*IBlip_SetOutlineIndicatorVisible$)(::alt::IBlip *, bool) = ::IBlip::SetOutlineIndicatorVisible;
  IBlip_SetOutlineIndicatorVisible$(ptr, val);
}

void IBlip$cxxbridge1$IBlip_SetFriendIndicatorVisible(::alt::IBlip *ptr, bool val) noexcept {
  void (*IBlip_SetFriendIndicatorVisible$)(::alt::IBlip *, bool) = ::IBlip::SetFriendIndicatorVisible;
  IBlip_SetFriendIndicatorVisible$(ptr, val);
}

void IBlip$cxxbridge1$IBlip_SetCrewIndicatorVisible(::alt::IBlip *ptr, bool val) noexcept {
  void (*IBlip_SetCrewIndicatorVisible$)(::alt::IBlip *, bool) = ::IBlip::SetCrewIndicatorVisible;
  IBlip_SetCrewIndicatorVisible$(ptr, val);
}

void IBlip$cxxbridge1$IBlip_SetCategory(::alt::IBlip *ptr, ::std::uint32_t val) noexcept {
  void (*IBlip_SetCategory$)(::alt::IBlip *, ::std::uint32_t) = ::IBlip::SetCategory;
  IBlip_SetCategory$(ptr, val);
}

void IBlip$cxxbridge1$IBlip_SetAsHighDetail(::alt::IBlip *ptr, bool val) noexcept {
  void (*IBlip_SetAsHighDetail$)(::alt::IBlip *, bool) = ::IBlip::SetAsHighDetail;
  IBlip_SetAsHighDetail$(ptr, val);
}

void IBlip$cxxbridge1$IBlip_SetShrinked(::alt::IBlip *ptr, bool val) noexcept {
  void (*IBlip_SetShrinked$)(::alt::IBlip *, bool) = ::IBlip::SetShrinked;
  IBlip_SetShrinked$(ptr, val);
}

void IBlip$cxxbridge1$IBlip_Fade(::alt::IBlip *ptr, ::std::uint32_t opacity, ::std::uint32_t duration) noexcept {
  void (*IBlip_Fade$)(::alt::IBlip *, ::std::uint32_t, ::std::uint32_t) = ::IBlip::Fade;
  IBlip_Fade$(ptr, opacity, duration);
}

bool IBlip$cxxbridge1$IBlip_IsHiddenOnLegend(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_IsHiddenOnLegend$)(::alt::IBlip const *) = ::IBlip::IsHiddenOnLegend;
  return IBlip_IsHiddenOnLegend$(ptr);
}

void IBlip$cxxbridge1$IBlip_SetHiddenOnLegend(::alt::IBlip *ptr, bool state) noexcept {
  void (*IBlip_SetHiddenOnLegend$)(::alt::IBlip *, bool) = ::IBlip::SetHiddenOnLegend;
  IBlip_SetHiddenOnLegend$(ptr, state);
}

bool IBlip$cxxbridge1$IBlip_IsMinimalOnEdge(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_IsMinimalOnEdge$)(::alt::IBlip const *) = ::IBlip::IsMinimalOnEdge;
  return IBlip_IsMinimalOnEdge$(ptr);
}

void IBlip$cxxbridge1$IBlip_SetMinimalOnEdge(::alt::IBlip *ptr, bool state) noexcept {
  void (*IBlip_SetMinimalOnEdge$)(::alt::IBlip *, bool) = ::IBlip::SetMinimalOnEdge;
  IBlip_SetMinimalOnEdge$(ptr, state);
}

bool IBlip$cxxbridge1$IBlip_IsUseHeightIndicatorOnEdge(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_IsUseHeightIndicatorOnEdge$)(::alt::IBlip const *) = ::IBlip::IsUseHeightIndicatorOnEdge;
  return IBlip_IsUseHeightIndicatorOnEdge$(ptr);
}

void IBlip$cxxbridge1$IBlip_SetUseHeightIndicatorOnEdge(::alt::IBlip *ptr, bool state) noexcept {
  void (*IBlip_SetUseHeightIndicatorOnEdge$)(::alt::IBlip *, bool) = ::IBlip::SetUseHeightIndicatorOnEdge;
  IBlip_SetUseHeightIndicatorOnEdge$(ptr, state);
}

bool IBlip$cxxbridge1$IBlip_IsShortHeightThreshold(::alt::IBlip const *ptr) noexcept {
  bool (*IBlip_IsShortHeightThreshold$)(::alt::IBlip const *) = ::IBlip::IsShortHeightThreshold;
  return IBlip_IsShortHeightThreshold$(ptr);
}

void IBlip$cxxbridge1$IBlip_SetShortHeightThreshold(::alt::IBlip *ptr, bool state) noexcept {
  void (*IBlip_SetShortHeightThreshold$)(::alt::IBlip *, bool) = ::IBlip::SetShortHeightThreshold;
  IBlip_SetShortHeightThreshold$(ptr, state);
}
} // extern "C"
} // namespace IBlip

namespace IMarker {
extern "C" {
bool IMarker$cxxbridge1$IMarker_IsGlobal(::alt::IMarker const *ptr) noexcept {
  bool (*IMarker_IsGlobal$)(::alt::IMarker const *) = ::IMarker::IsGlobal;
  return IMarker_IsGlobal$(ptr);
}

::alt::IPlayer *IMarker$cxxbridge1$IMarker_GetTarget(::alt::IMarker const *ptr) noexcept {
  ::alt::IPlayer *(*IMarker_GetTarget$)(::alt::IMarker const *) = ::IMarker::GetTarget;
  return IMarker_GetTarget$(ptr);
}
} // extern "C"
} // namespace IMarker

extern "C" {
void cxxbridge1$IMarker_GetColor_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IMarker const *ptr, ::RGBAWrapper *placement_return_type) noexcept {
  void (*IMarker_GetColor_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IMarker const *, ::RGBAWrapper *) = ::IMarker_GetColor_autocxx_wrapper_0xd5d0abec981e3e3a;
  IMarker_GetColor_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IMarker {
extern "C" {
void IMarker$cxxbridge1$IMarker_SetColor(::alt::IMarker *ptr, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a) noexcept {
  void (*IMarker_SetColor$)(::alt::IMarker *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::IMarker::SetColor;
  IMarker_SetColor$(ptr, color_r, color_g, color_b, color_a);
}

bool IMarker$cxxbridge1$IMarker_IsVisible(::alt::IMarker const *ptr) noexcept {
  bool (*IMarker_IsVisible$)(::alt::IMarker const *) = ::IMarker::IsVisible;
  return IMarker_IsVisible$(ptr);
}

void IMarker$cxxbridge1$IMarker_SetVisible(::alt::IMarker *ptr, bool visible) noexcept {
  void (*IMarker_SetVisible$)(::alt::IMarker *, bool) = ::IMarker::SetVisible;
  IMarker_SetVisible$(ptr, visible);
}

::std::uint32_t IMarker$cxxbridge1$IMarker_GetMarkerType(::alt::IMarker const *ptr) noexcept {
  ::std::uint32_t (*IMarker_GetMarkerType$)(::alt::IMarker const *) = ::IMarker::GetMarkerType;
  return IMarker_GetMarkerType$(ptr);
}

void IMarker$cxxbridge1$IMarker_SetMarkerType(::alt::IMarker *ptr, ::std::uint32_t type_) noexcept {
  void (*IMarker_SetMarkerType$)(::alt::IMarker *, ::std::uint32_t) = ::IMarker::SetMarkerType;
  IMarker_SetMarkerType$(ptr, type_);
}
} // extern "C"
} // namespace IMarker

extern "C" {
void cxxbridge1$IMarker_GetScale_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IMarker const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*IMarker_GetScale_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IMarker const *, ::Vector3Wrapper *) = ::IMarker_GetScale_autocxx_wrapper_0xd5d0abec981e3e3a;
  IMarker_GetScale_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IMarker {
extern "C" {
void IMarker$cxxbridge1$IMarker_SetScale(::alt::IMarker *ptr, float scale_x, float scale_y, float scale_z) noexcept {
  void (*IMarker_SetScale$)(::alt::IMarker *, float, float, float) = ::IMarker::SetScale;
  IMarker_SetScale$(ptr, scale_x, scale_y, scale_z);
}
} // extern "C"
} // namespace IMarker

extern "C" {
void cxxbridge1$IMarker_GetRotation_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IMarker const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*IMarker_GetRotation_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IMarker const *, ::Vector3Wrapper *) = ::IMarker_GetRotation_autocxx_wrapper_0xd5d0abec981e3e3a;
  IMarker_GetRotation_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IMarker {
extern "C" {
void IMarker$cxxbridge1$IMarker_SetRotation(::alt::IMarker *ptr, float _rot_x, float _rot_y, float _rot_z) noexcept {
  void (*IMarker_SetRotation$)(::alt::IMarker *, float, float, float) = ::IMarker::SetRotation;
  IMarker_SetRotation$(ptr, _rot_x, _rot_y, _rot_z);
}
} // extern "C"
} // namespace IMarker

extern "C" {
void cxxbridge1$IMarker_GetDirection_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IMarker const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*IMarker_GetDirection_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IMarker const *, ::Vector3Wrapper *) = ::IMarker_GetDirection_autocxx_wrapper_0xd5d0abec981e3e3a;
  IMarker_GetDirection_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace IMarker {
extern "C" {
void IMarker$cxxbridge1$IMarker_SetDirection(::alt::IMarker *ptr, float dir_x, float dir_y, float dir_z) noexcept {
  void (*IMarker_SetDirection$)(::alt::IMarker *, float, float, float) = ::IMarker::SetDirection;
  IMarker_SetDirection$(ptr, dir_x, dir_y, dir_z);
}

bool IMarker$cxxbridge1$IMarker_IsFaceCamera(::alt::IMarker const *ptr) noexcept {
  bool (*IMarker_IsFaceCamera$)(::alt::IMarker const *) = ::IMarker::IsFaceCamera;
  return IMarker_IsFaceCamera$(ptr);
}

void IMarker$cxxbridge1$IMarker_SetFaceCamera(::alt::IMarker *ptr, bool faceCamera) noexcept {
  void (*IMarker_SetFaceCamera$)(::alt::IMarker *, bool) = ::IMarker::SetFaceCamera;
  IMarker_SetFaceCamera$(ptr, faceCamera);
}

bool IMarker$cxxbridge1$IMarker_IsRotating(::alt::IMarker const *ptr) noexcept {
  bool (*IMarker_IsRotating$)(::alt::IMarker const *) = ::IMarker::IsRotating;
  return IMarker_IsRotating$(ptr);
}

void IMarker$cxxbridge1$IMarker_SetRotating(::alt::IMarker *ptr, bool rotating) noexcept {
  void (*IMarker_SetRotating$)(::alt::IMarker *, bool) = ::IMarker::SetRotating;
  IMarker_SetRotating$(ptr, rotating);
}

bool IMarker$cxxbridge1$IMarker_IsBobUpDown(::alt::IMarker const *ptr) noexcept {
  bool (*IMarker_IsBobUpDown$)(::alt::IMarker const *) = ::IMarker::IsBobUpDown;
  return IMarker_IsBobUpDown$(ptr);
}

void IMarker$cxxbridge1$IMarker_SetBobUpDown(::alt::IMarker *ptr, bool bobUpDown) noexcept {
  void (*IMarker_SetBobUpDown$)(::alt::IMarker *, bool) = ::IMarker::SetBobUpDown;
  IMarker_SetBobUpDown$(ptr, bobUpDown);
}

::std::uint32_t IMarker$cxxbridge1$IMarker_GetStreamingDistance(::alt::IMarker const *ptr) noexcept {
  ::std::uint32_t (*IMarker_GetStreamingDistance$)(::alt::IMarker const *) = ::IMarker::GetStreamingDistance;
  return IMarker_GetStreamingDistance$(ptr);
}
} // extern "C"
} // namespace IMarker

namespace ICheckpoint {
extern "C" {
::std::uint8_t ICheckpoint$cxxbridge1$ICheckpoint_GetCheckpointType(::alt::ICheckpoint const *ptr) noexcept {
  ::std::uint8_t (*ICheckpoint_GetCheckpointType$)(::alt::ICheckpoint const *) = ::ICheckpoint::GetCheckpointType;
  return ICheckpoint_GetCheckpointType$(ptr);
}

float ICheckpoint$cxxbridge1$ICheckpoint_GetHeight(::alt::ICheckpoint const *ptr) noexcept {
  float (*ICheckpoint_GetHeight$)(::alt::ICheckpoint const *) = ::ICheckpoint::GetHeight;
  return ICheckpoint_GetHeight$(ptr);
}

float ICheckpoint$cxxbridge1$ICheckpoint_GetRadius(::alt::ICheckpoint const *ptr) noexcept {
  float (*ICheckpoint_GetRadius$)(::alt::ICheckpoint const *) = ::ICheckpoint::GetRadius;
  return ICheckpoint_GetRadius$(ptr);
}
} // extern "C"
} // namespace ICheckpoint

extern "C" {
void cxxbridge1$ICheckpoint_GetColor_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::ICheckpoint const *ptr, ::RGBAWrapper *placement_return_type) noexcept {
  void (*ICheckpoint_GetColor_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::ICheckpoint const *, ::RGBAWrapper *) = ::ICheckpoint_GetColor_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICheckpoint_GetColor_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$ICheckpoint_GetIconColor_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::ICheckpoint const *ptr, ::RGBAWrapper *placement_return_type) noexcept {
  void (*ICheckpoint_GetIconColor_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::ICheckpoint const *, ::RGBAWrapper *) = ::ICheckpoint_GetIconColor_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICheckpoint_GetIconColor_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$ICheckpoint_GetNextPosition_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::ICheckpoint const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*ICheckpoint_GetNextPosition_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::ICheckpoint const *, ::Vector3Wrapper *) = ::ICheckpoint_GetNextPosition_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICheckpoint_GetNextPosition_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace ICheckpoint {
extern "C" {
void ICheckpoint$cxxbridge1$ICheckpoint_SetCheckpointType(::alt::ICheckpoint *ptr, ::std::uint8_t type_) noexcept {
  void (*ICheckpoint_SetCheckpointType$)(::alt::ICheckpoint *, ::std::uint8_t) = ::ICheckpoint::SetCheckpointType;
  ICheckpoint_SetCheckpointType$(ptr, type_);
}

void ICheckpoint$cxxbridge1$ICheckpoint_SetHeight(::alt::ICheckpoint *ptr, float height) noexcept {
  void (*ICheckpoint_SetHeight$)(::alt::ICheckpoint *, float) = ::ICheckpoint::SetHeight;
  ICheckpoint_SetHeight$(ptr, height);
}

void ICheckpoint$cxxbridge1$ICheckpoint_SetRadius(::alt::ICheckpoint *ptr, float radius) noexcept {
  void (*ICheckpoint_SetRadius$)(::alt::ICheckpoint *, float) = ::ICheckpoint::SetRadius;
  ICheckpoint_SetRadius$(ptr, radius);
}

void ICheckpoint$cxxbridge1$ICheckpoint_SetColor(::alt::ICheckpoint *ptr, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a) noexcept {
  void (*ICheckpoint_SetColor$)(::alt::ICheckpoint *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::ICheckpoint::SetColor;
  ICheckpoint_SetColor$(ptr, color_r, color_g, color_b, color_a);
}

void ICheckpoint$cxxbridge1$ICheckpoint_SetIconColor(::alt::ICheckpoint *ptr, ::std::uint8_t color_r, ::std::uint8_t color_g, ::std::uint8_t color_b, ::std::uint8_t color_a) noexcept {
  void (*ICheckpoint_SetIconColor$)(::alt::ICheckpoint *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::ICheckpoint::SetIconColor;
  ICheckpoint_SetIconColor$(ptr, color_r, color_g, color_b, color_a);
}

void ICheckpoint$cxxbridge1$ICheckpoint_SetNextPosition(::alt::ICheckpoint *ptr, float pos_x, float pos_y, float pos_z) noexcept {
  void (*ICheckpoint_SetNextPosition$)(::alt::ICheckpoint *, float, float, float) = ::ICheckpoint::SetNextPosition;
  ICheckpoint_SetNextPosition$(ptr, pos_x, pos_y, pos_z);
}

::std::uint32_t ICheckpoint$cxxbridge1$ICheckpoint_GetStreamingDistance(::alt::ICheckpoint const *ptr) noexcept {
  ::std::uint32_t (*ICheckpoint_GetStreamingDistance$)(::alt::ICheckpoint const *) = ::ICheckpoint::GetStreamingDistance;
  return ICheckpoint_GetStreamingDistance$(ptr);
}

void ICheckpoint$cxxbridge1$ICheckpoint_SetVisible(::alt::ICheckpoint *ptr, bool toggle) noexcept {
  void (*ICheckpoint_SetVisible$)(::alt::ICheckpoint *, bool) = ::ICheckpoint::SetVisible;
  ICheckpoint_SetVisible$(ptr, toggle);
}

bool ICheckpoint$cxxbridge1$ICheckpoint_IsVisible(::alt::ICheckpoint const *ptr) noexcept {
  bool (*ICheckpoint_IsVisible$)(::alt::ICheckpoint const *) = ::ICheckpoint::IsVisible;
  return ICheckpoint_IsVisible$(ptr);
}
} // extern "C"
} // namespace ICheckpoint

extern "C" {
bool cxxbridge1$ICheckpoint_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::ICheckpoint const *ptr, ::std::string *key) noexcept {
  bool (*ICheckpoint_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::ICheckpoint const *, ::std::unique_ptr<::std::string>) = ::ICheckpoint_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICheckpoint_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}

void cxxbridge1$ICheckpoint_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::ICheckpoint const *ptr, ::std::string *key, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*ICheckpoint_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::ICheckpoint const *, ::std::unique_ptr<::std::string>, ::ConstMValueWrapper *) = ::ICheckpoint_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICheckpoint_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), placement_return_type);
}

::std::vector<::std::string> *cxxbridge1$ICheckpoint_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::ICheckpoint const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*ICheckpoint_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::ICheckpoint const *) = ::ICheckpoint_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ICheckpoint_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$ICheckpoint_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::ICheckpoint *ptr, ::std::string *key, ::MValueMutWrapper *val) noexcept {
  void (*ICheckpoint_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::ICheckpoint *, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::ICheckpoint_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICheckpoint_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), val);
}
} // extern "C"

namespace ICheckpoint {
extern "C" {
void ICheckpoint$cxxbridge1$ICheckpoint_SetMultipleStreamSyncedMetaData(::alt::ICheckpoint *ptr, ::MValueUnorderedMapWrapper const &values) noexcept {
  void (*ICheckpoint_SetMultipleStreamSyncedMetaData$)(::alt::ICheckpoint *, ::MValueUnorderedMapWrapper const &) = ::ICheckpoint::SetMultipleStreamSyncedMetaData;
  ICheckpoint_SetMultipleStreamSyncedMetaData$(ptr, values);
}
} // extern "C"
} // namespace ICheckpoint

extern "C" {
void cxxbridge1$ICheckpoint_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::ICheckpoint *ptr, ::std::string *key) noexcept {
  void (*ICheckpoint_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::ICheckpoint *, ::std::unique_ptr<::std::string>) = ::ICheckpoint_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  ICheckpoint_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}
} // extern "C"

namespace CEvent {
extern "C" {
bool CEvent$cxxbridge1$CEvent_IsCancellable(::alt::CEvent const *ptr) noexcept {
  bool (*CEvent_IsCancellable$)(::alt::CEvent const *) = ::CEvent::IsCancellable;
  return CEvent_IsCancellable$(ptr);
}

::std::uint16_t CEvent$cxxbridge1$CEvent_GetType(::alt::CEvent const *ptr) noexcept {
  ::std::uint16_t (*CEvent_GetType$)(::alt::CEvent const *) = ::CEvent::GetType;
  return CEvent_GetType$(ptr);
}
} // extern "C"
} // namespace CEvent

namespace CCancellableEvent {
extern "C" {
bool CCancellableEvent$cxxbridge1$CCancellableEvent_IsCancellable(::alt::CCancellableEvent const *ptr) noexcept {
  bool (*CCancellableEvent_IsCancellable$)(::alt::CCancellableEvent const *) = ::CCancellableEvent::IsCancellable;
  return CCancellableEvent_IsCancellable$(ptr);
}

bool CCancellableEvent$cxxbridge1$CCancellableEvent_WasCancelled(::alt::CCancellableEvent const *ptr) noexcept {
  bool (*CCancellableEvent_WasCancelled$)(::alt::CCancellableEvent const *) = ::CCancellableEvent::WasCancelled;
  return CCancellableEvent_WasCancelled$(ptr);
}

void CCancellableEvent$cxxbridge1$CCancellableEvent_Cancel(::alt::CCancellableEvent const *ptr) noexcept {
  void (*CCancellableEvent_Cancel$)(::alt::CCancellableEvent const *) = ::CCancellableEvent::Cancel;
  CCancellableEvent_Cancel$(ptr);
}
} // extern "C"
} // namespace CCancellableEvent

namespace IResource {
extern "C" {
bool IResource$cxxbridge1$IResource_IsStarted(::alt::IResource const *ptr) noexcept {
  bool (*IResource_IsStarted$)(::alt::IResource const *) = ::IResource::IsStarted;
  return IResource_IsStarted$(ptr);
}
} // extern "C"
} // namespace IResource

extern "C" {
::std::string *cxxbridge1$IResource_GetType_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IResource const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IResource_GetType_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IResource const *) = ::IResource_GetType_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IResource_GetType_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$IResource_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IResource const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IResource_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IResource const *) = ::IResource_GetName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IResource_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$IResource_GetPath_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IResource const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IResource_GetPath_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IResource const *) = ::IResource_GetPath_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IResource_GetPath_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$IResource_GetMain_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IResource const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IResource_GetMain_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IResource const *) = ::IResource_GetMain_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IResource_GetMain_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::vector<::std::string> *cxxbridge1$IResource_GetDependencies_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IResource const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*IResource_GetDependencies_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IResource const *) = ::IResource_GetDependencies_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IResource_GetDependencies_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::vector<::std::string> *cxxbridge1$IResource_GetDependants_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IResource const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*IResource_GetDependants_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IResource const *) = ::IResource_GetDependants_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IResource_GetDependants_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$IResource_GetClientType_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IResource const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IResource_GetClientType_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IResource const *) = ::IResource_GetClientType_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IResource_GetClientType_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$IResource_GetClientMain_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IResource const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IResource_GetClientMain_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IResource const *) = ::IResource_GetClientMain_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IResource_GetClientMain_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::vector<::std::string> *cxxbridge1$IResource_GetClientFiles_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IResource const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*IResource_GetClientFiles_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IResource const *) = ::IResource_GetClientFiles_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IResource_GetClientFiles_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *cxxbridge1$IResource_GetConfig_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IResource const *ptr) noexcept {
  ::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> (*IResource_GetConfig_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IResource const *) = ::IResource_GetConfig_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IResource_GetConfig_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IVoiceChannel {
extern "C" {
bool IVoiceChannel$cxxbridge1$IVoiceChannel_IsSpatial(::alt::IVoiceChannel const *ptr) noexcept {
  bool (*IVoiceChannel_IsSpatial$)(::alt::IVoiceChannel const *) = ::IVoiceChannel::IsSpatial;
  return IVoiceChannel_IsSpatial$(ptr);
}

float IVoiceChannel$cxxbridge1$IVoiceChannel_GetMaxDistance(::alt::IVoiceChannel const *ptr) noexcept {
  float (*IVoiceChannel_GetMaxDistance$)(::alt::IVoiceChannel const *) = ::IVoiceChannel::GetMaxDistance;
  return IVoiceChannel_GetMaxDistance$(ptr);
}

bool IVoiceChannel$cxxbridge1$IVoiceChannel_HasPlayer(::alt::IVoiceChannel const *ptr, ::alt::IPlayer *player) noexcept {
  bool (*IVoiceChannel_HasPlayer$)(::alt::IVoiceChannel const *, ::alt::IPlayer *) = ::IVoiceChannel::HasPlayer;
  return IVoiceChannel_HasPlayer$(ptr, player);
}

void IVoiceChannel$cxxbridge1$IVoiceChannel_AddPlayer(::alt::IVoiceChannel *ptr, ::alt::IPlayer *player) noexcept {
  void (*IVoiceChannel_AddPlayer$)(::alt::IVoiceChannel *, ::alt::IPlayer *) = ::IVoiceChannel::AddPlayer;
  IVoiceChannel_AddPlayer$(ptr, player);
}

void IVoiceChannel$cxxbridge1$IVoiceChannel_RemovePlayer(::alt::IVoiceChannel *ptr, ::alt::IPlayer *player) noexcept {
  void (*IVoiceChannel_RemovePlayer$)(::alt::IVoiceChannel *, ::alt::IPlayer *) = ::IVoiceChannel::RemovePlayer;
  IVoiceChannel_RemovePlayer$(ptr, player);
}

bool IVoiceChannel$cxxbridge1$IVoiceChannel_IsPlayerMuted(::alt::IVoiceChannel const *ptr, ::alt::IPlayer *player) noexcept {
  bool (*IVoiceChannel_IsPlayerMuted$)(::alt::IVoiceChannel const *, ::alt::IPlayer *) = ::IVoiceChannel::IsPlayerMuted;
  return IVoiceChannel_IsPlayerMuted$(ptr, player);
}

void IVoiceChannel$cxxbridge1$IVoiceChannel_MutePlayer(::alt::IVoiceChannel *ptr, ::alt::IPlayer *player) noexcept {
  void (*IVoiceChannel_MutePlayer$)(::alt::IVoiceChannel *, ::alt::IPlayer *) = ::IVoiceChannel::MutePlayer;
  IVoiceChannel_MutePlayer$(ptr, player);
}

void IVoiceChannel$cxxbridge1$IVoiceChannel_UnmutePlayer(::alt::IVoiceChannel *ptr, ::alt::IPlayer *player) noexcept {
  void (*IVoiceChannel_UnmutePlayer$)(::alt::IVoiceChannel *, ::alt::IPlayer *) = ::IVoiceChannel::UnmutePlayer;
  IVoiceChannel_UnmutePlayer$(ptr, player);
}
} // extern "C"
} // namespace IVoiceChannel

extern "C" {
::std::vector<::PlayerPtrWrapper> *cxxbridge1$IVoiceChannel_GetPlayers_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVoiceChannel const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::PlayerPtrWrapper>> (*IVoiceChannel_GetPlayers_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVoiceChannel const *) = ::IVoiceChannel_GetPlayers_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IVoiceChannel_GetPlayers_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IVoiceChannel {
extern "C" {
::std::uint32_t IVoiceChannel$cxxbridge1$IVoiceChannel_GetFilter(::alt::IVoiceChannel const *ptr) noexcept {
  ::std::uint32_t (*IVoiceChannel_GetFilter$)(::alt::IVoiceChannel const *) = ::IVoiceChannel::GetFilter;
  return IVoiceChannel_GetFilter$(ptr);
}

void IVoiceChannel$cxxbridge1$IVoiceChannel_SetFilter(::alt::IVoiceChannel *ptr, ::std::uint32_t filter) noexcept {
  void (*IVoiceChannel_SetFilter$)(::alt::IVoiceChannel *, ::std::uint32_t) = ::IVoiceChannel::SetFilter;
  IVoiceChannel_SetFilter$(ptr, filter);
}

::std::int32_t IVoiceChannel$cxxbridge1$IVoiceChannel_GetPriority(::alt::IVoiceChannel const *ptr) noexcept {
  ::std::int32_t (*IVoiceChannel_GetPriority$)(::alt::IVoiceChannel const *) = ::IVoiceChannel::GetPriority;
  return IVoiceChannel_GetPriority$(ptr);
}

void IVoiceChannel$cxxbridge1$IVoiceChannel_SetPriority(::alt::IVoiceChannel *ptr, ::std::int32_t priority) noexcept {
  void (*IVoiceChannel_SetPriority$)(::alt::IVoiceChannel *, ::std::int32_t) = ::IVoiceChannel::SetPriority;
  IVoiceChannel_SetPriority$(ptr, priority);
}
} // extern "C"
} // namespace IVoiceChannel

extern "C" {
::std::string *cxxbridge1$CConsoleCommandEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CConsoleCommandEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CConsoleCommandEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CConsoleCommandEvent const *) = ::CConsoleCommandEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CConsoleCommandEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::vector<::std::string> *cxxbridge1$CConsoleCommandEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CConsoleCommandEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*CConsoleCommandEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CConsoleCommandEvent const *) = ::CConsoleCommandEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CConsoleCommandEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$CServerScriptEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CServerScriptEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CServerScriptEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CServerScriptEvent const *) = ::CServerScriptEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CServerScriptEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::vector<::ConstMValueWrapper> *cxxbridge1$CServerScriptEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CServerScriptEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::ConstMValueWrapper>> (*CServerScriptEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CServerScriptEvent const *) = ::CServerScriptEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CServerScriptEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace CClientScriptEvent {
extern "C" {
::alt::IPlayer *CClientScriptEvent$cxxbridge1$CClientScriptEvent_GetTarget(::alt::CClientScriptEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CClientScriptEvent_GetTarget$)(::alt::CClientScriptEvent const *) = ::CClientScriptEvent::GetTarget;
  return CClientScriptEvent_GetTarget$(ptr);
}
} // extern "C"
} // namespace CClientScriptEvent

extern "C" {
::std::string *cxxbridge1$CClientScriptEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CClientScriptEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CClientScriptEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CClientScriptEvent const *) = ::CClientScriptEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CClientScriptEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::vector<::ConstMValueWrapper> *cxxbridge1$CClientScriptEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CClientScriptEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::ConstMValueWrapper>> (*CClientScriptEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CClientScriptEvent const *) = ::CClientScriptEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CClientScriptEvent_GetArgs_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace CPlayerConnectEvent {
extern "C" {
::alt::IPlayer *CPlayerConnectEvent$cxxbridge1$CPlayerConnectEvent_GetTarget(::alt::CPlayerConnectEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerConnectEvent_GetTarget$)(::alt::CPlayerConnectEvent const *) = ::CPlayerConnectEvent::GetTarget;
  return CPlayerConnectEvent_GetTarget$(ptr);
}
} // extern "C"
} // namespace CPlayerConnectEvent

extern "C" {
::std::string *cxxbridge1$CPlayerConnectEvent_GetReason_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerConnectEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CPlayerConnectEvent_GetReason_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerConnectEvent const *) = ::CPlayerConnectEvent_GetReason_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerConnectEvent_GetReason_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$CPlayerConnectEvent_Cancel_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerConnectEvent *ptr, ::std::string *_reason) noexcept {
  void (*CPlayerConnectEvent_Cancel_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerConnectEvent *, ::std::unique_ptr<::std::string>) = ::CPlayerConnectEvent_Cancel_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerConnectEvent_Cancel_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(_reason));
}
} // extern "C"

namespace CPlayerDisconnectEvent {
extern "C" {
::alt::IPlayer *CPlayerDisconnectEvent$cxxbridge1$CPlayerDisconnectEvent_GetTarget(::alt::CPlayerDisconnectEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerDisconnectEvent_GetTarget$)(::alt::CPlayerDisconnectEvent const *) = ::CPlayerDisconnectEvent::GetTarget;
  return CPlayerDisconnectEvent_GetTarget$(ptr);
}
} // extern "C"
} // namespace CPlayerDisconnectEvent

extern "C" {
::std::string *cxxbridge1$CPlayerDisconnectEvent_GetReason_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerDisconnectEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CPlayerDisconnectEvent_GetReason_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerDisconnectEvent const *) = ::CPlayerDisconnectEvent_GetReason_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerDisconnectEvent_GetReason_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace CColShapeEvent {
extern "C" {
::alt::IColShape *CColShapeEvent$cxxbridge1$CColShapeEvent_GetTarget(::alt::CColShapeEvent const *ptr) noexcept {
  ::alt::IColShape *(*CColShapeEvent_GetTarget$)(::alt::CColShapeEvent const *) = ::CColShapeEvent::GetTarget;
  return CColShapeEvent_GetTarget$(ptr);
}

::alt::IWorldObject *CColShapeEvent$cxxbridge1$CColShapeEvent_GetEntity(::alt::CColShapeEvent const *ptr) noexcept {
  ::alt::IWorldObject *(*CColShapeEvent_GetEntity$)(::alt::CColShapeEvent const *) = ::CColShapeEvent::GetEntity;
  return CColShapeEvent_GetEntity$(ptr);
}

bool CColShapeEvent$cxxbridge1$CColShapeEvent_GetState(::alt::CColShapeEvent const *ptr) noexcept {
  bool (*CColShapeEvent_GetState$)(::alt::CColShapeEvent const *) = ::CColShapeEvent::GetState;
  return CColShapeEvent_GetState$(ptr);
}
} // extern "C"
} // namespace CColShapeEvent

namespace IVirtualEntity {
extern "C" {
::alt::IVirtualEntityGroup *IVirtualEntity$cxxbridge1$IVirtualEntity_GetGroup(::alt::IVirtualEntity const *ptr) noexcept {
  ::alt::IVirtualEntityGroup *(*IVirtualEntity_GetGroup$)(::alt::IVirtualEntity const *) = ::IVirtualEntity::GetGroup;
  return IVirtualEntity_GetGroup$(ptr);
}
} // extern "C"
} // namespace IVirtualEntity

extern "C" {
bool cxxbridge1$IVirtualEntity_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVirtualEntity const *ptr, ::std::string *key) noexcept {
  bool (*IVirtualEntity_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVirtualEntity const *, ::std::unique_ptr<::std::string>) = ::IVirtualEntity_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IVirtualEntity_HasStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}

void cxxbridge1$IVirtualEntity_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVirtualEntity const *ptr, ::std::string *key, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*IVirtualEntity_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVirtualEntity const *, ::std::unique_ptr<::std::string>, ::ConstMValueWrapper *) = ::IVirtualEntity_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVirtualEntity_GetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), placement_return_type);
}

::std::vector<::std::string> *cxxbridge1$IVirtualEntity_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVirtualEntity const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::std::string>> (*IVirtualEntity_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVirtualEntity const *) = ::IVirtualEntity_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IVirtualEntity_GetStreamSyncedMetaDataKeys_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IVirtualEntity {
extern "C" {
::std::uint32_t IVirtualEntity$cxxbridge1$IVirtualEntity_GetStreamingDistance(::alt::IVirtualEntity const *ptr) noexcept {
  ::std::uint32_t (*IVirtualEntity_GetStreamingDistance$)(::alt::IVirtualEntity const *) = ::IVirtualEntity::GetStreamingDistance;
  return IVirtualEntity_GetStreamingDistance$(ptr);
}

void IVirtualEntity$cxxbridge1$IVirtualEntity_SetVisible(::alt::IVirtualEntity *ptr, bool toggle) noexcept {
  void (*IVirtualEntity_SetVisible$)(::alt::IVirtualEntity *, bool) = ::IVirtualEntity::SetVisible;
  IVirtualEntity_SetVisible$(ptr, toggle);
}

bool IVirtualEntity$cxxbridge1$IVirtualEntity_IsVisible(::alt::IVirtualEntity const *ptr) noexcept {
  bool (*IVirtualEntity_IsVisible$)(::alt::IVirtualEntity const *) = ::IVirtualEntity::IsVisible;
  return IVirtualEntity_IsVisible$(ptr);
}
} // extern "C"
} // namespace IVirtualEntity

extern "C" {
void cxxbridge1$IVirtualEntity_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVirtualEntity *ptr, ::std::string *key, ::MValueMutWrapper *val) noexcept {
  void (*IVirtualEntity_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVirtualEntity *, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::IVirtualEntity_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVirtualEntity_SetStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key), val);
}
} // extern "C"

namespace IVirtualEntity {
extern "C" {
void IVirtualEntity$cxxbridge1$IVirtualEntity_SetMultipleStreamSyncedMetaData(::alt::IVirtualEntity *ptr, ::MValueUnorderedMapWrapper const &values) noexcept {
  void (*IVirtualEntity_SetMultipleStreamSyncedMetaData$)(::alt::IVirtualEntity *, ::MValueUnorderedMapWrapper const &) = ::IVirtualEntity::SetMultipleStreamSyncedMetaData;
  IVirtualEntity_SetMultipleStreamSyncedMetaData$(ptr, values);
}
} // extern "C"
} // namespace IVirtualEntity

extern "C" {
void cxxbridge1$IVirtualEntity_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IVirtualEntity *ptr, ::std::string *key) noexcept {
  void (*IVirtualEntity_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IVirtualEntity *, ::std::unique_ptr<::std::string>) = ::IVirtualEntity_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a;
  IVirtualEntity_DeleteStreamSyncedMetaData_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(key));
}
} // extern "C"

namespace IVirtualEntityGroup {
extern "C" {
::std::uint32_t IVirtualEntityGroup$cxxbridge1$IVirtualEntityGroup_GetMaxEntitiesInStream(::alt::IVirtualEntityGroup const *ptr) noexcept {
  ::std::uint32_t (*IVirtualEntityGroup_GetMaxEntitiesInStream$)(::alt::IVirtualEntityGroup const *) = ::IVirtualEntityGroup::GetMaxEntitiesInStream;
  return IVirtualEntityGroup_GetMaxEntitiesInStream$(ptr);
}
} // extern "C"
} // namespace IVirtualEntityGroup

namespace CWeaponDamageEvent {
extern "C" {
::alt::IPlayer *CWeaponDamageEvent$cxxbridge1$CWeaponDamageEvent_GetSource(::alt::CWeaponDamageEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CWeaponDamageEvent_GetSource$)(::alt::CWeaponDamageEvent const *) = ::CWeaponDamageEvent::GetSource;
  return CWeaponDamageEvent_GetSource$(ptr);
}

::alt::IEntity *CWeaponDamageEvent$cxxbridge1$CWeaponDamageEvent_GetTarget(::alt::CWeaponDamageEvent const *ptr) noexcept {
  ::alt::IEntity *(*CWeaponDamageEvent_GetTarget$)(::alt::CWeaponDamageEvent const *) = ::CWeaponDamageEvent::GetTarget;
  return CWeaponDamageEvent_GetTarget$(ptr);
}

::std::uint32_t CWeaponDamageEvent$cxxbridge1$CWeaponDamageEvent_GetWeaponHash(::alt::CWeaponDamageEvent const *ptr) noexcept {
  ::std::uint32_t (*CWeaponDamageEvent_GetWeaponHash$)(::alt::CWeaponDamageEvent const *) = ::CWeaponDamageEvent::GetWeaponHash;
  return CWeaponDamageEvent_GetWeaponHash$(ptr);
}

::std::uint32_t CWeaponDamageEvent$cxxbridge1$CWeaponDamageEvent_GetDamageValue(::alt::CWeaponDamageEvent const *ptr) noexcept {
  ::std::uint32_t (*CWeaponDamageEvent_GetDamageValue$)(::alt::CWeaponDamageEvent const *) = ::CWeaponDamageEvent::GetDamageValue;
  return CWeaponDamageEvent_GetDamageValue$(ptr);
}
} // extern "C"
} // namespace CWeaponDamageEvent

extern "C" {
void cxxbridge1$CWeaponDamageEvent_GetShotOffset_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CWeaponDamageEvent const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*CWeaponDamageEvent_GetShotOffset_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CWeaponDamageEvent const *, ::Vector3Wrapper *) = ::CWeaponDamageEvent_GetShotOffset_autocxx_wrapper_0xd5d0abec981e3e3a;
  CWeaponDamageEvent_GetShotOffset_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace CWeaponDamageEvent {
extern "C" {
::std::int8_t CWeaponDamageEvent$cxxbridge1$CWeaponDamageEvent_GetBodyPart(::alt::CWeaponDamageEvent const *ptr) noexcept {
  ::std::int8_t (*CWeaponDamageEvent_GetBodyPart$)(::alt::CWeaponDamageEvent const *) = ::CWeaponDamageEvent::GetBodyPart;
  return CWeaponDamageEvent_GetBodyPart$(ptr);
}

::alt::IEntity *CWeaponDamageEvent$cxxbridge1$CWeaponDamageEvent_GetSourceEntity(::alt::CWeaponDamageEvent const *ptr) noexcept {
  ::alt::IEntity *(*CWeaponDamageEvent_GetSourceEntity$)(::alt::CWeaponDamageEvent const *) = ::CWeaponDamageEvent::GetSourceEntity;
  return CWeaponDamageEvent_GetSourceEntity$(ptr);
}

void CWeaponDamageEvent$cxxbridge1$CWeaponDamageEvent_SetDamageValue(::alt::CWeaponDamageEvent *ptr, ::std::uint32_t _damageValue) noexcept {
  void (*CWeaponDamageEvent_SetDamageValue$)(::alt::CWeaponDamageEvent *, ::std::uint32_t) = ::CWeaponDamageEvent::SetDamageValue;
  CWeaponDamageEvent_SetDamageValue$(ptr, _damageValue);
}
} // extern "C"
} // namespace CWeaponDamageEvent

namespace CNetOwnerChangeEvent {
extern "C" {
::alt::IEntity *CNetOwnerChangeEvent$cxxbridge1$CNetOwnerChangeEvent_GetTarget(::alt::CNetOwnerChangeEvent const *ptr) noexcept {
  ::alt::IEntity *(*CNetOwnerChangeEvent_GetTarget$)(::alt::CNetOwnerChangeEvent const *) = ::CNetOwnerChangeEvent::GetTarget;
  return CNetOwnerChangeEvent_GetTarget$(ptr);
}

::alt::IPlayer *CNetOwnerChangeEvent$cxxbridge1$CNetOwnerChangeEvent_GetNewOwner(::alt::CNetOwnerChangeEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CNetOwnerChangeEvent_GetNewOwner$)(::alt::CNetOwnerChangeEvent const *) = ::CNetOwnerChangeEvent::GetNewOwner;
  return CNetOwnerChangeEvent_GetNewOwner$(ptr);
}

::alt::IPlayer *CNetOwnerChangeEvent$cxxbridge1$CNetOwnerChangeEvent_GetOldOwner(::alt::CNetOwnerChangeEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CNetOwnerChangeEvent_GetOldOwner$)(::alt::CNetOwnerChangeEvent const *) = ::CNetOwnerChangeEvent::GetOldOwner;
  return CNetOwnerChangeEvent_GetOldOwner$(ptr);
}
} // extern "C"
} // namespace CNetOwnerChangeEvent

namespace CPlayerDeathEvent {
extern "C" {
::alt::IPlayer *CPlayerDeathEvent$cxxbridge1$CPlayerDeathEvent_GetTarget(::alt::CPlayerDeathEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerDeathEvent_GetTarget$)(::alt::CPlayerDeathEvent const *) = ::CPlayerDeathEvent::GetTarget;
  return CPlayerDeathEvent_GetTarget$(ptr);
}

::alt::IEntity *CPlayerDeathEvent$cxxbridge1$CPlayerDeathEvent_GetKiller(::alt::CPlayerDeathEvent const *ptr) noexcept {
  ::alt::IEntity *(*CPlayerDeathEvent_GetKiller$)(::alt::CPlayerDeathEvent const *) = ::CPlayerDeathEvent::GetKiller;
  return CPlayerDeathEvent_GetKiller$(ptr);
}

::std::uint32_t CPlayerDeathEvent$cxxbridge1$CPlayerDeathEvent_GetWeapon(::alt::CPlayerDeathEvent const *ptr) noexcept {
  ::std::uint32_t (*CPlayerDeathEvent_GetWeapon$)(::alt::CPlayerDeathEvent const *) = ::CPlayerDeathEvent::GetWeapon;
  return CPlayerDeathEvent_GetWeapon$(ptr);
}
} // extern "C"
} // namespace CPlayerDeathEvent

namespace CPlayerDamageEvent {
extern "C" {
::alt::IPlayer *CPlayerDamageEvent$cxxbridge1$CPlayerDamageEvent_GetTarget(::alt::CPlayerDamageEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerDamageEvent_GetTarget$)(::alt::CPlayerDamageEvent const *) = ::CPlayerDamageEvent::GetTarget;
  return CPlayerDamageEvent_GetTarget$(ptr);
}

::alt::IEntity *CPlayerDamageEvent$cxxbridge1$CPlayerDamageEvent_GetAttacker(::alt::CPlayerDamageEvent const *ptr) noexcept {
  ::alt::IEntity *(*CPlayerDamageEvent_GetAttacker$)(::alt::CPlayerDamageEvent const *) = ::CPlayerDamageEvent::GetAttacker;
  return CPlayerDamageEvent_GetAttacker$(ptr);
}

::std::uint16_t CPlayerDamageEvent$cxxbridge1$CPlayerDamageEvent_GetHealthDamage(::alt::CPlayerDamageEvent const *ptr) noexcept {
  ::std::uint16_t (*CPlayerDamageEvent_GetHealthDamage$)(::alt::CPlayerDamageEvent const *) = ::CPlayerDamageEvent::GetHealthDamage;
  return CPlayerDamageEvent_GetHealthDamage$(ptr);
}

::std::uint16_t CPlayerDamageEvent$cxxbridge1$CPlayerDamageEvent_GetArmourDamage(::alt::CPlayerDamageEvent const *ptr) noexcept {
  ::std::uint16_t (*CPlayerDamageEvent_GetArmourDamage$)(::alt::CPlayerDamageEvent const *) = ::CPlayerDamageEvent::GetArmourDamage;
  return CPlayerDamageEvent_GetArmourDamage$(ptr);
}

::std::uint32_t CPlayerDamageEvent$cxxbridge1$CPlayerDamageEvent_GetWeapon(::alt::CPlayerDamageEvent const *ptr) noexcept {
  ::std::uint32_t (*CPlayerDamageEvent_GetWeapon$)(::alt::CPlayerDamageEvent const *) = ::CPlayerDamageEvent::GetWeapon;
  return CPlayerDamageEvent_GetWeapon$(ptr);
}
} // extern "C"
} // namespace CPlayerDamageEvent

namespace CPlayerEnteringVehicleEvent {
extern "C" {
::alt::IVehicle *CPlayerEnteringVehicleEvent$cxxbridge1$CPlayerEnteringVehicleEvent_GetTarget(::alt::CPlayerEnteringVehicleEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CPlayerEnteringVehicleEvent_GetTarget$)(::alt::CPlayerEnteringVehicleEvent const *) = ::CPlayerEnteringVehicleEvent::GetTarget;
  return CPlayerEnteringVehicleEvent_GetTarget$(ptr);
}

::alt::IPlayer *CPlayerEnteringVehicleEvent$cxxbridge1$CPlayerEnteringVehicleEvent_GetPlayer(::alt::CPlayerEnteringVehicleEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerEnteringVehicleEvent_GetPlayer$)(::alt::CPlayerEnteringVehicleEvent const *) = ::CPlayerEnteringVehicleEvent::GetPlayer;
  return CPlayerEnteringVehicleEvent_GetPlayer$(ptr);
}

::std::uint8_t CPlayerEnteringVehicleEvent$cxxbridge1$CPlayerEnteringVehicleEvent_GetSeat(::alt::CPlayerEnteringVehicleEvent const *ptr) noexcept {
  ::std::uint8_t (*CPlayerEnteringVehicleEvent_GetSeat$)(::alt::CPlayerEnteringVehicleEvent const *) = ::CPlayerEnteringVehicleEvent::GetSeat;
  return CPlayerEnteringVehicleEvent_GetSeat$(ptr);
}
} // extern "C"
} // namespace CPlayerEnteringVehicleEvent

namespace CPlayerEnterVehicleEvent {
extern "C" {
::alt::IVehicle *CPlayerEnterVehicleEvent$cxxbridge1$CPlayerEnterVehicleEvent_GetTarget(::alt::CPlayerEnterVehicleEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CPlayerEnterVehicleEvent_GetTarget$)(::alt::CPlayerEnterVehicleEvent const *) = ::CPlayerEnterVehicleEvent::GetTarget;
  return CPlayerEnterVehicleEvent_GetTarget$(ptr);
}

::alt::IPlayer *CPlayerEnterVehicleEvent$cxxbridge1$CPlayerEnterVehicleEvent_GetPlayer(::alt::CPlayerEnterVehicleEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerEnterVehicleEvent_GetPlayer$)(::alt::CPlayerEnterVehicleEvent const *) = ::CPlayerEnterVehicleEvent::GetPlayer;
  return CPlayerEnterVehicleEvent_GetPlayer$(ptr);
}

::std::uint8_t CPlayerEnterVehicleEvent$cxxbridge1$CPlayerEnterVehicleEvent_GetSeat(::alt::CPlayerEnterVehicleEvent const *ptr) noexcept {
  ::std::uint8_t (*CPlayerEnterVehicleEvent_GetSeat$)(::alt::CPlayerEnterVehicleEvent const *) = ::CPlayerEnterVehicleEvent::GetSeat;
  return CPlayerEnterVehicleEvent_GetSeat$(ptr);
}
} // extern "C"
} // namespace CPlayerEnterVehicleEvent

namespace CPlayerLeaveVehicleEvent {
extern "C" {
::alt::IVehicle *CPlayerLeaveVehicleEvent$cxxbridge1$CPlayerLeaveVehicleEvent_GetTarget(::alt::CPlayerLeaveVehicleEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CPlayerLeaveVehicleEvent_GetTarget$)(::alt::CPlayerLeaveVehicleEvent const *) = ::CPlayerLeaveVehicleEvent::GetTarget;
  return CPlayerLeaveVehicleEvent_GetTarget$(ptr);
}

::alt::IPlayer *CPlayerLeaveVehicleEvent$cxxbridge1$CPlayerLeaveVehicleEvent_GetPlayer(::alt::CPlayerLeaveVehicleEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerLeaveVehicleEvent_GetPlayer$)(::alt::CPlayerLeaveVehicleEvent const *) = ::CPlayerLeaveVehicleEvent::GetPlayer;
  return CPlayerLeaveVehicleEvent_GetPlayer$(ptr);
}

::std::uint8_t CPlayerLeaveVehicleEvent$cxxbridge1$CPlayerLeaveVehicleEvent_GetSeat(::alt::CPlayerLeaveVehicleEvent const *ptr) noexcept {
  ::std::uint8_t (*CPlayerLeaveVehicleEvent_GetSeat$)(::alt::CPlayerLeaveVehicleEvent const *) = ::CPlayerLeaveVehicleEvent::GetSeat;
  return CPlayerLeaveVehicleEvent_GetSeat$(ptr);
}
} // extern "C"
} // namespace CPlayerLeaveVehicleEvent

namespace CPlayerChangeAnimationEvent {
extern "C" {
::alt::IPlayer *CPlayerChangeAnimationEvent$cxxbridge1$CPlayerChangeAnimationEvent_GetTarget(::alt::CPlayerChangeAnimationEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerChangeAnimationEvent_GetTarget$)(::alt::CPlayerChangeAnimationEvent const *) = ::CPlayerChangeAnimationEvent::GetTarget;
  return CPlayerChangeAnimationEvent_GetTarget$(ptr);
}

::std::uint32_t CPlayerChangeAnimationEvent$cxxbridge1$CPlayerChangeAnimationEvent_GetOldAnimationDict(::alt::CPlayerChangeAnimationEvent const *ptr) noexcept {
  ::std::uint32_t (*CPlayerChangeAnimationEvent_GetOldAnimationDict$)(::alt::CPlayerChangeAnimationEvent const *) = ::CPlayerChangeAnimationEvent::GetOldAnimationDict;
  return CPlayerChangeAnimationEvent_GetOldAnimationDict$(ptr);
}

::std::uint32_t CPlayerChangeAnimationEvent$cxxbridge1$CPlayerChangeAnimationEvent_GetOldAnimationName(::alt::CPlayerChangeAnimationEvent const *ptr) noexcept {
  ::std::uint32_t (*CPlayerChangeAnimationEvent_GetOldAnimationName$)(::alt::CPlayerChangeAnimationEvent const *) = ::CPlayerChangeAnimationEvent::GetOldAnimationName;
  return CPlayerChangeAnimationEvent_GetOldAnimationName$(ptr);
}

::std::uint32_t CPlayerChangeAnimationEvent$cxxbridge1$CPlayerChangeAnimationEvent_GetNewAnimationDict(::alt::CPlayerChangeAnimationEvent const *ptr) noexcept {
  ::std::uint32_t (*CPlayerChangeAnimationEvent_GetNewAnimationDict$)(::alt::CPlayerChangeAnimationEvent const *) = ::CPlayerChangeAnimationEvent::GetNewAnimationDict;
  return CPlayerChangeAnimationEvent_GetNewAnimationDict$(ptr);
}

::std::uint32_t CPlayerChangeAnimationEvent$cxxbridge1$CPlayerChangeAnimationEvent_GetNewAnimationName(::alt::CPlayerChangeAnimationEvent const *ptr) noexcept {
  ::std::uint32_t (*CPlayerChangeAnimationEvent_GetNewAnimationName$)(::alt::CPlayerChangeAnimationEvent const *) = ::CPlayerChangeAnimationEvent::GetNewAnimationName;
  return CPlayerChangeAnimationEvent_GetNewAnimationName$(ptr);
}
} // extern "C"
} // namespace CPlayerChangeAnimationEvent

namespace CPlayerChangeVehicleSeatEvent {
extern "C" {
::alt::IVehicle *CPlayerChangeVehicleSeatEvent$cxxbridge1$CPlayerChangeVehicleSeatEvent_GetTarget(::alt::CPlayerChangeVehicleSeatEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CPlayerChangeVehicleSeatEvent_GetTarget$)(::alt::CPlayerChangeVehicleSeatEvent const *) = ::CPlayerChangeVehicleSeatEvent::GetTarget;
  return CPlayerChangeVehicleSeatEvent_GetTarget$(ptr);
}

::alt::IPlayer *CPlayerChangeVehicleSeatEvent$cxxbridge1$CPlayerChangeVehicleSeatEvent_GetPlayer(::alt::CPlayerChangeVehicleSeatEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerChangeVehicleSeatEvent_GetPlayer$)(::alt::CPlayerChangeVehicleSeatEvent const *) = ::CPlayerChangeVehicleSeatEvent::GetPlayer;
  return CPlayerChangeVehicleSeatEvent_GetPlayer$(ptr);
}

::std::uint8_t CPlayerChangeVehicleSeatEvent$cxxbridge1$CPlayerChangeVehicleSeatEvent_GetOldSeat(::alt::CPlayerChangeVehicleSeatEvent const *ptr) noexcept {
  ::std::uint8_t (*CPlayerChangeVehicleSeatEvent_GetOldSeat$)(::alt::CPlayerChangeVehicleSeatEvent const *) = ::CPlayerChangeVehicleSeatEvent::GetOldSeat;
  return CPlayerChangeVehicleSeatEvent_GetOldSeat$(ptr);
}

::std::uint8_t CPlayerChangeVehicleSeatEvent$cxxbridge1$CPlayerChangeVehicleSeatEvent_GetNewSeat(::alt::CPlayerChangeVehicleSeatEvent const *ptr) noexcept {
  ::std::uint8_t (*CPlayerChangeVehicleSeatEvent_GetNewSeat$)(::alt::CPlayerChangeVehicleSeatEvent const *) = ::CPlayerChangeVehicleSeatEvent::GetNewSeat;
  return CPlayerChangeVehicleSeatEvent_GetNewSeat$(ptr);
}
} // extern "C"
} // namespace CPlayerChangeVehicleSeatEvent

namespace CPlayerWeaponChangeEvent {
extern "C" {
::alt::IPlayer *CPlayerWeaponChangeEvent$cxxbridge1$CPlayerWeaponChangeEvent_GetTarget(::alt::CPlayerWeaponChangeEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerWeaponChangeEvent_GetTarget$)(::alt::CPlayerWeaponChangeEvent const *) = ::CPlayerWeaponChangeEvent::GetTarget;
  return CPlayerWeaponChangeEvent_GetTarget$(ptr);
}

::std::uint32_t CPlayerWeaponChangeEvent$cxxbridge1$CPlayerWeaponChangeEvent_GetOldWeapon(::alt::CPlayerWeaponChangeEvent const *ptr) noexcept {
  ::std::uint32_t (*CPlayerWeaponChangeEvent_GetOldWeapon$)(::alt::CPlayerWeaponChangeEvent const *) = ::CPlayerWeaponChangeEvent::GetOldWeapon;
  return CPlayerWeaponChangeEvent_GetOldWeapon$(ptr);
}

::std::uint32_t CPlayerWeaponChangeEvent$cxxbridge1$CPlayerWeaponChangeEvent_GetNewWeapon(::alt::CPlayerWeaponChangeEvent const *ptr) noexcept {
  ::std::uint32_t (*CPlayerWeaponChangeEvent_GetNewWeapon$)(::alt::CPlayerWeaponChangeEvent const *) = ::CPlayerWeaponChangeEvent::GetNewWeapon;
  return CPlayerWeaponChangeEvent_GetNewWeapon$(ptr);
}
} // extern "C"
} // namespace CPlayerWeaponChangeEvent

namespace CPlayerConnectDeniedEvent {
extern "C" {
::std::uint8_t CPlayerConnectDeniedEvent$cxxbridge1$CPlayerConnectDeniedEvent_GetReason(::alt::CPlayerConnectDeniedEvent const *ptr) noexcept {
  ::std::uint8_t (*CPlayerConnectDeniedEvent_GetReason$)(::alt::CPlayerConnectDeniedEvent const *) = ::CPlayerConnectDeniedEvent::GetReason;
  return CPlayerConnectDeniedEvent_GetReason$(ptr);
}
} // extern "C"
} // namespace CPlayerConnectDeniedEvent

extern "C" {
::std::string *cxxbridge1$CPlayerConnectDeniedEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerConnectDeniedEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CPlayerConnectDeniedEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerConnectDeniedEvent const *) = ::CPlayerConnectDeniedEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerConnectDeniedEvent_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$CPlayerConnectDeniedEvent_GetIp_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerConnectDeniedEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CPlayerConnectDeniedEvent_GetIp_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerConnectDeniedEvent const *) = ::CPlayerConnectDeniedEvent_GetIp_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerConnectDeniedEvent_GetIp_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace CPlayerConnectDeniedEvent {
extern "C" {
::std::uint64_t CPlayerConnectDeniedEvent$cxxbridge1$CPlayerConnectDeniedEvent_GetPasswordHash(::alt::CPlayerConnectDeniedEvent const *ptr) noexcept {
  ::std::uint64_t (*CPlayerConnectDeniedEvent_GetPasswordHash$)(::alt::CPlayerConnectDeniedEvent const *) = ::CPlayerConnectDeniedEvent::GetPasswordHash;
  return CPlayerConnectDeniedEvent_GetPasswordHash$(ptr);
}

bool CPlayerConnectDeniedEvent$cxxbridge1$CPlayerConnectDeniedEvent_IsDebug(::alt::CPlayerConnectDeniedEvent const *ptr) noexcept {
  bool (*CPlayerConnectDeniedEvent_IsDebug$)(::alt::CPlayerConnectDeniedEvent const *) = ::CPlayerConnectDeniedEvent::IsDebug;
  return CPlayerConnectDeniedEvent_IsDebug$(ptr);
}
} // extern "C"
} // namespace CPlayerConnectDeniedEvent

extern "C" {
::std::string *cxxbridge1$CPlayerConnectDeniedEvent_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerConnectDeniedEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CPlayerConnectDeniedEvent_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerConnectDeniedEvent const *) = ::CPlayerConnectDeniedEvent_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerConnectDeniedEvent_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace CPlayerConnectDeniedEvent {
extern "C" {
::std::uint16_t CPlayerConnectDeniedEvent$cxxbridge1$CPlayerConnectDeniedEvent_GetVersionMajor(::alt::CPlayerConnectDeniedEvent const *ptr) noexcept {
  ::std::uint16_t (*CPlayerConnectDeniedEvent_GetVersionMajor$)(::alt::CPlayerConnectDeniedEvent const *) = ::CPlayerConnectDeniedEvent::GetVersionMajor;
  return CPlayerConnectDeniedEvent_GetVersionMajor$(ptr);
}

::std::uint16_t CPlayerConnectDeniedEvent$cxxbridge1$CPlayerConnectDeniedEvent_GetVersionMinor(::alt::CPlayerConnectDeniedEvent const *ptr) noexcept {
  ::std::uint16_t (*CPlayerConnectDeniedEvent_GetVersionMinor$)(::alt::CPlayerConnectDeniedEvent const *) = ::CPlayerConnectDeniedEvent::GetVersionMinor;
  return CPlayerConnectDeniedEvent_GetVersionMinor$(ptr);
}
} // extern "C"
} // namespace CPlayerConnectDeniedEvent

extern "C" {
::std::string *cxxbridge1$CPlayerConnectDeniedEvent_GetCdnUrl_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerConnectDeniedEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CPlayerConnectDeniedEvent_GetCdnUrl_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerConnectDeniedEvent const *) = ::CPlayerConnectDeniedEvent_GetCdnUrl_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerConnectDeniedEvent_GetCdnUrl_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace CPlayerConnectDeniedEvent {
extern "C" {
::std::int64_t CPlayerConnectDeniedEvent$cxxbridge1$CPlayerConnectDeniedEvent_GetDiscordId(::alt::CPlayerConnectDeniedEvent const *ptr) noexcept {
  ::std::int64_t (*CPlayerConnectDeniedEvent_GetDiscordId$)(::alt::CPlayerConnectDeniedEvent const *) = ::CPlayerConnectDeniedEvent::GetDiscordId;
  return CPlayerConnectDeniedEvent_GetDiscordId$(ptr);
}
} // extern "C"
} // namespace CPlayerConnectDeniedEvent

namespace CPlayerSpawnEvent {
extern "C" {
::alt::IPlayer *CPlayerSpawnEvent$cxxbridge1$CPlayerSpawnEvent_GetPlayer(::alt::CPlayerSpawnEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerSpawnEvent_GetPlayer$)(::alt::CPlayerSpawnEvent const *) = ::CPlayerSpawnEvent::GetPlayer;
  return CPlayerSpawnEvent_GetPlayer$(ptr);
}
} // extern "C"
} // namespace CPlayerSpawnEvent

namespace CPlayerRequestControlEvent {
extern "C" {
::alt::IEntity *CPlayerRequestControlEvent$cxxbridge1$CPlayerRequestControlEvent_GetTarget(::alt::CPlayerRequestControlEvent const *ptr) noexcept {
  ::alt::IEntity *(*CPlayerRequestControlEvent_GetTarget$)(::alt::CPlayerRequestControlEvent const *) = ::CPlayerRequestControlEvent::GetTarget;
  return CPlayerRequestControlEvent_GetTarget$(ptr);
}

::alt::IPlayer *CPlayerRequestControlEvent$cxxbridge1$CPlayerRequestControlEvent_GetPlayer(::alt::CPlayerRequestControlEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerRequestControlEvent_GetPlayer$)(::alt::CPlayerRequestControlEvent const *) = ::CPlayerRequestControlEvent::GetPlayer;
  return CPlayerRequestControlEvent_GetPlayer$(ptr);
}
} // extern "C"
} // namespace CPlayerRequestControlEvent

namespace CPlayerDimensionChangeEvent {
extern "C" {
::alt::IPlayer *CPlayerDimensionChangeEvent$cxxbridge1$CPlayerDimensionChangeEvent_GetTarget(::alt::CPlayerDimensionChangeEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerDimensionChangeEvent_GetTarget$)(::alt::CPlayerDimensionChangeEvent const *) = ::CPlayerDimensionChangeEvent::GetTarget;
  return CPlayerDimensionChangeEvent_GetTarget$(ptr);
}

::std::int32_t CPlayerDimensionChangeEvent$cxxbridge1$CPlayerDimensionChangeEvent_GetOldDimension(::alt::CPlayerDimensionChangeEvent const *ptr) noexcept {
  ::std::int32_t (*CPlayerDimensionChangeEvent_GetOldDimension$)(::alt::CPlayerDimensionChangeEvent const *) = ::CPlayerDimensionChangeEvent::GetOldDimension;
  return CPlayerDimensionChangeEvent_GetOldDimension$(ptr);
}

::std::int32_t CPlayerDimensionChangeEvent$cxxbridge1$CPlayerDimensionChangeEvent_GetNewDimension(::alt::CPlayerDimensionChangeEvent const *ptr) noexcept {
  ::std::int32_t (*CPlayerDimensionChangeEvent_GetNewDimension$)(::alt::CPlayerDimensionChangeEvent const *) = ::CPlayerDimensionChangeEvent::GetNewDimension;
  return CPlayerDimensionChangeEvent_GetNewDimension$(ptr);
}
} // extern "C"
} // namespace CPlayerDimensionChangeEvent

namespace CPlayerChangeInteriorEvent {
extern "C" {
::alt::IPlayer *CPlayerChangeInteriorEvent$cxxbridge1$CPlayerChangeInteriorEvent_GetTarget(::alt::CPlayerChangeInteriorEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerChangeInteriorEvent_GetTarget$)(::alt::CPlayerChangeInteriorEvent const *) = ::CPlayerChangeInteriorEvent::GetTarget;
  return CPlayerChangeInteriorEvent_GetTarget$(ptr);
}

::std::uint32_t CPlayerChangeInteriorEvent$cxxbridge1$CPlayerChangeInteriorEvent_GetOldInteriorLocation(::alt::CPlayerChangeInteriorEvent const *ptr) noexcept {
  ::std::uint32_t (*CPlayerChangeInteriorEvent_GetOldInteriorLocation$)(::alt::CPlayerChangeInteriorEvent const *) = ::CPlayerChangeInteriorEvent::GetOldInteriorLocation;
  return CPlayerChangeInteriorEvent_GetOldInteriorLocation$(ptr);
}

::std::uint32_t CPlayerChangeInteriorEvent$cxxbridge1$CPlayerChangeInteriorEvent_GetNewInteriorLocation(::alt::CPlayerChangeInteriorEvent const *ptr) noexcept {
  ::std::uint32_t (*CPlayerChangeInteriorEvent_GetNewInteriorLocation$)(::alt::CPlayerChangeInteriorEvent const *) = ::CPlayerChangeInteriorEvent::GetNewInteriorLocation;
  return CPlayerChangeInteriorEvent_GetNewInteriorLocation$(ptr);
}
} // extern "C"
} // namespace CPlayerChangeInteriorEvent

namespace CConnectionQueueAddEvent {
extern "C" {
::alt::IConnectionInfo *CConnectionQueueAddEvent$cxxbridge1$CConnectionQueueAddEvent_GetConnectionInfo(::alt::CConnectionQueueAddEvent const *ptr) noexcept {
  ::alt::IConnectionInfo *(*CConnectionQueueAddEvent_GetConnectionInfo$)(::alt::CConnectionQueueAddEvent const *) = ::CConnectionQueueAddEvent::GetConnectionInfo;
  return CConnectionQueueAddEvent_GetConnectionInfo$(ptr);
}
} // extern "C"
} // namespace CConnectionQueueAddEvent

namespace CConnectionQueueRemoveEvent {
extern "C" {
::alt::IConnectionInfo *CConnectionQueueRemoveEvent$cxxbridge1$CConnectionQueueRemoveEvent_GetConnectionInfo(::alt::CConnectionQueueRemoveEvent const *ptr) noexcept {
  ::alt::IConnectionInfo *(*CConnectionQueueRemoveEvent_GetConnectionInfo$)(::alt::CConnectionQueueRemoveEvent const *) = ::CConnectionQueueRemoveEvent::GetConnectionInfo;
  return CConnectionQueueRemoveEvent_GetConnectionInfo$(ptr);
}
} // extern "C"
} // namespace CConnectionQueueRemoveEvent

namespace CPlayerHealEvent {
extern "C" {
::alt::IPlayer *CPlayerHealEvent$cxxbridge1$CPlayerHealEvent_GetTarget(::alt::CPlayerHealEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CPlayerHealEvent_GetTarget$)(::alt::CPlayerHealEvent const *) = ::CPlayerHealEvent::GetTarget;
  return CPlayerHealEvent_GetTarget$(ptr);
}

::std::uint16_t CPlayerHealEvent$cxxbridge1$CPlayerHealEvent_GetOldHealth(::alt::CPlayerHealEvent const *ptr) noexcept {
  ::std::uint16_t (*CPlayerHealEvent_GetOldHealth$)(::alt::CPlayerHealEvent const *) = ::CPlayerHealEvent::GetOldHealth;
  return CPlayerHealEvent_GetOldHealth$(ptr);
}

::std::uint16_t CPlayerHealEvent$cxxbridge1$CPlayerHealEvent_GetNewHealth(::alt::CPlayerHealEvent const *ptr) noexcept {
  ::std::uint16_t (*CPlayerHealEvent_GetNewHealth$)(::alt::CPlayerHealEvent const *) = ::CPlayerHealEvent::GetNewHealth;
  return CPlayerHealEvent_GetNewHealth$(ptr);
}

::std::uint16_t CPlayerHealEvent$cxxbridge1$CPlayerHealEvent_GetOldArmour(::alt::CPlayerHealEvent const *ptr) noexcept {
  ::std::uint16_t (*CPlayerHealEvent_GetOldArmour$)(::alt::CPlayerHealEvent const *) = ::CPlayerHealEvent::GetOldArmour;
  return CPlayerHealEvent_GetOldArmour$(ptr);
}

::std::uint16_t CPlayerHealEvent$cxxbridge1$CPlayerHealEvent_GetNewArmour(::alt::CPlayerHealEvent const *ptr) noexcept {
  ::std::uint16_t (*CPlayerHealEvent_GetNewArmour$)(::alt::CPlayerHealEvent const *) = ::CPlayerHealEvent::GetNewArmour;
  return CPlayerHealEvent_GetNewArmour$(ptr);
}
} // extern "C"
} // namespace CPlayerHealEvent

namespace CVehicleAttachEvent {
extern "C" {
::alt::IVehicle *CVehicleAttachEvent$cxxbridge1$CVehicleAttachEvent_GetTarget(::alt::CVehicleAttachEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CVehicleAttachEvent_GetTarget$)(::alt::CVehicleAttachEvent const *) = ::CVehicleAttachEvent::GetTarget;
  return CVehicleAttachEvent_GetTarget$(ptr);
}

::alt::IVehicle *CVehicleAttachEvent$cxxbridge1$CVehicleAttachEvent_GetAttached(::alt::CVehicleAttachEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CVehicleAttachEvent_GetAttached$)(::alt::CVehicleAttachEvent const *) = ::CVehicleAttachEvent::GetAttached;
  return CVehicleAttachEvent_GetAttached$(ptr);
}
} // extern "C"
} // namespace CVehicleAttachEvent

namespace CVehicleDetachEvent {
extern "C" {
::alt::IVehicle *CVehicleDetachEvent$cxxbridge1$CVehicleDetachEvent_GetTarget(::alt::CVehicleDetachEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CVehicleDetachEvent_GetTarget$)(::alt::CVehicleDetachEvent const *) = ::CVehicleDetachEvent::GetTarget;
  return CVehicleDetachEvent_GetTarget$(ptr);
}

::alt::IVehicle *CVehicleDetachEvent$cxxbridge1$CVehicleDetachEvent_GetDetached(::alt::CVehicleDetachEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CVehicleDetachEvent_GetDetached$)(::alt::CVehicleDetachEvent const *) = ::CVehicleDetachEvent::GetDetached;
  return CVehicleDetachEvent_GetDetached$(ptr);
}
} // extern "C"
} // namespace CVehicleDetachEvent

namespace CVehicleDestroyEvent {
extern "C" {
::alt::IVehicle *CVehicleDestroyEvent$cxxbridge1$CVehicleDestroyEvent_GetTarget(::alt::CVehicleDestroyEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CVehicleDestroyEvent_GetTarget$)(::alt::CVehicleDestroyEvent const *) = ::CVehicleDestroyEvent::GetTarget;
  return CVehicleDestroyEvent_GetTarget$(ptr);
}
} // extern "C"
} // namespace CVehicleDestroyEvent

namespace CVehicleDamageEvent {
extern "C" {
::alt::IVehicle *CVehicleDamageEvent$cxxbridge1$CVehicleDamageEvent_GetTarget(::alt::CVehicleDamageEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CVehicleDamageEvent_GetTarget$)(::alt::CVehicleDamageEvent const *) = ::CVehicleDamageEvent::GetTarget;
  return CVehicleDamageEvent_GetTarget$(ptr);
}

::alt::IEntity *CVehicleDamageEvent$cxxbridge1$CVehicleDamageEvent_GetDamager(::alt::CVehicleDamageEvent const *ptr) noexcept {
  ::alt::IEntity *(*CVehicleDamageEvent_GetDamager$)(::alt::CVehicleDamageEvent const *) = ::CVehicleDamageEvent::GetDamager;
  return CVehicleDamageEvent_GetDamager$(ptr);
}

::std::uint32_t CVehicleDamageEvent$cxxbridge1$CVehicleDamageEvent_GetBodyHealthDamage(::alt::CVehicleDamageEvent const *ptr) noexcept {
  ::std::uint32_t (*CVehicleDamageEvent_GetBodyHealthDamage$)(::alt::CVehicleDamageEvent const *) = ::CVehicleDamageEvent::GetBodyHealthDamage;
  return CVehicleDamageEvent_GetBodyHealthDamage$(ptr);
}

::std::uint32_t CVehicleDamageEvent$cxxbridge1$CVehicleDamageEvent_GetBodyAdditionalHealthDamage(::alt::CVehicleDamageEvent const *ptr) noexcept {
  ::std::uint32_t (*CVehicleDamageEvent_GetBodyAdditionalHealthDamage$)(::alt::CVehicleDamageEvent const *) = ::CVehicleDamageEvent::GetBodyAdditionalHealthDamage;
  return CVehicleDamageEvent_GetBodyAdditionalHealthDamage$(ptr);
}

::std::uint32_t CVehicleDamageEvent$cxxbridge1$CVehicleDamageEvent_GetEngineHealthDamage(::alt::CVehicleDamageEvent const *ptr) noexcept {
  ::std::uint32_t (*CVehicleDamageEvent_GetEngineHealthDamage$)(::alt::CVehicleDamageEvent const *) = ::CVehicleDamageEvent::GetEngineHealthDamage;
  return CVehicleDamageEvent_GetEngineHealthDamage$(ptr);
}

::std::uint32_t CVehicleDamageEvent$cxxbridge1$CVehicleDamageEvent_GetPetrolTankHealthDamage(::alt::CVehicleDamageEvent const *ptr) noexcept {
  ::std::uint32_t (*CVehicleDamageEvent_GetPetrolTankHealthDamage$)(::alt::CVehicleDamageEvent const *) = ::CVehicleDamageEvent::GetPetrolTankHealthDamage;
  return CVehicleDamageEvent_GetPetrolTankHealthDamage$(ptr);
}

::std::uint32_t CVehicleDamageEvent$cxxbridge1$CVehicleDamageEvent_GetDamagedWith(::alt::CVehicleDamageEvent const *ptr) noexcept {
  ::std::uint32_t (*CVehicleDamageEvent_GetDamagedWith$)(::alt::CVehicleDamageEvent const *) = ::CVehicleDamageEvent::GetDamagedWith;
  return CVehicleDamageEvent_GetDamagedWith$(ptr);
}
} // extern "C"
} // namespace CVehicleDamageEvent

namespace CVehicleHornEvent {
extern "C" {
::alt::IVehicle *CVehicleHornEvent$cxxbridge1$CVehicleHornEvent_GetTarget(::alt::CVehicleHornEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CVehicleHornEvent_GetTarget$)(::alt::CVehicleHornEvent const *) = ::CVehicleHornEvent::GetTarget;
  return CVehicleHornEvent_GetTarget$(ptr);
}

::alt::IPlayer *CVehicleHornEvent$cxxbridge1$CVehicleHornEvent_GetReporter(::alt::CVehicleHornEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CVehicleHornEvent_GetReporter$)(::alt::CVehicleHornEvent const *) = ::CVehicleHornEvent::GetReporter;
  return CVehicleHornEvent_GetReporter$(ptr);
}

bool CVehicleHornEvent$cxxbridge1$CVehicleHornEvent_GetToggle(::alt::CVehicleHornEvent const *ptr) noexcept {
  bool (*CVehicleHornEvent_GetToggle$)(::alt::CVehicleHornEvent const *) = ::CVehicleHornEvent::GetToggle;
  return CVehicleHornEvent_GetToggle$(ptr);
}
} // extern "C"
} // namespace CVehicleHornEvent

namespace CVehicleSirenEvent {
extern "C" {
::alt::IVehicle *CVehicleSirenEvent$cxxbridge1$CVehicleSirenEvent_GetTarget(::alt::CVehicleSirenEvent const *ptr) noexcept {
  ::alt::IVehicle *(*CVehicleSirenEvent_GetTarget$)(::alt::CVehicleSirenEvent const *) = ::CVehicleSirenEvent::GetTarget;
  return CVehicleSirenEvent_GetTarget$(ptr);
}

bool CVehicleSirenEvent$cxxbridge1$CVehicleSirenEvent_GetToggle(::alt::CVehicleSirenEvent const *ptr) noexcept {
  bool (*CVehicleSirenEvent_GetToggle$)(::alt::CVehicleSirenEvent const *) = ::CVehicleSirenEvent::GetToggle;
  return CVehicleSirenEvent_GetToggle$(ptr);
}
} // extern "C"
} // namespace CVehicleSirenEvent

namespace CStartProjectileEvent {
extern "C" {
::alt::IPlayer *CStartProjectileEvent$cxxbridge1$CStartProjectileEvent_GetSource(::alt::CStartProjectileEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CStartProjectileEvent_GetSource$)(::alt::CStartProjectileEvent const *) = ::CStartProjectileEvent::GetSource;
  return CStartProjectileEvent_GetSource$(ptr);
}
} // extern "C"
} // namespace CStartProjectileEvent

extern "C" {
void cxxbridge1$CStartProjectileEvent_GetStartPosition_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStartProjectileEvent const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*CStartProjectileEvent_GetStartPosition_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStartProjectileEvent const *, ::Vector3Wrapper *) = ::CStartProjectileEvent_GetStartPosition_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStartProjectileEvent_GetStartPosition_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$CStartProjectileEvent_GetDirection_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStartProjectileEvent const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*CStartProjectileEvent_GetDirection_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStartProjectileEvent const *, ::Vector3Wrapper *) = ::CStartProjectileEvent_GetDirection_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStartProjectileEvent_GetDirection_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace CStartProjectileEvent {
extern "C" {
::std::uint32_t CStartProjectileEvent$cxxbridge1$CStartProjectileEvent_GetAmmoHash(::alt::CStartProjectileEvent const *ptr) noexcept {
  ::std::uint32_t (*CStartProjectileEvent_GetAmmoHash$)(::alt::CStartProjectileEvent const *) = ::CStartProjectileEvent::GetAmmoHash;
  return CStartProjectileEvent_GetAmmoHash$(ptr);
}

::std::uint32_t CStartProjectileEvent$cxxbridge1$CStartProjectileEvent_GetWeaponHash(::alt::CStartProjectileEvent const *ptr) noexcept {
  ::std::uint32_t (*CStartProjectileEvent_GetWeaponHash$)(::alt::CStartProjectileEvent const *) = ::CStartProjectileEvent::GetWeaponHash;
  return CStartProjectileEvent_GetWeaponHash$(ptr);
}
} // extern "C"
} // namespace CStartProjectileEvent

namespace CFireEvent {
extern "C" {
::alt::IPlayer *CFireEvent$cxxbridge1$CFireEvent_GetSource(::alt::CFireEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CFireEvent_GetSource$)(::alt::CFireEvent const *) = ::CFireEvent::GetSource;
  return CFireEvent_GetSource$(ptr);
}
} // extern "C"
} // namespace CFireEvent

extern "C" {
::std::vector<::FireInfoWrapper> *cxxbridge1$CFireEvent_GetFires_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CFireEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::FireInfoWrapper>> (*CFireEvent_GetFires_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CFireEvent const *) = ::CFireEvent_GetFires_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CFireEvent_GetFires_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace CExplosionEvent {
extern "C" {
::alt::IPlayer *CExplosionEvent$cxxbridge1$CExplosionEvent_GetSource(::alt::CExplosionEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CExplosionEvent_GetSource$)(::alt::CExplosionEvent const *) = ::CExplosionEvent::GetSource;
  return CExplosionEvent_GetSource$(ptr);
}

::alt::IEntity *CExplosionEvent$cxxbridge1$CExplosionEvent_GetTarget(::alt::CExplosionEvent const *ptr) noexcept {
  ::alt::IEntity *(*CExplosionEvent_GetTarget$)(::alt::CExplosionEvent const *) = ::CExplosionEvent::GetTarget;
  return CExplosionEvent_GetTarget$(ptr);
}

::std::int8_t CExplosionEvent$cxxbridge1$CExplosionEvent_GetExplosionType(::alt::CExplosionEvent const *ptr) noexcept {
  ::std::int8_t (*CExplosionEvent_GetExplosionType$)(::alt::CExplosionEvent const *) = ::CExplosionEvent::GetExplosionType;
  return CExplosionEvent_GetExplosionType$(ptr);
}
} // extern "C"
} // namespace CExplosionEvent

extern "C" {
void cxxbridge1$CExplosionEvent_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CExplosionEvent const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*CExplosionEvent_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CExplosionEvent const *, ::Vector3Wrapper *) = ::CExplosionEvent_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a;
  CExplosionEvent_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace CExplosionEvent {
extern "C" {
::std::uint32_t CExplosionEvent$cxxbridge1$CExplosionEvent_GetExplosionFX(::alt::CExplosionEvent const *ptr) noexcept {
  ::std::uint32_t (*CExplosionEvent_GetExplosionFX$)(::alt::CExplosionEvent const *) = ::CExplosionEvent::GetExplosionFX;
  return CExplosionEvent_GetExplosionFX$(ptr);
}
} // extern "C"
} // namespace CExplosionEvent

extern "C" {
::std::string *cxxbridge1$IConnectionInfo_GetName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IConnectionInfo_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo const *) = ::IConnectionInfo_GetName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IConnectionInfo_GetName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IConnectionInfo {
extern "C" {
::std::uint64_t IConnectionInfo$cxxbridge1$IConnectionInfo_GetSocialId(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::uint64_t (*IConnectionInfo_GetSocialId$)(::alt::IConnectionInfo const *) = ::IConnectionInfo::GetSocialId;
  return IConnectionInfo_GetSocialId$(ptr);
}
} // extern "C"
} // namespace IConnectionInfo

extern "C" {
::std::string *cxxbridge1$IConnectionInfo_GetSocialName_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IConnectionInfo_GetSocialName_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo const *) = ::IConnectionInfo_GetSocialName_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IConnectionInfo_GetSocialName_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IConnectionInfo {
extern "C" {
::std::uint64_t IConnectionInfo$cxxbridge1$IConnectionInfo_GetHwIdHash(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::uint64_t (*IConnectionInfo_GetHwIdHash$)(::alt::IConnectionInfo const *) = ::IConnectionInfo::GetHwIdHash;
  return IConnectionInfo_GetHwIdHash$(ptr);
}

::std::uint64_t IConnectionInfo$cxxbridge1$IConnectionInfo_GetHwIdExHash(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::uint64_t (*IConnectionInfo_GetHwIdExHash$)(::alt::IConnectionInfo const *) = ::IConnectionInfo::GetHwIdExHash;
  return IConnectionInfo_GetHwIdExHash$(ptr);
}
} // extern "C"
} // namespace IConnectionInfo

extern "C" {
::std::string *cxxbridge1$IConnectionInfo_GetHwid3_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IConnectionInfo_GetHwid3_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo const *) = ::IConnectionInfo_GetHwid3_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IConnectionInfo_GetHwid3_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$IConnectionInfo_GetAuthToken_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IConnectionInfo_GetAuthToken_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo const *) = ::IConnectionInfo_GetAuthToken_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IConnectionInfo_GetAuthToken_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IConnectionInfo {
extern "C" {
bool IConnectionInfo$cxxbridge1$IConnectionInfo_GetIsDebug(::alt::IConnectionInfo const *ptr) noexcept {
  bool (*IConnectionInfo_GetIsDebug$)(::alt::IConnectionInfo const *) = ::IConnectionInfo::GetIsDebug;
  return IConnectionInfo_GetIsDebug$(ptr);
}
} // extern "C"
} // namespace IConnectionInfo

extern "C" {
::std::string *cxxbridge1$IConnectionInfo_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IConnectionInfo_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo const *) = ::IConnectionInfo_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IConnectionInfo_GetBranch_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IConnectionInfo {
extern "C" {
::std::uint16_t IConnectionInfo$cxxbridge1$IConnectionInfo_GetVersionMajor(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::uint16_t (*IConnectionInfo_GetVersionMajor$)(::alt::IConnectionInfo const *) = ::IConnectionInfo::GetVersionMajor;
  return IConnectionInfo_GetVersionMajor$(ptr);
}

::std::uint16_t IConnectionInfo$cxxbridge1$IConnectionInfo_GetVersionMinor(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::uint16_t (*IConnectionInfo_GetVersionMinor$)(::alt::IConnectionInfo const *) = ::IConnectionInfo::GetVersionMinor;
  return IConnectionInfo_GetVersionMinor$(ptr);
}
} // extern "C"
} // namespace IConnectionInfo

extern "C" {
::std::string *cxxbridge1$IConnectionInfo_GetCdnUrl_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IConnectionInfo_GetCdnUrl_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo const *) = ::IConnectionInfo_GetCdnUrl_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IConnectionInfo_GetCdnUrl_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IConnectionInfo {
extern "C" {
::std::uint64_t IConnectionInfo$cxxbridge1$IConnectionInfo_GetPasswordHash(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::uint64_t (*IConnectionInfo_GetPasswordHash$)(::alt::IConnectionInfo const *) = ::IConnectionInfo::GetPasswordHash;
  return IConnectionInfo_GetPasswordHash$(ptr);
}
} // extern "C"
} // namespace IConnectionInfo

extern "C" {
::std::string *cxxbridge1$IConnectionInfo_GetIp_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IConnectionInfo_GetIp_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo const *) = ::IConnectionInfo_GetIp_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IConnectionInfo_GetIp_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IConnectionInfo {
extern "C" {
::std::int64_t IConnectionInfo$cxxbridge1$IConnectionInfo_GetDiscordUserID(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::int64_t (*IConnectionInfo_GetDiscordUserID$)(::alt::IConnectionInfo const *) = ::IConnectionInfo::GetDiscordUserID;
  return IConnectionInfo_GetDiscordUserID$(ptr);
}
} // extern "C"
} // namespace IConnectionInfo

extern "C" {
::std::string *cxxbridge1$IConnectionInfo_GetText_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IConnectionInfo_GetText_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo const *) = ::IConnectionInfo_GetText_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IConnectionInfo_GetText_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$IConnectionInfo_GetCloudID_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*IConnectionInfo_GetCloudID_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo const *) = ::IConnectionInfo_GetCloudID_autocxx_wrapper_0xd5d0abec981e3e3a;
  return IConnectionInfo_GetCloudID_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}
} // extern "C"

namespace IConnectionInfo {
extern "C" {
::std::uint8_t IConnectionInfo$cxxbridge1$IConnectionInfo_GetCloudAuthResult(::alt::IConnectionInfo const *ptr) noexcept {
  ::std::uint8_t (*IConnectionInfo_GetCloudAuthResult$)(::alt::IConnectionInfo const *) = ::IConnectionInfo::GetCloudAuthResult;
  return IConnectionInfo_GetCloudAuthResult$(ptr);
}

void IConnectionInfo$cxxbridge1$IConnectionInfo_Accept(::alt::IConnectionInfo *ptr, bool sendNames) noexcept {
  void (*IConnectionInfo_Accept$)(::alt::IConnectionInfo *, bool) = ::IConnectionInfo::Accept;
  IConnectionInfo_Accept$(ptr, sendNames);
}
} // extern "C"
} // namespace IConnectionInfo

extern "C" {
void cxxbridge1$IConnectionInfo_Decline_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo *ptr, ::std::string *reason) noexcept {
  void (*IConnectionInfo_Decline_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo *, ::std::unique_ptr<::std::string>) = ::IConnectionInfo_Decline_autocxx_wrapper_0xd5d0abec981e3e3a;
  IConnectionInfo_Decline_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(reason));
}
} // extern "C"

namespace IConnectionInfo {
extern "C" {
bool IConnectionInfo$cxxbridge1$IConnectionInfo_IsAccepted(::alt::IConnectionInfo const *ptr) noexcept {
  bool (*IConnectionInfo_IsAccepted$)(::alt::IConnectionInfo const *) = ::IConnectionInfo::IsAccepted;
  return IConnectionInfo_IsAccepted$(ptr);
}
} // extern "C"
} // namespace IConnectionInfo

extern "C" {
void cxxbridge1$IConnectionInfo_SetText_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IConnectionInfo *ptr, ::std::string *text) noexcept {
  void (*IConnectionInfo_SetText_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IConnectionInfo *, ::std::unique_ptr<::std::string>) = ::IConnectionInfo_SetText_autocxx_wrapper_0xd5d0abec981e3e3a;
  IConnectionInfo_SetText_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, ::std::unique_ptr<::std::string>(text));
}
} // extern "C"

namespace VehicleModelInfo {
extern "C" {
bool VehicleModelInfo$cxxbridge1$VehicleModelInfo_DoesExtraExist(::alt::VehicleModelInfo const *ptr, ::std::uint8_t extraId) noexcept {
  bool (*VehicleModelInfo_DoesExtraExist$)(::alt::VehicleModelInfo const *, ::std::uint8_t) = ::VehicleModelInfo::DoesExtraExist;
  return VehicleModelInfo_DoesExtraExist$(ptr, extraId);
}

bool VehicleModelInfo$cxxbridge1$VehicleModelInfo_DoesExtraDefault(::alt::VehicleModelInfo const *ptr, ::std::uint8_t extraId) noexcept {
  bool (*VehicleModelInfo_DoesExtraDefault$)(::alt::VehicleModelInfo const *, ::std::uint8_t) = ::VehicleModelInfo::DoesExtraDefault;
  return VehicleModelInfo_DoesExtraDefault$(ptr, extraId);
}
} // extern "C"
} // namespace VehicleModelInfo

namespace CMetaChangeEvent {
extern "C" {
::alt::IBaseObject *CMetaChangeEvent$cxxbridge1$CMetaChangeEvent_GetTarget(::alt::CMetaChangeEvent const *ptr) noexcept {
  ::alt::IBaseObject *(*CMetaChangeEvent_GetTarget$)(::alt::CMetaChangeEvent const *) = ::CMetaChangeEvent::GetTarget;
  return CMetaChangeEvent_GetTarget$(ptr);
}
} // extern "C"
} // namespace CMetaChangeEvent

extern "C" {
::std::string *cxxbridge1$CMetaChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CMetaChangeEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CMetaChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CMetaChangeEvent const *) = ::CMetaChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CMetaChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$CMetaChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CMetaChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CMetaChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CMetaChangeEvent const *, ::ConstMValueWrapper *) = ::CMetaChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CMetaChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$CMetaChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CMetaChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CMetaChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CMetaChangeEvent const *, ::ConstMValueWrapper *) = ::CMetaChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CMetaChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

::std::string *cxxbridge1$CGlobalMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGlobalMetaDataChangeEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CGlobalMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGlobalMetaDataChangeEvent const *) = ::CGlobalMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CGlobalMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$CGlobalMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGlobalMetaDataChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CGlobalMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGlobalMetaDataChangeEvent const *, ::ConstMValueWrapper *) = ::CGlobalMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CGlobalMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$CGlobalMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGlobalMetaDataChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CGlobalMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGlobalMetaDataChangeEvent const *, ::ConstMValueWrapper *) = ::CGlobalMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CGlobalMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

::std::string *cxxbridge1$CGlobalSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGlobalSyncedMetaDataChangeEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CGlobalSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGlobalSyncedMetaDataChangeEvent const *) = ::CGlobalSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CGlobalSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$CGlobalSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGlobalSyncedMetaDataChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CGlobalSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGlobalSyncedMetaDataChangeEvent const *, ::ConstMValueWrapper *) = ::CGlobalSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CGlobalSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$CGlobalSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGlobalSyncedMetaDataChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CGlobalSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGlobalSyncedMetaDataChangeEvent const *, ::ConstMValueWrapper *) = ::CGlobalSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CGlobalSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace CSyncedMetaDataChangeEvent {
extern "C" {
::alt::IBaseObject *CSyncedMetaDataChangeEvent$cxxbridge1$CSyncedMetaDataChangeEvent_GetTarget(::alt::CSyncedMetaDataChangeEvent const *ptr) noexcept {
  ::alt::IBaseObject *(*CSyncedMetaDataChangeEvent_GetTarget$)(::alt::CSyncedMetaDataChangeEvent const *) = ::CSyncedMetaDataChangeEvent::GetTarget;
  return CSyncedMetaDataChangeEvent_GetTarget$(ptr);
}
} // extern "C"
} // namespace CSyncedMetaDataChangeEvent

extern "C" {
::std::string *cxxbridge1$CSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CSyncedMetaDataChangeEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CSyncedMetaDataChangeEvent const *) = ::CSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$CSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CSyncedMetaDataChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CSyncedMetaDataChangeEvent const *, ::ConstMValueWrapper *) = ::CSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$CSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CSyncedMetaDataChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CSyncedMetaDataChangeEvent const *, ::ConstMValueWrapper *) = ::CSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace CStreamSyncedMetaDataChangeEvent {
extern "C" {
::alt::IBaseObject *CStreamSyncedMetaDataChangeEvent$cxxbridge1$CStreamSyncedMetaDataChangeEvent_GetTarget(::alt::CStreamSyncedMetaDataChangeEvent const *ptr) noexcept {
  ::alt::IBaseObject *(*CStreamSyncedMetaDataChangeEvent_GetTarget$)(::alt::CStreamSyncedMetaDataChangeEvent const *) = ::CStreamSyncedMetaDataChangeEvent::GetTarget;
  return CStreamSyncedMetaDataChangeEvent_GetTarget$(ptr);
}
} // extern "C"
} // namespace CStreamSyncedMetaDataChangeEvent

extern "C" {
::std::string *cxxbridge1$CStreamSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStreamSyncedMetaDataChangeEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CStreamSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStreamSyncedMetaDataChangeEvent const *) = ::CStreamSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CStreamSyncedMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$CStreamSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStreamSyncedMetaDataChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CStreamSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStreamSyncedMetaDataChangeEvent const *, ::ConstMValueWrapper *) = ::CStreamSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStreamSyncedMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$CStreamSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStreamSyncedMetaDataChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CStreamSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStreamSyncedMetaDataChangeEvent const *, ::ConstMValueWrapper *) = ::CStreamSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStreamSyncedMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace CLocalMetaDataChangeEvent {
extern "C" {
::alt::IPlayer *CLocalMetaDataChangeEvent$cxxbridge1$CLocalMetaDataChangeEvent_GetTarget(::alt::CLocalMetaDataChangeEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CLocalMetaDataChangeEvent_GetTarget$)(::alt::CLocalMetaDataChangeEvent const *) = ::CLocalMetaDataChangeEvent::GetTarget;
  return CLocalMetaDataChangeEvent_GetTarget$(ptr);
}
} // extern "C"
} // namespace CLocalMetaDataChangeEvent

extern "C" {
::std::string *cxxbridge1$CLocalMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CLocalMetaDataChangeEvent const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*CLocalMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CLocalMetaDataChangeEvent const *) = ::CLocalMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CLocalMetaDataChangeEvent_GetKey_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$CLocalMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CLocalMetaDataChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CLocalMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CLocalMetaDataChangeEvent const *, ::ConstMValueWrapper *) = ::CLocalMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CLocalMetaDataChangeEvent_GetVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$CLocalMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CLocalMetaDataChangeEvent const *ptr, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*CLocalMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CLocalMetaDataChangeEvent const *, ::ConstMValueWrapper *) = ::CLocalMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a;
  CLocalMetaDataChangeEvent_GetOldVal_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace CResourceStopEvent {
extern "C" {
::alt::IResource *CResourceStopEvent$cxxbridge1$CResourceStopEvent_GetResource(::alt::CResourceStopEvent const *ptr) noexcept {
  ::alt::IResource *(*CResourceStopEvent_GetResource$)(::alt::CResourceStopEvent const *) = ::CResourceStopEvent::GetResource;
  return CResourceStopEvent_GetResource$(ptr);
}
} // extern "C"
} // namespace CResourceStopEvent

namespace CResourceStartEvent {
extern "C" {
::alt::IResource *CResourceStartEvent$cxxbridge1$CResourceStartEvent_GetResource(::alt::CResourceStartEvent const *ptr) noexcept {
  ::alt::IResource *(*CResourceStartEvent_GetResource$)(::alt::CResourceStartEvent const *) = ::CResourceStartEvent::GetResource;
  return CResourceStartEvent_GetResource$(ptr);
}
} // extern "C"
} // namespace CResourceStartEvent

namespace CVoiceConnectionEvent {
extern "C" {
::std::uint8_t CVoiceConnectionEvent$cxxbridge1$CVoiceConnectionEvent_GetState(::alt::CVoiceConnectionEvent const *ptr) noexcept {
  ::std::uint8_t (*CVoiceConnectionEvent_GetState$)(::alt::CVoiceConnectionEvent const *) = ::CVoiceConnectionEvent::GetState;
  return CVoiceConnectionEvent_GetState$(ptr);
}
} // extern "C"
} // namespace CVoiceConnectionEvent

namespace CRequestSyncedSceneEvent {
extern "C" {
::alt::IPlayer *CRequestSyncedSceneEvent$cxxbridge1$CRequestSyncedSceneEvent_GetSource(::alt::CRequestSyncedSceneEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CRequestSyncedSceneEvent_GetSource$)(::alt::CRequestSyncedSceneEvent const *) = ::CRequestSyncedSceneEvent::GetSource;
  return CRequestSyncedSceneEvent_GetSource$(ptr);
}

::std::int32_t CRequestSyncedSceneEvent$cxxbridge1$CRequestSyncedSceneEvent_GetSceneID(::alt::CRequestSyncedSceneEvent const *ptr) noexcept {
  ::std::int32_t (*CRequestSyncedSceneEvent_GetSceneID$)(::alt::CRequestSyncedSceneEvent const *) = ::CRequestSyncedSceneEvent::GetSceneID;
  return CRequestSyncedSceneEvent_GetSceneID$(ptr);
}
} // extern "C"
} // namespace CRequestSyncedSceneEvent

namespace CStartSyncedSceneEvent {
extern "C" {
::alt::IPlayer *CStartSyncedSceneEvent$cxxbridge1$CStartSyncedSceneEvent_GetSource(::alt::CStartSyncedSceneEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CStartSyncedSceneEvent_GetSource$)(::alt::CStartSyncedSceneEvent const *) = ::CStartSyncedSceneEvent::GetSource;
  return CStartSyncedSceneEvent_GetSource$(ptr);
}

::std::int32_t CStartSyncedSceneEvent$cxxbridge1$CStartSyncedSceneEvent_GetSceneID(::alt::CStartSyncedSceneEvent const *ptr) noexcept {
  ::std::int32_t (*CStartSyncedSceneEvent_GetSceneID$)(::alt::CStartSyncedSceneEvent const *) = ::CStartSyncedSceneEvent::GetSceneID;
  return CStartSyncedSceneEvent_GetSceneID$(ptr);
}
} // extern "C"
} // namespace CStartSyncedSceneEvent

extern "C" {
void cxxbridge1$CStartSyncedSceneEvent_GetStartPosition_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStartSyncedSceneEvent const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*CStartSyncedSceneEvent_GetStartPosition_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStartSyncedSceneEvent const *, ::Vector3Wrapper *) = ::CStartSyncedSceneEvent_GetStartPosition_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStartSyncedSceneEvent_GetStartPosition_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}

void cxxbridge1$CStartSyncedSceneEvent_GetStartRotation_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStartSyncedSceneEvent const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*CStartSyncedSceneEvent_GetStartRotation_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStartSyncedSceneEvent const *, ::Vector3Wrapper *) = ::CStartSyncedSceneEvent_GetStartRotation_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStartSyncedSceneEvent_GetStartRotation_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace CStartSyncedSceneEvent {
extern "C" {
::std::uint32_t CStartSyncedSceneEvent$cxxbridge1$CStartSyncedSceneEvent_GetAnimDictHash(::alt::CStartSyncedSceneEvent const *ptr) noexcept {
  ::std::uint32_t (*CStartSyncedSceneEvent_GetAnimDictHash$)(::alt::CStartSyncedSceneEvent const *) = ::CStartSyncedSceneEvent::GetAnimDictHash;
  return CStartSyncedSceneEvent_GetAnimDictHash$(ptr);
}
} // extern "C"
} // namespace CStartSyncedSceneEvent

extern "C" {
void cxxbridge1$CStartSyncedSceneEvent_GetEntityAndAnimHashPairs_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStartSyncedSceneEvent const *ptr, ::EntityAnimHashPairsWrapper *placement_return_type) noexcept {
  void (*CStartSyncedSceneEvent_GetEntityAndAnimHashPairs_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStartSyncedSceneEvent const *, ::EntityAnimHashPairsWrapper *) = ::CStartSyncedSceneEvent_GetEntityAndAnimHashPairs_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStartSyncedSceneEvent_GetEntityAndAnimHashPairs_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace CStopSyncedSceneEvent {
extern "C" {
::alt::IPlayer *CStopSyncedSceneEvent$cxxbridge1$CStopSyncedSceneEvent_GetSource(::alt::CStopSyncedSceneEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CStopSyncedSceneEvent_GetSource$)(::alt::CStopSyncedSceneEvent const *) = ::CStopSyncedSceneEvent::GetSource;
  return CStopSyncedSceneEvent_GetSource$(ptr);
}

::std::int32_t CStopSyncedSceneEvent$cxxbridge1$CStopSyncedSceneEvent_GetSceneID(::alt::CStopSyncedSceneEvent const *ptr) noexcept {
  ::std::int32_t (*CStopSyncedSceneEvent_GetSceneID$)(::alt::CStopSyncedSceneEvent const *) = ::CStopSyncedSceneEvent::GetSceneID;
  return CStopSyncedSceneEvent_GetSceneID$(ptr);
}
} // extern "C"
} // namespace CStopSyncedSceneEvent

namespace CUpdateSyncedSceneEvent {
extern "C" {
::alt::IPlayer *CUpdateSyncedSceneEvent$cxxbridge1$CUpdateSyncedSceneEvent_GetSource(::alt::CUpdateSyncedSceneEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CUpdateSyncedSceneEvent_GetSource$)(::alt::CUpdateSyncedSceneEvent const *) = ::CUpdateSyncedSceneEvent::GetSource;
  return CUpdateSyncedSceneEvent_GetSource$(ptr);
}

float CUpdateSyncedSceneEvent$cxxbridge1$CUpdateSyncedSceneEvent_GetStartRate(::alt::CUpdateSyncedSceneEvent const *ptr) noexcept {
  float (*CUpdateSyncedSceneEvent_GetStartRate$)(::alt::CUpdateSyncedSceneEvent const *) = ::CUpdateSyncedSceneEvent::GetStartRate;
  return CUpdateSyncedSceneEvent_GetStartRate$(ptr);
}

::std::int32_t CUpdateSyncedSceneEvent$cxxbridge1$CUpdateSyncedSceneEvent_GetSceneID(::alt::CUpdateSyncedSceneEvent const *ptr) noexcept {
  ::std::int32_t (*CUpdateSyncedSceneEvent_GetSceneID$)(::alt::CUpdateSyncedSceneEvent const *) = ::CUpdateSyncedSceneEvent::GetSceneID;
  return CUpdateSyncedSceneEvent_GetSceneID$(ptr);
}
} // extern "C"
} // namespace CUpdateSyncedSceneEvent

namespace CClientDeleteObjectEvent {
extern "C" {
::alt::IPlayer *CClientDeleteObjectEvent$cxxbridge1$CClientDeleteObjectEvent_GetTarget(::alt::CClientDeleteObjectEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CClientDeleteObjectEvent_GetTarget$)(::alt::CClientDeleteObjectEvent const *) = ::CClientDeleteObjectEvent::GetTarget;
  return CClientDeleteObjectEvent_GetTarget$(ptr);
}
} // extern "C"
} // namespace CClientDeleteObjectEvent

namespace CClientRequestObjectEvent {
extern "C" {
::alt::IPlayer *CClientRequestObjectEvent$cxxbridge1$CClientRequestObjectEvent_GetTarget(::alt::CClientRequestObjectEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CClientRequestObjectEvent_GetTarget$)(::alt::CClientRequestObjectEvent const *) = ::CClientRequestObjectEvent::GetTarget;
  return CClientRequestObjectEvent_GetTarget$(ptr);
}

::std::uint32_t CClientRequestObjectEvent$cxxbridge1$CClientRequestObjectEvent_GetModel(::alt::CClientRequestObjectEvent const *ptr) noexcept {
  ::std::uint32_t (*CClientRequestObjectEvent_GetModel$)(::alt::CClientRequestObjectEvent const *) = ::CClientRequestObjectEvent::GetModel;
  return CClientRequestObjectEvent_GetModel$(ptr);
}
} // extern "C"
} // namespace CClientRequestObjectEvent

extern "C" {
void cxxbridge1$CClientRequestObjectEvent_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CClientRequestObjectEvent const *ptr, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*CClientRequestObjectEvent_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CClientRequestObjectEvent const *, ::Vector3Wrapper *) = ::CClientRequestObjectEvent_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a;
  CClientRequestObjectEvent_GetPosition_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr, placement_return_type);
}
} // extern "C"

namespace CGivePedScriptedTaskEvent {
extern "C" {
::alt::IPlayer *CGivePedScriptedTaskEvent$cxxbridge1$CGivePedScriptedTaskEvent_GetSource(::alt::CGivePedScriptedTaskEvent const *ptr) noexcept {
  ::alt::IPlayer *(*CGivePedScriptedTaskEvent_GetSource$)(::alt::CGivePedScriptedTaskEvent const *) = ::CGivePedScriptedTaskEvent::GetSource;
  return CGivePedScriptedTaskEvent_GetSource$(ptr);
}

::alt::IPed *CGivePedScriptedTaskEvent$cxxbridge1$CGivePedScriptedTaskEvent_GetTarget(::alt::CGivePedScriptedTaskEvent const *ptr) noexcept {
  ::alt::IPed *(*CGivePedScriptedTaskEvent_GetTarget$)(::alt::CGivePedScriptedTaskEvent const *) = ::CGivePedScriptedTaskEvent::GetTarget;
  return CGivePedScriptedTaskEvent_GetTarget$(ptr);
}

::std::uint32_t CGivePedScriptedTaskEvent$cxxbridge1$CGivePedScriptedTaskEvent_GetTaskType(::alt::CGivePedScriptedTaskEvent const *ptr) noexcept {
  ::std::uint32_t (*CGivePedScriptedTaskEvent_GetTaskType$)(::alt::CGivePedScriptedTaskEvent const *) = ::CGivePedScriptedTaskEvent::GetTaskType;
  return CGivePedScriptedTaskEvent_GetTaskType$(ptr);
}
} // extern "C"
} // namespace CGivePedScriptedTaskEvent

namespace CPedDeathEvent {
extern "C" {
::alt::IPed *CPedDeathEvent$cxxbridge1$CPedDeathEvent_GetTarget(::alt::CPedDeathEvent const *ptr) noexcept {
  ::alt::IPed *(*CPedDeathEvent_GetTarget$)(::alt::CPedDeathEvent const *) = ::CPedDeathEvent::GetTarget;
  return CPedDeathEvent_GetTarget$(ptr);
}

::alt::IEntity *CPedDeathEvent$cxxbridge1$CPedDeathEvent_GetKiller(::alt::CPedDeathEvent const *ptr) noexcept {
  ::alt::IEntity *(*CPedDeathEvent_GetKiller$)(::alt::CPedDeathEvent const *) = ::CPedDeathEvent::GetKiller;
  return CPedDeathEvent_GetKiller$(ptr);
}

::std::uint32_t CPedDeathEvent$cxxbridge1$CPedDeathEvent_GetWeapon(::alt::CPedDeathEvent const *ptr) noexcept {
  ::std::uint32_t (*CPedDeathEvent_GetWeapon$)(::alt::CPedDeathEvent const *) = ::CPedDeathEvent::GetWeapon;
  return CPedDeathEvent_GetWeapon$(ptr);
}
} // extern "C"
} // namespace CPedDeathEvent

namespace CPedDamageEvent {
extern "C" {
::alt::IPed *CPedDamageEvent$cxxbridge1$CPedDamageEvent_GetTarget(::alt::CPedDamageEvent const *ptr) noexcept {
  ::alt::IPed *(*CPedDamageEvent_GetTarget$)(::alt::CPedDamageEvent const *) = ::CPedDamageEvent::GetTarget;
  return CPedDamageEvent_GetTarget$(ptr);
}

::alt::IEntity *CPedDamageEvent$cxxbridge1$CPedDamageEvent_GetAttacker(::alt::CPedDamageEvent const *ptr) noexcept {
  ::alt::IEntity *(*CPedDamageEvent_GetAttacker$)(::alt::CPedDamageEvent const *) = ::CPedDamageEvent::GetAttacker;
  return CPedDamageEvent_GetAttacker$(ptr);
}

::std::uint16_t CPedDamageEvent$cxxbridge1$CPedDamageEvent_GetHealthDamage(::alt::CPedDamageEvent const *ptr) noexcept {
  ::std::uint16_t (*CPedDamageEvent_GetHealthDamage$)(::alt::CPedDamageEvent const *) = ::CPedDamageEvent::GetHealthDamage;
  return CPedDamageEvent_GetHealthDamage$(ptr);
}

::std::uint16_t CPedDamageEvent$cxxbridge1$CPedDamageEvent_GetArmourDamage(::alt::CPedDamageEvent const *ptr) noexcept {
  ::std::uint16_t (*CPedDamageEvent_GetArmourDamage$)(::alt::CPedDamageEvent const *) = ::CPedDamageEvent::GetArmourDamage;
  return CPedDamageEvent_GetArmourDamage$(ptr);
}

::std::uint32_t CPedDamageEvent$cxxbridge1$CPedDamageEvent_GetWeapon(::alt::CPedDamageEvent const *ptr) noexcept {
  ::std::uint32_t (*CPedDamageEvent_GetWeapon$)(::alt::CPedDamageEvent const *) = ::CPedDamageEvent::GetWeapon;
  return CPedDamageEvent_GetWeapon$(ptr);
}
} // extern "C"
} // namespace CPedDamageEvent

namespace CPedHealEvent {
extern "C" {
::alt::IPed *CPedHealEvent$cxxbridge1$CPedHealEvent_GetTarget(::alt::CPedHealEvent const *ptr) noexcept {
  ::alt::IPed *(*CPedHealEvent_GetTarget$)(::alt::CPedHealEvent const *) = ::CPedHealEvent::GetTarget;
  return CPedHealEvent_GetTarget$(ptr);
}

::std::uint16_t CPedHealEvent$cxxbridge1$CPedHealEvent_GetOldHealth(::alt::CPedHealEvent const *ptr) noexcept {
  ::std::uint16_t (*CPedHealEvent_GetOldHealth$)(::alt::CPedHealEvent const *) = ::CPedHealEvent::GetOldHealth;
  return CPedHealEvent_GetOldHealth$(ptr);
}

::std::uint16_t CPedHealEvent$cxxbridge1$CPedHealEvent_GetNewHealth(::alt::CPedHealEvent const *ptr) noexcept {
  ::std::uint16_t (*CPedHealEvent_GetNewHealth$)(::alt::CPedHealEvent const *) = ::CPedHealEvent::GetNewHealth;
  return CPedHealEvent_GetNewHealth$(ptr);
}

::std::uint16_t CPedHealEvent$cxxbridge1$CPedHealEvent_GetOldArmour(::alt::CPedHealEvent const *ptr) noexcept {
  ::std::uint16_t (*CPedHealEvent_GetOldArmour$)(::alt::CPedHealEvent const *) = ::CPedHealEvent::GetOldArmour;
  return CPedHealEvent_GetOldArmour$(ptr);
}

::std::uint16_t CPedHealEvent$cxxbridge1$CPedHealEvent_GetNewArmour(::alt::CPedHealEvent const *ptr) noexcept {
  ::std::uint16_t (*CPedHealEvent_GetNewArmour$)(::alt::CPedHealEvent const *) = ::CPedHealEvent::GetNewArmour;
  return CPedHealEvent_GetNewArmour$(ptr);
}
} // extern "C"
} // namespace CPedHealEvent

extern "C" {
void cxxbridge1$set_alt_core(::alt::ICore *core) noexcept {
  void (*set_alt_core$)(::alt::ICore *) = ::set_alt_core;
  set_alt_core$(core);
}

::alt::ICore *cxxbridge1$get_alt_core() noexcept {
  ::alt::ICore *(*get_alt_core$)() = ::get_alt_core;
  return get_alt_core$();
}

::alt::IScriptRuntime *cxxbridge1$create_script_runtime() noexcept {
  ::alt::IScriptRuntime *(*create_script_runtime$)() = ::create_script_runtime;
  return create_script_runtime$();
}

void cxxbridge1$register_script_runtime_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::ICore *core, ::std::string *resource_type, ::alt::IScriptRuntime *runtime) noexcept {
  void (*register_script_runtime_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::ICore *, ::std::unique_ptr<::std::string>, ::alt::IScriptRuntime *) = ::register_script_runtime_autocxx_wrapper_0xd5d0abec981e3e3a;
  register_script_runtime_autocxx_wrapper_0xd5d0abec981e3e3a$(core, ::std::unique_ptr<::std::string>(resource_type), runtime);
}

void cxxbridge1$clone_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper const &autocxx_gen_this, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*clone_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper const &, ::ConstMValueWrapper *) = ::clone_autocxx_wrapper_0xd5d0abec981e3e3a;
  clone_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, placement_return_type);
}

void cxxbridge1$copy_const_mvalue_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper const &wrapper, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*copy_const_mvalue_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper const &, ::ConstMValueWrapper *) = ::copy_const_mvalue_autocxx_wrapper_0xd5d0abec981e3e3a;
  copy_const_mvalue_autocxx_wrapper_0xd5d0abec981e3e3a$(wrapper, placement_return_type);
}

void cxxbridge1$copy_mut_mvalue_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueMutWrapper const &wrapper, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*copy_mut_mvalue_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueMutWrapper const &, ::MValueMutWrapper *) = ::copy_mut_mvalue_autocxx_wrapper_0xd5d0abec981e3e3a;
  copy_mut_mvalue_autocxx_wrapper_0xd5d0abec981e3e3a$(wrapper, placement_return_type);
}

void cxxbridge1$convert_mvalue_mut_wrapper_to_const_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueMutWrapper *mut_wrapper, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*convert_mvalue_mut_wrapper_to_const_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueMutWrapper *, ::ConstMValueWrapper *) = ::convert_mvalue_mut_wrapper_to_const_autocxx_wrapper_0xd5d0abec981e3e3a;
  convert_mvalue_mut_wrapper_to_const_autocxx_wrapper_0xd5d0abec981e3e3a$(mut_wrapper, placement_return_type);
}

void cxxbridge1$copy_mvalue_dict_pair_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueDictPairWrapper const &wrapper, ::MValueDictPairWrapper *placement_return_type) noexcept {
  void (*copy_mvalue_dict_pair_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueDictPairWrapper const &, ::MValueDictPairWrapper *) = ::copy_mvalue_dict_pair_autocxx_wrapper_0xd5d0abec981e3e3a;
  copy_mvalue_dict_pair_autocxx_wrapper_0xd5d0abec981e3e3a$(wrapper, placement_return_type);
}

void cxxbridge1$ConfigDictPairWrapper_clone_autocxx_wrapper_0xd5d0abec981e3e3a(::ConfigDictPairWrapper &autocxx_gen_this, ::ConfigDictPairWrapper *placement_return_type) noexcept {
  void (*ConfigDictPairWrapper_clone_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConfigDictPairWrapper &, ::ConfigDictPairWrapper *) = ::ConfigDictPairWrapper_clone_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConfigDictPairWrapper_clone_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, placement_return_type);
}

::alt::IBaseObject *cxxbridge1$read_base_object_ptr_wrapper(::BaseObjectPtrWrapper const &wrapper) noexcept {
  ::alt::IBaseObject *(*read_base_object_ptr_wrapper$)(::BaseObjectPtrWrapper const &) = ::read_base_object_ptr_wrapper;
  return read_base_object_ptr_wrapper$(wrapper);
}

::std::vector<::BaseObjectPtrWrapper> *cxxbridge1$create_base_object_vec_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>> (*create_base_object_vec_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::create_base_object_vec_autocxx_wrapper_0xd5d0abec981e3e3a;
  return create_base_object_vec_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}

void cxxbridge1$push_to_base_object_vec(::std::vector<::BaseObjectPtrWrapper> &base_object_vec, ::alt::IBaseObject *base_object) noexcept {
  void (*push_to_base_object_vec$)(::std::vector<::BaseObjectPtrWrapper> &, ::alt::IBaseObject *) = ::push_to_base_object_vec;
  push_to_base_object_vec$(base_object_vec, base_object);
}

::alt::IPlayer *cxxbridge1$read_player_ptr_wrapper(::PlayerPtrWrapper const &wrapper) noexcept {
  ::alt::IPlayer *(*read_player_ptr_wrapper$)(::PlayerPtrWrapper const &) = ::read_player_ptr_wrapper;
  return read_player_ptr_wrapper$(wrapper);
}

::std::vector<::PlayerPtrWrapper> *cxxbridge1$create_player_vec_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::std::vector<::PlayerPtrWrapper>> (*create_player_vec_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::create_player_vec_autocxx_wrapper_0xd5d0abec981e3e3a;
  return create_player_vec_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}

void cxxbridge1$push_to_player_vec(::std::vector<::PlayerPtrWrapper> &player_vec, ::alt::IPlayer *player) noexcept {
  void (*push_to_player_vec$)(::std::vector<::PlayerPtrWrapper> &, ::alt::IPlayer *) = ::push_to_player_vec;
  push_to_player_vec$(player_vec, player);
}

::alt::IResource *cxxbridge1$read_resource_ptr_wrapper(::ResourcePtrWrapper const &wrapper) noexcept {
  ::alt::IResource *(*read_resource_ptr_wrapper$)(::ResourcePtrWrapper const &) = ::read_resource_ptr_wrapper;
  return read_resource_ptr_wrapper$(wrapper);
}

void cxxbridge1$Vector3Wrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector3Wrapper *autocxx_gen_this, float _x, float _y, float _z) noexcept {
  void (*Vector3Wrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector3Wrapper *, float, float, float) = ::Vector3Wrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector3Wrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, _x, _y, _z);
}

void cxxbridge1$read_vector3(::Vector3Wrapper const &vector3, float *out_x, float *out_y, float *out_z) noexcept {
  void (*read_vector3$)(::Vector3Wrapper const &, float *, float *, float *) = ::read_vector3;
  read_vector3$(vector3, out_x, out_y, out_z);
}

void cxxbridge1$Vector2Wrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector2Wrapper *autocxx_gen_this, float _x, float _y) noexcept {
  void (*Vector2Wrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector2Wrapper *, float, float) = ::Vector2Wrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector2Wrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, _x, _y);
}

void cxxbridge1$read_vector2(::Vector2Wrapper const &vector2, float *out_x, float *out_y) noexcept {
  void (*read_vector2$)(::Vector2Wrapper const &, float *, float *) = ::read_vector2;
  read_vector2$(vector2, out_x, out_y);
}

void cxxbridge1$create_vector2_vec_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector2Vec *placement_return_type) noexcept {
  void (*create_vector2_vec_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector2Vec *) = ::create_vector2_vec_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_vector2_vec_autocxx_wrapper_0xd5d0abec981e3e3a$(placement_return_type);
}

void cxxbridge1$push_to_vector2_vec(::Vector2Vec &vec, float x, float y) noexcept {
  void (*push_to_vector2_vec$)(::Vector2Vec &, float, float) = ::push_to_vector2_vec;
  push_to_vector2_vec$(vec, x, y);
}

void cxxbridge1$RGBAWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(::RGBAWrapper *autocxx_gen_this, ::std::uint8_t _r, ::std::uint8_t _g, ::std::uint8_t _b, ::std::uint8_t _a) noexcept {
  void (*RGBAWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$)(::RGBAWrapper *, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t) = ::RGBAWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a;
  RGBAWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, _r, _g, _b, _a);
}

void cxxbridge1$read_rgba(::RGBAWrapper const &rgba, ::std::uint8_t *out_r, ::std::uint8_t *out_g, ::std::uint8_t *out_b, ::std::uint8_t *out_a) noexcept {
  void (*read_rgba$)(::RGBAWrapper const &, ::std::uint8_t *, ::std::uint8_t *, ::std::uint8_t *, ::std::uint8_t *) = ::read_rgba;
  read_rgba$(rgba, out_r, out_g, out_b, out_a);
}

void cxxbridge1$read_weapon(::WeaponWrapper const &weapon, ::std::uint32_t *out_hash, ::std::uint8_t *out_tint_index) noexcept {
  void (*read_weapon$)(::WeaponWrapper const &, ::std::uint32_t *, ::std::uint8_t *) = ::read_weapon;
  read_weapon$(weapon, out_hash, out_tint_index);
}

::std::vector<::std::uint32_t> *cxxbridge1$read_weapon_components_autocxx_wrapper_0xd5d0abec981e3e3a(::WeaponWrapper const &weapon) noexcept {
  ::std::unique_ptr<::std::vector<::std::uint32_t>> (*read_weapon_components_autocxx_wrapper_0xd5d0abec981e3e3a$)(::WeaponWrapper const &) = ::read_weapon_components_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_weapon_components_autocxx_wrapper_0xd5d0abec981e3e3a$(weapon).release();
}

void cxxbridge1$FireInfoWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(::FireInfoWrapper *autocxx_gen_this, ::Vector3Wrapper *_pos, ::std::uint32_t _weapon_hash) noexcept {
  void (*FireInfoWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$)(::FireInfoWrapper *, ::Vector3Wrapper *, ::std::uint32_t) = ::FireInfoWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a;
  FireInfoWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, _pos, _weapon_hash);
}

void cxxbridge1$read_fire_info_pos_autocxx_wrapper_0xd5d0abec981e3e3a(::FireInfoWrapper const &fire, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*read_fire_info_pos_autocxx_wrapper_0xd5d0abec981e3e3a$)(::FireInfoWrapper const &, ::Vector3Wrapper *) = ::read_fire_info_pos_autocxx_wrapper_0xd5d0abec981e3e3a;
  read_fire_info_pos_autocxx_wrapper_0xd5d0abec981e3e3a$(fire, placement_return_type);
}

::std::uint32_t cxxbridge1$read_fire_info_weapon_hash(::FireInfoWrapper const &fire) noexcept {
  ::std::uint32_t (*read_fire_info_weapon_hash$)(::FireInfoWrapper const &) = ::read_fire_info_weapon_hash;
  return read_fire_info_weapon_hash$(fire);
}

void cxxbridge1$create_mvalue_unordered_map_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueUnorderedMapWrapper *placement_return_type) noexcept {
  void (*create_mvalue_unordered_map_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueUnorderedMapWrapper *) = ::create_mvalue_unordered_map_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_unordered_map_autocxx_wrapper_0xd5d0abec981e3e3a$(placement_return_type);
}

void cxxbridge1$push_to_mvalue_unordered_map_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueUnorderedMapWrapper &map, ::std::string *key, ::MValueMutWrapper *value) noexcept {
  void (*push_to_mvalue_unordered_map_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueUnorderedMapWrapper &, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::push_to_mvalue_unordered_map_autocxx_wrapper_0xd5d0abec981e3e3a;
  push_to_mvalue_unordered_map_autocxx_wrapper_0xd5d0abec981e3e3a$(map, ::std::unique_ptr<::std::string>(key), value);
}

::std::vector<::EntityAnimHashPairWrapper> *cxxbridge1$read_entity_anim_hash_pairs_autocxx_wrapper_0xd5d0abec981e3e3a(::EntityAnimHashPairsWrapper const &wrapper) noexcept {
  ::std::unique_ptr<::std::vector<::EntityAnimHashPairWrapper>> (*read_entity_anim_hash_pairs_autocxx_wrapper_0xd5d0abec981e3e3a$)(::EntityAnimHashPairsWrapper const &) = ::read_entity_anim_hash_pairs_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_entity_anim_hash_pairs_autocxx_wrapper_0xd5d0abec981e3e3a$(wrapper).release();
}

::alt::IEntity *cxxbridge1$read_entity_anim_hash_pair_entity(::EntityAnimHashPairWrapper const &wrapper) noexcept {
  ::alt::IEntity *(*read_entity_anim_hash_pair_entity$)(::EntityAnimHashPairWrapper const &) = ::read_entity_anim_hash_pair_entity;
  return read_entity_anim_hash_pair_entity$(wrapper);
}

::std::uint32_t cxxbridge1$read_entity_anim_hash_pair_anim_hash(::EntityAnimHashPairWrapper const &wrapper) noexcept {
  ::std::uint32_t (*read_entity_anim_hash_pair_anim_hash$)(::EntityAnimHashPairWrapper const &) = ::read_entity_anim_hash_pair_anim_hash;
  return read_entity_anim_hash_pair_anim_hash$(wrapper);
}

::alt::IEntity *cxxbridge1$read_streamed_entity_key(::StreamedEntityWrapper const &wrapper) noexcept {
  ::alt::IEntity *(*read_streamed_entity_key$)(::StreamedEntityWrapper const &) = ::read_streamed_entity_key;
  return read_streamed_entity_key$(wrapper);
}

::std::int32_t cxxbridge1$read_streamed_entity_value(::StreamedEntityWrapper const &wrapper) noexcept {
  ::std::int32_t (*read_streamed_entity_value$)(::StreamedEntityWrapper const &) = ::read_streamed_entity_value;
  return read_streamed_entity_value$(wrapper);
}

::std::vector<::ConstMValueWrapper> *cxxbridge1$create_mvalue_vec_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::std::unique_ptr<::std::vector<::ConstMValueWrapper>> (*create_mvalue_vec_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::create_mvalue_vec_autocxx_wrapper_0xd5d0abec981e3e3a;
  return create_mvalue_vec_autocxx_wrapper_0xd5d0abec981e3e3a$().release();
}

::std::uint8_t cxxbridge1$read_mvalue_type_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue) noexcept {
  ::std::uint8_t (*read_mvalue_type_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::read_mvalue_type_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_type_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue);
}

bool cxxbridge1$read_mvalue_bool_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue) noexcept {
  bool (*read_mvalue_bool_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::read_mvalue_bool_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_bool_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue);
}

double cxxbridge1$read_mvalue_double_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue) noexcept {
  double (*read_mvalue_double_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::read_mvalue_double_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_double_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue);
}

::std::string *cxxbridge1$read_mvalue_string_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue) noexcept {
  ::std::unique_ptr<::std::string> (*read_mvalue_string_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::read_mvalue_string_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_string_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue).release();
}

::std::int64_t cxxbridge1$read_mvalue_int_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue) noexcept {
  ::std::int64_t (*read_mvalue_int_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::read_mvalue_int_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_int_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue);
}

::std::uint64_t cxxbridge1$read_mvalue_uint_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue) noexcept {
  ::std::uint64_t (*read_mvalue_uint_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::read_mvalue_uint_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_uint_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue);
}

::std::vector<::ConstMValueWrapper> *cxxbridge1$read_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue) noexcept {
  ::std::unique_ptr<::std::vector<::ConstMValueWrapper>> (*read_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::read_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue).release();
}

::std::vector<::MValueDictPairWrapper> *cxxbridge1$read_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue) noexcept {
  ::std::unique_ptr<::std::vector<::MValueDictPairWrapper>> (*read_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::read_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue).release();
}

::std::string *cxxbridge1$read_mvalue_dict_pair_key_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueDictPairWrapper const &pair) noexcept {
  ::std::unique_ptr<::std::string> (*read_mvalue_dict_pair_key_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueDictPairWrapper const &) = ::read_mvalue_dict_pair_key_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_dict_pair_key_autocxx_wrapper_0xd5d0abec981e3e3a$(pair).release();
}

void cxxbridge1$read_mvalue_dict_pair_value_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueDictPairWrapper const &pair, ::ConstMValueWrapper *placement_return_type) noexcept {
  void (*read_mvalue_dict_pair_value_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueDictPairWrapper const &, ::ConstMValueWrapper *) = ::read_mvalue_dict_pair_value_autocxx_wrapper_0xd5d0abec981e3e3a;
  read_mvalue_dict_pair_value_autocxx_wrapper_0xd5d0abec981e3e3a$(pair, placement_return_type);
}

::alt::IBaseObject *cxxbridge1$read_mvalue_base_object_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue) noexcept {
  ::alt::IBaseObject *(*read_mvalue_base_object_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::read_mvalue_base_object_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_base_object_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue);
}

void cxxbridge1$read_mvalue_vector3_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue, ::Vector3Wrapper *placement_return_type) noexcept {
  void (*read_mvalue_vector3_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *, ::Vector3Wrapper *) = ::read_mvalue_vector3_autocxx_wrapper_0xd5d0abec981e3e3a;
  read_mvalue_vector3_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue, placement_return_type);
}

void cxxbridge1$read_mvalue_vector2_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue, ::Vector2Wrapper *placement_return_type) noexcept {
  void (*read_mvalue_vector2_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *, ::Vector2Wrapper *) = ::read_mvalue_vector2_autocxx_wrapper_0xd5d0abec981e3e3a;
  read_mvalue_vector2_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue, placement_return_type);
}

::std::size_t cxxbridge1$read_mvalue_byte_array_size_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue) noexcept {
  ::std::size_t (*read_mvalue_byte_array_size_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::read_mvalue_byte_array_size_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_mvalue_byte_array_size_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue);
}

void cxxbridge1$read_mvalue_byte_array_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue, ::std::uint8_t *data) noexcept {
  void (*read_mvalue_byte_array_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *, ::std::uint8_t *) = ::read_mvalue_byte_array_autocxx_wrapper_0xd5d0abec981e3e3a;
  read_mvalue_byte_array_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue, data);
}

void cxxbridge1$read_mvalue_rgba_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *mvalue, ::RGBAWrapper *placement_return_type) noexcept {
  void (*read_mvalue_rgba_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *, ::RGBAWrapper *) = ::read_mvalue_rgba_autocxx_wrapper_0xd5d0abec981e3e3a;
  read_mvalue_rgba_autocxx_wrapper_0xd5d0abec981e3e3a$(mvalue, placement_return_type);
}

void cxxbridge1$create_mvalue_bool_autocxx_wrapper_0xd5d0abec981e3e3a(bool value, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_bool_autocxx_wrapper_0xd5d0abec981e3e3a$)(bool, ::MValueMutWrapper *) = ::create_mvalue_bool_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_bool_autocxx_wrapper_0xd5d0abec981e3e3a$(value, placement_return_type);
}

void cxxbridge1$create_mvalue_double_autocxx_wrapper_0xd5d0abec981e3e3a(double value, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_double_autocxx_wrapper_0xd5d0abec981e3e3a$)(double, ::MValueMutWrapper *) = ::create_mvalue_double_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_double_autocxx_wrapper_0xd5d0abec981e3e3a$(value, placement_return_type);
}

void cxxbridge1$create_mvalue_string_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *value, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_string_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::create_mvalue_string_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_string_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(value), placement_return_type);
}

void cxxbridge1$create_mvalue_nil_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_nil_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueMutWrapper *) = ::create_mvalue_nil_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_nil_autocxx_wrapper_0xd5d0abec981e3e3a$(placement_return_type);
}

void cxxbridge1$create_mvalue_int_autocxx_wrapper_0xd5d0abec981e3e3a(::std::int64_t value, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_int_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::int64_t, ::MValueMutWrapper *) = ::create_mvalue_int_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_int_autocxx_wrapper_0xd5d0abec981e3e3a$(value, placement_return_type);
}

void cxxbridge1$create_mvalue_uint_autocxx_wrapper_0xd5d0abec981e3e3a(::std::uint64_t value, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_uint_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::uint64_t, ::MValueMutWrapper *) = ::create_mvalue_uint_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_uint_autocxx_wrapper_0xd5d0abec981e3e3a$(value, placement_return_type);
}

void cxxbridge1$create_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueMutWrapper *) = ::create_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a$(placement_return_type);
}

void cxxbridge1$push_to_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueMutWrapper &list, ::ConstMValueWrapper *value) noexcept {
  void (*push_to_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueMutWrapper &, ::ConstMValueWrapper *) = ::push_to_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a;
  push_to_mvalue_list_autocxx_wrapper_0xd5d0abec981e3e3a$(list, value);
}

void cxxbridge1$create_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueMutWrapper *) = ::create_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a$(placement_return_type);
}

void cxxbridge1$push_to_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueMutWrapper &dict, ::MValueMutWrapper *key, ::MValueMutWrapper *mvalue) noexcept {
  void (*push_to_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueMutWrapper &, ::MValueMutWrapper *, ::MValueMutWrapper *) = ::push_to_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a;
  push_to_mvalue_dict_autocxx_wrapper_0xd5d0abec981e3e3a$(dict, key, mvalue);
}

void cxxbridge1$create_mvalue_base_object_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IBaseObject *value, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_base_object_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IBaseObject *, ::MValueMutWrapper *) = ::create_mvalue_base_object_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_base_object_autocxx_wrapper_0xd5d0abec981e3e3a$(value, placement_return_type);
}

void cxxbridge1$create_mvalue_vector3_autocxx_wrapper_0xd5d0abec981e3e3a(float x, float y, float z, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_vector3_autocxx_wrapper_0xd5d0abec981e3e3a$)(float, float, float, ::MValueMutWrapper *) = ::create_mvalue_vector3_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_vector3_autocxx_wrapper_0xd5d0abec981e3e3a$(x, y, z, placement_return_type);
}

void cxxbridge1$create_mvalue_vector2_autocxx_wrapper_0xd5d0abec981e3e3a(float x, float y, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_vector2_autocxx_wrapper_0xd5d0abec981e3e3a$)(float, float, ::MValueMutWrapper *) = ::create_mvalue_vector2_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_vector2_autocxx_wrapper_0xd5d0abec981e3e3a$(x, y, placement_return_type);
}

void cxxbridge1$create_mvalue_byte_array_autocxx_wrapper_0xd5d0abec981e3e3a(::std::uint8_t const *data, ::std::size_t size, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_byte_array_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::uint8_t const *, ::std::size_t, ::MValueMutWrapper *) = ::create_mvalue_byte_array_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_byte_array_autocxx_wrapper_0xd5d0abec981e3e3a$(data, size, placement_return_type);
}

void cxxbridge1$create_mvalue_rgba_autocxx_wrapper_0xd5d0abec981e3e3a(::std::uint8_t r, ::std::uint8_t g, ::std::uint8_t b, ::std::uint8_t a, ::MValueMutWrapper *placement_return_type) noexcept {
  void (*create_mvalue_rgba_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::std::uint8_t, ::MValueMutWrapper *) = ::create_mvalue_rgba_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_mvalue_rgba_autocxx_wrapper_0xd5d0abec981e3e3a$(r, g, b, a, placement_return_type);
}

void cxxbridge1$trigger_local_event_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *event_name, ::MValueMutWrapper *mvalue_list) noexcept {
  void (*trigger_local_event_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::trigger_local_event_autocxx_wrapper_0xd5d0abec981e3e3a;
  trigger_local_event_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(event_name), mvalue_list);
}

void cxxbridge1$trigger_client_event_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer *player, ::std::string *event_name, ::MValueMutWrapper *mvalue_list) noexcept {
  void (*trigger_client_event_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer *, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::trigger_client_event_autocxx_wrapper_0xd5d0abec981e3e3a;
  trigger_client_event_autocxx_wrapper_0xd5d0abec981e3e3a$(player, ::std::unique_ptr<::std::string>(event_name), mvalue_list);
}

void cxxbridge1$trigger_client_event_unreliable_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::IPlayer *player, ::std::string *event_name, ::MValueMutWrapper *mvalue_list) noexcept {
  void (*trigger_client_event_unreliable_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::IPlayer *, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::trigger_client_event_unreliable_autocxx_wrapper_0xd5d0abec981e3e3a;
  trigger_client_event_unreliable_autocxx_wrapper_0xd5d0abec981e3e3a$(player, ::std::unique_ptr<::std::string>(event_name), mvalue_list);
}

void cxxbridge1$trigger_client_event_for_some_autocxx_wrapper_0xd5d0abec981e3e3a(::std::vector<::PlayerPtrWrapper> *players, ::std::string *event_name, ::MValueMutWrapper *mvalue_list) noexcept {
  void (*trigger_client_event_for_some_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::vector<::PlayerPtrWrapper> *, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::trigger_client_event_for_some_autocxx_wrapper_0xd5d0abec981e3e3a;
  trigger_client_event_for_some_autocxx_wrapper_0xd5d0abec981e3e3a$(players, ::std::unique_ptr<::std::string>(event_name), mvalue_list);
}

void cxxbridge1$trigger_client_event_unreliable_for_some_autocxx_wrapper_0xd5d0abec981e3e3a(::std::vector<::PlayerPtrWrapper> *players, ::std::string *event_name, ::MValueMutWrapper *mvalue_list) noexcept {
  void (*trigger_client_event_unreliable_for_some_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::vector<::PlayerPtrWrapper> *, ::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::trigger_client_event_unreliable_for_some_autocxx_wrapper_0xd5d0abec981e3e3a;
  trigger_client_event_unreliable_for_some_autocxx_wrapper_0xd5d0abec981e3e3a$(players, ::std::unique_ptr<::std::string>(event_name), mvalue_list);
}

void cxxbridge1$trigger_client_event_for_all_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *event_name, ::MValueMutWrapper *mvalue_list) noexcept {
  void (*trigger_client_event_for_all_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::trigger_client_event_for_all_autocxx_wrapper_0xd5d0abec981e3e3a;
  trigger_client_event_for_all_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(event_name), mvalue_list);
}

void cxxbridge1$trigger_client_event_unreliable_for_all_autocxx_wrapper_0xd5d0abec981e3e3a(::std::string *event_name, ::MValueMutWrapper *mvalue_list) noexcept {
  void (*trigger_client_event_unreliable_for_all_autocxx_wrapper_0xd5d0abec981e3e3a$)(::std::unique_ptr<::std::string>, ::MValueMutWrapper *) = ::trigger_client_event_unreliable_for_all_autocxx_wrapper_0xd5d0abec981e3e3a;
  trigger_client_event_unreliable_for_all_autocxx_wrapper_0xd5d0abec981e3e3a$(::std::unique_ptr<::std::string>(event_name), mvalue_list);
}

void cxxbridge1$read_alt_prop(::alt::Prop const &prop, ::std::uint16_t *out_drawable, ::std::uint8_t *out_texture) noexcept {
  void (*read_alt_prop$)(::alt::Prop const &, ::std::uint16_t *, ::std::uint8_t *) = ::read_alt_prop;
  read_alt_prop$(prop, out_drawable, out_texture);
}

void cxxbridge1$read_alt_dlc_prop(::alt::DlcProp const &prop, ::std::uint8_t *out_drawable, ::std::uint8_t *out_texture, ::std::uint32_t *out_dlc) noexcept {
  void (*read_alt_dlc_prop$)(::alt::DlcProp const &, ::std::uint8_t *, ::std::uint8_t *, ::std::uint32_t *) = ::read_alt_dlc_prop;
  read_alt_dlc_prop$(prop, out_drawable, out_texture, out_dlc);
}

void cxxbridge1$read_alt_cloth(::alt::Cloth const &cloth, ::std::uint16_t *out_drawable, ::std::uint8_t *out_texture, ::std::uint8_t *out_palette) noexcept {
  void (*read_alt_cloth$)(::alt::Cloth const &, ::std::uint16_t *, ::std::uint8_t *, ::std::uint8_t *) = ::read_alt_cloth;
  read_alt_cloth$(cloth, out_drawable, out_texture, out_palette);
}

void cxxbridge1$read_alt_dlc_cloth(::alt::DlcCloth const &cloth, ::std::uint16_t *out_drawable, ::std::uint8_t *out_texture, ::std::uint8_t *out_palette, ::std::uint32_t *out_dlc) noexcept {
  void (*read_alt_dlc_cloth$)(::alt::DlcCloth const &, ::std::uint16_t *, ::std::uint8_t *, ::std::uint8_t *, ::std::uint32_t *) = ::read_alt_dlc_cloth;
  read_alt_dlc_cloth$(cloth, out_drawable, out_texture, out_palette, out_dlc);
}

void cxxbridge1$read_alt_head_overlay(::alt::HeadOverlay const &overlay, ::std::uint8_t *out_index, float *out_opacity, ::std::uint8_t *out_color_type, ::std::uint8_t *out_color_index, ::std::uint8_t *out_second_color_index) noexcept {
  void (*read_alt_head_overlay$)(::alt::HeadOverlay const &, ::std::uint8_t *, float *, ::std::uint8_t *, ::std::uint8_t *, ::std::uint8_t *) = ::read_alt_head_overlay;
  read_alt_head_overlay$(overlay, out_index, out_opacity, out_color_type, out_color_index, out_second_color_index);
}

void cxxbridge1$read_alt_head_blend_data(::alt::HeadBlendData const &blend_data, ::std::uint32_t *out_shape_first_id, ::std::uint32_t *out_shape_second_id, ::std::uint32_t *out_shape_third_id, ::std::uint32_t *out_skin_first_id, ::std::uint32_t *out_skin_second_id, ::std::uint32_t *out_skin_third_id, float *out_shape_mix, float *out_skin_mix, float *out_third_mix) noexcept {
  void (*read_alt_head_blend_data$)(::alt::HeadBlendData const &, ::std::uint32_t *, ::std::uint32_t *, ::std::uint32_t *, ::std::uint32_t *, ::std::uint32_t *, ::std::uint32_t *, float *, float *, float *) = ::read_alt_head_blend_data;
  read_alt_head_blend_data$(blend_data, out_shape_first_id, out_shape_second_id, out_shape_third_id, out_skin_first_id, out_skin_second_id, out_skin_third_id, out_shape_mix, out_skin_mix, out_third_mix);
}

void cxxbridge1$read_bone_info(::alt::BoneInfo const &bone, ::std::uint16_t *id, ::std::uint16_t *index) noexcept {
  void (*read_bone_info$)(::alt::BoneInfo const &, ::std::uint16_t *, ::std::uint16_t *) = ::read_bone_info;
  read_bone_info$(bone, id, index);
}

::std::string *cxxbridge1$read_bone_info_name_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::BoneInfo const &bone) noexcept {
  ::std::unique_ptr<::std::string> (*read_bone_info_name_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::BoneInfo const &) = ::read_bone_info_name_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_bone_info_name_autocxx_wrapper_0xd5d0abec981e3e3a$(bone).release();
}

bool cxxbridge1$is_vehicle_model_info_valid(::alt::VehicleModelInfo const *ptr) noexcept {
  bool (*is_vehicle_model_info_valid$)(::alt::VehicleModelInfo const *) = ::is_vehicle_model_info_valid;
  return is_vehicle_model_info_valid$(ptr);
}

void cxxbridge1$read_vehicle_model_info(::alt::VehicleModelInfo const *ptr, ::std::uint8_t *out_model_type, ::std::uint8_t *out_wheels_count, bool *out_has_armored_windows, ::std::uint8_t *out_primary_color, ::std::uint8_t *out_secondary_color, ::std::uint8_t *out_pearl_color, ::std::uint8_t *out_wheels_color, ::std::uint8_t *out_interior_color, ::std::uint8_t *out_dashboard_color, bool *out_modkits, bool *out_has_auto_attach_trailer, bool *can_attach_cars, ::std::uint32_t *handling_name_hash) noexcept {
  void (*read_vehicle_model_info$)(::alt::VehicleModelInfo const *, ::std::uint8_t *, ::std::uint8_t *, bool *, ::std::uint8_t *, ::std::uint8_t *, ::std::uint8_t *, ::std::uint8_t *, ::std::uint8_t *, ::std::uint8_t *, bool *, bool *, bool *, ::std::uint32_t *) = ::read_vehicle_model_info;
  read_vehicle_model_info$(ptr, out_model_type, out_wheels_count, out_has_armored_windows, out_primary_color, out_secondary_color, out_pearl_color, out_wheels_color, out_interior_color, out_dashboard_color, out_modkits, out_has_auto_attach_trailer, can_attach_cars, handling_name_hash);
}

::std::string *cxxbridge1$read_vehicle_model_info_title_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::VehicleModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_vehicle_model_info_title_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::VehicleModelInfo const *) = ::read_vehicle_model_info_title_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_vehicle_model_info_title_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::vector<::alt::BoneInfo> *cxxbridge1$read_vehicle_model_info_bones_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::VehicleModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::alt::BoneInfo>> (*read_vehicle_model_info_bones_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::VehicleModelInfo const *) = ::read_vehicle_model_info_bones_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_vehicle_model_info_bones_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

bool cxxbridge1$is_ped_model_info_valid(::alt::PedModelInfo const *ptr) noexcept {
  bool (*is_ped_model_info_valid$)(::alt::PedModelInfo const *) = ::is_ped_model_info_valid;
  return is_ped_model_info_valid$(ptr);
}

::std::vector<::alt::BoneInfo> *cxxbridge1$read_ped_model_info_bones_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::PedModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::vector<::alt::BoneInfo>> (*read_ped_model_info_bones_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::PedModelInfo const *) = ::read_ped_model_info_bones_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_ped_model_info_bones_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::uint32_t cxxbridge1$read_ped_model_info_hash(::alt::PedModelInfo const *ptr) noexcept {
  ::std::uint32_t (*read_ped_model_info_hash$)(::alt::PedModelInfo const *) = ::read_ped_model_info_hash;
  return read_ped_model_info_hash$(ptr);
}

::std::string *cxxbridge1$read_ped_model_info_name_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::PedModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_ped_model_info_name_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::PedModelInfo const *) = ::read_ped_model_info_name_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_ped_model_info_name_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$read_ped_model_info_type_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::PedModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_ped_model_info_type_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::PedModelInfo const *) = ::read_ped_model_info_type_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_ped_model_info_type_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$read_ped_model_info_dlc_name_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::PedModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_ped_model_info_dlc_name_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::PedModelInfo const *) = ::read_ped_model_info_dlc_name_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_ped_model_info_dlc_name_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$read_ped_model_info_movement_clip_set_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::PedModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_ped_model_info_movement_clip_set_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::PedModelInfo const *) = ::read_ped_model_info_movement_clip_set_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_ped_model_info_movement_clip_set_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$read_ped_model_info_default_unarmed_weapon_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::PedModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_ped_model_info_default_unarmed_weapon_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::PedModelInfo const *) = ::read_ped_model_info_default_unarmed_weapon_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_ped_model_info_default_unarmed_weapon_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$read_quaternion(::alt::Quaternion const &quat, float *out_x, float *out_y, float *out_z, float *out_w) noexcept {
  void (*read_quaternion$)(::alt::Quaternion const &, float *, float *, float *, float *) = ::read_quaternion;
  read_quaternion$(quat, out_x, out_y, out_z, out_w);
}

bool cxxbridge1$is_weapon_model_info_valid(::alt::WeaponModelInfo const *ptr) noexcept {
  bool (*is_weapon_model_info_valid$)(::alt::WeaponModelInfo const *) = ::is_weapon_model_info_valid;
  return is_weapon_model_info_valid$(ptr);
}

void cxxbridge1$read_weapon_model_info(::alt::WeaponModelInfo const *ptr, ::std::uint32_t *hash, ::std::uint32_t *model_hash, ::std::uint32_t *ammo_type_hash, ::std::uint32_t *ammo_model_hash, ::std::int32_t *default_max_ammo_mp, ::std::int32_t *skill_above_50_max_ammo_mp, ::std::int32_t *max_skill_max_ammo_mp, ::std::int32_t *bonus_max_ammo_mp) noexcept {
  void (*read_weapon_model_info$)(::alt::WeaponModelInfo const *, ::std::uint32_t *, ::std::uint32_t *, ::std::uint32_t *, ::std::uint32_t *, ::std::int32_t *, ::std::int32_t *, ::std::int32_t *, ::std::int32_t *) = ::read_weapon_model_info;
  read_weapon_model_info$(ptr, hash, model_hash, ammo_type_hash, ammo_model_hash, default_max_ammo_mp, skill_above_50_max_ammo_mp, max_skill_max_ammo_mp, bonus_max_ammo_mp);
}

::std::string *cxxbridge1$read_weapon_model_info_name_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::WeaponModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_weapon_model_info_name_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::WeaponModelInfo const *) = ::read_weapon_model_info_name_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_weapon_model_info_name_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$read_weapon_model_info_ammo_type_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::WeaponModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_weapon_model_info_ammo_type_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::WeaponModelInfo const *) = ::read_weapon_model_info_ammo_type_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_weapon_model_info_ammo_type_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$read_weapon_model_info_model_name_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::WeaponModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_weapon_model_info_model_name_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::WeaponModelInfo const *) = ::read_weapon_model_info_model_name_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_weapon_model_info_model_name_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$read_weapon_model_info_ammo_model_name_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::WeaponModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_weapon_model_info_ammo_model_name_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::WeaponModelInfo const *) = ::read_weapon_model_info_ammo_model_name_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_weapon_model_info_ammo_model_name_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

::std::string *cxxbridge1$read_weapon_model_info_damage_type_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::WeaponModelInfo const *ptr) noexcept {
  ::std::unique_ptr<::std::string> (*read_weapon_model_info_damage_type_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::WeaponModelInfo const *) = ::read_weapon_model_info_damage_type_autocxx_wrapper_0xd5d0abec981e3e3a;
  return read_weapon_model_info_damage_type_autocxx_wrapper_0xd5d0abec981e3e3a$(ptr).release();
}

void cxxbridge1$read_ammo_flags(::alt::AmmoFlags const &flags, bool *infinite_ammo, bool *add_smoke_on_explosion, bool *fuse, bool *fixed_after_explosion) noexcept {
  void (*read_ammo_flags$)(::alt::AmmoFlags const &, bool *, bool *, bool *, bool *) = ::read_ammo_flags;
  read_ammo_flags$(flags, infinite_ammo, add_smoke_on_explosion, fuse, fixed_after_explosion);
}

void cxxbridge1$create_ammo_flags_from_params_autocxx_wrapper_0xd5d0abec981e3e3a(bool infinite_ammo, bool add_smoke_on_explosion, bool fuse, bool fixed_after_explosion, ::alt::AmmoFlags *placement_return_type) noexcept {
  void (*create_ammo_flags_from_params_autocxx_wrapper_0xd5d0abec981e3e3a$)(bool, bool, bool, bool, ::alt::AmmoFlags *) = ::create_ammo_flags_from_params_autocxx_wrapper_0xd5d0abec981e3e3a;
  create_ammo_flags_from_params_autocxx_wrapper_0xd5d0abec981e3e3a$(infinite_ammo, add_smoke_on_explosion, fuse, fixed_after_explosion, placement_return_type);
}

void cxxbridge1$read_alt_decoration(::alt::CDecoration const &decoration, ::std::uint32_t *out_collection, ::std::uint32_t *out_overlay) noexcept {
  void (*read_alt_decoration$)(::alt::CDecoration const &, ::std::uint32_t *, ::std::uint32_t *) = ::read_alt_decoration;
  read_alt_decoration$(decoration, out_collection, out_overlay);
}

void cxxbridge1$ConstMValueWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *autocxx_gen_this) noexcept {
  void (*ConstMValueWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::ConstMValueWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConstMValueWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this);
}

void cxxbridge1$ConstMValueWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *autocxx_gen_this, ::ConstMValueWrapper *other) noexcept {
  void (*ConstMValueWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *, ::ConstMValueWrapper *) = ::ConstMValueWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConstMValueWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$ConstMValueWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *autocxx_gen_this, ::ConstMValueWrapper const &other) noexcept {
  void (*ConstMValueWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *, ::ConstMValueWrapper const &) = ::ConstMValueWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConstMValueWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$ConstMValueWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::ConstMValueWrapper *autocxx_gen_this) noexcept {
  void (*ConstMValueWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConstMValueWrapper *) = ::ConstMValueWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConstMValueWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this);
}

void cxxbridge1$ConfigDictPairWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a(::ConfigDictPairWrapper *autocxx_gen_this) noexcept {
  void (*ConfigDictPairWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConfigDictPairWrapper *) = ::ConfigDictPairWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConfigDictPairWrapper_new_autocxx_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this);
}

void cxxbridge1$ConfigDictPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::ConfigDictPairWrapper *autocxx_gen_this, ::ConfigDictPairWrapper *other) noexcept {
  void (*ConfigDictPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConfigDictPairWrapper *, ::ConfigDictPairWrapper *) = ::ConfigDictPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConfigDictPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$ConfigDictPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::ConfigDictPairWrapper *autocxx_gen_this, ::ConfigDictPairWrapper const &other) noexcept {
  void (*ConfigDictPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConfigDictPairWrapper *, ::ConfigDictPairWrapper const &) = ::ConfigDictPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConfigDictPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$ConfigDictPairWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::ConfigDictPairWrapper *autocxx_gen_this) noexcept {
  void (*ConfigDictPairWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ConfigDictPairWrapper *) = ::ConfigDictPairWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  ConfigDictPairWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this);
}

void cxxbridge1$Vector3Wrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector3Wrapper *autocxx_gen_this, ::Vector3Wrapper *other) noexcept {
  void (*Vector3Wrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector3Wrapper *, ::Vector3Wrapper *) = ::Vector3Wrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector3Wrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$Vector3Wrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector3Wrapper *autocxx_gen_this, ::Vector3Wrapper const &other) noexcept {
  void (*Vector3Wrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector3Wrapper *, ::Vector3Wrapper const &) = ::Vector3Wrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector3Wrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$Vector3Wrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector3Wrapper *autocxx_gen_this) noexcept {
  void (*Vector3Wrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector3Wrapper *) = ::Vector3Wrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector3Wrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this);
}

void cxxbridge1$Vector2Wrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector2Wrapper *autocxx_gen_this, ::Vector2Wrapper *other) noexcept {
  void (*Vector2Wrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector2Wrapper *, ::Vector2Wrapper *) = ::Vector2Wrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector2Wrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$Vector2Wrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector2Wrapper *autocxx_gen_this, ::Vector2Wrapper const &other) noexcept {
  void (*Vector2Wrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector2Wrapper *, ::Vector2Wrapper const &) = ::Vector2Wrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector2Wrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$Vector2Wrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector2Wrapper *autocxx_gen_this) noexcept {
  void (*Vector2Wrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector2Wrapper *) = ::Vector2Wrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector2Wrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this);
}

void cxxbridge1$RGBAWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::RGBAWrapper *autocxx_gen_this, ::RGBAWrapper *other) noexcept {
  void (*RGBAWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::RGBAWrapper *, ::RGBAWrapper *) = ::RGBAWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  RGBAWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$RGBAWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::RGBAWrapper *autocxx_gen_this, ::RGBAWrapper const &other) noexcept {
  void (*RGBAWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::RGBAWrapper *, ::RGBAWrapper const &) = ::RGBAWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  RGBAWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$RGBAWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::RGBAWrapper *autocxx_gen_this) noexcept {
  void (*RGBAWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::RGBAWrapper *) = ::RGBAWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  RGBAWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this);
}

void cxxbridge1$WeaponWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::WeaponWrapper *autocxx_gen_this, ::WeaponWrapper *other) noexcept {
  void (*WeaponWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::WeaponWrapper *, ::WeaponWrapper *) = ::WeaponWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  WeaponWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$WeaponWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::WeaponWrapper *autocxx_gen_this) noexcept {
  void (*WeaponWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::WeaponWrapper *) = ::WeaponWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  WeaponWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this);
}

void cxxbridge1$FireInfoWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::FireInfoWrapper *autocxx_gen_this, ::FireInfoWrapper *other) noexcept {
  void (*FireInfoWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::FireInfoWrapper *, ::FireInfoWrapper *) = ::FireInfoWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  FireInfoWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$FireInfoWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::FireInfoWrapper *autocxx_gen_this, ::FireInfoWrapper const &other) noexcept {
  void (*FireInfoWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::FireInfoWrapper *, ::FireInfoWrapper const &) = ::FireInfoWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  FireInfoWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$FireInfoWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::FireInfoWrapper *autocxx_gen_this) noexcept {
  void (*FireInfoWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::FireInfoWrapper *) = ::FireInfoWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  FireInfoWrapper_synthetic_destructor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this);
}

::alt::CEvent *cxxbridge1$CEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CEvent *(*CEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CEvent *arg0) noexcept {
  void (*CEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CEvent *) = ::CEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CEvent *autocxx_gen_this, ::alt::CEvent const &other) noexcept {
  void (*alt_CEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CEvent *, ::alt::CEvent const &) = ::alt_CEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CCancellableEvent *cxxbridge1$CCancellableEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CCancellableEvent *(*CCancellableEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CCancellableEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CCancellableEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CCancellableEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CCancellableEvent *arg0) noexcept {
  void (*CCancellableEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CCancellableEvent *) = ::CCancellableEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CCancellableEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CCancellableEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CCancellableEvent *autocxx_gen_this, ::alt::CCancellableEvent const &other) noexcept {
  void (*alt_CCancellableEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CCancellableEvent *, ::alt::CCancellableEvent const &) = ::alt_CCancellableEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CCancellableEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CConsoleCommandEvent *cxxbridge1$CConsoleCommandEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CConsoleCommandEvent *(*CConsoleCommandEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CConsoleCommandEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CConsoleCommandEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CConsoleCommandEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CConsoleCommandEvent *arg0) noexcept {
  void (*CConsoleCommandEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CConsoleCommandEvent *) = ::CConsoleCommandEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CConsoleCommandEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::alt::CServerScriptEvent *cxxbridge1$CServerScriptEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CServerScriptEvent *(*CServerScriptEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CServerScriptEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CServerScriptEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CServerScriptEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CServerScriptEvent *arg0) noexcept {
  void (*CServerScriptEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CServerScriptEvent *) = ::CServerScriptEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CServerScriptEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CServerScriptEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CServerScriptEvent *autocxx_gen_this, ::alt::CServerScriptEvent const &other) noexcept {
  void (*alt_CServerScriptEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CServerScriptEvent *, ::alt::CServerScriptEvent const &) = ::alt_CServerScriptEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CServerScriptEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CClientScriptEvent *cxxbridge1$CClientScriptEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CClientScriptEvent *(*CClientScriptEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CClientScriptEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CClientScriptEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CClientScriptEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CClientScriptEvent *arg0) noexcept {
  void (*CClientScriptEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CClientScriptEvent *) = ::CClientScriptEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CClientScriptEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CClientScriptEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CClientScriptEvent *autocxx_gen_this, ::alt::CClientScriptEvent const &other) noexcept {
  void (*alt_CClientScriptEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CClientScriptEvent *, ::alt::CClientScriptEvent const &) = ::alt_CClientScriptEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CClientScriptEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerDisconnectEvent *cxxbridge1$CPlayerDisconnectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerDisconnectEvent *(*CPlayerDisconnectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerDisconnectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerDisconnectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerDisconnectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerDisconnectEvent *arg0) noexcept {
  void (*CPlayerDisconnectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerDisconnectEvent *) = ::CPlayerDisconnectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerDisconnectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerDisconnectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerDisconnectEvent *autocxx_gen_this, ::alt::CPlayerDisconnectEvent const &other) noexcept {
  void (*alt_CPlayerDisconnectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerDisconnectEvent *, ::alt::CPlayerDisconnectEvent const &) = ::alt_CPlayerDisconnectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerDisconnectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerConnectEvent *cxxbridge1$CPlayerConnectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerConnectEvent *(*CPlayerConnectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerConnectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerConnectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerConnectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerConnectEvent *arg0) noexcept {
  void (*CPlayerConnectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerConnectEvent *) = ::CPlayerConnectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerConnectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerConnectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerConnectEvent *autocxx_gen_this, ::alt::CPlayerConnectEvent const &other) noexcept {
  void (*alt_CPlayerConnectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerConnectEvent *, ::alt::CPlayerConnectEvent const &) = ::alt_CPlayerConnectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerConnectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CColShapeEvent *cxxbridge1$CColShapeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CColShapeEvent *(*CColShapeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CColShapeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CColShapeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CColShapeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CColShapeEvent *arg0) noexcept {
  void (*CColShapeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CColShapeEvent *) = ::CColShapeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CColShapeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CColShapeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CColShapeEvent *autocxx_gen_this, ::alt::CColShapeEvent const &other) noexcept {
  void (*alt_CColShapeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CColShapeEvent *, ::alt::CColShapeEvent const &) = ::alt_CColShapeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CColShapeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CWeaponDamageEvent *cxxbridge1$CWeaponDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CWeaponDamageEvent *(*CWeaponDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CWeaponDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CWeaponDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CWeaponDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CWeaponDamageEvent *arg0) noexcept {
  void (*CWeaponDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CWeaponDamageEvent *) = ::CWeaponDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CWeaponDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CWeaponDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CWeaponDamageEvent *autocxx_gen_this, ::alt::CWeaponDamageEvent const &other) noexcept {
  void (*alt_CWeaponDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CWeaponDamageEvent *, ::alt::CWeaponDamageEvent const &) = ::alt_CWeaponDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CWeaponDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerDeathEvent *cxxbridge1$CPlayerDeathEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerDeathEvent *(*CPlayerDeathEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerDeathEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerDeathEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerDeathEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerDeathEvent *arg0) noexcept {
  void (*CPlayerDeathEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerDeathEvent *) = ::CPlayerDeathEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerDeathEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerDeathEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerDeathEvent *autocxx_gen_this, ::alt::CPlayerDeathEvent const &other) noexcept {
  void (*alt_CPlayerDeathEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerDeathEvent *, ::alt::CPlayerDeathEvent const &) = ::alt_CPlayerDeathEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerDeathEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerDamageEvent *cxxbridge1$CPlayerDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerDamageEvent *(*CPlayerDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerDamageEvent *arg0) noexcept {
  void (*CPlayerDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerDamageEvent *) = ::CPlayerDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerDamageEvent *autocxx_gen_this, ::alt::CPlayerDamageEvent const &other) noexcept {
  void (*alt_CPlayerDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerDamageEvent *, ::alt::CPlayerDamageEvent const &) = ::alt_CPlayerDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerEnteringVehicleEvent *cxxbridge1$CPlayerEnteringVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerEnteringVehicleEvent *(*CPlayerEnteringVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerEnteringVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerEnteringVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerEnteringVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerEnteringVehicleEvent *arg0) noexcept {
  void (*CPlayerEnteringVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerEnteringVehicleEvent *) = ::CPlayerEnteringVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerEnteringVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerEnteringVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerEnteringVehicleEvent *autocxx_gen_this, ::alt::CPlayerEnteringVehicleEvent const &other) noexcept {
  void (*alt_CPlayerEnteringVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerEnteringVehicleEvent *, ::alt::CPlayerEnteringVehicleEvent const &) = ::alt_CPlayerEnteringVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerEnteringVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerEnterVehicleEvent *cxxbridge1$CPlayerEnterVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerEnterVehicleEvent *(*CPlayerEnterVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerEnterVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerEnterVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerEnterVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerEnterVehicleEvent *arg0) noexcept {
  void (*CPlayerEnterVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerEnterVehicleEvent *) = ::CPlayerEnterVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerEnterVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerEnterVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerEnterVehicleEvent *autocxx_gen_this, ::alt::CPlayerEnterVehicleEvent const &other) noexcept {
  void (*alt_CPlayerEnterVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerEnterVehicleEvent *, ::alt::CPlayerEnterVehicleEvent const &) = ::alt_CPlayerEnterVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerEnterVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerLeaveVehicleEvent *cxxbridge1$CPlayerLeaveVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerLeaveVehicleEvent *(*CPlayerLeaveVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerLeaveVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerLeaveVehicleEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerLeaveVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerLeaveVehicleEvent *arg0) noexcept {
  void (*CPlayerLeaveVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerLeaveVehicleEvent *) = ::CPlayerLeaveVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerLeaveVehicleEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerLeaveVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerLeaveVehicleEvent *autocxx_gen_this, ::alt::CPlayerLeaveVehicleEvent const &other) noexcept {
  void (*alt_CPlayerLeaveVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerLeaveVehicleEvent *, ::alt::CPlayerLeaveVehicleEvent const &) = ::alt_CPlayerLeaveVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerLeaveVehicleEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerChangeAnimationEvent *cxxbridge1$CPlayerChangeAnimationEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerChangeAnimationEvent *(*CPlayerChangeAnimationEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerChangeAnimationEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerChangeAnimationEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerChangeAnimationEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerChangeAnimationEvent *arg0) noexcept {
  void (*CPlayerChangeAnimationEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerChangeAnimationEvent *) = ::CPlayerChangeAnimationEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerChangeAnimationEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerChangeAnimationEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerChangeAnimationEvent *autocxx_gen_this, ::alt::CPlayerChangeAnimationEvent const &other) noexcept {
  void (*alt_CPlayerChangeAnimationEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerChangeAnimationEvent *, ::alt::CPlayerChangeAnimationEvent const &) = ::alt_CPlayerChangeAnimationEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerChangeAnimationEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerChangeVehicleSeatEvent *cxxbridge1$CPlayerChangeVehicleSeatEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerChangeVehicleSeatEvent *(*CPlayerChangeVehicleSeatEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerChangeVehicleSeatEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerChangeVehicleSeatEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerChangeVehicleSeatEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerChangeVehicleSeatEvent *arg0) noexcept {
  void (*CPlayerChangeVehicleSeatEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerChangeVehicleSeatEvent *) = ::CPlayerChangeVehicleSeatEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerChangeVehicleSeatEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerChangeVehicleSeatEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerChangeVehicleSeatEvent *autocxx_gen_this, ::alt::CPlayerChangeVehicleSeatEvent const &other) noexcept {
  void (*alt_CPlayerChangeVehicleSeatEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerChangeVehicleSeatEvent *, ::alt::CPlayerChangeVehicleSeatEvent const &) = ::alt_CPlayerChangeVehicleSeatEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerChangeVehicleSeatEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerWeaponChangeEvent *cxxbridge1$CPlayerWeaponChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerWeaponChangeEvent *(*CPlayerWeaponChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerWeaponChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerWeaponChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerWeaponChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerWeaponChangeEvent *arg0) noexcept {
  void (*CPlayerWeaponChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerWeaponChangeEvent *) = ::CPlayerWeaponChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerWeaponChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerWeaponChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerWeaponChangeEvent *autocxx_gen_this, ::alt::CPlayerWeaponChangeEvent const &other) noexcept {
  void (*alt_CPlayerWeaponChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerWeaponChangeEvent *, ::alt::CPlayerWeaponChangeEvent const &) = ::alt_CPlayerWeaponChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerWeaponChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerConnectDeniedEvent *cxxbridge1$CPlayerConnectDeniedEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerConnectDeniedEvent *(*CPlayerConnectDeniedEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerConnectDeniedEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerConnectDeniedEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerConnectDeniedEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerConnectDeniedEvent *arg0) noexcept {
  void (*CPlayerConnectDeniedEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerConnectDeniedEvent *) = ::CPlayerConnectDeniedEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerConnectDeniedEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerConnectDeniedEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerConnectDeniedEvent *autocxx_gen_this, ::alt::CPlayerConnectDeniedEvent const &other) noexcept {
  void (*alt_CPlayerConnectDeniedEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerConnectDeniedEvent *, ::alt::CPlayerConnectDeniedEvent const &) = ::alt_CPlayerConnectDeniedEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerConnectDeniedEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerSpawnEvent *cxxbridge1$CPlayerSpawnEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerSpawnEvent *(*CPlayerSpawnEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerSpawnEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerSpawnEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerSpawnEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerSpawnEvent *arg0) noexcept {
  void (*CPlayerSpawnEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerSpawnEvent *) = ::CPlayerSpawnEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerSpawnEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerSpawnEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerSpawnEvent *autocxx_gen_this, ::alt::CPlayerSpawnEvent const &other) noexcept {
  void (*alt_CPlayerSpawnEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerSpawnEvent *, ::alt::CPlayerSpawnEvent const &) = ::alt_CPlayerSpawnEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerSpawnEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CStartProjectileEvent *cxxbridge1$CStartProjectileEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CStartProjectileEvent *(*CStartProjectileEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CStartProjectileEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CStartProjectileEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CStartProjectileEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStartProjectileEvent *arg0) noexcept {
  void (*CStartProjectileEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStartProjectileEvent *) = ::CStartProjectileEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStartProjectileEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CStartProjectileEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStartProjectileEvent *autocxx_gen_this, ::alt::CStartProjectileEvent const &other) noexcept {
  void (*alt_CStartProjectileEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStartProjectileEvent *, ::alt::CStartProjectileEvent const &) = ::alt_CStartProjectileEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CStartProjectileEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerRequestControlEvent *cxxbridge1$CPlayerRequestControlEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerRequestControlEvent *(*CPlayerRequestControlEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerRequestControlEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerRequestControlEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerRequestControlEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerRequestControlEvent *arg0) noexcept {
  void (*CPlayerRequestControlEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerRequestControlEvent *) = ::CPlayerRequestControlEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerRequestControlEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerRequestControlEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerRequestControlEvent *autocxx_gen_this, ::alt::CPlayerRequestControlEvent const &other) noexcept {
  void (*alt_CPlayerRequestControlEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerRequestControlEvent *, ::alt::CPlayerRequestControlEvent const &) = ::alt_CPlayerRequestControlEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerRequestControlEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerDimensionChangeEvent *cxxbridge1$CPlayerDimensionChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerDimensionChangeEvent *(*CPlayerDimensionChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerDimensionChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerDimensionChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerDimensionChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerDimensionChangeEvent *arg0) noexcept {
  void (*CPlayerDimensionChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerDimensionChangeEvent *) = ::CPlayerDimensionChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerDimensionChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerDimensionChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerDimensionChangeEvent *autocxx_gen_this, ::alt::CPlayerDimensionChangeEvent const &other) noexcept {
  void (*alt_CPlayerDimensionChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerDimensionChangeEvent *, ::alt::CPlayerDimensionChangeEvent const &) = ::alt_CPlayerDimensionChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerDimensionChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerChangeInteriorEvent *cxxbridge1$CPlayerChangeInteriorEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerChangeInteriorEvent *(*CPlayerChangeInteriorEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerChangeInteriorEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerChangeInteriorEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerChangeInteriorEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerChangeInteriorEvent *arg0) noexcept {
  void (*CPlayerChangeInteriorEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerChangeInteriorEvent *) = ::CPlayerChangeInteriorEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerChangeInteriorEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerChangeInteriorEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerChangeInteriorEvent *autocxx_gen_this, ::alt::CPlayerChangeInteriorEvent const &other) noexcept {
  void (*alt_CPlayerChangeInteriorEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerChangeInteriorEvent *, ::alt::CPlayerChangeInteriorEvent const &) = ::alt_CPlayerChangeInteriorEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerChangeInteriorEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CExplosionEvent *cxxbridge1$CExplosionEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CExplosionEvent *(*CExplosionEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CExplosionEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CExplosionEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CExplosionEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CExplosionEvent *arg0) noexcept {
  void (*CExplosionEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CExplosionEvent *) = ::CExplosionEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CExplosionEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CExplosionEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CExplosionEvent *autocxx_gen_this, ::alt::CExplosionEvent const &other) noexcept {
  void (*alt_CExplosionEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CExplosionEvent *, ::alt::CExplosionEvent const &) = ::alt_CExplosionEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CExplosionEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CFireEvent *cxxbridge1$CFireEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CFireEvent *(*CFireEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CFireEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CFireEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CFireEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CFireEvent *arg0) noexcept {
  void (*CFireEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CFireEvent *) = ::CFireEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CFireEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::alt::CConnectionQueueAddEvent *cxxbridge1$CConnectionQueueAddEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CConnectionQueueAddEvent *(*CConnectionQueueAddEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CConnectionQueueAddEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CConnectionQueueAddEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CConnectionQueueAddEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CConnectionQueueAddEvent *arg0) noexcept {
  void (*CConnectionQueueAddEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CConnectionQueueAddEvent *) = ::CConnectionQueueAddEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CConnectionQueueAddEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CConnectionQueueAddEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CConnectionQueueAddEvent *autocxx_gen_this, ::alt::CConnectionQueueAddEvent const &other) noexcept {
  void (*alt_CConnectionQueueAddEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CConnectionQueueAddEvent *, ::alt::CConnectionQueueAddEvent const &) = ::alt_CConnectionQueueAddEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CConnectionQueueAddEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CConnectionQueueRemoveEvent *cxxbridge1$CConnectionQueueRemoveEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CConnectionQueueRemoveEvent *(*CConnectionQueueRemoveEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CConnectionQueueRemoveEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CConnectionQueueRemoveEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CConnectionQueueRemoveEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CConnectionQueueRemoveEvent *arg0) noexcept {
  void (*CConnectionQueueRemoveEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CConnectionQueueRemoveEvent *) = ::CConnectionQueueRemoveEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CConnectionQueueRemoveEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CConnectionQueueRemoveEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CConnectionQueueRemoveEvent *autocxx_gen_this, ::alt::CConnectionQueueRemoveEvent const &other) noexcept {
  void (*alt_CConnectionQueueRemoveEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CConnectionQueueRemoveEvent *, ::alt::CConnectionQueueRemoveEvent const &) = ::alt_CConnectionQueueRemoveEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CConnectionQueueRemoveEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPlayerHealEvent *cxxbridge1$CPlayerHealEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPlayerHealEvent *(*CPlayerHealEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPlayerHealEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPlayerHealEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPlayerHealEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerHealEvent *arg0) noexcept {
  void (*CPlayerHealEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerHealEvent *) = ::CPlayerHealEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPlayerHealEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPlayerHealEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPlayerHealEvent *autocxx_gen_this, ::alt::CPlayerHealEvent const &other) noexcept {
  void (*alt_CPlayerHealEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPlayerHealEvent *, ::alt::CPlayerHealEvent const &) = ::alt_CPlayerHealEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPlayerHealEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CVehicleAttachEvent *cxxbridge1$CVehicleAttachEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CVehicleAttachEvent *(*CVehicleAttachEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CVehicleAttachEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CVehicleAttachEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CVehicleAttachEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleAttachEvent *arg0) noexcept {
  void (*CVehicleAttachEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleAttachEvent *) = ::CVehicleAttachEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CVehicleAttachEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CVehicleAttachEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleAttachEvent *autocxx_gen_this, ::alt::CVehicleAttachEvent const &other) noexcept {
  void (*alt_CVehicleAttachEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleAttachEvent *, ::alt::CVehicleAttachEvent const &) = ::alt_CVehicleAttachEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CVehicleAttachEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CVehicleDetachEvent *cxxbridge1$CVehicleDetachEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CVehicleDetachEvent *(*CVehicleDetachEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CVehicleDetachEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CVehicleDetachEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CVehicleDetachEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleDetachEvent *arg0) noexcept {
  void (*CVehicleDetachEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleDetachEvent *) = ::CVehicleDetachEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CVehicleDetachEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CVehicleDetachEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleDetachEvent *autocxx_gen_this, ::alt::CVehicleDetachEvent const &other) noexcept {
  void (*alt_CVehicleDetachEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleDetachEvent *, ::alt::CVehicleDetachEvent const &) = ::alt_CVehicleDetachEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CVehicleDetachEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CVehicleDestroyEvent *cxxbridge1$CVehicleDestroyEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CVehicleDestroyEvent *(*CVehicleDestroyEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CVehicleDestroyEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CVehicleDestroyEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CVehicleDestroyEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleDestroyEvent *arg0) noexcept {
  void (*CVehicleDestroyEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleDestroyEvent *) = ::CVehicleDestroyEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CVehicleDestroyEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CVehicleDestroyEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleDestroyEvent *autocxx_gen_this, ::alt::CVehicleDestroyEvent const &other) noexcept {
  void (*alt_CVehicleDestroyEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleDestroyEvent *, ::alt::CVehicleDestroyEvent const &) = ::alt_CVehicleDestroyEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CVehicleDestroyEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CVehicleDamageEvent *cxxbridge1$CVehicleDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CVehicleDamageEvent *(*CVehicleDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CVehicleDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CVehicleDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CVehicleDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleDamageEvent *arg0) noexcept {
  void (*CVehicleDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleDamageEvent *) = ::CVehicleDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CVehicleDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CVehicleDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleDamageEvent *autocxx_gen_this, ::alt::CVehicleDamageEvent const &other) noexcept {
  void (*alt_CVehicleDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleDamageEvent *, ::alt::CVehicleDamageEvent const &) = ::alt_CVehicleDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CVehicleDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CVehicleHornEvent *cxxbridge1$CVehicleHornEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CVehicleHornEvent *(*CVehicleHornEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CVehicleHornEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CVehicleHornEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CVehicleHornEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleHornEvent *arg0) noexcept {
  void (*CVehicleHornEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleHornEvent *) = ::CVehicleHornEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CVehicleHornEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CVehicleHornEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleHornEvent *autocxx_gen_this, ::alt::CVehicleHornEvent const &other) noexcept {
  void (*alt_CVehicleHornEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleHornEvent *, ::alt::CVehicleHornEvent const &) = ::alt_CVehicleHornEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CVehicleHornEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CVehicleSirenEvent *cxxbridge1$CVehicleSirenEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CVehicleSirenEvent *(*CVehicleSirenEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CVehicleSirenEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CVehicleSirenEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CVehicleSirenEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleSirenEvent *arg0) noexcept {
  void (*CVehicleSirenEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleSirenEvent *) = ::CVehicleSirenEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CVehicleSirenEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CVehicleSirenEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVehicleSirenEvent *autocxx_gen_this, ::alt::CVehicleSirenEvent const &other) noexcept {
  void (*alt_CVehicleSirenEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVehicleSirenEvent *, ::alt::CVehicleSirenEvent const &) = ::alt_CVehicleSirenEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CVehicleSirenEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CNetOwnerChangeEvent *cxxbridge1$CNetOwnerChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CNetOwnerChangeEvent *(*CNetOwnerChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CNetOwnerChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CNetOwnerChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CNetOwnerChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CNetOwnerChangeEvent *arg0) noexcept {
  void (*CNetOwnerChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CNetOwnerChangeEvent *) = ::CNetOwnerChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CNetOwnerChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CNetOwnerChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CNetOwnerChangeEvent *autocxx_gen_this, ::alt::CNetOwnerChangeEvent const &other) noexcept {
  void (*alt_CNetOwnerChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CNetOwnerChangeEvent *, ::alt::CNetOwnerChangeEvent const &) = ::alt_CNetOwnerChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CNetOwnerChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CMetaChangeEvent *cxxbridge1$CMetaChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CMetaChangeEvent *(*CMetaChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CMetaChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CMetaChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CMetaChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CMetaChangeEvent *arg0) noexcept {
  void (*CMetaChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CMetaChangeEvent *) = ::CMetaChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CMetaChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CMetaChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CMetaChangeEvent *autocxx_gen_this, ::alt::CMetaChangeEvent const &other) noexcept {
  void (*alt_CMetaChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CMetaChangeEvent *, ::alt::CMetaChangeEvent const &) = ::alt_CMetaChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CMetaChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CGlobalMetaDataChangeEvent *cxxbridge1$CGlobalMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CGlobalMetaDataChangeEvent *(*CGlobalMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CGlobalMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CGlobalMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CGlobalMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGlobalMetaDataChangeEvent *arg0) noexcept {
  void (*CGlobalMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGlobalMetaDataChangeEvent *) = ::CGlobalMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CGlobalMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CGlobalMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGlobalMetaDataChangeEvent *autocxx_gen_this, ::alt::CGlobalMetaDataChangeEvent const &other) noexcept {
  void (*alt_CGlobalMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGlobalMetaDataChangeEvent *, ::alt::CGlobalMetaDataChangeEvent const &) = ::alt_CGlobalMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CGlobalMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CGlobalSyncedMetaDataChangeEvent *cxxbridge1$CGlobalSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CGlobalSyncedMetaDataChangeEvent *(*CGlobalSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CGlobalSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CGlobalSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CGlobalSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGlobalSyncedMetaDataChangeEvent *arg0) noexcept {
  void (*CGlobalSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGlobalSyncedMetaDataChangeEvent *) = ::CGlobalSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CGlobalSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CGlobalSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGlobalSyncedMetaDataChangeEvent *autocxx_gen_this, ::alt::CGlobalSyncedMetaDataChangeEvent const &other) noexcept {
  void (*alt_CGlobalSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGlobalSyncedMetaDataChangeEvent *, ::alt::CGlobalSyncedMetaDataChangeEvent const &) = ::alt_CGlobalSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CGlobalSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CSyncedMetaDataChangeEvent *cxxbridge1$CSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CSyncedMetaDataChangeEvent *(*CSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CSyncedMetaDataChangeEvent *arg0) noexcept {
  void (*CSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CSyncedMetaDataChangeEvent *) = ::CSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CSyncedMetaDataChangeEvent *autocxx_gen_this, ::alt::CSyncedMetaDataChangeEvent const &other) noexcept {
  void (*alt_CSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CSyncedMetaDataChangeEvent *, ::alt::CSyncedMetaDataChangeEvent const &) = ::alt_CSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CStreamSyncedMetaDataChangeEvent *cxxbridge1$CStreamSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CStreamSyncedMetaDataChangeEvent *(*CStreamSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CStreamSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CStreamSyncedMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CStreamSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStreamSyncedMetaDataChangeEvent *arg0) noexcept {
  void (*CStreamSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStreamSyncedMetaDataChangeEvent *) = ::CStreamSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStreamSyncedMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CStreamSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStreamSyncedMetaDataChangeEvent *autocxx_gen_this, ::alt::CStreamSyncedMetaDataChangeEvent const &other) noexcept {
  void (*alt_CStreamSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStreamSyncedMetaDataChangeEvent *, ::alt::CStreamSyncedMetaDataChangeEvent const &) = ::alt_CStreamSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CStreamSyncedMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CLocalMetaDataChangeEvent *cxxbridge1$CLocalMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CLocalMetaDataChangeEvent *(*CLocalMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CLocalMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CLocalMetaDataChangeEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CLocalMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CLocalMetaDataChangeEvent *arg0) noexcept {
  void (*CLocalMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CLocalMetaDataChangeEvent *) = ::CLocalMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CLocalMetaDataChangeEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CLocalMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CLocalMetaDataChangeEvent *autocxx_gen_this, ::alt::CLocalMetaDataChangeEvent const &other) noexcept {
  void (*alt_CLocalMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CLocalMetaDataChangeEvent *, ::alt::CLocalMetaDataChangeEvent const &) = ::alt_CLocalMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CLocalMetaDataChangeEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CResourceStopEvent *cxxbridge1$CResourceStopEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CResourceStopEvent *(*CResourceStopEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CResourceStopEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CResourceStopEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CResourceStopEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CResourceStopEvent *arg0) noexcept {
  void (*CResourceStopEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CResourceStopEvent *) = ::CResourceStopEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CResourceStopEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CResourceStopEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CResourceStopEvent *autocxx_gen_this, ::alt::CResourceStopEvent const &other) noexcept {
  void (*alt_CResourceStopEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CResourceStopEvent *, ::alt::CResourceStopEvent const &) = ::alt_CResourceStopEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CResourceStopEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CResourceStartEvent *cxxbridge1$CResourceStartEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CResourceStartEvent *(*CResourceStartEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CResourceStartEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CResourceStartEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CResourceStartEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CResourceStartEvent *arg0) noexcept {
  void (*CResourceStartEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CResourceStartEvent *) = ::CResourceStartEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CResourceStartEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CResourceStartEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CResourceStartEvent *autocxx_gen_this, ::alt::CResourceStartEvent const &other) noexcept {
  void (*alt_CResourceStartEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CResourceStartEvent *, ::alt::CResourceStartEvent const &) = ::alt_CResourceStartEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CResourceStartEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CVoiceConnectionEvent *cxxbridge1$CVoiceConnectionEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CVoiceConnectionEvent *(*CVoiceConnectionEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CVoiceConnectionEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CVoiceConnectionEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CVoiceConnectionEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVoiceConnectionEvent *arg0) noexcept {
  void (*CVoiceConnectionEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVoiceConnectionEvent *) = ::CVoiceConnectionEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CVoiceConnectionEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CVoiceConnectionEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CVoiceConnectionEvent *autocxx_gen_this, ::alt::CVoiceConnectionEvent const &other) noexcept {
  void (*alt_CVoiceConnectionEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CVoiceConnectionEvent *, ::alt::CVoiceConnectionEvent const &) = ::alt_CVoiceConnectionEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CVoiceConnectionEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CRequestSyncedSceneEvent *cxxbridge1$CRequestSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CRequestSyncedSceneEvent *(*CRequestSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CRequestSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CRequestSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CRequestSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CRequestSyncedSceneEvent *arg0) noexcept {
  void (*CRequestSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CRequestSyncedSceneEvent *) = ::CRequestSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CRequestSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CRequestSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CRequestSyncedSceneEvent *autocxx_gen_this, ::alt::CRequestSyncedSceneEvent const &other) noexcept {
  void (*alt_CRequestSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CRequestSyncedSceneEvent *, ::alt::CRequestSyncedSceneEvent const &) = ::alt_CRequestSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CRequestSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CStartSyncedSceneEvent *cxxbridge1$CStartSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CStartSyncedSceneEvent *(*CStartSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CStartSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CStartSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CStartSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStartSyncedSceneEvent *arg0) noexcept {
  void (*CStartSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStartSyncedSceneEvent *) = ::CStartSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStartSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::alt::CStopSyncedSceneEvent *cxxbridge1$CStopSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CStopSyncedSceneEvent *(*CStopSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CStopSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CStopSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CStopSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStopSyncedSceneEvent *arg0) noexcept {
  void (*CStopSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStopSyncedSceneEvent *) = ::CStopSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CStopSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CStopSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CStopSyncedSceneEvent *autocxx_gen_this, ::alt::CStopSyncedSceneEvent const &other) noexcept {
  void (*alt_CStopSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CStopSyncedSceneEvent *, ::alt::CStopSyncedSceneEvent const &) = ::alt_CStopSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CStopSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CUpdateSyncedSceneEvent *cxxbridge1$CUpdateSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CUpdateSyncedSceneEvent *(*CUpdateSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CUpdateSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CUpdateSyncedSceneEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CUpdateSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CUpdateSyncedSceneEvent *arg0) noexcept {
  void (*CUpdateSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CUpdateSyncedSceneEvent *) = ::CUpdateSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CUpdateSyncedSceneEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CUpdateSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CUpdateSyncedSceneEvent *autocxx_gen_this, ::alt::CUpdateSyncedSceneEvent const &other) noexcept {
  void (*alt_CUpdateSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CUpdateSyncedSceneEvent *, ::alt::CUpdateSyncedSceneEvent const &) = ::alt_CUpdateSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CUpdateSyncedSceneEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CClientDeleteObjectEvent *cxxbridge1$CClientDeleteObjectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CClientDeleteObjectEvent *(*CClientDeleteObjectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CClientDeleteObjectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CClientDeleteObjectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CClientDeleteObjectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CClientDeleteObjectEvent *arg0) noexcept {
  void (*CClientDeleteObjectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CClientDeleteObjectEvent *) = ::CClientDeleteObjectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CClientDeleteObjectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CClientDeleteObjectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CClientDeleteObjectEvent *autocxx_gen_this, ::alt::CClientDeleteObjectEvent const &other) noexcept {
  void (*alt_CClientDeleteObjectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CClientDeleteObjectEvent *, ::alt::CClientDeleteObjectEvent const &) = ::alt_CClientDeleteObjectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CClientDeleteObjectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CClientRequestObjectEvent *cxxbridge1$CClientRequestObjectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CClientRequestObjectEvent *(*CClientRequestObjectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CClientRequestObjectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CClientRequestObjectEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CClientRequestObjectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CClientRequestObjectEvent *arg0) noexcept {
  void (*CClientRequestObjectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CClientRequestObjectEvent *) = ::CClientRequestObjectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CClientRequestObjectEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CClientRequestObjectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CClientRequestObjectEvent *autocxx_gen_this, ::alt::CClientRequestObjectEvent const &other) noexcept {
  void (*alt_CClientRequestObjectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CClientRequestObjectEvent *, ::alt::CClientRequestObjectEvent const &) = ::alt_CClientRequestObjectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CClientRequestObjectEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CGivePedScriptedTaskEvent *cxxbridge1$CGivePedScriptedTaskEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CGivePedScriptedTaskEvent *(*CGivePedScriptedTaskEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CGivePedScriptedTaskEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CGivePedScriptedTaskEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CGivePedScriptedTaskEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGivePedScriptedTaskEvent *arg0) noexcept {
  void (*CGivePedScriptedTaskEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGivePedScriptedTaskEvent *) = ::CGivePedScriptedTaskEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CGivePedScriptedTaskEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CGivePedScriptedTaskEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CGivePedScriptedTaskEvent *autocxx_gen_this, ::alt::CGivePedScriptedTaskEvent const &other) noexcept {
  void (*alt_CGivePedScriptedTaskEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CGivePedScriptedTaskEvent *, ::alt::CGivePedScriptedTaskEvent const &) = ::alt_CGivePedScriptedTaskEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CGivePedScriptedTaskEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPedDeathEvent *cxxbridge1$CPedDeathEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPedDeathEvent *(*CPedDeathEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPedDeathEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPedDeathEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPedDeathEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPedDeathEvent *arg0) noexcept {
  void (*CPedDeathEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPedDeathEvent *) = ::CPedDeathEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPedDeathEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPedDeathEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPedDeathEvent *autocxx_gen_this, ::alt::CPedDeathEvent const &other) noexcept {
  void (*alt_CPedDeathEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPedDeathEvent *, ::alt::CPedDeathEvent const &) = ::alt_CPedDeathEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPedDeathEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPedDamageEvent *cxxbridge1$CPedDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPedDamageEvent *(*CPedDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPedDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPedDamageEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPedDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPedDamageEvent *arg0) noexcept {
  void (*CPedDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPedDamageEvent *) = ::CPedDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPedDamageEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPedDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPedDamageEvent *autocxx_gen_this, ::alt::CPedDamageEvent const &other) noexcept {
  void (*alt_CPedDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPedDamageEvent *, ::alt::CPedDamageEvent const &) = ::alt_CPedDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPedDamageEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CPedHealEvent *cxxbridge1$CPedHealEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CPedHealEvent *(*CPedHealEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CPedHealEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CPedHealEvent_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CPedHealEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPedHealEvent *arg0) noexcept {
  void (*CPedHealEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPedHealEvent *) = ::CPedHealEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CPedHealEvent_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CPedHealEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CPedHealEvent *autocxx_gen_this, ::alt::CPedHealEvent const &other) noexcept {
  void (*alt_CPedHealEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CPedHealEvent *, ::alt::CPedHealEvent const &) = ::alt_CPedHealEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CPedHealEvent_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::MValueUnorderedMapWrapper *cxxbridge1$MValueUnorderedMapWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::MValueUnorderedMapWrapper *(*MValueUnorderedMapWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::MValueUnorderedMapWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return MValueUnorderedMapWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$MValueUnorderedMapWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueUnorderedMapWrapper *arg0) noexcept {
  void (*MValueUnorderedMapWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueUnorderedMapWrapper *) = ::MValueUnorderedMapWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  MValueUnorderedMapWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::Vector2Vec *cxxbridge1$Vector2Vec_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::Vector2Vec *(*Vector2Vec_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::Vector2Vec_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return Vector2Vec_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$Vector2Vec_free_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector2Vec *arg0) noexcept {
  void (*Vector2Vec_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector2Vec *) = ::Vector2Vec_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector2Vec_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$Vector2Vec_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::Vector2Vec *autocxx_gen_this, ::Vector2Vec *other) noexcept {
  void (*Vector2Vec_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::Vector2Vec *, ::Vector2Vec *) = ::Vector2Vec_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  Vector2Vec_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::MValueMutWrapper *cxxbridge1$MValueMutWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::MValueMutWrapper *(*MValueMutWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::MValueMutWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return MValueMutWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$MValueMutWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueMutWrapper *arg0) noexcept {
  void (*MValueMutWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueMutWrapper *) = ::MValueMutWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  MValueMutWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$MValueMutWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueMutWrapper *autocxx_gen_this, ::MValueMutWrapper *other) noexcept {
  void (*MValueMutWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueMutWrapper *, ::MValueMutWrapper *) = ::MValueMutWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  MValueMutWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$MValueMutWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueMutWrapper *autocxx_gen_this, ::MValueMutWrapper const &other) noexcept {
  void (*MValueMutWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueMutWrapper *, ::MValueMutWrapper const &) = ::MValueMutWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  MValueMutWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::ResourcePtrWrapper *cxxbridge1$ResourcePtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::ResourcePtrWrapper *(*ResourcePtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::ResourcePtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return ResourcePtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$ResourcePtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::ResourcePtrWrapper *arg0) noexcept {
  void (*ResourcePtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ResourcePtrWrapper *) = ::ResourcePtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  ResourcePtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$ResourcePtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::ResourcePtrWrapper *autocxx_gen_this, ::ResourcePtrWrapper *other) noexcept {
  void (*ResourcePtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ResourcePtrWrapper *, ::ResourcePtrWrapper *) = ::ResourcePtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  ResourcePtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$ResourcePtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::ResourcePtrWrapper *autocxx_gen_this, ::ResourcePtrWrapper const &other) noexcept {
  void (*ResourcePtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::ResourcePtrWrapper *, ::ResourcePtrWrapper const &) = ::ResourcePtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  ResourcePtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::PlayerPtrWrapper *cxxbridge1$PlayerPtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::PlayerPtrWrapper *(*PlayerPtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::PlayerPtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return PlayerPtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$PlayerPtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::PlayerPtrWrapper *arg0) noexcept {
  void (*PlayerPtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::PlayerPtrWrapper *) = ::PlayerPtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  PlayerPtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$PlayerPtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::PlayerPtrWrapper *autocxx_gen_this, ::PlayerPtrWrapper *other) noexcept {
  void (*PlayerPtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::PlayerPtrWrapper *, ::PlayerPtrWrapper *) = ::PlayerPtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  PlayerPtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$PlayerPtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::PlayerPtrWrapper *autocxx_gen_this, ::PlayerPtrWrapper const &other) noexcept {
  void (*PlayerPtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::PlayerPtrWrapper *, ::PlayerPtrWrapper const &) = ::PlayerPtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  PlayerPtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::VehicleModelInfo *cxxbridge1$VehicleModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::VehicleModelInfo *(*VehicleModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::VehicleModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return VehicleModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$VehicleModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::VehicleModelInfo *arg0) noexcept {
  void (*VehicleModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::VehicleModelInfo *) = ::VehicleModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  VehicleModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_VehicleModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::VehicleModelInfo *autocxx_gen_this, ::alt::VehicleModelInfo *other) noexcept {
  void (*alt_VehicleModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::VehicleModelInfo *, ::alt::VehicleModelInfo *) = ::alt_VehicleModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_VehicleModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::PedModelInfo *cxxbridge1$PedModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::PedModelInfo *(*PedModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::PedModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return PedModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$PedModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::PedModelInfo *arg0) noexcept {
  void (*PedModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::PedModelInfo *) = ::PedModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  PedModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_PedModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::PedModelInfo *autocxx_gen_this, ::alt::PedModelInfo *other) noexcept {
  void (*alt_PedModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::PedModelInfo *, ::alt::PedModelInfo *) = ::alt_PedModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_PedModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::WeaponModelInfo *cxxbridge1$WeaponModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::WeaponModelInfo *(*WeaponModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::WeaponModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return WeaponModelInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$WeaponModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::WeaponModelInfo *arg0) noexcept {
  void (*WeaponModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::WeaponModelInfo *) = ::WeaponModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  WeaponModelInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_WeaponModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::WeaponModelInfo *autocxx_gen_this, ::alt::WeaponModelInfo *other) noexcept {
  void (*alt_WeaponModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::WeaponModelInfo *, ::alt::WeaponModelInfo *) = ::alt_WeaponModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_WeaponModelInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_WeaponModelInfo_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::WeaponModelInfo *autocxx_gen_this, ::alt::WeaponModelInfo const &other) noexcept {
  void (*alt_WeaponModelInfo_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::WeaponModelInfo *, ::alt::WeaponModelInfo const &) = ::alt_WeaponModelInfo_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_WeaponModelInfo_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::BaseObjectPtrWrapper *cxxbridge1$BaseObjectPtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::BaseObjectPtrWrapper *(*BaseObjectPtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::BaseObjectPtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return BaseObjectPtrWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$BaseObjectPtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::BaseObjectPtrWrapper *arg0) noexcept {
  void (*BaseObjectPtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::BaseObjectPtrWrapper *) = ::BaseObjectPtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  BaseObjectPtrWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$BaseObjectPtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::BaseObjectPtrWrapper *autocxx_gen_this, ::BaseObjectPtrWrapper *other) noexcept {
  void (*BaseObjectPtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::BaseObjectPtrWrapper *, ::BaseObjectPtrWrapper *) = ::BaseObjectPtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  BaseObjectPtrWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$BaseObjectPtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::BaseObjectPtrWrapper *autocxx_gen_this, ::BaseObjectPtrWrapper const &other) noexcept {
  void (*BaseObjectPtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::BaseObjectPtrWrapper *, ::BaseObjectPtrWrapper const &) = ::BaseObjectPtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  BaseObjectPtrWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::Cloth *cxxbridge1$Cloth_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::Cloth *(*Cloth_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::Cloth_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return Cloth_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$Cloth_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::Cloth *arg0) noexcept {
  void (*Cloth_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::Cloth *) = ::Cloth_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  Cloth_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_Cloth_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::Cloth *autocxx_gen_this, ::alt::Cloth *other) noexcept {
  void (*alt_Cloth_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::Cloth *, ::alt::Cloth *) = ::alt_Cloth_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_Cloth_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_Cloth_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::Cloth *autocxx_gen_this, ::alt::Cloth const &other) noexcept {
  void (*alt_Cloth_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::Cloth *, ::alt::Cloth const &) = ::alt_Cloth_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_Cloth_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::DlcCloth *cxxbridge1$DlcCloth_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::DlcCloth *(*DlcCloth_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::DlcCloth_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return DlcCloth_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$DlcCloth_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::DlcCloth *arg0) noexcept {
  void (*DlcCloth_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::DlcCloth *) = ::DlcCloth_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  DlcCloth_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_DlcCloth_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::DlcCloth *autocxx_gen_this, ::alt::DlcCloth *other) noexcept {
  void (*alt_DlcCloth_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::DlcCloth *, ::alt::DlcCloth *) = ::alt_DlcCloth_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_DlcCloth_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_DlcCloth_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::DlcCloth *autocxx_gen_this, ::alt::DlcCloth const &other) noexcept {
  void (*alt_DlcCloth_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::DlcCloth *, ::alt::DlcCloth const &) = ::alt_DlcCloth_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_DlcCloth_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::Prop *cxxbridge1$Prop_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::Prop *(*Prop_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::Prop_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return Prop_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$Prop_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::Prop *arg0) noexcept {
  void (*Prop_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::Prop *) = ::Prop_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  Prop_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_Prop_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::Prop *autocxx_gen_this, ::alt::Prop *other) noexcept {
  void (*alt_Prop_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::Prop *, ::alt::Prop *) = ::alt_Prop_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_Prop_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_Prop_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::Prop *autocxx_gen_this, ::alt::Prop const &other) noexcept {
  void (*alt_Prop_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::Prop *, ::alt::Prop const &) = ::alt_Prop_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_Prop_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::DlcProp *cxxbridge1$DlcProp_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::DlcProp *(*DlcProp_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::DlcProp_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return DlcProp_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$DlcProp_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::DlcProp *arg0) noexcept {
  void (*DlcProp_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::DlcProp *) = ::DlcProp_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  DlcProp_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_DlcProp_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::DlcProp *autocxx_gen_this, ::alt::DlcProp *other) noexcept {
  void (*alt_DlcProp_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::DlcProp *, ::alt::DlcProp *) = ::alt_DlcProp_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_DlcProp_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_DlcProp_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::DlcProp *autocxx_gen_this, ::alt::DlcProp const &other) noexcept {
  void (*alt_DlcProp_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::DlcProp *, ::alt::DlcProp const &) = ::alt_DlcProp_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_DlcProp_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::HeadOverlay *cxxbridge1$HeadOverlay_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::HeadOverlay *(*HeadOverlay_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::HeadOverlay_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return HeadOverlay_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$HeadOverlay_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::HeadOverlay *arg0) noexcept {
  void (*HeadOverlay_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::HeadOverlay *) = ::HeadOverlay_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  HeadOverlay_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_HeadOverlay_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::HeadOverlay *autocxx_gen_this, ::alt::HeadOverlay *other) noexcept {
  void (*alt_HeadOverlay_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::HeadOverlay *, ::alt::HeadOverlay *) = ::alt_HeadOverlay_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_HeadOverlay_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_HeadOverlay_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::HeadOverlay *autocxx_gen_this, ::alt::HeadOverlay const &other) noexcept {
  void (*alt_HeadOverlay_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::HeadOverlay *, ::alt::HeadOverlay const &) = ::alt_HeadOverlay_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_HeadOverlay_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::HeadBlendData *cxxbridge1$HeadBlendData_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::HeadBlendData *(*HeadBlendData_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::HeadBlendData_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return HeadBlendData_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$HeadBlendData_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::HeadBlendData *arg0) noexcept {
  void (*HeadBlendData_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::HeadBlendData *) = ::HeadBlendData_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  HeadBlendData_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_HeadBlendData_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::HeadBlendData *autocxx_gen_this, ::alt::HeadBlendData *other) noexcept {
  void (*alt_HeadBlendData_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::HeadBlendData *, ::alt::HeadBlendData *) = ::alt_HeadBlendData_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_HeadBlendData_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_HeadBlendData_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::HeadBlendData *autocxx_gen_this, ::alt::HeadBlendData const &other) noexcept {
  void (*alt_HeadBlendData_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::HeadBlendData *, ::alt::HeadBlendData const &) = ::alt_HeadBlendData_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_HeadBlendData_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::StreamedEntityWrapper *cxxbridge1$StreamedEntityWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::StreamedEntityWrapper *(*StreamedEntityWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::StreamedEntityWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return StreamedEntityWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$StreamedEntityWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::StreamedEntityWrapper *arg0) noexcept {
  void (*StreamedEntityWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::StreamedEntityWrapper *) = ::StreamedEntityWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  StreamedEntityWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$StreamedEntityWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::StreamedEntityWrapper *autocxx_gen_this, ::StreamedEntityWrapper *other) noexcept {
  void (*StreamedEntityWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::StreamedEntityWrapper *, ::StreamedEntityWrapper *) = ::StreamedEntityWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  StreamedEntityWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$StreamedEntityWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::StreamedEntityWrapper *autocxx_gen_this, ::StreamedEntityWrapper const &other) noexcept {
  void (*StreamedEntityWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::StreamedEntityWrapper *, ::StreamedEntityWrapper const &) = ::StreamedEntityWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  StreamedEntityWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::AmmoFlags *cxxbridge1$AmmoFlags_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::AmmoFlags *(*AmmoFlags_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::AmmoFlags_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return AmmoFlags_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$AmmoFlags_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::AmmoFlags *arg0) noexcept {
  void (*AmmoFlags_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::AmmoFlags *) = ::AmmoFlags_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  AmmoFlags_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_AmmoFlags_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::AmmoFlags *autocxx_gen_this, ::alt::AmmoFlags *other) noexcept {
  void (*alt_AmmoFlags_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::AmmoFlags *, ::alt::AmmoFlags *) = ::alt_AmmoFlags_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_AmmoFlags_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_AmmoFlags_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::AmmoFlags *autocxx_gen_this, ::alt::AmmoFlags const &other) noexcept {
  void (*alt_AmmoFlags_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::AmmoFlags *, ::alt::AmmoFlags const &) = ::alt_AmmoFlags_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_AmmoFlags_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::CDecoration *cxxbridge1$CDecoration_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::CDecoration *(*CDecoration_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::CDecoration_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return CDecoration_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$CDecoration_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CDecoration *arg0) noexcept {
  void (*CDecoration_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CDecoration *) = ::CDecoration_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  CDecoration_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_CDecoration_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CDecoration *autocxx_gen_this, ::alt::CDecoration *other) noexcept {
  void (*alt_CDecoration_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CDecoration *, ::alt::CDecoration *) = ::alt_CDecoration_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CDecoration_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_CDecoration_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::CDecoration *autocxx_gen_this, ::alt::CDecoration const &other) noexcept {
  void (*alt_CDecoration_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::CDecoration *, ::alt::CDecoration const &) = ::alt_CDecoration_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_CDecoration_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::Quaternion *cxxbridge1$Quaternion_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::Quaternion *(*Quaternion_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::Quaternion_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return Quaternion_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$Quaternion_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::Quaternion *arg0) noexcept {
  void (*Quaternion_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::Quaternion *) = ::Quaternion_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  Quaternion_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_Quaternion_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::Quaternion *autocxx_gen_this, ::alt::Quaternion *other) noexcept {
  void (*alt_Quaternion_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::Quaternion *, ::alt::Quaternion *) = ::alt_Quaternion_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_Quaternion_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_Quaternion_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::Quaternion *autocxx_gen_this, ::alt::Quaternion const &other) noexcept {
  void (*alt_Quaternion_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::Quaternion *, ::alt::Quaternion const &) = ::alt_Quaternion_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_Quaternion_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::EntityAnimHashPairsWrapper *cxxbridge1$EntityAnimHashPairsWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::EntityAnimHashPairsWrapper *(*EntityAnimHashPairsWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::EntityAnimHashPairsWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return EntityAnimHashPairsWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$EntityAnimHashPairsWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::EntityAnimHashPairsWrapper *arg0) noexcept {
  void (*EntityAnimHashPairsWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::EntityAnimHashPairsWrapper *) = ::EntityAnimHashPairsWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  EntityAnimHashPairsWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

::MValueDictPairWrapper *cxxbridge1$MValueDictPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::MValueDictPairWrapper *(*MValueDictPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::MValueDictPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return MValueDictPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$MValueDictPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueDictPairWrapper *arg0) noexcept {
  void (*MValueDictPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueDictPairWrapper *) = ::MValueDictPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  MValueDictPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$MValueDictPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueDictPairWrapper *autocxx_gen_this, ::MValueDictPairWrapper *other) noexcept {
  void (*MValueDictPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueDictPairWrapper *, ::MValueDictPairWrapper *) = ::MValueDictPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  MValueDictPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$MValueDictPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::MValueDictPairWrapper *autocxx_gen_this, ::MValueDictPairWrapper const &other) noexcept {
  void (*MValueDictPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::MValueDictPairWrapper *, ::MValueDictPairWrapper const &) = ::MValueDictPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  MValueDictPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::EntityAnimHashPairWrapper *cxxbridge1$EntityAnimHashPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::EntityAnimHashPairWrapper *(*EntityAnimHashPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::EntityAnimHashPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return EntityAnimHashPairWrapper_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$EntityAnimHashPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a(::EntityAnimHashPairWrapper *arg0) noexcept {
  void (*EntityAnimHashPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::EntityAnimHashPairWrapper *) = ::EntityAnimHashPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  EntityAnimHashPairWrapper_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$EntityAnimHashPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::EntityAnimHashPairWrapper *autocxx_gen_this, ::EntityAnimHashPairWrapper *other) noexcept {
  void (*EntityAnimHashPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::EntityAnimHashPairWrapper *, ::EntityAnimHashPairWrapper *) = ::EntityAnimHashPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  EntityAnimHashPairWrapper_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$EntityAnimHashPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::EntityAnimHashPairWrapper *autocxx_gen_this, ::EntityAnimHashPairWrapper const &other) noexcept {
  void (*EntityAnimHashPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::EntityAnimHashPairWrapper *, ::EntityAnimHashPairWrapper const &) = ::EntityAnimHashPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  EntityAnimHashPairWrapper_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

::alt::BoneInfo *cxxbridge1$BoneInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a() noexcept {
  ::alt::BoneInfo *(*BoneInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$)() = ::BoneInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a;
  return BoneInfo_alloc_autocxx_wrapper_0xd5d0abec981e3e3a$();
}

void cxxbridge1$BoneInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::BoneInfo *arg0) noexcept {
  void (*BoneInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::BoneInfo *) = ::BoneInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a;
  BoneInfo_free_autocxx_wrapper_0xd5d0abec981e3e3a$(arg0);
}

void cxxbridge1$alt_BoneInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::BoneInfo *autocxx_gen_this, ::alt::BoneInfo *other) noexcept {
  void (*alt_BoneInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::BoneInfo *, ::alt::BoneInfo *) = ::alt_BoneInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_BoneInfo_new_synthetic_move_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

void cxxbridge1$alt_BoneInfo_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a(::alt::BoneInfo *autocxx_gen_this, ::alt::BoneInfo const &other) noexcept {
  void (*alt_BoneInfo_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$)(::alt::BoneInfo *, ::alt::BoneInfo const &) = ::alt_BoneInfo_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a;
  alt_BoneInfo_new_synthetic_const_copy_ctor_0xd5d0abec981e3e3a_autocxx_wrapper_0xd5d0abec981e3e3a$(autocxx_gen_this, other);
}

static_assert(::rust::detail::is_complete<::ConstMValueWrapper>::value, "definition of ConstMValueWrapper is required");
static_assert(sizeof(::std::unique_ptr<::ConstMValueWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::ConstMValueWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$ConstMValueWrapper$null(::std::unique_ptr<::ConstMValueWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::ConstMValueWrapper>();
}
::ConstMValueWrapper *cxxbridge1$unique_ptr$ConstMValueWrapper$uninit(::std::unique_ptr<::ConstMValueWrapper> *ptr) noexcept {
  ::ConstMValueWrapper *uninit = reinterpret_cast<::ConstMValueWrapper *>(new ::rust::MaybeUninit<::ConstMValueWrapper>);
  ::new (ptr) ::std::unique_ptr<::ConstMValueWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$ConstMValueWrapper$raw(::std::unique_ptr<::ConstMValueWrapper> *ptr, ::ConstMValueWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::ConstMValueWrapper>(raw);
}
::ConstMValueWrapper const *cxxbridge1$unique_ptr$ConstMValueWrapper$get(::std::unique_ptr<::ConstMValueWrapper> const &ptr) noexcept {
  return ptr.get();
}
::ConstMValueWrapper *cxxbridge1$unique_ptr$ConstMValueWrapper$release(::std::unique_ptr<::ConstMValueWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$ConstMValueWrapper$drop(::std::unique_ptr<::ConstMValueWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::ConstMValueWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::ConstMValueWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::ConstMValueWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$ConstMValueWrapper$null(::std::shared_ptr<::ConstMValueWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::ConstMValueWrapper>();
}
::ConstMValueWrapper *cxxbridge1$shared_ptr$ConstMValueWrapper$uninit(::std::shared_ptr<::ConstMValueWrapper> *ptr) noexcept {
  ::ConstMValueWrapper *uninit = reinterpret_cast<::ConstMValueWrapper *>(new ::rust::MaybeUninit<::ConstMValueWrapper>);
  ::new (ptr) ::std::shared_ptr<::ConstMValueWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$ConstMValueWrapper$clone(::std::shared_ptr<::ConstMValueWrapper> const &self, ::std::shared_ptr<::ConstMValueWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::ConstMValueWrapper>(self);
}
::ConstMValueWrapper const *cxxbridge1$shared_ptr$ConstMValueWrapper$get(::std::shared_ptr<::ConstMValueWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$ConstMValueWrapper$drop(::std::shared_ptr<::ConstMValueWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::ConstMValueWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::ConstMValueWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$ConstMValueWrapper$null(::std::weak_ptr<::ConstMValueWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::ConstMValueWrapper>();
}
void cxxbridge1$weak_ptr$ConstMValueWrapper$clone(::std::weak_ptr<::ConstMValueWrapper> const &self, ::std::weak_ptr<::ConstMValueWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::ConstMValueWrapper>(self);
}
void cxxbridge1$weak_ptr$ConstMValueWrapper$downgrade(::std::shared_ptr<::ConstMValueWrapper> const &shared, ::std::weak_ptr<::ConstMValueWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::ConstMValueWrapper>(shared);
}
void cxxbridge1$weak_ptr$ConstMValueWrapper$upgrade(::std::weak_ptr<::ConstMValueWrapper> const &weak, ::std::shared_ptr<::ConstMValueWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::ConstMValueWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$ConstMValueWrapper$drop(::std::weak_ptr<::ConstMValueWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::ConstMValueWrapper> *cxxbridge1$std$vector$ConstMValueWrapper$new() noexcept {
  return new ::std::vector<::ConstMValueWrapper>();
}
::std::size_t cxxbridge1$std$vector$ConstMValueWrapper$size(::std::vector<::ConstMValueWrapper> const &s) noexcept {
  return s.size();
}
::ConstMValueWrapper *cxxbridge1$std$vector$ConstMValueWrapper$get_unchecked(::std::vector<::ConstMValueWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$ConstMValueWrapper$push_back(::std::vector<::ConstMValueWrapper> *v, ::ConstMValueWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$ConstMValueWrapper$pop_back(::std::vector<::ConstMValueWrapper> *v, ::ConstMValueWrapper *out) noexcept {
  ::new (out) ::ConstMValueWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::ConstMValueWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::ConstMValueWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$ConstMValueWrapper$null(::std::unique_ptr<::std::vector<::ConstMValueWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::ConstMValueWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$ConstMValueWrapper$raw(::std::unique_ptr<::std::vector<::ConstMValueWrapper>> *ptr, ::std::vector<::ConstMValueWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::ConstMValueWrapper>>(raw);
}
::std::vector<::ConstMValueWrapper> const *cxxbridge1$unique_ptr$std$vector$ConstMValueWrapper$get(::std::unique_ptr<::std::vector<::ConstMValueWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::ConstMValueWrapper> *cxxbridge1$unique_ptr$std$vector$ConstMValueWrapper$release(::std::unique_ptr<::std::vector<::ConstMValueWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$ConstMValueWrapper$drop(::std::unique_ptr<::std::vector<::ConstMValueWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::ConfigDictPairWrapper>::value, "definition of ConfigDictPairWrapper is required");
static_assert(sizeof(::std::unique_ptr<::ConfigDictPairWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::ConfigDictPairWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$ConfigDictPairWrapper$null(::std::unique_ptr<::ConfigDictPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::ConfigDictPairWrapper>();
}
::ConfigDictPairWrapper *cxxbridge1$unique_ptr$ConfigDictPairWrapper$uninit(::std::unique_ptr<::ConfigDictPairWrapper> *ptr) noexcept {
  ::ConfigDictPairWrapper *uninit = reinterpret_cast<::ConfigDictPairWrapper *>(new ::rust::MaybeUninit<::ConfigDictPairWrapper>);
  ::new (ptr) ::std::unique_ptr<::ConfigDictPairWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$ConfigDictPairWrapper$raw(::std::unique_ptr<::ConfigDictPairWrapper> *ptr, ::ConfigDictPairWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::ConfigDictPairWrapper>(raw);
}
::ConfigDictPairWrapper const *cxxbridge1$unique_ptr$ConfigDictPairWrapper$get(::std::unique_ptr<::ConfigDictPairWrapper> const &ptr) noexcept {
  return ptr.get();
}
::ConfigDictPairWrapper *cxxbridge1$unique_ptr$ConfigDictPairWrapper$release(::std::unique_ptr<::ConfigDictPairWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$ConfigDictPairWrapper$drop(::std::unique_ptr<::ConfigDictPairWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::ConfigDictPairWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::ConfigDictPairWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::ConfigDictPairWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$ConfigDictPairWrapper$null(::std::shared_ptr<::ConfigDictPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::ConfigDictPairWrapper>();
}
::ConfigDictPairWrapper *cxxbridge1$shared_ptr$ConfigDictPairWrapper$uninit(::std::shared_ptr<::ConfigDictPairWrapper> *ptr) noexcept {
  ::ConfigDictPairWrapper *uninit = reinterpret_cast<::ConfigDictPairWrapper *>(new ::rust::MaybeUninit<::ConfigDictPairWrapper>);
  ::new (ptr) ::std::shared_ptr<::ConfigDictPairWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$ConfigDictPairWrapper$clone(::std::shared_ptr<::ConfigDictPairWrapper> const &self, ::std::shared_ptr<::ConfigDictPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::ConfigDictPairWrapper>(self);
}
::ConfigDictPairWrapper const *cxxbridge1$shared_ptr$ConfigDictPairWrapper$get(::std::shared_ptr<::ConfigDictPairWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$ConfigDictPairWrapper$drop(::std::shared_ptr<::ConfigDictPairWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::ConfigDictPairWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::ConfigDictPairWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$ConfigDictPairWrapper$null(::std::weak_ptr<::ConfigDictPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::ConfigDictPairWrapper>();
}
void cxxbridge1$weak_ptr$ConfigDictPairWrapper$clone(::std::weak_ptr<::ConfigDictPairWrapper> const &self, ::std::weak_ptr<::ConfigDictPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::ConfigDictPairWrapper>(self);
}
void cxxbridge1$weak_ptr$ConfigDictPairWrapper$downgrade(::std::shared_ptr<::ConfigDictPairWrapper> const &shared, ::std::weak_ptr<::ConfigDictPairWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::ConfigDictPairWrapper>(shared);
}
void cxxbridge1$weak_ptr$ConfigDictPairWrapper$upgrade(::std::weak_ptr<::ConfigDictPairWrapper> const &weak, ::std::shared_ptr<::ConfigDictPairWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::ConfigDictPairWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$ConfigDictPairWrapper$drop(::std::weak_ptr<::ConfigDictPairWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::ConfigDictPairWrapper> *cxxbridge1$std$vector$ConfigDictPairWrapper$new() noexcept {
  return new ::std::vector<::ConfigDictPairWrapper>();
}
::std::size_t cxxbridge1$std$vector$ConfigDictPairWrapper$size(::std::vector<::ConfigDictPairWrapper> const &s) noexcept {
  return s.size();
}
::ConfigDictPairWrapper *cxxbridge1$std$vector$ConfigDictPairWrapper$get_unchecked(::std::vector<::ConfigDictPairWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$ConfigDictPairWrapper$push_back(::std::vector<::ConfigDictPairWrapper> *v, ::ConfigDictPairWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$ConfigDictPairWrapper$pop_back(::std::vector<::ConfigDictPairWrapper> *v, ::ConfigDictPairWrapper *out) noexcept {
  ::new (out) ::ConfigDictPairWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::ConfigDictPairWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::ConfigDictPairWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$ConfigDictPairWrapper$null(::std::unique_ptr<::std::vector<::ConfigDictPairWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::ConfigDictPairWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$ConfigDictPairWrapper$raw(::std::unique_ptr<::std::vector<::ConfigDictPairWrapper>> *ptr, ::std::vector<::ConfigDictPairWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::ConfigDictPairWrapper>>(raw);
}
::std::vector<::ConfigDictPairWrapper> const *cxxbridge1$unique_ptr$std$vector$ConfigDictPairWrapper$get(::std::unique_ptr<::std::vector<::ConfigDictPairWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::ConfigDictPairWrapper> *cxxbridge1$unique_ptr$std$vector$ConfigDictPairWrapper$release(::std::unique_ptr<::std::vector<::ConfigDictPairWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$ConfigDictPairWrapper$drop(::std::unique_ptr<::std::vector<::ConfigDictPairWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::Vector3Wrapper>::value, "definition of Vector3Wrapper is required");
static_assert(sizeof(::std::unique_ptr<::Vector3Wrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::Vector3Wrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$Vector3Wrapper$null(::std::unique_ptr<::Vector3Wrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::Vector3Wrapper>();
}
::Vector3Wrapper *cxxbridge1$unique_ptr$Vector3Wrapper$uninit(::std::unique_ptr<::Vector3Wrapper> *ptr) noexcept {
  ::Vector3Wrapper *uninit = reinterpret_cast<::Vector3Wrapper *>(new ::rust::MaybeUninit<::Vector3Wrapper>);
  ::new (ptr) ::std::unique_ptr<::Vector3Wrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$Vector3Wrapper$raw(::std::unique_ptr<::Vector3Wrapper> *ptr, ::Vector3Wrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::Vector3Wrapper>(raw);
}
::Vector3Wrapper const *cxxbridge1$unique_ptr$Vector3Wrapper$get(::std::unique_ptr<::Vector3Wrapper> const &ptr) noexcept {
  return ptr.get();
}
::Vector3Wrapper *cxxbridge1$unique_ptr$Vector3Wrapper$release(::std::unique_ptr<::Vector3Wrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$Vector3Wrapper$drop(::std::unique_ptr<::Vector3Wrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::Vector3Wrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::Vector3Wrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::Vector3Wrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$Vector3Wrapper$null(::std::shared_ptr<::Vector3Wrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::Vector3Wrapper>();
}
::Vector3Wrapper *cxxbridge1$shared_ptr$Vector3Wrapper$uninit(::std::shared_ptr<::Vector3Wrapper> *ptr) noexcept {
  ::Vector3Wrapper *uninit = reinterpret_cast<::Vector3Wrapper *>(new ::rust::MaybeUninit<::Vector3Wrapper>);
  ::new (ptr) ::std::shared_ptr<::Vector3Wrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$Vector3Wrapper$clone(::std::shared_ptr<::Vector3Wrapper> const &self, ::std::shared_ptr<::Vector3Wrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::Vector3Wrapper>(self);
}
::Vector3Wrapper const *cxxbridge1$shared_ptr$Vector3Wrapper$get(::std::shared_ptr<::Vector3Wrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$Vector3Wrapper$drop(::std::shared_ptr<::Vector3Wrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::Vector3Wrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::Vector3Wrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$Vector3Wrapper$null(::std::weak_ptr<::Vector3Wrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::Vector3Wrapper>();
}
void cxxbridge1$weak_ptr$Vector3Wrapper$clone(::std::weak_ptr<::Vector3Wrapper> const &self, ::std::weak_ptr<::Vector3Wrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::Vector3Wrapper>(self);
}
void cxxbridge1$weak_ptr$Vector3Wrapper$downgrade(::std::shared_ptr<::Vector3Wrapper> const &shared, ::std::weak_ptr<::Vector3Wrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::Vector3Wrapper>(shared);
}
void cxxbridge1$weak_ptr$Vector3Wrapper$upgrade(::std::weak_ptr<::Vector3Wrapper> const &weak, ::std::shared_ptr<::Vector3Wrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::Vector3Wrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$Vector3Wrapper$drop(::std::weak_ptr<::Vector3Wrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::Vector3Wrapper> *cxxbridge1$std$vector$Vector3Wrapper$new() noexcept {
  return new ::std::vector<::Vector3Wrapper>();
}
::std::size_t cxxbridge1$std$vector$Vector3Wrapper$size(::std::vector<::Vector3Wrapper> const &s) noexcept {
  return s.size();
}
::Vector3Wrapper *cxxbridge1$std$vector$Vector3Wrapper$get_unchecked(::std::vector<::Vector3Wrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$Vector3Wrapper$push_back(::std::vector<::Vector3Wrapper> *v, ::Vector3Wrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$Vector3Wrapper$pop_back(::std::vector<::Vector3Wrapper> *v, ::Vector3Wrapper *out) noexcept {
  ::new (out) ::Vector3Wrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::Vector3Wrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::Vector3Wrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$Vector3Wrapper$null(::std::unique_ptr<::std::vector<::Vector3Wrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::Vector3Wrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$Vector3Wrapper$raw(::std::unique_ptr<::std::vector<::Vector3Wrapper>> *ptr, ::std::vector<::Vector3Wrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::Vector3Wrapper>>(raw);
}
::std::vector<::Vector3Wrapper> const *cxxbridge1$unique_ptr$std$vector$Vector3Wrapper$get(::std::unique_ptr<::std::vector<::Vector3Wrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::Vector3Wrapper> *cxxbridge1$unique_ptr$std$vector$Vector3Wrapper$release(::std::unique_ptr<::std::vector<::Vector3Wrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$Vector3Wrapper$drop(::std::unique_ptr<::std::vector<::Vector3Wrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::Vector2Wrapper>::value, "definition of Vector2Wrapper is required");
static_assert(sizeof(::std::unique_ptr<::Vector2Wrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::Vector2Wrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$Vector2Wrapper$null(::std::unique_ptr<::Vector2Wrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::Vector2Wrapper>();
}
::Vector2Wrapper *cxxbridge1$unique_ptr$Vector2Wrapper$uninit(::std::unique_ptr<::Vector2Wrapper> *ptr) noexcept {
  ::Vector2Wrapper *uninit = reinterpret_cast<::Vector2Wrapper *>(new ::rust::MaybeUninit<::Vector2Wrapper>);
  ::new (ptr) ::std::unique_ptr<::Vector2Wrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$Vector2Wrapper$raw(::std::unique_ptr<::Vector2Wrapper> *ptr, ::Vector2Wrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::Vector2Wrapper>(raw);
}
::Vector2Wrapper const *cxxbridge1$unique_ptr$Vector2Wrapper$get(::std::unique_ptr<::Vector2Wrapper> const &ptr) noexcept {
  return ptr.get();
}
::Vector2Wrapper *cxxbridge1$unique_ptr$Vector2Wrapper$release(::std::unique_ptr<::Vector2Wrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$Vector2Wrapper$drop(::std::unique_ptr<::Vector2Wrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::Vector2Wrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::Vector2Wrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::Vector2Wrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$Vector2Wrapper$null(::std::shared_ptr<::Vector2Wrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::Vector2Wrapper>();
}
::Vector2Wrapper *cxxbridge1$shared_ptr$Vector2Wrapper$uninit(::std::shared_ptr<::Vector2Wrapper> *ptr) noexcept {
  ::Vector2Wrapper *uninit = reinterpret_cast<::Vector2Wrapper *>(new ::rust::MaybeUninit<::Vector2Wrapper>);
  ::new (ptr) ::std::shared_ptr<::Vector2Wrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$Vector2Wrapper$clone(::std::shared_ptr<::Vector2Wrapper> const &self, ::std::shared_ptr<::Vector2Wrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::Vector2Wrapper>(self);
}
::Vector2Wrapper const *cxxbridge1$shared_ptr$Vector2Wrapper$get(::std::shared_ptr<::Vector2Wrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$Vector2Wrapper$drop(::std::shared_ptr<::Vector2Wrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::Vector2Wrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::Vector2Wrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$Vector2Wrapper$null(::std::weak_ptr<::Vector2Wrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::Vector2Wrapper>();
}
void cxxbridge1$weak_ptr$Vector2Wrapper$clone(::std::weak_ptr<::Vector2Wrapper> const &self, ::std::weak_ptr<::Vector2Wrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::Vector2Wrapper>(self);
}
void cxxbridge1$weak_ptr$Vector2Wrapper$downgrade(::std::shared_ptr<::Vector2Wrapper> const &shared, ::std::weak_ptr<::Vector2Wrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::Vector2Wrapper>(shared);
}
void cxxbridge1$weak_ptr$Vector2Wrapper$upgrade(::std::weak_ptr<::Vector2Wrapper> const &weak, ::std::shared_ptr<::Vector2Wrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::Vector2Wrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$Vector2Wrapper$drop(::std::weak_ptr<::Vector2Wrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::Vector2Wrapper> *cxxbridge1$std$vector$Vector2Wrapper$new() noexcept {
  return new ::std::vector<::Vector2Wrapper>();
}
::std::size_t cxxbridge1$std$vector$Vector2Wrapper$size(::std::vector<::Vector2Wrapper> const &s) noexcept {
  return s.size();
}
::Vector2Wrapper *cxxbridge1$std$vector$Vector2Wrapper$get_unchecked(::std::vector<::Vector2Wrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$Vector2Wrapper$push_back(::std::vector<::Vector2Wrapper> *v, ::Vector2Wrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$Vector2Wrapper$pop_back(::std::vector<::Vector2Wrapper> *v, ::Vector2Wrapper *out) noexcept {
  ::new (out) ::Vector2Wrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::Vector2Wrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::Vector2Wrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$Vector2Wrapper$null(::std::unique_ptr<::std::vector<::Vector2Wrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::Vector2Wrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$Vector2Wrapper$raw(::std::unique_ptr<::std::vector<::Vector2Wrapper>> *ptr, ::std::vector<::Vector2Wrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::Vector2Wrapper>>(raw);
}
::std::vector<::Vector2Wrapper> const *cxxbridge1$unique_ptr$std$vector$Vector2Wrapper$get(::std::unique_ptr<::std::vector<::Vector2Wrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::Vector2Wrapper> *cxxbridge1$unique_ptr$std$vector$Vector2Wrapper$release(::std::unique_ptr<::std::vector<::Vector2Wrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$Vector2Wrapper$drop(::std::unique_ptr<::std::vector<::Vector2Wrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::RGBAWrapper>::value, "definition of RGBAWrapper is required");
static_assert(sizeof(::std::unique_ptr<::RGBAWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::RGBAWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$RGBAWrapper$null(::std::unique_ptr<::RGBAWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::RGBAWrapper>();
}
::RGBAWrapper *cxxbridge1$unique_ptr$RGBAWrapper$uninit(::std::unique_ptr<::RGBAWrapper> *ptr) noexcept {
  ::RGBAWrapper *uninit = reinterpret_cast<::RGBAWrapper *>(new ::rust::MaybeUninit<::RGBAWrapper>);
  ::new (ptr) ::std::unique_ptr<::RGBAWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$RGBAWrapper$raw(::std::unique_ptr<::RGBAWrapper> *ptr, ::RGBAWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::RGBAWrapper>(raw);
}
::RGBAWrapper const *cxxbridge1$unique_ptr$RGBAWrapper$get(::std::unique_ptr<::RGBAWrapper> const &ptr) noexcept {
  return ptr.get();
}
::RGBAWrapper *cxxbridge1$unique_ptr$RGBAWrapper$release(::std::unique_ptr<::RGBAWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$RGBAWrapper$drop(::std::unique_ptr<::RGBAWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::RGBAWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::RGBAWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::RGBAWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$RGBAWrapper$null(::std::shared_ptr<::RGBAWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::RGBAWrapper>();
}
::RGBAWrapper *cxxbridge1$shared_ptr$RGBAWrapper$uninit(::std::shared_ptr<::RGBAWrapper> *ptr) noexcept {
  ::RGBAWrapper *uninit = reinterpret_cast<::RGBAWrapper *>(new ::rust::MaybeUninit<::RGBAWrapper>);
  ::new (ptr) ::std::shared_ptr<::RGBAWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$RGBAWrapper$clone(::std::shared_ptr<::RGBAWrapper> const &self, ::std::shared_ptr<::RGBAWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::RGBAWrapper>(self);
}
::RGBAWrapper const *cxxbridge1$shared_ptr$RGBAWrapper$get(::std::shared_ptr<::RGBAWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$RGBAWrapper$drop(::std::shared_ptr<::RGBAWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::RGBAWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::RGBAWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$RGBAWrapper$null(::std::weak_ptr<::RGBAWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::RGBAWrapper>();
}
void cxxbridge1$weak_ptr$RGBAWrapper$clone(::std::weak_ptr<::RGBAWrapper> const &self, ::std::weak_ptr<::RGBAWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::RGBAWrapper>(self);
}
void cxxbridge1$weak_ptr$RGBAWrapper$downgrade(::std::shared_ptr<::RGBAWrapper> const &shared, ::std::weak_ptr<::RGBAWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::RGBAWrapper>(shared);
}
void cxxbridge1$weak_ptr$RGBAWrapper$upgrade(::std::weak_ptr<::RGBAWrapper> const &weak, ::std::shared_ptr<::RGBAWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::RGBAWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$RGBAWrapper$drop(::std::weak_ptr<::RGBAWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::RGBAWrapper> *cxxbridge1$std$vector$RGBAWrapper$new() noexcept {
  return new ::std::vector<::RGBAWrapper>();
}
::std::size_t cxxbridge1$std$vector$RGBAWrapper$size(::std::vector<::RGBAWrapper> const &s) noexcept {
  return s.size();
}
::RGBAWrapper *cxxbridge1$std$vector$RGBAWrapper$get_unchecked(::std::vector<::RGBAWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$RGBAWrapper$push_back(::std::vector<::RGBAWrapper> *v, ::RGBAWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$RGBAWrapper$pop_back(::std::vector<::RGBAWrapper> *v, ::RGBAWrapper *out) noexcept {
  ::new (out) ::RGBAWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::RGBAWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::RGBAWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$RGBAWrapper$null(::std::unique_ptr<::std::vector<::RGBAWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::RGBAWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$RGBAWrapper$raw(::std::unique_ptr<::std::vector<::RGBAWrapper>> *ptr, ::std::vector<::RGBAWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::RGBAWrapper>>(raw);
}
::std::vector<::RGBAWrapper> const *cxxbridge1$unique_ptr$std$vector$RGBAWrapper$get(::std::unique_ptr<::std::vector<::RGBAWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::RGBAWrapper> *cxxbridge1$unique_ptr$std$vector$RGBAWrapper$release(::std::unique_ptr<::std::vector<::RGBAWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$RGBAWrapper$drop(::std::unique_ptr<::std::vector<::RGBAWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::WeaponWrapper>::value, "definition of WeaponWrapper is required");
static_assert(sizeof(::std::unique_ptr<::WeaponWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::WeaponWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$WeaponWrapper$null(::std::unique_ptr<::WeaponWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::WeaponWrapper>();
}
::WeaponWrapper *cxxbridge1$unique_ptr$WeaponWrapper$uninit(::std::unique_ptr<::WeaponWrapper> *ptr) noexcept {
  ::WeaponWrapper *uninit = reinterpret_cast<::WeaponWrapper *>(new ::rust::MaybeUninit<::WeaponWrapper>);
  ::new (ptr) ::std::unique_ptr<::WeaponWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$WeaponWrapper$raw(::std::unique_ptr<::WeaponWrapper> *ptr, ::WeaponWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::WeaponWrapper>(raw);
}
::WeaponWrapper const *cxxbridge1$unique_ptr$WeaponWrapper$get(::std::unique_ptr<::WeaponWrapper> const &ptr) noexcept {
  return ptr.get();
}
::WeaponWrapper *cxxbridge1$unique_ptr$WeaponWrapper$release(::std::unique_ptr<::WeaponWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$WeaponWrapper$drop(::std::unique_ptr<::WeaponWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::WeaponWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::WeaponWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::WeaponWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$WeaponWrapper$null(::std::shared_ptr<::WeaponWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::WeaponWrapper>();
}
::WeaponWrapper *cxxbridge1$shared_ptr$WeaponWrapper$uninit(::std::shared_ptr<::WeaponWrapper> *ptr) noexcept {
  ::WeaponWrapper *uninit = reinterpret_cast<::WeaponWrapper *>(new ::rust::MaybeUninit<::WeaponWrapper>);
  ::new (ptr) ::std::shared_ptr<::WeaponWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$WeaponWrapper$clone(::std::shared_ptr<::WeaponWrapper> const &self, ::std::shared_ptr<::WeaponWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::WeaponWrapper>(self);
}
::WeaponWrapper const *cxxbridge1$shared_ptr$WeaponWrapper$get(::std::shared_ptr<::WeaponWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$WeaponWrapper$drop(::std::shared_ptr<::WeaponWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::WeaponWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::WeaponWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$WeaponWrapper$null(::std::weak_ptr<::WeaponWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::WeaponWrapper>();
}
void cxxbridge1$weak_ptr$WeaponWrapper$clone(::std::weak_ptr<::WeaponWrapper> const &self, ::std::weak_ptr<::WeaponWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::WeaponWrapper>(self);
}
void cxxbridge1$weak_ptr$WeaponWrapper$downgrade(::std::shared_ptr<::WeaponWrapper> const &shared, ::std::weak_ptr<::WeaponWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::WeaponWrapper>(shared);
}
void cxxbridge1$weak_ptr$WeaponWrapper$upgrade(::std::weak_ptr<::WeaponWrapper> const &weak, ::std::shared_ptr<::WeaponWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::WeaponWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$WeaponWrapper$drop(::std::weak_ptr<::WeaponWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::WeaponWrapper> *cxxbridge1$std$vector$WeaponWrapper$new() noexcept {
  return new ::std::vector<::WeaponWrapper>();
}
::std::size_t cxxbridge1$std$vector$WeaponWrapper$size(::std::vector<::WeaponWrapper> const &s) noexcept {
  return s.size();
}
::WeaponWrapper *cxxbridge1$std$vector$WeaponWrapper$get_unchecked(::std::vector<::WeaponWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$WeaponWrapper$push_back(::std::vector<::WeaponWrapper> *v, ::WeaponWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$WeaponWrapper$pop_back(::std::vector<::WeaponWrapper> *v, ::WeaponWrapper *out) noexcept {
  ::new (out) ::WeaponWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::WeaponWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::WeaponWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$WeaponWrapper$null(::std::unique_ptr<::std::vector<::WeaponWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::WeaponWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$WeaponWrapper$raw(::std::unique_ptr<::std::vector<::WeaponWrapper>> *ptr, ::std::vector<::WeaponWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::WeaponWrapper>>(raw);
}
::std::vector<::WeaponWrapper> const *cxxbridge1$unique_ptr$std$vector$WeaponWrapper$get(::std::unique_ptr<::std::vector<::WeaponWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::WeaponWrapper> *cxxbridge1$unique_ptr$std$vector$WeaponWrapper$release(::std::unique_ptr<::std::vector<::WeaponWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$WeaponWrapper$drop(::std::unique_ptr<::std::vector<::WeaponWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::FireInfoWrapper>::value, "definition of FireInfoWrapper is required");
static_assert(sizeof(::std::unique_ptr<::FireInfoWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::FireInfoWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$FireInfoWrapper$null(::std::unique_ptr<::FireInfoWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::FireInfoWrapper>();
}
::FireInfoWrapper *cxxbridge1$unique_ptr$FireInfoWrapper$uninit(::std::unique_ptr<::FireInfoWrapper> *ptr) noexcept {
  ::FireInfoWrapper *uninit = reinterpret_cast<::FireInfoWrapper *>(new ::rust::MaybeUninit<::FireInfoWrapper>);
  ::new (ptr) ::std::unique_ptr<::FireInfoWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$FireInfoWrapper$raw(::std::unique_ptr<::FireInfoWrapper> *ptr, ::FireInfoWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::FireInfoWrapper>(raw);
}
::FireInfoWrapper const *cxxbridge1$unique_ptr$FireInfoWrapper$get(::std::unique_ptr<::FireInfoWrapper> const &ptr) noexcept {
  return ptr.get();
}
::FireInfoWrapper *cxxbridge1$unique_ptr$FireInfoWrapper$release(::std::unique_ptr<::FireInfoWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$FireInfoWrapper$drop(::std::unique_ptr<::FireInfoWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::FireInfoWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::FireInfoWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::FireInfoWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$FireInfoWrapper$null(::std::shared_ptr<::FireInfoWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::FireInfoWrapper>();
}
::FireInfoWrapper *cxxbridge1$shared_ptr$FireInfoWrapper$uninit(::std::shared_ptr<::FireInfoWrapper> *ptr) noexcept {
  ::FireInfoWrapper *uninit = reinterpret_cast<::FireInfoWrapper *>(new ::rust::MaybeUninit<::FireInfoWrapper>);
  ::new (ptr) ::std::shared_ptr<::FireInfoWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$FireInfoWrapper$clone(::std::shared_ptr<::FireInfoWrapper> const &self, ::std::shared_ptr<::FireInfoWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::FireInfoWrapper>(self);
}
::FireInfoWrapper const *cxxbridge1$shared_ptr$FireInfoWrapper$get(::std::shared_ptr<::FireInfoWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$FireInfoWrapper$drop(::std::shared_ptr<::FireInfoWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::FireInfoWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::FireInfoWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$FireInfoWrapper$null(::std::weak_ptr<::FireInfoWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::FireInfoWrapper>();
}
void cxxbridge1$weak_ptr$FireInfoWrapper$clone(::std::weak_ptr<::FireInfoWrapper> const &self, ::std::weak_ptr<::FireInfoWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::FireInfoWrapper>(self);
}
void cxxbridge1$weak_ptr$FireInfoWrapper$downgrade(::std::shared_ptr<::FireInfoWrapper> const &shared, ::std::weak_ptr<::FireInfoWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::FireInfoWrapper>(shared);
}
void cxxbridge1$weak_ptr$FireInfoWrapper$upgrade(::std::weak_ptr<::FireInfoWrapper> const &weak, ::std::shared_ptr<::FireInfoWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::FireInfoWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$FireInfoWrapper$drop(::std::weak_ptr<::FireInfoWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::FireInfoWrapper> *cxxbridge1$std$vector$FireInfoWrapper$new() noexcept {
  return new ::std::vector<::FireInfoWrapper>();
}
::std::size_t cxxbridge1$std$vector$FireInfoWrapper$size(::std::vector<::FireInfoWrapper> const &s) noexcept {
  return s.size();
}
::FireInfoWrapper *cxxbridge1$std$vector$FireInfoWrapper$get_unchecked(::std::vector<::FireInfoWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$FireInfoWrapper$push_back(::std::vector<::FireInfoWrapper> *v, ::FireInfoWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$FireInfoWrapper$pop_back(::std::vector<::FireInfoWrapper> *v, ::FireInfoWrapper *out) noexcept {
  ::new (out) ::FireInfoWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::FireInfoWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::FireInfoWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$FireInfoWrapper$null(::std::unique_ptr<::std::vector<::FireInfoWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::FireInfoWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$FireInfoWrapper$raw(::std::unique_ptr<::std::vector<::FireInfoWrapper>> *ptr, ::std::vector<::FireInfoWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::FireInfoWrapper>>(raw);
}
::std::vector<::FireInfoWrapper> const *cxxbridge1$unique_ptr$std$vector$FireInfoWrapper$get(::std::unique_ptr<::std::vector<::FireInfoWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::FireInfoWrapper> *cxxbridge1$unique_ptr$std$vector$FireInfoWrapper$release(::std::unique_ptr<::std::vector<::FireInfoWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$FireInfoWrapper$drop(::std::unique_ptr<::std::vector<::FireInfoWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::IVirtualEntity>::value, "definition of IVirtualEntity is required");
static_assert(sizeof(::std::unique_ptr<::alt::IVirtualEntity>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::IVirtualEntity>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$IVirtualEntity$null(::std::unique_ptr<::alt::IVirtualEntity> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IVirtualEntity>();
}
void cxxbridge1$unique_ptr$alt$IVirtualEntity$raw(::std::unique_ptr<::alt::IVirtualEntity> *ptr, ::alt::IVirtualEntity *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IVirtualEntity>(raw);
}
::alt::IVirtualEntity const *cxxbridge1$unique_ptr$alt$IVirtualEntity$get(::std::unique_ptr<::alt::IVirtualEntity> const &ptr) noexcept {
  return ptr.get();
}
::alt::IVirtualEntity *cxxbridge1$unique_ptr$alt$IVirtualEntity$release(::std::unique_ptr<::alt::IVirtualEntity> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$IVirtualEntity$drop(::std::unique_ptr<::alt::IVirtualEntity> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::IVirtualEntity>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::IVirtualEntity>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::IVirtualEntity>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$IVirtualEntity$null(::std::shared_ptr<::alt::IVirtualEntity> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IVirtualEntity>();
}
void cxxbridge1$shared_ptr$alt$IVirtualEntity$clone(::std::shared_ptr<::alt::IVirtualEntity> const &self, ::std::shared_ptr<::alt::IVirtualEntity> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IVirtualEntity>(self);
}
::alt::IVirtualEntity const *cxxbridge1$shared_ptr$alt$IVirtualEntity$get(::std::shared_ptr<::alt::IVirtualEntity> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$IVirtualEntity$drop(::std::shared_ptr<::alt::IVirtualEntity> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::IVirtualEntity>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::IVirtualEntity>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$IVirtualEntity$null(::std::weak_ptr<::alt::IVirtualEntity> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IVirtualEntity>();
}
void cxxbridge1$weak_ptr$alt$IVirtualEntity$clone(::std::weak_ptr<::alt::IVirtualEntity> const &self, ::std::weak_ptr<::alt::IVirtualEntity> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IVirtualEntity>(self);
}
void cxxbridge1$weak_ptr$alt$IVirtualEntity$downgrade(::std::shared_ptr<::alt::IVirtualEntity> const &shared, ::std::weak_ptr<::alt::IVirtualEntity> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::IVirtualEntity>(shared);
}
void cxxbridge1$weak_ptr$alt$IVirtualEntity$upgrade(::std::weak_ptr<::alt::IVirtualEntity> const &weak, ::std::shared_ptr<::alt::IVirtualEntity> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::IVirtualEntity>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$IVirtualEntity$drop(::std::weak_ptr<::alt::IVirtualEntity> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::IVirtualEntityGroup>::value, "definition of IVirtualEntityGroup is required");
static_assert(sizeof(::std::unique_ptr<::alt::IVirtualEntityGroup>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::IVirtualEntityGroup>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$IVirtualEntityGroup$null(::std::unique_ptr<::alt::IVirtualEntityGroup> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IVirtualEntityGroup>();
}
void cxxbridge1$unique_ptr$alt$IVirtualEntityGroup$raw(::std::unique_ptr<::alt::IVirtualEntityGroup> *ptr, ::alt::IVirtualEntityGroup *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IVirtualEntityGroup>(raw);
}
::alt::IVirtualEntityGroup const *cxxbridge1$unique_ptr$alt$IVirtualEntityGroup$get(::std::unique_ptr<::alt::IVirtualEntityGroup> const &ptr) noexcept {
  return ptr.get();
}
::alt::IVirtualEntityGroup *cxxbridge1$unique_ptr$alt$IVirtualEntityGroup$release(::std::unique_ptr<::alt::IVirtualEntityGroup> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$IVirtualEntityGroup$drop(::std::unique_ptr<::alt::IVirtualEntityGroup> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::IVirtualEntityGroup>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::IVirtualEntityGroup>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::IVirtualEntityGroup>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$IVirtualEntityGroup$null(::std::shared_ptr<::alt::IVirtualEntityGroup> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IVirtualEntityGroup>();
}
void cxxbridge1$shared_ptr$alt$IVirtualEntityGroup$clone(::std::shared_ptr<::alt::IVirtualEntityGroup> const &self, ::std::shared_ptr<::alt::IVirtualEntityGroup> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IVirtualEntityGroup>(self);
}
::alt::IVirtualEntityGroup const *cxxbridge1$shared_ptr$alt$IVirtualEntityGroup$get(::std::shared_ptr<::alt::IVirtualEntityGroup> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$IVirtualEntityGroup$drop(::std::shared_ptr<::alt::IVirtualEntityGroup> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::IVirtualEntityGroup>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::IVirtualEntityGroup>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$IVirtualEntityGroup$null(::std::weak_ptr<::alt::IVirtualEntityGroup> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IVirtualEntityGroup>();
}
void cxxbridge1$weak_ptr$alt$IVirtualEntityGroup$clone(::std::weak_ptr<::alt::IVirtualEntityGroup> const &self, ::std::weak_ptr<::alt::IVirtualEntityGroup> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IVirtualEntityGroup>(self);
}
void cxxbridge1$weak_ptr$alt$IVirtualEntityGroup$downgrade(::std::shared_ptr<::alt::IVirtualEntityGroup> const &shared, ::std::weak_ptr<::alt::IVirtualEntityGroup> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::IVirtualEntityGroup>(shared);
}
void cxxbridge1$weak_ptr$alt$IVirtualEntityGroup$upgrade(::std::weak_ptr<::alt::IVirtualEntityGroup> const &weak, ::std::shared_ptr<::alt::IVirtualEntityGroup> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::IVirtualEntityGroup>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$IVirtualEntityGroup$drop(::std::weak_ptr<::alt::IVirtualEntityGroup> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::IColShape>::value, "definition of IColShape is required");
static_assert(sizeof(::std::unique_ptr<::alt::IColShape>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::IColShape>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$IColShape$null(::std::unique_ptr<::alt::IColShape> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IColShape>();
}
void cxxbridge1$unique_ptr$alt$IColShape$raw(::std::unique_ptr<::alt::IColShape> *ptr, ::alt::IColShape *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IColShape>(raw);
}
::alt::IColShape const *cxxbridge1$unique_ptr$alt$IColShape$get(::std::unique_ptr<::alt::IColShape> const &ptr) noexcept {
  return ptr.get();
}
::alt::IColShape *cxxbridge1$unique_ptr$alt$IColShape$release(::std::unique_ptr<::alt::IColShape> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$IColShape$drop(::std::unique_ptr<::alt::IColShape> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::IColShape>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::IColShape>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::IColShape>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$IColShape$null(::std::shared_ptr<::alt::IColShape> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IColShape>();
}
void cxxbridge1$shared_ptr$alt$IColShape$clone(::std::shared_ptr<::alt::IColShape> const &self, ::std::shared_ptr<::alt::IColShape> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IColShape>(self);
}
::alt::IColShape const *cxxbridge1$shared_ptr$alt$IColShape$get(::std::shared_ptr<::alt::IColShape> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$IColShape$drop(::std::shared_ptr<::alt::IColShape> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::IColShape>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::IColShape>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$IColShape$null(::std::weak_ptr<::alt::IColShape> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IColShape>();
}
void cxxbridge1$weak_ptr$alt$IColShape$clone(::std::weak_ptr<::alt::IColShape> const &self, ::std::weak_ptr<::alt::IColShape> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IColShape>(self);
}
void cxxbridge1$weak_ptr$alt$IColShape$downgrade(::std::shared_ptr<::alt::IColShape> const &shared, ::std::weak_ptr<::alt::IColShape> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::IColShape>(shared);
}
void cxxbridge1$weak_ptr$alt$IColShape$upgrade(::std::weak_ptr<::alt::IColShape> const &weak, ::std::shared_ptr<::alt::IColShape> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::IColShape>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$IColShape$drop(::std::weak_ptr<::alt::IColShape> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::IBlip>::value, "definition of IBlip is required");
static_assert(sizeof(::std::unique_ptr<::alt::IBlip>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::IBlip>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$IBlip$null(::std::unique_ptr<::alt::IBlip> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IBlip>();
}
void cxxbridge1$unique_ptr$alt$IBlip$raw(::std::unique_ptr<::alt::IBlip> *ptr, ::alt::IBlip *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IBlip>(raw);
}
::alt::IBlip const *cxxbridge1$unique_ptr$alt$IBlip$get(::std::unique_ptr<::alt::IBlip> const &ptr) noexcept {
  return ptr.get();
}
::alt::IBlip *cxxbridge1$unique_ptr$alt$IBlip$release(::std::unique_ptr<::alt::IBlip> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$IBlip$drop(::std::unique_ptr<::alt::IBlip> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::IBlip>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::IBlip>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::IBlip>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$IBlip$null(::std::shared_ptr<::alt::IBlip> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IBlip>();
}
void cxxbridge1$shared_ptr$alt$IBlip$clone(::std::shared_ptr<::alt::IBlip> const &self, ::std::shared_ptr<::alt::IBlip> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IBlip>(self);
}
::alt::IBlip const *cxxbridge1$shared_ptr$alt$IBlip$get(::std::shared_ptr<::alt::IBlip> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$IBlip$drop(::std::shared_ptr<::alt::IBlip> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::IBlip>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::IBlip>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$IBlip$null(::std::weak_ptr<::alt::IBlip> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IBlip>();
}
void cxxbridge1$weak_ptr$alt$IBlip$clone(::std::weak_ptr<::alt::IBlip> const &self, ::std::weak_ptr<::alt::IBlip> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IBlip>(self);
}
void cxxbridge1$weak_ptr$alt$IBlip$downgrade(::std::shared_ptr<::alt::IBlip> const &shared, ::std::weak_ptr<::alt::IBlip> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::IBlip>(shared);
}
void cxxbridge1$weak_ptr$alt$IBlip$upgrade(::std::weak_ptr<::alt::IBlip> const &weak, ::std::shared_ptr<::alt::IBlip> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::IBlip>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$IBlip$drop(::std::weak_ptr<::alt::IBlip> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::IVoiceChannel>::value, "definition of IVoiceChannel is required");
static_assert(sizeof(::std::unique_ptr<::alt::IVoiceChannel>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::IVoiceChannel>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$IVoiceChannel$null(::std::unique_ptr<::alt::IVoiceChannel> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IVoiceChannel>();
}
void cxxbridge1$unique_ptr$alt$IVoiceChannel$raw(::std::unique_ptr<::alt::IVoiceChannel> *ptr, ::alt::IVoiceChannel *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IVoiceChannel>(raw);
}
::alt::IVoiceChannel const *cxxbridge1$unique_ptr$alt$IVoiceChannel$get(::std::unique_ptr<::alt::IVoiceChannel> const &ptr) noexcept {
  return ptr.get();
}
::alt::IVoiceChannel *cxxbridge1$unique_ptr$alt$IVoiceChannel$release(::std::unique_ptr<::alt::IVoiceChannel> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$IVoiceChannel$drop(::std::unique_ptr<::alt::IVoiceChannel> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::IVoiceChannel>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::IVoiceChannel>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::IVoiceChannel>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$IVoiceChannel$null(::std::shared_ptr<::alt::IVoiceChannel> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IVoiceChannel>();
}
void cxxbridge1$shared_ptr$alt$IVoiceChannel$clone(::std::shared_ptr<::alt::IVoiceChannel> const &self, ::std::shared_ptr<::alt::IVoiceChannel> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IVoiceChannel>(self);
}
::alt::IVoiceChannel const *cxxbridge1$shared_ptr$alt$IVoiceChannel$get(::std::shared_ptr<::alt::IVoiceChannel> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$IVoiceChannel$drop(::std::shared_ptr<::alt::IVoiceChannel> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::IVoiceChannel>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::IVoiceChannel>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$IVoiceChannel$null(::std::weak_ptr<::alt::IVoiceChannel> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IVoiceChannel>();
}
void cxxbridge1$weak_ptr$alt$IVoiceChannel$clone(::std::weak_ptr<::alt::IVoiceChannel> const &self, ::std::weak_ptr<::alt::IVoiceChannel> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IVoiceChannel>(self);
}
void cxxbridge1$weak_ptr$alt$IVoiceChannel$downgrade(::std::shared_ptr<::alt::IVoiceChannel> const &shared, ::std::weak_ptr<::alt::IVoiceChannel> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::IVoiceChannel>(shared);
}
void cxxbridge1$weak_ptr$alt$IVoiceChannel$upgrade(::std::weak_ptr<::alt::IVoiceChannel> const &weak, ::std::shared_ptr<::alt::IVoiceChannel> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::IVoiceChannel>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$IVoiceChannel$drop(::std::weak_ptr<::alt::IVoiceChannel> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::IMarker>::value, "definition of IMarker is required");
static_assert(sizeof(::std::unique_ptr<::alt::IMarker>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::IMarker>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$IMarker$null(::std::unique_ptr<::alt::IMarker> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IMarker>();
}
void cxxbridge1$unique_ptr$alt$IMarker$raw(::std::unique_ptr<::alt::IMarker> *ptr, ::alt::IMarker *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IMarker>(raw);
}
::alt::IMarker const *cxxbridge1$unique_ptr$alt$IMarker$get(::std::unique_ptr<::alt::IMarker> const &ptr) noexcept {
  return ptr.get();
}
::alt::IMarker *cxxbridge1$unique_ptr$alt$IMarker$release(::std::unique_ptr<::alt::IMarker> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$IMarker$drop(::std::unique_ptr<::alt::IMarker> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::IMarker>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::IMarker>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::IMarker>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$IMarker$null(::std::shared_ptr<::alt::IMarker> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IMarker>();
}
void cxxbridge1$shared_ptr$alt$IMarker$clone(::std::shared_ptr<::alt::IMarker> const &self, ::std::shared_ptr<::alt::IMarker> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IMarker>(self);
}
::alt::IMarker const *cxxbridge1$shared_ptr$alt$IMarker$get(::std::shared_ptr<::alt::IMarker> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$IMarker$drop(::std::shared_ptr<::alt::IMarker> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::IMarker>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::IMarker>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$IMarker$null(::std::weak_ptr<::alt::IMarker> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IMarker>();
}
void cxxbridge1$weak_ptr$alt$IMarker$clone(::std::weak_ptr<::alt::IMarker> const &self, ::std::weak_ptr<::alt::IMarker> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IMarker>(self);
}
void cxxbridge1$weak_ptr$alt$IMarker$downgrade(::std::shared_ptr<::alt::IMarker> const &shared, ::std::weak_ptr<::alt::IMarker> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::IMarker>(shared);
}
void cxxbridge1$weak_ptr$alt$IMarker$upgrade(::std::weak_ptr<::alt::IMarker> const &weak, ::std::shared_ptr<::alt::IMarker> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::IMarker>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$IMarker$drop(::std::weak_ptr<::alt::IMarker> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::ICheckpoint>::value, "definition of ICheckpoint is required");
static_assert(sizeof(::std::unique_ptr<::alt::ICheckpoint>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::ICheckpoint>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$ICheckpoint$null(::std::unique_ptr<::alt::ICheckpoint> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::ICheckpoint>();
}
void cxxbridge1$unique_ptr$alt$ICheckpoint$raw(::std::unique_ptr<::alt::ICheckpoint> *ptr, ::alt::ICheckpoint *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::ICheckpoint>(raw);
}
::alt::ICheckpoint const *cxxbridge1$unique_ptr$alt$ICheckpoint$get(::std::unique_ptr<::alt::ICheckpoint> const &ptr) noexcept {
  return ptr.get();
}
::alt::ICheckpoint *cxxbridge1$unique_ptr$alt$ICheckpoint$release(::std::unique_ptr<::alt::ICheckpoint> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$ICheckpoint$drop(::std::unique_ptr<::alt::ICheckpoint> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::ICheckpoint>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::ICheckpoint>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::ICheckpoint>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$ICheckpoint$null(::std::shared_ptr<::alt::ICheckpoint> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::ICheckpoint>();
}
void cxxbridge1$shared_ptr$alt$ICheckpoint$clone(::std::shared_ptr<::alt::ICheckpoint> const &self, ::std::shared_ptr<::alt::ICheckpoint> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::ICheckpoint>(self);
}
::alt::ICheckpoint const *cxxbridge1$shared_ptr$alt$ICheckpoint$get(::std::shared_ptr<::alt::ICheckpoint> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$ICheckpoint$drop(::std::shared_ptr<::alt::ICheckpoint> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::ICheckpoint>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::ICheckpoint>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$ICheckpoint$null(::std::weak_ptr<::alt::ICheckpoint> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::ICheckpoint>();
}
void cxxbridge1$weak_ptr$alt$ICheckpoint$clone(::std::weak_ptr<::alt::ICheckpoint> const &self, ::std::weak_ptr<::alt::ICheckpoint> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::ICheckpoint>(self);
}
void cxxbridge1$weak_ptr$alt$ICheckpoint$downgrade(::std::shared_ptr<::alt::ICheckpoint> const &shared, ::std::weak_ptr<::alt::ICheckpoint> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::ICheckpoint>(shared);
}
void cxxbridge1$weak_ptr$alt$ICheckpoint$upgrade(::std::weak_ptr<::alt::ICheckpoint> const &weak, ::std::shared_ptr<::alt::ICheckpoint> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::ICheckpoint>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$ICheckpoint$drop(::std::weak_ptr<::alt::ICheckpoint> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CEvent>::value, "definition of CEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CEvent$null(::std::unique_ptr<::alt::CEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CEvent>();
}
::alt::CEvent *cxxbridge1$unique_ptr$alt$CEvent$uninit(::std::unique_ptr<::alt::CEvent> *ptr) noexcept {
  ::alt::CEvent *uninit = reinterpret_cast<::alt::CEvent *>(new ::rust::MaybeUninit<::alt::CEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CEvent$raw(::std::unique_ptr<::alt::CEvent> *ptr, ::alt::CEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CEvent>(raw);
}
::alt::CEvent const *cxxbridge1$unique_ptr$alt$CEvent$get(::std::unique_ptr<::alt::CEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CEvent *cxxbridge1$unique_ptr$alt$CEvent$release(::std::unique_ptr<::alt::CEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CEvent$drop(::std::unique_ptr<::alt::CEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CEvent$null(::std::shared_ptr<::alt::CEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CEvent>();
}
::alt::CEvent *cxxbridge1$shared_ptr$alt$CEvent$uninit(::std::shared_ptr<::alt::CEvent> *ptr) noexcept {
  ::alt::CEvent *uninit = reinterpret_cast<::alt::CEvent *>(new ::rust::MaybeUninit<::alt::CEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CEvent$clone(::std::shared_ptr<::alt::CEvent> const &self, ::std::shared_ptr<::alt::CEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CEvent>(self);
}
::alt::CEvent const *cxxbridge1$shared_ptr$alt$CEvent$get(::std::shared_ptr<::alt::CEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CEvent$drop(::std::shared_ptr<::alt::CEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CEvent$null(::std::weak_ptr<::alt::CEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CEvent>();
}
void cxxbridge1$weak_ptr$alt$CEvent$clone(::std::weak_ptr<::alt::CEvent> const &self, ::std::weak_ptr<::alt::CEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CEvent$downgrade(::std::shared_ptr<::alt::CEvent> const &shared, ::std::weak_ptr<::alt::CEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CEvent$upgrade(::std::weak_ptr<::alt::CEvent> const &weak, ::std::shared_ptr<::alt::CEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CEvent$drop(::std::weak_ptr<::alt::CEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CCancellableEvent>::value, "definition of CCancellableEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CCancellableEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CCancellableEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CCancellableEvent$null(::std::unique_ptr<::alt::CCancellableEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CCancellableEvent>();
}
::alt::CCancellableEvent *cxxbridge1$unique_ptr$alt$CCancellableEvent$uninit(::std::unique_ptr<::alt::CCancellableEvent> *ptr) noexcept {
  ::alt::CCancellableEvent *uninit = reinterpret_cast<::alt::CCancellableEvent *>(new ::rust::MaybeUninit<::alt::CCancellableEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CCancellableEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CCancellableEvent$raw(::std::unique_ptr<::alt::CCancellableEvent> *ptr, ::alt::CCancellableEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CCancellableEvent>(raw);
}
::alt::CCancellableEvent const *cxxbridge1$unique_ptr$alt$CCancellableEvent$get(::std::unique_ptr<::alt::CCancellableEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CCancellableEvent *cxxbridge1$unique_ptr$alt$CCancellableEvent$release(::std::unique_ptr<::alt::CCancellableEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CCancellableEvent$drop(::std::unique_ptr<::alt::CCancellableEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CCancellableEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CCancellableEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CCancellableEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CCancellableEvent$null(::std::shared_ptr<::alt::CCancellableEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CCancellableEvent>();
}
::alt::CCancellableEvent *cxxbridge1$shared_ptr$alt$CCancellableEvent$uninit(::std::shared_ptr<::alt::CCancellableEvent> *ptr) noexcept {
  ::alt::CCancellableEvent *uninit = reinterpret_cast<::alt::CCancellableEvent *>(new ::rust::MaybeUninit<::alt::CCancellableEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CCancellableEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CCancellableEvent$clone(::std::shared_ptr<::alt::CCancellableEvent> const &self, ::std::shared_ptr<::alt::CCancellableEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CCancellableEvent>(self);
}
::alt::CCancellableEvent const *cxxbridge1$shared_ptr$alt$CCancellableEvent$get(::std::shared_ptr<::alt::CCancellableEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CCancellableEvent$drop(::std::shared_ptr<::alt::CCancellableEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CCancellableEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CCancellableEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CCancellableEvent$null(::std::weak_ptr<::alt::CCancellableEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CCancellableEvent>();
}
void cxxbridge1$weak_ptr$alt$CCancellableEvent$clone(::std::weak_ptr<::alt::CCancellableEvent> const &self, ::std::weak_ptr<::alt::CCancellableEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CCancellableEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CCancellableEvent$downgrade(::std::shared_ptr<::alt::CCancellableEvent> const &shared, ::std::weak_ptr<::alt::CCancellableEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CCancellableEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CCancellableEvent$upgrade(::std::weak_ptr<::alt::CCancellableEvent> const &weak, ::std::shared_ptr<::alt::CCancellableEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CCancellableEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CCancellableEvent$drop(::std::weak_ptr<::alt::CCancellableEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CConsoleCommandEvent>::value, "definition of CConsoleCommandEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CConsoleCommandEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CConsoleCommandEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CConsoleCommandEvent$null(::std::unique_ptr<::alt::CConsoleCommandEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CConsoleCommandEvent>();
}
::alt::CConsoleCommandEvent *cxxbridge1$unique_ptr$alt$CConsoleCommandEvent$uninit(::std::unique_ptr<::alt::CConsoleCommandEvent> *ptr) noexcept {
  ::alt::CConsoleCommandEvent *uninit = reinterpret_cast<::alt::CConsoleCommandEvent *>(new ::rust::MaybeUninit<::alt::CConsoleCommandEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CConsoleCommandEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CConsoleCommandEvent$raw(::std::unique_ptr<::alt::CConsoleCommandEvent> *ptr, ::alt::CConsoleCommandEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CConsoleCommandEvent>(raw);
}
::alt::CConsoleCommandEvent const *cxxbridge1$unique_ptr$alt$CConsoleCommandEvent$get(::std::unique_ptr<::alt::CConsoleCommandEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CConsoleCommandEvent *cxxbridge1$unique_ptr$alt$CConsoleCommandEvent$release(::std::unique_ptr<::alt::CConsoleCommandEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CConsoleCommandEvent$drop(::std::unique_ptr<::alt::CConsoleCommandEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CConsoleCommandEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CConsoleCommandEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CConsoleCommandEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CConsoleCommandEvent$null(::std::shared_ptr<::alt::CConsoleCommandEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CConsoleCommandEvent>();
}
::alt::CConsoleCommandEvent *cxxbridge1$shared_ptr$alt$CConsoleCommandEvent$uninit(::std::shared_ptr<::alt::CConsoleCommandEvent> *ptr) noexcept {
  ::alt::CConsoleCommandEvent *uninit = reinterpret_cast<::alt::CConsoleCommandEvent *>(new ::rust::MaybeUninit<::alt::CConsoleCommandEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CConsoleCommandEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CConsoleCommandEvent$clone(::std::shared_ptr<::alt::CConsoleCommandEvent> const &self, ::std::shared_ptr<::alt::CConsoleCommandEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CConsoleCommandEvent>(self);
}
::alt::CConsoleCommandEvent const *cxxbridge1$shared_ptr$alt$CConsoleCommandEvent$get(::std::shared_ptr<::alt::CConsoleCommandEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CConsoleCommandEvent$drop(::std::shared_ptr<::alt::CConsoleCommandEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CConsoleCommandEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CConsoleCommandEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CConsoleCommandEvent$null(::std::weak_ptr<::alt::CConsoleCommandEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CConsoleCommandEvent>();
}
void cxxbridge1$weak_ptr$alt$CConsoleCommandEvent$clone(::std::weak_ptr<::alt::CConsoleCommandEvent> const &self, ::std::weak_ptr<::alt::CConsoleCommandEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CConsoleCommandEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CConsoleCommandEvent$downgrade(::std::shared_ptr<::alt::CConsoleCommandEvent> const &shared, ::std::weak_ptr<::alt::CConsoleCommandEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CConsoleCommandEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CConsoleCommandEvent$upgrade(::std::weak_ptr<::alt::CConsoleCommandEvent> const &weak, ::std::shared_ptr<::alt::CConsoleCommandEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CConsoleCommandEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CConsoleCommandEvent$drop(::std::weak_ptr<::alt::CConsoleCommandEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CServerScriptEvent>::value, "definition of CServerScriptEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CServerScriptEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CServerScriptEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CServerScriptEvent$null(::std::unique_ptr<::alt::CServerScriptEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CServerScriptEvent>();
}
::alt::CServerScriptEvent *cxxbridge1$unique_ptr$alt$CServerScriptEvent$uninit(::std::unique_ptr<::alt::CServerScriptEvent> *ptr) noexcept {
  ::alt::CServerScriptEvent *uninit = reinterpret_cast<::alt::CServerScriptEvent *>(new ::rust::MaybeUninit<::alt::CServerScriptEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CServerScriptEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CServerScriptEvent$raw(::std::unique_ptr<::alt::CServerScriptEvent> *ptr, ::alt::CServerScriptEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CServerScriptEvent>(raw);
}
::alt::CServerScriptEvent const *cxxbridge1$unique_ptr$alt$CServerScriptEvent$get(::std::unique_ptr<::alt::CServerScriptEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CServerScriptEvent *cxxbridge1$unique_ptr$alt$CServerScriptEvent$release(::std::unique_ptr<::alt::CServerScriptEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CServerScriptEvent$drop(::std::unique_ptr<::alt::CServerScriptEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CServerScriptEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CServerScriptEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CServerScriptEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CServerScriptEvent$null(::std::shared_ptr<::alt::CServerScriptEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CServerScriptEvent>();
}
::alt::CServerScriptEvent *cxxbridge1$shared_ptr$alt$CServerScriptEvent$uninit(::std::shared_ptr<::alt::CServerScriptEvent> *ptr) noexcept {
  ::alt::CServerScriptEvent *uninit = reinterpret_cast<::alt::CServerScriptEvent *>(new ::rust::MaybeUninit<::alt::CServerScriptEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CServerScriptEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CServerScriptEvent$clone(::std::shared_ptr<::alt::CServerScriptEvent> const &self, ::std::shared_ptr<::alt::CServerScriptEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CServerScriptEvent>(self);
}
::alt::CServerScriptEvent const *cxxbridge1$shared_ptr$alt$CServerScriptEvent$get(::std::shared_ptr<::alt::CServerScriptEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CServerScriptEvent$drop(::std::shared_ptr<::alt::CServerScriptEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CServerScriptEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CServerScriptEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CServerScriptEvent$null(::std::weak_ptr<::alt::CServerScriptEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CServerScriptEvent>();
}
void cxxbridge1$weak_ptr$alt$CServerScriptEvent$clone(::std::weak_ptr<::alt::CServerScriptEvent> const &self, ::std::weak_ptr<::alt::CServerScriptEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CServerScriptEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CServerScriptEvent$downgrade(::std::shared_ptr<::alt::CServerScriptEvent> const &shared, ::std::weak_ptr<::alt::CServerScriptEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CServerScriptEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CServerScriptEvent$upgrade(::std::weak_ptr<::alt::CServerScriptEvent> const &weak, ::std::shared_ptr<::alt::CServerScriptEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CServerScriptEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CServerScriptEvent$drop(::std::weak_ptr<::alt::CServerScriptEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CClientScriptEvent>::value, "definition of CClientScriptEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CClientScriptEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CClientScriptEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CClientScriptEvent$null(::std::unique_ptr<::alt::CClientScriptEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CClientScriptEvent>();
}
::alt::CClientScriptEvent *cxxbridge1$unique_ptr$alt$CClientScriptEvent$uninit(::std::unique_ptr<::alt::CClientScriptEvent> *ptr) noexcept {
  ::alt::CClientScriptEvent *uninit = reinterpret_cast<::alt::CClientScriptEvent *>(new ::rust::MaybeUninit<::alt::CClientScriptEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CClientScriptEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CClientScriptEvent$raw(::std::unique_ptr<::alt::CClientScriptEvent> *ptr, ::alt::CClientScriptEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CClientScriptEvent>(raw);
}
::alt::CClientScriptEvent const *cxxbridge1$unique_ptr$alt$CClientScriptEvent$get(::std::unique_ptr<::alt::CClientScriptEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CClientScriptEvent *cxxbridge1$unique_ptr$alt$CClientScriptEvent$release(::std::unique_ptr<::alt::CClientScriptEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CClientScriptEvent$drop(::std::unique_ptr<::alt::CClientScriptEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CClientScriptEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CClientScriptEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CClientScriptEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CClientScriptEvent$null(::std::shared_ptr<::alt::CClientScriptEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CClientScriptEvent>();
}
::alt::CClientScriptEvent *cxxbridge1$shared_ptr$alt$CClientScriptEvent$uninit(::std::shared_ptr<::alt::CClientScriptEvent> *ptr) noexcept {
  ::alt::CClientScriptEvent *uninit = reinterpret_cast<::alt::CClientScriptEvent *>(new ::rust::MaybeUninit<::alt::CClientScriptEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CClientScriptEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CClientScriptEvent$clone(::std::shared_ptr<::alt::CClientScriptEvent> const &self, ::std::shared_ptr<::alt::CClientScriptEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CClientScriptEvent>(self);
}
::alt::CClientScriptEvent const *cxxbridge1$shared_ptr$alt$CClientScriptEvent$get(::std::shared_ptr<::alt::CClientScriptEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CClientScriptEvent$drop(::std::shared_ptr<::alt::CClientScriptEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CClientScriptEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CClientScriptEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CClientScriptEvent$null(::std::weak_ptr<::alt::CClientScriptEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CClientScriptEvent>();
}
void cxxbridge1$weak_ptr$alt$CClientScriptEvent$clone(::std::weak_ptr<::alt::CClientScriptEvent> const &self, ::std::weak_ptr<::alt::CClientScriptEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CClientScriptEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CClientScriptEvent$downgrade(::std::shared_ptr<::alt::CClientScriptEvent> const &shared, ::std::weak_ptr<::alt::CClientScriptEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CClientScriptEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CClientScriptEvent$upgrade(::std::weak_ptr<::alt::CClientScriptEvent> const &weak, ::std::shared_ptr<::alt::CClientScriptEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CClientScriptEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CClientScriptEvent$drop(::std::weak_ptr<::alt::CClientScriptEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerDisconnectEvent>::value, "definition of CPlayerDisconnectEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerDisconnectEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerDisconnectEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerDisconnectEvent$null(::std::unique_ptr<::alt::CPlayerDisconnectEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDisconnectEvent>();
}
::alt::CPlayerDisconnectEvent *cxxbridge1$unique_ptr$alt$CPlayerDisconnectEvent$uninit(::std::unique_ptr<::alt::CPlayerDisconnectEvent> *ptr) noexcept {
  ::alt::CPlayerDisconnectEvent *uninit = reinterpret_cast<::alt::CPlayerDisconnectEvent *>(new ::rust::MaybeUninit<::alt::CPlayerDisconnectEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDisconnectEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerDisconnectEvent$raw(::std::unique_ptr<::alt::CPlayerDisconnectEvent> *ptr, ::alt::CPlayerDisconnectEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDisconnectEvent>(raw);
}
::alt::CPlayerDisconnectEvent const *cxxbridge1$unique_ptr$alt$CPlayerDisconnectEvent$get(::std::unique_ptr<::alt::CPlayerDisconnectEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerDisconnectEvent *cxxbridge1$unique_ptr$alt$CPlayerDisconnectEvent$release(::std::unique_ptr<::alt::CPlayerDisconnectEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerDisconnectEvent$drop(::std::unique_ptr<::alt::CPlayerDisconnectEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerDisconnectEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerDisconnectEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerDisconnectEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerDisconnectEvent$null(::std::shared_ptr<::alt::CPlayerDisconnectEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDisconnectEvent>();
}
::alt::CPlayerDisconnectEvent *cxxbridge1$shared_ptr$alt$CPlayerDisconnectEvent$uninit(::std::shared_ptr<::alt::CPlayerDisconnectEvent> *ptr) noexcept {
  ::alt::CPlayerDisconnectEvent *uninit = reinterpret_cast<::alt::CPlayerDisconnectEvent *>(new ::rust::MaybeUninit<::alt::CPlayerDisconnectEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDisconnectEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerDisconnectEvent$clone(::std::shared_ptr<::alt::CPlayerDisconnectEvent> const &self, ::std::shared_ptr<::alt::CPlayerDisconnectEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDisconnectEvent>(self);
}
::alt::CPlayerDisconnectEvent const *cxxbridge1$shared_ptr$alt$CPlayerDisconnectEvent$get(::std::shared_ptr<::alt::CPlayerDisconnectEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerDisconnectEvent$drop(::std::shared_ptr<::alt::CPlayerDisconnectEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerDisconnectEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerDisconnectEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerDisconnectEvent$null(::std::weak_ptr<::alt::CPlayerDisconnectEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerDisconnectEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerDisconnectEvent$clone(::std::weak_ptr<::alt::CPlayerDisconnectEvent> const &self, ::std::weak_ptr<::alt::CPlayerDisconnectEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerDisconnectEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerDisconnectEvent$downgrade(::std::shared_ptr<::alt::CPlayerDisconnectEvent> const &shared, ::std::weak_ptr<::alt::CPlayerDisconnectEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerDisconnectEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerDisconnectEvent$upgrade(::std::weak_ptr<::alt::CPlayerDisconnectEvent> const &weak, ::std::shared_ptr<::alt::CPlayerDisconnectEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerDisconnectEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerDisconnectEvent$drop(::std::weak_ptr<::alt::CPlayerDisconnectEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerConnectEvent>::value, "definition of CPlayerConnectEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerConnectEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerConnectEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerConnectEvent$null(::std::unique_ptr<::alt::CPlayerConnectEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerConnectEvent>();
}
::alt::CPlayerConnectEvent *cxxbridge1$unique_ptr$alt$CPlayerConnectEvent$uninit(::std::unique_ptr<::alt::CPlayerConnectEvent> *ptr) noexcept {
  ::alt::CPlayerConnectEvent *uninit = reinterpret_cast<::alt::CPlayerConnectEvent *>(new ::rust::MaybeUninit<::alt::CPlayerConnectEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerConnectEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerConnectEvent$raw(::std::unique_ptr<::alt::CPlayerConnectEvent> *ptr, ::alt::CPlayerConnectEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerConnectEvent>(raw);
}
::alt::CPlayerConnectEvent const *cxxbridge1$unique_ptr$alt$CPlayerConnectEvent$get(::std::unique_ptr<::alt::CPlayerConnectEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerConnectEvent *cxxbridge1$unique_ptr$alt$CPlayerConnectEvent$release(::std::unique_ptr<::alt::CPlayerConnectEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerConnectEvent$drop(::std::unique_ptr<::alt::CPlayerConnectEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerConnectEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerConnectEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerConnectEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerConnectEvent$null(::std::shared_ptr<::alt::CPlayerConnectEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerConnectEvent>();
}
::alt::CPlayerConnectEvent *cxxbridge1$shared_ptr$alt$CPlayerConnectEvent$uninit(::std::shared_ptr<::alt::CPlayerConnectEvent> *ptr) noexcept {
  ::alt::CPlayerConnectEvent *uninit = reinterpret_cast<::alt::CPlayerConnectEvent *>(new ::rust::MaybeUninit<::alt::CPlayerConnectEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerConnectEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerConnectEvent$clone(::std::shared_ptr<::alt::CPlayerConnectEvent> const &self, ::std::shared_ptr<::alt::CPlayerConnectEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerConnectEvent>(self);
}
::alt::CPlayerConnectEvent const *cxxbridge1$shared_ptr$alt$CPlayerConnectEvent$get(::std::shared_ptr<::alt::CPlayerConnectEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerConnectEvent$drop(::std::shared_ptr<::alt::CPlayerConnectEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerConnectEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerConnectEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerConnectEvent$null(::std::weak_ptr<::alt::CPlayerConnectEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerConnectEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerConnectEvent$clone(::std::weak_ptr<::alt::CPlayerConnectEvent> const &self, ::std::weak_ptr<::alt::CPlayerConnectEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerConnectEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerConnectEvent$downgrade(::std::shared_ptr<::alt::CPlayerConnectEvent> const &shared, ::std::weak_ptr<::alt::CPlayerConnectEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerConnectEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerConnectEvent$upgrade(::std::weak_ptr<::alt::CPlayerConnectEvent> const &weak, ::std::shared_ptr<::alt::CPlayerConnectEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerConnectEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerConnectEvent$drop(::std::weak_ptr<::alt::CPlayerConnectEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CColShapeEvent>::value, "definition of CColShapeEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CColShapeEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CColShapeEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CColShapeEvent$null(::std::unique_ptr<::alt::CColShapeEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CColShapeEvent>();
}
::alt::CColShapeEvent *cxxbridge1$unique_ptr$alt$CColShapeEvent$uninit(::std::unique_ptr<::alt::CColShapeEvent> *ptr) noexcept {
  ::alt::CColShapeEvent *uninit = reinterpret_cast<::alt::CColShapeEvent *>(new ::rust::MaybeUninit<::alt::CColShapeEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CColShapeEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CColShapeEvent$raw(::std::unique_ptr<::alt::CColShapeEvent> *ptr, ::alt::CColShapeEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CColShapeEvent>(raw);
}
::alt::CColShapeEvent const *cxxbridge1$unique_ptr$alt$CColShapeEvent$get(::std::unique_ptr<::alt::CColShapeEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CColShapeEvent *cxxbridge1$unique_ptr$alt$CColShapeEvent$release(::std::unique_ptr<::alt::CColShapeEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CColShapeEvent$drop(::std::unique_ptr<::alt::CColShapeEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CColShapeEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CColShapeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CColShapeEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CColShapeEvent$null(::std::shared_ptr<::alt::CColShapeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CColShapeEvent>();
}
::alt::CColShapeEvent *cxxbridge1$shared_ptr$alt$CColShapeEvent$uninit(::std::shared_ptr<::alt::CColShapeEvent> *ptr) noexcept {
  ::alt::CColShapeEvent *uninit = reinterpret_cast<::alt::CColShapeEvent *>(new ::rust::MaybeUninit<::alt::CColShapeEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CColShapeEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CColShapeEvent$clone(::std::shared_ptr<::alt::CColShapeEvent> const &self, ::std::shared_ptr<::alt::CColShapeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CColShapeEvent>(self);
}
::alt::CColShapeEvent const *cxxbridge1$shared_ptr$alt$CColShapeEvent$get(::std::shared_ptr<::alt::CColShapeEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CColShapeEvent$drop(::std::shared_ptr<::alt::CColShapeEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CColShapeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CColShapeEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CColShapeEvent$null(::std::weak_ptr<::alt::CColShapeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CColShapeEvent>();
}
void cxxbridge1$weak_ptr$alt$CColShapeEvent$clone(::std::weak_ptr<::alt::CColShapeEvent> const &self, ::std::weak_ptr<::alt::CColShapeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CColShapeEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CColShapeEvent$downgrade(::std::shared_ptr<::alt::CColShapeEvent> const &shared, ::std::weak_ptr<::alt::CColShapeEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CColShapeEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CColShapeEvent$upgrade(::std::weak_ptr<::alt::CColShapeEvent> const &weak, ::std::shared_ptr<::alt::CColShapeEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CColShapeEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CColShapeEvent$drop(::std::weak_ptr<::alt::CColShapeEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CWeaponDamageEvent>::value, "definition of CWeaponDamageEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CWeaponDamageEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CWeaponDamageEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CWeaponDamageEvent$null(::std::unique_ptr<::alt::CWeaponDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CWeaponDamageEvent>();
}
::alt::CWeaponDamageEvent *cxxbridge1$unique_ptr$alt$CWeaponDamageEvent$uninit(::std::unique_ptr<::alt::CWeaponDamageEvent> *ptr) noexcept {
  ::alt::CWeaponDamageEvent *uninit = reinterpret_cast<::alt::CWeaponDamageEvent *>(new ::rust::MaybeUninit<::alt::CWeaponDamageEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CWeaponDamageEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CWeaponDamageEvent$raw(::std::unique_ptr<::alt::CWeaponDamageEvent> *ptr, ::alt::CWeaponDamageEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CWeaponDamageEvent>(raw);
}
::alt::CWeaponDamageEvent const *cxxbridge1$unique_ptr$alt$CWeaponDamageEvent$get(::std::unique_ptr<::alt::CWeaponDamageEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CWeaponDamageEvent *cxxbridge1$unique_ptr$alt$CWeaponDamageEvent$release(::std::unique_ptr<::alt::CWeaponDamageEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CWeaponDamageEvent$drop(::std::unique_ptr<::alt::CWeaponDamageEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CWeaponDamageEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CWeaponDamageEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CWeaponDamageEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CWeaponDamageEvent$null(::std::shared_ptr<::alt::CWeaponDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CWeaponDamageEvent>();
}
::alt::CWeaponDamageEvent *cxxbridge1$shared_ptr$alt$CWeaponDamageEvent$uninit(::std::shared_ptr<::alt::CWeaponDamageEvent> *ptr) noexcept {
  ::alt::CWeaponDamageEvent *uninit = reinterpret_cast<::alt::CWeaponDamageEvent *>(new ::rust::MaybeUninit<::alt::CWeaponDamageEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CWeaponDamageEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CWeaponDamageEvent$clone(::std::shared_ptr<::alt::CWeaponDamageEvent> const &self, ::std::shared_ptr<::alt::CWeaponDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CWeaponDamageEvent>(self);
}
::alt::CWeaponDamageEvent const *cxxbridge1$shared_ptr$alt$CWeaponDamageEvent$get(::std::shared_ptr<::alt::CWeaponDamageEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CWeaponDamageEvent$drop(::std::shared_ptr<::alt::CWeaponDamageEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CWeaponDamageEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CWeaponDamageEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CWeaponDamageEvent$null(::std::weak_ptr<::alt::CWeaponDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CWeaponDamageEvent>();
}
void cxxbridge1$weak_ptr$alt$CWeaponDamageEvent$clone(::std::weak_ptr<::alt::CWeaponDamageEvent> const &self, ::std::weak_ptr<::alt::CWeaponDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CWeaponDamageEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CWeaponDamageEvent$downgrade(::std::shared_ptr<::alt::CWeaponDamageEvent> const &shared, ::std::weak_ptr<::alt::CWeaponDamageEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CWeaponDamageEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CWeaponDamageEvent$upgrade(::std::weak_ptr<::alt::CWeaponDamageEvent> const &weak, ::std::shared_ptr<::alt::CWeaponDamageEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CWeaponDamageEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CWeaponDamageEvent$drop(::std::weak_ptr<::alt::CWeaponDamageEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerDeathEvent>::value, "definition of CPlayerDeathEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerDeathEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerDeathEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerDeathEvent$null(::std::unique_ptr<::alt::CPlayerDeathEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDeathEvent>();
}
::alt::CPlayerDeathEvent *cxxbridge1$unique_ptr$alt$CPlayerDeathEvent$uninit(::std::unique_ptr<::alt::CPlayerDeathEvent> *ptr) noexcept {
  ::alt::CPlayerDeathEvent *uninit = reinterpret_cast<::alt::CPlayerDeathEvent *>(new ::rust::MaybeUninit<::alt::CPlayerDeathEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDeathEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerDeathEvent$raw(::std::unique_ptr<::alt::CPlayerDeathEvent> *ptr, ::alt::CPlayerDeathEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDeathEvent>(raw);
}
::alt::CPlayerDeathEvent const *cxxbridge1$unique_ptr$alt$CPlayerDeathEvent$get(::std::unique_ptr<::alt::CPlayerDeathEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerDeathEvent *cxxbridge1$unique_ptr$alt$CPlayerDeathEvent$release(::std::unique_ptr<::alt::CPlayerDeathEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerDeathEvent$drop(::std::unique_ptr<::alt::CPlayerDeathEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerDeathEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerDeathEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerDeathEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerDeathEvent$null(::std::shared_ptr<::alt::CPlayerDeathEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDeathEvent>();
}
::alt::CPlayerDeathEvent *cxxbridge1$shared_ptr$alt$CPlayerDeathEvent$uninit(::std::shared_ptr<::alt::CPlayerDeathEvent> *ptr) noexcept {
  ::alt::CPlayerDeathEvent *uninit = reinterpret_cast<::alt::CPlayerDeathEvent *>(new ::rust::MaybeUninit<::alt::CPlayerDeathEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDeathEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerDeathEvent$clone(::std::shared_ptr<::alt::CPlayerDeathEvent> const &self, ::std::shared_ptr<::alt::CPlayerDeathEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDeathEvent>(self);
}
::alt::CPlayerDeathEvent const *cxxbridge1$shared_ptr$alt$CPlayerDeathEvent$get(::std::shared_ptr<::alt::CPlayerDeathEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerDeathEvent$drop(::std::shared_ptr<::alt::CPlayerDeathEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerDeathEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerDeathEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerDeathEvent$null(::std::weak_ptr<::alt::CPlayerDeathEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerDeathEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerDeathEvent$clone(::std::weak_ptr<::alt::CPlayerDeathEvent> const &self, ::std::weak_ptr<::alt::CPlayerDeathEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerDeathEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerDeathEvent$downgrade(::std::shared_ptr<::alt::CPlayerDeathEvent> const &shared, ::std::weak_ptr<::alt::CPlayerDeathEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerDeathEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerDeathEvent$upgrade(::std::weak_ptr<::alt::CPlayerDeathEvent> const &weak, ::std::shared_ptr<::alt::CPlayerDeathEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerDeathEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerDeathEvent$drop(::std::weak_ptr<::alt::CPlayerDeathEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerDamageEvent>::value, "definition of CPlayerDamageEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerDamageEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerDamageEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerDamageEvent$null(::std::unique_ptr<::alt::CPlayerDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDamageEvent>();
}
::alt::CPlayerDamageEvent *cxxbridge1$unique_ptr$alt$CPlayerDamageEvent$uninit(::std::unique_ptr<::alt::CPlayerDamageEvent> *ptr) noexcept {
  ::alt::CPlayerDamageEvent *uninit = reinterpret_cast<::alt::CPlayerDamageEvent *>(new ::rust::MaybeUninit<::alt::CPlayerDamageEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDamageEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerDamageEvent$raw(::std::unique_ptr<::alt::CPlayerDamageEvent> *ptr, ::alt::CPlayerDamageEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDamageEvent>(raw);
}
::alt::CPlayerDamageEvent const *cxxbridge1$unique_ptr$alt$CPlayerDamageEvent$get(::std::unique_ptr<::alt::CPlayerDamageEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerDamageEvent *cxxbridge1$unique_ptr$alt$CPlayerDamageEvent$release(::std::unique_ptr<::alt::CPlayerDamageEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerDamageEvent$drop(::std::unique_ptr<::alt::CPlayerDamageEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerDamageEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerDamageEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerDamageEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerDamageEvent$null(::std::shared_ptr<::alt::CPlayerDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDamageEvent>();
}
::alt::CPlayerDamageEvent *cxxbridge1$shared_ptr$alt$CPlayerDamageEvent$uninit(::std::shared_ptr<::alt::CPlayerDamageEvent> *ptr) noexcept {
  ::alt::CPlayerDamageEvent *uninit = reinterpret_cast<::alt::CPlayerDamageEvent *>(new ::rust::MaybeUninit<::alt::CPlayerDamageEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDamageEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerDamageEvent$clone(::std::shared_ptr<::alt::CPlayerDamageEvent> const &self, ::std::shared_ptr<::alt::CPlayerDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDamageEvent>(self);
}
::alt::CPlayerDamageEvent const *cxxbridge1$shared_ptr$alt$CPlayerDamageEvent$get(::std::shared_ptr<::alt::CPlayerDamageEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerDamageEvent$drop(::std::shared_ptr<::alt::CPlayerDamageEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerDamageEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerDamageEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerDamageEvent$null(::std::weak_ptr<::alt::CPlayerDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerDamageEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerDamageEvent$clone(::std::weak_ptr<::alt::CPlayerDamageEvent> const &self, ::std::weak_ptr<::alt::CPlayerDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerDamageEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerDamageEvent$downgrade(::std::shared_ptr<::alt::CPlayerDamageEvent> const &shared, ::std::weak_ptr<::alt::CPlayerDamageEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerDamageEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerDamageEvent$upgrade(::std::weak_ptr<::alt::CPlayerDamageEvent> const &weak, ::std::shared_ptr<::alt::CPlayerDamageEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerDamageEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerDamageEvent$drop(::std::weak_ptr<::alt::CPlayerDamageEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerEnteringVehicleEvent>::value, "definition of CPlayerEnteringVehicleEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerEnteringVehicleEvent$null(::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent>();
}
::alt::CPlayerEnteringVehicleEvent *cxxbridge1$unique_ptr$alt$CPlayerEnteringVehicleEvent$uninit(::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent> *ptr) noexcept {
  ::alt::CPlayerEnteringVehicleEvent *uninit = reinterpret_cast<::alt::CPlayerEnteringVehicleEvent *>(new ::rust::MaybeUninit<::alt::CPlayerEnteringVehicleEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerEnteringVehicleEvent$raw(::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent> *ptr, ::alt::CPlayerEnteringVehicleEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent>(raw);
}
::alt::CPlayerEnteringVehicleEvent const *cxxbridge1$unique_ptr$alt$CPlayerEnteringVehicleEvent$get(::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerEnteringVehicleEvent *cxxbridge1$unique_ptr$alt$CPlayerEnteringVehicleEvent$release(::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerEnteringVehicleEvent$drop(::std::unique_ptr<::alt::CPlayerEnteringVehicleEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerEnteringVehicleEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerEnteringVehicleEvent$null(::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent>();
}
::alt::CPlayerEnteringVehicleEvent *cxxbridge1$shared_ptr$alt$CPlayerEnteringVehicleEvent$uninit(::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent> *ptr) noexcept {
  ::alt::CPlayerEnteringVehicleEvent *uninit = reinterpret_cast<::alt::CPlayerEnteringVehicleEvent *>(new ::rust::MaybeUninit<::alt::CPlayerEnteringVehicleEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerEnteringVehicleEvent$clone(::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent> const &self, ::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent>(self);
}
::alt::CPlayerEnteringVehicleEvent const *cxxbridge1$shared_ptr$alt$CPlayerEnteringVehicleEvent$get(::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerEnteringVehicleEvent$drop(::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerEnteringVehicleEvent$null(::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerEnteringVehicleEvent$clone(::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent> const &self, ::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerEnteringVehicleEvent$downgrade(::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent> const &shared, ::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerEnteringVehicleEvent$upgrade(::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent> const &weak, ::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerEnteringVehicleEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerEnteringVehicleEvent$drop(::std::weak_ptr<::alt::CPlayerEnteringVehicleEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerEnterVehicleEvent>::value, "definition of CPlayerEnterVehicleEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerEnterVehicleEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerEnterVehicleEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerEnterVehicleEvent$null(::std::unique_ptr<::alt::CPlayerEnterVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerEnterVehicleEvent>();
}
::alt::CPlayerEnterVehicleEvent *cxxbridge1$unique_ptr$alt$CPlayerEnterVehicleEvent$uninit(::std::unique_ptr<::alt::CPlayerEnterVehicleEvent> *ptr) noexcept {
  ::alt::CPlayerEnterVehicleEvent *uninit = reinterpret_cast<::alt::CPlayerEnterVehicleEvent *>(new ::rust::MaybeUninit<::alt::CPlayerEnterVehicleEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerEnterVehicleEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerEnterVehicleEvent$raw(::std::unique_ptr<::alt::CPlayerEnterVehicleEvent> *ptr, ::alt::CPlayerEnterVehicleEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerEnterVehicleEvent>(raw);
}
::alt::CPlayerEnterVehicleEvent const *cxxbridge1$unique_ptr$alt$CPlayerEnterVehicleEvent$get(::std::unique_ptr<::alt::CPlayerEnterVehicleEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerEnterVehicleEvent *cxxbridge1$unique_ptr$alt$CPlayerEnterVehicleEvent$release(::std::unique_ptr<::alt::CPlayerEnterVehicleEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerEnterVehicleEvent$drop(::std::unique_ptr<::alt::CPlayerEnterVehicleEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerEnterVehicleEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerEnterVehicleEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerEnterVehicleEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerEnterVehicleEvent$null(::std::shared_ptr<::alt::CPlayerEnterVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerEnterVehicleEvent>();
}
::alt::CPlayerEnterVehicleEvent *cxxbridge1$shared_ptr$alt$CPlayerEnterVehicleEvent$uninit(::std::shared_ptr<::alt::CPlayerEnterVehicleEvent> *ptr) noexcept {
  ::alt::CPlayerEnterVehicleEvent *uninit = reinterpret_cast<::alt::CPlayerEnterVehicleEvent *>(new ::rust::MaybeUninit<::alt::CPlayerEnterVehicleEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerEnterVehicleEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerEnterVehicleEvent$clone(::std::shared_ptr<::alt::CPlayerEnterVehicleEvent> const &self, ::std::shared_ptr<::alt::CPlayerEnterVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerEnterVehicleEvent>(self);
}
::alt::CPlayerEnterVehicleEvent const *cxxbridge1$shared_ptr$alt$CPlayerEnterVehicleEvent$get(::std::shared_ptr<::alt::CPlayerEnterVehicleEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerEnterVehicleEvent$drop(::std::shared_ptr<::alt::CPlayerEnterVehicleEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerEnterVehicleEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerEnterVehicleEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerEnterVehicleEvent$null(::std::weak_ptr<::alt::CPlayerEnterVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerEnterVehicleEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerEnterVehicleEvent$clone(::std::weak_ptr<::alt::CPlayerEnterVehicleEvent> const &self, ::std::weak_ptr<::alt::CPlayerEnterVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerEnterVehicleEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerEnterVehicleEvent$downgrade(::std::shared_ptr<::alt::CPlayerEnterVehicleEvent> const &shared, ::std::weak_ptr<::alt::CPlayerEnterVehicleEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerEnterVehicleEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerEnterVehicleEvent$upgrade(::std::weak_ptr<::alt::CPlayerEnterVehicleEvent> const &weak, ::std::shared_ptr<::alt::CPlayerEnterVehicleEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerEnterVehicleEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerEnterVehicleEvent$drop(::std::weak_ptr<::alt::CPlayerEnterVehicleEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerLeaveVehicleEvent>::value, "definition of CPlayerLeaveVehicleEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerLeaveVehicleEvent$null(::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent>();
}
::alt::CPlayerLeaveVehicleEvent *cxxbridge1$unique_ptr$alt$CPlayerLeaveVehicleEvent$uninit(::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent> *ptr) noexcept {
  ::alt::CPlayerLeaveVehicleEvent *uninit = reinterpret_cast<::alt::CPlayerLeaveVehicleEvent *>(new ::rust::MaybeUninit<::alt::CPlayerLeaveVehicleEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerLeaveVehicleEvent$raw(::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent> *ptr, ::alt::CPlayerLeaveVehicleEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent>(raw);
}
::alt::CPlayerLeaveVehicleEvent const *cxxbridge1$unique_ptr$alt$CPlayerLeaveVehicleEvent$get(::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerLeaveVehicleEvent *cxxbridge1$unique_ptr$alt$CPlayerLeaveVehicleEvent$release(::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerLeaveVehicleEvent$drop(::std::unique_ptr<::alt::CPlayerLeaveVehicleEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerLeaveVehicleEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerLeaveVehicleEvent$null(::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent>();
}
::alt::CPlayerLeaveVehicleEvent *cxxbridge1$shared_ptr$alt$CPlayerLeaveVehicleEvent$uninit(::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent> *ptr) noexcept {
  ::alt::CPlayerLeaveVehicleEvent *uninit = reinterpret_cast<::alt::CPlayerLeaveVehicleEvent *>(new ::rust::MaybeUninit<::alt::CPlayerLeaveVehicleEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerLeaveVehicleEvent$clone(::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent> const &self, ::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent>(self);
}
::alt::CPlayerLeaveVehicleEvent const *cxxbridge1$shared_ptr$alt$CPlayerLeaveVehicleEvent$get(::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerLeaveVehicleEvent$drop(::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerLeaveVehicleEvent$null(::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerLeaveVehicleEvent$clone(::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent> const &self, ::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerLeaveVehicleEvent$downgrade(::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent> const &shared, ::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerLeaveVehicleEvent$upgrade(::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent> const &weak, ::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerLeaveVehicleEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerLeaveVehicleEvent$drop(::std::weak_ptr<::alt::CPlayerLeaveVehicleEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerChangeAnimationEvent>::value, "definition of CPlayerChangeAnimationEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerChangeAnimationEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerChangeAnimationEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerChangeAnimationEvent$null(::std::unique_ptr<::alt::CPlayerChangeAnimationEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerChangeAnimationEvent>();
}
::alt::CPlayerChangeAnimationEvent *cxxbridge1$unique_ptr$alt$CPlayerChangeAnimationEvent$uninit(::std::unique_ptr<::alt::CPlayerChangeAnimationEvent> *ptr) noexcept {
  ::alt::CPlayerChangeAnimationEvent *uninit = reinterpret_cast<::alt::CPlayerChangeAnimationEvent *>(new ::rust::MaybeUninit<::alt::CPlayerChangeAnimationEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerChangeAnimationEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerChangeAnimationEvent$raw(::std::unique_ptr<::alt::CPlayerChangeAnimationEvent> *ptr, ::alt::CPlayerChangeAnimationEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerChangeAnimationEvent>(raw);
}
::alt::CPlayerChangeAnimationEvent const *cxxbridge1$unique_ptr$alt$CPlayerChangeAnimationEvent$get(::std::unique_ptr<::alt::CPlayerChangeAnimationEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerChangeAnimationEvent *cxxbridge1$unique_ptr$alt$CPlayerChangeAnimationEvent$release(::std::unique_ptr<::alt::CPlayerChangeAnimationEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerChangeAnimationEvent$drop(::std::unique_ptr<::alt::CPlayerChangeAnimationEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerChangeAnimationEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerChangeAnimationEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerChangeAnimationEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerChangeAnimationEvent$null(::std::shared_ptr<::alt::CPlayerChangeAnimationEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerChangeAnimationEvent>();
}
::alt::CPlayerChangeAnimationEvent *cxxbridge1$shared_ptr$alt$CPlayerChangeAnimationEvent$uninit(::std::shared_ptr<::alt::CPlayerChangeAnimationEvent> *ptr) noexcept {
  ::alt::CPlayerChangeAnimationEvent *uninit = reinterpret_cast<::alt::CPlayerChangeAnimationEvent *>(new ::rust::MaybeUninit<::alt::CPlayerChangeAnimationEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerChangeAnimationEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerChangeAnimationEvent$clone(::std::shared_ptr<::alt::CPlayerChangeAnimationEvent> const &self, ::std::shared_ptr<::alt::CPlayerChangeAnimationEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerChangeAnimationEvent>(self);
}
::alt::CPlayerChangeAnimationEvent const *cxxbridge1$shared_ptr$alt$CPlayerChangeAnimationEvent$get(::std::shared_ptr<::alt::CPlayerChangeAnimationEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerChangeAnimationEvent$drop(::std::shared_ptr<::alt::CPlayerChangeAnimationEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerChangeAnimationEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerChangeAnimationEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerChangeAnimationEvent$null(::std::weak_ptr<::alt::CPlayerChangeAnimationEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerChangeAnimationEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeAnimationEvent$clone(::std::weak_ptr<::alt::CPlayerChangeAnimationEvent> const &self, ::std::weak_ptr<::alt::CPlayerChangeAnimationEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerChangeAnimationEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeAnimationEvent$downgrade(::std::shared_ptr<::alt::CPlayerChangeAnimationEvent> const &shared, ::std::weak_ptr<::alt::CPlayerChangeAnimationEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerChangeAnimationEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeAnimationEvent$upgrade(::std::weak_ptr<::alt::CPlayerChangeAnimationEvent> const &weak, ::std::shared_ptr<::alt::CPlayerChangeAnimationEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerChangeAnimationEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeAnimationEvent$drop(::std::weak_ptr<::alt::CPlayerChangeAnimationEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerChangeVehicleSeatEvent>::value, "definition of CPlayerChangeVehicleSeatEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerChangeVehicleSeatEvent$null(::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent>();
}
::alt::CPlayerChangeVehicleSeatEvent *cxxbridge1$unique_ptr$alt$CPlayerChangeVehicleSeatEvent$uninit(::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent> *ptr) noexcept {
  ::alt::CPlayerChangeVehicleSeatEvent *uninit = reinterpret_cast<::alt::CPlayerChangeVehicleSeatEvent *>(new ::rust::MaybeUninit<::alt::CPlayerChangeVehicleSeatEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerChangeVehicleSeatEvent$raw(::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent> *ptr, ::alt::CPlayerChangeVehicleSeatEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent>(raw);
}
::alt::CPlayerChangeVehicleSeatEvent const *cxxbridge1$unique_ptr$alt$CPlayerChangeVehicleSeatEvent$get(::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerChangeVehicleSeatEvent *cxxbridge1$unique_ptr$alt$CPlayerChangeVehicleSeatEvent$release(::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerChangeVehicleSeatEvent$drop(::std::unique_ptr<::alt::CPlayerChangeVehicleSeatEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerChangeVehicleSeatEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerChangeVehicleSeatEvent$null(::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent>();
}
::alt::CPlayerChangeVehicleSeatEvent *cxxbridge1$shared_ptr$alt$CPlayerChangeVehicleSeatEvent$uninit(::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent> *ptr) noexcept {
  ::alt::CPlayerChangeVehicleSeatEvent *uninit = reinterpret_cast<::alt::CPlayerChangeVehicleSeatEvent *>(new ::rust::MaybeUninit<::alt::CPlayerChangeVehicleSeatEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerChangeVehicleSeatEvent$clone(::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent> const &self, ::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent>(self);
}
::alt::CPlayerChangeVehicleSeatEvent const *cxxbridge1$shared_ptr$alt$CPlayerChangeVehicleSeatEvent$get(::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerChangeVehicleSeatEvent$drop(::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerChangeVehicleSeatEvent$null(::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeVehicleSeatEvent$clone(::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent> const &self, ::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeVehicleSeatEvent$downgrade(::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent> const &shared, ::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeVehicleSeatEvent$upgrade(::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent> const &weak, ::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerChangeVehicleSeatEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeVehicleSeatEvent$drop(::std::weak_ptr<::alt::CPlayerChangeVehicleSeatEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerWeaponChangeEvent>::value, "definition of CPlayerWeaponChangeEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerWeaponChangeEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerWeaponChangeEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerWeaponChangeEvent$null(::std::unique_ptr<::alt::CPlayerWeaponChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerWeaponChangeEvent>();
}
::alt::CPlayerWeaponChangeEvent *cxxbridge1$unique_ptr$alt$CPlayerWeaponChangeEvent$uninit(::std::unique_ptr<::alt::CPlayerWeaponChangeEvent> *ptr) noexcept {
  ::alt::CPlayerWeaponChangeEvent *uninit = reinterpret_cast<::alt::CPlayerWeaponChangeEvent *>(new ::rust::MaybeUninit<::alt::CPlayerWeaponChangeEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerWeaponChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerWeaponChangeEvent$raw(::std::unique_ptr<::alt::CPlayerWeaponChangeEvent> *ptr, ::alt::CPlayerWeaponChangeEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerWeaponChangeEvent>(raw);
}
::alt::CPlayerWeaponChangeEvent const *cxxbridge1$unique_ptr$alt$CPlayerWeaponChangeEvent$get(::std::unique_ptr<::alt::CPlayerWeaponChangeEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerWeaponChangeEvent *cxxbridge1$unique_ptr$alt$CPlayerWeaponChangeEvent$release(::std::unique_ptr<::alt::CPlayerWeaponChangeEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerWeaponChangeEvent$drop(::std::unique_ptr<::alt::CPlayerWeaponChangeEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerWeaponChangeEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerWeaponChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerWeaponChangeEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerWeaponChangeEvent$null(::std::shared_ptr<::alt::CPlayerWeaponChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerWeaponChangeEvent>();
}
::alt::CPlayerWeaponChangeEvent *cxxbridge1$shared_ptr$alt$CPlayerWeaponChangeEvent$uninit(::std::shared_ptr<::alt::CPlayerWeaponChangeEvent> *ptr) noexcept {
  ::alt::CPlayerWeaponChangeEvent *uninit = reinterpret_cast<::alt::CPlayerWeaponChangeEvent *>(new ::rust::MaybeUninit<::alt::CPlayerWeaponChangeEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerWeaponChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerWeaponChangeEvent$clone(::std::shared_ptr<::alt::CPlayerWeaponChangeEvent> const &self, ::std::shared_ptr<::alt::CPlayerWeaponChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerWeaponChangeEvent>(self);
}
::alt::CPlayerWeaponChangeEvent const *cxxbridge1$shared_ptr$alt$CPlayerWeaponChangeEvent$get(::std::shared_ptr<::alt::CPlayerWeaponChangeEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerWeaponChangeEvent$drop(::std::shared_ptr<::alt::CPlayerWeaponChangeEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerWeaponChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerWeaponChangeEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerWeaponChangeEvent$null(::std::weak_ptr<::alt::CPlayerWeaponChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerWeaponChangeEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerWeaponChangeEvent$clone(::std::weak_ptr<::alt::CPlayerWeaponChangeEvent> const &self, ::std::weak_ptr<::alt::CPlayerWeaponChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerWeaponChangeEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerWeaponChangeEvent$downgrade(::std::shared_ptr<::alt::CPlayerWeaponChangeEvent> const &shared, ::std::weak_ptr<::alt::CPlayerWeaponChangeEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerWeaponChangeEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerWeaponChangeEvent$upgrade(::std::weak_ptr<::alt::CPlayerWeaponChangeEvent> const &weak, ::std::shared_ptr<::alt::CPlayerWeaponChangeEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerWeaponChangeEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerWeaponChangeEvent$drop(::std::weak_ptr<::alt::CPlayerWeaponChangeEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerConnectDeniedEvent>::value, "definition of CPlayerConnectDeniedEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerConnectDeniedEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerConnectDeniedEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerConnectDeniedEvent$null(::std::unique_ptr<::alt::CPlayerConnectDeniedEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerConnectDeniedEvent>();
}
::alt::CPlayerConnectDeniedEvent *cxxbridge1$unique_ptr$alt$CPlayerConnectDeniedEvent$uninit(::std::unique_ptr<::alt::CPlayerConnectDeniedEvent> *ptr) noexcept {
  ::alt::CPlayerConnectDeniedEvent *uninit = reinterpret_cast<::alt::CPlayerConnectDeniedEvent *>(new ::rust::MaybeUninit<::alt::CPlayerConnectDeniedEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerConnectDeniedEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerConnectDeniedEvent$raw(::std::unique_ptr<::alt::CPlayerConnectDeniedEvent> *ptr, ::alt::CPlayerConnectDeniedEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerConnectDeniedEvent>(raw);
}
::alt::CPlayerConnectDeniedEvent const *cxxbridge1$unique_ptr$alt$CPlayerConnectDeniedEvent$get(::std::unique_ptr<::alt::CPlayerConnectDeniedEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerConnectDeniedEvent *cxxbridge1$unique_ptr$alt$CPlayerConnectDeniedEvent$release(::std::unique_ptr<::alt::CPlayerConnectDeniedEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerConnectDeniedEvent$drop(::std::unique_ptr<::alt::CPlayerConnectDeniedEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerConnectDeniedEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerConnectDeniedEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerConnectDeniedEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerConnectDeniedEvent$null(::std::shared_ptr<::alt::CPlayerConnectDeniedEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerConnectDeniedEvent>();
}
::alt::CPlayerConnectDeniedEvent *cxxbridge1$shared_ptr$alt$CPlayerConnectDeniedEvent$uninit(::std::shared_ptr<::alt::CPlayerConnectDeniedEvent> *ptr) noexcept {
  ::alt::CPlayerConnectDeniedEvent *uninit = reinterpret_cast<::alt::CPlayerConnectDeniedEvent *>(new ::rust::MaybeUninit<::alt::CPlayerConnectDeniedEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerConnectDeniedEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerConnectDeniedEvent$clone(::std::shared_ptr<::alt::CPlayerConnectDeniedEvent> const &self, ::std::shared_ptr<::alt::CPlayerConnectDeniedEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerConnectDeniedEvent>(self);
}
::alt::CPlayerConnectDeniedEvent const *cxxbridge1$shared_ptr$alt$CPlayerConnectDeniedEvent$get(::std::shared_ptr<::alt::CPlayerConnectDeniedEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerConnectDeniedEvent$drop(::std::shared_ptr<::alt::CPlayerConnectDeniedEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerConnectDeniedEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerConnectDeniedEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerConnectDeniedEvent$null(::std::weak_ptr<::alt::CPlayerConnectDeniedEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerConnectDeniedEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerConnectDeniedEvent$clone(::std::weak_ptr<::alt::CPlayerConnectDeniedEvent> const &self, ::std::weak_ptr<::alt::CPlayerConnectDeniedEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerConnectDeniedEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerConnectDeniedEvent$downgrade(::std::shared_ptr<::alt::CPlayerConnectDeniedEvent> const &shared, ::std::weak_ptr<::alt::CPlayerConnectDeniedEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerConnectDeniedEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerConnectDeniedEvent$upgrade(::std::weak_ptr<::alt::CPlayerConnectDeniedEvent> const &weak, ::std::shared_ptr<::alt::CPlayerConnectDeniedEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerConnectDeniedEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerConnectDeniedEvent$drop(::std::weak_ptr<::alt::CPlayerConnectDeniedEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerSpawnEvent>::value, "definition of CPlayerSpawnEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerSpawnEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerSpawnEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerSpawnEvent$null(::std::unique_ptr<::alt::CPlayerSpawnEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerSpawnEvent>();
}
::alt::CPlayerSpawnEvent *cxxbridge1$unique_ptr$alt$CPlayerSpawnEvent$uninit(::std::unique_ptr<::alt::CPlayerSpawnEvent> *ptr) noexcept {
  ::alt::CPlayerSpawnEvent *uninit = reinterpret_cast<::alt::CPlayerSpawnEvent *>(new ::rust::MaybeUninit<::alt::CPlayerSpawnEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerSpawnEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerSpawnEvent$raw(::std::unique_ptr<::alt::CPlayerSpawnEvent> *ptr, ::alt::CPlayerSpawnEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerSpawnEvent>(raw);
}
::alt::CPlayerSpawnEvent const *cxxbridge1$unique_ptr$alt$CPlayerSpawnEvent$get(::std::unique_ptr<::alt::CPlayerSpawnEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerSpawnEvent *cxxbridge1$unique_ptr$alt$CPlayerSpawnEvent$release(::std::unique_ptr<::alt::CPlayerSpawnEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerSpawnEvent$drop(::std::unique_ptr<::alt::CPlayerSpawnEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerSpawnEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerSpawnEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerSpawnEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerSpawnEvent$null(::std::shared_ptr<::alt::CPlayerSpawnEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerSpawnEvent>();
}
::alt::CPlayerSpawnEvent *cxxbridge1$shared_ptr$alt$CPlayerSpawnEvent$uninit(::std::shared_ptr<::alt::CPlayerSpawnEvent> *ptr) noexcept {
  ::alt::CPlayerSpawnEvent *uninit = reinterpret_cast<::alt::CPlayerSpawnEvent *>(new ::rust::MaybeUninit<::alt::CPlayerSpawnEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerSpawnEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerSpawnEvent$clone(::std::shared_ptr<::alt::CPlayerSpawnEvent> const &self, ::std::shared_ptr<::alt::CPlayerSpawnEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerSpawnEvent>(self);
}
::alt::CPlayerSpawnEvent const *cxxbridge1$shared_ptr$alt$CPlayerSpawnEvent$get(::std::shared_ptr<::alt::CPlayerSpawnEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerSpawnEvent$drop(::std::shared_ptr<::alt::CPlayerSpawnEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerSpawnEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerSpawnEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerSpawnEvent$null(::std::weak_ptr<::alt::CPlayerSpawnEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerSpawnEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerSpawnEvent$clone(::std::weak_ptr<::alt::CPlayerSpawnEvent> const &self, ::std::weak_ptr<::alt::CPlayerSpawnEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerSpawnEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerSpawnEvent$downgrade(::std::shared_ptr<::alt::CPlayerSpawnEvent> const &shared, ::std::weak_ptr<::alt::CPlayerSpawnEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerSpawnEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerSpawnEvent$upgrade(::std::weak_ptr<::alt::CPlayerSpawnEvent> const &weak, ::std::shared_ptr<::alt::CPlayerSpawnEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerSpawnEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerSpawnEvent$drop(::std::weak_ptr<::alt::CPlayerSpawnEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CStartProjectileEvent>::value, "definition of CStartProjectileEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CStartProjectileEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CStartProjectileEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CStartProjectileEvent$null(::std::unique_ptr<::alt::CStartProjectileEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CStartProjectileEvent>();
}
::alt::CStartProjectileEvent *cxxbridge1$unique_ptr$alt$CStartProjectileEvent$uninit(::std::unique_ptr<::alt::CStartProjectileEvent> *ptr) noexcept {
  ::alt::CStartProjectileEvent *uninit = reinterpret_cast<::alt::CStartProjectileEvent *>(new ::rust::MaybeUninit<::alt::CStartProjectileEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CStartProjectileEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CStartProjectileEvent$raw(::std::unique_ptr<::alt::CStartProjectileEvent> *ptr, ::alt::CStartProjectileEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CStartProjectileEvent>(raw);
}
::alt::CStartProjectileEvent const *cxxbridge1$unique_ptr$alt$CStartProjectileEvent$get(::std::unique_ptr<::alt::CStartProjectileEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CStartProjectileEvent *cxxbridge1$unique_ptr$alt$CStartProjectileEvent$release(::std::unique_ptr<::alt::CStartProjectileEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CStartProjectileEvent$drop(::std::unique_ptr<::alt::CStartProjectileEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CStartProjectileEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CStartProjectileEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CStartProjectileEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CStartProjectileEvent$null(::std::shared_ptr<::alt::CStartProjectileEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CStartProjectileEvent>();
}
::alt::CStartProjectileEvent *cxxbridge1$shared_ptr$alt$CStartProjectileEvent$uninit(::std::shared_ptr<::alt::CStartProjectileEvent> *ptr) noexcept {
  ::alt::CStartProjectileEvent *uninit = reinterpret_cast<::alt::CStartProjectileEvent *>(new ::rust::MaybeUninit<::alt::CStartProjectileEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CStartProjectileEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CStartProjectileEvent$clone(::std::shared_ptr<::alt::CStartProjectileEvent> const &self, ::std::shared_ptr<::alt::CStartProjectileEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CStartProjectileEvent>(self);
}
::alt::CStartProjectileEvent const *cxxbridge1$shared_ptr$alt$CStartProjectileEvent$get(::std::shared_ptr<::alt::CStartProjectileEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CStartProjectileEvent$drop(::std::shared_ptr<::alt::CStartProjectileEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CStartProjectileEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CStartProjectileEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CStartProjectileEvent$null(::std::weak_ptr<::alt::CStartProjectileEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CStartProjectileEvent>();
}
void cxxbridge1$weak_ptr$alt$CStartProjectileEvent$clone(::std::weak_ptr<::alt::CStartProjectileEvent> const &self, ::std::weak_ptr<::alt::CStartProjectileEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CStartProjectileEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CStartProjectileEvent$downgrade(::std::shared_ptr<::alt::CStartProjectileEvent> const &shared, ::std::weak_ptr<::alt::CStartProjectileEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CStartProjectileEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CStartProjectileEvent$upgrade(::std::weak_ptr<::alt::CStartProjectileEvent> const &weak, ::std::shared_ptr<::alt::CStartProjectileEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CStartProjectileEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CStartProjectileEvent$drop(::std::weak_ptr<::alt::CStartProjectileEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerRequestControlEvent>::value, "definition of CPlayerRequestControlEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerRequestControlEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerRequestControlEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerRequestControlEvent$null(::std::unique_ptr<::alt::CPlayerRequestControlEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerRequestControlEvent>();
}
::alt::CPlayerRequestControlEvent *cxxbridge1$unique_ptr$alt$CPlayerRequestControlEvent$uninit(::std::unique_ptr<::alt::CPlayerRequestControlEvent> *ptr) noexcept {
  ::alt::CPlayerRequestControlEvent *uninit = reinterpret_cast<::alt::CPlayerRequestControlEvent *>(new ::rust::MaybeUninit<::alt::CPlayerRequestControlEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerRequestControlEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerRequestControlEvent$raw(::std::unique_ptr<::alt::CPlayerRequestControlEvent> *ptr, ::alt::CPlayerRequestControlEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerRequestControlEvent>(raw);
}
::alt::CPlayerRequestControlEvent const *cxxbridge1$unique_ptr$alt$CPlayerRequestControlEvent$get(::std::unique_ptr<::alt::CPlayerRequestControlEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerRequestControlEvent *cxxbridge1$unique_ptr$alt$CPlayerRequestControlEvent$release(::std::unique_ptr<::alt::CPlayerRequestControlEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerRequestControlEvent$drop(::std::unique_ptr<::alt::CPlayerRequestControlEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerRequestControlEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerRequestControlEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerRequestControlEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerRequestControlEvent$null(::std::shared_ptr<::alt::CPlayerRequestControlEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerRequestControlEvent>();
}
::alt::CPlayerRequestControlEvent *cxxbridge1$shared_ptr$alt$CPlayerRequestControlEvent$uninit(::std::shared_ptr<::alt::CPlayerRequestControlEvent> *ptr) noexcept {
  ::alt::CPlayerRequestControlEvent *uninit = reinterpret_cast<::alt::CPlayerRequestControlEvent *>(new ::rust::MaybeUninit<::alt::CPlayerRequestControlEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerRequestControlEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerRequestControlEvent$clone(::std::shared_ptr<::alt::CPlayerRequestControlEvent> const &self, ::std::shared_ptr<::alt::CPlayerRequestControlEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerRequestControlEvent>(self);
}
::alt::CPlayerRequestControlEvent const *cxxbridge1$shared_ptr$alt$CPlayerRequestControlEvent$get(::std::shared_ptr<::alt::CPlayerRequestControlEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerRequestControlEvent$drop(::std::shared_ptr<::alt::CPlayerRequestControlEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerRequestControlEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerRequestControlEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerRequestControlEvent$null(::std::weak_ptr<::alt::CPlayerRequestControlEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerRequestControlEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerRequestControlEvent$clone(::std::weak_ptr<::alt::CPlayerRequestControlEvent> const &self, ::std::weak_ptr<::alt::CPlayerRequestControlEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerRequestControlEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerRequestControlEvent$downgrade(::std::shared_ptr<::alt::CPlayerRequestControlEvent> const &shared, ::std::weak_ptr<::alt::CPlayerRequestControlEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerRequestControlEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerRequestControlEvent$upgrade(::std::weak_ptr<::alt::CPlayerRequestControlEvent> const &weak, ::std::shared_ptr<::alt::CPlayerRequestControlEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerRequestControlEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerRequestControlEvent$drop(::std::weak_ptr<::alt::CPlayerRequestControlEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerDimensionChangeEvent>::value, "definition of CPlayerDimensionChangeEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerDimensionChangeEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerDimensionChangeEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerDimensionChangeEvent$null(::std::unique_ptr<::alt::CPlayerDimensionChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDimensionChangeEvent>();
}
::alt::CPlayerDimensionChangeEvent *cxxbridge1$unique_ptr$alt$CPlayerDimensionChangeEvent$uninit(::std::unique_ptr<::alt::CPlayerDimensionChangeEvent> *ptr) noexcept {
  ::alt::CPlayerDimensionChangeEvent *uninit = reinterpret_cast<::alt::CPlayerDimensionChangeEvent *>(new ::rust::MaybeUninit<::alt::CPlayerDimensionChangeEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDimensionChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerDimensionChangeEvent$raw(::std::unique_ptr<::alt::CPlayerDimensionChangeEvent> *ptr, ::alt::CPlayerDimensionChangeEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerDimensionChangeEvent>(raw);
}
::alt::CPlayerDimensionChangeEvent const *cxxbridge1$unique_ptr$alt$CPlayerDimensionChangeEvent$get(::std::unique_ptr<::alt::CPlayerDimensionChangeEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerDimensionChangeEvent *cxxbridge1$unique_ptr$alt$CPlayerDimensionChangeEvent$release(::std::unique_ptr<::alt::CPlayerDimensionChangeEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerDimensionChangeEvent$drop(::std::unique_ptr<::alt::CPlayerDimensionChangeEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerDimensionChangeEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerDimensionChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerDimensionChangeEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerDimensionChangeEvent$null(::std::shared_ptr<::alt::CPlayerDimensionChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDimensionChangeEvent>();
}
::alt::CPlayerDimensionChangeEvent *cxxbridge1$shared_ptr$alt$CPlayerDimensionChangeEvent$uninit(::std::shared_ptr<::alt::CPlayerDimensionChangeEvent> *ptr) noexcept {
  ::alt::CPlayerDimensionChangeEvent *uninit = reinterpret_cast<::alt::CPlayerDimensionChangeEvent *>(new ::rust::MaybeUninit<::alt::CPlayerDimensionChangeEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDimensionChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerDimensionChangeEvent$clone(::std::shared_ptr<::alt::CPlayerDimensionChangeEvent> const &self, ::std::shared_ptr<::alt::CPlayerDimensionChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerDimensionChangeEvent>(self);
}
::alt::CPlayerDimensionChangeEvent const *cxxbridge1$shared_ptr$alt$CPlayerDimensionChangeEvent$get(::std::shared_ptr<::alt::CPlayerDimensionChangeEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerDimensionChangeEvent$drop(::std::shared_ptr<::alt::CPlayerDimensionChangeEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerDimensionChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerDimensionChangeEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerDimensionChangeEvent$null(::std::weak_ptr<::alt::CPlayerDimensionChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerDimensionChangeEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerDimensionChangeEvent$clone(::std::weak_ptr<::alt::CPlayerDimensionChangeEvent> const &self, ::std::weak_ptr<::alt::CPlayerDimensionChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerDimensionChangeEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerDimensionChangeEvent$downgrade(::std::shared_ptr<::alt::CPlayerDimensionChangeEvent> const &shared, ::std::weak_ptr<::alt::CPlayerDimensionChangeEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerDimensionChangeEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerDimensionChangeEvent$upgrade(::std::weak_ptr<::alt::CPlayerDimensionChangeEvent> const &weak, ::std::shared_ptr<::alt::CPlayerDimensionChangeEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerDimensionChangeEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerDimensionChangeEvent$drop(::std::weak_ptr<::alt::CPlayerDimensionChangeEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerChangeInteriorEvent>::value, "definition of CPlayerChangeInteriorEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerChangeInteriorEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerChangeInteriorEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerChangeInteriorEvent$null(::std::unique_ptr<::alt::CPlayerChangeInteriorEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerChangeInteriorEvent>();
}
::alt::CPlayerChangeInteriorEvent *cxxbridge1$unique_ptr$alt$CPlayerChangeInteriorEvent$uninit(::std::unique_ptr<::alt::CPlayerChangeInteriorEvent> *ptr) noexcept {
  ::alt::CPlayerChangeInteriorEvent *uninit = reinterpret_cast<::alt::CPlayerChangeInteriorEvent *>(new ::rust::MaybeUninit<::alt::CPlayerChangeInteriorEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerChangeInteriorEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerChangeInteriorEvent$raw(::std::unique_ptr<::alt::CPlayerChangeInteriorEvent> *ptr, ::alt::CPlayerChangeInteriorEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerChangeInteriorEvent>(raw);
}
::alt::CPlayerChangeInteriorEvent const *cxxbridge1$unique_ptr$alt$CPlayerChangeInteriorEvent$get(::std::unique_ptr<::alt::CPlayerChangeInteriorEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerChangeInteriorEvent *cxxbridge1$unique_ptr$alt$CPlayerChangeInteriorEvent$release(::std::unique_ptr<::alt::CPlayerChangeInteriorEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerChangeInteriorEvent$drop(::std::unique_ptr<::alt::CPlayerChangeInteriorEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerChangeInteriorEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerChangeInteriorEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerChangeInteriorEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerChangeInteriorEvent$null(::std::shared_ptr<::alt::CPlayerChangeInteriorEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerChangeInteriorEvent>();
}
::alt::CPlayerChangeInteriorEvent *cxxbridge1$shared_ptr$alt$CPlayerChangeInteriorEvent$uninit(::std::shared_ptr<::alt::CPlayerChangeInteriorEvent> *ptr) noexcept {
  ::alt::CPlayerChangeInteriorEvent *uninit = reinterpret_cast<::alt::CPlayerChangeInteriorEvent *>(new ::rust::MaybeUninit<::alt::CPlayerChangeInteriorEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerChangeInteriorEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerChangeInteriorEvent$clone(::std::shared_ptr<::alt::CPlayerChangeInteriorEvent> const &self, ::std::shared_ptr<::alt::CPlayerChangeInteriorEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerChangeInteriorEvent>(self);
}
::alt::CPlayerChangeInteriorEvent const *cxxbridge1$shared_ptr$alt$CPlayerChangeInteriorEvent$get(::std::shared_ptr<::alt::CPlayerChangeInteriorEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerChangeInteriorEvent$drop(::std::shared_ptr<::alt::CPlayerChangeInteriorEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerChangeInteriorEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerChangeInteriorEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerChangeInteriorEvent$null(::std::weak_ptr<::alt::CPlayerChangeInteriorEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerChangeInteriorEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeInteriorEvent$clone(::std::weak_ptr<::alt::CPlayerChangeInteriorEvent> const &self, ::std::weak_ptr<::alt::CPlayerChangeInteriorEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerChangeInteriorEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeInteriorEvent$downgrade(::std::shared_ptr<::alt::CPlayerChangeInteriorEvent> const &shared, ::std::weak_ptr<::alt::CPlayerChangeInteriorEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerChangeInteriorEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeInteriorEvent$upgrade(::std::weak_ptr<::alt::CPlayerChangeInteriorEvent> const &weak, ::std::shared_ptr<::alt::CPlayerChangeInteriorEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerChangeInteriorEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerChangeInteriorEvent$drop(::std::weak_ptr<::alt::CPlayerChangeInteriorEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CExplosionEvent>::value, "definition of CExplosionEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CExplosionEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CExplosionEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CExplosionEvent$null(::std::unique_ptr<::alt::CExplosionEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CExplosionEvent>();
}
::alt::CExplosionEvent *cxxbridge1$unique_ptr$alt$CExplosionEvent$uninit(::std::unique_ptr<::alt::CExplosionEvent> *ptr) noexcept {
  ::alt::CExplosionEvent *uninit = reinterpret_cast<::alt::CExplosionEvent *>(new ::rust::MaybeUninit<::alt::CExplosionEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CExplosionEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CExplosionEvent$raw(::std::unique_ptr<::alt::CExplosionEvent> *ptr, ::alt::CExplosionEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CExplosionEvent>(raw);
}
::alt::CExplosionEvent const *cxxbridge1$unique_ptr$alt$CExplosionEvent$get(::std::unique_ptr<::alt::CExplosionEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CExplosionEvent *cxxbridge1$unique_ptr$alt$CExplosionEvent$release(::std::unique_ptr<::alt::CExplosionEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CExplosionEvent$drop(::std::unique_ptr<::alt::CExplosionEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CExplosionEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CExplosionEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CExplosionEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CExplosionEvent$null(::std::shared_ptr<::alt::CExplosionEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CExplosionEvent>();
}
::alt::CExplosionEvent *cxxbridge1$shared_ptr$alt$CExplosionEvent$uninit(::std::shared_ptr<::alt::CExplosionEvent> *ptr) noexcept {
  ::alt::CExplosionEvent *uninit = reinterpret_cast<::alt::CExplosionEvent *>(new ::rust::MaybeUninit<::alt::CExplosionEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CExplosionEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CExplosionEvent$clone(::std::shared_ptr<::alt::CExplosionEvent> const &self, ::std::shared_ptr<::alt::CExplosionEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CExplosionEvent>(self);
}
::alt::CExplosionEvent const *cxxbridge1$shared_ptr$alt$CExplosionEvent$get(::std::shared_ptr<::alt::CExplosionEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CExplosionEvent$drop(::std::shared_ptr<::alt::CExplosionEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CExplosionEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CExplosionEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CExplosionEvent$null(::std::weak_ptr<::alt::CExplosionEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CExplosionEvent>();
}
void cxxbridge1$weak_ptr$alt$CExplosionEvent$clone(::std::weak_ptr<::alt::CExplosionEvent> const &self, ::std::weak_ptr<::alt::CExplosionEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CExplosionEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CExplosionEvent$downgrade(::std::shared_ptr<::alt::CExplosionEvent> const &shared, ::std::weak_ptr<::alt::CExplosionEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CExplosionEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CExplosionEvent$upgrade(::std::weak_ptr<::alt::CExplosionEvent> const &weak, ::std::shared_ptr<::alt::CExplosionEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CExplosionEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CExplosionEvent$drop(::std::weak_ptr<::alt::CExplosionEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CFireEvent>::value, "definition of CFireEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CFireEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CFireEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CFireEvent$null(::std::unique_ptr<::alt::CFireEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CFireEvent>();
}
::alt::CFireEvent *cxxbridge1$unique_ptr$alt$CFireEvent$uninit(::std::unique_ptr<::alt::CFireEvent> *ptr) noexcept {
  ::alt::CFireEvent *uninit = reinterpret_cast<::alt::CFireEvent *>(new ::rust::MaybeUninit<::alt::CFireEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CFireEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CFireEvent$raw(::std::unique_ptr<::alt::CFireEvent> *ptr, ::alt::CFireEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CFireEvent>(raw);
}
::alt::CFireEvent const *cxxbridge1$unique_ptr$alt$CFireEvent$get(::std::unique_ptr<::alt::CFireEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CFireEvent *cxxbridge1$unique_ptr$alt$CFireEvent$release(::std::unique_ptr<::alt::CFireEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CFireEvent$drop(::std::unique_ptr<::alt::CFireEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CFireEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CFireEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CFireEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CFireEvent$null(::std::shared_ptr<::alt::CFireEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CFireEvent>();
}
::alt::CFireEvent *cxxbridge1$shared_ptr$alt$CFireEvent$uninit(::std::shared_ptr<::alt::CFireEvent> *ptr) noexcept {
  ::alt::CFireEvent *uninit = reinterpret_cast<::alt::CFireEvent *>(new ::rust::MaybeUninit<::alt::CFireEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CFireEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CFireEvent$clone(::std::shared_ptr<::alt::CFireEvent> const &self, ::std::shared_ptr<::alt::CFireEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CFireEvent>(self);
}
::alt::CFireEvent const *cxxbridge1$shared_ptr$alt$CFireEvent$get(::std::shared_ptr<::alt::CFireEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CFireEvent$drop(::std::shared_ptr<::alt::CFireEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CFireEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CFireEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CFireEvent$null(::std::weak_ptr<::alt::CFireEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CFireEvent>();
}
void cxxbridge1$weak_ptr$alt$CFireEvent$clone(::std::weak_ptr<::alt::CFireEvent> const &self, ::std::weak_ptr<::alt::CFireEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CFireEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CFireEvent$downgrade(::std::shared_ptr<::alt::CFireEvent> const &shared, ::std::weak_ptr<::alt::CFireEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CFireEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CFireEvent$upgrade(::std::weak_ptr<::alt::CFireEvent> const &weak, ::std::shared_ptr<::alt::CFireEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CFireEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CFireEvent$drop(::std::weak_ptr<::alt::CFireEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CConnectionQueueAddEvent>::value, "definition of CConnectionQueueAddEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CConnectionQueueAddEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CConnectionQueueAddEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CConnectionQueueAddEvent$null(::std::unique_ptr<::alt::CConnectionQueueAddEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CConnectionQueueAddEvent>();
}
::alt::CConnectionQueueAddEvent *cxxbridge1$unique_ptr$alt$CConnectionQueueAddEvent$uninit(::std::unique_ptr<::alt::CConnectionQueueAddEvent> *ptr) noexcept {
  ::alt::CConnectionQueueAddEvent *uninit = reinterpret_cast<::alt::CConnectionQueueAddEvent *>(new ::rust::MaybeUninit<::alt::CConnectionQueueAddEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CConnectionQueueAddEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CConnectionQueueAddEvent$raw(::std::unique_ptr<::alt::CConnectionQueueAddEvent> *ptr, ::alt::CConnectionQueueAddEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CConnectionQueueAddEvent>(raw);
}
::alt::CConnectionQueueAddEvent const *cxxbridge1$unique_ptr$alt$CConnectionQueueAddEvent$get(::std::unique_ptr<::alt::CConnectionQueueAddEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CConnectionQueueAddEvent *cxxbridge1$unique_ptr$alt$CConnectionQueueAddEvent$release(::std::unique_ptr<::alt::CConnectionQueueAddEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CConnectionQueueAddEvent$drop(::std::unique_ptr<::alt::CConnectionQueueAddEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CConnectionQueueAddEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CConnectionQueueAddEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CConnectionQueueAddEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CConnectionQueueAddEvent$null(::std::shared_ptr<::alt::CConnectionQueueAddEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CConnectionQueueAddEvent>();
}
::alt::CConnectionQueueAddEvent *cxxbridge1$shared_ptr$alt$CConnectionQueueAddEvent$uninit(::std::shared_ptr<::alt::CConnectionQueueAddEvent> *ptr) noexcept {
  ::alt::CConnectionQueueAddEvent *uninit = reinterpret_cast<::alt::CConnectionQueueAddEvent *>(new ::rust::MaybeUninit<::alt::CConnectionQueueAddEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CConnectionQueueAddEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CConnectionQueueAddEvent$clone(::std::shared_ptr<::alt::CConnectionQueueAddEvent> const &self, ::std::shared_ptr<::alt::CConnectionQueueAddEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CConnectionQueueAddEvent>(self);
}
::alt::CConnectionQueueAddEvent const *cxxbridge1$shared_ptr$alt$CConnectionQueueAddEvent$get(::std::shared_ptr<::alt::CConnectionQueueAddEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CConnectionQueueAddEvent$drop(::std::shared_ptr<::alt::CConnectionQueueAddEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CConnectionQueueAddEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CConnectionQueueAddEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CConnectionQueueAddEvent$null(::std::weak_ptr<::alt::CConnectionQueueAddEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CConnectionQueueAddEvent>();
}
void cxxbridge1$weak_ptr$alt$CConnectionQueueAddEvent$clone(::std::weak_ptr<::alt::CConnectionQueueAddEvent> const &self, ::std::weak_ptr<::alt::CConnectionQueueAddEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CConnectionQueueAddEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CConnectionQueueAddEvent$downgrade(::std::shared_ptr<::alt::CConnectionQueueAddEvent> const &shared, ::std::weak_ptr<::alt::CConnectionQueueAddEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CConnectionQueueAddEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CConnectionQueueAddEvent$upgrade(::std::weak_ptr<::alt::CConnectionQueueAddEvent> const &weak, ::std::shared_ptr<::alt::CConnectionQueueAddEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CConnectionQueueAddEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CConnectionQueueAddEvent$drop(::std::weak_ptr<::alt::CConnectionQueueAddEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CConnectionQueueRemoveEvent>::value, "definition of CConnectionQueueRemoveEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CConnectionQueueRemoveEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CConnectionQueueRemoveEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CConnectionQueueRemoveEvent$null(::std::unique_ptr<::alt::CConnectionQueueRemoveEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CConnectionQueueRemoveEvent>();
}
::alt::CConnectionQueueRemoveEvent *cxxbridge1$unique_ptr$alt$CConnectionQueueRemoveEvent$uninit(::std::unique_ptr<::alt::CConnectionQueueRemoveEvent> *ptr) noexcept {
  ::alt::CConnectionQueueRemoveEvent *uninit = reinterpret_cast<::alt::CConnectionQueueRemoveEvent *>(new ::rust::MaybeUninit<::alt::CConnectionQueueRemoveEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CConnectionQueueRemoveEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CConnectionQueueRemoveEvent$raw(::std::unique_ptr<::alt::CConnectionQueueRemoveEvent> *ptr, ::alt::CConnectionQueueRemoveEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CConnectionQueueRemoveEvent>(raw);
}
::alt::CConnectionQueueRemoveEvent const *cxxbridge1$unique_ptr$alt$CConnectionQueueRemoveEvent$get(::std::unique_ptr<::alt::CConnectionQueueRemoveEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CConnectionQueueRemoveEvent *cxxbridge1$unique_ptr$alt$CConnectionQueueRemoveEvent$release(::std::unique_ptr<::alt::CConnectionQueueRemoveEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CConnectionQueueRemoveEvent$drop(::std::unique_ptr<::alt::CConnectionQueueRemoveEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CConnectionQueueRemoveEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CConnectionQueueRemoveEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CConnectionQueueRemoveEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CConnectionQueueRemoveEvent$null(::std::shared_ptr<::alt::CConnectionQueueRemoveEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CConnectionQueueRemoveEvent>();
}
::alt::CConnectionQueueRemoveEvent *cxxbridge1$shared_ptr$alt$CConnectionQueueRemoveEvent$uninit(::std::shared_ptr<::alt::CConnectionQueueRemoveEvent> *ptr) noexcept {
  ::alt::CConnectionQueueRemoveEvent *uninit = reinterpret_cast<::alt::CConnectionQueueRemoveEvent *>(new ::rust::MaybeUninit<::alt::CConnectionQueueRemoveEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CConnectionQueueRemoveEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CConnectionQueueRemoveEvent$clone(::std::shared_ptr<::alt::CConnectionQueueRemoveEvent> const &self, ::std::shared_ptr<::alt::CConnectionQueueRemoveEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CConnectionQueueRemoveEvent>(self);
}
::alt::CConnectionQueueRemoveEvent const *cxxbridge1$shared_ptr$alt$CConnectionQueueRemoveEvent$get(::std::shared_ptr<::alt::CConnectionQueueRemoveEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CConnectionQueueRemoveEvent$drop(::std::shared_ptr<::alt::CConnectionQueueRemoveEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CConnectionQueueRemoveEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CConnectionQueueRemoveEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CConnectionQueueRemoveEvent$null(::std::weak_ptr<::alt::CConnectionQueueRemoveEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CConnectionQueueRemoveEvent>();
}
void cxxbridge1$weak_ptr$alt$CConnectionQueueRemoveEvent$clone(::std::weak_ptr<::alt::CConnectionQueueRemoveEvent> const &self, ::std::weak_ptr<::alt::CConnectionQueueRemoveEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CConnectionQueueRemoveEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CConnectionQueueRemoveEvent$downgrade(::std::shared_ptr<::alt::CConnectionQueueRemoveEvent> const &shared, ::std::weak_ptr<::alt::CConnectionQueueRemoveEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CConnectionQueueRemoveEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CConnectionQueueRemoveEvent$upgrade(::std::weak_ptr<::alt::CConnectionQueueRemoveEvent> const &weak, ::std::shared_ptr<::alt::CConnectionQueueRemoveEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CConnectionQueueRemoveEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CConnectionQueueRemoveEvent$drop(::std::weak_ptr<::alt::CConnectionQueueRemoveEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPlayerHealEvent>::value, "definition of CPlayerHealEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPlayerHealEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPlayerHealEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPlayerHealEvent$null(::std::unique_ptr<::alt::CPlayerHealEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerHealEvent>();
}
::alt::CPlayerHealEvent *cxxbridge1$unique_ptr$alt$CPlayerHealEvent$uninit(::std::unique_ptr<::alt::CPlayerHealEvent> *ptr) noexcept {
  ::alt::CPlayerHealEvent *uninit = reinterpret_cast<::alt::CPlayerHealEvent *>(new ::rust::MaybeUninit<::alt::CPlayerHealEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerHealEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPlayerHealEvent$raw(::std::unique_ptr<::alt::CPlayerHealEvent> *ptr, ::alt::CPlayerHealEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPlayerHealEvent>(raw);
}
::alt::CPlayerHealEvent const *cxxbridge1$unique_ptr$alt$CPlayerHealEvent$get(::std::unique_ptr<::alt::CPlayerHealEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPlayerHealEvent *cxxbridge1$unique_ptr$alt$CPlayerHealEvent$release(::std::unique_ptr<::alt::CPlayerHealEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPlayerHealEvent$drop(::std::unique_ptr<::alt::CPlayerHealEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPlayerHealEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPlayerHealEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPlayerHealEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPlayerHealEvent$null(::std::shared_ptr<::alt::CPlayerHealEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerHealEvent>();
}
::alt::CPlayerHealEvent *cxxbridge1$shared_ptr$alt$CPlayerHealEvent$uninit(::std::shared_ptr<::alt::CPlayerHealEvent> *ptr) noexcept {
  ::alt::CPlayerHealEvent *uninit = reinterpret_cast<::alt::CPlayerHealEvent *>(new ::rust::MaybeUninit<::alt::CPlayerHealEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerHealEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPlayerHealEvent$clone(::std::shared_ptr<::alt::CPlayerHealEvent> const &self, ::std::shared_ptr<::alt::CPlayerHealEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPlayerHealEvent>(self);
}
::alt::CPlayerHealEvent const *cxxbridge1$shared_ptr$alt$CPlayerHealEvent$get(::std::shared_ptr<::alt::CPlayerHealEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPlayerHealEvent$drop(::std::shared_ptr<::alt::CPlayerHealEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPlayerHealEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPlayerHealEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPlayerHealEvent$null(::std::weak_ptr<::alt::CPlayerHealEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerHealEvent>();
}
void cxxbridge1$weak_ptr$alt$CPlayerHealEvent$clone(::std::weak_ptr<::alt::CPlayerHealEvent> const &self, ::std::weak_ptr<::alt::CPlayerHealEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPlayerHealEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPlayerHealEvent$downgrade(::std::shared_ptr<::alt::CPlayerHealEvent> const &shared, ::std::weak_ptr<::alt::CPlayerHealEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPlayerHealEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPlayerHealEvent$upgrade(::std::weak_ptr<::alt::CPlayerHealEvent> const &weak, ::std::shared_ptr<::alt::CPlayerHealEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPlayerHealEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPlayerHealEvent$drop(::std::weak_ptr<::alt::CPlayerHealEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CVehicleAttachEvent>::value, "definition of CVehicleAttachEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CVehicleAttachEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CVehicleAttachEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CVehicleAttachEvent$null(::std::unique_ptr<::alt::CVehicleAttachEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleAttachEvent>();
}
::alt::CVehicleAttachEvent *cxxbridge1$unique_ptr$alt$CVehicleAttachEvent$uninit(::std::unique_ptr<::alt::CVehicleAttachEvent> *ptr) noexcept {
  ::alt::CVehicleAttachEvent *uninit = reinterpret_cast<::alt::CVehicleAttachEvent *>(new ::rust::MaybeUninit<::alt::CVehicleAttachEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleAttachEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CVehicleAttachEvent$raw(::std::unique_ptr<::alt::CVehicleAttachEvent> *ptr, ::alt::CVehicleAttachEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleAttachEvent>(raw);
}
::alt::CVehicleAttachEvent const *cxxbridge1$unique_ptr$alt$CVehicleAttachEvent$get(::std::unique_ptr<::alt::CVehicleAttachEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CVehicleAttachEvent *cxxbridge1$unique_ptr$alt$CVehicleAttachEvent$release(::std::unique_ptr<::alt::CVehicleAttachEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CVehicleAttachEvent$drop(::std::unique_ptr<::alt::CVehicleAttachEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CVehicleAttachEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CVehicleAttachEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CVehicleAttachEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CVehicleAttachEvent$null(::std::shared_ptr<::alt::CVehicleAttachEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleAttachEvent>();
}
::alt::CVehicleAttachEvent *cxxbridge1$shared_ptr$alt$CVehicleAttachEvent$uninit(::std::shared_ptr<::alt::CVehicleAttachEvent> *ptr) noexcept {
  ::alt::CVehicleAttachEvent *uninit = reinterpret_cast<::alt::CVehicleAttachEvent *>(new ::rust::MaybeUninit<::alt::CVehicleAttachEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleAttachEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CVehicleAttachEvent$clone(::std::shared_ptr<::alt::CVehicleAttachEvent> const &self, ::std::shared_ptr<::alt::CVehicleAttachEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleAttachEvent>(self);
}
::alt::CVehicleAttachEvent const *cxxbridge1$shared_ptr$alt$CVehicleAttachEvent$get(::std::shared_ptr<::alt::CVehicleAttachEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CVehicleAttachEvent$drop(::std::shared_ptr<::alt::CVehicleAttachEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CVehicleAttachEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CVehicleAttachEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CVehicleAttachEvent$null(::std::weak_ptr<::alt::CVehicleAttachEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleAttachEvent>();
}
void cxxbridge1$weak_ptr$alt$CVehicleAttachEvent$clone(::std::weak_ptr<::alt::CVehicleAttachEvent> const &self, ::std::weak_ptr<::alt::CVehicleAttachEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleAttachEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CVehicleAttachEvent$downgrade(::std::shared_ptr<::alt::CVehicleAttachEvent> const &shared, ::std::weak_ptr<::alt::CVehicleAttachEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CVehicleAttachEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CVehicleAttachEvent$upgrade(::std::weak_ptr<::alt::CVehicleAttachEvent> const &weak, ::std::shared_ptr<::alt::CVehicleAttachEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CVehicleAttachEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CVehicleAttachEvent$drop(::std::weak_ptr<::alt::CVehicleAttachEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CVehicleDetachEvent>::value, "definition of CVehicleDetachEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CVehicleDetachEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CVehicleDetachEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CVehicleDetachEvent$null(::std::unique_ptr<::alt::CVehicleDetachEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleDetachEvent>();
}
::alt::CVehicleDetachEvent *cxxbridge1$unique_ptr$alt$CVehicleDetachEvent$uninit(::std::unique_ptr<::alt::CVehicleDetachEvent> *ptr) noexcept {
  ::alt::CVehicleDetachEvent *uninit = reinterpret_cast<::alt::CVehicleDetachEvent *>(new ::rust::MaybeUninit<::alt::CVehicleDetachEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleDetachEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CVehicleDetachEvent$raw(::std::unique_ptr<::alt::CVehicleDetachEvent> *ptr, ::alt::CVehicleDetachEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleDetachEvent>(raw);
}
::alt::CVehicleDetachEvent const *cxxbridge1$unique_ptr$alt$CVehicleDetachEvent$get(::std::unique_ptr<::alt::CVehicleDetachEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CVehicleDetachEvent *cxxbridge1$unique_ptr$alt$CVehicleDetachEvent$release(::std::unique_ptr<::alt::CVehicleDetachEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CVehicleDetachEvent$drop(::std::unique_ptr<::alt::CVehicleDetachEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CVehicleDetachEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CVehicleDetachEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CVehicleDetachEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CVehicleDetachEvent$null(::std::shared_ptr<::alt::CVehicleDetachEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleDetachEvent>();
}
::alt::CVehicleDetachEvent *cxxbridge1$shared_ptr$alt$CVehicleDetachEvent$uninit(::std::shared_ptr<::alt::CVehicleDetachEvent> *ptr) noexcept {
  ::alt::CVehicleDetachEvent *uninit = reinterpret_cast<::alt::CVehicleDetachEvent *>(new ::rust::MaybeUninit<::alt::CVehicleDetachEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleDetachEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CVehicleDetachEvent$clone(::std::shared_ptr<::alt::CVehicleDetachEvent> const &self, ::std::shared_ptr<::alt::CVehicleDetachEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleDetachEvent>(self);
}
::alt::CVehicleDetachEvent const *cxxbridge1$shared_ptr$alt$CVehicleDetachEvent$get(::std::shared_ptr<::alt::CVehicleDetachEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CVehicleDetachEvent$drop(::std::shared_ptr<::alt::CVehicleDetachEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CVehicleDetachEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CVehicleDetachEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CVehicleDetachEvent$null(::std::weak_ptr<::alt::CVehicleDetachEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleDetachEvent>();
}
void cxxbridge1$weak_ptr$alt$CVehicleDetachEvent$clone(::std::weak_ptr<::alt::CVehicleDetachEvent> const &self, ::std::weak_ptr<::alt::CVehicleDetachEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleDetachEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CVehicleDetachEvent$downgrade(::std::shared_ptr<::alt::CVehicleDetachEvent> const &shared, ::std::weak_ptr<::alt::CVehicleDetachEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CVehicleDetachEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CVehicleDetachEvent$upgrade(::std::weak_ptr<::alt::CVehicleDetachEvent> const &weak, ::std::shared_ptr<::alt::CVehicleDetachEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CVehicleDetachEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CVehicleDetachEvent$drop(::std::weak_ptr<::alt::CVehicleDetachEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CVehicleDestroyEvent>::value, "definition of CVehicleDestroyEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CVehicleDestroyEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CVehicleDestroyEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CVehicleDestroyEvent$null(::std::unique_ptr<::alt::CVehicleDestroyEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleDestroyEvent>();
}
::alt::CVehicleDestroyEvent *cxxbridge1$unique_ptr$alt$CVehicleDestroyEvent$uninit(::std::unique_ptr<::alt::CVehicleDestroyEvent> *ptr) noexcept {
  ::alt::CVehicleDestroyEvent *uninit = reinterpret_cast<::alt::CVehicleDestroyEvent *>(new ::rust::MaybeUninit<::alt::CVehicleDestroyEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleDestroyEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CVehicleDestroyEvent$raw(::std::unique_ptr<::alt::CVehicleDestroyEvent> *ptr, ::alt::CVehicleDestroyEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleDestroyEvent>(raw);
}
::alt::CVehicleDestroyEvent const *cxxbridge1$unique_ptr$alt$CVehicleDestroyEvent$get(::std::unique_ptr<::alt::CVehicleDestroyEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CVehicleDestroyEvent *cxxbridge1$unique_ptr$alt$CVehicleDestroyEvent$release(::std::unique_ptr<::alt::CVehicleDestroyEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CVehicleDestroyEvent$drop(::std::unique_ptr<::alt::CVehicleDestroyEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CVehicleDestroyEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CVehicleDestroyEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CVehicleDestroyEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CVehicleDestroyEvent$null(::std::shared_ptr<::alt::CVehicleDestroyEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleDestroyEvent>();
}
::alt::CVehicleDestroyEvent *cxxbridge1$shared_ptr$alt$CVehicleDestroyEvent$uninit(::std::shared_ptr<::alt::CVehicleDestroyEvent> *ptr) noexcept {
  ::alt::CVehicleDestroyEvent *uninit = reinterpret_cast<::alt::CVehicleDestroyEvent *>(new ::rust::MaybeUninit<::alt::CVehicleDestroyEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleDestroyEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CVehicleDestroyEvent$clone(::std::shared_ptr<::alt::CVehicleDestroyEvent> const &self, ::std::shared_ptr<::alt::CVehicleDestroyEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleDestroyEvent>(self);
}
::alt::CVehicleDestroyEvent const *cxxbridge1$shared_ptr$alt$CVehicleDestroyEvent$get(::std::shared_ptr<::alt::CVehicleDestroyEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CVehicleDestroyEvent$drop(::std::shared_ptr<::alt::CVehicleDestroyEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CVehicleDestroyEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CVehicleDestroyEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CVehicleDestroyEvent$null(::std::weak_ptr<::alt::CVehicleDestroyEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleDestroyEvent>();
}
void cxxbridge1$weak_ptr$alt$CVehicleDestroyEvent$clone(::std::weak_ptr<::alt::CVehicleDestroyEvent> const &self, ::std::weak_ptr<::alt::CVehicleDestroyEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleDestroyEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CVehicleDestroyEvent$downgrade(::std::shared_ptr<::alt::CVehicleDestroyEvent> const &shared, ::std::weak_ptr<::alt::CVehicleDestroyEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CVehicleDestroyEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CVehicleDestroyEvent$upgrade(::std::weak_ptr<::alt::CVehicleDestroyEvent> const &weak, ::std::shared_ptr<::alt::CVehicleDestroyEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CVehicleDestroyEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CVehicleDestroyEvent$drop(::std::weak_ptr<::alt::CVehicleDestroyEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CVehicleDamageEvent>::value, "definition of CVehicleDamageEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CVehicleDamageEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CVehicleDamageEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CVehicleDamageEvent$null(::std::unique_ptr<::alt::CVehicleDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleDamageEvent>();
}
::alt::CVehicleDamageEvent *cxxbridge1$unique_ptr$alt$CVehicleDamageEvent$uninit(::std::unique_ptr<::alt::CVehicleDamageEvent> *ptr) noexcept {
  ::alt::CVehicleDamageEvent *uninit = reinterpret_cast<::alt::CVehicleDamageEvent *>(new ::rust::MaybeUninit<::alt::CVehicleDamageEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleDamageEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CVehicleDamageEvent$raw(::std::unique_ptr<::alt::CVehicleDamageEvent> *ptr, ::alt::CVehicleDamageEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleDamageEvent>(raw);
}
::alt::CVehicleDamageEvent const *cxxbridge1$unique_ptr$alt$CVehicleDamageEvent$get(::std::unique_ptr<::alt::CVehicleDamageEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CVehicleDamageEvent *cxxbridge1$unique_ptr$alt$CVehicleDamageEvent$release(::std::unique_ptr<::alt::CVehicleDamageEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CVehicleDamageEvent$drop(::std::unique_ptr<::alt::CVehicleDamageEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CVehicleDamageEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CVehicleDamageEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CVehicleDamageEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CVehicleDamageEvent$null(::std::shared_ptr<::alt::CVehicleDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleDamageEvent>();
}
::alt::CVehicleDamageEvent *cxxbridge1$shared_ptr$alt$CVehicleDamageEvent$uninit(::std::shared_ptr<::alt::CVehicleDamageEvent> *ptr) noexcept {
  ::alt::CVehicleDamageEvent *uninit = reinterpret_cast<::alt::CVehicleDamageEvent *>(new ::rust::MaybeUninit<::alt::CVehicleDamageEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleDamageEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CVehicleDamageEvent$clone(::std::shared_ptr<::alt::CVehicleDamageEvent> const &self, ::std::shared_ptr<::alt::CVehicleDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleDamageEvent>(self);
}
::alt::CVehicleDamageEvent const *cxxbridge1$shared_ptr$alt$CVehicleDamageEvent$get(::std::shared_ptr<::alt::CVehicleDamageEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CVehicleDamageEvent$drop(::std::shared_ptr<::alt::CVehicleDamageEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CVehicleDamageEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CVehicleDamageEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CVehicleDamageEvent$null(::std::weak_ptr<::alt::CVehicleDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleDamageEvent>();
}
void cxxbridge1$weak_ptr$alt$CVehicleDamageEvent$clone(::std::weak_ptr<::alt::CVehicleDamageEvent> const &self, ::std::weak_ptr<::alt::CVehicleDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleDamageEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CVehicleDamageEvent$downgrade(::std::shared_ptr<::alt::CVehicleDamageEvent> const &shared, ::std::weak_ptr<::alt::CVehicleDamageEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CVehicleDamageEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CVehicleDamageEvent$upgrade(::std::weak_ptr<::alt::CVehicleDamageEvent> const &weak, ::std::shared_ptr<::alt::CVehicleDamageEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CVehicleDamageEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CVehicleDamageEvent$drop(::std::weak_ptr<::alt::CVehicleDamageEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CVehicleHornEvent>::value, "definition of CVehicleHornEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CVehicleHornEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CVehicleHornEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CVehicleHornEvent$null(::std::unique_ptr<::alt::CVehicleHornEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleHornEvent>();
}
::alt::CVehicleHornEvent *cxxbridge1$unique_ptr$alt$CVehicleHornEvent$uninit(::std::unique_ptr<::alt::CVehicleHornEvent> *ptr) noexcept {
  ::alt::CVehicleHornEvent *uninit = reinterpret_cast<::alt::CVehicleHornEvent *>(new ::rust::MaybeUninit<::alt::CVehicleHornEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleHornEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CVehicleHornEvent$raw(::std::unique_ptr<::alt::CVehicleHornEvent> *ptr, ::alt::CVehicleHornEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleHornEvent>(raw);
}
::alt::CVehicleHornEvent const *cxxbridge1$unique_ptr$alt$CVehicleHornEvent$get(::std::unique_ptr<::alt::CVehicleHornEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CVehicleHornEvent *cxxbridge1$unique_ptr$alt$CVehicleHornEvent$release(::std::unique_ptr<::alt::CVehicleHornEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CVehicleHornEvent$drop(::std::unique_ptr<::alt::CVehicleHornEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CVehicleHornEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CVehicleHornEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CVehicleHornEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CVehicleHornEvent$null(::std::shared_ptr<::alt::CVehicleHornEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleHornEvent>();
}
::alt::CVehicleHornEvent *cxxbridge1$shared_ptr$alt$CVehicleHornEvent$uninit(::std::shared_ptr<::alt::CVehicleHornEvent> *ptr) noexcept {
  ::alt::CVehicleHornEvent *uninit = reinterpret_cast<::alt::CVehicleHornEvent *>(new ::rust::MaybeUninit<::alt::CVehicleHornEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleHornEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CVehicleHornEvent$clone(::std::shared_ptr<::alt::CVehicleHornEvent> const &self, ::std::shared_ptr<::alt::CVehicleHornEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleHornEvent>(self);
}
::alt::CVehicleHornEvent const *cxxbridge1$shared_ptr$alt$CVehicleHornEvent$get(::std::shared_ptr<::alt::CVehicleHornEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CVehicleHornEvent$drop(::std::shared_ptr<::alt::CVehicleHornEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CVehicleHornEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CVehicleHornEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CVehicleHornEvent$null(::std::weak_ptr<::alt::CVehicleHornEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleHornEvent>();
}
void cxxbridge1$weak_ptr$alt$CVehicleHornEvent$clone(::std::weak_ptr<::alt::CVehicleHornEvent> const &self, ::std::weak_ptr<::alt::CVehicleHornEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleHornEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CVehicleHornEvent$downgrade(::std::shared_ptr<::alt::CVehicleHornEvent> const &shared, ::std::weak_ptr<::alt::CVehicleHornEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CVehicleHornEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CVehicleHornEvent$upgrade(::std::weak_ptr<::alt::CVehicleHornEvent> const &weak, ::std::shared_ptr<::alt::CVehicleHornEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CVehicleHornEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CVehicleHornEvent$drop(::std::weak_ptr<::alt::CVehicleHornEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CVehicleSirenEvent>::value, "definition of CVehicleSirenEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CVehicleSirenEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CVehicleSirenEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CVehicleSirenEvent$null(::std::unique_ptr<::alt::CVehicleSirenEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleSirenEvent>();
}
::alt::CVehicleSirenEvent *cxxbridge1$unique_ptr$alt$CVehicleSirenEvent$uninit(::std::unique_ptr<::alt::CVehicleSirenEvent> *ptr) noexcept {
  ::alt::CVehicleSirenEvent *uninit = reinterpret_cast<::alt::CVehicleSirenEvent *>(new ::rust::MaybeUninit<::alt::CVehicleSirenEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleSirenEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CVehicleSirenEvent$raw(::std::unique_ptr<::alt::CVehicleSirenEvent> *ptr, ::alt::CVehicleSirenEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVehicleSirenEvent>(raw);
}
::alt::CVehicleSirenEvent const *cxxbridge1$unique_ptr$alt$CVehicleSirenEvent$get(::std::unique_ptr<::alt::CVehicleSirenEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CVehicleSirenEvent *cxxbridge1$unique_ptr$alt$CVehicleSirenEvent$release(::std::unique_ptr<::alt::CVehicleSirenEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CVehicleSirenEvent$drop(::std::unique_ptr<::alt::CVehicleSirenEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CVehicleSirenEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CVehicleSirenEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CVehicleSirenEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CVehicleSirenEvent$null(::std::shared_ptr<::alt::CVehicleSirenEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleSirenEvent>();
}
::alt::CVehicleSirenEvent *cxxbridge1$shared_ptr$alt$CVehicleSirenEvent$uninit(::std::shared_ptr<::alt::CVehicleSirenEvent> *ptr) noexcept {
  ::alt::CVehicleSirenEvent *uninit = reinterpret_cast<::alt::CVehicleSirenEvent *>(new ::rust::MaybeUninit<::alt::CVehicleSirenEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleSirenEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CVehicleSirenEvent$clone(::std::shared_ptr<::alt::CVehicleSirenEvent> const &self, ::std::shared_ptr<::alt::CVehicleSirenEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVehicleSirenEvent>(self);
}
::alt::CVehicleSirenEvent const *cxxbridge1$shared_ptr$alt$CVehicleSirenEvent$get(::std::shared_ptr<::alt::CVehicleSirenEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CVehicleSirenEvent$drop(::std::shared_ptr<::alt::CVehicleSirenEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CVehicleSirenEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CVehicleSirenEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CVehicleSirenEvent$null(::std::weak_ptr<::alt::CVehicleSirenEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleSirenEvent>();
}
void cxxbridge1$weak_ptr$alt$CVehicleSirenEvent$clone(::std::weak_ptr<::alt::CVehicleSirenEvent> const &self, ::std::weak_ptr<::alt::CVehicleSirenEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVehicleSirenEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CVehicleSirenEvent$downgrade(::std::shared_ptr<::alt::CVehicleSirenEvent> const &shared, ::std::weak_ptr<::alt::CVehicleSirenEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CVehicleSirenEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CVehicleSirenEvent$upgrade(::std::weak_ptr<::alt::CVehicleSirenEvent> const &weak, ::std::shared_ptr<::alt::CVehicleSirenEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CVehicleSirenEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CVehicleSirenEvent$drop(::std::weak_ptr<::alt::CVehicleSirenEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CNetOwnerChangeEvent>::value, "definition of CNetOwnerChangeEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CNetOwnerChangeEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CNetOwnerChangeEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CNetOwnerChangeEvent$null(::std::unique_ptr<::alt::CNetOwnerChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CNetOwnerChangeEvent>();
}
::alt::CNetOwnerChangeEvent *cxxbridge1$unique_ptr$alt$CNetOwnerChangeEvent$uninit(::std::unique_ptr<::alt::CNetOwnerChangeEvent> *ptr) noexcept {
  ::alt::CNetOwnerChangeEvent *uninit = reinterpret_cast<::alt::CNetOwnerChangeEvent *>(new ::rust::MaybeUninit<::alt::CNetOwnerChangeEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CNetOwnerChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CNetOwnerChangeEvent$raw(::std::unique_ptr<::alt::CNetOwnerChangeEvent> *ptr, ::alt::CNetOwnerChangeEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CNetOwnerChangeEvent>(raw);
}
::alt::CNetOwnerChangeEvent const *cxxbridge1$unique_ptr$alt$CNetOwnerChangeEvent$get(::std::unique_ptr<::alt::CNetOwnerChangeEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CNetOwnerChangeEvent *cxxbridge1$unique_ptr$alt$CNetOwnerChangeEvent$release(::std::unique_ptr<::alt::CNetOwnerChangeEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CNetOwnerChangeEvent$drop(::std::unique_ptr<::alt::CNetOwnerChangeEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CNetOwnerChangeEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CNetOwnerChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CNetOwnerChangeEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CNetOwnerChangeEvent$null(::std::shared_ptr<::alt::CNetOwnerChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CNetOwnerChangeEvent>();
}
::alt::CNetOwnerChangeEvent *cxxbridge1$shared_ptr$alt$CNetOwnerChangeEvent$uninit(::std::shared_ptr<::alt::CNetOwnerChangeEvent> *ptr) noexcept {
  ::alt::CNetOwnerChangeEvent *uninit = reinterpret_cast<::alt::CNetOwnerChangeEvent *>(new ::rust::MaybeUninit<::alt::CNetOwnerChangeEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CNetOwnerChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CNetOwnerChangeEvent$clone(::std::shared_ptr<::alt::CNetOwnerChangeEvent> const &self, ::std::shared_ptr<::alt::CNetOwnerChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CNetOwnerChangeEvent>(self);
}
::alt::CNetOwnerChangeEvent const *cxxbridge1$shared_ptr$alt$CNetOwnerChangeEvent$get(::std::shared_ptr<::alt::CNetOwnerChangeEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CNetOwnerChangeEvent$drop(::std::shared_ptr<::alt::CNetOwnerChangeEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CNetOwnerChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CNetOwnerChangeEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CNetOwnerChangeEvent$null(::std::weak_ptr<::alt::CNetOwnerChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CNetOwnerChangeEvent>();
}
void cxxbridge1$weak_ptr$alt$CNetOwnerChangeEvent$clone(::std::weak_ptr<::alt::CNetOwnerChangeEvent> const &self, ::std::weak_ptr<::alt::CNetOwnerChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CNetOwnerChangeEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CNetOwnerChangeEvent$downgrade(::std::shared_ptr<::alt::CNetOwnerChangeEvent> const &shared, ::std::weak_ptr<::alt::CNetOwnerChangeEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CNetOwnerChangeEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CNetOwnerChangeEvent$upgrade(::std::weak_ptr<::alt::CNetOwnerChangeEvent> const &weak, ::std::shared_ptr<::alt::CNetOwnerChangeEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CNetOwnerChangeEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CNetOwnerChangeEvent$drop(::std::weak_ptr<::alt::CNetOwnerChangeEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CMetaChangeEvent>::value, "definition of CMetaChangeEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CMetaChangeEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CMetaChangeEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CMetaChangeEvent$null(::std::unique_ptr<::alt::CMetaChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CMetaChangeEvent>();
}
::alt::CMetaChangeEvent *cxxbridge1$unique_ptr$alt$CMetaChangeEvent$uninit(::std::unique_ptr<::alt::CMetaChangeEvent> *ptr) noexcept {
  ::alt::CMetaChangeEvent *uninit = reinterpret_cast<::alt::CMetaChangeEvent *>(new ::rust::MaybeUninit<::alt::CMetaChangeEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CMetaChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CMetaChangeEvent$raw(::std::unique_ptr<::alt::CMetaChangeEvent> *ptr, ::alt::CMetaChangeEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CMetaChangeEvent>(raw);
}
::alt::CMetaChangeEvent const *cxxbridge1$unique_ptr$alt$CMetaChangeEvent$get(::std::unique_ptr<::alt::CMetaChangeEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CMetaChangeEvent *cxxbridge1$unique_ptr$alt$CMetaChangeEvent$release(::std::unique_ptr<::alt::CMetaChangeEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CMetaChangeEvent$drop(::std::unique_ptr<::alt::CMetaChangeEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CMetaChangeEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CMetaChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CMetaChangeEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CMetaChangeEvent$null(::std::shared_ptr<::alt::CMetaChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CMetaChangeEvent>();
}
::alt::CMetaChangeEvent *cxxbridge1$shared_ptr$alt$CMetaChangeEvent$uninit(::std::shared_ptr<::alt::CMetaChangeEvent> *ptr) noexcept {
  ::alt::CMetaChangeEvent *uninit = reinterpret_cast<::alt::CMetaChangeEvent *>(new ::rust::MaybeUninit<::alt::CMetaChangeEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CMetaChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CMetaChangeEvent$clone(::std::shared_ptr<::alt::CMetaChangeEvent> const &self, ::std::shared_ptr<::alt::CMetaChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CMetaChangeEvent>(self);
}
::alt::CMetaChangeEvent const *cxxbridge1$shared_ptr$alt$CMetaChangeEvent$get(::std::shared_ptr<::alt::CMetaChangeEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CMetaChangeEvent$drop(::std::shared_ptr<::alt::CMetaChangeEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CMetaChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CMetaChangeEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CMetaChangeEvent$null(::std::weak_ptr<::alt::CMetaChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CMetaChangeEvent>();
}
void cxxbridge1$weak_ptr$alt$CMetaChangeEvent$clone(::std::weak_ptr<::alt::CMetaChangeEvent> const &self, ::std::weak_ptr<::alt::CMetaChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CMetaChangeEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CMetaChangeEvent$downgrade(::std::shared_ptr<::alt::CMetaChangeEvent> const &shared, ::std::weak_ptr<::alt::CMetaChangeEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CMetaChangeEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CMetaChangeEvent$upgrade(::std::weak_ptr<::alt::CMetaChangeEvent> const &weak, ::std::shared_ptr<::alt::CMetaChangeEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CMetaChangeEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CMetaChangeEvent$drop(::std::weak_ptr<::alt::CMetaChangeEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CGlobalMetaDataChangeEvent>::value, "definition of CGlobalMetaDataChangeEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CGlobalMetaDataChangeEvent$null(::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent>();
}
::alt::CGlobalMetaDataChangeEvent *cxxbridge1$unique_ptr$alt$CGlobalMetaDataChangeEvent$uninit(::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent> *ptr) noexcept {
  ::alt::CGlobalMetaDataChangeEvent *uninit = reinterpret_cast<::alt::CGlobalMetaDataChangeEvent *>(new ::rust::MaybeUninit<::alt::CGlobalMetaDataChangeEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CGlobalMetaDataChangeEvent$raw(::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent> *ptr, ::alt::CGlobalMetaDataChangeEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent>(raw);
}
::alt::CGlobalMetaDataChangeEvent const *cxxbridge1$unique_ptr$alt$CGlobalMetaDataChangeEvent$get(::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CGlobalMetaDataChangeEvent *cxxbridge1$unique_ptr$alt$CGlobalMetaDataChangeEvent$release(::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CGlobalMetaDataChangeEvent$drop(::std::unique_ptr<::alt::CGlobalMetaDataChangeEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CGlobalMetaDataChangeEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CGlobalMetaDataChangeEvent$null(::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent>();
}
::alt::CGlobalMetaDataChangeEvent *cxxbridge1$shared_ptr$alt$CGlobalMetaDataChangeEvent$uninit(::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent> *ptr) noexcept {
  ::alt::CGlobalMetaDataChangeEvent *uninit = reinterpret_cast<::alt::CGlobalMetaDataChangeEvent *>(new ::rust::MaybeUninit<::alt::CGlobalMetaDataChangeEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CGlobalMetaDataChangeEvent$clone(::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent> const &self, ::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent>(self);
}
::alt::CGlobalMetaDataChangeEvent const *cxxbridge1$shared_ptr$alt$CGlobalMetaDataChangeEvent$get(::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CGlobalMetaDataChangeEvent$drop(::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CGlobalMetaDataChangeEvent$null(::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent>();
}
void cxxbridge1$weak_ptr$alt$CGlobalMetaDataChangeEvent$clone(::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent> const &self, ::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CGlobalMetaDataChangeEvent$downgrade(::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent> const &shared, ::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CGlobalMetaDataChangeEvent$upgrade(::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent> const &weak, ::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CGlobalMetaDataChangeEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CGlobalMetaDataChangeEvent$drop(::std::weak_ptr<::alt::CGlobalMetaDataChangeEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CGlobalSyncedMetaDataChangeEvent>::value, "definition of CGlobalSyncedMetaDataChangeEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CGlobalSyncedMetaDataChangeEvent$null(::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>();
}
::alt::CGlobalSyncedMetaDataChangeEvent *cxxbridge1$unique_ptr$alt$CGlobalSyncedMetaDataChangeEvent$uninit(::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::alt::CGlobalSyncedMetaDataChangeEvent *uninit = reinterpret_cast<::alt::CGlobalSyncedMetaDataChangeEvent *>(new ::rust::MaybeUninit<::alt::CGlobalSyncedMetaDataChangeEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CGlobalSyncedMetaDataChangeEvent$raw(::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *ptr, ::alt::CGlobalSyncedMetaDataChangeEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>(raw);
}
::alt::CGlobalSyncedMetaDataChangeEvent const *cxxbridge1$unique_ptr$alt$CGlobalSyncedMetaDataChangeEvent$get(::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CGlobalSyncedMetaDataChangeEvent *cxxbridge1$unique_ptr$alt$CGlobalSyncedMetaDataChangeEvent$release(::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CGlobalSyncedMetaDataChangeEvent$drop(::std::unique_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CGlobalSyncedMetaDataChangeEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CGlobalSyncedMetaDataChangeEvent$null(::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>();
}
::alt::CGlobalSyncedMetaDataChangeEvent *cxxbridge1$shared_ptr$alt$CGlobalSyncedMetaDataChangeEvent$uninit(::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::alt::CGlobalSyncedMetaDataChangeEvent *uninit = reinterpret_cast<::alt::CGlobalSyncedMetaDataChangeEvent *>(new ::rust::MaybeUninit<::alt::CGlobalSyncedMetaDataChangeEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CGlobalSyncedMetaDataChangeEvent$clone(::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> const &self, ::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>(self);
}
::alt::CGlobalSyncedMetaDataChangeEvent const *cxxbridge1$shared_ptr$alt$CGlobalSyncedMetaDataChangeEvent$get(::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CGlobalSyncedMetaDataChangeEvent$drop(::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CGlobalSyncedMetaDataChangeEvent$null(::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>();
}
void cxxbridge1$weak_ptr$alt$CGlobalSyncedMetaDataChangeEvent$clone(::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> const &self, ::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CGlobalSyncedMetaDataChangeEvent$downgrade(::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> const &shared, ::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CGlobalSyncedMetaDataChangeEvent$upgrade(::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> const &weak, ::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CGlobalSyncedMetaDataChangeEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CGlobalSyncedMetaDataChangeEvent$drop(::std::weak_ptr<::alt::CGlobalSyncedMetaDataChangeEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CSyncedMetaDataChangeEvent>::value, "definition of CSyncedMetaDataChangeEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CSyncedMetaDataChangeEvent$null(::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent>();
}
::alt::CSyncedMetaDataChangeEvent *cxxbridge1$unique_ptr$alt$CSyncedMetaDataChangeEvent$uninit(::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::alt::CSyncedMetaDataChangeEvent *uninit = reinterpret_cast<::alt::CSyncedMetaDataChangeEvent *>(new ::rust::MaybeUninit<::alt::CSyncedMetaDataChangeEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CSyncedMetaDataChangeEvent$raw(::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent> *ptr, ::alt::CSyncedMetaDataChangeEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent>(raw);
}
::alt::CSyncedMetaDataChangeEvent const *cxxbridge1$unique_ptr$alt$CSyncedMetaDataChangeEvent$get(::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CSyncedMetaDataChangeEvent *cxxbridge1$unique_ptr$alt$CSyncedMetaDataChangeEvent$release(::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CSyncedMetaDataChangeEvent$drop(::std::unique_ptr<::alt::CSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CSyncedMetaDataChangeEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CSyncedMetaDataChangeEvent$null(::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent>();
}
::alt::CSyncedMetaDataChangeEvent *cxxbridge1$shared_ptr$alt$CSyncedMetaDataChangeEvent$uninit(::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::alt::CSyncedMetaDataChangeEvent *uninit = reinterpret_cast<::alt::CSyncedMetaDataChangeEvent *>(new ::rust::MaybeUninit<::alt::CSyncedMetaDataChangeEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CSyncedMetaDataChangeEvent$clone(::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent> const &self, ::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent>(self);
}
::alt::CSyncedMetaDataChangeEvent const *cxxbridge1$shared_ptr$alt$CSyncedMetaDataChangeEvent$get(::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CSyncedMetaDataChangeEvent$drop(::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CSyncedMetaDataChangeEvent$null(::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent>();
}
void cxxbridge1$weak_ptr$alt$CSyncedMetaDataChangeEvent$clone(::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent> const &self, ::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CSyncedMetaDataChangeEvent$downgrade(::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent> const &shared, ::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CSyncedMetaDataChangeEvent$upgrade(::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent> const &weak, ::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CSyncedMetaDataChangeEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CSyncedMetaDataChangeEvent$drop(::std::weak_ptr<::alt::CSyncedMetaDataChangeEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CStreamSyncedMetaDataChangeEvent>::value, "definition of CStreamSyncedMetaDataChangeEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CStreamSyncedMetaDataChangeEvent$null(::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent>();
}
::alt::CStreamSyncedMetaDataChangeEvent *cxxbridge1$unique_ptr$alt$CStreamSyncedMetaDataChangeEvent$uninit(::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::alt::CStreamSyncedMetaDataChangeEvent *uninit = reinterpret_cast<::alt::CStreamSyncedMetaDataChangeEvent *>(new ::rust::MaybeUninit<::alt::CStreamSyncedMetaDataChangeEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CStreamSyncedMetaDataChangeEvent$raw(::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *ptr, ::alt::CStreamSyncedMetaDataChangeEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent>(raw);
}
::alt::CStreamSyncedMetaDataChangeEvent const *cxxbridge1$unique_ptr$alt$CStreamSyncedMetaDataChangeEvent$get(::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CStreamSyncedMetaDataChangeEvent *cxxbridge1$unique_ptr$alt$CStreamSyncedMetaDataChangeEvent$release(::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CStreamSyncedMetaDataChangeEvent$drop(::std::unique_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CStreamSyncedMetaDataChangeEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CStreamSyncedMetaDataChangeEvent$null(::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent>();
}
::alt::CStreamSyncedMetaDataChangeEvent *cxxbridge1$shared_ptr$alt$CStreamSyncedMetaDataChangeEvent$uninit(::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::alt::CStreamSyncedMetaDataChangeEvent *uninit = reinterpret_cast<::alt::CStreamSyncedMetaDataChangeEvent *>(new ::rust::MaybeUninit<::alt::CStreamSyncedMetaDataChangeEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CStreamSyncedMetaDataChangeEvent$clone(::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent> const &self, ::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent>(self);
}
::alt::CStreamSyncedMetaDataChangeEvent const *cxxbridge1$shared_ptr$alt$CStreamSyncedMetaDataChangeEvent$get(::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CStreamSyncedMetaDataChangeEvent$drop(::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CStreamSyncedMetaDataChangeEvent$null(::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent>();
}
void cxxbridge1$weak_ptr$alt$CStreamSyncedMetaDataChangeEvent$clone(::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent> const &self, ::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CStreamSyncedMetaDataChangeEvent$downgrade(::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent> const &shared, ::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CStreamSyncedMetaDataChangeEvent$upgrade(::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent> const &weak, ::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CStreamSyncedMetaDataChangeEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CStreamSyncedMetaDataChangeEvent$drop(::std::weak_ptr<::alt::CStreamSyncedMetaDataChangeEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CLocalMetaDataChangeEvent>::value, "definition of CLocalMetaDataChangeEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CLocalMetaDataChangeEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CLocalMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CLocalMetaDataChangeEvent$null(::std::unique_ptr<::alt::CLocalMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CLocalMetaDataChangeEvent>();
}
::alt::CLocalMetaDataChangeEvent *cxxbridge1$unique_ptr$alt$CLocalMetaDataChangeEvent$uninit(::std::unique_ptr<::alt::CLocalMetaDataChangeEvent> *ptr) noexcept {
  ::alt::CLocalMetaDataChangeEvent *uninit = reinterpret_cast<::alt::CLocalMetaDataChangeEvent *>(new ::rust::MaybeUninit<::alt::CLocalMetaDataChangeEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CLocalMetaDataChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CLocalMetaDataChangeEvent$raw(::std::unique_ptr<::alt::CLocalMetaDataChangeEvent> *ptr, ::alt::CLocalMetaDataChangeEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CLocalMetaDataChangeEvent>(raw);
}
::alt::CLocalMetaDataChangeEvent const *cxxbridge1$unique_ptr$alt$CLocalMetaDataChangeEvent$get(::std::unique_ptr<::alt::CLocalMetaDataChangeEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CLocalMetaDataChangeEvent *cxxbridge1$unique_ptr$alt$CLocalMetaDataChangeEvent$release(::std::unique_ptr<::alt::CLocalMetaDataChangeEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CLocalMetaDataChangeEvent$drop(::std::unique_ptr<::alt::CLocalMetaDataChangeEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CLocalMetaDataChangeEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CLocalMetaDataChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CLocalMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CLocalMetaDataChangeEvent$null(::std::shared_ptr<::alt::CLocalMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CLocalMetaDataChangeEvent>();
}
::alt::CLocalMetaDataChangeEvent *cxxbridge1$shared_ptr$alt$CLocalMetaDataChangeEvent$uninit(::std::shared_ptr<::alt::CLocalMetaDataChangeEvent> *ptr) noexcept {
  ::alt::CLocalMetaDataChangeEvent *uninit = reinterpret_cast<::alt::CLocalMetaDataChangeEvent *>(new ::rust::MaybeUninit<::alt::CLocalMetaDataChangeEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CLocalMetaDataChangeEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CLocalMetaDataChangeEvent$clone(::std::shared_ptr<::alt::CLocalMetaDataChangeEvent> const &self, ::std::shared_ptr<::alt::CLocalMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CLocalMetaDataChangeEvent>(self);
}
::alt::CLocalMetaDataChangeEvent const *cxxbridge1$shared_ptr$alt$CLocalMetaDataChangeEvent$get(::std::shared_ptr<::alt::CLocalMetaDataChangeEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CLocalMetaDataChangeEvent$drop(::std::shared_ptr<::alt::CLocalMetaDataChangeEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CLocalMetaDataChangeEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CLocalMetaDataChangeEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CLocalMetaDataChangeEvent$null(::std::weak_ptr<::alt::CLocalMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CLocalMetaDataChangeEvent>();
}
void cxxbridge1$weak_ptr$alt$CLocalMetaDataChangeEvent$clone(::std::weak_ptr<::alt::CLocalMetaDataChangeEvent> const &self, ::std::weak_ptr<::alt::CLocalMetaDataChangeEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CLocalMetaDataChangeEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CLocalMetaDataChangeEvent$downgrade(::std::shared_ptr<::alt::CLocalMetaDataChangeEvent> const &shared, ::std::weak_ptr<::alt::CLocalMetaDataChangeEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CLocalMetaDataChangeEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CLocalMetaDataChangeEvent$upgrade(::std::weak_ptr<::alt::CLocalMetaDataChangeEvent> const &weak, ::std::shared_ptr<::alt::CLocalMetaDataChangeEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CLocalMetaDataChangeEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CLocalMetaDataChangeEvent$drop(::std::weak_ptr<::alt::CLocalMetaDataChangeEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CResourceStopEvent>::value, "definition of CResourceStopEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CResourceStopEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CResourceStopEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CResourceStopEvent$null(::std::unique_ptr<::alt::CResourceStopEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CResourceStopEvent>();
}
::alt::CResourceStopEvent *cxxbridge1$unique_ptr$alt$CResourceStopEvent$uninit(::std::unique_ptr<::alt::CResourceStopEvent> *ptr) noexcept {
  ::alt::CResourceStopEvent *uninit = reinterpret_cast<::alt::CResourceStopEvent *>(new ::rust::MaybeUninit<::alt::CResourceStopEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CResourceStopEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CResourceStopEvent$raw(::std::unique_ptr<::alt::CResourceStopEvent> *ptr, ::alt::CResourceStopEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CResourceStopEvent>(raw);
}
::alt::CResourceStopEvent const *cxxbridge1$unique_ptr$alt$CResourceStopEvent$get(::std::unique_ptr<::alt::CResourceStopEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CResourceStopEvent *cxxbridge1$unique_ptr$alt$CResourceStopEvent$release(::std::unique_ptr<::alt::CResourceStopEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CResourceStopEvent$drop(::std::unique_ptr<::alt::CResourceStopEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CResourceStopEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CResourceStopEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CResourceStopEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CResourceStopEvent$null(::std::shared_ptr<::alt::CResourceStopEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CResourceStopEvent>();
}
::alt::CResourceStopEvent *cxxbridge1$shared_ptr$alt$CResourceStopEvent$uninit(::std::shared_ptr<::alt::CResourceStopEvent> *ptr) noexcept {
  ::alt::CResourceStopEvent *uninit = reinterpret_cast<::alt::CResourceStopEvent *>(new ::rust::MaybeUninit<::alt::CResourceStopEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CResourceStopEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CResourceStopEvent$clone(::std::shared_ptr<::alt::CResourceStopEvent> const &self, ::std::shared_ptr<::alt::CResourceStopEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CResourceStopEvent>(self);
}
::alt::CResourceStopEvent const *cxxbridge1$shared_ptr$alt$CResourceStopEvent$get(::std::shared_ptr<::alt::CResourceStopEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CResourceStopEvent$drop(::std::shared_ptr<::alt::CResourceStopEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CResourceStopEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CResourceStopEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CResourceStopEvent$null(::std::weak_ptr<::alt::CResourceStopEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CResourceStopEvent>();
}
void cxxbridge1$weak_ptr$alt$CResourceStopEvent$clone(::std::weak_ptr<::alt::CResourceStopEvent> const &self, ::std::weak_ptr<::alt::CResourceStopEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CResourceStopEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CResourceStopEvent$downgrade(::std::shared_ptr<::alt::CResourceStopEvent> const &shared, ::std::weak_ptr<::alt::CResourceStopEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CResourceStopEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CResourceStopEvent$upgrade(::std::weak_ptr<::alt::CResourceStopEvent> const &weak, ::std::shared_ptr<::alt::CResourceStopEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CResourceStopEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CResourceStopEvent$drop(::std::weak_ptr<::alt::CResourceStopEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CResourceStartEvent>::value, "definition of CResourceStartEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CResourceStartEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CResourceStartEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CResourceStartEvent$null(::std::unique_ptr<::alt::CResourceStartEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CResourceStartEvent>();
}
::alt::CResourceStartEvent *cxxbridge1$unique_ptr$alt$CResourceStartEvent$uninit(::std::unique_ptr<::alt::CResourceStartEvent> *ptr) noexcept {
  ::alt::CResourceStartEvent *uninit = reinterpret_cast<::alt::CResourceStartEvent *>(new ::rust::MaybeUninit<::alt::CResourceStartEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CResourceStartEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CResourceStartEvent$raw(::std::unique_ptr<::alt::CResourceStartEvent> *ptr, ::alt::CResourceStartEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CResourceStartEvent>(raw);
}
::alt::CResourceStartEvent const *cxxbridge1$unique_ptr$alt$CResourceStartEvent$get(::std::unique_ptr<::alt::CResourceStartEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CResourceStartEvent *cxxbridge1$unique_ptr$alt$CResourceStartEvent$release(::std::unique_ptr<::alt::CResourceStartEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CResourceStartEvent$drop(::std::unique_ptr<::alt::CResourceStartEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CResourceStartEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CResourceStartEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CResourceStartEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CResourceStartEvent$null(::std::shared_ptr<::alt::CResourceStartEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CResourceStartEvent>();
}
::alt::CResourceStartEvent *cxxbridge1$shared_ptr$alt$CResourceStartEvent$uninit(::std::shared_ptr<::alt::CResourceStartEvent> *ptr) noexcept {
  ::alt::CResourceStartEvent *uninit = reinterpret_cast<::alt::CResourceStartEvent *>(new ::rust::MaybeUninit<::alt::CResourceStartEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CResourceStartEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CResourceStartEvent$clone(::std::shared_ptr<::alt::CResourceStartEvent> const &self, ::std::shared_ptr<::alt::CResourceStartEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CResourceStartEvent>(self);
}
::alt::CResourceStartEvent const *cxxbridge1$shared_ptr$alt$CResourceStartEvent$get(::std::shared_ptr<::alt::CResourceStartEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CResourceStartEvent$drop(::std::shared_ptr<::alt::CResourceStartEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CResourceStartEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CResourceStartEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CResourceStartEvent$null(::std::weak_ptr<::alt::CResourceStartEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CResourceStartEvent>();
}
void cxxbridge1$weak_ptr$alt$CResourceStartEvent$clone(::std::weak_ptr<::alt::CResourceStartEvent> const &self, ::std::weak_ptr<::alt::CResourceStartEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CResourceStartEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CResourceStartEvent$downgrade(::std::shared_ptr<::alt::CResourceStartEvent> const &shared, ::std::weak_ptr<::alt::CResourceStartEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CResourceStartEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CResourceStartEvent$upgrade(::std::weak_ptr<::alt::CResourceStartEvent> const &weak, ::std::shared_ptr<::alt::CResourceStartEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CResourceStartEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CResourceStartEvent$drop(::std::weak_ptr<::alt::CResourceStartEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CVoiceConnectionEvent>::value, "definition of CVoiceConnectionEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CVoiceConnectionEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CVoiceConnectionEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CVoiceConnectionEvent$null(::std::unique_ptr<::alt::CVoiceConnectionEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVoiceConnectionEvent>();
}
::alt::CVoiceConnectionEvent *cxxbridge1$unique_ptr$alt$CVoiceConnectionEvent$uninit(::std::unique_ptr<::alt::CVoiceConnectionEvent> *ptr) noexcept {
  ::alt::CVoiceConnectionEvent *uninit = reinterpret_cast<::alt::CVoiceConnectionEvent *>(new ::rust::MaybeUninit<::alt::CVoiceConnectionEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CVoiceConnectionEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CVoiceConnectionEvent$raw(::std::unique_ptr<::alt::CVoiceConnectionEvent> *ptr, ::alt::CVoiceConnectionEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CVoiceConnectionEvent>(raw);
}
::alt::CVoiceConnectionEvent const *cxxbridge1$unique_ptr$alt$CVoiceConnectionEvent$get(::std::unique_ptr<::alt::CVoiceConnectionEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CVoiceConnectionEvent *cxxbridge1$unique_ptr$alt$CVoiceConnectionEvent$release(::std::unique_ptr<::alt::CVoiceConnectionEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CVoiceConnectionEvent$drop(::std::unique_ptr<::alt::CVoiceConnectionEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CVoiceConnectionEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CVoiceConnectionEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CVoiceConnectionEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CVoiceConnectionEvent$null(::std::shared_ptr<::alt::CVoiceConnectionEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVoiceConnectionEvent>();
}
::alt::CVoiceConnectionEvent *cxxbridge1$shared_ptr$alt$CVoiceConnectionEvent$uninit(::std::shared_ptr<::alt::CVoiceConnectionEvent> *ptr) noexcept {
  ::alt::CVoiceConnectionEvent *uninit = reinterpret_cast<::alt::CVoiceConnectionEvent *>(new ::rust::MaybeUninit<::alt::CVoiceConnectionEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CVoiceConnectionEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CVoiceConnectionEvent$clone(::std::shared_ptr<::alt::CVoiceConnectionEvent> const &self, ::std::shared_ptr<::alt::CVoiceConnectionEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CVoiceConnectionEvent>(self);
}
::alt::CVoiceConnectionEvent const *cxxbridge1$shared_ptr$alt$CVoiceConnectionEvent$get(::std::shared_ptr<::alt::CVoiceConnectionEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CVoiceConnectionEvent$drop(::std::shared_ptr<::alt::CVoiceConnectionEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CVoiceConnectionEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CVoiceConnectionEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CVoiceConnectionEvent$null(::std::weak_ptr<::alt::CVoiceConnectionEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVoiceConnectionEvent>();
}
void cxxbridge1$weak_ptr$alt$CVoiceConnectionEvent$clone(::std::weak_ptr<::alt::CVoiceConnectionEvent> const &self, ::std::weak_ptr<::alt::CVoiceConnectionEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CVoiceConnectionEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CVoiceConnectionEvent$downgrade(::std::shared_ptr<::alt::CVoiceConnectionEvent> const &shared, ::std::weak_ptr<::alt::CVoiceConnectionEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CVoiceConnectionEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CVoiceConnectionEvent$upgrade(::std::weak_ptr<::alt::CVoiceConnectionEvent> const &weak, ::std::shared_ptr<::alt::CVoiceConnectionEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CVoiceConnectionEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CVoiceConnectionEvent$drop(::std::weak_ptr<::alt::CVoiceConnectionEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CRequestSyncedSceneEvent>::value, "definition of CRequestSyncedSceneEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CRequestSyncedSceneEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CRequestSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CRequestSyncedSceneEvent$null(::std::unique_ptr<::alt::CRequestSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CRequestSyncedSceneEvent>();
}
::alt::CRequestSyncedSceneEvent *cxxbridge1$unique_ptr$alt$CRequestSyncedSceneEvent$uninit(::std::unique_ptr<::alt::CRequestSyncedSceneEvent> *ptr) noexcept {
  ::alt::CRequestSyncedSceneEvent *uninit = reinterpret_cast<::alt::CRequestSyncedSceneEvent *>(new ::rust::MaybeUninit<::alt::CRequestSyncedSceneEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CRequestSyncedSceneEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CRequestSyncedSceneEvent$raw(::std::unique_ptr<::alt::CRequestSyncedSceneEvent> *ptr, ::alt::CRequestSyncedSceneEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CRequestSyncedSceneEvent>(raw);
}
::alt::CRequestSyncedSceneEvent const *cxxbridge1$unique_ptr$alt$CRequestSyncedSceneEvent$get(::std::unique_ptr<::alt::CRequestSyncedSceneEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CRequestSyncedSceneEvent *cxxbridge1$unique_ptr$alt$CRequestSyncedSceneEvent$release(::std::unique_ptr<::alt::CRequestSyncedSceneEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CRequestSyncedSceneEvent$drop(::std::unique_ptr<::alt::CRequestSyncedSceneEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CRequestSyncedSceneEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CRequestSyncedSceneEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CRequestSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CRequestSyncedSceneEvent$null(::std::shared_ptr<::alt::CRequestSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CRequestSyncedSceneEvent>();
}
::alt::CRequestSyncedSceneEvent *cxxbridge1$shared_ptr$alt$CRequestSyncedSceneEvent$uninit(::std::shared_ptr<::alt::CRequestSyncedSceneEvent> *ptr) noexcept {
  ::alt::CRequestSyncedSceneEvent *uninit = reinterpret_cast<::alt::CRequestSyncedSceneEvent *>(new ::rust::MaybeUninit<::alt::CRequestSyncedSceneEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CRequestSyncedSceneEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CRequestSyncedSceneEvent$clone(::std::shared_ptr<::alt::CRequestSyncedSceneEvent> const &self, ::std::shared_ptr<::alt::CRequestSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CRequestSyncedSceneEvent>(self);
}
::alt::CRequestSyncedSceneEvent const *cxxbridge1$shared_ptr$alt$CRequestSyncedSceneEvent$get(::std::shared_ptr<::alt::CRequestSyncedSceneEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CRequestSyncedSceneEvent$drop(::std::shared_ptr<::alt::CRequestSyncedSceneEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CRequestSyncedSceneEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CRequestSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CRequestSyncedSceneEvent$null(::std::weak_ptr<::alt::CRequestSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CRequestSyncedSceneEvent>();
}
void cxxbridge1$weak_ptr$alt$CRequestSyncedSceneEvent$clone(::std::weak_ptr<::alt::CRequestSyncedSceneEvent> const &self, ::std::weak_ptr<::alt::CRequestSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CRequestSyncedSceneEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CRequestSyncedSceneEvent$downgrade(::std::shared_ptr<::alt::CRequestSyncedSceneEvent> const &shared, ::std::weak_ptr<::alt::CRequestSyncedSceneEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CRequestSyncedSceneEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CRequestSyncedSceneEvent$upgrade(::std::weak_ptr<::alt::CRequestSyncedSceneEvent> const &weak, ::std::shared_ptr<::alt::CRequestSyncedSceneEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CRequestSyncedSceneEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CRequestSyncedSceneEvent$drop(::std::weak_ptr<::alt::CRequestSyncedSceneEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CStartSyncedSceneEvent>::value, "definition of CStartSyncedSceneEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CStartSyncedSceneEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CStartSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CStartSyncedSceneEvent$null(::std::unique_ptr<::alt::CStartSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CStartSyncedSceneEvent>();
}
::alt::CStartSyncedSceneEvent *cxxbridge1$unique_ptr$alt$CStartSyncedSceneEvent$uninit(::std::unique_ptr<::alt::CStartSyncedSceneEvent> *ptr) noexcept {
  ::alt::CStartSyncedSceneEvent *uninit = reinterpret_cast<::alt::CStartSyncedSceneEvent *>(new ::rust::MaybeUninit<::alt::CStartSyncedSceneEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CStartSyncedSceneEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CStartSyncedSceneEvent$raw(::std::unique_ptr<::alt::CStartSyncedSceneEvent> *ptr, ::alt::CStartSyncedSceneEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CStartSyncedSceneEvent>(raw);
}
::alt::CStartSyncedSceneEvent const *cxxbridge1$unique_ptr$alt$CStartSyncedSceneEvent$get(::std::unique_ptr<::alt::CStartSyncedSceneEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CStartSyncedSceneEvent *cxxbridge1$unique_ptr$alt$CStartSyncedSceneEvent$release(::std::unique_ptr<::alt::CStartSyncedSceneEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CStartSyncedSceneEvent$drop(::std::unique_ptr<::alt::CStartSyncedSceneEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CStartSyncedSceneEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CStartSyncedSceneEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CStartSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CStartSyncedSceneEvent$null(::std::shared_ptr<::alt::CStartSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CStartSyncedSceneEvent>();
}
::alt::CStartSyncedSceneEvent *cxxbridge1$shared_ptr$alt$CStartSyncedSceneEvent$uninit(::std::shared_ptr<::alt::CStartSyncedSceneEvent> *ptr) noexcept {
  ::alt::CStartSyncedSceneEvent *uninit = reinterpret_cast<::alt::CStartSyncedSceneEvent *>(new ::rust::MaybeUninit<::alt::CStartSyncedSceneEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CStartSyncedSceneEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CStartSyncedSceneEvent$clone(::std::shared_ptr<::alt::CStartSyncedSceneEvent> const &self, ::std::shared_ptr<::alt::CStartSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CStartSyncedSceneEvent>(self);
}
::alt::CStartSyncedSceneEvent const *cxxbridge1$shared_ptr$alt$CStartSyncedSceneEvent$get(::std::shared_ptr<::alt::CStartSyncedSceneEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CStartSyncedSceneEvent$drop(::std::shared_ptr<::alt::CStartSyncedSceneEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CStartSyncedSceneEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CStartSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CStartSyncedSceneEvent$null(::std::weak_ptr<::alt::CStartSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CStartSyncedSceneEvent>();
}
void cxxbridge1$weak_ptr$alt$CStartSyncedSceneEvent$clone(::std::weak_ptr<::alt::CStartSyncedSceneEvent> const &self, ::std::weak_ptr<::alt::CStartSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CStartSyncedSceneEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CStartSyncedSceneEvent$downgrade(::std::shared_ptr<::alt::CStartSyncedSceneEvent> const &shared, ::std::weak_ptr<::alt::CStartSyncedSceneEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CStartSyncedSceneEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CStartSyncedSceneEvent$upgrade(::std::weak_ptr<::alt::CStartSyncedSceneEvent> const &weak, ::std::shared_ptr<::alt::CStartSyncedSceneEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CStartSyncedSceneEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CStartSyncedSceneEvent$drop(::std::weak_ptr<::alt::CStartSyncedSceneEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CStopSyncedSceneEvent>::value, "definition of CStopSyncedSceneEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CStopSyncedSceneEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CStopSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CStopSyncedSceneEvent$null(::std::unique_ptr<::alt::CStopSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CStopSyncedSceneEvent>();
}
::alt::CStopSyncedSceneEvent *cxxbridge1$unique_ptr$alt$CStopSyncedSceneEvent$uninit(::std::unique_ptr<::alt::CStopSyncedSceneEvent> *ptr) noexcept {
  ::alt::CStopSyncedSceneEvent *uninit = reinterpret_cast<::alt::CStopSyncedSceneEvent *>(new ::rust::MaybeUninit<::alt::CStopSyncedSceneEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CStopSyncedSceneEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CStopSyncedSceneEvent$raw(::std::unique_ptr<::alt::CStopSyncedSceneEvent> *ptr, ::alt::CStopSyncedSceneEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CStopSyncedSceneEvent>(raw);
}
::alt::CStopSyncedSceneEvent const *cxxbridge1$unique_ptr$alt$CStopSyncedSceneEvent$get(::std::unique_ptr<::alt::CStopSyncedSceneEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CStopSyncedSceneEvent *cxxbridge1$unique_ptr$alt$CStopSyncedSceneEvent$release(::std::unique_ptr<::alt::CStopSyncedSceneEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CStopSyncedSceneEvent$drop(::std::unique_ptr<::alt::CStopSyncedSceneEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CStopSyncedSceneEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CStopSyncedSceneEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CStopSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CStopSyncedSceneEvent$null(::std::shared_ptr<::alt::CStopSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CStopSyncedSceneEvent>();
}
::alt::CStopSyncedSceneEvent *cxxbridge1$shared_ptr$alt$CStopSyncedSceneEvent$uninit(::std::shared_ptr<::alt::CStopSyncedSceneEvent> *ptr) noexcept {
  ::alt::CStopSyncedSceneEvent *uninit = reinterpret_cast<::alt::CStopSyncedSceneEvent *>(new ::rust::MaybeUninit<::alt::CStopSyncedSceneEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CStopSyncedSceneEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CStopSyncedSceneEvent$clone(::std::shared_ptr<::alt::CStopSyncedSceneEvent> const &self, ::std::shared_ptr<::alt::CStopSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CStopSyncedSceneEvent>(self);
}
::alt::CStopSyncedSceneEvent const *cxxbridge1$shared_ptr$alt$CStopSyncedSceneEvent$get(::std::shared_ptr<::alt::CStopSyncedSceneEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CStopSyncedSceneEvent$drop(::std::shared_ptr<::alt::CStopSyncedSceneEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CStopSyncedSceneEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CStopSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CStopSyncedSceneEvent$null(::std::weak_ptr<::alt::CStopSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CStopSyncedSceneEvent>();
}
void cxxbridge1$weak_ptr$alt$CStopSyncedSceneEvent$clone(::std::weak_ptr<::alt::CStopSyncedSceneEvent> const &self, ::std::weak_ptr<::alt::CStopSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CStopSyncedSceneEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CStopSyncedSceneEvent$downgrade(::std::shared_ptr<::alt::CStopSyncedSceneEvent> const &shared, ::std::weak_ptr<::alt::CStopSyncedSceneEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CStopSyncedSceneEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CStopSyncedSceneEvent$upgrade(::std::weak_ptr<::alt::CStopSyncedSceneEvent> const &weak, ::std::shared_ptr<::alt::CStopSyncedSceneEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CStopSyncedSceneEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CStopSyncedSceneEvent$drop(::std::weak_ptr<::alt::CStopSyncedSceneEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CUpdateSyncedSceneEvent>::value, "definition of CUpdateSyncedSceneEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CUpdateSyncedSceneEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CUpdateSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CUpdateSyncedSceneEvent$null(::std::unique_ptr<::alt::CUpdateSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CUpdateSyncedSceneEvent>();
}
::alt::CUpdateSyncedSceneEvent *cxxbridge1$unique_ptr$alt$CUpdateSyncedSceneEvent$uninit(::std::unique_ptr<::alt::CUpdateSyncedSceneEvent> *ptr) noexcept {
  ::alt::CUpdateSyncedSceneEvent *uninit = reinterpret_cast<::alt::CUpdateSyncedSceneEvent *>(new ::rust::MaybeUninit<::alt::CUpdateSyncedSceneEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CUpdateSyncedSceneEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CUpdateSyncedSceneEvent$raw(::std::unique_ptr<::alt::CUpdateSyncedSceneEvent> *ptr, ::alt::CUpdateSyncedSceneEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CUpdateSyncedSceneEvent>(raw);
}
::alt::CUpdateSyncedSceneEvent const *cxxbridge1$unique_ptr$alt$CUpdateSyncedSceneEvent$get(::std::unique_ptr<::alt::CUpdateSyncedSceneEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CUpdateSyncedSceneEvent *cxxbridge1$unique_ptr$alt$CUpdateSyncedSceneEvent$release(::std::unique_ptr<::alt::CUpdateSyncedSceneEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CUpdateSyncedSceneEvent$drop(::std::unique_ptr<::alt::CUpdateSyncedSceneEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CUpdateSyncedSceneEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CUpdateSyncedSceneEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CUpdateSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CUpdateSyncedSceneEvent$null(::std::shared_ptr<::alt::CUpdateSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CUpdateSyncedSceneEvent>();
}
::alt::CUpdateSyncedSceneEvent *cxxbridge1$shared_ptr$alt$CUpdateSyncedSceneEvent$uninit(::std::shared_ptr<::alt::CUpdateSyncedSceneEvent> *ptr) noexcept {
  ::alt::CUpdateSyncedSceneEvent *uninit = reinterpret_cast<::alt::CUpdateSyncedSceneEvent *>(new ::rust::MaybeUninit<::alt::CUpdateSyncedSceneEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CUpdateSyncedSceneEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CUpdateSyncedSceneEvent$clone(::std::shared_ptr<::alt::CUpdateSyncedSceneEvent> const &self, ::std::shared_ptr<::alt::CUpdateSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CUpdateSyncedSceneEvent>(self);
}
::alt::CUpdateSyncedSceneEvent const *cxxbridge1$shared_ptr$alt$CUpdateSyncedSceneEvent$get(::std::shared_ptr<::alt::CUpdateSyncedSceneEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CUpdateSyncedSceneEvent$drop(::std::shared_ptr<::alt::CUpdateSyncedSceneEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CUpdateSyncedSceneEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CUpdateSyncedSceneEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CUpdateSyncedSceneEvent$null(::std::weak_ptr<::alt::CUpdateSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CUpdateSyncedSceneEvent>();
}
void cxxbridge1$weak_ptr$alt$CUpdateSyncedSceneEvent$clone(::std::weak_ptr<::alt::CUpdateSyncedSceneEvent> const &self, ::std::weak_ptr<::alt::CUpdateSyncedSceneEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CUpdateSyncedSceneEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CUpdateSyncedSceneEvent$downgrade(::std::shared_ptr<::alt::CUpdateSyncedSceneEvent> const &shared, ::std::weak_ptr<::alt::CUpdateSyncedSceneEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CUpdateSyncedSceneEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CUpdateSyncedSceneEvent$upgrade(::std::weak_ptr<::alt::CUpdateSyncedSceneEvent> const &weak, ::std::shared_ptr<::alt::CUpdateSyncedSceneEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CUpdateSyncedSceneEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CUpdateSyncedSceneEvent$drop(::std::weak_ptr<::alt::CUpdateSyncedSceneEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CClientDeleteObjectEvent>::value, "definition of CClientDeleteObjectEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CClientDeleteObjectEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CClientDeleteObjectEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CClientDeleteObjectEvent$null(::std::unique_ptr<::alt::CClientDeleteObjectEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CClientDeleteObjectEvent>();
}
::alt::CClientDeleteObjectEvent *cxxbridge1$unique_ptr$alt$CClientDeleteObjectEvent$uninit(::std::unique_ptr<::alt::CClientDeleteObjectEvent> *ptr) noexcept {
  ::alt::CClientDeleteObjectEvent *uninit = reinterpret_cast<::alt::CClientDeleteObjectEvent *>(new ::rust::MaybeUninit<::alt::CClientDeleteObjectEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CClientDeleteObjectEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CClientDeleteObjectEvent$raw(::std::unique_ptr<::alt::CClientDeleteObjectEvent> *ptr, ::alt::CClientDeleteObjectEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CClientDeleteObjectEvent>(raw);
}
::alt::CClientDeleteObjectEvent const *cxxbridge1$unique_ptr$alt$CClientDeleteObjectEvent$get(::std::unique_ptr<::alt::CClientDeleteObjectEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CClientDeleteObjectEvent *cxxbridge1$unique_ptr$alt$CClientDeleteObjectEvent$release(::std::unique_ptr<::alt::CClientDeleteObjectEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CClientDeleteObjectEvent$drop(::std::unique_ptr<::alt::CClientDeleteObjectEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CClientDeleteObjectEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CClientDeleteObjectEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CClientDeleteObjectEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CClientDeleteObjectEvent$null(::std::shared_ptr<::alt::CClientDeleteObjectEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CClientDeleteObjectEvent>();
}
::alt::CClientDeleteObjectEvent *cxxbridge1$shared_ptr$alt$CClientDeleteObjectEvent$uninit(::std::shared_ptr<::alt::CClientDeleteObjectEvent> *ptr) noexcept {
  ::alt::CClientDeleteObjectEvent *uninit = reinterpret_cast<::alt::CClientDeleteObjectEvent *>(new ::rust::MaybeUninit<::alt::CClientDeleteObjectEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CClientDeleteObjectEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CClientDeleteObjectEvent$clone(::std::shared_ptr<::alt::CClientDeleteObjectEvent> const &self, ::std::shared_ptr<::alt::CClientDeleteObjectEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CClientDeleteObjectEvent>(self);
}
::alt::CClientDeleteObjectEvent const *cxxbridge1$shared_ptr$alt$CClientDeleteObjectEvent$get(::std::shared_ptr<::alt::CClientDeleteObjectEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CClientDeleteObjectEvent$drop(::std::shared_ptr<::alt::CClientDeleteObjectEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CClientDeleteObjectEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CClientDeleteObjectEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CClientDeleteObjectEvent$null(::std::weak_ptr<::alt::CClientDeleteObjectEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CClientDeleteObjectEvent>();
}
void cxxbridge1$weak_ptr$alt$CClientDeleteObjectEvent$clone(::std::weak_ptr<::alt::CClientDeleteObjectEvent> const &self, ::std::weak_ptr<::alt::CClientDeleteObjectEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CClientDeleteObjectEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CClientDeleteObjectEvent$downgrade(::std::shared_ptr<::alt::CClientDeleteObjectEvent> const &shared, ::std::weak_ptr<::alt::CClientDeleteObjectEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CClientDeleteObjectEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CClientDeleteObjectEvent$upgrade(::std::weak_ptr<::alt::CClientDeleteObjectEvent> const &weak, ::std::shared_ptr<::alt::CClientDeleteObjectEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CClientDeleteObjectEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CClientDeleteObjectEvent$drop(::std::weak_ptr<::alt::CClientDeleteObjectEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CClientRequestObjectEvent>::value, "definition of CClientRequestObjectEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CClientRequestObjectEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CClientRequestObjectEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CClientRequestObjectEvent$null(::std::unique_ptr<::alt::CClientRequestObjectEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CClientRequestObjectEvent>();
}
::alt::CClientRequestObjectEvent *cxxbridge1$unique_ptr$alt$CClientRequestObjectEvent$uninit(::std::unique_ptr<::alt::CClientRequestObjectEvent> *ptr) noexcept {
  ::alt::CClientRequestObjectEvent *uninit = reinterpret_cast<::alt::CClientRequestObjectEvent *>(new ::rust::MaybeUninit<::alt::CClientRequestObjectEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CClientRequestObjectEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CClientRequestObjectEvent$raw(::std::unique_ptr<::alt::CClientRequestObjectEvent> *ptr, ::alt::CClientRequestObjectEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CClientRequestObjectEvent>(raw);
}
::alt::CClientRequestObjectEvent const *cxxbridge1$unique_ptr$alt$CClientRequestObjectEvent$get(::std::unique_ptr<::alt::CClientRequestObjectEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CClientRequestObjectEvent *cxxbridge1$unique_ptr$alt$CClientRequestObjectEvent$release(::std::unique_ptr<::alt::CClientRequestObjectEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CClientRequestObjectEvent$drop(::std::unique_ptr<::alt::CClientRequestObjectEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CClientRequestObjectEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CClientRequestObjectEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CClientRequestObjectEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CClientRequestObjectEvent$null(::std::shared_ptr<::alt::CClientRequestObjectEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CClientRequestObjectEvent>();
}
::alt::CClientRequestObjectEvent *cxxbridge1$shared_ptr$alt$CClientRequestObjectEvent$uninit(::std::shared_ptr<::alt::CClientRequestObjectEvent> *ptr) noexcept {
  ::alt::CClientRequestObjectEvent *uninit = reinterpret_cast<::alt::CClientRequestObjectEvent *>(new ::rust::MaybeUninit<::alt::CClientRequestObjectEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CClientRequestObjectEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CClientRequestObjectEvent$clone(::std::shared_ptr<::alt::CClientRequestObjectEvent> const &self, ::std::shared_ptr<::alt::CClientRequestObjectEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CClientRequestObjectEvent>(self);
}
::alt::CClientRequestObjectEvent const *cxxbridge1$shared_ptr$alt$CClientRequestObjectEvent$get(::std::shared_ptr<::alt::CClientRequestObjectEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CClientRequestObjectEvent$drop(::std::shared_ptr<::alt::CClientRequestObjectEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CClientRequestObjectEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CClientRequestObjectEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CClientRequestObjectEvent$null(::std::weak_ptr<::alt::CClientRequestObjectEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CClientRequestObjectEvent>();
}
void cxxbridge1$weak_ptr$alt$CClientRequestObjectEvent$clone(::std::weak_ptr<::alt::CClientRequestObjectEvent> const &self, ::std::weak_ptr<::alt::CClientRequestObjectEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CClientRequestObjectEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CClientRequestObjectEvent$downgrade(::std::shared_ptr<::alt::CClientRequestObjectEvent> const &shared, ::std::weak_ptr<::alt::CClientRequestObjectEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CClientRequestObjectEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CClientRequestObjectEvent$upgrade(::std::weak_ptr<::alt::CClientRequestObjectEvent> const &weak, ::std::shared_ptr<::alt::CClientRequestObjectEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CClientRequestObjectEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CClientRequestObjectEvent$drop(::std::weak_ptr<::alt::CClientRequestObjectEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CGivePedScriptedTaskEvent>::value, "definition of CGivePedScriptedTaskEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CGivePedScriptedTaskEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CGivePedScriptedTaskEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CGivePedScriptedTaskEvent$null(::std::unique_ptr<::alt::CGivePedScriptedTaskEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CGivePedScriptedTaskEvent>();
}
::alt::CGivePedScriptedTaskEvent *cxxbridge1$unique_ptr$alt$CGivePedScriptedTaskEvent$uninit(::std::unique_ptr<::alt::CGivePedScriptedTaskEvent> *ptr) noexcept {
  ::alt::CGivePedScriptedTaskEvent *uninit = reinterpret_cast<::alt::CGivePedScriptedTaskEvent *>(new ::rust::MaybeUninit<::alt::CGivePedScriptedTaskEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CGivePedScriptedTaskEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CGivePedScriptedTaskEvent$raw(::std::unique_ptr<::alt::CGivePedScriptedTaskEvent> *ptr, ::alt::CGivePedScriptedTaskEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CGivePedScriptedTaskEvent>(raw);
}
::alt::CGivePedScriptedTaskEvent const *cxxbridge1$unique_ptr$alt$CGivePedScriptedTaskEvent$get(::std::unique_ptr<::alt::CGivePedScriptedTaskEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CGivePedScriptedTaskEvent *cxxbridge1$unique_ptr$alt$CGivePedScriptedTaskEvent$release(::std::unique_ptr<::alt::CGivePedScriptedTaskEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CGivePedScriptedTaskEvent$drop(::std::unique_ptr<::alt::CGivePedScriptedTaskEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CGivePedScriptedTaskEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CGivePedScriptedTaskEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CGivePedScriptedTaskEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CGivePedScriptedTaskEvent$null(::std::shared_ptr<::alt::CGivePedScriptedTaskEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CGivePedScriptedTaskEvent>();
}
::alt::CGivePedScriptedTaskEvent *cxxbridge1$shared_ptr$alt$CGivePedScriptedTaskEvent$uninit(::std::shared_ptr<::alt::CGivePedScriptedTaskEvent> *ptr) noexcept {
  ::alt::CGivePedScriptedTaskEvent *uninit = reinterpret_cast<::alt::CGivePedScriptedTaskEvent *>(new ::rust::MaybeUninit<::alt::CGivePedScriptedTaskEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CGivePedScriptedTaskEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CGivePedScriptedTaskEvent$clone(::std::shared_ptr<::alt::CGivePedScriptedTaskEvent> const &self, ::std::shared_ptr<::alt::CGivePedScriptedTaskEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CGivePedScriptedTaskEvent>(self);
}
::alt::CGivePedScriptedTaskEvent const *cxxbridge1$shared_ptr$alt$CGivePedScriptedTaskEvent$get(::std::shared_ptr<::alt::CGivePedScriptedTaskEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CGivePedScriptedTaskEvent$drop(::std::shared_ptr<::alt::CGivePedScriptedTaskEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CGivePedScriptedTaskEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CGivePedScriptedTaskEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CGivePedScriptedTaskEvent$null(::std::weak_ptr<::alt::CGivePedScriptedTaskEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CGivePedScriptedTaskEvent>();
}
void cxxbridge1$weak_ptr$alt$CGivePedScriptedTaskEvent$clone(::std::weak_ptr<::alt::CGivePedScriptedTaskEvent> const &self, ::std::weak_ptr<::alt::CGivePedScriptedTaskEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CGivePedScriptedTaskEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CGivePedScriptedTaskEvent$downgrade(::std::shared_ptr<::alt::CGivePedScriptedTaskEvent> const &shared, ::std::weak_ptr<::alt::CGivePedScriptedTaskEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CGivePedScriptedTaskEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CGivePedScriptedTaskEvent$upgrade(::std::weak_ptr<::alt::CGivePedScriptedTaskEvent> const &weak, ::std::shared_ptr<::alt::CGivePedScriptedTaskEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CGivePedScriptedTaskEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CGivePedScriptedTaskEvent$drop(::std::weak_ptr<::alt::CGivePedScriptedTaskEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPedDeathEvent>::value, "definition of CPedDeathEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPedDeathEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPedDeathEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPedDeathEvent$null(::std::unique_ptr<::alt::CPedDeathEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPedDeathEvent>();
}
::alt::CPedDeathEvent *cxxbridge1$unique_ptr$alt$CPedDeathEvent$uninit(::std::unique_ptr<::alt::CPedDeathEvent> *ptr) noexcept {
  ::alt::CPedDeathEvent *uninit = reinterpret_cast<::alt::CPedDeathEvent *>(new ::rust::MaybeUninit<::alt::CPedDeathEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPedDeathEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPedDeathEvent$raw(::std::unique_ptr<::alt::CPedDeathEvent> *ptr, ::alt::CPedDeathEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPedDeathEvent>(raw);
}
::alt::CPedDeathEvent const *cxxbridge1$unique_ptr$alt$CPedDeathEvent$get(::std::unique_ptr<::alt::CPedDeathEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPedDeathEvent *cxxbridge1$unique_ptr$alt$CPedDeathEvent$release(::std::unique_ptr<::alt::CPedDeathEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPedDeathEvent$drop(::std::unique_ptr<::alt::CPedDeathEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPedDeathEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPedDeathEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPedDeathEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPedDeathEvent$null(::std::shared_ptr<::alt::CPedDeathEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPedDeathEvent>();
}
::alt::CPedDeathEvent *cxxbridge1$shared_ptr$alt$CPedDeathEvent$uninit(::std::shared_ptr<::alt::CPedDeathEvent> *ptr) noexcept {
  ::alt::CPedDeathEvent *uninit = reinterpret_cast<::alt::CPedDeathEvent *>(new ::rust::MaybeUninit<::alt::CPedDeathEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPedDeathEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPedDeathEvent$clone(::std::shared_ptr<::alt::CPedDeathEvent> const &self, ::std::shared_ptr<::alt::CPedDeathEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPedDeathEvent>(self);
}
::alt::CPedDeathEvent const *cxxbridge1$shared_ptr$alt$CPedDeathEvent$get(::std::shared_ptr<::alt::CPedDeathEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPedDeathEvent$drop(::std::shared_ptr<::alt::CPedDeathEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPedDeathEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPedDeathEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPedDeathEvent$null(::std::weak_ptr<::alt::CPedDeathEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPedDeathEvent>();
}
void cxxbridge1$weak_ptr$alt$CPedDeathEvent$clone(::std::weak_ptr<::alt::CPedDeathEvent> const &self, ::std::weak_ptr<::alt::CPedDeathEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPedDeathEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPedDeathEvent$downgrade(::std::shared_ptr<::alt::CPedDeathEvent> const &shared, ::std::weak_ptr<::alt::CPedDeathEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPedDeathEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPedDeathEvent$upgrade(::std::weak_ptr<::alt::CPedDeathEvent> const &weak, ::std::shared_ptr<::alt::CPedDeathEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPedDeathEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPedDeathEvent$drop(::std::weak_ptr<::alt::CPedDeathEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPedDamageEvent>::value, "definition of CPedDamageEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPedDamageEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPedDamageEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPedDamageEvent$null(::std::unique_ptr<::alt::CPedDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPedDamageEvent>();
}
::alt::CPedDamageEvent *cxxbridge1$unique_ptr$alt$CPedDamageEvent$uninit(::std::unique_ptr<::alt::CPedDamageEvent> *ptr) noexcept {
  ::alt::CPedDamageEvent *uninit = reinterpret_cast<::alt::CPedDamageEvent *>(new ::rust::MaybeUninit<::alt::CPedDamageEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPedDamageEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPedDamageEvent$raw(::std::unique_ptr<::alt::CPedDamageEvent> *ptr, ::alt::CPedDamageEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPedDamageEvent>(raw);
}
::alt::CPedDamageEvent const *cxxbridge1$unique_ptr$alt$CPedDamageEvent$get(::std::unique_ptr<::alt::CPedDamageEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPedDamageEvent *cxxbridge1$unique_ptr$alt$CPedDamageEvent$release(::std::unique_ptr<::alt::CPedDamageEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPedDamageEvent$drop(::std::unique_ptr<::alt::CPedDamageEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPedDamageEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPedDamageEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPedDamageEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPedDamageEvent$null(::std::shared_ptr<::alt::CPedDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPedDamageEvent>();
}
::alt::CPedDamageEvent *cxxbridge1$shared_ptr$alt$CPedDamageEvent$uninit(::std::shared_ptr<::alt::CPedDamageEvent> *ptr) noexcept {
  ::alt::CPedDamageEvent *uninit = reinterpret_cast<::alt::CPedDamageEvent *>(new ::rust::MaybeUninit<::alt::CPedDamageEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPedDamageEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPedDamageEvent$clone(::std::shared_ptr<::alt::CPedDamageEvent> const &self, ::std::shared_ptr<::alt::CPedDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPedDamageEvent>(self);
}
::alt::CPedDamageEvent const *cxxbridge1$shared_ptr$alt$CPedDamageEvent$get(::std::shared_ptr<::alt::CPedDamageEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPedDamageEvent$drop(::std::shared_ptr<::alt::CPedDamageEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPedDamageEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPedDamageEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPedDamageEvent$null(::std::weak_ptr<::alt::CPedDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPedDamageEvent>();
}
void cxxbridge1$weak_ptr$alt$CPedDamageEvent$clone(::std::weak_ptr<::alt::CPedDamageEvent> const &self, ::std::weak_ptr<::alt::CPedDamageEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPedDamageEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPedDamageEvent$downgrade(::std::shared_ptr<::alt::CPedDamageEvent> const &shared, ::std::weak_ptr<::alt::CPedDamageEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPedDamageEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPedDamageEvent$upgrade(::std::weak_ptr<::alt::CPedDamageEvent> const &weak, ::std::shared_ptr<::alt::CPedDamageEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPedDamageEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPedDamageEvent$drop(::std::weak_ptr<::alt::CPedDamageEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CPedHealEvent>::value, "definition of CPedHealEvent is required");
static_assert(sizeof(::std::unique_ptr<::alt::CPedHealEvent>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CPedHealEvent>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CPedHealEvent$null(::std::unique_ptr<::alt::CPedHealEvent> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPedHealEvent>();
}
::alt::CPedHealEvent *cxxbridge1$unique_ptr$alt$CPedHealEvent$uninit(::std::unique_ptr<::alt::CPedHealEvent> *ptr) noexcept {
  ::alt::CPedHealEvent *uninit = reinterpret_cast<::alt::CPedHealEvent *>(new ::rust::MaybeUninit<::alt::CPedHealEvent>);
  ::new (ptr) ::std::unique_ptr<::alt::CPedHealEvent>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CPedHealEvent$raw(::std::unique_ptr<::alt::CPedHealEvent> *ptr, ::alt::CPedHealEvent *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CPedHealEvent>(raw);
}
::alt::CPedHealEvent const *cxxbridge1$unique_ptr$alt$CPedHealEvent$get(::std::unique_ptr<::alt::CPedHealEvent> const &ptr) noexcept {
  return ptr.get();
}
::alt::CPedHealEvent *cxxbridge1$unique_ptr$alt$CPedHealEvent$release(::std::unique_ptr<::alt::CPedHealEvent> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CPedHealEvent$drop(::std::unique_ptr<::alt::CPedHealEvent> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CPedHealEvent>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CPedHealEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CPedHealEvent>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CPedHealEvent$null(::std::shared_ptr<::alt::CPedHealEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPedHealEvent>();
}
::alt::CPedHealEvent *cxxbridge1$shared_ptr$alt$CPedHealEvent$uninit(::std::shared_ptr<::alt::CPedHealEvent> *ptr) noexcept {
  ::alt::CPedHealEvent *uninit = reinterpret_cast<::alt::CPedHealEvent *>(new ::rust::MaybeUninit<::alt::CPedHealEvent>);
  ::new (ptr) ::std::shared_ptr<::alt::CPedHealEvent>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CPedHealEvent$clone(::std::shared_ptr<::alt::CPedHealEvent> const &self, ::std::shared_ptr<::alt::CPedHealEvent> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CPedHealEvent>(self);
}
::alt::CPedHealEvent const *cxxbridge1$shared_ptr$alt$CPedHealEvent$get(::std::shared_ptr<::alt::CPedHealEvent> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CPedHealEvent$drop(::std::shared_ptr<::alt::CPedHealEvent> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CPedHealEvent>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CPedHealEvent>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CPedHealEvent$null(::std::weak_ptr<::alt::CPedHealEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPedHealEvent>();
}
void cxxbridge1$weak_ptr$alt$CPedHealEvent$clone(::std::weak_ptr<::alt::CPedHealEvent> const &self, ::std::weak_ptr<::alt::CPedHealEvent> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CPedHealEvent>(self);
}
void cxxbridge1$weak_ptr$alt$CPedHealEvent$downgrade(::std::shared_ptr<::alt::CPedHealEvent> const &shared, ::std::weak_ptr<::alt::CPedHealEvent> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CPedHealEvent>(shared);
}
void cxxbridge1$weak_ptr$alt$CPedHealEvent$upgrade(::std::weak_ptr<::alt::CPedHealEvent> const &weak, ::std::shared_ptr<::alt::CPedHealEvent> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CPedHealEvent>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CPedHealEvent$drop(::std::weak_ptr<::alt::CPedHealEvent> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>::value, "definition of Config_internal_ValueWrapper_Config_Value_AutocxxConcrete is required");
static_assert(sizeof(::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>) == alignof(void *), "");
void cxxbridge1$unique_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$null(::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>();
}
void cxxbridge1$unique_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$raw(::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *ptr, ::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>(raw);
}
::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete const *cxxbridge1$unique_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$get(::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> const &ptr) noexcept {
  return ptr.get();
}
::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *cxxbridge1$unique_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$release(::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$drop(::std::unique_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>) == alignof(void *), "");
void cxxbridge1$shared_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$null(::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>();
}
void cxxbridge1$shared_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$clone(::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> const &self, ::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>(self);
}
::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete const *cxxbridge1$shared_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$get(::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$drop(::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>) == alignof(void *), "");
void cxxbridge1$weak_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$null(::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>();
}
void cxxbridge1$weak_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$clone(::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> const &self, ::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>(self);
}
void cxxbridge1$weak_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$downgrade(::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> const &shared, ::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>(shared);
}
void cxxbridge1$weak_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$upgrade(::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> const &weak, ::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>(weak.lock());
}
void cxxbridge1$weak_ptr$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$drop(::std::weak_ptr<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::MValueUnorderedMapWrapper>::value, "definition of MValueUnorderedMapWrapper is required");
static_assert(sizeof(::std::unique_ptr<::MValueUnorderedMapWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::MValueUnorderedMapWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$MValueUnorderedMapWrapper$null(::std::unique_ptr<::MValueUnorderedMapWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::MValueUnorderedMapWrapper>();
}
::MValueUnorderedMapWrapper *cxxbridge1$unique_ptr$MValueUnorderedMapWrapper$uninit(::std::unique_ptr<::MValueUnorderedMapWrapper> *ptr) noexcept {
  ::MValueUnorderedMapWrapper *uninit = reinterpret_cast<::MValueUnorderedMapWrapper *>(new ::rust::MaybeUninit<::MValueUnorderedMapWrapper>);
  ::new (ptr) ::std::unique_ptr<::MValueUnorderedMapWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$MValueUnorderedMapWrapper$raw(::std::unique_ptr<::MValueUnorderedMapWrapper> *ptr, ::MValueUnorderedMapWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::MValueUnorderedMapWrapper>(raw);
}
::MValueUnorderedMapWrapper const *cxxbridge1$unique_ptr$MValueUnorderedMapWrapper$get(::std::unique_ptr<::MValueUnorderedMapWrapper> const &ptr) noexcept {
  return ptr.get();
}
::MValueUnorderedMapWrapper *cxxbridge1$unique_ptr$MValueUnorderedMapWrapper$release(::std::unique_ptr<::MValueUnorderedMapWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$MValueUnorderedMapWrapper$drop(::std::unique_ptr<::MValueUnorderedMapWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::MValueUnorderedMapWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::MValueUnorderedMapWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::MValueUnorderedMapWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$MValueUnorderedMapWrapper$null(::std::shared_ptr<::MValueUnorderedMapWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::MValueUnorderedMapWrapper>();
}
::MValueUnorderedMapWrapper *cxxbridge1$shared_ptr$MValueUnorderedMapWrapper$uninit(::std::shared_ptr<::MValueUnorderedMapWrapper> *ptr) noexcept {
  ::MValueUnorderedMapWrapper *uninit = reinterpret_cast<::MValueUnorderedMapWrapper *>(new ::rust::MaybeUninit<::MValueUnorderedMapWrapper>);
  ::new (ptr) ::std::shared_ptr<::MValueUnorderedMapWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$MValueUnorderedMapWrapper$clone(::std::shared_ptr<::MValueUnorderedMapWrapper> const &self, ::std::shared_ptr<::MValueUnorderedMapWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::MValueUnorderedMapWrapper>(self);
}
::MValueUnorderedMapWrapper const *cxxbridge1$shared_ptr$MValueUnorderedMapWrapper$get(::std::shared_ptr<::MValueUnorderedMapWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$MValueUnorderedMapWrapper$drop(::std::shared_ptr<::MValueUnorderedMapWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::MValueUnorderedMapWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::MValueUnorderedMapWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$MValueUnorderedMapWrapper$null(::std::weak_ptr<::MValueUnorderedMapWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::MValueUnorderedMapWrapper>();
}
void cxxbridge1$weak_ptr$MValueUnorderedMapWrapper$clone(::std::weak_ptr<::MValueUnorderedMapWrapper> const &self, ::std::weak_ptr<::MValueUnorderedMapWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::MValueUnorderedMapWrapper>(self);
}
void cxxbridge1$weak_ptr$MValueUnorderedMapWrapper$downgrade(::std::shared_ptr<::MValueUnorderedMapWrapper> const &shared, ::std::weak_ptr<::MValueUnorderedMapWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::MValueUnorderedMapWrapper>(shared);
}
void cxxbridge1$weak_ptr$MValueUnorderedMapWrapper$upgrade(::std::weak_ptr<::MValueUnorderedMapWrapper> const &weak, ::std::shared_ptr<::MValueUnorderedMapWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::MValueUnorderedMapWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$MValueUnorderedMapWrapper$drop(::std::weak_ptr<::MValueUnorderedMapWrapper> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::Vector2Vec>::value, "definition of Vector2Vec is required");
static_assert(sizeof(::std::unique_ptr<::Vector2Vec>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::Vector2Vec>) == alignof(void *), "");
void cxxbridge1$unique_ptr$Vector2Vec$null(::std::unique_ptr<::Vector2Vec> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::Vector2Vec>();
}
::Vector2Vec *cxxbridge1$unique_ptr$Vector2Vec$uninit(::std::unique_ptr<::Vector2Vec> *ptr) noexcept {
  ::Vector2Vec *uninit = reinterpret_cast<::Vector2Vec *>(new ::rust::MaybeUninit<::Vector2Vec>);
  ::new (ptr) ::std::unique_ptr<::Vector2Vec>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$Vector2Vec$raw(::std::unique_ptr<::Vector2Vec> *ptr, ::Vector2Vec *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::Vector2Vec>(raw);
}
::Vector2Vec const *cxxbridge1$unique_ptr$Vector2Vec$get(::std::unique_ptr<::Vector2Vec> const &ptr) noexcept {
  return ptr.get();
}
::Vector2Vec *cxxbridge1$unique_ptr$Vector2Vec$release(::std::unique_ptr<::Vector2Vec> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$Vector2Vec$drop(::std::unique_ptr<::Vector2Vec> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::Vector2Vec>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::Vector2Vec>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::Vector2Vec>) == alignof(void *), "");
void cxxbridge1$shared_ptr$Vector2Vec$null(::std::shared_ptr<::Vector2Vec> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::Vector2Vec>();
}
::Vector2Vec *cxxbridge1$shared_ptr$Vector2Vec$uninit(::std::shared_ptr<::Vector2Vec> *ptr) noexcept {
  ::Vector2Vec *uninit = reinterpret_cast<::Vector2Vec *>(new ::rust::MaybeUninit<::Vector2Vec>);
  ::new (ptr) ::std::shared_ptr<::Vector2Vec>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$Vector2Vec$clone(::std::shared_ptr<::Vector2Vec> const &self, ::std::shared_ptr<::Vector2Vec> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::Vector2Vec>(self);
}
::Vector2Vec const *cxxbridge1$shared_ptr$Vector2Vec$get(::std::shared_ptr<::Vector2Vec> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$Vector2Vec$drop(::std::shared_ptr<::Vector2Vec> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::Vector2Vec>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::Vector2Vec>) == alignof(void *), "");
void cxxbridge1$weak_ptr$Vector2Vec$null(::std::weak_ptr<::Vector2Vec> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::Vector2Vec>();
}
void cxxbridge1$weak_ptr$Vector2Vec$clone(::std::weak_ptr<::Vector2Vec> const &self, ::std::weak_ptr<::Vector2Vec> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::Vector2Vec>(self);
}
void cxxbridge1$weak_ptr$Vector2Vec$downgrade(::std::shared_ptr<::Vector2Vec> const &shared, ::std::weak_ptr<::Vector2Vec> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::Vector2Vec>(shared);
}
void cxxbridge1$weak_ptr$Vector2Vec$upgrade(::std::weak_ptr<::Vector2Vec> const &weak, ::std::shared_ptr<::Vector2Vec> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::Vector2Vec>(weak.lock());
}
void cxxbridge1$weak_ptr$Vector2Vec$drop(::std::weak_ptr<::Vector2Vec> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::Vector2Vec> *cxxbridge1$std$vector$Vector2Vec$new() noexcept {
  return new ::std::vector<::Vector2Vec>();
}
::std::size_t cxxbridge1$std$vector$Vector2Vec$size(::std::vector<::Vector2Vec> const &s) noexcept {
  return s.size();
}
::Vector2Vec *cxxbridge1$std$vector$Vector2Vec$get_unchecked(::std::vector<::Vector2Vec> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$Vector2Vec$push_back(::std::vector<::Vector2Vec> *v, ::Vector2Vec *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$Vector2Vec$pop_back(::std::vector<::Vector2Vec> *v, ::Vector2Vec *out) noexcept {
  ::new (out) ::Vector2Vec(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::Vector2Vec>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::Vector2Vec>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$Vector2Vec$null(::std::unique_ptr<::std::vector<::Vector2Vec>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::Vector2Vec>>();
}
void cxxbridge1$unique_ptr$std$vector$Vector2Vec$raw(::std::unique_ptr<::std::vector<::Vector2Vec>> *ptr, ::std::vector<::Vector2Vec> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::Vector2Vec>>(raw);
}
::std::vector<::Vector2Vec> const *cxxbridge1$unique_ptr$std$vector$Vector2Vec$get(::std::unique_ptr<::std::vector<::Vector2Vec>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::Vector2Vec> *cxxbridge1$unique_ptr$std$vector$Vector2Vec$release(::std::unique_ptr<::std::vector<::Vector2Vec>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$Vector2Vec$drop(::std::unique_ptr<::std::vector<::Vector2Vec>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::MValueMutWrapper>::value, "definition of MValueMutWrapper is required");
static_assert(sizeof(::std::unique_ptr<::MValueMutWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::MValueMutWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$MValueMutWrapper$null(::std::unique_ptr<::MValueMutWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::MValueMutWrapper>();
}
::MValueMutWrapper *cxxbridge1$unique_ptr$MValueMutWrapper$uninit(::std::unique_ptr<::MValueMutWrapper> *ptr) noexcept {
  ::MValueMutWrapper *uninit = reinterpret_cast<::MValueMutWrapper *>(new ::rust::MaybeUninit<::MValueMutWrapper>);
  ::new (ptr) ::std::unique_ptr<::MValueMutWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$MValueMutWrapper$raw(::std::unique_ptr<::MValueMutWrapper> *ptr, ::MValueMutWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::MValueMutWrapper>(raw);
}
::MValueMutWrapper const *cxxbridge1$unique_ptr$MValueMutWrapper$get(::std::unique_ptr<::MValueMutWrapper> const &ptr) noexcept {
  return ptr.get();
}
::MValueMutWrapper *cxxbridge1$unique_ptr$MValueMutWrapper$release(::std::unique_ptr<::MValueMutWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$MValueMutWrapper$drop(::std::unique_ptr<::MValueMutWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::MValueMutWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::MValueMutWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::MValueMutWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$MValueMutWrapper$null(::std::shared_ptr<::MValueMutWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::MValueMutWrapper>();
}
::MValueMutWrapper *cxxbridge1$shared_ptr$MValueMutWrapper$uninit(::std::shared_ptr<::MValueMutWrapper> *ptr) noexcept {
  ::MValueMutWrapper *uninit = reinterpret_cast<::MValueMutWrapper *>(new ::rust::MaybeUninit<::MValueMutWrapper>);
  ::new (ptr) ::std::shared_ptr<::MValueMutWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$MValueMutWrapper$clone(::std::shared_ptr<::MValueMutWrapper> const &self, ::std::shared_ptr<::MValueMutWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::MValueMutWrapper>(self);
}
::MValueMutWrapper const *cxxbridge1$shared_ptr$MValueMutWrapper$get(::std::shared_ptr<::MValueMutWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$MValueMutWrapper$drop(::std::shared_ptr<::MValueMutWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::MValueMutWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::MValueMutWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$MValueMutWrapper$null(::std::weak_ptr<::MValueMutWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::MValueMutWrapper>();
}
void cxxbridge1$weak_ptr$MValueMutWrapper$clone(::std::weak_ptr<::MValueMutWrapper> const &self, ::std::weak_ptr<::MValueMutWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::MValueMutWrapper>(self);
}
void cxxbridge1$weak_ptr$MValueMutWrapper$downgrade(::std::shared_ptr<::MValueMutWrapper> const &shared, ::std::weak_ptr<::MValueMutWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::MValueMutWrapper>(shared);
}
void cxxbridge1$weak_ptr$MValueMutWrapper$upgrade(::std::weak_ptr<::MValueMutWrapper> const &weak, ::std::shared_ptr<::MValueMutWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::MValueMutWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$MValueMutWrapper$drop(::std::weak_ptr<::MValueMutWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::MValueMutWrapper> *cxxbridge1$std$vector$MValueMutWrapper$new() noexcept {
  return new ::std::vector<::MValueMutWrapper>();
}
::std::size_t cxxbridge1$std$vector$MValueMutWrapper$size(::std::vector<::MValueMutWrapper> const &s) noexcept {
  return s.size();
}
::MValueMutWrapper *cxxbridge1$std$vector$MValueMutWrapper$get_unchecked(::std::vector<::MValueMutWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$MValueMutWrapper$push_back(::std::vector<::MValueMutWrapper> *v, ::MValueMutWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$MValueMutWrapper$pop_back(::std::vector<::MValueMutWrapper> *v, ::MValueMutWrapper *out) noexcept {
  ::new (out) ::MValueMutWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::MValueMutWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::MValueMutWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$MValueMutWrapper$null(::std::unique_ptr<::std::vector<::MValueMutWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::MValueMutWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$MValueMutWrapper$raw(::std::unique_ptr<::std::vector<::MValueMutWrapper>> *ptr, ::std::vector<::MValueMutWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::MValueMutWrapper>>(raw);
}
::std::vector<::MValueMutWrapper> const *cxxbridge1$unique_ptr$std$vector$MValueMutWrapper$get(::std::unique_ptr<::std::vector<::MValueMutWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::MValueMutWrapper> *cxxbridge1$unique_ptr$std$vector$MValueMutWrapper$release(::std::unique_ptr<::std::vector<::MValueMutWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$MValueMutWrapper$drop(::std::unique_ptr<::std::vector<::MValueMutWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::ResourcePtrWrapper>::value, "definition of ResourcePtrWrapper is required");
static_assert(sizeof(::std::unique_ptr<::ResourcePtrWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::ResourcePtrWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$ResourcePtrWrapper$null(::std::unique_ptr<::ResourcePtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::ResourcePtrWrapper>();
}
::ResourcePtrWrapper *cxxbridge1$unique_ptr$ResourcePtrWrapper$uninit(::std::unique_ptr<::ResourcePtrWrapper> *ptr) noexcept {
  ::ResourcePtrWrapper *uninit = reinterpret_cast<::ResourcePtrWrapper *>(new ::rust::MaybeUninit<::ResourcePtrWrapper>);
  ::new (ptr) ::std::unique_ptr<::ResourcePtrWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$ResourcePtrWrapper$raw(::std::unique_ptr<::ResourcePtrWrapper> *ptr, ::ResourcePtrWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::ResourcePtrWrapper>(raw);
}
::ResourcePtrWrapper const *cxxbridge1$unique_ptr$ResourcePtrWrapper$get(::std::unique_ptr<::ResourcePtrWrapper> const &ptr) noexcept {
  return ptr.get();
}
::ResourcePtrWrapper *cxxbridge1$unique_ptr$ResourcePtrWrapper$release(::std::unique_ptr<::ResourcePtrWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$ResourcePtrWrapper$drop(::std::unique_ptr<::ResourcePtrWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::ResourcePtrWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::ResourcePtrWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::ResourcePtrWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$ResourcePtrWrapper$null(::std::shared_ptr<::ResourcePtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::ResourcePtrWrapper>();
}
::ResourcePtrWrapper *cxxbridge1$shared_ptr$ResourcePtrWrapper$uninit(::std::shared_ptr<::ResourcePtrWrapper> *ptr) noexcept {
  ::ResourcePtrWrapper *uninit = reinterpret_cast<::ResourcePtrWrapper *>(new ::rust::MaybeUninit<::ResourcePtrWrapper>);
  ::new (ptr) ::std::shared_ptr<::ResourcePtrWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$ResourcePtrWrapper$clone(::std::shared_ptr<::ResourcePtrWrapper> const &self, ::std::shared_ptr<::ResourcePtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::ResourcePtrWrapper>(self);
}
::ResourcePtrWrapper const *cxxbridge1$shared_ptr$ResourcePtrWrapper$get(::std::shared_ptr<::ResourcePtrWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$ResourcePtrWrapper$drop(::std::shared_ptr<::ResourcePtrWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::ResourcePtrWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::ResourcePtrWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$ResourcePtrWrapper$null(::std::weak_ptr<::ResourcePtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::ResourcePtrWrapper>();
}
void cxxbridge1$weak_ptr$ResourcePtrWrapper$clone(::std::weak_ptr<::ResourcePtrWrapper> const &self, ::std::weak_ptr<::ResourcePtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::ResourcePtrWrapper>(self);
}
void cxxbridge1$weak_ptr$ResourcePtrWrapper$downgrade(::std::shared_ptr<::ResourcePtrWrapper> const &shared, ::std::weak_ptr<::ResourcePtrWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::ResourcePtrWrapper>(shared);
}
void cxxbridge1$weak_ptr$ResourcePtrWrapper$upgrade(::std::weak_ptr<::ResourcePtrWrapper> const &weak, ::std::shared_ptr<::ResourcePtrWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::ResourcePtrWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$ResourcePtrWrapper$drop(::std::weak_ptr<::ResourcePtrWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::ResourcePtrWrapper> *cxxbridge1$std$vector$ResourcePtrWrapper$new() noexcept {
  return new ::std::vector<::ResourcePtrWrapper>();
}
::std::size_t cxxbridge1$std$vector$ResourcePtrWrapper$size(::std::vector<::ResourcePtrWrapper> const &s) noexcept {
  return s.size();
}
::ResourcePtrWrapper *cxxbridge1$std$vector$ResourcePtrWrapper$get_unchecked(::std::vector<::ResourcePtrWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$ResourcePtrWrapper$push_back(::std::vector<::ResourcePtrWrapper> *v, ::ResourcePtrWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$ResourcePtrWrapper$pop_back(::std::vector<::ResourcePtrWrapper> *v, ::ResourcePtrWrapper *out) noexcept {
  ::new (out) ::ResourcePtrWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::ResourcePtrWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::ResourcePtrWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$ResourcePtrWrapper$null(::std::unique_ptr<::std::vector<::ResourcePtrWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::ResourcePtrWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$ResourcePtrWrapper$raw(::std::unique_ptr<::std::vector<::ResourcePtrWrapper>> *ptr, ::std::vector<::ResourcePtrWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::ResourcePtrWrapper>>(raw);
}
::std::vector<::ResourcePtrWrapper> const *cxxbridge1$unique_ptr$std$vector$ResourcePtrWrapper$get(::std::unique_ptr<::std::vector<::ResourcePtrWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::ResourcePtrWrapper> *cxxbridge1$unique_ptr$std$vector$ResourcePtrWrapper$release(::std::unique_ptr<::std::vector<::ResourcePtrWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$ResourcePtrWrapper$drop(::std::unique_ptr<::std::vector<::ResourcePtrWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::PlayerPtrWrapper>::value, "definition of PlayerPtrWrapper is required");
static_assert(sizeof(::std::unique_ptr<::PlayerPtrWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::PlayerPtrWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$PlayerPtrWrapper$null(::std::unique_ptr<::PlayerPtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::PlayerPtrWrapper>();
}
::PlayerPtrWrapper *cxxbridge1$unique_ptr$PlayerPtrWrapper$uninit(::std::unique_ptr<::PlayerPtrWrapper> *ptr) noexcept {
  ::PlayerPtrWrapper *uninit = reinterpret_cast<::PlayerPtrWrapper *>(new ::rust::MaybeUninit<::PlayerPtrWrapper>);
  ::new (ptr) ::std::unique_ptr<::PlayerPtrWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$PlayerPtrWrapper$raw(::std::unique_ptr<::PlayerPtrWrapper> *ptr, ::PlayerPtrWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::PlayerPtrWrapper>(raw);
}
::PlayerPtrWrapper const *cxxbridge1$unique_ptr$PlayerPtrWrapper$get(::std::unique_ptr<::PlayerPtrWrapper> const &ptr) noexcept {
  return ptr.get();
}
::PlayerPtrWrapper *cxxbridge1$unique_ptr$PlayerPtrWrapper$release(::std::unique_ptr<::PlayerPtrWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$PlayerPtrWrapper$drop(::std::unique_ptr<::PlayerPtrWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::PlayerPtrWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::PlayerPtrWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::PlayerPtrWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$PlayerPtrWrapper$null(::std::shared_ptr<::PlayerPtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::PlayerPtrWrapper>();
}
::PlayerPtrWrapper *cxxbridge1$shared_ptr$PlayerPtrWrapper$uninit(::std::shared_ptr<::PlayerPtrWrapper> *ptr) noexcept {
  ::PlayerPtrWrapper *uninit = reinterpret_cast<::PlayerPtrWrapper *>(new ::rust::MaybeUninit<::PlayerPtrWrapper>);
  ::new (ptr) ::std::shared_ptr<::PlayerPtrWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$PlayerPtrWrapper$clone(::std::shared_ptr<::PlayerPtrWrapper> const &self, ::std::shared_ptr<::PlayerPtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::PlayerPtrWrapper>(self);
}
::PlayerPtrWrapper const *cxxbridge1$shared_ptr$PlayerPtrWrapper$get(::std::shared_ptr<::PlayerPtrWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$PlayerPtrWrapper$drop(::std::shared_ptr<::PlayerPtrWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::PlayerPtrWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::PlayerPtrWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$PlayerPtrWrapper$null(::std::weak_ptr<::PlayerPtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::PlayerPtrWrapper>();
}
void cxxbridge1$weak_ptr$PlayerPtrWrapper$clone(::std::weak_ptr<::PlayerPtrWrapper> const &self, ::std::weak_ptr<::PlayerPtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::PlayerPtrWrapper>(self);
}
void cxxbridge1$weak_ptr$PlayerPtrWrapper$downgrade(::std::shared_ptr<::PlayerPtrWrapper> const &shared, ::std::weak_ptr<::PlayerPtrWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::PlayerPtrWrapper>(shared);
}
void cxxbridge1$weak_ptr$PlayerPtrWrapper$upgrade(::std::weak_ptr<::PlayerPtrWrapper> const &weak, ::std::shared_ptr<::PlayerPtrWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::PlayerPtrWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$PlayerPtrWrapper$drop(::std::weak_ptr<::PlayerPtrWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::PlayerPtrWrapper> *cxxbridge1$std$vector$PlayerPtrWrapper$new() noexcept {
  return new ::std::vector<::PlayerPtrWrapper>();
}
::std::size_t cxxbridge1$std$vector$PlayerPtrWrapper$size(::std::vector<::PlayerPtrWrapper> const &s) noexcept {
  return s.size();
}
::PlayerPtrWrapper *cxxbridge1$std$vector$PlayerPtrWrapper$get_unchecked(::std::vector<::PlayerPtrWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$PlayerPtrWrapper$push_back(::std::vector<::PlayerPtrWrapper> *v, ::PlayerPtrWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$PlayerPtrWrapper$pop_back(::std::vector<::PlayerPtrWrapper> *v, ::PlayerPtrWrapper *out) noexcept {
  ::new (out) ::PlayerPtrWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::PlayerPtrWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::PlayerPtrWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$PlayerPtrWrapper$null(::std::unique_ptr<::std::vector<::PlayerPtrWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::PlayerPtrWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$PlayerPtrWrapper$raw(::std::unique_ptr<::std::vector<::PlayerPtrWrapper>> *ptr, ::std::vector<::PlayerPtrWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::PlayerPtrWrapper>>(raw);
}
::std::vector<::PlayerPtrWrapper> const *cxxbridge1$unique_ptr$std$vector$PlayerPtrWrapper$get(::std::unique_ptr<::std::vector<::PlayerPtrWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::PlayerPtrWrapper> *cxxbridge1$unique_ptr$std$vector$PlayerPtrWrapper$release(::std::unique_ptr<::std::vector<::PlayerPtrWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$PlayerPtrWrapper$drop(::std::unique_ptr<::std::vector<::PlayerPtrWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::VehicleModelInfo>::value, "definition of VehicleModelInfo is required");
static_assert(sizeof(::std::unique_ptr<::alt::VehicleModelInfo>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::VehicleModelInfo>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$VehicleModelInfo$null(::std::unique_ptr<::alt::VehicleModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::VehicleModelInfo>();
}
::alt::VehicleModelInfo *cxxbridge1$unique_ptr$alt$VehicleModelInfo$uninit(::std::unique_ptr<::alt::VehicleModelInfo> *ptr) noexcept {
  ::alt::VehicleModelInfo *uninit = reinterpret_cast<::alt::VehicleModelInfo *>(new ::rust::MaybeUninit<::alt::VehicleModelInfo>);
  ::new (ptr) ::std::unique_ptr<::alt::VehicleModelInfo>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$VehicleModelInfo$raw(::std::unique_ptr<::alt::VehicleModelInfo> *ptr, ::alt::VehicleModelInfo *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::VehicleModelInfo>(raw);
}
::alt::VehicleModelInfo const *cxxbridge1$unique_ptr$alt$VehicleModelInfo$get(::std::unique_ptr<::alt::VehicleModelInfo> const &ptr) noexcept {
  return ptr.get();
}
::alt::VehicleModelInfo *cxxbridge1$unique_ptr$alt$VehicleModelInfo$release(::std::unique_ptr<::alt::VehicleModelInfo> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$VehicleModelInfo$drop(::std::unique_ptr<::alt::VehicleModelInfo> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::VehicleModelInfo>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::VehicleModelInfo>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::VehicleModelInfo>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$VehicleModelInfo$null(::std::shared_ptr<::alt::VehicleModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::VehicleModelInfo>();
}
::alt::VehicleModelInfo *cxxbridge1$shared_ptr$alt$VehicleModelInfo$uninit(::std::shared_ptr<::alt::VehicleModelInfo> *ptr) noexcept {
  ::alt::VehicleModelInfo *uninit = reinterpret_cast<::alt::VehicleModelInfo *>(new ::rust::MaybeUninit<::alt::VehicleModelInfo>);
  ::new (ptr) ::std::shared_ptr<::alt::VehicleModelInfo>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$VehicleModelInfo$clone(::std::shared_ptr<::alt::VehicleModelInfo> const &self, ::std::shared_ptr<::alt::VehicleModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::VehicleModelInfo>(self);
}
::alt::VehicleModelInfo const *cxxbridge1$shared_ptr$alt$VehicleModelInfo$get(::std::shared_ptr<::alt::VehicleModelInfo> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$VehicleModelInfo$drop(::std::shared_ptr<::alt::VehicleModelInfo> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::VehicleModelInfo>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::VehicleModelInfo>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$VehicleModelInfo$null(::std::weak_ptr<::alt::VehicleModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::VehicleModelInfo>();
}
void cxxbridge1$weak_ptr$alt$VehicleModelInfo$clone(::std::weak_ptr<::alt::VehicleModelInfo> const &self, ::std::weak_ptr<::alt::VehicleModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::VehicleModelInfo>(self);
}
void cxxbridge1$weak_ptr$alt$VehicleModelInfo$downgrade(::std::shared_ptr<::alt::VehicleModelInfo> const &shared, ::std::weak_ptr<::alt::VehicleModelInfo> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::VehicleModelInfo>(shared);
}
void cxxbridge1$weak_ptr$alt$VehicleModelInfo$upgrade(::std::weak_ptr<::alt::VehicleModelInfo> const &weak, ::std::shared_ptr<::alt::VehicleModelInfo> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::VehicleModelInfo>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$VehicleModelInfo$drop(::std::weak_ptr<::alt::VehicleModelInfo> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::VehicleModelInfo> *cxxbridge1$std$vector$alt$VehicleModelInfo$new() noexcept {
  return new ::std::vector<::alt::VehicleModelInfo>();
}
::std::size_t cxxbridge1$std$vector$alt$VehicleModelInfo$size(::std::vector<::alt::VehicleModelInfo> const &s) noexcept {
  return s.size();
}
::alt::VehicleModelInfo *cxxbridge1$std$vector$alt$VehicleModelInfo$get_unchecked(::std::vector<::alt::VehicleModelInfo> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$VehicleModelInfo$push_back(::std::vector<::alt::VehicleModelInfo> *v, ::alt::VehicleModelInfo *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$VehicleModelInfo$pop_back(::std::vector<::alt::VehicleModelInfo> *v, ::alt::VehicleModelInfo *out) noexcept {
  ::new (out) ::alt::VehicleModelInfo(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::VehicleModelInfo>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::VehicleModelInfo>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$VehicleModelInfo$null(::std::unique_ptr<::std::vector<::alt::VehicleModelInfo>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::VehicleModelInfo>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$VehicleModelInfo$raw(::std::unique_ptr<::std::vector<::alt::VehicleModelInfo>> *ptr, ::std::vector<::alt::VehicleModelInfo> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::VehicleModelInfo>>(raw);
}
::std::vector<::alt::VehicleModelInfo> const *cxxbridge1$unique_ptr$std$vector$alt$VehicleModelInfo$get(::std::unique_ptr<::std::vector<::alt::VehicleModelInfo>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::VehicleModelInfo> *cxxbridge1$unique_ptr$std$vector$alt$VehicleModelInfo$release(::std::unique_ptr<::std::vector<::alt::VehicleModelInfo>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$VehicleModelInfo$drop(::std::unique_ptr<::std::vector<::alt::VehicleModelInfo>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::PedModelInfo>::value, "definition of PedModelInfo is required");
static_assert(sizeof(::std::unique_ptr<::alt::PedModelInfo>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::PedModelInfo>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$PedModelInfo$null(::std::unique_ptr<::alt::PedModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::PedModelInfo>();
}
::alt::PedModelInfo *cxxbridge1$unique_ptr$alt$PedModelInfo$uninit(::std::unique_ptr<::alt::PedModelInfo> *ptr) noexcept {
  ::alt::PedModelInfo *uninit = reinterpret_cast<::alt::PedModelInfo *>(new ::rust::MaybeUninit<::alt::PedModelInfo>);
  ::new (ptr) ::std::unique_ptr<::alt::PedModelInfo>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$PedModelInfo$raw(::std::unique_ptr<::alt::PedModelInfo> *ptr, ::alt::PedModelInfo *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::PedModelInfo>(raw);
}
::alt::PedModelInfo const *cxxbridge1$unique_ptr$alt$PedModelInfo$get(::std::unique_ptr<::alt::PedModelInfo> const &ptr) noexcept {
  return ptr.get();
}
::alt::PedModelInfo *cxxbridge1$unique_ptr$alt$PedModelInfo$release(::std::unique_ptr<::alt::PedModelInfo> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$PedModelInfo$drop(::std::unique_ptr<::alt::PedModelInfo> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::PedModelInfo>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::PedModelInfo>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::PedModelInfo>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$PedModelInfo$null(::std::shared_ptr<::alt::PedModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::PedModelInfo>();
}
::alt::PedModelInfo *cxxbridge1$shared_ptr$alt$PedModelInfo$uninit(::std::shared_ptr<::alt::PedModelInfo> *ptr) noexcept {
  ::alt::PedModelInfo *uninit = reinterpret_cast<::alt::PedModelInfo *>(new ::rust::MaybeUninit<::alt::PedModelInfo>);
  ::new (ptr) ::std::shared_ptr<::alt::PedModelInfo>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$PedModelInfo$clone(::std::shared_ptr<::alt::PedModelInfo> const &self, ::std::shared_ptr<::alt::PedModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::PedModelInfo>(self);
}
::alt::PedModelInfo const *cxxbridge1$shared_ptr$alt$PedModelInfo$get(::std::shared_ptr<::alt::PedModelInfo> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$PedModelInfo$drop(::std::shared_ptr<::alt::PedModelInfo> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::PedModelInfo>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::PedModelInfo>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$PedModelInfo$null(::std::weak_ptr<::alt::PedModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::PedModelInfo>();
}
void cxxbridge1$weak_ptr$alt$PedModelInfo$clone(::std::weak_ptr<::alt::PedModelInfo> const &self, ::std::weak_ptr<::alt::PedModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::PedModelInfo>(self);
}
void cxxbridge1$weak_ptr$alt$PedModelInfo$downgrade(::std::shared_ptr<::alt::PedModelInfo> const &shared, ::std::weak_ptr<::alt::PedModelInfo> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::PedModelInfo>(shared);
}
void cxxbridge1$weak_ptr$alt$PedModelInfo$upgrade(::std::weak_ptr<::alt::PedModelInfo> const &weak, ::std::shared_ptr<::alt::PedModelInfo> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::PedModelInfo>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$PedModelInfo$drop(::std::weak_ptr<::alt::PedModelInfo> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::PedModelInfo> *cxxbridge1$std$vector$alt$PedModelInfo$new() noexcept {
  return new ::std::vector<::alt::PedModelInfo>();
}
::std::size_t cxxbridge1$std$vector$alt$PedModelInfo$size(::std::vector<::alt::PedModelInfo> const &s) noexcept {
  return s.size();
}
::alt::PedModelInfo *cxxbridge1$std$vector$alt$PedModelInfo$get_unchecked(::std::vector<::alt::PedModelInfo> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$PedModelInfo$push_back(::std::vector<::alt::PedModelInfo> *v, ::alt::PedModelInfo *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$PedModelInfo$pop_back(::std::vector<::alt::PedModelInfo> *v, ::alt::PedModelInfo *out) noexcept {
  ::new (out) ::alt::PedModelInfo(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::PedModelInfo>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::PedModelInfo>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$PedModelInfo$null(::std::unique_ptr<::std::vector<::alt::PedModelInfo>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::PedModelInfo>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$PedModelInfo$raw(::std::unique_ptr<::std::vector<::alt::PedModelInfo>> *ptr, ::std::vector<::alt::PedModelInfo> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::PedModelInfo>>(raw);
}
::std::vector<::alt::PedModelInfo> const *cxxbridge1$unique_ptr$std$vector$alt$PedModelInfo$get(::std::unique_ptr<::std::vector<::alt::PedModelInfo>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::PedModelInfo> *cxxbridge1$unique_ptr$std$vector$alt$PedModelInfo$release(::std::unique_ptr<::std::vector<::alt::PedModelInfo>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$PedModelInfo$drop(::std::unique_ptr<::std::vector<::alt::PedModelInfo>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::WeaponModelInfo>::value, "definition of WeaponModelInfo is required");
static_assert(sizeof(::std::unique_ptr<::alt::WeaponModelInfo>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::WeaponModelInfo>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$WeaponModelInfo$null(::std::unique_ptr<::alt::WeaponModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::WeaponModelInfo>();
}
::alt::WeaponModelInfo *cxxbridge1$unique_ptr$alt$WeaponModelInfo$uninit(::std::unique_ptr<::alt::WeaponModelInfo> *ptr) noexcept {
  ::alt::WeaponModelInfo *uninit = reinterpret_cast<::alt::WeaponModelInfo *>(new ::rust::MaybeUninit<::alt::WeaponModelInfo>);
  ::new (ptr) ::std::unique_ptr<::alt::WeaponModelInfo>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$WeaponModelInfo$raw(::std::unique_ptr<::alt::WeaponModelInfo> *ptr, ::alt::WeaponModelInfo *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::WeaponModelInfo>(raw);
}
::alt::WeaponModelInfo const *cxxbridge1$unique_ptr$alt$WeaponModelInfo$get(::std::unique_ptr<::alt::WeaponModelInfo> const &ptr) noexcept {
  return ptr.get();
}
::alt::WeaponModelInfo *cxxbridge1$unique_ptr$alt$WeaponModelInfo$release(::std::unique_ptr<::alt::WeaponModelInfo> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$WeaponModelInfo$drop(::std::unique_ptr<::alt::WeaponModelInfo> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::WeaponModelInfo>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::WeaponModelInfo>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::WeaponModelInfo>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$WeaponModelInfo$null(::std::shared_ptr<::alt::WeaponModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::WeaponModelInfo>();
}
::alt::WeaponModelInfo *cxxbridge1$shared_ptr$alt$WeaponModelInfo$uninit(::std::shared_ptr<::alt::WeaponModelInfo> *ptr) noexcept {
  ::alt::WeaponModelInfo *uninit = reinterpret_cast<::alt::WeaponModelInfo *>(new ::rust::MaybeUninit<::alt::WeaponModelInfo>);
  ::new (ptr) ::std::shared_ptr<::alt::WeaponModelInfo>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$WeaponModelInfo$clone(::std::shared_ptr<::alt::WeaponModelInfo> const &self, ::std::shared_ptr<::alt::WeaponModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::WeaponModelInfo>(self);
}
::alt::WeaponModelInfo const *cxxbridge1$shared_ptr$alt$WeaponModelInfo$get(::std::shared_ptr<::alt::WeaponModelInfo> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$WeaponModelInfo$drop(::std::shared_ptr<::alt::WeaponModelInfo> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::WeaponModelInfo>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::WeaponModelInfo>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$WeaponModelInfo$null(::std::weak_ptr<::alt::WeaponModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::WeaponModelInfo>();
}
void cxxbridge1$weak_ptr$alt$WeaponModelInfo$clone(::std::weak_ptr<::alt::WeaponModelInfo> const &self, ::std::weak_ptr<::alt::WeaponModelInfo> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::WeaponModelInfo>(self);
}
void cxxbridge1$weak_ptr$alt$WeaponModelInfo$downgrade(::std::shared_ptr<::alt::WeaponModelInfo> const &shared, ::std::weak_ptr<::alt::WeaponModelInfo> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::WeaponModelInfo>(shared);
}
void cxxbridge1$weak_ptr$alt$WeaponModelInfo$upgrade(::std::weak_ptr<::alt::WeaponModelInfo> const &weak, ::std::shared_ptr<::alt::WeaponModelInfo> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::WeaponModelInfo>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$WeaponModelInfo$drop(::std::weak_ptr<::alt::WeaponModelInfo> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::WeaponModelInfo> *cxxbridge1$std$vector$alt$WeaponModelInfo$new() noexcept {
  return new ::std::vector<::alt::WeaponModelInfo>();
}
::std::size_t cxxbridge1$std$vector$alt$WeaponModelInfo$size(::std::vector<::alt::WeaponModelInfo> const &s) noexcept {
  return s.size();
}
::alt::WeaponModelInfo *cxxbridge1$std$vector$alt$WeaponModelInfo$get_unchecked(::std::vector<::alt::WeaponModelInfo> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$WeaponModelInfo$push_back(::std::vector<::alt::WeaponModelInfo> *v, ::alt::WeaponModelInfo *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$WeaponModelInfo$pop_back(::std::vector<::alt::WeaponModelInfo> *v, ::alt::WeaponModelInfo *out) noexcept {
  ::new (out) ::alt::WeaponModelInfo(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::WeaponModelInfo>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::WeaponModelInfo>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$WeaponModelInfo$null(::std::unique_ptr<::std::vector<::alt::WeaponModelInfo>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::WeaponModelInfo>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$WeaponModelInfo$raw(::std::unique_ptr<::std::vector<::alt::WeaponModelInfo>> *ptr, ::std::vector<::alt::WeaponModelInfo> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::WeaponModelInfo>>(raw);
}
::std::vector<::alt::WeaponModelInfo> const *cxxbridge1$unique_ptr$std$vector$alt$WeaponModelInfo$get(::std::unique_ptr<::std::vector<::alt::WeaponModelInfo>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::WeaponModelInfo> *cxxbridge1$unique_ptr$std$vector$alt$WeaponModelInfo$release(::std::unique_ptr<::std::vector<::alt::WeaponModelInfo>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$WeaponModelInfo$drop(::std::unique_ptr<::std::vector<::alt::WeaponModelInfo>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::BaseObjectPtrWrapper>::value, "definition of BaseObjectPtrWrapper is required");
static_assert(sizeof(::std::unique_ptr<::BaseObjectPtrWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::BaseObjectPtrWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$BaseObjectPtrWrapper$null(::std::unique_ptr<::BaseObjectPtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::BaseObjectPtrWrapper>();
}
::BaseObjectPtrWrapper *cxxbridge1$unique_ptr$BaseObjectPtrWrapper$uninit(::std::unique_ptr<::BaseObjectPtrWrapper> *ptr) noexcept {
  ::BaseObjectPtrWrapper *uninit = reinterpret_cast<::BaseObjectPtrWrapper *>(new ::rust::MaybeUninit<::BaseObjectPtrWrapper>);
  ::new (ptr) ::std::unique_ptr<::BaseObjectPtrWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$BaseObjectPtrWrapper$raw(::std::unique_ptr<::BaseObjectPtrWrapper> *ptr, ::BaseObjectPtrWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::BaseObjectPtrWrapper>(raw);
}
::BaseObjectPtrWrapper const *cxxbridge1$unique_ptr$BaseObjectPtrWrapper$get(::std::unique_ptr<::BaseObjectPtrWrapper> const &ptr) noexcept {
  return ptr.get();
}
::BaseObjectPtrWrapper *cxxbridge1$unique_ptr$BaseObjectPtrWrapper$release(::std::unique_ptr<::BaseObjectPtrWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$BaseObjectPtrWrapper$drop(::std::unique_ptr<::BaseObjectPtrWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::BaseObjectPtrWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::BaseObjectPtrWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::BaseObjectPtrWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$BaseObjectPtrWrapper$null(::std::shared_ptr<::BaseObjectPtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::BaseObjectPtrWrapper>();
}
::BaseObjectPtrWrapper *cxxbridge1$shared_ptr$BaseObjectPtrWrapper$uninit(::std::shared_ptr<::BaseObjectPtrWrapper> *ptr) noexcept {
  ::BaseObjectPtrWrapper *uninit = reinterpret_cast<::BaseObjectPtrWrapper *>(new ::rust::MaybeUninit<::BaseObjectPtrWrapper>);
  ::new (ptr) ::std::shared_ptr<::BaseObjectPtrWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$BaseObjectPtrWrapper$clone(::std::shared_ptr<::BaseObjectPtrWrapper> const &self, ::std::shared_ptr<::BaseObjectPtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::BaseObjectPtrWrapper>(self);
}
::BaseObjectPtrWrapper const *cxxbridge1$shared_ptr$BaseObjectPtrWrapper$get(::std::shared_ptr<::BaseObjectPtrWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$BaseObjectPtrWrapper$drop(::std::shared_ptr<::BaseObjectPtrWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::BaseObjectPtrWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::BaseObjectPtrWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$BaseObjectPtrWrapper$null(::std::weak_ptr<::BaseObjectPtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::BaseObjectPtrWrapper>();
}
void cxxbridge1$weak_ptr$BaseObjectPtrWrapper$clone(::std::weak_ptr<::BaseObjectPtrWrapper> const &self, ::std::weak_ptr<::BaseObjectPtrWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::BaseObjectPtrWrapper>(self);
}
void cxxbridge1$weak_ptr$BaseObjectPtrWrapper$downgrade(::std::shared_ptr<::BaseObjectPtrWrapper> const &shared, ::std::weak_ptr<::BaseObjectPtrWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::BaseObjectPtrWrapper>(shared);
}
void cxxbridge1$weak_ptr$BaseObjectPtrWrapper$upgrade(::std::weak_ptr<::BaseObjectPtrWrapper> const &weak, ::std::shared_ptr<::BaseObjectPtrWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::BaseObjectPtrWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$BaseObjectPtrWrapper$drop(::std::weak_ptr<::BaseObjectPtrWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::BaseObjectPtrWrapper> *cxxbridge1$std$vector$BaseObjectPtrWrapper$new() noexcept {
  return new ::std::vector<::BaseObjectPtrWrapper>();
}
::std::size_t cxxbridge1$std$vector$BaseObjectPtrWrapper$size(::std::vector<::BaseObjectPtrWrapper> const &s) noexcept {
  return s.size();
}
::BaseObjectPtrWrapper *cxxbridge1$std$vector$BaseObjectPtrWrapper$get_unchecked(::std::vector<::BaseObjectPtrWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$BaseObjectPtrWrapper$push_back(::std::vector<::BaseObjectPtrWrapper> *v, ::BaseObjectPtrWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$BaseObjectPtrWrapper$pop_back(::std::vector<::BaseObjectPtrWrapper> *v, ::BaseObjectPtrWrapper *out) noexcept {
  ::new (out) ::BaseObjectPtrWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$BaseObjectPtrWrapper$null(::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$BaseObjectPtrWrapper$raw(::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>> *ptr, ::std::vector<::BaseObjectPtrWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>>(raw);
}
::std::vector<::BaseObjectPtrWrapper> const *cxxbridge1$unique_ptr$std$vector$BaseObjectPtrWrapper$get(::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::BaseObjectPtrWrapper> *cxxbridge1$unique_ptr$std$vector$BaseObjectPtrWrapper$release(::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$BaseObjectPtrWrapper$drop(::std::unique_ptr<::std::vector<::BaseObjectPtrWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::Cloth>::value, "definition of Cloth is required");
static_assert(sizeof(::std::unique_ptr<::alt::Cloth>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::Cloth>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$Cloth$null(::std::unique_ptr<::alt::Cloth> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::Cloth>();
}
::alt::Cloth *cxxbridge1$unique_ptr$alt$Cloth$uninit(::std::unique_ptr<::alt::Cloth> *ptr) noexcept {
  ::alt::Cloth *uninit = reinterpret_cast<::alt::Cloth *>(new ::rust::MaybeUninit<::alt::Cloth>);
  ::new (ptr) ::std::unique_ptr<::alt::Cloth>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$Cloth$raw(::std::unique_ptr<::alt::Cloth> *ptr, ::alt::Cloth *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::Cloth>(raw);
}
::alt::Cloth const *cxxbridge1$unique_ptr$alt$Cloth$get(::std::unique_ptr<::alt::Cloth> const &ptr) noexcept {
  return ptr.get();
}
::alt::Cloth *cxxbridge1$unique_ptr$alt$Cloth$release(::std::unique_ptr<::alt::Cloth> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$Cloth$drop(::std::unique_ptr<::alt::Cloth> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::Cloth>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::Cloth>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::Cloth>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$Cloth$null(::std::shared_ptr<::alt::Cloth> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::Cloth>();
}
::alt::Cloth *cxxbridge1$shared_ptr$alt$Cloth$uninit(::std::shared_ptr<::alt::Cloth> *ptr) noexcept {
  ::alt::Cloth *uninit = reinterpret_cast<::alt::Cloth *>(new ::rust::MaybeUninit<::alt::Cloth>);
  ::new (ptr) ::std::shared_ptr<::alt::Cloth>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$Cloth$clone(::std::shared_ptr<::alt::Cloth> const &self, ::std::shared_ptr<::alt::Cloth> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::Cloth>(self);
}
::alt::Cloth const *cxxbridge1$shared_ptr$alt$Cloth$get(::std::shared_ptr<::alt::Cloth> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$Cloth$drop(::std::shared_ptr<::alt::Cloth> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::Cloth>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::Cloth>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$Cloth$null(::std::weak_ptr<::alt::Cloth> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::Cloth>();
}
void cxxbridge1$weak_ptr$alt$Cloth$clone(::std::weak_ptr<::alt::Cloth> const &self, ::std::weak_ptr<::alt::Cloth> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::Cloth>(self);
}
void cxxbridge1$weak_ptr$alt$Cloth$downgrade(::std::shared_ptr<::alt::Cloth> const &shared, ::std::weak_ptr<::alt::Cloth> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::Cloth>(shared);
}
void cxxbridge1$weak_ptr$alt$Cloth$upgrade(::std::weak_ptr<::alt::Cloth> const &weak, ::std::shared_ptr<::alt::Cloth> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::Cloth>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$Cloth$drop(::std::weak_ptr<::alt::Cloth> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::Cloth> *cxxbridge1$std$vector$alt$Cloth$new() noexcept {
  return new ::std::vector<::alt::Cloth>();
}
::std::size_t cxxbridge1$std$vector$alt$Cloth$size(::std::vector<::alt::Cloth> const &s) noexcept {
  return s.size();
}
::alt::Cloth *cxxbridge1$std$vector$alt$Cloth$get_unchecked(::std::vector<::alt::Cloth> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$Cloth$push_back(::std::vector<::alt::Cloth> *v, ::alt::Cloth *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$Cloth$pop_back(::std::vector<::alt::Cloth> *v, ::alt::Cloth *out) noexcept {
  ::new (out) ::alt::Cloth(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::Cloth>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::Cloth>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$Cloth$null(::std::unique_ptr<::std::vector<::alt::Cloth>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::Cloth>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$Cloth$raw(::std::unique_ptr<::std::vector<::alt::Cloth>> *ptr, ::std::vector<::alt::Cloth> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::Cloth>>(raw);
}
::std::vector<::alt::Cloth> const *cxxbridge1$unique_ptr$std$vector$alt$Cloth$get(::std::unique_ptr<::std::vector<::alt::Cloth>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::Cloth> *cxxbridge1$unique_ptr$std$vector$alt$Cloth$release(::std::unique_ptr<::std::vector<::alt::Cloth>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$Cloth$drop(::std::unique_ptr<::std::vector<::alt::Cloth>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::DlcCloth>::value, "definition of DlcCloth is required");
static_assert(sizeof(::std::unique_ptr<::alt::DlcCloth>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::DlcCloth>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$DlcCloth$null(::std::unique_ptr<::alt::DlcCloth> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::DlcCloth>();
}
::alt::DlcCloth *cxxbridge1$unique_ptr$alt$DlcCloth$uninit(::std::unique_ptr<::alt::DlcCloth> *ptr) noexcept {
  ::alt::DlcCloth *uninit = reinterpret_cast<::alt::DlcCloth *>(new ::rust::MaybeUninit<::alt::DlcCloth>);
  ::new (ptr) ::std::unique_ptr<::alt::DlcCloth>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$DlcCloth$raw(::std::unique_ptr<::alt::DlcCloth> *ptr, ::alt::DlcCloth *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::DlcCloth>(raw);
}
::alt::DlcCloth const *cxxbridge1$unique_ptr$alt$DlcCloth$get(::std::unique_ptr<::alt::DlcCloth> const &ptr) noexcept {
  return ptr.get();
}
::alt::DlcCloth *cxxbridge1$unique_ptr$alt$DlcCloth$release(::std::unique_ptr<::alt::DlcCloth> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$DlcCloth$drop(::std::unique_ptr<::alt::DlcCloth> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::DlcCloth>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::DlcCloth>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::DlcCloth>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$DlcCloth$null(::std::shared_ptr<::alt::DlcCloth> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::DlcCloth>();
}
::alt::DlcCloth *cxxbridge1$shared_ptr$alt$DlcCloth$uninit(::std::shared_ptr<::alt::DlcCloth> *ptr) noexcept {
  ::alt::DlcCloth *uninit = reinterpret_cast<::alt::DlcCloth *>(new ::rust::MaybeUninit<::alt::DlcCloth>);
  ::new (ptr) ::std::shared_ptr<::alt::DlcCloth>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$DlcCloth$clone(::std::shared_ptr<::alt::DlcCloth> const &self, ::std::shared_ptr<::alt::DlcCloth> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::DlcCloth>(self);
}
::alt::DlcCloth const *cxxbridge1$shared_ptr$alt$DlcCloth$get(::std::shared_ptr<::alt::DlcCloth> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$DlcCloth$drop(::std::shared_ptr<::alt::DlcCloth> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::DlcCloth>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::DlcCloth>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$DlcCloth$null(::std::weak_ptr<::alt::DlcCloth> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::DlcCloth>();
}
void cxxbridge1$weak_ptr$alt$DlcCloth$clone(::std::weak_ptr<::alt::DlcCloth> const &self, ::std::weak_ptr<::alt::DlcCloth> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::DlcCloth>(self);
}
void cxxbridge1$weak_ptr$alt$DlcCloth$downgrade(::std::shared_ptr<::alt::DlcCloth> const &shared, ::std::weak_ptr<::alt::DlcCloth> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::DlcCloth>(shared);
}
void cxxbridge1$weak_ptr$alt$DlcCloth$upgrade(::std::weak_ptr<::alt::DlcCloth> const &weak, ::std::shared_ptr<::alt::DlcCloth> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::DlcCloth>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$DlcCloth$drop(::std::weak_ptr<::alt::DlcCloth> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::DlcCloth> *cxxbridge1$std$vector$alt$DlcCloth$new() noexcept {
  return new ::std::vector<::alt::DlcCloth>();
}
::std::size_t cxxbridge1$std$vector$alt$DlcCloth$size(::std::vector<::alt::DlcCloth> const &s) noexcept {
  return s.size();
}
::alt::DlcCloth *cxxbridge1$std$vector$alt$DlcCloth$get_unchecked(::std::vector<::alt::DlcCloth> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$DlcCloth$push_back(::std::vector<::alt::DlcCloth> *v, ::alt::DlcCloth *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$DlcCloth$pop_back(::std::vector<::alt::DlcCloth> *v, ::alt::DlcCloth *out) noexcept {
  ::new (out) ::alt::DlcCloth(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::DlcCloth>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::DlcCloth>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$DlcCloth$null(::std::unique_ptr<::std::vector<::alt::DlcCloth>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::DlcCloth>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$DlcCloth$raw(::std::unique_ptr<::std::vector<::alt::DlcCloth>> *ptr, ::std::vector<::alt::DlcCloth> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::DlcCloth>>(raw);
}
::std::vector<::alt::DlcCloth> const *cxxbridge1$unique_ptr$std$vector$alt$DlcCloth$get(::std::unique_ptr<::std::vector<::alt::DlcCloth>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::DlcCloth> *cxxbridge1$unique_ptr$std$vector$alt$DlcCloth$release(::std::unique_ptr<::std::vector<::alt::DlcCloth>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$DlcCloth$drop(::std::unique_ptr<::std::vector<::alt::DlcCloth>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::Prop>::value, "definition of Prop is required");
static_assert(sizeof(::std::unique_ptr<::alt::Prop>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::Prop>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$Prop$null(::std::unique_ptr<::alt::Prop> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::Prop>();
}
::alt::Prop *cxxbridge1$unique_ptr$alt$Prop$uninit(::std::unique_ptr<::alt::Prop> *ptr) noexcept {
  ::alt::Prop *uninit = reinterpret_cast<::alt::Prop *>(new ::rust::MaybeUninit<::alt::Prop>);
  ::new (ptr) ::std::unique_ptr<::alt::Prop>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$Prop$raw(::std::unique_ptr<::alt::Prop> *ptr, ::alt::Prop *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::Prop>(raw);
}
::alt::Prop const *cxxbridge1$unique_ptr$alt$Prop$get(::std::unique_ptr<::alt::Prop> const &ptr) noexcept {
  return ptr.get();
}
::alt::Prop *cxxbridge1$unique_ptr$alt$Prop$release(::std::unique_ptr<::alt::Prop> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$Prop$drop(::std::unique_ptr<::alt::Prop> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::Prop>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::Prop>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::Prop>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$Prop$null(::std::shared_ptr<::alt::Prop> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::Prop>();
}
::alt::Prop *cxxbridge1$shared_ptr$alt$Prop$uninit(::std::shared_ptr<::alt::Prop> *ptr) noexcept {
  ::alt::Prop *uninit = reinterpret_cast<::alt::Prop *>(new ::rust::MaybeUninit<::alt::Prop>);
  ::new (ptr) ::std::shared_ptr<::alt::Prop>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$Prop$clone(::std::shared_ptr<::alt::Prop> const &self, ::std::shared_ptr<::alt::Prop> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::Prop>(self);
}
::alt::Prop const *cxxbridge1$shared_ptr$alt$Prop$get(::std::shared_ptr<::alt::Prop> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$Prop$drop(::std::shared_ptr<::alt::Prop> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::Prop>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::Prop>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$Prop$null(::std::weak_ptr<::alt::Prop> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::Prop>();
}
void cxxbridge1$weak_ptr$alt$Prop$clone(::std::weak_ptr<::alt::Prop> const &self, ::std::weak_ptr<::alt::Prop> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::Prop>(self);
}
void cxxbridge1$weak_ptr$alt$Prop$downgrade(::std::shared_ptr<::alt::Prop> const &shared, ::std::weak_ptr<::alt::Prop> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::Prop>(shared);
}
void cxxbridge1$weak_ptr$alt$Prop$upgrade(::std::weak_ptr<::alt::Prop> const &weak, ::std::shared_ptr<::alt::Prop> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::Prop>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$Prop$drop(::std::weak_ptr<::alt::Prop> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::Prop> *cxxbridge1$std$vector$alt$Prop$new() noexcept {
  return new ::std::vector<::alt::Prop>();
}
::std::size_t cxxbridge1$std$vector$alt$Prop$size(::std::vector<::alt::Prop> const &s) noexcept {
  return s.size();
}
::alt::Prop *cxxbridge1$std$vector$alt$Prop$get_unchecked(::std::vector<::alt::Prop> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$Prop$push_back(::std::vector<::alt::Prop> *v, ::alt::Prop *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$Prop$pop_back(::std::vector<::alt::Prop> *v, ::alt::Prop *out) noexcept {
  ::new (out) ::alt::Prop(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::Prop>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::Prop>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$Prop$null(::std::unique_ptr<::std::vector<::alt::Prop>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::Prop>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$Prop$raw(::std::unique_ptr<::std::vector<::alt::Prop>> *ptr, ::std::vector<::alt::Prop> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::Prop>>(raw);
}
::std::vector<::alt::Prop> const *cxxbridge1$unique_ptr$std$vector$alt$Prop$get(::std::unique_ptr<::std::vector<::alt::Prop>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::Prop> *cxxbridge1$unique_ptr$std$vector$alt$Prop$release(::std::unique_ptr<::std::vector<::alt::Prop>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$Prop$drop(::std::unique_ptr<::std::vector<::alt::Prop>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::DlcProp>::value, "definition of DlcProp is required");
static_assert(sizeof(::std::unique_ptr<::alt::DlcProp>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::DlcProp>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$DlcProp$null(::std::unique_ptr<::alt::DlcProp> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::DlcProp>();
}
::alt::DlcProp *cxxbridge1$unique_ptr$alt$DlcProp$uninit(::std::unique_ptr<::alt::DlcProp> *ptr) noexcept {
  ::alt::DlcProp *uninit = reinterpret_cast<::alt::DlcProp *>(new ::rust::MaybeUninit<::alt::DlcProp>);
  ::new (ptr) ::std::unique_ptr<::alt::DlcProp>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$DlcProp$raw(::std::unique_ptr<::alt::DlcProp> *ptr, ::alt::DlcProp *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::DlcProp>(raw);
}
::alt::DlcProp const *cxxbridge1$unique_ptr$alt$DlcProp$get(::std::unique_ptr<::alt::DlcProp> const &ptr) noexcept {
  return ptr.get();
}
::alt::DlcProp *cxxbridge1$unique_ptr$alt$DlcProp$release(::std::unique_ptr<::alt::DlcProp> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$DlcProp$drop(::std::unique_ptr<::alt::DlcProp> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::DlcProp>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::DlcProp>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::DlcProp>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$DlcProp$null(::std::shared_ptr<::alt::DlcProp> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::DlcProp>();
}
::alt::DlcProp *cxxbridge1$shared_ptr$alt$DlcProp$uninit(::std::shared_ptr<::alt::DlcProp> *ptr) noexcept {
  ::alt::DlcProp *uninit = reinterpret_cast<::alt::DlcProp *>(new ::rust::MaybeUninit<::alt::DlcProp>);
  ::new (ptr) ::std::shared_ptr<::alt::DlcProp>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$DlcProp$clone(::std::shared_ptr<::alt::DlcProp> const &self, ::std::shared_ptr<::alt::DlcProp> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::DlcProp>(self);
}
::alt::DlcProp const *cxxbridge1$shared_ptr$alt$DlcProp$get(::std::shared_ptr<::alt::DlcProp> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$DlcProp$drop(::std::shared_ptr<::alt::DlcProp> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::DlcProp>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::DlcProp>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$DlcProp$null(::std::weak_ptr<::alt::DlcProp> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::DlcProp>();
}
void cxxbridge1$weak_ptr$alt$DlcProp$clone(::std::weak_ptr<::alt::DlcProp> const &self, ::std::weak_ptr<::alt::DlcProp> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::DlcProp>(self);
}
void cxxbridge1$weak_ptr$alt$DlcProp$downgrade(::std::shared_ptr<::alt::DlcProp> const &shared, ::std::weak_ptr<::alt::DlcProp> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::DlcProp>(shared);
}
void cxxbridge1$weak_ptr$alt$DlcProp$upgrade(::std::weak_ptr<::alt::DlcProp> const &weak, ::std::shared_ptr<::alt::DlcProp> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::DlcProp>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$DlcProp$drop(::std::weak_ptr<::alt::DlcProp> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::DlcProp> *cxxbridge1$std$vector$alt$DlcProp$new() noexcept {
  return new ::std::vector<::alt::DlcProp>();
}
::std::size_t cxxbridge1$std$vector$alt$DlcProp$size(::std::vector<::alt::DlcProp> const &s) noexcept {
  return s.size();
}
::alt::DlcProp *cxxbridge1$std$vector$alt$DlcProp$get_unchecked(::std::vector<::alt::DlcProp> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$DlcProp$push_back(::std::vector<::alt::DlcProp> *v, ::alt::DlcProp *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$DlcProp$pop_back(::std::vector<::alt::DlcProp> *v, ::alt::DlcProp *out) noexcept {
  ::new (out) ::alt::DlcProp(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::DlcProp>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::DlcProp>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$DlcProp$null(::std::unique_ptr<::std::vector<::alt::DlcProp>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::DlcProp>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$DlcProp$raw(::std::unique_ptr<::std::vector<::alt::DlcProp>> *ptr, ::std::vector<::alt::DlcProp> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::DlcProp>>(raw);
}
::std::vector<::alt::DlcProp> const *cxxbridge1$unique_ptr$std$vector$alt$DlcProp$get(::std::unique_ptr<::std::vector<::alt::DlcProp>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::DlcProp> *cxxbridge1$unique_ptr$std$vector$alt$DlcProp$release(::std::unique_ptr<::std::vector<::alt::DlcProp>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$DlcProp$drop(::std::unique_ptr<::std::vector<::alt::DlcProp>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::HeadOverlay>::value, "definition of HeadOverlay is required");
static_assert(sizeof(::std::unique_ptr<::alt::HeadOverlay>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::HeadOverlay>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$HeadOverlay$null(::std::unique_ptr<::alt::HeadOverlay> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::HeadOverlay>();
}
::alt::HeadOverlay *cxxbridge1$unique_ptr$alt$HeadOverlay$uninit(::std::unique_ptr<::alt::HeadOverlay> *ptr) noexcept {
  ::alt::HeadOverlay *uninit = reinterpret_cast<::alt::HeadOverlay *>(new ::rust::MaybeUninit<::alt::HeadOverlay>);
  ::new (ptr) ::std::unique_ptr<::alt::HeadOverlay>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$HeadOverlay$raw(::std::unique_ptr<::alt::HeadOverlay> *ptr, ::alt::HeadOverlay *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::HeadOverlay>(raw);
}
::alt::HeadOverlay const *cxxbridge1$unique_ptr$alt$HeadOverlay$get(::std::unique_ptr<::alt::HeadOverlay> const &ptr) noexcept {
  return ptr.get();
}
::alt::HeadOverlay *cxxbridge1$unique_ptr$alt$HeadOverlay$release(::std::unique_ptr<::alt::HeadOverlay> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$HeadOverlay$drop(::std::unique_ptr<::alt::HeadOverlay> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::HeadOverlay>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::HeadOverlay>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::HeadOverlay>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$HeadOverlay$null(::std::shared_ptr<::alt::HeadOverlay> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::HeadOverlay>();
}
::alt::HeadOverlay *cxxbridge1$shared_ptr$alt$HeadOverlay$uninit(::std::shared_ptr<::alt::HeadOverlay> *ptr) noexcept {
  ::alt::HeadOverlay *uninit = reinterpret_cast<::alt::HeadOverlay *>(new ::rust::MaybeUninit<::alt::HeadOverlay>);
  ::new (ptr) ::std::shared_ptr<::alt::HeadOverlay>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$HeadOverlay$clone(::std::shared_ptr<::alt::HeadOverlay> const &self, ::std::shared_ptr<::alt::HeadOverlay> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::HeadOverlay>(self);
}
::alt::HeadOverlay const *cxxbridge1$shared_ptr$alt$HeadOverlay$get(::std::shared_ptr<::alt::HeadOverlay> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$HeadOverlay$drop(::std::shared_ptr<::alt::HeadOverlay> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::HeadOverlay>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::HeadOverlay>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$HeadOverlay$null(::std::weak_ptr<::alt::HeadOverlay> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::HeadOverlay>();
}
void cxxbridge1$weak_ptr$alt$HeadOverlay$clone(::std::weak_ptr<::alt::HeadOverlay> const &self, ::std::weak_ptr<::alt::HeadOverlay> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::HeadOverlay>(self);
}
void cxxbridge1$weak_ptr$alt$HeadOverlay$downgrade(::std::shared_ptr<::alt::HeadOverlay> const &shared, ::std::weak_ptr<::alt::HeadOverlay> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::HeadOverlay>(shared);
}
void cxxbridge1$weak_ptr$alt$HeadOverlay$upgrade(::std::weak_ptr<::alt::HeadOverlay> const &weak, ::std::shared_ptr<::alt::HeadOverlay> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::HeadOverlay>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$HeadOverlay$drop(::std::weak_ptr<::alt::HeadOverlay> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::HeadOverlay> *cxxbridge1$std$vector$alt$HeadOverlay$new() noexcept {
  return new ::std::vector<::alt::HeadOverlay>();
}
::std::size_t cxxbridge1$std$vector$alt$HeadOverlay$size(::std::vector<::alt::HeadOverlay> const &s) noexcept {
  return s.size();
}
::alt::HeadOverlay *cxxbridge1$std$vector$alt$HeadOverlay$get_unchecked(::std::vector<::alt::HeadOverlay> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$HeadOverlay$push_back(::std::vector<::alt::HeadOverlay> *v, ::alt::HeadOverlay *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$HeadOverlay$pop_back(::std::vector<::alt::HeadOverlay> *v, ::alt::HeadOverlay *out) noexcept {
  ::new (out) ::alt::HeadOverlay(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::HeadOverlay>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::HeadOverlay>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$HeadOverlay$null(::std::unique_ptr<::std::vector<::alt::HeadOverlay>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::HeadOverlay>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$HeadOverlay$raw(::std::unique_ptr<::std::vector<::alt::HeadOverlay>> *ptr, ::std::vector<::alt::HeadOverlay> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::HeadOverlay>>(raw);
}
::std::vector<::alt::HeadOverlay> const *cxxbridge1$unique_ptr$std$vector$alt$HeadOverlay$get(::std::unique_ptr<::std::vector<::alt::HeadOverlay>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::HeadOverlay> *cxxbridge1$unique_ptr$std$vector$alt$HeadOverlay$release(::std::unique_ptr<::std::vector<::alt::HeadOverlay>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$HeadOverlay$drop(::std::unique_ptr<::std::vector<::alt::HeadOverlay>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::HeadBlendData>::value, "definition of HeadBlendData is required");
static_assert(sizeof(::std::unique_ptr<::alt::HeadBlendData>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::HeadBlendData>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$HeadBlendData$null(::std::unique_ptr<::alt::HeadBlendData> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::HeadBlendData>();
}
::alt::HeadBlendData *cxxbridge1$unique_ptr$alt$HeadBlendData$uninit(::std::unique_ptr<::alt::HeadBlendData> *ptr) noexcept {
  ::alt::HeadBlendData *uninit = reinterpret_cast<::alt::HeadBlendData *>(new ::rust::MaybeUninit<::alt::HeadBlendData>);
  ::new (ptr) ::std::unique_ptr<::alt::HeadBlendData>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$HeadBlendData$raw(::std::unique_ptr<::alt::HeadBlendData> *ptr, ::alt::HeadBlendData *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::HeadBlendData>(raw);
}
::alt::HeadBlendData const *cxxbridge1$unique_ptr$alt$HeadBlendData$get(::std::unique_ptr<::alt::HeadBlendData> const &ptr) noexcept {
  return ptr.get();
}
::alt::HeadBlendData *cxxbridge1$unique_ptr$alt$HeadBlendData$release(::std::unique_ptr<::alt::HeadBlendData> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$HeadBlendData$drop(::std::unique_ptr<::alt::HeadBlendData> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::HeadBlendData>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::HeadBlendData>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::HeadBlendData>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$HeadBlendData$null(::std::shared_ptr<::alt::HeadBlendData> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::HeadBlendData>();
}
::alt::HeadBlendData *cxxbridge1$shared_ptr$alt$HeadBlendData$uninit(::std::shared_ptr<::alt::HeadBlendData> *ptr) noexcept {
  ::alt::HeadBlendData *uninit = reinterpret_cast<::alt::HeadBlendData *>(new ::rust::MaybeUninit<::alt::HeadBlendData>);
  ::new (ptr) ::std::shared_ptr<::alt::HeadBlendData>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$HeadBlendData$clone(::std::shared_ptr<::alt::HeadBlendData> const &self, ::std::shared_ptr<::alt::HeadBlendData> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::HeadBlendData>(self);
}
::alt::HeadBlendData const *cxxbridge1$shared_ptr$alt$HeadBlendData$get(::std::shared_ptr<::alt::HeadBlendData> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$HeadBlendData$drop(::std::shared_ptr<::alt::HeadBlendData> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::HeadBlendData>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::HeadBlendData>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$HeadBlendData$null(::std::weak_ptr<::alt::HeadBlendData> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::HeadBlendData>();
}
void cxxbridge1$weak_ptr$alt$HeadBlendData$clone(::std::weak_ptr<::alt::HeadBlendData> const &self, ::std::weak_ptr<::alt::HeadBlendData> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::HeadBlendData>(self);
}
void cxxbridge1$weak_ptr$alt$HeadBlendData$downgrade(::std::shared_ptr<::alt::HeadBlendData> const &shared, ::std::weak_ptr<::alt::HeadBlendData> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::HeadBlendData>(shared);
}
void cxxbridge1$weak_ptr$alt$HeadBlendData$upgrade(::std::weak_ptr<::alt::HeadBlendData> const &weak, ::std::shared_ptr<::alt::HeadBlendData> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::HeadBlendData>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$HeadBlendData$drop(::std::weak_ptr<::alt::HeadBlendData> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::HeadBlendData> *cxxbridge1$std$vector$alt$HeadBlendData$new() noexcept {
  return new ::std::vector<::alt::HeadBlendData>();
}
::std::size_t cxxbridge1$std$vector$alt$HeadBlendData$size(::std::vector<::alt::HeadBlendData> const &s) noexcept {
  return s.size();
}
::alt::HeadBlendData *cxxbridge1$std$vector$alt$HeadBlendData$get_unchecked(::std::vector<::alt::HeadBlendData> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$HeadBlendData$push_back(::std::vector<::alt::HeadBlendData> *v, ::alt::HeadBlendData *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$HeadBlendData$pop_back(::std::vector<::alt::HeadBlendData> *v, ::alt::HeadBlendData *out) noexcept {
  ::new (out) ::alt::HeadBlendData(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::HeadBlendData>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::HeadBlendData>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$HeadBlendData$null(::std::unique_ptr<::std::vector<::alt::HeadBlendData>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::HeadBlendData>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$HeadBlendData$raw(::std::unique_ptr<::std::vector<::alt::HeadBlendData>> *ptr, ::std::vector<::alt::HeadBlendData> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::HeadBlendData>>(raw);
}
::std::vector<::alt::HeadBlendData> const *cxxbridge1$unique_ptr$std$vector$alt$HeadBlendData$get(::std::unique_ptr<::std::vector<::alt::HeadBlendData>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::HeadBlendData> *cxxbridge1$unique_ptr$std$vector$alt$HeadBlendData$release(::std::unique_ptr<::std::vector<::alt::HeadBlendData>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$HeadBlendData$drop(::std::unique_ptr<::std::vector<::alt::HeadBlendData>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::StreamedEntityWrapper>::value, "definition of StreamedEntityWrapper is required");
static_assert(sizeof(::std::unique_ptr<::StreamedEntityWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::StreamedEntityWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$StreamedEntityWrapper$null(::std::unique_ptr<::StreamedEntityWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::StreamedEntityWrapper>();
}
::StreamedEntityWrapper *cxxbridge1$unique_ptr$StreamedEntityWrapper$uninit(::std::unique_ptr<::StreamedEntityWrapper> *ptr) noexcept {
  ::StreamedEntityWrapper *uninit = reinterpret_cast<::StreamedEntityWrapper *>(new ::rust::MaybeUninit<::StreamedEntityWrapper>);
  ::new (ptr) ::std::unique_ptr<::StreamedEntityWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$StreamedEntityWrapper$raw(::std::unique_ptr<::StreamedEntityWrapper> *ptr, ::StreamedEntityWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::StreamedEntityWrapper>(raw);
}
::StreamedEntityWrapper const *cxxbridge1$unique_ptr$StreamedEntityWrapper$get(::std::unique_ptr<::StreamedEntityWrapper> const &ptr) noexcept {
  return ptr.get();
}
::StreamedEntityWrapper *cxxbridge1$unique_ptr$StreamedEntityWrapper$release(::std::unique_ptr<::StreamedEntityWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$StreamedEntityWrapper$drop(::std::unique_ptr<::StreamedEntityWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::StreamedEntityWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::StreamedEntityWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::StreamedEntityWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$StreamedEntityWrapper$null(::std::shared_ptr<::StreamedEntityWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::StreamedEntityWrapper>();
}
::StreamedEntityWrapper *cxxbridge1$shared_ptr$StreamedEntityWrapper$uninit(::std::shared_ptr<::StreamedEntityWrapper> *ptr) noexcept {
  ::StreamedEntityWrapper *uninit = reinterpret_cast<::StreamedEntityWrapper *>(new ::rust::MaybeUninit<::StreamedEntityWrapper>);
  ::new (ptr) ::std::shared_ptr<::StreamedEntityWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$StreamedEntityWrapper$clone(::std::shared_ptr<::StreamedEntityWrapper> const &self, ::std::shared_ptr<::StreamedEntityWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::StreamedEntityWrapper>(self);
}
::StreamedEntityWrapper const *cxxbridge1$shared_ptr$StreamedEntityWrapper$get(::std::shared_ptr<::StreamedEntityWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$StreamedEntityWrapper$drop(::std::shared_ptr<::StreamedEntityWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::StreamedEntityWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::StreamedEntityWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$StreamedEntityWrapper$null(::std::weak_ptr<::StreamedEntityWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::StreamedEntityWrapper>();
}
void cxxbridge1$weak_ptr$StreamedEntityWrapper$clone(::std::weak_ptr<::StreamedEntityWrapper> const &self, ::std::weak_ptr<::StreamedEntityWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::StreamedEntityWrapper>(self);
}
void cxxbridge1$weak_ptr$StreamedEntityWrapper$downgrade(::std::shared_ptr<::StreamedEntityWrapper> const &shared, ::std::weak_ptr<::StreamedEntityWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::StreamedEntityWrapper>(shared);
}
void cxxbridge1$weak_ptr$StreamedEntityWrapper$upgrade(::std::weak_ptr<::StreamedEntityWrapper> const &weak, ::std::shared_ptr<::StreamedEntityWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::StreamedEntityWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$StreamedEntityWrapper$drop(::std::weak_ptr<::StreamedEntityWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::StreamedEntityWrapper> *cxxbridge1$std$vector$StreamedEntityWrapper$new() noexcept {
  return new ::std::vector<::StreamedEntityWrapper>();
}
::std::size_t cxxbridge1$std$vector$StreamedEntityWrapper$size(::std::vector<::StreamedEntityWrapper> const &s) noexcept {
  return s.size();
}
::StreamedEntityWrapper *cxxbridge1$std$vector$StreamedEntityWrapper$get_unchecked(::std::vector<::StreamedEntityWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$StreamedEntityWrapper$push_back(::std::vector<::StreamedEntityWrapper> *v, ::StreamedEntityWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$StreamedEntityWrapper$pop_back(::std::vector<::StreamedEntityWrapper> *v, ::StreamedEntityWrapper *out) noexcept {
  ::new (out) ::StreamedEntityWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::StreamedEntityWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::StreamedEntityWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$StreamedEntityWrapper$null(::std::unique_ptr<::std::vector<::StreamedEntityWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::StreamedEntityWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$StreamedEntityWrapper$raw(::std::unique_ptr<::std::vector<::StreamedEntityWrapper>> *ptr, ::std::vector<::StreamedEntityWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::StreamedEntityWrapper>>(raw);
}
::std::vector<::StreamedEntityWrapper> const *cxxbridge1$unique_ptr$std$vector$StreamedEntityWrapper$get(::std::unique_ptr<::std::vector<::StreamedEntityWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::StreamedEntityWrapper> *cxxbridge1$unique_ptr$std$vector$StreamedEntityWrapper$release(::std::unique_ptr<::std::vector<::StreamedEntityWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$StreamedEntityWrapper$drop(::std::unique_ptr<::std::vector<::StreamedEntityWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::AmmoFlags>::value, "definition of AmmoFlags is required");
static_assert(sizeof(::std::unique_ptr<::alt::AmmoFlags>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::AmmoFlags>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$AmmoFlags$null(::std::unique_ptr<::alt::AmmoFlags> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::AmmoFlags>();
}
::alt::AmmoFlags *cxxbridge1$unique_ptr$alt$AmmoFlags$uninit(::std::unique_ptr<::alt::AmmoFlags> *ptr) noexcept {
  ::alt::AmmoFlags *uninit = reinterpret_cast<::alt::AmmoFlags *>(new ::rust::MaybeUninit<::alt::AmmoFlags>);
  ::new (ptr) ::std::unique_ptr<::alt::AmmoFlags>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$AmmoFlags$raw(::std::unique_ptr<::alt::AmmoFlags> *ptr, ::alt::AmmoFlags *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::AmmoFlags>(raw);
}
::alt::AmmoFlags const *cxxbridge1$unique_ptr$alt$AmmoFlags$get(::std::unique_ptr<::alt::AmmoFlags> const &ptr) noexcept {
  return ptr.get();
}
::alt::AmmoFlags *cxxbridge1$unique_ptr$alt$AmmoFlags$release(::std::unique_ptr<::alt::AmmoFlags> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$AmmoFlags$drop(::std::unique_ptr<::alt::AmmoFlags> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::AmmoFlags>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::AmmoFlags>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::AmmoFlags>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$AmmoFlags$null(::std::shared_ptr<::alt::AmmoFlags> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::AmmoFlags>();
}
::alt::AmmoFlags *cxxbridge1$shared_ptr$alt$AmmoFlags$uninit(::std::shared_ptr<::alt::AmmoFlags> *ptr) noexcept {
  ::alt::AmmoFlags *uninit = reinterpret_cast<::alt::AmmoFlags *>(new ::rust::MaybeUninit<::alt::AmmoFlags>);
  ::new (ptr) ::std::shared_ptr<::alt::AmmoFlags>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$AmmoFlags$clone(::std::shared_ptr<::alt::AmmoFlags> const &self, ::std::shared_ptr<::alt::AmmoFlags> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::AmmoFlags>(self);
}
::alt::AmmoFlags const *cxxbridge1$shared_ptr$alt$AmmoFlags$get(::std::shared_ptr<::alt::AmmoFlags> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$AmmoFlags$drop(::std::shared_ptr<::alt::AmmoFlags> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::AmmoFlags>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::AmmoFlags>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$AmmoFlags$null(::std::weak_ptr<::alt::AmmoFlags> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::AmmoFlags>();
}
void cxxbridge1$weak_ptr$alt$AmmoFlags$clone(::std::weak_ptr<::alt::AmmoFlags> const &self, ::std::weak_ptr<::alt::AmmoFlags> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::AmmoFlags>(self);
}
void cxxbridge1$weak_ptr$alt$AmmoFlags$downgrade(::std::shared_ptr<::alt::AmmoFlags> const &shared, ::std::weak_ptr<::alt::AmmoFlags> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::AmmoFlags>(shared);
}
void cxxbridge1$weak_ptr$alt$AmmoFlags$upgrade(::std::weak_ptr<::alt::AmmoFlags> const &weak, ::std::shared_ptr<::alt::AmmoFlags> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::AmmoFlags>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$AmmoFlags$drop(::std::weak_ptr<::alt::AmmoFlags> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::AmmoFlags> *cxxbridge1$std$vector$alt$AmmoFlags$new() noexcept {
  return new ::std::vector<::alt::AmmoFlags>();
}
::std::size_t cxxbridge1$std$vector$alt$AmmoFlags$size(::std::vector<::alt::AmmoFlags> const &s) noexcept {
  return s.size();
}
::alt::AmmoFlags *cxxbridge1$std$vector$alt$AmmoFlags$get_unchecked(::std::vector<::alt::AmmoFlags> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$AmmoFlags$push_back(::std::vector<::alt::AmmoFlags> *v, ::alt::AmmoFlags *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$AmmoFlags$pop_back(::std::vector<::alt::AmmoFlags> *v, ::alt::AmmoFlags *out) noexcept {
  ::new (out) ::alt::AmmoFlags(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::AmmoFlags>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::AmmoFlags>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$AmmoFlags$null(::std::unique_ptr<::std::vector<::alt::AmmoFlags>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::AmmoFlags>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$AmmoFlags$raw(::std::unique_ptr<::std::vector<::alt::AmmoFlags>> *ptr, ::std::vector<::alt::AmmoFlags> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::AmmoFlags>>(raw);
}
::std::vector<::alt::AmmoFlags> const *cxxbridge1$unique_ptr$std$vector$alt$AmmoFlags$get(::std::unique_ptr<::std::vector<::alt::AmmoFlags>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::AmmoFlags> *cxxbridge1$unique_ptr$std$vector$alt$AmmoFlags$release(::std::unique_ptr<::std::vector<::alt::AmmoFlags>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$AmmoFlags$drop(::std::unique_ptr<::std::vector<::alt::AmmoFlags>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::CDecoration>::value, "definition of CDecoration is required");
static_assert(sizeof(::std::unique_ptr<::alt::CDecoration>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::CDecoration>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$CDecoration$null(::std::unique_ptr<::alt::CDecoration> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CDecoration>();
}
::alt::CDecoration *cxxbridge1$unique_ptr$alt$CDecoration$uninit(::std::unique_ptr<::alt::CDecoration> *ptr) noexcept {
  ::alt::CDecoration *uninit = reinterpret_cast<::alt::CDecoration *>(new ::rust::MaybeUninit<::alt::CDecoration>);
  ::new (ptr) ::std::unique_ptr<::alt::CDecoration>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$CDecoration$raw(::std::unique_ptr<::alt::CDecoration> *ptr, ::alt::CDecoration *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::CDecoration>(raw);
}
::alt::CDecoration const *cxxbridge1$unique_ptr$alt$CDecoration$get(::std::unique_ptr<::alt::CDecoration> const &ptr) noexcept {
  return ptr.get();
}
::alt::CDecoration *cxxbridge1$unique_ptr$alt$CDecoration$release(::std::unique_ptr<::alt::CDecoration> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$CDecoration$drop(::std::unique_ptr<::alt::CDecoration> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::CDecoration>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::CDecoration>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::CDecoration>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$CDecoration$null(::std::shared_ptr<::alt::CDecoration> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CDecoration>();
}
::alt::CDecoration *cxxbridge1$shared_ptr$alt$CDecoration$uninit(::std::shared_ptr<::alt::CDecoration> *ptr) noexcept {
  ::alt::CDecoration *uninit = reinterpret_cast<::alt::CDecoration *>(new ::rust::MaybeUninit<::alt::CDecoration>);
  ::new (ptr) ::std::shared_ptr<::alt::CDecoration>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$CDecoration$clone(::std::shared_ptr<::alt::CDecoration> const &self, ::std::shared_ptr<::alt::CDecoration> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::CDecoration>(self);
}
::alt::CDecoration const *cxxbridge1$shared_ptr$alt$CDecoration$get(::std::shared_ptr<::alt::CDecoration> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$CDecoration$drop(::std::shared_ptr<::alt::CDecoration> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::CDecoration>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::CDecoration>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$CDecoration$null(::std::weak_ptr<::alt::CDecoration> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CDecoration>();
}
void cxxbridge1$weak_ptr$alt$CDecoration$clone(::std::weak_ptr<::alt::CDecoration> const &self, ::std::weak_ptr<::alt::CDecoration> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::CDecoration>(self);
}
void cxxbridge1$weak_ptr$alt$CDecoration$downgrade(::std::shared_ptr<::alt::CDecoration> const &shared, ::std::weak_ptr<::alt::CDecoration> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::CDecoration>(shared);
}
void cxxbridge1$weak_ptr$alt$CDecoration$upgrade(::std::weak_ptr<::alt::CDecoration> const &weak, ::std::shared_ptr<::alt::CDecoration> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::CDecoration>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$CDecoration$drop(::std::weak_ptr<::alt::CDecoration> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::CDecoration> *cxxbridge1$std$vector$alt$CDecoration$new() noexcept {
  return new ::std::vector<::alt::CDecoration>();
}
::std::size_t cxxbridge1$std$vector$alt$CDecoration$size(::std::vector<::alt::CDecoration> const &s) noexcept {
  return s.size();
}
::alt::CDecoration *cxxbridge1$std$vector$alt$CDecoration$get_unchecked(::std::vector<::alt::CDecoration> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$CDecoration$push_back(::std::vector<::alt::CDecoration> *v, ::alt::CDecoration *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$CDecoration$pop_back(::std::vector<::alt::CDecoration> *v, ::alt::CDecoration *out) noexcept {
  ::new (out) ::alt::CDecoration(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::CDecoration>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::CDecoration>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$CDecoration$null(::std::unique_ptr<::std::vector<::alt::CDecoration>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::CDecoration>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$CDecoration$raw(::std::unique_ptr<::std::vector<::alt::CDecoration>> *ptr, ::std::vector<::alt::CDecoration> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::CDecoration>>(raw);
}
::std::vector<::alt::CDecoration> const *cxxbridge1$unique_ptr$std$vector$alt$CDecoration$get(::std::unique_ptr<::std::vector<::alt::CDecoration>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::CDecoration> *cxxbridge1$unique_ptr$std$vector$alt$CDecoration$release(::std::unique_ptr<::std::vector<::alt::CDecoration>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$CDecoration$drop(::std::unique_ptr<::std::vector<::alt::CDecoration>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::Quaternion>::value, "definition of Quaternion is required");
static_assert(sizeof(::std::unique_ptr<::alt::Quaternion>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::Quaternion>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$Quaternion$null(::std::unique_ptr<::alt::Quaternion> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::Quaternion>();
}
::alt::Quaternion *cxxbridge1$unique_ptr$alt$Quaternion$uninit(::std::unique_ptr<::alt::Quaternion> *ptr) noexcept {
  ::alt::Quaternion *uninit = reinterpret_cast<::alt::Quaternion *>(new ::rust::MaybeUninit<::alt::Quaternion>);
  ::new (ptr) ::std::unique_ptr<::alt::Quaternion>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$Quaternion$raw(::std::unique_ptr<::alt::Quaternion> *ptr, ::alt::Quaternion *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::Quaternion>(raw);
}
::alt::Quaternion const *cxxbridge1$unique_ptr$alt$Quaternion$get(::std::unique_ptr<::alt::Quaternion> const &ptr) noexcept {
  return ptr.get();
}
::alt::Quaternion *cxxbridge1$unique_ptr$alt$Quaternion$release(::std::unique_ptr<::alt::Quaternion> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$Quaternion$drop(::std::unique_ptr<::alt::Quaternion> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::Quaternion>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::Quaternion>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::Quaternion>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$Quaternion$null(::std::shared_ptr<::alt::Quaternion> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::Quaternion>();
}
::alt::Quaternion *cxxbridge1$shared_ptr$alt$Quaternion$uninit(::std::shared_ptr<::alt::Quaternion> *ptr) noexcept {
  ::alt::Quaternion *uninit = reinterpret_cast<::alt::Quaternion *>(new ::rust::MaybeUninit<::alt::Quaternion>);
  ::new (ptr) ::std::shared_ptr<::alt::Quaternion>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$Quaternion$clone(::std::shared_ptr<::alt::Quaternion> const &self, ::std::shared_ptr<::alt::Quaternion> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::Quaternion>(self);
}
::alt::Quaternion const *cxxbridge1$shared_ptr$alt$Quaternion$get(::std::shared_ptr<::alt::Quaternion> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$Quaternion$drop(::std::shared_ptr<::alt::Quaternion> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::Quaternion>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::Quaternion>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$Quaternion$null(::std::weak_ptr<::alt::Quaternion> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::Quaternion>();
}
void cxxbridge1$weak_ptr$alt$Quaternion$clone(::std::weak_ptr<::alt::Quaternion> const &self, ::std::weak_ptr<::alt::Quaternion> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::Quaternion>(self);
}
void cxxbridge1$weak_ptr$alt$Quaternion$downgrade(::std::shared_ptr<::alt::Quaternion> const &shared, ::std::weak_ptr<::alt::Quaternion> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::Quaternion>(shared);
}
void cxxbridge1$weak_ptr$alt$Quaternion$upgrade(::std::weak_ptr<::alt::Quaternion> const &weak, ::std::shared_ptr<::alt::Quaternion> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::Quaternion>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$Quaternion$drop(::std::weak_ptr<::alt::Quaternion> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::Quaternion> *cxxbridge1$std$vector$alt$Quaternion$new() noexcept {
  return new ::std::vector<::alt::Quaternion>();
}
::std::size_t cxxbridge1$std$vector$alt$Quaternion$size(::std::vector<::alt::Quaternion> const &s) noexcept {
  return s.size();
}
::alt::Quaternion *cxxbridge1$std$vector$alt$Quaternion$get_unchecked(::std::vector<::alt::Quaternion> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$Quaternion$push_back(::std::vector<::alt::Quaternion> *v, ::alt::Quaternion *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$Quaternion$pop_back(::std::vector<::alt::Quaternion> *v, ::alt::Quaternion *out) noexcept {
  ::new (out) ::alt::Quaternion(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::Quaternion>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::Quaternion>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$Quaternion$null(::std::unique_ptr<::std::vector<::alt::Quaternion>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::Quaternion>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$Quaternion$raw(::std::unique_ptr<::std::vector<::alt::Quaternion>> *ptr, ::std::vector<::alt::Quaternion> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::Quaternion>>(raw);
}
::std::vector<::alt::Quaternion> const *cxxbridge1$unique_ptr$std$vector$alt$Quaternion$get(::std::unique_ptr<::std::vector<::alt::Quaternion>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::Quaternion> *cxxbridge1$unique_ptr$std$vector$alt$Quaternion$release(::std::unique_ptr<::std::vector<::alt::Quaternion>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$Quaternion$drop(::std::unique_ptr<::std::vector<::alt::Quaternion>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::EntityAnimHashPairsWrapper>::value, "definition of EntityAnimHashPairsWrapper is required");
static_assert(sizeof(::std::unique_ptr<::EntityAnimHashPairsWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::EntityAnimHashPairsWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$EntityAnimHashPairsWrapper$null(::std::unique_ptr<::EntityAnimHashPairsWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::EntityAnimHashPairsWrapper>();
}
::EntityAnimHashPairsWrapper *cxxbridge1$unique_ptr$EntityAnimHashPairsWrapper$uninit(::std::unique_ptr<::EntityAnimHashPairsWrapper> *ptr) noexcept {
  ::EntityAnimHashPairsWrapper *uninit = reinterpret_cast<::EntityAnimHashPairsWrapper *>(new ::rust::MaybeUninit<::EntityAnimHashPairsWrapper>);
  ::new (ptr) ::std::unique_ptr<::EntityAnimHashPairsWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$EntityAnimHashPairsWrapper$raw(::std::unique_ptr<::EntityAnimHashPairsWrapper> *ptr, ::EntityAnimHashPairsWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::EntityAnimHashPairsWrapper>(raw);
}
::EntityAnimHashPairsWrapper const *cxxbridge1$unique_ptr$EntityAnimHashPairsWrapper$get(::std::unique_ptr<::EntityAnimHashPairsWrapper> const &ptr) noexcept {
  return ptr.get();
}
::EntityAnimHashPairsWrapper *cxxbridge1$unique_ptr$EntityAnimHashPairsWrapper$release(::std::unique_ptr<::EntityAnimHashPairsWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$EntityAnimHashPairsWrapper$drop(::std::unique_ptr<::EntityAnimHashPairsWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::EntityAnimHashPairsWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::EntityAnimHashPairsWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::EntityAnimHashPairsWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$EntityAnimHashPairsWrapper$null(::std::shared_ptr<::EntityAnimHashPairsWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::EntityAnimHashPairsWrapper>();
}
::EntityAnimHashPairsWrapper *cxxbridge1$shared_ptr$EntityAnimHashPairsWrapper$uninit(::std::shared_ptr<::EntityAnimHashPairsWrapper> *ptr) noexcept {
  ::EntityAnimHashPairsWrapper *uninit = reinterpret_cast<::EntityAnimHashPairsWrapper *>(new ::rust::MaybeUninit<::EntityAnimHashPairsWrapper>);
  ::new (ptr) ::std::shared_ptr<::EntityAnimHashPairsWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$EntityAnimHashPairsWrapper$clone(::std::shared_ptr<::EntityAnimHashPairsWrapper> const &self, ::std::shared_ptr<::EntityAnimHashPairsWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::EntityAnimHashPairsWrapper>(self);
}
::EntityAnimHashPairsWrapper const *cxxbridge1$shared_ptr$EntityAnimHashPairsWrapper$get(::std::shared_ptr<::EntityAnimHashPairsWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$EntityAnimHashPairsWrapper$drop(::std::shared_ptr<::EntityAnimHashPairsWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::EntityAnimHashPairsWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::EntityAnimHashPairsWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$EntityAnimHashPairsWrapper$null(::std::weak_ptr<::EntityAnimHashPairsWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::EntityAnimHashPairsWrapper>();
}
void cxxbridge1$weak_ptr$EntityAnimHashPairsWrapper$clone(::std::weak_ptr<::EntityAnimHashPairsWrapper> const &self, ::std::weak_ptr<::EntityAnimHashPairsWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::EntityAnimHashPairsWrapper>(self);
}
void cxxbridge1$weak_ptr$EntityAnimHashPairsWrapper$downgrade(::std::shared_ptr<::EntityAnimHashPairsWrapper> const &shared, ::std::weak_ptr<::EntityAnimHashPairsWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::EntityAnimHashPairsWrapper>(shared);
}
void cxxbridge1$weak_ptr$EntityAnimHashPairsWrapper$upgrade(::std::weak_ptr<::EntityAnimHashPairsWrapper> const &weak, ::std::shared_ptr<::EntityAnimHashPairsWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::EntityAnimHashPairsWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$EntityAnimHashPairsWrapper$drop(::std::weak_ptr<::EntityAnimHashPairsWrapper> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::alt::IScriptRuntime>::value, "definition of IScriptRuntime is required");
static_assert(sizeof(::std::unique_ptr<::alt::IScriptRuntime>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::IScriptRuntime>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$IScriptRuntime$null(::std::unique_ptr<::alt::IScriptRuntime> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IScriptRuntime>();
}
void cxxbridge1$unique_ptr$alt$IScriptRuntime$raw(::std::unique_ptr<::alt::IScriptRuntime> *ptr, ::alt::IScriptRuntime *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::IScriptRuntime>(raw);
}
::alt::IScriptRuntime const *cxxbridge1$unique_ptr$alt$IScriptRuntime$get(::std::unique_ptr<::alt::IScriptRuntime> const &ptr) noexcept {
  return ptr.get();
}
::alt::IScriptRuntime *cxxbridge1$unique_ptr$alt$IScriptRuntime$release(::std::unique_ptr<::alt::IScriptRuntime> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$IScriptRuntime$drop(::std::unique_ptr<::alt::IScriptRuntime> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::IScriptRuntime>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::IScriptRuntime>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::IScriptRuntime>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$IScriptRuntime$null(::std::shared_ptr<::alt::IScriptRuntime> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IScriptRuntime>();
}
void cxxbridge1$shared_ptr$alt$IScriptRuntime$clone(::std::shared_ptr<::alt::IScriptRuntime> const &self, ::std::shared_ptr<::alt::IScriptRuntime> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::IScriptRuntime>(self);
}
::alt::IScriptRuntime const *cxxbridge1$shared_ptr$alt$IScriptRuntime$get(::std::shared_ptr<::alt::IScriptRuntime> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$IScriptRuntime$drop(::std::shared_ptr<::alt::IScriptRuntime> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::IScriptRuntime>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::IScriptRuntime>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$IScriptRuntime$null(::std::weak_ptr<::alt::IScriptRuntime> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IScriptRuntime>();
}
void cxxbridge1$weak_ptr$alt$IScriptRuntime$clone(::std::weak_ptr<::alt::IScriptRuntime> const &self, ::std::weak_ptr<::alt::IScriptRuntime> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::IScriptRuntime>(self);
}
void cxxbridge1$weak_ptr$alt$IScriptRuntime$downgrade(::std::shared_ptr<::alt::IScriptRuntime> const &shared, ::std::weak_ptr<::alt::IScriptRuntime> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::IScriptRuntime>(shared);
}
void cxxbridge1$weak_ptr$alt$IScriptRuntime$upgrade(::std::weak_ptr<::alt::IScriptRuntime> const &weak, ::std::shared_ptr<::alt::IScriptRuntime> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::IScriptRuntime>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$IScriptRuntime$drop(::std::weak_ptr<::alt::IScriptRuntime> *self) noexcept {
  self->~weak_ptr();
}

static_assert(::rust::detail::is_complete<::MValueDictPairWrapper>::value, "definition of MValueDictPairWrapper is required");
static_assert(sizeof(::std::unique_ptr<::MValueDictPairWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::MValueDictPairWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$MValueDictPairWrapper$null(::std::unique_ptr<::MValueDictPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::MValueDictPairWrapper>();
}
::MValueDictPairWrapper *cxxbridge1$unique_ptr$MValueDictPairWrapper$uninit(::std::unique_ptr<::MValueDictPairWrapper> *ptr) noexcept {
  ::MValueDictPairWrapper *uninit = reinterpret_cast<::MValueDictPairWrapper *>(new ::rust::MaybeUninit<::MValueDictPairWrapper>);
  ::new (ptr) ::std::unique_ptr<::MValueDictPairWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$MValueDictPairWrapper$raw(::std::unique_ptr<::MValueDictPairWrapper> *ptr, ::MValueDictPairWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::MValueDictPairWrapper>(raw);
}
::MValueDictPairWrapper const *cxxbridge1$unique_ptr$MValueDictPairWrapper$get(::std::unique_ptr<::MValueDictPairWrapper> const &ptr) noexcept {
  return ptr.get();
}
::MValueDictPairWrapper *cxxbridge1$unique_ptr$MValueDictPairWrapper$release(::std::unique_ptr<::MValueDictPairWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$MValueDictPairWrapper$drop(::std::unique_ptr<::MValueDictPairWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::MValueDictPairWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::MValueDictPairWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::MValueDictPairWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$MValueDictPairWrapper$null(::std::shared_ptr<::MValueDictPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::MValueDictPairWrapper>();
}
::MValueDictPairWrapper *cxxbridge1$shared_ptr$MValueDictPairWrapper$uninit(::std::shared_ptr<::MValueDictPairWrapper> *ptr) noexcept {
  ::MValueDictPairWrapper *uninit = reinterpret_cast<::MValueDictPairWrapper *>(new ::rust::MaybeUninit<::MValueDictPairWrapper>);
  ::new (ptr) ::std::shared_ptr<::MValueDictPairWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$MValueDictPairWrapper$clone(::std::shared_ptr<::MValueDictPairWrapper> const &self, ::std::shared_ptr<::MValueDictPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::MValueDictPairWrapper>(self);
}
::MValueDictPairWrapper const *cxxbridge1$shared_ptr$MValueDictPairWrapper$get(::std::shared_ptr<::MValueDictPairWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$MValueDictPairWrapper$drop(::std::shared_ptr<::MValueDictPairWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::MValueDictPairWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::MValueDictPairWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$MValueDictPairWrapper$null(::std::weak_ptr<::MValueDictPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::MValueDictPairWrapper>();
}
void cxxbridge1$weak_ptr$MValueDictPairWrapper$clone(::std::weak_ptr<::MValueDictPairWrapper> const &self, ::std::weak_ptr<::MValueDictPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::MValueDictPairWrapper>(self);
}
void cxxbridge1$weak_ptr$MValueDictPairWrapper$downgrade(::std::shared_ptr<::MValueDictPairWrapper> const &shared, ::std::weak_ptr<::MValueDictPairWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::MValueDictPairWrapper>(shared);
}
void cxxbridge1$weak_ptr$MValueDictPairWrapper$upgrade(::std::weak_ptr<::MValueDictPairWrapper> const &weak, ::std::shared_ptr<::MValueDictPairWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::MValueDictPairWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$MValueDictPairWrapper$drop(::std::weak_ptr<::MValueDictPairWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::MValueDictPairWrapper> *cxxbridge1$std$vector$MValueDictPairWrapper$new() noexcept {
  return new ::std::vector<::MValueDictPairWrapper>();
}
::std::size_t cxxbridge1$std$vector$MValueDictPairWrapper$size(::std::vector<::MValueDictPairWrapper> const &s) noexcept {
  return s.size();
}
::MValueDictPairWrapper *cxxbridge1$std$vector$MValueDictPairWrapper$get_unchecked(::std::vector<::MValueDictPairWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$MValueDictPairWrapper$push_back(::std::vector<::MValueDictPairWrapper> *v, ::MValueDictPairWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$MValueDictPairWrapper$pop_back(::std::vector<::MValueDictPairWrapper> *v, ::MValueDictPairWrapper *out) noexcept {
  ::new (out) ::MValueDictPairWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::MValueDictPairWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::MValueDictPairWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$MValueDictPairWrapper$null(::std::unique_ptr<::std::vector<::MValueDictPairWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::MValueDictPairWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$MValueDictPairWrapper$raw(::std::unique_ptr<::std::vector<::MValueDictPairWrapper>> *ptr, ::std::vector<::MValueDictPairWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::MValueDictPairWrapper>>(raw);
}
::std::vector<::MValueDictPairWrapper> const *cxxbridge1$unique_ptr$std$vector$MValueDictPairWrapper$get(::std::unique_ptr<::std::vector<::MValueDictPairWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::MValueDictPairWrapper> *cxxbridge1$unique_ptr$std$vector$MValueDictPairWrapper$release(::std::unique_ptr<::std::vector<::MValueDictPairWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$MValueDictPairWrapper$drop(::std::unique_ptr<::std::vector<::MValueDictPairWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::EntityAnimHashPairWrapper>::value, "definition of EntityAnimHashPairWrapper is required");
static_assert(sizeof(::std::unique_ptr<::EntityAnimHashPairWrapper>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::EntityAnimHashPairWrapper>) == alignof(void *), "");
void cxxbridge1$unique_ptr$EntityAnimHashPairWrapper$null(::std::unique_ptr<::EntityAnimHashPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::EntityAnimHashPairWrapper>();
}
::EntityAnimHashPairWrapper *cxxbridge1$unique_ptr$EntityAnimHashPairWrapper$uninit(::std::unique_ptr<::EntityAnimHashPairWrapper> *ptr) noexcept {
  ::EntityAnimHashPairWrapper *uninit = reinterpret_cast<::EntityAnimHashPairWrapper *>(new ::rust::MaybeUninit<::EntityAnimHashPairWrapper>);
  ::new (ptr) ::std::unique_ptr<::EntityAnimHashPairWrapper>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$EntityAnimHashPairWrapper$raw(::std::unique_ptr<::EntityAnimHashPairWrapper> *ptr, ::EntityAnimHashPairWrapper *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::EntityAnimHashPairWrapper>(raw);
}
::EntityAnimHashPairWrapper const *cxxbridge1$unique_ptr$EntityAnimHashPairWrapper$get(::std::unique_ptr<::EntityAnimHashPairWrapper> const &ptr) noexcept {
  return ptr.get();
}
::EntityAnimHashPairWrapper *cxxbridge1$unique_ptr$EntityAnimHashPairWrapper$release(::std::unique_ptr<::EntityAnimHashPairWrapper> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$EntityAnimHashPairWrapper$drop(::std::unique_ptr<::EntityAnimHashPairWrapper> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::EntityAnimHashPairWrapper>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::EntityAnimHashPairWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::EntityAnimHashPairWrapper>) == alignof(void *), "");
void cxxbridge1$shared_ptr$EntityAnimHashPairWrapper$null(::std::shared_ptr<::EntityAnimHashPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::EntityAnimHashPairWrapper>();
}
::EntityAnimHashPairWrapper *cxxbridge1$shared_ptr$EntityAnimHashPairWrapper$uninit(::std::shared_ptr<::EntityAnimHashPairWrapper> *ptr) noexcept {
  ::EntityAnimHashPairWrapper *uninit = reinterpret_cast<::EntityAnimHashPairWrapper *>(new ::rust::MaybeUninit<::EntityAnimHashPairWrapper>);
  ::new (ptr) ::std::shared_ptr<::EntityAnimHashPairWrapper>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$EntityAnimHashPairWrapper$clone(::std::shared_ptr<::EntityAnimHashPairWrapper> const &self, ::std::shared_ptr<::EntityAnimHashPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::EntityAnimHashPairWrapper>(self);
}
::EntityAnimHashPairWrapper const *cxxbridge1$shared_ptr$EntityAnimHashPairWrapper$get(::std::shared_ptr<::EntityAnimHashPairWrapper> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$EntityAnimHashPairWrapper$drop(::std::shared_ptr<::EntityAnimHashPairWrapper> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::EntityAnimHashPairWrapper>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::EntityAnimHashPairWrapper>) == alignof(void *), "");
void cxxbridge1$weak_ptr$EntityAnimHashPairWrapper$null(::std::weak_ptr<::EntityAnimHashPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::EntityAnimHashPairWrapper>();
}
void cxxbridge1$weak_ptr$EntityAnimHashPairWrapper$clone(::std::weak_ptr<::EntityAnimHashPairWrapper> const &self, ::std::weak_ptr<::EntityAnimHashPairWrapper> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::EntityAnimHashPairWrapper>(self);
}
void cxxbridge1$weak_ptr$EntityAnimHashPairWrapper$downgrade(::std::shared_ptr<::EntityAnimHashPairWrapper> const &shared, ::std::weak_ptr<::EntityAnimHashPairWrapper> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::EntityAnimHashPairWrapper>(shared);
}
void cxxbridge1$weak_ptr$EntityAnimHashPairWrapper$upgrade(::std::weak_ptr<::EntityAnimHashPairWrapper> const &weak, ::std::shared_ptr<::EntityAnimHashPairWrapper> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::EntityAnimHashPairWrapper>(weak.lock());
}
void cxxbridge1$weak_ptr$EntityAnimHashPairWrapper$drop(::std::weak_ptr<::EntityAnimHashPairWrapper> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::EntityAnimHashPairWrapper> *cxxbridge1$std$vector$EntityAnimHashPairWrapper$new() noexcept {
  return new ::std::vector<::EntityAnimHashPairWrapper>();
}
::std::size_t cxxbridge1$std$vector$EntityAnimHashPairWrapper$size(::std::vector<::EntityAnimHashPairWrapper> const &s) noexcept {
  return s.size();
}
::EntityAnimHashPairWrapper *cxxbridge1$std$vector$EntityAnimHashPairWrapper$get_unchecked(::std::vector<::EntityAnimHashPairWrapper> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$EntityAnimHashPairWrapper$push_back(::std::vector<::EntityAnimHashPairWrapper> *v, ::EntityAnimHashPairWrapper *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$EntityAnimHashPairWrapper$pop_back(::std::vector<::EntityAnimHashPairWrapper> *v, ::EntityAnimHashPairWrapper *out) noexcept {
  ::new (out) ::EntityAnimHashPairWrapper(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::EntityAnimHashPairWrapper>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::EntityAnimHashPairWrapper>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$EntityAnimHashPairWrapper$null(::std::unique_ptr<::std::vector<::EntityAnimHashPairWrapper>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::EntityAnimHashPairWrapper>>();
}
void cxxbridge1$unique_ptr$std$vector$EntityAnimHashPairWrapper$raw(::std::unique_ptr<::std::vector<::EntityAnimHashPairWrapper>> *ptr, ::std::vector<::EntityAnimHashPairWrapper> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::EntityAnimHashPairWrapper>>(raw);
}
::std::vector<::EntityAnimHashPairWrapper> const *cxxbridge1$unique_ptr$std$vector$EntityAnimHashPairWrapper$get(::std::unique_ptr<::std::vector<::EntityAnimHashPairWrapper>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::EntityAnimHashPairWrapper> *cxxbridge1$unique_ptr$std$vector$EntityAnimHashPairWrapper$release(::std::unique_ptr<::std::vector<::EntityAnimHashPairWrapper>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$EntityAnimHashPairWrapper$drop(::std::unique_ptr<::std::vector<::EntityAnimHashPairWrapper>> *ptr) noexcept {
  ptr->~unique_ptr();
}

static_assert(::rust::detail::is_complete<::alt::BoneInfo>::value, "definition of BoneInfo is required");
static_assert(sizeof(::std::unique_ptr<::alt::BoneInfo>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::alt::BoneInfo>) == alignof(void *), "");
void cxxbridge1$unique_ptr$alt$BoneInfo$null(::std::unique_ptr<::alt::BoneInfo> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::BoneInfo>();
}
::alt::BoneInfo *cxxbridge1$unique_ptr$alt$BoneInfo$uninit(::std::unique_ptr<::alt::BoneInfo> *ptr) noexcept {
  ::alt::BoneInfo *uninit = reinterpret_cast<::alt::BoneInfo *>(new ::rust::MaybeUninit<::alt::BoneInfo>);
  ::new (ptr) ::std::unique_ptr<::alt::BoneInfo>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$alt$BoneInfo$raw(::std::unique_ptr<::alt::BoneInfo> *ptr, ::alt::BoneInfo *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::alt::BoneInfo>(raw);
}
::alt::BoneInfo const *cxxbridge1$unique_ptr$alt$BoneInfo$get(::std::unique_ptr<::alt::BoneInfo> const &ptr) noexcept {
  return ptr.get();
}
::alt::BoneInfo *cxxbridge1$unique_ptr$alt$BoneInfo$release(::std::unique_ptr<::alt::BoneInfo> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$alt$BoneInfo$drop(::std::unique_ptr<::alt::BoneInfo> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::alt::BoneInfo>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::alt::BoneInfo>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::alt::BoneInfo>) == alignof(void *), "");
void cxxbridge1$shared_ptr$alt$BoneInfo$null(::std::shared_ptr<::alt::BoneInfo> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::BoneInfo>();
}
::alt::BoneInfo *cxxbridge1$shared_ptr$alt$BoneInfo$uninit(::std::shared_ptr<::alt::BoneInfo> *ptr) noexcept {
  ::alt::BoneInfo *uninit = reinterpret_cast<::alt::BoneInfo *>(new ::rust::MaybeUninit<::alt::BoneInfo>);
  ::new (ptr) ::std::shared_ptr<::alt::BoneInfo>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$alt$BoneInfo$clone(::std::shared_ptr<::alt::BoneInfo> const &self, ::std::shared_ptr<::alt::BoneInfo> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::alt::BoneInfo>(self);
}
::alt::BoneInfo const *cxxbridge1$shared_ptr$alt$BoneInfo$get(::std::shared_ptr<::alt::BoneInfo> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$alt$BoneInfo$drop(::std::shared_ptr<::alt::BoneInfo> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::alt::BoneInfo>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::alt::BoneInfo>) == alignof(void *), "");
void cxxbridge1$weak_ptr$alt$BoneInfo$null(::std::weak_ptr<::alt::BoneInfo> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::BoneInfo>();
}
void cxxbridge1$weak_ptr$alt$BoneInfo$clone(::std::weak_ptr<::alt::BoneInfo> const &self, ::std::weak_ptr<::alt::BoneInfo> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::alt::BoneInfo>(self);
}
void cxxbridge1$weak_ptr$alt$BoneInfo$downgrade(::std::shared_ptr<::alt::BoneInfo> const &shared, ::std::weak_ptr<::alt::BoneInfo> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::alt::BoneInfo>(shared);
}
void cxxbridge1$weak_ptr$alt$BoneInfo$upgrade(::std::weak_ptr<::alt::BoneInfo> const &weak, ::std::shared_ptr<::alt::BoneInfo> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::alt::BoneInfo>(weak.lock());
}
void cxxbridge1$weak_ptr$alt$BoneInfo$drop(::std::weak_ptr<::alt::BoneInfo> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::alt::BoneInfo> *cxxbridge1$std$vector$alt$BoneInfo$new() noexcept {
  return new ::std::vector<::alt::BoneInfo>();
}
::std::size_t cxxbridge1$std$vector$alt$BoneInfo$size(::std::vector<::alt::BoneInfo> const &s) noexcept {
  return s.size();
}
::alt::BoneInfo *cxxbridge1$std$vector$alt$BoneInfo$get_unchecked(::std::vector<::alt::BoneInfo> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$alt$BoneInfo$push_back(::std::vector<::alt::BoneInfo> *v, ::alt::BoneInfo *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$alt$BoneInfo$pop_back(::std::vector<::alt::BoneInfo> *v, ::alt::BoneInfo *out) noexcept {
  ::new (out) ::alt::BoneInfo(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::alt::BoneInfo>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::alt::BoneInfo>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$alt$BoneInfo$null(::std::unique_ptr<::std::vector<::alt::BoneInfo>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::BoneInfo>>();
}
void cxxbridge1$unique_ptr$std$vector$alt$BoneInfo$raw(::std::unique_ptr<::std::vector<::alt::BoneInfo>> *ptr, ::std::vector<::alt::BoneInfo> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::alt::BoneInfo>>(raw);
}
::std::vector<::alt::BoneInfo> const *cxxbridge1$unique_ptr$std$vector$alt$BoneInfo$get(::std::unique_ptr<::std::vector<::alt::BoneInfo>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::alt::BoneInfo> *cxxbridge1$unique_ptr$std$vector$alt$BoneInfo$release(::std::unique_ptr<::std::vector<::alt::BoneInfo>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$alt$BoneInfo$drop(::std::unique_ptr<::std::vector<::alt::BoneInfo>> *ptr) noexcept {
  ptr->~unique_ptr();
}

::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *cxxbridge1$std$vector$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$new() noexcept {
  return new ::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>();
}
::std::size_t cxxbridge1$std$vector$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$size(::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> const &s) noexcept {
  return s.size();
}
::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete *cxxbridge1$std$vector$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$get_unchecked(::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$null(::std::unique_ptr<::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>>();
}
void cxxbridge1$unique_ptr$std$vector$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$raw(::std::unique_ptr<::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>> *ptr, ::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>>(raw);
}
::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> const *cxxbridge1$unique_ptr$std$vector$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$get(::std::unique_ptr<::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete> *cxxbridge1$unique_ptr$std$vector$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$release(::std::unique_ptr<::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$Config_internal_ValueWrapper_Config_Value_AutocxxConcrete$drop(::std::unique_ptr<::std::vector<::Config_internal_ValueWrapper_Config_Value_AutocxxConcrete>> *ptr) noexcept {
  ptr->~unique_ptr();
}
} // extern "C"
