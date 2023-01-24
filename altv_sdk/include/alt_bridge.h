#pragma once

#include "altv_sdk/include/cpp-sdk/SDK.h"
#include "rust/cxx.h"

namespace alt_rs
{
    // cpp-sdk
    // TODO: is it really needed?
    using ICore = alt::ICore;
    using IScriptRuntime = alt::IScriptRuntime;
    using IBaseObject = alt::IBaseObject;
    using IEntity = alt::IEntity;
    using IVehicle = alt::IVehicle;
    using IPlayer = alt::IPlayer;
    using IBlip = alt::IBlip;
    using CEvent = alt::CEvent;

    // custom
    using RustResourceImpl = alt::IResource::Impl*;
    using RuntimeCreateImplCallback = void (*)(RustResourceImpl rust_resource_impl);
    using RuntimeDestroyImplCallback = void (*)();
    using RuntimeOnTickCallback = void (*)();
    using ResourceStartCallback = void (*)(rust::Str full_main_path);
    using ResourceStopCallback = void (*)(rust::Str full_main_path);
    using ResourceOnTickCallback = void (*)();
    using ResourceOnEventCallback = void (*)(rust::Str full_main_path, const alt::CEvent* event);

    using StdString = std::unique_ptr<std::string>;

    class RustRuntime: public alt::IScriptRuntime
    {
    private:
        static RustRuntime*& _instance()
        {
            static RustRuntime* instance = nullptr;
            return instance;
        }

    public:
        RuntimeCreateImplCallback create_impl_callback;
        RuntimeDestroyImplCallback destroy_impl_callback;
        RuntimeOnTickCallback on_tick_callback;
        ResourceStartCallback resource_start_callback;
        ResourceStopCallback resource_stop_callback;
        ResourceOnTickCallback resource_on_tick_callback;
        ResourceOnEventCallback resource_on_event_callback;

        static RustRuntime& get_instance()
        {
            return *_instance();
        }
        static void set_instance(RustRuntime* rust_runtime) { _instance() = rust_runtime; }

        class RustResource: public alt::IResource::Impl
        {
            RustRuntime* runtime;
            alt::IResource* resource;
            std::string full_main_path;

        public:
            RustResource(
                RustRuntime* runtime,
                alt::IResource* resource,
                const std::string&& full_main_path
            ):
                runtime(runtime),
                resource(resource),
                full_main_path(full_main_path)
            {};

            ~RustResource() = default;

            bool Start() override;
            bool Stop() override;

            void OnEvent(const alt::CEvent* event) override;
            void OnTick() override;

            void OnCreateBaseObject(alt::IBaseObject* object) override;
            void OnRemoveBaseObject(alt::IBaseObject* object) override;

            alt::IResource* GetIResource()
            {
                return resource;
            }

            RustRuntime* GetRuntime()
            {
                return runtime;
            }
        };

        RustRuntime();

        alt::IResource::Impl* CreateImpl(alt::IResource* resource) override;
        void DestroyImpl(alt::IResource::Impl* impl) override;
        void OnTick() override;

        void set_callbacks(
            RuntimeCreateImplCallback _create_impl_callback,
            RuntimeDestroyImplCallback _destroy_impl_callback,
            RuntimeOnTickCallback _on_tick_callback,
            ResourceStartCallback _resource_start_callback,
            ResourceStopCallback _resource_stop_callback,
            ResourceOnTickCallback _resource_on_tick_callback,
            ResourceOnEventCallback _resource_on_event_callback
        )
        {
            create_impl_callback = _create_impl_callback;
            destroy_impl_callback = _destroy_impl_callback;
            on_tick_callback = _on_tick_callback;
            resource_start_callback = _resource_start_callback;
            resource_stop_callback = _resource_stop_callback;
            resource_on_tick_callback = _resource_on_tick_callback;
            resource_on_event_callback = _resource_on_event_callback;
        }
    };

    void set_alt_core(alt_rs::ICore* core);
    alt_rs::ICore* alt_core_instance();
    void log_colored(const std::string& str);
    void log_error(const std::string& str);
    void log_warn(const std::string& str);

    alt::IScriptRuntime* create_script_runtime();

    void register_script_runtime(
        alt::ICore* core,
        const std::string& resource_type,
        alt::IScriptRuntime* runtime
    );

    void setup_callbacks(
        RuntimeCreateImplCallback create_impl_callback,
        RuntimeDestroyImplCallback destroy_impl_callback,
        RuntimeOnTickCallback on_tick_callback,
        ResourceStartCallback resource_start_callback,
        ResourceStopCallback resource_stop_callback,
        ResourceOnTickCallback resource_on_tick_callback,
        ResourceOnEventCallback _resource_on_event_callback
    );

    // events
    void toggle_event_type(uint16_t event_type, bool state);
    // CEvent methods
    uint16_t get_event_type(const alt::CEvent* event);
    IPlayer* get_event_player_target(const alt::CEvent* event);
    StdString get_event_reason(const alt::CEvent* event);

    // baseobject conversions
    IBaseObject* convert_vehicle_to_baseobject(IVehicle* baseobject);
    IBaseObject* convert_player_to_baseobject(IPlayer* baseobject);

    IEntity* convert_vehicle_to_entity(IVehicle* entity);
    IEntity* convert_player_to_entity(IPlayer* entity);

    // baseobject
    void destroy_baseobject(IBaseObject* baseobject);

    // entity
    uint16_t get_entity_id(IEntity* entity);

    // vehicle
    IVehicle* create_vehicle(
        uint32_t model,
        float x, float y, float z,
        float rx, float ry, float rz
    );

    // player
    StdString get_player_name(IPlayer* player);
}
