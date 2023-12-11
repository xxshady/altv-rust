use altv_sdk::ffi as sdk;
use autocxx::prelude::*;
use shared::Vector3;

pub fn read_cpp_vector3(cpp_vector: UniquePtr<sdk::Vector3Wrapper>) -> Vector3 {
    let (mut x, mut y, mut z) = Default::default();
    unsafe {
        sdk::read_vector3(cpp_vector.as_ref().unwrap(), &mut x, &mut y, &mut z);
    }
    Vector3 { x, y, z }
}

pub fn create_c_string_ptr(string: Option<String>) -> UniquePtr<sdk::natives::CStringPtr> {
    if let Some(string) = string {
        unsafe { sdk::natives::create_c_string_ptr(string).within_unique_ptr() }
    } else {
        unsafe { sdk::natives::create_null_c_string_ptr().within_unique_ptr() }
    }
}

pub fn read_c_string_ptr(ptr: UniquePtr<sdk::natives::CStringPtr>) -> Option<String> {
    unsafe {
        let ptr_ref = ptr.as_ref().unwrap();
        if sdk::natives::is_c_string_ptr_null(ptr_ref) {
            None
        } else {
            Some(sdk::natives::read_c_string_ptr(ptr_ref).to_string())
        }
    }
}

pub fn create_vector3_ptr(vector3: Option<Vector3>) -> UniquePtr<sdk::Vector3Wrapper> {
    if let Some(vector3) = vector3 {
        unsafe {
            sdk::create_vector3_with_values(vector3.x, vector3.y, vector3.z).within_unique_ptr()
        }
    } else {
        unsafe {
            sdk::create_vector3().within_unique_ptr() 
        }
    }
}
