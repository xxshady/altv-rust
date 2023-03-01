#pragma once

#include "shared.h"
#include <filesystem>

class RustRuntime: public alt::IScriptRuntime {
private:
    static RustRuntime*& _instance() {
        static RustRuntime* instance = nullptr;
        return instance;
    }

public:
    shared::ResourceStartCallback resource_start_callback = nullptr;
    shared::ResourceStopCallback resource_stop_callback = nullptr;
    shared::RuntimeResourceDestroyImplCallback resource_impl_destroy_callback = nullptr;
    shared::RuntimeOnTickCallback on_tick_callback = nullptr;
    shared::ResourceOnEventCallback resource_on_event_callback = nullptr;
    shared::ResourceOnCreateBaseObjectCallback resource_on_create_base_object_callback = nullptr;
    shared::ResourceOnRemoveBaseObjectCallback resource_on_remove_base_object_callback = nullptr;

    static RustRuntime& get_instance() {
        return *_instance();
    }
    static void set_instance(RustRuntime* rust_runtime) { _instance() = rust_runtime; }

    class RustResource: public alt::IResource::Impl {
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

        bool Start() override {
            assert(runtime->resource_start_callback != nullptr);
            runtime->resource_start_callback(full_main_path);
            return true;
        }

        bool Stop() override {
            assert(runtime->resource_stop_callback != nullptr);
            runtime->resource_stop_callback(full_main_path);
            return true;
        }

        void OnEvent(const alt::CEvent* event) override {
            assert(runtime->resource_on_event_callback != nullptr);
            runtime->resource_on_event_callback(full_main_path, event);
        }

        void OnCreateBaseObject(alt::IBaseObject* base_object) override {
            assert(runtime->resource_on_create_base_object_callback != nullptr);
            runtime->resource_on_create_base_object_callback(full_main_path, base_object);
        }
        void OnRemoveBaseObject(alt::IBaseObject* base_object) override {
            assert(runtime->resource_on_remove_base_object_callback != nullptr);
            runtime->resource_on_remove_base_object_callback(full_main_path, base_object);
        }

        alt::IResource* GetIResource()
        {
            return resource;
        }

        RustRuntime* GetRuntime()
        {
            return runtime;
        }
    };

    RustRuntime() {}

    alt::IResource::Impl* CreateImpl(alt::IResource* resource) override {
        std::filesystem::path _full_main_path(std::filesystem::path(resource->GetPath()) / resource->GetMain());
        std::string full_main_path = _full_main_path.string();

        auto resource_impl = new RustRuntime::RustResource(
            this,
            resource,
            std::move(full_main_path)
        );

        return static_cast<alt::IResource::Impl*>(resource_impl);
    }

    void DestroyImpl(alt::IResource::Impl* impl) override {
        assert(resource_impl_destroy_callback != nullptr);
        resource_impl_destroy_callback();

        delete impl;
    }

    void OnTick() override {
        assert(on_tick_callback != nullptr);
        on_tick_callback();
    }

    void set_callbacks(
        shared::ResourceStartCallback _resource_start,
        shared::ResourceStopCallback _resource_stop,
        shared::RuntimeResourceDestroyImplCallback _resource_impl_destroy,
        shared::RuntimeOnTickCallback _on_tick,
        shared::ResourceOnEventCallback _resource_on_event,
        shared::ResourceOnCreateBaseObjectCallback _resource_on_create_base_object,
        shared::ResourceOnRemoveBaseObjectCallback _resource_on_remove_base_object
    )
    {
        resource_start_callback = _resource_start;
        resource_stop_callback = _resource_stop;
        resource_impl_destroy_callback = _resource_impl_destroy;
        on_tick_callback = _on_tick;
        resource_on_event_callback = _resource_on_event;
        resource_on_create_base_object_callback = _resource_on_create_base_object;
        resource_on_remove_base_object_callback = _resource_on_remove_base_object;
    }
};
