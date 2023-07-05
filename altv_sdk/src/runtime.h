#pragma once

#include "shared.h"
#include <filesystem>

class RustRuntime : public alt::IScriptRuntime {
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

    class RustResource : public alt::IResource::Impl {
        RustRuntime* runtime;
        alt::IResource* resource;
        std::string name;

    public:
        RustResource(
            RustRuntime* runtime,
            alt::IResource* resource,
            std::string name
        ) :
            runtime(runtime),
            resource(resource),
            name(name)
        {};

        ~RustResource() = default;

        bool Start() override {
            return true;
        }

        bool Stop() override {
            assert(runtime->resource_stop_callback != nullptr);
            runtime->resource_stop_callback(name);
            return true;
        }

        void OnEvent(const alt::CEvent* event) override {
            assert(runtime->resource_on_event_callback != nullptr);
            runtime->resource_on_event_callback(name, event);
        }

        void OnCreateBaseObject(alt::IBaseObject* base_object) override {
            assert(runtime->resource_on_create_base_object_callback != nullptr);
            runtime->resource_on_create_base_object_callback(name, base_object);
        }
        void OnRemoveBaseObject(alt::IBaseObject* base_object) override {
            assert(runtime->resource_on_remove_base_object_callback != nullptr);
            runtime->resource_on_remove_base_object_callback(name, base_object);
        }

        alt::IResource* GetIResource()
        {
            return resource;
        }

        RustRuntime* GetRuntime()
        {
            return runtime;
        }

        std::vector<u8> read_file(std::string path, bool* exist)
        {
            auto pkg = resource->GetPackage();

            // Check if file exists
            if (!pkg->FileExists(path)) {
                *exist = false;
                return {};
            }
            *exist = true;

            // Open file
            alt::IPackage::File* pkg_file = pkg->OpenFile(path);
            std::vector<u8> buf{};
            auto size = pkg->GetFileSize(pkg_file);
            buf.reserve(size);

            // Read file content
            pkg->ReadFile(pkg_file, buf.data(), size);
            pkg->CloseFile(pkg_file);

            return buf;
        }
    };

    RustRuntime() {}

    alt::IResource::Impl* CreateImpl(alt::IResource* resource) override {
        auto resource_name = resource->GetName();
        std::filesystem::path _full_main_path(std::filesystem::path(resource->GetPath()) / resource->GetMain());
        std::string full_main_path = _full_main_path.string();

        auto resource_impl = new RustRuntime::RustResource(
            this,
            resource,
            resource_name
        );

        assert(resource_start_callback != nullptr);
        auto alt_resource_impl = static_cast<alt::IResource::Impl*>(resource_impl);
        resource_start_callback(resource_name, full_main_path, alt_resource_impl);

        return alt_resource_impl;
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
