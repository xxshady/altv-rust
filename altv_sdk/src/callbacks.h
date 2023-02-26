#pragma once

#include "shared.h"
#include "runtime.h"

namespace callbacks {
    using ResourceStartCallback = shared::ResourceStartCallback;
    using ResourceStopCallback = shared::ResourceStopCallback;
    using RuntimeResourceDestroyImplCallback = shared::RuntimeResourceDestroyImplCallback;
    using RuntimeOnTickCallback = shared::RuntimeOnTickCallback;
    using ResourceOnTickCallback = shared::ResourceOnTickCallback;
    using ResourceOnEventCallback = shared::ResourceOnEventCallback;
    using ResourceOnCreateBaseObjectCallback = shared::ResourceOnCreateBaseObjectCallback;
    using ResourceOnRemoveBaseObjectCallback = shared::ResourceOnRemoveBaseObjectCallback;

    void setup_callbacks(
        ResourceStartCallback resource_start,
        ResourceStopCallback resource_stop,
        RuntimeResourceDestroyImplCallback resource_impl_destroy,
        RuntimeOnTickCallback on_tick,
        ResourceOnTickCallback resource_on_tick,
        ResourceOnEventCallback resource_on_event,
        ResourceOnCreateBaseObjectCallback resource_on_create_base_object,
        ResourceOnRemoveBaseObjectCallback resource_on_remove_base_object
    ) {
        RustRuntime::get_instance().set_callbacks(
            resource_start,
            resource_stop,
            resource_impl_destroy,
            on_tick,
            resource_on_tick,
            resource_on_event,
            resource_on_create_base_object,
            resource_on_remove_base_object
        );
    }
}
