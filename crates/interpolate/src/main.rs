use interpolate::{f, print, Formatter};

fn main() {
    let name = "Robot";

    // Generated from: ```
    // print(f"Hello {name}!\n");
    // ```
    print(f(["Hello ", "!\n"]).fill(name).finish());

    // let byte_string = b(["abcd"]).finish();
}
