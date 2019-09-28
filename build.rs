extern crate cc;

fn main() {
    cc::Build::new().file("main.c").compile("libcompile.a");
}
