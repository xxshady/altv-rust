#pragma once

#include "../cpp-sdk/SDK.h"
#include <string>
#include "cxx.h"

namespace shared {
    using ResourceStartCallback = void (*)(rust::Str full_main_path);
    using ResourceStopCallback = void (*)(rust::Str full_main_path);
    using RuntimeResourceDestroyImplCallback = void (*)();
    using RuntimeOnTickCallback = void (*)();
    using ResourceOnEventCallback = void (*)(rust::Str full_main_path, const alt::CEvent* event);
    using ResourceOnCreateBaseObjectCallback = void (*)(rust::Str full_main_path, alt::IBaseObject* base_object);
    using ResourceOnRemoveBaseObjectCallback = void (*)(rust::Str full_main_path, alt::IBaseObject* base_object);
}
