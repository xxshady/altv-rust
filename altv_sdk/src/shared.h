#pragma once

#include "../cpp-sdk/SDK.h"
#include <string>
#include "cxx.h"

namespace shared {
    using ResourceStartCallback = void (*)(rust::String full_main_path);
    using ResourceStopCallback = void (*)(rust::String full_main_path);
    using RuntimeResourceDestroyImplCallback = void (*)();
    using RuntimeOnTickCallback = void (*)();
    using ResourceOnTickCallback = void (*)(rust::String full_main_path);
    using ResourceOnEventCallback = void (*)(rust::String full_main_path, const alt::CEvent* event);
    using ResourceOnCreateBaseObjectCallback = void (*)(rust::String full_main_path, alt::IBaseObject* base_object);
    using ResourceOnRemoveBaseObjectCallback = void (*)(rust::String full_main_path, alt::IBaseObject* base_object);
}
