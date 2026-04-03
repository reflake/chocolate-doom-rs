pub mod fixed;
pub mod trigonometry;
pub mod mode;
pub mod tri_tables;
pub mod tickcmd;
pub mod vector;

pub fn ptr_as_ref_mut<'a, T>(ptr: *mut T) -> Option<&'a mut T>
{
    unsafe {
        if !ptr.is_null() {
            Some(&mut *ptr)
        } else {
            None
        }
    }
}

pub fn ptr_as_ref<'a, T>(ptr: *const T) -> Option<&'a T>
{
    unsafe {
        if !ptr.is_null() {
            Some(&*ptr)
        } else {
            None
        }
    }
}