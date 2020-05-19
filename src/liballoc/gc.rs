#![allow(missing_docs)]
use core::alloc::Layout;
use core::any::Any;
use core::ffi::c_void;
use core::fmt;
use core::marker::PhantomData;
use core::mem::{self, ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};

use crate::alloc::{AllocInit, AllocRef};
use crate::boehm::{self, BoehmGcAllocator};

/// A garbage collected pointer.
///
/// The type `Gc<T>` provides shared ownership of a value of type `T`,
/// allocted in the heap. `Gc` pointers are `Copyable`, so new pointers to
/// the same value in the heap can be produced trivially. The lifetime of
/// `T` is tracked automatically: it is freed when the application
/// determines that no references to `T` are in scope. This does not happen
/// deterministically, and no guarantees are given about when a value
/// managed by `Gc` is freed.
///
/// Shared references in Rust disallow mutation by default, and `Gc` is no
/// exception: you cannot generally obtain a mutable reference to something
/// inside an `Gc`. If you need mutability, put a `Cell` or `RefCell` inside
/// the `Gc`.
///
/// Unlike `Rc<T>`, cycles between `Gc` pointers are allowed and can be
/// deallocated without issue.
///
/// `Gc<T>` automatically dereferences to `T` (via the `Deref` trait), so
/// you can call `T`'s methods on a value of type `Gc<T>`.
#[unstable(feature = "gc", reason = "gc", issue = "none")]
pub struct Gc<T: ?Sized> {
    ptr: NonNull<GcBox<T>>,
    _phantom: PhantomData<T>,
}

impl<T> Gc<T> {
    /// Constructs a new `Gc<T>`.
    pub fn new(v: T) -> Self {
        Gc { ptr: unsafe { NonNull::new_unchecked(GcBox::new(v)) }, _phantom: PhantomData }
    }

    /// Constructs a new `Gc<MaybeUninit<T>>` which is capable of storing data
    /// up-to the size permissible by `layout`.
    ///
    /// This can be useful if you want to store a value with a custom layout,
    /// but have the collector treat the value as if it were T.
    ///
    /// `layout` must be at least as large as `T`, and have an alignment which
    /// is the same, or bigger than, `T`.
    pub fn new_from_layout(layout: Layout) -> Option<Gc<MaybeUninit<T>>> {
        let tl = Layout::new::<T>();
        if layout.size() < tl.size() && layout.align() >= tl.align() {
            return None;
        }
        Some(Gc::from_inner(GcBox::new_from_layout(layout)))
    }
}

impl Gc<dyn Any> {
    pub fn downcast<T: Any>(&self) -> Result<Gc<T>, Gc<dyn Any>> {
        if (*self).is::<T>() {
            let ptr = self.ptr.cast::<GcBox<T>>();
            Ok(Gc::from_inner(ptr))
        } else {
            Err(Gc::from_inner(self.ptr))
        }
    }
}

impl<T: ?Sized> Gc<T> {
    /// Get a raw pointer to the underlying value `T`.
    pub fn into_raw(this: Self) -> *const T {
        this.ptr.as_ptr() as *const T
    }

    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.ptr.as_ptr() == other.ptr.as_ptr()
    }

    pub fn from_raw(raw: *const T) -> Gc<T> {
        Gc { ptr: unsafe { NonNull::new_unchecked(raw as *mut GcBox<T>) }, _phantom: PhantomData }
    }

    fn from_inner(ptr: NonNull<GcBox<T>>) -> Self {
        Self { ptr, _phantom: PhantomData }
    }
}

impl<T> Gc<MaybeUninit<T>> {
    /// As with `MaybeUninit::assume_init`, it is up to the caller to guarantee
    /// that the inner value really is in an initialized state. Calling this
    /// when the content is not yet fully initialized causes immediate undefined
    /// behaviour.
    pub unsafe fn assume_init(self) -> Gc<T> {
        let ptr = self.ptr.as_ptr() as *mut GcBox<MaybeUninit<T>>;
        Gc::from_inner((&mut *ptr).assume_init())
    }
}

/// A `GcBox` is a 0-cost wrapper which allows a single `Drop` implementation
/// while also permitting multiple, copyable `Gc` references. The `drop` method
/// on `GcBox` acts as a guard, preventing the destructors on its contents from
/// running unless the object is really dead.
#[unstable(feature = "gc", reason = "gc", issue = "none")]
struct GcBox<T: ?Sized>(ManuallyDrop<T>);

impl<T> GcBox<T> {
    fn new(value: T) -> *mut GcBox<T> {
        let layout = Layout::new::<T>();
        let ptr = BoehmGcAllocator.alloc(layout, AllocInit::Uninitialized).unwrap().ptr.as_ptr()
            as *mut GcBox<T>;
        let gcbox = GcBox(ManuallyDrop::new(value));

        unsafe {
            ptr.copy_from_nonoverlapping(&gcbox, 1);
            GcBox::register_finalizer(&mut *ptr);
        }

        mem::forget(gcbox);
        ptr
    }

    fn new_from_layout(layout: Layout) -> NonNull<GcBox<MaybeUninit<T>>> {
        unsafe {
            let base_ptr =
                BoehmGcAllocator.alloc(layout, AllocInit::Uninitialized).unwrap().ptr.as_ptr()
                    as *mut usize;
            NonNull::new_unchecked((base_ptr.add(1)) as *mut GcBox<MaybeUninit<T>>)
        }
    }

    fn register_finalizer(&mut self) {
        unsafe extern "C" fn fshim<T>(obj: *mut c_void, _meta: *mut c_void) {
            ManuallyDrop::drop(&mut *(obj as *mut ManuallyDrop<T>));
        }

        unsafe {
            boehm::GC_register_finalizer(
                self as *mut _ as *mut c_void,
                fshim::<T>,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }
    }
}

impl<T> GcBox<MaybeUninit<T>> {
    unsafe fn assume_init(&mut self) -> NonNull<GcBox<T>> {
        // With T now considered initialized, we must make sure that if GcBox<T>
        // is reclaimed, T will be dropped. We need to find its vptr and replace the
        // GcDummyDrop vptr in the block header with it.
        self.register_finalizer();
        NonNull::new_unchecked(self as *mut _ as *mut GcBox<T>)
    }
}

#[unstable(feature = "gc", reason = "gc", issue = "none")]
impl<T: ?Sized> Copy for Gc<T> {}

#[unstable(feature = "gc", reason = "gc", issue = "none")]
impl<T: ?Sized> Clone for Gc<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[unstable(feature = "gc", reason = "gc", issue = "none")]
impl<T: ?Sized> Deref for Gc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.ptr.as_ptr() as *const T) }
    }
}

#[unstable(feature = "gc", reason = "gc", issue = "none")]
impl<T: ?Sized> DerefMut for Gc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr.as_ptr() as *mut T) }
    }
}

#[unstable(feature = "gc", reason = "gc", issue = "none")]
impl<T: ?Sized + fmt::Debug> fmt::Debug for Gc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[unstable(feature = "gc", reason = "gc", issue = "none")]
impl<T: ?Sized + fmt::Display> fmt::Display for Gc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}
