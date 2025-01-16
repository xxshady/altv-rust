use std::mem::ManuallyDrop;

/// FFI-safe `&[T]`
#[repr(C)]
pub struct RawSlice<T> {
  pub ptr: *const T,
  pub len: usize,
}

impl<T> RawSlice<T> {
  /// # Safety
  /// See `Safety` of [`std::slice::from_raw_parts`]
  pub unsafe fn into_slice<'a>(self) -> &'a [T] {
    std::slice::from_raw_parts(self.ptr, self.len)
  }

  /// # Safety
  /// See `Safety` of [`std::slice::from_raw_parts`]
  pub unsafe fn to_vec(&self) -> Vec<T>
  where
    T: Clone,
  {
    std::slice::from_raw_parts(self.ptr, self.len).to_vec()
  }
}

impl<T> From<&[T]> for RawSlice<T> {
  fn from(value: &[T]) -> Self {
    RawSlice {
      ptr: value.as_ptr(),
      len: value.len(),
    }
  }
}

/// FFI-safe `&str`
#[repr(C)]
pub struct Str(RawSlice<u8>);

impl Str {
  /// # Safety
  /// See `Safety` of [`std::slice::from_raw_parts`]
  pub unsafe fn into_str<'a>(self) -> &'a str {
    let bytes = self.0.into_slice();
    std::str::from_utf8(bytes).expect("Failed to get valid UTF-8 string slice back")
  }

  /// # Safety
  /// See `Safety` of [`std::slice::from_raw_parts`]
  pub unsafe fn to_string(&self) -> String {
    let bytes = self.0.to_vec();
    String::from_utf8(bytes).expect("Failed to convert to valid UTF-8 string")
  }

  /// `From<&str>` for const contexts
  pub const fn const_from(value: &str) -> Self {
    let bytes = value.as_bytes();
    Self(RawSlice {
      ptr: bytes.as_ptr(),
      len: bytes.len(),
    })
  }
}

impl From<&str> for Str {
  fn from(value: &str) -> Self {
    Self(value.as_bytes().into())
  }
}

pub struct OwnedStr {
  ptr: *mut u8,
  len: usize,
  capacity: usize,
}

impl From<String> for OwnedStr {
  fn from(value: String) -> Self {
    let mut value = ManuallyDrop::new(value);

    Self {
      ptr: value.as_mut_ptr(),
      len: value.len(),
      capacity: value.capacity(),
    }
  }
}

impl From<OwnedStr> for String {
  fn from(value: OwnedStr) -> Self {
    unsafe { owned_str_into_string(value.ptr, value.len, value.capacity) }
  }
}

impl Clone for OwnedStr {
  fn clone(&self) -> Self {
    unsafe {
      let slice = std::slice::from_raw_parts(self.ptr, self.len);
      let str = std::str::from_utf8_unchecked(slice);
      let string = str.to_owned();
      string.into()
    }
  }
}

impl Drop for OwnedStr {
  fn drop(&mut self) {
    unsafe {
      owned_str_into_string(self.ptr, self.len, self.capacity);
    }
  }
}

unsafe fn owned_str_into_string(ptr: *mut u8, len: usize, capacity: usize) -> String {
  String::from_raw_parts(ptr, len, capacity)
}
