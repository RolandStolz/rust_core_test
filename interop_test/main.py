"""Demonstrates zero-copy access to a Rust core (`cr_core`) from a *separate*
PyO3 extension (`rust_consumer`), via a named PyCapsule.

Run: uv run python main.py
"""

import cr_core
import rust_consumer


def main() -> None:
    # 1. Producer object, owned by Python.
    p = cr_core.Point(1.0, 2.0)

    # The consumer borrows it zero-copy and returns the address it dereferenced.
    read_addr = rust_consumer.print_point(p)

    # 2. Proof it's the *same* memory, not a copy.
    assert read_addr == p._addr(), (hex(read_addr), hex(p._addr()))
    print(f"[main] address match: {hex(read_addr)} == {hex(p._addr())}  -> zero-copy\n")

    # 3. Mutate through the shared pointer in Rust; observe it from Python.
    rust_consumer.bump_point_x(p, 10.0)
    assert p.x == 11.0, p.x
    print(f"[main] after bump_point_x(+10): p.x = {p.x}  -> shared memory, not a copy\n")

    # 4. Nested struct (State holds a Point by value).
    state = cr_core.State(cr_core.Point(3.0, 3.0), 0.5, 1.5, 7)
    s_addr = rust_consumer.print_state(state)
    assert s_addr == state._addr()
    print(f"[main] state address match: {hex(s_addr)}\n")

    # 5. Vec-backed struct.
    lanelet = cr_core.create_dummy_lanelet()
    lanelet.id = 99
    l_addr = rust_consumer.print_lanelet(lanelet)
    assert l_addr == lanelet._addr()
    print(f"[main] lanelet address match: {hex(l_addr)}\n")

    # 6. Capsule name == type tag: a State cannot masquerade as a Point.
    rust_consumer.print_point(state)


if __name__ == "__main__":
    main()
