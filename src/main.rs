const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("{} v{}", PACKAGE_NAME, PACKAGE_VERSION);
}
