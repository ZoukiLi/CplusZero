# CplusZero

CplusZero is a modern, high-level programming language that transpiles to C, offering zero-cost abstraction and powerful features such as traits and generics.

## What Does "Zero" Mean?

- **Zero Gap from C**: CplusZero generates C code that can be used seamlessly with existing C codebases, without any concerns over FFI.
- **Zero Black Box**: The generated C code is human-readable, making it easy to understand and integrate with existing C code.
- **Zero Cost**: CplusZero supports modern concepts such as traits and slices while still maintaining zero-cost abstractions, making it an efficient and powerful language.

## Key Features

- **Traits**: Use traits to define interfaces and create polymorphic functions.
- **Enums**: Use enums to create types that can be one of a fixed set of variants.
- **Generics**: Create generic functions and data structures that can be used with any data type.
- **Safe Slices and Strings**: CplusZero provides safer alternatives to C pointers for manipulating slices and strings.
- **Transpiles to C**: CplusZero code can be compiled to C and integrated with existing C codebases.

## Example

Here's an example of using traits in CplusZero:

```rust
trait Printable {
    fn print(&self);
}

struct Point {
    x: i32,
    y: i32,
}

impl Printable for Point {
    fn print(&self) {
        printf("Point(%d, %d)", self.x, self.y);
    }
}

struct Circle {
    center: Point,
    radius: i32,
}

impl Printable for Circle {
    fn print(&self) {
        printf("Circle(%d, %d, %d)", self.center.x, self.center.y, self.radius);
    }
}

enum Shape {
    Point(Point),
    Circle(Circle),
}

impl Printable for Shape {
    fn print(&self) {
        match self {
            Shape::Point(point) => point.print(),
            Shape::Circle(circle) => circle.print(),
        }
    }
}

fn main() {
    let v = [Shape::Point(Point { x: 1, y: 2 }), Shape::Circle(Circle { center: Point { x: 3, y: 4 }, radius: 5 })];
    for shape in v {
        shape.print();
    }
}

```

will be transpiled to:

```c
struct Point {
    int x;
    int y;
};

void Point_print(struct Point* self) {
    printf("Point(%d, %d)", self->x, self->y);
}

struct Circle {
    struct Point center;
    int radius;
};

void Circle_print(struct Circle* self) {
    printf("Circle(%d, %d, %d)", self->center.x, self->center.y, self->radius);
}

enum ShapeType {
    Point,
    Circle,
};

struct Shape {
    enum ShapeType type;
    union {
        struct Point Point;
        struct Circle Circle;
    };
};

void Shape_print(struct Shape* self) {
    switch (self->type) {
        case Point:
            Point_print(&self->Point);
            break;
        case Circle:
            Circle_print(&self->Circle);
            break;
    }
}

int main() {
    struct Shape v[] = {
        { .type = Point, .Point = { .x = 1, .y = 2 } },
        { .type = Circle, .Circle = { .center = { .x = 3, .y = 4 }, .radius = 5 } },
    };
    for (int i = 0; i < sizeof(v) / sizeof(v[0]); i++) {
        Shape_print(&v[i]);
    }
}
```
