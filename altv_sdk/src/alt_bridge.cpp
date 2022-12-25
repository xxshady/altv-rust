#define ALT_SERVER_API
#include <string>
#include <iostream>
#include "altv_sdk/include/SDK_extend.h"
#include "altv_sdk/include/cpp-sdk/SDK.h"

alt_rs::RustRuntime::RustRuntime()
{
    alt::ICore::Instance().LogInfo("RustRuntime ctor call");
}

alt::IResource::Impl *alt_rs::RustRuntime::CreateImpl(alt::IResource *resource)
{
    alt::ICore::Instance().LogInfo("RustRuntime::CreateImpl");
    alt::ICore::Instance().LogInfo("calling rust func");
    auto resource_impl = new alt_rs::RustRuntime::RustResource(this, resource);

    if (create_impl_callback == nullptr)
        alt::ICore::Instance().LogError("rust func is null");
    else
    {
        create_impl_callback(resource_impl);
    }

    return static_cast<alt::IResource::Impl *>(resource_impl);
}

void alt_rs::RustRuntime::DestroyImpl(alt::IResource::Impl *impl)
{
    alt::ICore::Instance().LogInfo("RustRuntime::DestroyImpl");

    if (destroy_impl_callback == nullptr)
        alt::ICore::Instance().LogError("rust func is null");
    else
    {
        destroy_impl_callback();
    }

    delete impl;
}

void alt_rs::RustRuntime::OnTick()
{
}

bool alt_rs::RustRuntime::RustResource::Start()
{
    alt::ICore::Instance().LogInfo("RustResource::Start");
    runtime->resource_start_callback(GetIResource()->GetPath(), GetIResource()->GetMain());
    return true;
}

bool alt_rs::RustRuntime::RustResource::Stop()
{
    alt::ICore::Instance().LogInfo("RustResource::Stop");
    return true;
}

void alt_rs::RustRuntime::RustResource::OnEvent(const alt::CEvent *ev)
{
}

void alt_rs::RustRuntime::RustResource::OnTick()
{
    runtime->resource_on_tick_callback();
}

void alt_rs::RustRuntime::RustResource::OnCreateBaseObject(alt::IBaseObject *object)
{
    // Called every time a base object has been created, so you can use this to keep track
    // of all the existing base objects, to check if they are valid in the user scripts
}

void alt_rs::RustRuntime::RustResource::OnRemoveBaseObject(alt::IBaseObject *object)
{
    // Called every time a base object has been created, so you can use this to keep track
    // of all the existing base objects, to check if they are valid in the user scripts
}

namespace alt_rs
{
    void set_alt_core(alt_rs::ICore *core)
    {
        return alt::ICore::SetInstance(core);
    }

    alt_rs::ICore *alt_core_instance()
    {
        return &alt::ICore::Instance();
    };

    void log_colored(const std::string &str)
    {
        return alt::ICore::Instance().LogColored(str);
    }

    alt::IScriptRuntime *create_script_runtime()
    {
        return new alt_rs::RustRuntime();
    }

    void register_script_runtime(
        alt::ICore *core,
        const std::string &resource_type,
        alt::IScriptRuntime *runtime,
        alt_rs::RuntimeCreateImplCallback create_impl_callback,
        alt_rs::RuntimeDestroyImplCallback destroy_impl_callback,
        alt_rs::ResourceStartCallback resource_start_callback,
        ResourceOnTickCallback resource_on_tick_callback)
    {
        bool register_result = core->RegisterScriptRuntime(resource_type, runtime);
        std::cout << "RegisterScriptRuntime: " << register_result << std::endl;
        reinterpret_cast<alt_rs::RustRuntime *>(runtime)->set_callbacks(
            create_impl_callback,
            destroy_impl_callback,
            resource_start_callback,
            resource_on_tick_callback);
    }

    IVehicle *create_vehicle(
        uint32_t model,
        float x, float y, float z, float rx, float ry, float rz)
    {
        return ICore::Instance().CreateVehicle(model, {x, y, z}, {rx, ry, rz});
    }

    // TODO: add entity and move it to it
    uint16_t vehicle_get_id(IVehicle *vehicle)
    {
        return vehicle->GetID();
    }

    void vehicle_destroy(IVehicle *vehicle)
    {
        ICore::Instance().DestroyBaseObject(vehicle);
    }
}
