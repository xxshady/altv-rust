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
using MarkerType = uint32_t;
using WeaponDamageEventBodyPart = int8_t;
using EventType = uint16_t;
using PlayerConnectDeniedReason = uint8_t;
using ExplosionType = int8_t;

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

using ConfigDictPair = std::pair<std::string, Config::Value::ValuePtr>;

class ConfigDictPairWrapper {
public:
    std::shared_ptr<ConfigDictPair> ptr;

    ConfigDictPairWrapper clone() {
        ConfigDictPairWrapper instance;
        instance.ptr = this->ptr;

        return instance;
    }
};

class BaseObjectPtrWrapper {
public:
    std::shared_ptr<alt::IBaseObject*> ptr;

    BaseObjectPtrWrapper clone() {
        BaseObjectPtrWrapper instance;
        instance.ptr = this->ptr;

        return instance;
    }
};

using BaseObjectVector = std::vector<BaseObjectPtrWrapper>;

alt::IBaseObject* read_base_object_ptr_wrapper(const BaseObjectPtrWrapper& wrapper) {
    return *wrapper.ptr;
}

BaseObjectVector create_base_object_vec() {
    BaseObjectVector vec;
    return vec;
}

void push_to_base_object_vec(BaseObjectVector& base_object_vec, alt::IBaseObject* base_object) {
    BaseObjectPtrWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::IBaseObject*>(base_object);
    base_object_vec.push_back(wrapper.clone());
}

class PlayerPtrWrapper {
public:
    std::shared_ptr<alt::IPlayer*> ptr;

    PlayerPtrWrapper clone() {
        PlayerPtrWrapper instance;
        instance.ptr = this->ptr;

        return instance;
    }
};

alt::IPlayer* read_player_ptr_wrapper(const PlayerPtrWrapper& wrapper) {
    return *wrapper.ptr;
}

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

class ResourcePtrWrapper {
public:
    std::shared_ptr<alt::IResource*> ptr;

    ResourcePtrWrapper clone() {
        ResourcePtrWrapper instance;
        instance.ptr = this->ptr;

        return instance;
    }
};

using ResourceVector = std::vector<ResourcePtrWrapper>;

alt::IResource* read_resource_ptr_wrapper(const ResourcePtrWrapper& wrapper) {
    return *wrapper.ptr;
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

class Vector2Vec {
public:
    std::vector<Vector2Wrapper> vec{};

    std::vector<alt::Vector2f> into_alt_vec() {
        std::vector<alt::Vector2f> alt_vec;

        alt_vec.reserve(this->vec.size());
        for (const auto& e : this->vec) {
            alt_vec.push_back({ e.x, e.y });
        }
        this->vec.clear();

        return alt_vec;
    }
};

Vector2Vec create_vector2_vec() {
    return Vector2Vec{};
}

void push_to_vector2_vec(Vector2Vec& vec, f32 x, f32 y) {
    vec.vec.push_back({ x, y });
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

class FireInfoWrapper {
public:
    Vector3Wrapper pos;
    u32 weapon_hash;

    FireInfoWrapper(Vector3Wrapper _pos, u32 _weapon_hash):
        pos(_pos),
        weapon_hash(_weapon_hash)
    {}
};

Vector3Wrapper read_fire_info_pos(const FireInfoWrapper& fire) {
    return fire.pos;
}

u32 read_fire_info_weapon_hash(const FireInfoWrapper& fire) {
    return fire.weapon_hash;
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

alt::MValueArgs mvalue_wrapper_vec_to_alt(MValueWrapperVec mvalue_vec) {
    alt::MValueArgs args;
    auto size = mvalue_vec.size();
    for (alt::Size i = 0; i < size; ++i) {
        args.push_back(*(mvalue_vec[i].ptr));
    }
    return args;
}

void trigger_local_event(std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::ICore::Instance().TriggerLocalEvent(event_name, mvalue_wrapper_vec_to_alt(mvalue_vec));
}

std::vector<alt::IPlayer*> player_wrapper_vec_to_alt(PlayerVector player_vec) {
    std::vector<alt::IPlayer*> players;
    for (auto& player : player_vec)
    {
        players.push_back(*player.ptr);
    }
    return players;
}

void trigger_client_event(alt::IPlayer* player, std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::ICore::Instance().TriggerClientEvent(player, event_name, mvalue_wrapper_vec_to_alt(mvalue_vec));
}

void trigger_client_event_unreliable(alt::IPlayer* player, std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::ICore::Instance().TriggerClientEventUnreliable(player, event_name, mvalue_wrapper_vec_to_alt(mvalue_vec));
}

void trigger_client_event_for_some(PlayerVector players, std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::ICore::Instance().TriggerClientEvent(
        player_wrapper_vec_to_alt(players),
        event_name,
        mvalue_wrapper_vec_to_alt(mvalue_vec)
    );
}

void trigger_client_event_unreliable_for_some(PlayerVector players, std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::ICore::Instance().TriggerClientEventUnreliable(
        player_wrapper_vec_to_alt(players),
        event_name,
        mvalue_wrapper_vec_to_alt(mvalue_vec)
    );
}

void trigger_client_event_for_all(std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::ICore::Instance().TriggerClientEventForAll(event_name, mvalue_wrapper_vec_to_alt(mvalue_vec));
}

void trigger_client_event_unreliable_for_all(std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::ICore::Instance().TriggerClientEventUnreliableForAll(event_name, mvalue_wrapper_vec_to_alt(mvalue_vec));
}

namespace base_object
{
    alt::IWorldObject* to_world_object(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IWorldObject*>(base_object);
    }

    alt::IVirtualEntity* to_virtual_entity(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IVirtualEntity*>(base_object);
    }

    alt::IVirtualEntityGroup* to_virtual_entity_group(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IVirtualEntityGroup*>(base_object);
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

    alt::IColShape* to_col_shape(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IColShape*>(base_object);
    }

    alt::IBlip* to_blip(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IBlip*>(base_object);
    }

    alt::IVoiceChannel* to_voice_channel(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IVoiceChannel*>(base_object);
    }

    alt::IMarker* to_marker(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::IMarker*>(base_object);
    }

    alt::ICheckpoint* to_checkpoint(alt::IBaseObject* base_object) {
        return dynamic_cast<alt::ICheckpoint*>(base_object);
    }
} // namespace base_object

namespace world_object
{
    alt::IBaseObject* to_base_object(alt::IWorldObject* world_object) {
        return static_cast<alt::IBaseObject*>(world_object);
    }
} // namespace base_object

namespace entity
{
    alt::IBaseObject* to_base_object(alt::IEntity* entity) {
        return static_cast<alt::IBaseObject*>(entity);
    }
} // namespace entity

namespace player
{
    alt::IEntity* to_entity(alt::IPlayer* player) {
        return static_cast<alt::IEntity*>(player);
    }

    alt::IBaseObject* to_base_object(alt::IPlayer* player) {
        return static_cast<alt::IBaseObject*>(player);
    }
} // namespace player

namespace col_shape {
    alt::IBaseObject* to_base_object(alt::IColShape* col_shape) {
        return static_cast<alt::IBaseObject*>(col_shape);
    }
} // namespace colshape

namespace vehicle
{
    alt::IBaseObject* to_base_object(alt::IVehicle* vehicle) {
        return static_cast<alt::IBaseObject*>(vehicle);
    }

    alt::IEntity* to_entity(alt::IVehicle* vehicle) {
        return static_cast<alt::IEntity*>(vehicle);
    }
} // namespace vehicle

namespace virtual_entity
{
    alt::IBaseObject* to_base_object(alt::IVirtualEntity* vehicle) {
        return static_cast<alt::IBaseObject*>(vehicle);
    }
} // namespace virtual_entity

namespace virtual_entity_group
{
    alt::IBaseObject* to_base_object(alt::IVirtualEntityGroup* vehicle) {
        return static_cast<alt::IBaseObject*>(vehicle);
    }
} // namespace virtual_entity_group

namespace blip {
    alt::IBaseObject* to_base_object(alt::IBlip* blip) {
        return static_cast<alt::IBaseObject*>(blip);
    }
} // namespace blip

namespace voice_channel {
    alt::IBaseObject* to_base_object(alt::IVoiceChannel* voice_channel) {
        return static_cast<alt::IBaseObject*>(voice_channel);
    }
} // namespace voice_channel

namespace marker {
    alt::IBaseObject* to_base_object(alt::IMarker* marker) {
        return static_cast<alt::IBaseObject*>(marker);
    }
} // namespace marker

namespace checkpoint {
    alt::IBaseObject* to_base_object(alt::ICheckpoint* checkpoint) {
        return static_cast<alt::IBaseObject*>(checkpoint);
    }
} // namespace checkpoint

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

void read_bone_info(const alt::BoneInfo& bone, u16* id, u16* index) {
    *id = bone.id;
    *index = bone.index;
}

std::string read_bone_info_name(const alt::BoneInfo& bone) {
    return bone.name;
}

bool is_vehicle_model_info_valid(const alt::VehicleModelInfo* ptr) {
    return ptr->modelType != alt::VehicleModelInfo::Type::INVALID;
}

void read_vehicle_model_info(
    const alt::VehicleModelInfo* ptr,
    u8* out_model_type,
    u8* out_wheels_count,
    bool* out_has_armored_windows,
    u8* out_primary_color,
    u8* out_secondary_color,
    u8* out_pearl_color,
    u8* out_wheels_color,
    u8* out_interior_color,
    u8* out_dashboard_color,
    bool* out_modkits,
    bool* out_has_auto_attach_trailer
) {
    *out_model_type = static_cast<u8>(ptr->modelType);
    *out_wheels_count = ptr->wheelsCount;
    *out_has_armored_windows = ptr->hasArmoredWindows;
    *out_primary_color = ptr->primaryColor;
    *out_secondary_color = ptr->secondaryColor;
    *out_pearl_color = ptr->pearlColor;
    *out_wheels_color = ptr->wheelsColor;
    *out_interior_color = ptr->interiorColor;
    *out_dashboard_color = ptr->dashboardColor;

    size_t modkits_size = std::size(ptr->modkits);
    for (size_t i = 0; i < modkits_size; ++i)
    {
        // wtf?
        out_modkits[i] = ptr->modkits[i] != 0xFFFF;
    }

    *out_has_auto_attach_trailer = ptr->hasAutoAttachTrailer;
}

std::string read_vehicle_model_info_title(const alt::VehicleModelInfo* ptr) {
    return ptr->title;
}

std::vector<alt::BoneInfo> read_vehicle_model_info_bones(const alt::VehicleModelInfo* ptr) {
    return ptr->bones;
}

bool is_ped_model_info_valid(const alt::PedModelInfo* ptr) {
    return ptr->hash != 0;
}

std::vector<alt::BoneInfo> read_ped_model_info_bones(const alt::PedModelInfo* ptr) {
    return ptr->bones;
}

u32 read_ped_model_info_hash(const alt::PedModelInfo* ptr) {
    return ptr->hash;
}

std::string read_ped_model_info_name(const alt::PedModelInfo* ptr) {
    return ptr->name;
}

std::string read_ped_model_info_type(const alt::PedModelInfo* ptr) {
    return ptr->type;
}

std::string read_ped_model_info_dlc_name(const alt::PedModelInfo* ptr) {
    return ptr->dlcName;
}

std::string read_ped_model_info_movement_clip_set(const alt::PedModelInfo* ptr) {
    return ptr->movementClipSet;
}

std::string read_ped_model_info_default_unarmed_weapon(const alt::PedModelInfo* ptr) {
    return ptr->defaultUnarmedWeapon;
}

void read_quaternion(const alt::Quaternion& quat, f32* out_x, f32* out_y, f32* out_z, f32* out_w) {
    *out_x = quat.x;
    *out_y = quat.y;
    *out_z = quat.z;
    *out_w = quat.w;
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

    // mutable pointer because of SetDamageValue
    alt::CWeaponDamageEvent* to_CWeaponDamageEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::WEAPON_DAMAGE_EVENT);
        return static_cast<alt::CWeaponDamageEvent*>(const_cast<alt::CEvent*>(event));
    }

    const alt::CPlayerDeathEvent* to_CPlayerDeathEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_DEATH);
        return static_cast<const alt::CPlayerDeathEvent*>(event);
    }

    const alt::CPlayerDamageEvent* to_CPlayerDamageEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_DAMAGE);
        return static_cast<const alt::CPlayerDamageEvent*>(event);
    }

    const alt::CPlayerEnteringVehicleEvent* to_CPlayerEnteringVehicleEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_ENTERING_VEHICLE);
        return static_cast<const alt::CPlayerEnteringVehicleEvent*>(event);
    }

    const alt::CPlayerEnterVehicleEvent* to_CPlayerEnterVehicleEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_ENTER_VEHICLE);
        return static_cast<const alt::CPlayerEnterVehicleEvent*>(event);
    }

    const alt::CPlayerLeaveVehicleEvent* to_CPlayerLeaveVehicleEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_LEAVE_VEHICLE);
        return static_cast<const alt::CPlayerLeaveVehicleEvent*>(event);
    }

    const alt::CPlayerChangeAnimationEvent* to_CPlayerChangeAnimationEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_CHANGE_ANIMATION_EVENT);
        return static_cast<const alt::CPlayerChangeAnimationEvent*>(event);
    }

    const alt::CPlayerChangeVehicleSeatEvent* to_CPlayerChangeVehicleSeatEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_CHANGE_VEHICLE_SEAT);
        return static_cast<const alt::CPlayerChangeVehicleSeatEvent*>(event);
    }

    const alt::CPlayerWeaponChangeEvent* to_CPlayerWeaponChangeEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_WEAPON_CHANGE);
        return static_cast<const alt::CPlayerWeaponChangeEvent*>(event);
    }

    const alt::CPlayerConnectDeniedEvent* to_CPlayerConnectDeniedEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_CONNECT_DENIED);
        return static_cast<const alt::CPlayerConnectDeniedEvent*>(event);
    }

    const alt::CPlayerSpawnEvent* to_CPlayerSpawnEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_SPAWN);
        return static_cast<const alt::CPlayerSpawnEvent*>(event);
    }

    const alt::CStartProjectileEvent* to_CStartProjectileEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::START_PROJECTILE_EVENT);
        return static_cast<const alt::CStartProjectileEvent*>(event);
    }

    const alt::CPlayerRequestControlEvent* to_CPlayerRequestControlEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_REQUEST_CONTROL);
        return static_cast<const alt::CPlayerRequestControlEvent*>(event);
    }

    const alt::CPlayerDimensionChangeEvent* to_CPlayerDimensionChangeEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_DIMENSION_CHANGE);
        return static_cast<const alt::CPlayerDimensionChangeEvent*>(event);
    }

    const alt::CPlayerChangeInteriorEvent* to_CPlayerChangeInteriorEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::PLAYER_CHANGE_INTERIOR_EVENT);
        return static_cast<const alt::CPlayerChangeInteriorEvent*>(event);
    }

    const alt::CExplosionEvent* to_CExplosionEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::EXPLOSION_EVENT);
        return static_cast<const alt::CExplosionEvent*>(event);
    }

    const alt::CFireEvent* to_CFireEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::FIRE_EVENT);
        return static_cast<const alt::CFireEvent*>(event);
    }

    const alt::CConnectionQueueAddEvent* to_CConnectionQueueAddEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::CONNECTION_QUEUE_ADD);
        return static_cast<const alt::CConnectionQueueAddEvent*>(event);
    }

    const alt::CConnectionQueueRemoveEvent* to_CConnectionQueueRemoveEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::CONNECTION_QUEUE_REMOVE);
        return static_cast<const alt::CConnectionQueueRemoveEvent*>(event);
    }

    const alt::CVehicleAttachEvent* to_CVehicleAttachEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::VEHICLE_ATTACH);
        return static_cast<const alt::CVehicleAttachEvent*>(event);
    }

    const alt::CVehicleDetachEvent* to_CVehicleDetachEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::VEHICLE_DETACH);
        return static_cast<const alt::CVehicleDetachEvent*>(event);
    }

    const alt::CVehicleDestroyEvent* to_CVehicleDestroyEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::VEHICLE_DESTROY);
        return static_cast<const alt::CVehicleDestroyEvent*>(event);
    }

    const alt::CVehicleDamageEvent* to_CVehicleDamageEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::VEHICLE_DAMAGE);
        return static_cast<const alt::CVehicleDamageEvent*>(event);
    }

    const alt::CVehicleHornEvent* to_CVehicleHornEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::VEHICLE_HORN);
        return static_cast<const alt::CVehicleHornEvent*>(event);
    }

    const alt::CVehicleSirenEvent* to_CVehicleSirenEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::VEHICLE_SIREN);
        return static_cast<const alt::CVehicleSirenEvent*>(event);
    }

    const alt::CNetOwnerChangeEvent* to_CNetOwnerChangeEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::NETOWNER_CHANGE);
        return static_cast<const alt::CNetOwnerChangeEvent*>(event);
    }

    const alt::CMetaChangeEvent* to_CMetaChangeEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::META_CHANGE);
        return static_cast<const alt::CMetaChangeEvent*>(event);
    }

    const alt::CGlobalMetaDataChangeEvent* to_CGlobalMetaDataChangeEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::GLOBAL_META_CHANGE);
        return static_cast<const alt::CGlobalMetaDataChangeEvent*>(event);
    }

    const alt::CGlobalSyncedMetaDataChangeEvent* to_CGlobalSyncedMetaDataChangeEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::GLOBAL_SYNCED_META_CHANGE);
        return static_cast<const alt::CGlobalSyncedMetaDataChangeEvent*>(event);
    }

    const alt::CSyncedMetaDataChangeEvent* to_CSyncedMetaDataChangeEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::SYNCED_META_CHANGE);
        return static_cast<const alt::CSyncedMetaDataChangeEvent*>(event);
    }

    const alt::CStreamSyncedMetaDataChangeEvent* to_CStreamSyncedMetaDataChangeEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::STREAM_SYNCED_META_CHANGE);
        return static_cast<const alt::CStreamSyncedMetaDataChangeEvent*>(event);
    }

    const alt::CLocalMetaDataChangeEvent* to_CLocalMetaDataChangeEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::LOCAL_SYNCED_META_CHANGE);
        return static_cast<const alt::CLocalMetaDataChangeEvent*>(event);
    }

    const alt::CResourceStopEvent* to_CResourceStopEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::RESOURCE_STOP);
        return static_cast<const alt::CResourceStopEvent*>(event);
    }

    const alt::CResourceStartEvent* to_CResourceStartEvent(const alt::CEvent* event) {
        assert(event->GetType() == alt::CEvent::Type::RESOURCE_START);
        return static_cast<const alt::CResourceStartEvent*>(event);
    }
} // namespace events

namespace config_node
{
    u8 get_type(const Config::Value::ValuePtr& node) {
        return static_cast<u8>(node->GetType());
    }

    bool read_bool(Config::Value::ValuePtr node) {
        assert(node->GetType() == Config::Value::Type::BOOL);
        return node->AsBool();
    }

    f64 read_f64(Config::Value::ValuePtr node) {
        assert(node->GetType() == Config::Value::Type::NUMBER);
        return node->AsNumber();
    }

    std::string read_string(Config::Value::ValuePtr node) {
        assert(node->GetType() == Config::Value::Type::STRING);
        return node->AsString();
    }

    std::vector<Config::Value::ValuePtr> read_list(Config::Value::ValuePtr node) {
        assert(node->GetType() == Config::Value::Type::LIST);
        return node->AsList();
    }

    Config::Value::ValuePtr copy_value_ptr(const Config::Value::ValuePtr& node) {
        return node;
    }

    std::vector<ConfigDictPairWrapper> read_dict(Config::Value::ValuePtr node) {
        assert(node->GetType() == Config::Value::Type::DICT);
        std::vector<ConfigDictPairWrapper> vec;

        for (auto& pair : node->AsDict()) {
            // TODO: handle empty/none node?
            ConfigDictPairWrapper wrapper;
            wrapper.ptr = std::make_shared<ConfigDictPair>(std::pair { pair.first, pair.second });
            vec.push_back(wrapper.clone());
        }

        return vec;
    }

    std::string read_dict_pair_key(const ConfigDictPairWrapper& pair) {
        return pair.ptr->first;
    }

    Config::Value::ValuePtr read_dict_pair_value(const ConfigDictPairWrapper& pair) {
        return pair.ptr->second;
    }
} // namespace config
