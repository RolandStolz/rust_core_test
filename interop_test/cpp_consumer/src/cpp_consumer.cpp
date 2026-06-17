// C++ (nanobind) *consumer* — the mirror of `rust_consumer`, but in C++.
//
// It uses the **cxx-generated** `cr_core::Point/State/Lanelet` structs (from the
// Rust core's `#[cxx::bridge]` in ../../src/bindings/cpp.rs) and borrows a producer
// object zero-copy: it calls `_cr_capsule()` on the Python object (a cr_core object
// produced by the Rust/PyO3 core), validates the capsule name, and reinterprets the
// raw pointer as the cxx struct. This works because the Rust `#[repr(C)]` core struct
// and the cxx shared struct have identical layout.

#include <cstddef>
#include <cstdint>
#include <cstdio>

#include <nanobind/nanobind.h>

// Pulls the cxx-generated cpp.rs.h (namespace cr_core { struct Point/State/Lanelet; }).
// We include demo.h rather than cpp.rs.h directly because the generated header
// #includes demo.h at the top; including demo.h first gets the definition order right.
#include "cr_core/demo.h"

namespace nb = nanobind;

// Pull the typed pointer out of `obj._cr_capsule()`, validating the capsule name
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

static std::size_t print_point(nb::handle obj) {
    const cr_core::Point *p = cr_borrow<cr_core::Point>(obj, "cr_core.Point");
    std::printf("[cpp_consumer] point = (%g, %g) @ %p\n", p->x, p->y,
                static_cast<const void *>(p));
    std::fflush(stdout);  // keep ordering vs Python's stdout when piped
    return reinterpret_cast<std::size_t>(p);
}

static std::size_t print_state(nb::handle obj) {
    const cr_core::State *s = cr_borrow<cr_core::State>(obj, "cr_core.State");
    std::printf("[cpp_consumer] state = pos=(%g, %g) ori=%g vel=%g time=%zu @ %p\n",
                s->position.x, s->position.y, s->orientation, s->velocity,
                static_cast<std::size_t>(s->time), static_cast<const void *>(s));
    std::fflush(stdout);
    return reinterpret_cast<std::size_t>(s);
}

static std::size_t print_lanelet(nb::handle obj) {
    const cr_core::Lanelet *l = cr_borrow<cr_core::Lanelet>(obj, "cr_core.Lanelet");
    // Reads the POD `id` field only. Iterating the `rust::Vec` bounds would require
    // linking the cxx runtime; reading `id` already proves the layout lines up.
    std::printf("[cpp_consumer] lanelet id = %zu @ %p\n",
                static_cast<std::size_t>(l->id), static_cast<const void *>(l));
    std::fflush(stdout);
    return reinterpret_cast<std::size_t>(l);
}

// Mutates the producer's Point through the shared pointer — proves it's the same
// memory, observable back in Python.
static void bump_point_x(nb::handle obj, double dx) {
    cr_core::Point *p = cr_borrow<cr_core::Point>(obj, "cr_core.Point");
    p->x += dx;
}

NB_MODULE(cpp_consumer, m) {
    m.doc() = "C++/nanobind consumer that borrows cr_core objects zero-copy via cxx structs.";
    m.def("print_point", &print_point);
    m.def("print_state", &print_state);
    m.def("print_lanelet", &print_lanelet);
    m.def("bump_point_x", &bump_point_x);
}
