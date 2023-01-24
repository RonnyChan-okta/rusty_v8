use std::mem::size_of;
use std::mem::MaybeUninit;

use crate::Local;
use crate::Value;

extern "C" {
  fn v8__PropertyDescriptor__CONSTRUCT(out: *mut PropertyDescriptor);
  fn v8__PropertyDescriptor__CONSTRUCT_Get_Set(
    this: *const PropertyDescriptor,
    get: *const Value,
    set: *const Value,
  );
  fn v8__PropertyDescriptor__DESTRUCT(this: *mut PropertyDescriptor);
  fn v8__PropertyDesctiptor__set_enumerable(
    this: *mut PropertyDescriptor,
    enumerable: bool,
  );
  fn v8__PropertyDesctiptor__set_configurable(
    this: *mut PropertyDescriptor,
    configurable: bool,
  );
}

#[repr(transparent)]
pub struct PropertyDescriptor([usize; 1]);

const _: () = {
  assert!(
    size_of::<PropertyDescriptor>() == size_of::<usize>(),
    "PropertyDescriptor size is not 1 usize"
  );
};

impl PropertyDescriptor {
  pub fn new() -> Self {
    let mut this = MaybeUninit::<Self>::uninit();
    unsafe {
      v8__PropertyDescriptor__CONSTRUCT(this.as_mut_ptr());
      this.assume_init()
    }
  }

  pub fn new_from_get_set(get: Local<Value>, set: Local<Value>) -> Self {
    let mut this = MaybeUninit::<Self>::uninit();
    unsafe {
      v8__PropertyDescriptor__CONSTRUCT_Get_Set(
        this.as_mut_ptr(),
        &*get,
        &*set,
      );
      this.assume_init()
    }
  }

  pub fn set_enumerable(&mut self, enumerable: bool) {
    unsafe { v8__PropertyDesctiptor__set_enumerable(self, enumerable) }
  }

  pub fn set_configurable(&mut self, configurable: bool) {
    unsafe { v8__PropertyDesctiptor__set_configurable(self, configurable) }
  }
}

impl Drop for PropertyDescriptor {
  fn drop(&mut self) {
    unsafe { v8__PropertyDescriptor__DESTRUCT(self) }
  }
}
