#[macro_use]
extern crate rutie;

use rutie::{Class, Object, RString};

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

class!(RutieExample);

methods!(
    RutieExample,
    _rtself,
    fn pub_reverse(raw_input: RString) -> RString {
        let output = {
            let input = raw_input.unwrap().to_string();
            reverse(&input)
        };
        RString::new_utf8(&output)
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rutie_ruby_example() {
    Class::new("RutieExample", None).define(|klass| {
        klass.def_self("reverse", pub_reverse);
    });
}
