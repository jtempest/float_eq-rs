# Introduction

[Floating point types] have [a reputation] for being [difficult to use] compared
to integer types. This is partly because they have unintuitive sources of
rounding error but it also stems from the kinds of calculations they are used
for, which contain plentiful other sources of [numeric error].

The [float_eq] crate provides an API for comparing floating point primitives,
structs and collections for equality that allows users to communicate their
reasoning and intent with respect to numeric error.

This guide is not designed to be read from cover to cover, but instead is a
series of categorised articles. If you are unsure of how floats work or why they
are considered difficult to use then begin with the [Basic Usage](./tutorials/basic_usage.md)
guide, otherwise check out one of the categories for more:

- [Tutorials](./tutorials.md): getting started.
- [How to guides](./how_to.md): solutions to specific problems.
- [Background](./background.md): explanation and discussion.
- [API documentation](./api_documentation.md): detailed technical reference.

[a reputation]: https://www.exploringbinary.com/floating-point-questions-are-endless-on-stackoverflow-com/
[difficult to use]: https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
[float_eq]: http://crates.io/crates/float_eq
[Floating point types]: https://floating-point-gui.de/formats/fp/
[numeric error]: https://en.wikipedia.org/wiki/Numerical_error

-----------

Thanks go to everyone who has provided feedback and patiently answered my questions, particularly Robin Leroy. This guide's structure was inspired by the [Diátaxis Framework].

[Diátaxis Framework]: https://diataxis.fr/