use keychain::{GenericKeyPath, KeyPath as IKeyPath};
use panic::handle_exception_result;
use result::{CResult, CharPtr, ErrorPtr};
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyPath {
  purpose: u32,
  coin: u32,
  account: u32,
  change: u32,
  address: u32
}

impl IKeyPath for KeyPath {
  fn purpose(&self) -> u32 {
    self.purpose
  }
  fn coin(&self) -> u32 {
    self.coin
  }
  fn account(&self) -> u32 {
    self.account
  }
  fn change(&self) -> u32 {
    self.change
  }
  fn address(&self) -> u32 {
    self.address
  }
}

impl From<&IKeyPath> for KeyPath {
  fn from(path: &IKeyPath) -> Self {
    Self {
      purpose: path.purpose(),
      coin: path.coin(),
      account: path.account(),
      change: path.change(),
      address: path.address()
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn keypath_from_string(
  string: CharPtr, key_path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    let path = CStr::from_ptr(string as *const c_char).to_str().unwrap();
    GenericKeyPath::from(path).map_err(|err| err.into()).map(|path| (&path as &IKeyPath).into())
  })
  .response(key_path, error)
}
