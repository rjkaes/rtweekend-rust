# Rust Port of "Ray Tracing in One Weekend"

An initial straight Rust port of Peter Shirley's "Ray Tracing in One
Weekend"[1]

Mostly an exercise in writing Rust coming from a C/C++ and Ruby background.

Do not consider this a good example of Rust! It's an almost direct translation
from C++ without any consideration of whether it's *good* Rust code.  The only
differences are concessions to Rust's borrow checker, where the API changed
slightly to reduce the amount of copying.

[1] https://raytracing.github.io/books/RayTracingInOneWeekend.html
