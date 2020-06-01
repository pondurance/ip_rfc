# IP RFC backport

This is a method to allow checking whether an IP address is globally routable.

While this can be done using the `is_global()` method in nightly, the feature is still unstable, and there has been a lot of debate on if/when it will finally be stabilized. This library provides matching functionality, based on the unstable version, but usable with a stable compiler.

# Future plans

This version, extracted from the nightly compiler, with sligh adjustments, can be used in place of the `is_global()` method on stable rust, and as soon as the feature is stabalized (see https://github.com/rust-lang/rust/pull/66584) then this library can be deprecated.
