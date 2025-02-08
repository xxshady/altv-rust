#include "callbacks.h"
#include <type_traits>
#include <utility>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_RELOCATABLE
#define CXXBRIDGE1_RELOCATABLE
namespace detail {
template <typename... Ts>
struct make_void {
  using type = void;
};

template <typename... Ts>
using void_t = typename make_void<Ts...>::type;

template <typename Void, template <typename...> class, typename...>
struct detect : std::false_type {};
template <template <typename...> class T, typename... A>
struct detect<void_t<T<A...>>, T, A...> : std::true_type {};

template <template <typename...> class T, typename... A>
using is_detected = detect<void, T, A...>;

template <typename T>
using detect_IsRelocatable = typename T::IsRelocatable;

template <typename T>
struct get_IsRelocatable
    : std::is_same<typename T::IsRelocatable, std::true_type> {};
} // namespace detail

template <typename T>
struct IsRelocatable
    : std::conditional<
          detail::is_detected<detail::detect_IsRelocatable, T>::value,
          detail::get_IsRelocatable<T>,
          std::integral_constant<
              bool, std::is_trivially_move_constructible<T>::value &&
                        std::is_trivially_destructible<T>::value>>::type {};
#endif // CXXBRIDGE1_RELOCATABLE
} // namespace cxxbridge1
} // namespace rust

static_assert(
    ::rust::IsRelocatable<::callbacks::ResourceStartCallback>::value,
    "type callbacks::ResourceStartCallback should be trivially move constructible and trivially destructible in C++ to be used as an argument of `setup_callbacks` in Rust");
static_assert(
    ::rust::IsRelocatable<::callbacks::ResourceStopCallback>::value,
    "type callbacks::ResourceStopCallback should be trivially move constructible and trivially destructible in C++ to be used as an argument of `setup_callbacks` in Rust");
static_assert(
    ::rust::IsRelocatable<::callbacks::RuntimeResourceDestroyImplCallback>::value,
    "type callbacks::RuntimeResourceDestroyImplCallback should be trivially move constructible and trivially destructible in C++ to be used as an argument of `setup_callbacks` in Rust");
static_assert(
    ::rust::IsRelocatable<::callbacks::RuntimeOnTickCallback>::value,
    "type callbacks::RuntimeOnTickCallback should be trivially move constructible and trivially destructible in C++ to be used as an argument of `setup_callbacks` in Rust");
static_assert(
    ::rust::IsRelocatable<::callbacks::ResourceOnEventCallback>::value,
    "type callbacks::ResourceOnEventCallback should be trivially move constructible and trivially destructible in C++ to be used as an argument of `setup_callbacks` in Rust");
static_assert(
    ::rust::IsRelocatable<::callbacks::ResourceOnCreateBaseObjectCallback>::value,
    "type callbacks::ResourceOnCreateBaseObjectCallback should be trivially move constructible and trivially destructible in C++ to be used as an argument of `setup_callbacks` in Rust");
static_assert(
    ::rust::IsRelocatable<::callbacks::ResourceOnRemoveBaseObjectCallback>::value,
    "type callbacks::ResourceOnRemoveBaseObjectCallback should be trivially move constructible and trivially destructible in C++ to be used as an argument of `setup_callbacks` in Rust");

namespace callbacks {
extern "C" {
void callbacks$cxxbridge1$setup_callbacks(::callbacks::ResourceStartCallback *resource_start, ::callbacks::ResourceStopCallback *resource_stop, ::callbacks::RuntimeResourceDestroyImplCallback *resource_impl_destroy, ::callbacks::RuntimeOnTickCallback *on_tick, ::callbacks::ResourceOnEventCallback *resource_on_event, ::callbacks::ResourceOnCreateBaseObjectCallback *resource_on_create_base_object, ::callbacks::ResourceOnRemoveBaseObjectCallback *resource_on_remove_base_object) noexcept {
  void (*setup_callbacks$)(::callbacks::ResourceStartCallback, ::callbacks::ResourceStopCallback, ::callbacks::RuntimeResourceDestroyImplCallback, ::callbacks::RuntimeOnTickCallback, ::callbacks::ResourceOnEventCallback, ::callbacks::ResourceOnCreateBaseObjectCallback, ::callbacks::ResourceOnRemoveBaseObjectCallback) = ::callbacks::setup_callbacks;
  setup_callbacks$(::std::move(*resource_start), ::std::move(*resource_stop), ::std::move(*resource_impl_destroy), ::std::move(*on_tick), ::std::move(*resource_on_event), ::std::move(*resource_on_create_base_object), ::std::move(*resource_on_remove_base_object));
}
} // extern "C"
} // namespace callbacks
