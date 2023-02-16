#include <string>
#include <iostream>
#include <memory>
#include <filesystem>
#include "altv_sdk/include/alt_bridge.h"
#include "altv_sdk/include/cpp-sdk/SDK.h"

alt_rs::RustRuntime::RustRuntime()
{
    alt::ICore::Instance().LogInfo("RustRuntime ctor call");
}

alt::IResource::Impl* alt_rs::RustRuntime::CreateImpl(alt::IResource* resource)
{
    alt::ICore::Instance().LogInfo("RustRuntime::CreateImpl");
    alt::ICore::Instance().LogInfo("calling rust func");

    std::filesystem::path _full_main_path(std::filesystem::path(resource->GetPath()) / resource->GetMain());
    std::string full_main_path = _full_main_path.string();

    std::cout << "full_main_path: " << full_main_path << std::endl;

    auto resource_impl = new alt_rs::RustRuntime::RustResource(
        this,
        resource,
        std::move(full_main_path)
    );

    if (create_impl_callback == nullptr)
        alt::ICore::Instance().LogError("rust func is null");
    else
    {
        create_impl_callback(resource_impl);
    }

    return static_cast<alt::IResource::Impl*>(resource_impl);
}

void alt_rs::RustRuntime::DestroyImpl(alt::IResource::Impl* impl)
{
    alt::ICore::Instance().LogInfo("RustRuntime::DestroyImpl");

    destroy_impl_callback();

    delete impl;
}

void alt_rs::RustRuntime::OnTick()
{
    on_tick_callback();
}

bool alt_rs::RustRuntime::RustResource::Start()
{
    alt::ICore::Instance().LogInfo("RustResource::Start");
    runtime->resource_start_callback(full_main_path);
    return true;
}

bool alt_rs::RustRuntime::RustResource::Stop()
{
    alt::ICore::Instance().LogInfo("RustResource::Stop");
    runtime->resource_stop_callback(full_main_path);
    return true;
}

void alt_rs::RustRuntime::RustResource::OnEvent(const alt::CEvent* event)
{
    runtime->resource_on_event_callback(full_main_path, event);
}

void alt_rs::RustRuntime::RustResource::OnTick()
{
    runtime->resource_on_tick_callback(full_main_path);
}

void alt_rs::RustRuntime::RustResource::OnCreateBaseObject(alt::IBaseObject* base_object)
{
    runtime->resource_on_create_base_object_callback(full_main_path, base_object);
}

void alt_rs::RustRuntime::RustResource::OnRemoveBaseObject(alt::IBaseObject* base_object)
{
    runtime->resource_on_remove_base_object_callback(full_main_path, base_object);
}

namespace alt_rs
{
    void set_alt_core(alt_rs::ICore* core)
    {
        return alt::ICore::SetInstance(core);
    }

    alt_rs::ICore* alt_core_instance()
    {
        return &alt::ICore::Instance();
    };

    void log_colored(const std::string& str)
    {
        return alt::ICore::Instance().LogColored(str);
    }

    void log_error(const std::string& str)
    {
        return alt::ICore::Instance().LogError(str);
    }

    void log_warn(const std::string& str)
    {
        return alt::ICore::Instance().LogWarning(str);
    }

    alt::IScriptRuntime* create_script_runtime()
    {
        return new alt_rs::RustRuntime();
    }

    void register_script_runtime(
        alt::ICore* core,
        const std::string& resource_type,
        alt::IScriptRuntime* runtime)
    {
        bool register_result = core->RegisterScriptRuntime(resource_type, runtime);
        std::cout << "RegisterScriptRuntime: " << register_result << std::endl;

        RustRuntime::set_instance(static_cast<RustRuntime*>(runtime));
    }

    void setup_callbacks(
        RuntimeCreateImplCallback create_impl_callback,
        RuntimeDestroyImplCallback destroy_impl_callback,
        RuntimeOnTickCallback on_tick_callback,
        ResourceStartCallback resource_start_callback,
        ResourceStopCallback resource_stop_callback,
        ResourceOnTickCallback resource_on_tick_callback,
        ResourceOnEventCallback resource_on_event_callback,
        ResourceOnCreateBaseObjectCallback resource_on_create_base_object_callback,
        ResourceOnRemoveBaseObjectCallback resource_on_remove_base_object_callback
    ) {
        RustRuntime::get_instance().set_callbacks(
            create_impl_callback,
            destroy_impl_callback,
            on_tick_callback,
            resource_start_callback,
            resource_stop_callback,
            resource_on_tick_callback,
            resource_on_event_callback,
            resource_on_create_base_object_callback,
            resource_on_remove_base_object_callback
        );
    }

    // events
    void toggle_event_type(uint16_t event_type, bool state)
    {
        ICore::Instance().ToggleEvent(static_cast<CEvent::Type>(event_type), state);
    }

    // CEvent methods
    uint16_t get_event_type(const alt::CEvent* event)
    {
        return static_cast<uint16_t>(event->GetType());
    }

    IPlayer* get_event_player_target(const alt::CEvent* event)
    {
        auto type = event->GetType();

        if (type == CEvent::Type::PLAYER_CONNECT)
        {
            return static_cast<const alt::CPlayerConnectEvent*>(event)->GetTarget();
        }

        ICore::Instance().LogError(
            "get_event_player_target unknown event type: " +
            std::to_string(static_cast<uint16_t>(type))
        );
        return nullptr;
    }

    StdString get_event_reason(const alt::CEvent* event)
    {
        auto type = event->GetType();

        switch (type)
        {
        case CEvent::Type::PLAYER_DISCONNECT:
            return std::make_unique<std::string>(std::string{
                static_cast<const alt::CPlayerDisconnectEvent*>(event)->GetReason()
            });
            break;

        default:
            ICore::Instance().LogError(
                "get_event_reason unknown event type: " +
                std::to_string(static_cast<uint16_t>(type))
            );
            return std::make_unique<std::string>(std::string{ "" });
            break;
        }
    }

    // entity conversions
    IEntity* convert_base_object_to_entity(IBaseObject* entity)
    {
        return dynamic_cast<IEntity*>(entity);
    }

    // vehicle conversions
    IBaseObject* convert_vehicle_to_base_object(IVehicle* base_object)
    {
        return static_cast<IBaseObject*>(base_object);
    }

    IVehicle* convert_base_object_to_vehicle(IBaseObject* vehicle)
    {
        return dynamic_cast<IVehicle*>(vehicle);
    }

    IEntity* convert_vehicle_to_entity(IVehicle* entity)
    {
        return static_cast<IEntity*>(entity);
    }

    // player conversions
    IBaseObject* convert_player_to_base_object(IPlayer* base_object)
    {
        return static_cast<IBaseObject*>(base_object);
    }

    IPlayer* convert_base_object_to_player(IBaseObject* player)
    {
        return dynamic_cast<IPlayer*>(player);
    }

    IEntity* convert_player_to_entity(IPlayer* entity)
    {
        return static_cast<IEntity*>(entity);
    }

    // vehicle
    IVehicle* create_vehicle(
        uint32_t model,
        float x, float y, float z, float rx, float ry, float rz)
    {
        return ICore::Instance().CreateVehicle(model, { x, y, z }, { rx, ry, rz });
    }

    void set_vehicle_primary_color(IVehicle* vehicle, uint8_t color)
    {
        vehicle->SetPrimaryColor(color);
    }
    uint8_t get_vehicle_primary_color(const IVehicle* vehicle)
    {
        return vehicle->GetPrimaryColor();
    }

    uint16_t get_entity_id(IEntity* entity)
    {
        if (!entity)
        {
            ICore::Instance().LogError("get_entity_id nullptr entity");
            return 0;
        }

        // ICore::Instance().LogInfo("get_entity_id entity type: " + std::to_string(static_cast<uint8_t>(entity->GetType())));
        return entity->GetID();
    }

    void destroy_base_object(IBaseObject* base_object)
    {
        if (!base_object)
        {
            ICore::Instance().LogError("destroy_base_object nullptr base_object");
            return;
        }

        alt::IBaseObject::Type type = base_object->GetType();

        ICore::Instance().LogInfo("destroy_base_object type: " + std::to_string(static_cast<uint8_t>(base_object->GetType())));
        ICore::Instance().DestroyBaseObject(base_object);
    }

    uint8_t get_base_object_type(const IBaseObject* base_object)
    {
        if (!base_object)
        {
            ICore::Instance().LogError("get_base_object_type nullptr base_object");
            return 255;
        }

        uint8_t type = static_cast<uint8_t>(base_object->GetType());

        ICore::Instance().LogInfo("get_base_object_type type: " + std::to_string(type));
        return type;
    }

    // player
    StdString get_player_name(IPlayer* player)
    {
        return std::make_unique<std::string>(player->GetName());
    }

}
