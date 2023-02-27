#pragma once

#define ALT_SERVER_API

#include <memory>
#include "shared.h"
#include "runtime.h"

using ResourceOnRemoveBaseObjectCallback = shared::ResourceOnRemoveBaseObjectCallback;
using ResourceStartCallback = shared::ResourceStartCallback;

using StdStringPtr = std::unique_ptr<std::string>;
using StdStringVector = std::vector<std::string>;

using u8 = uint8_t;
using u16 = uint16_t;
using u32 = uint32_t;
using u64 = uint64_t;
using i64 = int64_t;
using f32 = float;
using f64 = double;

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

using MValueWrapperVec = std::vector<MValueWrapper>;

MValueWrapperVec create_mvalue_vec() {
    MValueWrapperVec vec;
    return vec;
}

void push_to_mvalue_vec(MValueWrapperVec& mvalue_vec, MValueWrapper mvalue) {
    mvalue_vec.push_back(mvalue.clone());
}

MValueWrapper create_mvalue_bool(bool value) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(alt::ICore::Instance().CreateMValueBool(value));
    return wrapper;
}

MValueWrapper create_mvalue_double(f64 value) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(alt::ICore::Instance().CreateMValueDouble(value));
    return wrapper;
}

MValueWrapper create_mvalue_string(std::string value) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(alt::ICore::Instance().CreateMValueString(value));
    return wrapper;
}

MValueWrapper create_mvalue_nil() {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(alt::ICore::Instance().CreateMValueNil());
    return wrapper;
}

MValueWrapper create_mvalue_int(i64 value) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(alt::ICore::Instance().CreateMValueInt(value));
    return wrapper;
}

MValueWrapper create_mvalue_uint(u64 value) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(alt::ICore::Instance().CreateMValueUInt(value));
    return wrapper;
}

// events

void trigger_local_event(std::string event_name, MValueWrapperVec mvalue_vec) {
    alt::MValueArgs args;
    for (int i = 0; i < mvalue_vec.size(); ++i)
    {
        args.Push(*(mvalue_vec[i].ptr));
    }
    alt::ICore::Instance().TriggerLocalEvent(event_name, args);
}

void toggle_event_type(u16 event_type, bool state) {
    alt::ICore::Instance().ToggleEvent(static_cast<alt::CEvent::Type>(event_type), state);
}

u16 get_event_type(const alt::CEvent* event) {
    return static_cast<u16>(event->GetType());
}

alt::IPlayer* get_event_player_target(const alt::CEvent* event) {
    auto type = event->GetType();

    switch (type)
    {
    case alt::CEvent::Type::PLAYER_CONNECT:
        return static_cast<const alt::CPlayerConnectEvent*>(event)->GetTarget();
    case alt::CEvent::Type::PLAYER_DISCONNECT:
        return static_cast<const alt::CPlayerDisconnectEvent*>(event)->GetTarget();
    default:
        alt::ICore::Instance().LogError(
            "get_event_player_target unknown event type: " +
            std::to_string(static_cast<u16>(type))
        );
        return nullptr;
    }
}

StdStringPtr get_event_reason(const alt::CEvent* event) {
    auto type = event->GetType();

    switch (type)
    {
    case alt::CEvent::Type::PLAYER_DISCONNECT:
        return std::make_unique<std::string>(std::string{
            static_cast<const alt::CPlayerDisconnectEvent*>(event)->GetReason()
        });
        break;

    default:
        alt::ICore::Instance().LogError(
            "get_event_reason unknown event type: " +
            std::to_string(static_cast<u16>(type))
        );
        return std::make_unique<std::string>(std::string{ "" });
        break;
    }
}

StdStringPtr get_event_console_command_name(const alt::CEvent* event) {
    auto type = event->GetType();

    switch (type)
    {
    case alt::CEvent::Type::CONSOLE_COMMAND_EVENT:
        return std::make_unique<std::string>(std::string{
            static_cast<const alt::CConsoleCommandEvent*>(event)->GetName()
        });
        break;

    default:
        alt::ICore::Instance().LogError(
            "get_event_console_command_name unknown event type: " +
            std::to_string(static_cast<u16>(type))
        );
        return std::make_unique<std::string>(std::string{ "" });
        break;
    }
}

std::unique_ptr<StdStringVector> get_event_console_command_args(const alt::CEvent* event) {
    auto type = event->GetType();

    switch (type)
    {
    case alt::CEvent::Type::CONSOLE_COMMAND_EVENT:
        return std::make_unique<StdStringVector>(static_cast<const alt::CConsoleCommandEvent*>(event)->GetArgs());
        break;

    default:
        alt::ICore::Instance().LogError(
            "get_event_console_command_name unknown event type: " +
            std::to_string(static_cast<u16>(type))
        );
        return std::make_unique<StdStringVector>();
        break;
    }
}

StdStringPtr get_event_server_script_event_name(const alt::CEvent* event) {
    assert(event->GetType() == alt::CEvent::Type::SERVER_SCRIPT_EVENT);

    return std::make_unique<std::string>(std::string {
        static_cast<const alt::CServerScriptEvent*>(event)->GetName()
    });
}

MValueWrapperVec get_event_server_script_event_args(const alt::CEvent* event) {
    assert(event->GetType() == alt::CEvent::Type::SERVER_SCRIPT_EVENT);

    auto args = static_cast<const alt::CServerScriptEvent*>(event)->GetArgs();
    auto mvalue_vec = create_mvalue_vec();

    for (int i = 0; i < args.GetSize(); ++i)
    {
        MValueWrapper wrapper;
        wrapper.ptr = std::make_shared<alt::MValueConst>(args[i]);
        mvalue_vec.push_back(wrapper.clone());
    }

    return mvalue_vec;
}

void log_colored(std::string str) {
    return alt::ICore::Instance().LogColored(str);
}

void log_error(std::string str) {
    return alt::ICore::Instance().LogError(str);
}

void log_warn(std::string str) {
    return alt::ICore::Instance().LogWarning(str);
}

// entity conversions
alt::IEntity* convert_base_object_to_entity(alt::IBaseObject* base_object) {
    return dynamic_cast<alt::IEntity*>(base_object);
}

// vehicle conversions
alt::IBaseObject* convert_vehicle_to_base_object(alt::IVehicle* vehicle) {
    return static_cast<alt::IBaseObject*>(vehicle);
}

alt::IVehicle* convert_base_object_to_vehicle(alt::IBaseObject* base_object) {
    return dynamic_cast<alt::IVehicle*>(base_object);
}

alt::IEntity* convert_vehicle_to_entity(alt::IVehicle* vehicle) {
    return static_cast<alt::IEntity*>(vehicle);
}

// player conversions
alt::IBaseObject* convert_player_to_base_object(alt::IPlayer* player) {
    return static_cast<alt::IBaseObject*>(player);
}

alt::IPlayer* convert_base_object_to_player(alt::IBaseObject* base_object) {
    return dynamic_cast<alt::IPlayer*>(base_object);
}

alt::IEntity* convert_player_to_entity(alt::IPlayer* player) {
    return static_cast<alt::IEntity*>(player);
}

// vehicle
alt::IVehicle* create_vehicle(
    u32 model,
    f32 x, f32 y, f32 z,
    f32 rx, f32 ry, f32 rz
) {
    return alt::ICore::Instance().CreateVehicle(model, { x, y, z }, { rx, ry, rz });
}

void set_vehicle_primary_color(alt::IVehicle* vehicle, u8 color) {
    vehicle->SetPrimaryColor(color);
}

u8 get_vehicle_primary_color(const alt::IVehicle* vehicle) {
    return vehicle->GetPrimaryColor();
}

u16 get_entity_id(alt::IEntity* entity) {
    if (!entity)
    {
        alt::ICore::Instance().LogError("get_entity_id nullptr entity");
        return 0;
    }

    // alt::ICore::Instance().LogInfo("get_entity_id entity type: " + std::to_string(static_cast<u8>(entity->GetType())));
    return entity->GetID();
}

void destroy_base_object(alt::IBaseObject* base_object) {
    if (!base_object)
    {
        alt::ICore::Instance().LogError("destroy_base_object nullptr base_object");
        return;
    }

    auto type = base_object->GetType();

    alt::ICore::Instance().LogInfo("destroy_base_object type: " + std::to_string(static_cast<u8>(base_object->GetType())));
    alt::ICore::Instance().DestroyBaseObject(base_object);
}

u8 get_base_object_type(const alt::IBaseObject* base_object) {
    if (!base_object)
    {
        alt::ICore::Instance().LogError("get_base_object_type nullptr base_object");
        return 255;
    }

    u8 type = static_cast<u8>(base_object->GetType());

    alt::ICore::Instance().LogInfo("get_base_object_type type: " + std::to_string(type));
    return type;
}

// player
StdStringPtr get_player_name(const alt::IPlayer* player) {
    return std::make_unique<std::string>(player->GetName());
}
