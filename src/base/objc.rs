use crate::base::ptr;
use std::ptr::NonNull;

#[repr(C)]
struct OpaqueAutoreleasePool {
    _private: [u8; 0],
}

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_autoreleasePoolPush() -> Option<NonNull<OpaqueAutoreleasePool>>;
    fn objc_autoreleasePoolPop(pool: NonNull<OpaqueAutoreleasePool>);
}

struct AutoreleasePoolGuard {
    pool: NonNull<OpaqueAutoreleasePool>,
}

impl AutoreleasePoolGuard {
    fn push() -> AutoreleasePoolGuard {
        let pool = unsafe { objc_autoreleasePoolPush() }
            .expect("expecting objc_autoreleasePoolPush() to return a non-null value");
        AutoreleasePoolGuard { pool }
    }
}

impl Drop for AutoreleasePoolGuard {
    fn drop(&mut self) {
        unsafe { objc_autoreleasePoolPop(self.pool) }
    }
}

pub fn autorelease_pool<Ret, F: FnOnce() -> Ret>(f: F) -> Ret {
    let _pool = AutoreleasePoolGuard::push();
    f()
}
