# How to compare custom types

To extend `float_eq` functionality over a new type requires implenting the
relevant extension traits.

1) If your type is a struct or tuple struct that consists of fields that already
implement the required traits, then you may use a derive macro. See [How to
derive the traits].

2) If your type cannot have the traits derived for it, of if you do not wish to
enable the "derive" feature, see [How to manually implement the traits].

[How to derive the traits]: ./derive_the_traits.html
[How to manually implement the traits]: ./manually_implement_the_traits.html