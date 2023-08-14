use quote::quote;

pub(crate) fn shared_mod() -> proc_macro2::TokenStream {
    quote! {
        mod __shared {
            pub type FatPtr = u64;

            pub type Size = u32;
            pub type Ptr = u32;

            pub fn from_fat_ptr(fat_ptr: FatPtr) -> (Ptr, Size) {
                let ptr = (fat_ptr >> 32) as Ptr;
                let size = fat_ptr as Size;
                (ptr, size)
            }

            pub fn to_fat_ptr(ptr: Ptr, size: Size) -> FatPtr {
                ((ptr as u64) << 32) | (size as u64)
            }
        }
    }
}
