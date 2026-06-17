#include "cr_core/demo.h"

namespace cr_core {

Point lanelet_centroid(const Lanelet &lanelet) {
  double sum_x = 0.0;
  double sum_y = 0.0;
  size_t n = 0;

  // rust::Vec<Point> is iterable; fields are plain doubles.
  for (const Point &p : lanelet.left_bound) {
    sum_x += p.x;
    sum_y += p.y;
    ++n;
  }
  for (const Point &p : lanelet.right_bound) {
    sum_x += p.x;
    sum_y += p.y;
    ++n;
  }

  Point centroid{0.0, 0.0};
  if (n != 0) {
    centroid.x = sum_x / static_cast<double>(n);
    centroid.y = sum_y / static_cast<double>(n);
  }
  return centroid;
}

} // namespace cr_core
