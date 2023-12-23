extern crate device_sys;
use device_sys::{memory_allocate_device, memory_deallocate_device, transfer};
use std::ffi::c_void;
use std::mem::size_of;
use std::{default::Default, clone::Clone, fmt::{Display}};
use num_complex::Complex;


pub struct DevicePtr<T: Display+Default+Clone> {
    ptr: *mut T,
    size: i32,
}

impl<T: Display+Default+Clone> DevicePtr<T> {
    pub fn new(sz: i32) -> Self {
        // println!("Memory getting allocated in Rust\n");
        Self {
            ptr: unsafe { memory_allocate_device(sz*(size_of::<T>() as i32)) } as *mut T,
            size: sz,
        }
    }

    pub fn display(&self, count: i32) {
        let right_num = {
            if count < self.size {
                count
            }
            else {
                self.size
            }
        };
        unsafe {
            let mut z = vec![T::default(); right_num as usize];
            transfer(z.as_mut_ptr() as *mut c_void, self.ptr as *mut c_void, right_num);
            for ind in 0..(right_num as usize) {
                println!(
                    "{}\n", z[ind]
                );
            }
        };
    }
}

impl<T: Display+Default+Clone> Drop for DevicePtr<T> {
    fn drop(&mut self) {
        // println!("Memory getting deallocated in Rust");
        unsafe { memory_deallocate_device(self.ptr as *mut c_void) };
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut device_ptr = DevicePtr::<Complex<f64>>::new(100);
        device_ptr.display(10);
    }

    fn others() {
        todo!();
    }
}
