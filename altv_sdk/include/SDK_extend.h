#pragma once

#include "altv_sdk/include/cpp-sdk/SDK.h"
#include "rust/cxx.h"

namespace alt_rs
{
    // cpp-sdk
    // TODO: is it really needed?
    using ICore = alt::ICore;
    using IScriptRuntime = alt::IScriptRuntime;
    using IVehicle = alt::IVehicle;

    // custom
    using RustResourceImpl = alt::IResource::Impl *;
    using RuntimeCreateImplCallback = void (*)(RustResourceImpl rust_resource_impl);
    using RuntimeDestroyImplCallback = void (*)();
    using ResourceStartCallback = void (*)(rust::Str resource_path, rust::Str resource_main);
    using ResourceOnTickCallback = void (*)();

    void set_alt_core(alt_rs::ICore *core);
    alt_rs::ICore *alt_core_instance();
    void log_colored(const std::string &str);

    class RustRuntime : public alt::IScriptRuntime
    {
        RuntimeCreateImplCallback create_impl_callback;
        RuntimeDestroyImplCallback destroy_impl_callback;
        ResourceStartCallback resource_start_callback;
        ResourceOnTickCallback resource_on_tick_callback;

    public:
        class RustResource : public alt::IResource::Impl
        {
            RustRuntime *runtime;
            alt::IResource *resource;

        public:
            RustResource(RustRuntime *runtime, alt::IResource *resource) : runtime(runtime), resource(resource){};
            ~RustResource() = default;

            bool Start() override;
            bool Stop() override;

            void OnEvent(const alt::CEvent *event) override;
            void OnTick() override;

            void OnCreateBaseObject(alt::IBaseObject *object) override;
            void OnRemoveBaseObject(alt::IBaseObject *object) override;

            // Gets the alt:V IResource instance
            alt::IResource *GetIResource()
            {
                return resource;
            }
            // Gets the module runtime that instantiated this resource
            RustRuntime *GetRuntime()
            {
                return runtime;
            }
        };

        RustRuntime();
        alt::IResource::Impl *CreateImpl(alt::IResource *resource) override;
        void DestroyImpl(alt::IResource::Impl *impl) override;
        void OnTick() override;

        void set_callbacks(
            RuntimeCreateImplCallback _create_impl_callback,
            RuntimeDestroyImplCallback _destroy_impl_callback,
            ResourceStartCallback _resource_start_callback,
            ResourceOnTickCallback _resource_on_tick_callback)
        {
            create_impl_callback = _create_impl_callback;
            destroy_impl_callback = _destroy_impl_callback;
            resource_start_callback = _resource_start_callback;
            resource_on_tick_callback = _resource_on_tick_callback;
        }
    };

    alt::IScriptRuntime *create_script_runtime();

    void register_script_runtime(
        alt::ICore *core,
        const std::string &resource_type,
        alt::IScriptRuntime *runtime,
        RuntimeCreateImplCallback create_impl_callback,
        RuntimeDestroyImplCallback destroy_impl_callback,
        ResourceStartCallback resource_start_callback,
        ResourceOnTickCallback resource_on_tick_callback);

    IVehicle *create_vehicle(
        uint32_t model,
        float x, float y, float z, float rx, float ry, float rz);
    uint16_t vehicle_get_id(IVehicle *vehicle);
    void vehicle_destroy(IVehicle *vehicle);
}
