#[macro_use]
extern crate rutie;

use rutie::{AnyException, Class, Exception, Object, RString, VM};

mod tree_sitter_adapter;

class!(TreeSitterAdapter);

methods!(
    TreeSitterAdapter,
    _rtself,
    fn pub_highlight(raw_code: RString, raw_parsers_dir: RString) -> RString {
        VM::unwrap_or_raise_ex(
            tree_sitter_adapter::highlight(
                &raw_code.unwrap().to_string(),
                &raw_parsers_dir.unwrap().to_string(),
            )
            .as_ref()
            .map(String::as_str)
            .map(RString::new_utf8)
            .map_err(String::as_str)
            .map_err(AnyException::new_runtime_error),
        )
    }
);

#[no_mangle]
pub extern "C" fn Init_tree_sitter_adapter() {
    Class::new("TreeSitterAdapter", None).define(|class_| {
        class_.def_self("highlight", pub_highlight);
    });
}

pub trait VMExt {
    fn unwrap_or_raise_ex<T, E>(x: Result<T, E>) -> T
    where
        E: Into<AnyException>;
}

impl VMExt for VM {
    fn unwrap_or_raise_ex<T, E>(result: Result<T, E>) -> T
    where
        E: Into<AnyException>,
    {
        result.unwrap_or_else(|e| {
            VM::raise_ex(e);
            unreachable!();
        })
    }
}

pub trait AnyExceptionExt {
    fn new_runtime_error(message: &str) -> Self;
}

impl AnyExceptionExt for AnyException {
    fn new_runtime_error(message: &str) -> Self {
        AnyException::new("RuntimeError", Some(message))
    }
}
