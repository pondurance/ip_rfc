# IP RFC backport

This is a method to allow checking whether an IP address is globally routable.

While this can be done using the `is_global()` method in nightly, the feature is still unstable, and there has been a lot of debate on if/when it will finally be stabilized. This library provides matching functionality, based on the unstable version, but usable with a stable compiler.
