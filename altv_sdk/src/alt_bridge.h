#pragma once

#define ALT_SERVER_API

#include <memory>
#include <utility>
#include "shared.h"
#include "runtime.h"
#include "unordered_set"

using ResourceOnRemoveBaseObjectCallback = shared::ResourceOnRemoveBaseObjectCallback;
using ResourceStartCallback = shared::ResourceStartCallback;

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
using WeaponDamageEventBodyPart = int8_t;
using EventType = uint16_t;

// used for const std::string& return values in altv event classes
using StdStringClone = std::string;

void set_alt_core(alt::ICore* core) {
    alt::ICore::SetInstance(core);
}

alt::ICore* get_alt_core() {
    return &alt::ICore::Instance();
}

alt::IScriptRuntime* create_script_runtime() {
    return new RustRuntime();
}

void register_script_runtime(
    alt::ICore* core,
    std::string resource_type,
    alt::IScriptRuntime* runtime
) {
    core->RegisterScriptRuntime(resource_type, runtime);
    RustRuntime::set_instance(static_cast<RustRuntime*>(runtime));
}

// mvalue

class MValueWrapper {
public:
    std::shared_ptr<alt::MValueConst> ptr;

    MValueWrapper clone() {
        MValueWrapper instance;
        instance.ptr = this->ptr;

        return instance;
    }
};

class MValueMutWrapper {
public:
    std::shared_ptr<alt::MValue> ptr;

    MValueMutWrapper clone() {
        MValueMutWrapper instance;
        instance.ptr = this->ptr;

        return instance;
    }
};

MValueWrapper convert_mvalue_mut_wrapper_to_const(MValueMutWrapper mut_wrapper) {
    MValueWrapper wrapper;
    // is this even legal?
    wrapper.ptr = std::make_shared<alt::MValueConst>(*mut_wrapper.ptr);
    mut_wrapper.ptr = nullptr;
    return wrapper;
}

using MValueDictPair = std::pair<std::string, MValueWrapper>;

class MValueDictPairWrapper {
public:
    std::shared_ptr<MValueDictPair> ptr;

    MValueDictPairWrapper clone() {
        MValueDictPairWrapper instance;
        instance.ptr = this->ptr;

        return instance;
    }
};

class PlayerPtrWrapper {
public:
    std::shared_ptr<alt::IPlayer*> ptr;

    PlayerPtrWrapper clone() {
        PlayerPtrWrapper instance;
        instance.ptr = this->ptr;

        return instance;
    }
};

using PlayerVector = std::vector<PlayerPtrWrapper>;

PlayerVector create_player_vec() {
    PlayerVector vec;
    return vec;
}

void push_to_player_vec(PlayerVector& player_vec, alt::IPlayer* player) {
    PlayerPtrWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::IPlayer*>(player);
    player_vec.push_back(wrapper.clone());
}

class Vector3Wrapper {
public:
    f32 x = 0;
    f32 y = 0;
    f32 z = 0;

    Vector3Wrapper(f32 _x, f32 _y, f32 _z): x(_x), y(_y), z(_z) {}
};

void read_vector3(const Vector3Wrapper& vector3, f32* out_x, f32* out_y, f32* out_z) {
    *out_x = vector3.x;
    *out_y = vector3.y;
    *out_z = vector3.z;
}

class Vector2Wrapper {
public:
    f32 x = 0;
    f32 y = 0;

    Vector2Wrapper(f32 _x, f32 _y): x(_x), y(_y) {}
};

void read_vector2(const Vector2Wrapper& vector2, f32* out_x, f32* out_y) {
    *out_x = vector2.x;
    *out_y = vector2.y;
}

class RGBAWrapper {
public:
    u8 r = 0;
    u8 g = 0;
    u8 b = 0;
    u8 a = 0;

    RGBAWrapper(u8 _r, u8 _g, u8 _b, u8 _a): r(_r), g(_g), b(_b), a(_a) {}
};

void read_rgba(const RGBAWrapper& rgba, u8* out_r, u8* out_g, u8* out_b, u8* out_a) {
    *out_r = rgba.r;
    *out_g = rgba.g;
    *out_b = rgba.b;
    *out_a = rgba.a;
}

class WeaponWrapper {
public:
    u32 hash = 0;
    u8 tint_index = 0;
    std::vector<u32> components {};

    WeaponWrapper(u32 _hash, u8 _tint_index, std::unordered_set<u32> _components):
        hash(_hash),
        tint_index(_tint_index)
    {
        this->components.reserve(_components.size());
        for (const auto& comp : _components) {
            this->components.push_back(comp);
        }
    }
};

void read_weapon(const WeaponWrapper& weapon, u32* out_hash, u8* out_tint_index) {
    *out_hash = weapon.hash;
    *out_tint_index = weapon.tint_index;
}

std::vector<u32> read_weapon_components(const WeaponWrapper& weapon) {
    return weapon.components;
}

using MValueWrapperVec = std::vector<MValueWrapper>;

MValueWrapperVec create_mvalue_vec() {
    MValueWrapperVec vec;
    return vec;
}

void push_to_mvalue_vec(MValueWrapperVec& mvalue_vec, MValueMutWrapper mvalue) {
    mvalue_vec.push_back(convert_mvalue_mut_wrapper_to_const(mvalue));
}

u8 get_mvalue_type(MValueWrapper mvalue) {
    return static_cast<u8>(mvalue.ptr->Get()->GetType());
}

bool get_mvalue_bool(MValueWrapper mvalue) {
    assert(mvalue.ptr->Get()->GetType() == alt::IMValue::Type::BOOL);
    return mvalue.ptr->As<alt::IMValueBool>().Get()->Value();
}

f64 get_mvalue_double(MValueWrapper mvalue) {
    assert(mvalue.ptr->Get()->GetType() == alt::IMValue::Type::DOUBLE);
    return mvalue.ptr->As<alt::IMValueDouble>().Get()->Value();
}

std::string get_mvalue_string(MValueWrapper mvalue) {
    assert(mvalue.ptr->Get()->GetType() == alt::IMValue::Type::STRING);
    return mvalue.ptr->As<alt::IMValueString>().Get()->Value();
}

i64 get_mvalue_int(MValueWrapper mvalue) {
    assert(mvalue.ptr->Get()->GetType() == alt::IMValue::Type::INT);
    return mvalue.ptr->As<alt::IMValueInt>().Get()->Value();
}

u64 get_mvalue_uint(MValueWrapper mvalue) {
    assert(mvalue.ptr->Get()->GetType() == alt::IMValue::Type::UINT);
    return mvalue.ptr->As<alt::IMValueUInt>().Get()->Value();
}

MValueWrapperVec get_mvalue_list(MValueWrapper mvalue) {
    assert(mvalue.ptr->Get()->GetType() == alt::IMValue::Type::LIST);

    auto list = mvalue.ptr->As<alt::IMValueList>().Get();
    auto size = list->GetSize();

    auto mvalue_vec = create_mvalue_vec();

    for (alt::Size i = 0; i < size; ++i) {
        MValueWrapper wrapper;
        wrapper.ptr = std::make_shared<alt::MValueConst>(list->Get(i));
        mvalue_vec.push_back(wrapper.clone());
    }

    return mvalue_vec;
}

std::vector<MValueDictPairWrapper> get_mvalue_dict(MValueWrapper mvalue) {
    assert(mvalue.ptr->Get()->GetType() == alt::IMValue::Type::DICT);
    auto dict = mvalue.ptr->As<alt::IMValueDict>().Get();

    std::vector<MValueDictPairWrapper> vec;

    for (auto it = dict->Begin(); it; it = dict->Next()) {
        MValueWrapper value_wrapper;
        value_wrapper.ptr = std::make_shared<alt::MValueConst>(it->GetValue());

        MValueDictPairWrapper pair;
        pair.ptr = std::make_shared<MValueDictPair>(std::pair { it->GetKey(), value_wrapper.clone() });
        vec.push_back(pair.clone());
    }

    return vec;
}

std::string get_mvalue_dict_pair_key(const MValueDictPairWrapper& pair) {
    return pair.ptr->first;
}

MValueWrapper get_mvalue_dict_pair_value(const MValueDictPairWrapper& pair) {
    return pair.ptr->second.clone();
}

alt::IBaseObject* get_mvalue_base_object(MValueWrapper mvalue) {
    assert(mvalue.ptr->Get()->GetType() == alt::IMValue::Type::BASE_OBJECT);
    return mvalue.ptr->As<alt::IMValueBaseObject>().Get()->RawValue();
}

Vector3Wrapper get_mvalue_vector3(MValueWrapper mvalue) {
    assert(mvalue.ptr->Get()->GetType() == alt::IMValue::Type::VECTOR3);
    auto vector3 = mvalue.ptr->As<alt::IMValueVector3>().Get()->Value();
    return { vector3[0], vector3[1], vector3[2] };
}

Vector2Wrapper get_mvalue_vector2(MValueWrapper mvalue) {
    assert(mvalue.ptr->Get()->GetType() == alt::IMValue::Type::VECTOR2);
    auto vector2 = mvalue.ptr->As<alt::IMValueVector2>().Get()->Value();
    return { vector2[0], vector2[1] };
}

MValueMutWrapper create_mvalue_bool(bool value) {
    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(alt::ICore::Instance().CreateMValueBool(value));
    return wrapper;
}

MValueMutWrapper create_mvalue_double(f64 value) {
    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(alt::ICore::Instance().CreateMValueDouble(value));
    return wrapper;
}

MValueMutWrapper create_mvalue_string(std::string value) {
    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(alt::ICore::Instance().CreateMValueString(value));
    return wrapper;
}

MValueMutWrapper create_mvalue_nil() {
    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(alt::ICore::Instance().CreateMValueNil());
    return wrapper;
}

MValueMutWrapper create_mvalue_int(i64 value) {
    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(alt::ICore::Instance().CreateMValueInt(value));
    return wrapper;
}

MValueMutWrapper create_mvalue_uint(u64 value) {
    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(alt::ICore::Instance().CreateMValueUInt(value));
    return wrapper;
}

MValueMutWrapper create_mvalue_list(MValueWrapperVec mvalue_vec) {
    auto mvalue_list = alt::ICore::Instance().CreateMValueList();
    auto size = mvalue_vec.size();

    for (size_t i = 0; i < size; ++i) {
        mvalue_list->PushConst(*(mvalue_vec[i].ptr));
    }

    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(mvalue_list);
    return wrapper;
}

MValueMutWrapper create_mvalue_dict() {
    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(alt::ICore::Instance().CreateMValueDict());
    return wrapper;
}

void push_to_mvalue_dict(MValueMutWrapper& dict, std::string key, MValueMutWrapper mvalue) {
    assert(dict.ptr->Get()->GetType() == alt::IMValue::Type::DICT);
    dict.ptr->As<alt::IMValueDict>().Get()->SetConst(key, *(mvalue.ptr));
}

MValueMutWrapper create_mvalue_base_object(alt::IBaseObject* value) {
    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(alt::ICore::Instance().CreateMValueBaseObject(value));
    return wrapper;
}

MValueMutWrapper create_mvalue_vector3(f32 x, f32 y, f32 z) {
    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(alt::ICore::Instance().CreateMValueVector3({ x, y, z }));
    return wrapper;
}

MValueMutWrapper create_mvalue_vector2(f32 x, f32 y) {
    MValueMutWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValue>(alt::ICore::Instance().CreateMValueVector2({ x, y }));
    return wrapper;
}

// events

void trigger_local_event(std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::MValueArgs args;
    auto size = mvalue_vec.size();

    for (alt::Size i = 0; i < size; ++i) {
        args.Push(*(mvalue_vec[i].ptr));
    }

    alt::ICore::Instance().TriggerLocalEvent(event_name, args);
}

void trigger_client_event(alt::IPlayer* player, std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::MValueArgs args;
    auto size = mvalue_vec.size();

    for (alt::Size i = 0; i < size; ++i) {
        args.Push(*(mvalue_vec[i].ptr));
    }

    alt::ICore::Instance().TriggerClientEvent(player, event_name, args);
}

void trigger_client_event_for_some(PlayerVector players, std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::MValueArgs args;
    auto size = mvalue_vec.size();

    for (alt::Size i = 0; i < size; ++i) {
        args.Push(*(mvalue_vec[i].ptr));
    }

    std::vector<alt::IPlayer*> normal_players_vec;
    for (auto& player : players)
    {
        normal_players_vec.push_back(*player.ptr);
    }

    alt::ICore::Instance().TriggerClientEvent(normal_players_vec, event_name, args);
}

void trigger_client_event_for_all(std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::MValueArgs args;
    auto size = mvalue_vec.size();

    for (alt::Size i = 0; i < size; ++i) {
        args.Push(*(mvalue_vec[i].ptr));
    }

    alt::ICore::Instance().TriggerClientEventForAll(event_name, args);
}

namespace base_object
{
    alt::IWorldObject* to_world_object(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IWorldObject*>(base_object);
    }

    alt::IEntity* to_entity(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IEntity*>(base_object);
    }

    alt::IVehicle* to_vehicle(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IVehicle*>(base_object);
    }

    alt::IPlayer* to_player(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IPlayer*>(base_object);
    }
} // namespace base_object

namespace world_object
{

} // namespace base_object

namespace entity
{

} // namespace entity

namespace player
{
    alt::IEntity* to_entity(alt::IPlayer* player) {
        return static_cast<alt::IEntity*>(player);
    }

    // player conversions
    alt::IBaseObject* to_base_object(alt::IPlayer* player) {
        return static_cast<alt::IBaseObject*>(player);
    }
} // namespace player

namespace vehicle
{
    alt::IBaseObject* to_base_object(alt::IVehicle* vehicle) {
        return static_cast<alt::IBaseObject*>(vehicle);
    }

    alt::IEntity* to_entity(alt::IVehicle* vehicle) {
        return static_cast<alt::IEntity*>(vehicle);
    }
} // namespace vehicle

void read_alt_prop(const alt::Prop& prop, u16* out_drawable, u8* out_texture) {
    *out_drawable = prop.drawableId;
    *out_texture = prop.textureId;
}

void read_alt_dlc_prop(const alt::DlcProp& prop, u8* out_drawable, u8* out_texture, u32* out_dlc) {
    *out_drawable = prop.drawableId;
    *out_texture = prop.textureId;
    *out_dlc = prop.dlc;
}

void read_alt_cloth(const alt::Cloth& cloth, u16* out_drawable, u8* out_texture, u8* out_palette) {
    *out_drawable = cloth.drawableId;
    *out_texture = cloth.textureId;
    *out_palette = cloth.paletteId;
}

void read_alt_dlc_cloth(const alt::DlcCloth& cloth, u16* out_drawable, u8* out_texture, u8* out_palette, u32* out_dlc) {
    *out_drawable = cloth.drawableId;
    *out_texture = cloth.textureId;
    *out_palette = cloth.paletteId;
    *out_dlc = cloth.dlc;
}

void read_alt_head_overlay(
    const alt::HeadOverlay& overlay,
    u8* out_index,
    f32* out_opacity,
    u8* out_color_type,
    u8* out_color_index,
    u8* out_second_color_index
) {
    *out_index = overlay.index;
    *out_opacity = overlay.opacity;
    *out_color_type = overlay.colorType;
    *out_color_index = overlay.colorIndex;
    *out_second_color_index = overlay.secondColorIndex;
}

void read_alt_head_blend_data(
    const alt::HeadBlendData& blend_data,
    u32* out_shape_first_id,
    u32* out_shape_second_id,
    u32* out_shape_third_id,
    u32* out_skin_first_id,
    u32* out_skin_second_id,
    u32* out_skin_third_id,
    f32* out_shape_mix,
    f32* out_skin_mix,
    f32* out_third_mix
) {
    *out_shape_first_id = blend_data.shapeFirstID;
    *out_shape_second_id = blend_data.shapeSecondID;
    *out_shape_third_id = blend_data.shapeThirdID;
    *out_skin_first_id = blend_data.skinFirstID;
    *out_skin_second_id = blend_data.skinSecondID;
    *out_skin_third_id = blend_data.skinThirdID;
    *out_shape_mix = blend_data.shapeMix;
    *out_skin_mix = blend_data.skinMix;
    *out_third_mix = blend_data.thirdMix;
}

namespace events
{
    const alt::CConsoleCommandEvent* to_CConsoleCommandEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::CONSOLE_COMMAND_EVENT);
        return static_cast<const alt::CConsoleCommandEvent*>(event);
    }

    const alt::CServerScriptEvent* to_CServerScriptEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::SERVER_SCRIPT_EVENT);
        return static_cast<const alt::CServerScriptEvent*>(event);
    }

    const alt::CClientScriptEvent* to_CClientScriptEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::CLIENT_SCRIPT_EVENT);
        return static_cast<const alt::CClientScriptEvent*>(event);
    }

    const alt::CPlayerDisconnectEvent* to_CPlayerDisconnectEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_DISCONNECT);
        return static_cast<const alt::CPlayerDisconnectEvent*>(event);
    }

    const alt::CPlayerConnectEvent* to_CPlayerConnectEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_CONNECT);
        return static_cast<const alt::CPlayerConnectEvent*>(event);
    }

    const alt::CColShapeEvent* to_CColShapeEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::COLSHAPE_EVENT);
        return static_cast<const alt::CColShapeEvent*>(event);
    }

    const alt::CWeaponDamageEvent* to_CWeaponDamageEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::WEAPON_DAMAGE_EVENT);
        return static_cast<const alt::CWeaponDamageEvent*>(event);
    }
} // namespace events
