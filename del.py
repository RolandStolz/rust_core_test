import copy


class OtherClass:
    def __init__(self) -> None:
        self.x: int = 1

class Test:
    def __init__(self) -> None:
        self.other = OtherClass()

a = Test()
b = a.other
print(id(a.other))
print(id(b))
print(a.other)

c = copy.copy(a)
d = copy.deepcopy(a)

del a

print(b)
print(c.other)
print(d.other)