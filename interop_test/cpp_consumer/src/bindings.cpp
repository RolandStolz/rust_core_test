// nanobind binding layer. Its ONLY job is to turn an incoming `cr_core` Python
// object into a pointer to the underlying struct (via the producer's named
// PyCapsule) and hand that to the core logic in `cr_logic`. No data logic here.

#include <cstddef>

#include <nanobind/nanobind.h>

#include "logic.h"

namespace nb = nanobind;

// Pull a typed pointer out of `obj._cr_capsule()`, validating the capsule name
// (the name is the cross-language type tag).
template <typename T>
static T *cr_borrow(nb::handle obj, const char *name) {
    nb::object cap = obj.attr("_cr_capsule")();
    void *ptr = PyCapsule_GetPointer(cap.ptr(), name);
    if (ptr == nullptr) {
        // PyCapsule_GetPointer set a ValueError (e.g. wrong name == wrong type).
        throw nb::python_error();
    }
    return reinterpret_cast<T *>(ptr);
}

NB_MODULE(cpp_consumer, m) {
    m.doc() = "nanobind layer: extracts the cr_core pointer, delegates to cr_logic.";

    // The args are `nb::handle` (untyped to C++), so we override the generated stub
    // signature with `nb::sig(...)` to type them as the actual `cr_core.*` classes.
    // Each wrapper extracts the pointer, calls the logic, and returns the address it
    // dereferenced so Python can assert it matches the producer object (zero-copy).
    m.def(
        "print_point",
        [](nb::handle obj) -> std::size_t {
            cr_core::Point *p = cr_borrow<cr_core::Point>(obj, "cr_core.Point");
            cr_logic::print_point(*p);
            return reinterpret_cast<std::size_t>(p);
        },
        nb::sig("def print_point(point: cr_core.Point) -> int"));

    m.def(
        "print_state",
        [](nb::handle obj) -> std::size_t {
            cr_core::State *s = cr_borrow<cr_core::State>(obj, "cr_core.State");
            cr_logic::print_state(*s);
            return reinterpret_cast<std::size_t>(s);
        },
        nb::sig("def print_state(state: cr_core.State) -> int"));

    m.def(
        "print_lanelet",
        [](nb::handle obj) -> std::size_t {
            cr_core::Lanelet *l = cr_borrow<cr_core::Lanelet>(obj, "cr_core.Lanelet");
            cr_logic::print_lanelet(*l);
            return reinterpret_cast<std::size_t>(l);
        },
        nb::sig("def print_lanelet(lanelet: cr_core.Lanelet) -> int"));

    m.def(
        "bump_point_x",
        [](nb::handle obj, double dx) {
            cr_core::Point *p = cr_borrow<cr_core::Point>(obj, "cr_core.Point");
            cr_logic::bump_point_x(*p, dx);
        },
        nb::sig("def bump_point_x(point: cr_core.Point, dx: float) -> None"));
}
