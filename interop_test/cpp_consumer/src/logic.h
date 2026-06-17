#pragma once

// cxx-generated cr_core::Point/State/Lanelet. (demo.h pulls cpp.rs.h in the right
// order — see note in logic.cpp / bindings.cpp.)
#include "cr_core/demo.h"

// Core logic layer: plain functions over cr_core structs. Knows nothing about
// Python, nanobind, or capsules — it just operates on the data.
namespace cr_logic {

void print_point(const cr_core::Point &p);
void print_state(const cr_core::State &s);
void print_lanelet(const cr_core::Lanelet &l);
void bump_point_x(cr_core::Point &p, double dx);

} // namespace cr_logic
