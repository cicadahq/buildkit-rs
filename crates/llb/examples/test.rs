use std::io::Write;

use buildkit_rs_llb::*;

fn main() {
    let builder_image =
        Image::new("alpine:latest").with_custom_name("Using alpine:latest as a builder");

    let command = Exec::shlex("/bin/sh -c \"echo 'hello world'\"")
        .with_custom_name("create a dummy file")
        .with_mount(Mount::layer_readonly(builder_image.output(), "/"))
        .with_mount(Mount::scratch("/out", 0));

    let a = Definition::new(command.output(0)).into_bytes();

    std::io::stdout().write_all(&a).unwrap();
}
