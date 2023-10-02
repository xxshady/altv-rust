#pragma once

#include <cstdint>
#include "../cpp-sdk/SDK.h"
#include "../cpp-sdk/events/CMetaDataChangeEvent.h"
#include <string>
#include "cxx.h"

namespace shared {
    using ResourceStartCallback = void (*)(rust::Str name, rust::Str full_main_path);
    using ResourceStopCallback = void (*)(rust::Str name);
    using RuntimeResourceDestroyImplCallback = void (*)();
    using RuntimeOnTickCallback = void (*)();
    using ResourceOnEventCallback = void (*)(rust::Str name, const alt::CEvent* event);
    using ResourceOnCreateBaseObjectCallback = void (*)(rust::Str name, alt::IBaseObject* base_object);
    using ResourceOnRemoveBaseObjectCallback = void (*)(rust::Str name, alt::IBaseObject* base_object);
}
