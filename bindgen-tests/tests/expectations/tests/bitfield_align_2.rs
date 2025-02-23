#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(not(target_os = "windows"))]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MyEnum {
    ONE = 0,
    TWO = 1,
    THREE = 2,
    FOUR = 3,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct TaggedPtr {
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[test]
fn bindgen_test_layout_TaggedPtr() {
    assert_eq!(
        ::std::mem::size_of:: < TaggedPtr > (), 8usize, concat!("Size of: ",
        stringify!(TaggedPtr))
    );
    assert_eq!(
        ::std::mem::align_of:: < TaggedPtr > (), 8usize, concat!("Alignment of ",
        stringify!(TaggedPtr))
    );
}
impl Default for TaggedPtr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl TaggedPtr {
    #[inline]
    pub fn tag(&self) -> MyEnum {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_tag(&mut self, val: MyEnum) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn ptr(&self) -> ::std::os::raw::c_long {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 62u8) as u64) }
    }
    #[inline]
    pub fn set_ptr(&mut self, val: ::std::os::raw::c_long) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 62u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        tag: MyEnum,
        ptr: ::std::os::raw::c_long,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                2u8,
                {
                    let tag: u32 = unsafe { ::std::mem::transmute(tag) };
                    tag as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                2usize,
                62u8,
                {
                    let ptr: u64 = unsafe { ::std::mem::transmute(ptr) };
                    ptr as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
