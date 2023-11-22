#pragma once
#include "alt_bridge.h"

namespace natives {
    using Success = bool;

    std::shared_ptr<alt::INative::Context> ctx;

    void init() {
        ctx = alt::ICore::Instance().CreateNativesContext();
    }

    static char* save_c_string(const char* str) {
        if (str == nullptr) return nullptr;
        static char* stringValues[256] = { 0 };
        static int nextString = 0;
        if (stringValues[nextString]) free(stringValues[nextString]);
        char* _str = _strdup(str);
        stringValues[nextString] = _str;
        nextString = (nextString + 1) % 256;
        return _str;
    }

    char* clone_c_string(const char* str) {
        size_t stringSize = strlen(str);
        char* writable = new char[stringSize + 1];
        std::memcpy(writable, str, stringSize);
        writable[stringSize] = '\0';
        return writable;
    }


    class CStringPtr {
    public:
        char* ptr = nullptr;

        CStringPtr(std::string content) : ptr(save_c_string(content.c_str())) {}
        CStringPtr() {}
    };

    CStringPtr create_c_string_ptr(std::string content) {
        return { content };
    }

    CStringPtr create_null_c_string_ptr() {
        return {};
    }

    std::string read_c_string_ptr(const CStringPtr& str_ref) {
        return { str_ref.ptr };
    }

    bool is_c_string_ptr_null(const CStringPtr& str_ref) {
        return str_ref.ptr == nullptr;
    }
Success get_dlc_weapon_data(bool* native_return, void* outData_, i32 dlcWeaponIndex_) {
    static auto native = alt::ICore::Instance().GetNativeByHash(0x79923CD21BECE14E);
    ctx->Reset();

    ctx->Push(dlcWeaponIndex_);
    ctx->Push(outData_);

    Success result = native->Invoke(ctx);
    if (result) {
        
        *native_return = ctx->ResultBool();
    }
    return result;
}
}