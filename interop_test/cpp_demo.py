"""Rust core -> Python -> C++ zero-copy.

`cr_core` objects come from the Rust/PyO3 core. `cpp_consumer` is a C++/nanobind
module that reads them in place through the cxx struct view + capsule pointer.

Run: uv run python cpp_demo.py
"""

import cr_core
import cpp_consumer


def main() -> None:
    # Producer object from the Rust core.
    p = cr_core.Point(1.0, 2.0)

    # C++ borrows it zero-copy and returns the address it dereferenced.
    addr = cpp_consumer.print_point(p)
    assert addr == p._addr(), (hex(addr), hex(p._addr()))
    print(f"[main] address match: {hex(addr)} == {hex(p._addr())}  -> Rust->Python->C++ zero-copy\n")

    # C++ mutates through the shared pointer; Python sees it.
    cpp_consumer.bump_point_x(p, 10.0)
    assert p.x == 11.0, p.x
    print(f"[main] after C++ bump_point_x(+10): p.x = {p.x}  -> shared memory\n")

    # Nested struct.
    state = cr_core.State(cr_core.Point(3.0, 3.0), 0.5, 1.5, 7)
    s_addr = cpp_consumer.print_state(state)
    assert s_addr == state._addr()
    print(f"[main] state address match: {hex(s_addr)}\n")

    # Vec-backed struct (C++ reads the POD `id`).
    lanelet = cr_core.create_dummy_lanelet()
    lanelet.id = 99
    l_addr = cpp_consumer.print_lanelet(lanelet)
    assert l_addr == lanelet._addr()
    print(f"[main] lanelet address match: {hex(l_addr)}\n")

    # Capsule name == type tag: a State cannot masquerade as a Point on the C++ side.
    try:
        cpp_consumer.print_point(state)
    except Exception as e:
        print(f"[main] type guard works: print_point(State) rejected -> {type(e).__name__}: {e}")
    else:
        raise SystemExit("ERROR: expected the wrong-type capsule to be rejected")


if __name__ == "__main__":
    main()
