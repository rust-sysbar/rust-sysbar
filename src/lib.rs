#[cfg(target_os = "macos")]
mod mac_os;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub trait Barfly {
    fn new(name: &str) -> Self;
    fn add_item(&mut self, menu_item: &str, cbs: Box<Fn() -> ()>);
    fn add_quit_item(&mut self, label: &str);
    fn display(&mut self);
}

#[cfg(target_os = "macos")]
pub type PlatformFly = mac_os::MacOsBarfly;

pub fn new(name: &str) -> PlatformFly {
    PlatformFly::new(name)
}

#[test]
fn it_works() {
    let mut bf = new("Test"); //this is barfly::new()
    bf.add_item("Test", Box::new(|| {}));
}
