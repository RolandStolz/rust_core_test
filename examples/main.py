from cr_core import Point, Lanelet
import cr_core


def main():
    print(Point.__name__)

    a = Point(1.0, 2.0)
    print(a)
    print(a.x)
    print(a.y)
    a.x = 5.0
    print(a.x)

    lanelet: Lanelet = cr_core.create_dummy_lanelet()
    print(lanelet)


if __name__ == "__main__":
    main()
