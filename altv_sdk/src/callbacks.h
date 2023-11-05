#pragma once

#include "shared.h"
#include "runtime.h"

namespace callbacks {
    using ResourceStartCallback = shared::ResourceStartCallback;
    using ResourceStopCallback = shared::ResourceStopCallback;
    using RuntimeResourceDestroyImplCallback = shared::RuntimeResourceDestroyImplCallback;
    using RuntimeResourceImplCreateCallback = shared::RuntimeResourceImplCreateCallback;
    using ResourceOnTickCallback = shared::ResourceOnTickCallback;
    using ResourceOnEventCallback = shared::ResourceOnEventCallback;
    using ResourceOnCreateBaseObjectCallback = shared::ResourceOnCreateBaseObjectCallback;
    using ResourceOnRemoveBaseObjectCallback = shared::ResourceOnRemoveBaseObjectCallback;

    void setup_callbacks(
        ResourceStartCallback resource_start,
        ResourceStopCallback resource_stop,
        RuntimeResourceDestroyImplCallback resource_impl_destroy,
        RuntimeResourceImplCreateCallback resource_impl_create,
        ResourceOnTickCallback resource_on_tick,
        ResourceOnEventCallback resource_on_event,
        ResourceOnCreateBaseObjectCallback resource_on_create_base_object,
        ResourceOnRemoveBaseObjectCallback resource_on_remove_base_object
    ) {
        RustRuntime::get_instance().set_callbacks(
            resource_start,
            resource_stop,
            resource_impl_destroy,
            resource_impl_create,
            resource_on_tick,
            resource_on_event,
            resource_on_create_base_object,
            resource_on_remove_base_object
        );
    }

    using AltCore = shared::AltCore;
    using ConsoleCommandCallback = shared::ConsoleCommandCallback;

    void setup_command(alt::ICore* core, std::string const& name, ConsoleCommandCallback callback) {
        auto callback_ = [=](const std::vector<std::string>& args) {
            // for some reason CxxVector<CxxString> did not work, it contained garbage on rust side
            std::stringstream ss;
            for (auto& arg : args) {
                ss << arg << " ";
            }
            callback(ss.str());
            };
        core->SubscribeCommand(name, callback_);
    }
}
