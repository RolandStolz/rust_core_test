// Standalone C++ program using the cr_core data structures.
// Build/run with: ./cpp/build_demo.sh
#include "cr_core/demo.h"
#include <iostream>

int main() {
  // Struct + Rust constructor returned by value into C++.
  cr_core::Lanelet lanelet = cr_core::create_dummy_lanelet();
  std::cout << "lanelet id = " << lanelet.id << "\n";
  std::cout << "left bound points = " << lanelet.left_bound.size() << "\n";

  for (const auto &p : lanelet.left_bound) {
    std::printf("x: %f y: %f\n", p.x, p.y);
  }

  // C++ function reading the shared struct's fields.
  cr_core::Point c = cr_core::lanelet_centroid(lanelet);
  std::cout << "centroid = (" << c.x << ", " << c.y << ")\n";

  // Construct a shared struct directly in C++.
  cr_core::Point p{2.0, 3.0};
  std::cout << "c++-made point = (" << p.x << ", " << p.y << ")\n";
  return 0;
}
