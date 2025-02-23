#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of:: < Foo > (), 1usize, concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of:: < Foo > (), 1usize, concat!("Alignment of ",
        stringify!(Foo))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN3Foo3barEv"]
    pub fn Foo_bar() -> ::std::os::raw::c_int;
}
impl Foo {
    #[inline]
    pub unsafe fn bar() -> ::std::os::raw::c_int {
        Foo_bar()
    }
}
extern "C" {
    #[link_name = "\u{1}_Z3foov"]
    pub fn foo() -> ::std::os::raw::c_int;
}
