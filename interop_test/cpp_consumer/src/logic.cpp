#include "logic.h"

#include <cstddef>
#include <cstdio>

namespace cr_logic {

void print_point(const cr_core::Point &p) {
    std::printf("[cpp_consumer] point = (%g, %g)\n", p.x, p.y);
    std::fflush(stdout);
}

void print_state(const cr_core::State &s) {
    std::printf("[cpp_consumer] state = pos=(%g, %g) ori=%g vel=%g time=%zu\n",
                s.position.x, s.position.y, s.orientation, s.velocity,
                static_cast<std::size_t>(s.time));
    std::fflush(stdout);
}

void print_lanelet(const cr_core::Lanelet &l) {
    // Reads the POD `id` only. Iterating the rust::Vec bounds would require linking
    // the cxx runtime; reading `id` already proves the layout lines up.
    std::printf("[cpp_consumer] lanelet id = %zu\n", static_cast<std::size_t>(l.id));
    std::fflush(stdout);
}

void bump_point_x(cr_core::Point &p, double dx) {
    p.x += dx;
}

} // namespace cr_logic
