use buildkit_rs_llb::*;

fn main() {
    let base_image = Image::new("alpine:latest").with_custom_name("base_image");
    dbg!(base_image);
}
