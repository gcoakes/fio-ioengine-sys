# fio-ioengine-sys

Rust bindings to [fio](https://github.com/axboe/fio)'s ioengine API. This
enables writing custom I/O engine plugins in Rust. For an example of how to use
it, refer to the [hello-null](examples/hello_null.rs) which is a near 1-1
translation of fio's null I/O engine. There is also an example of how to debug
an I/O engine in VSCode using [launch.json](.vscode/launch.json). This is
generally a bad practice to checkin editor specific configuration files, but
this has been done to provide a good example of the correct usage.
