static mut UID: u32 = 0;

pub fn issue() -> u32 {
    unsafe {
        UID += 1;
        UID
    }
}