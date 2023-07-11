#pragma once

#include "../cpp-sdk/SDK.h"
#include "../cpp-sdk/events/CMetaDataChangeEvent.h"
#include <string>
#include "cxx.h"

using StdStringPtr = std::unique_ptr<std::string>;

using u8 = uint8_t;
using u16 = uint16_t;
using u32 = uint32_t;
using u64 = uint64_t;

using i8 = int8_t;
using i16 = int16_t;
using i32 = int32_t;
using cpp_int = int; // why? for some reason sdk uses int and int32_t at the same time
using i64 = int64_t;

using f32 = float;
using f64 = double;

using BaseObjectType = uint8_t;
using ColShapeType = uint8_t;
using BlipType = uint8_t;
using MarkerType = uint32_t;
using WeaponDamageEventBodyPart = int8_t;
using EventType = uint16_t;
using PlayerConnectDeniedReason = uint8_t;
using ExplosionType = int8_t;

namespace shared {
    using AltResourceImpl = alt::IResource::Impl;
    using AltResource = alt::IResource;

    using ResourceStartCallback = void (*)(rust::Str name, rust::Str full_main_path, AltResourceImpl* resource_impl, AltResource* resource);
    using ResourceStopCallback = void (*)(rust::Str name);
    using RuntimeResourceDestroyImplCallback = void (*)();
    using RuntimeOnTickCallback = void (*)();
    using ResourceOnEventCallback = void (*)(rust::Str name, const alt::CEvent* event);
    using ResourceOnCreateBaseObjectCallback = void (*)(rust::Str name, alt::IBaseObject* base_object);
    using ResourceOnRemoveBaseObjectCallback = void (*)(rust::Str name, alt::IBaseObject* base_object);
}
