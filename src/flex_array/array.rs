use core::alloc::Layout;
use core::marker::PhantomData;
use core::ops::Index;
use core::ops::IndexMut;
use core::ptr;
use core::slice;

use super::inner::Inner;
use crate::types::AltAllocator;
use crate::types::ErrorReason;
use crate::types::FlexArrErr;
use crate::types::FlexArrResult;
#[cfg(feature = "std_alloc")]
use crate::types::Global;
use crate::types::LengthType;

macro_rules! define_array_struct {
    ($($global:ty)?) => {
        /// `FlexArr` is a dynamic array that addresses some of the limitations of Rust’s standard `Vec`.
        ///
        /// `FlexArr` uses fallible allocations, meaning that instead of panicking on allocation failure,
        /// it returns an error. This allow one to handle the error in a more graceful or robust manner.
        /// `Vec` does have some fallible allocation methods, but most are currently unstable.
        ///
        /// In addition, one can customize the type used for the length, capacity, and indexing operations.
        /// For example on a 64-bit system, the standard `Vec` typically uses 24 bytes. `FlexArr` specifying
        /// a smaller type than `usize` as a generic (e.g. `u32`) with `FlexArr` can reduce this overhead to
        /// just 16 bytes.
        ///
        /// Lastly, the allocator API is not stable yet, so this crate provides and alternate trait `AltAllocator`
        /// that works like `Allocator` the trait and can be used with `FlexArr`
        #[derive(Debug)]
        pub struct FlexArr<T, A: AltAllocator $(= $global)?, L: LengthType = u32>
        where
            usize: TryFrom<L>,
        {
            inner: Inner<A, L>,
            _ph:   PhantomData<T>,
        }
    };
}

#[cfg(feature = "std_alloc")]
define_array_struct!(Global);

#[cfg(not(feature = "std_alloc"))]
define_array_struct!();

impl<T, A: AltAllocator, L: LengthType> FlexArr<T, A, L>
where
    usize: TryFrom<L>,
{
    const LAYOUT: Layout = Layout::new::<T>();
    const SIZE: usize = size_of::<T>();

    /// Constructs a new, empty `FlexArr` using the given allocator.
    ///
    /// This function initializes the array without performing any memory allocation. The resulting
    /// `FlexArr` is empty, and memory will only be allocated when elements are added.
    pub const fn new_in(alloc: A) -> Self {
        return Self {
            inner: Inner::new_in::<T>(alloc),
            _ph:   PhantomData,
        };
    }

    /// Creates a new `FlexArr` with the specified capacity using the provided allocator.
    ///
    /// This function attempts to allocate enough memory for the desired capacity during initialization.
    /// If the allocation fails, a `FlexArrErr` is returned.
    pub fn with_capacity_in(alloc: A, capacity: L) -> FlexArrResult<Self> {
        let mut inner = Inner::new_in::<T>(alloc);
        inner.expand_capacity_to(capacity, Self::LAYOUT)?;
        return Ok(Self {
            inner: inner,
            _ph:   PhantomData,
        });
    }

    /// Removes and returns the last element from the `FlexArr`.
    ///
    /// If the array is empty, this method returns `None`.
    pub fn pop(&mut self) -> Option<T> {
        let len = self.inner.length;
        if len <= L::ZERO_VALUE {
            return None;
        }
        let ret = unsafe { ptr::read(self.as_ptr().add(len.as_usize() - 1)) };
        self.inner.length = len - L::ONE_VALUE;
        return Some(ret);
    }

    /// Appends an element to the end of the `FlexArr`.
    ///
    /// If there isn’t enough capacity, this method attempts to expand the underlying storage.
    /// Should the allocation fail, a `FlexArrErr` is returned.
    ///
    /// # Errors
    ///
    /// Returns a `FlexArrErr` if memory expansion fails or if there is a conversion error when
    /// determining the new index.
    pub fn push(&mut self, item: T) -> FlexArrResult<()> {
        let needed = self.capacity_needed(L::ONE_VALUE)?;

        if needed >= self.capacity() {
            self.inner.expand_capacity_at_least(needed, Self::LAYOUT)?;
        }

        let Ok(len) = usize::try_from(self.inner.length) else {
            return Err(FlexArrErr::new(ErrorReason::UsizeOverflow));
        };

        let loc = unsafe { self.as_mut_ptr().add(len) };
        unsafe { ptr::write(loc, item) };
        self.inner.length += L::ONE_VALUE;

        return Ok(());
    }

    /// Ensures that `FlexArr` has enough capacity to store at least `additional` more elements.
    /// It may reserve more than `additional` elements. You can use this if you anticipate
    /// how many elements need to be inserted to avoid frequent reallocations.
    ///
    /// If the capacity is already sufficient, this method does nothing.
    ///
    /// # Errors
    ///
    /// Returns a `FlexArrErr` if memory reallocation fails or if there is an error converting
    /// the required capacity.
    pub fn reserve(&mut self, additional: L) -> FlexArrResult<()> {
        let needed = self.capacity_needed(additional)?;
        let cap = self.capacity();
        if cap >= needed {
            return Ok(());
        }

        return self.inner.expand_capacity_at_least(needed, Self::LAYOUT);
    }

    pub fn reserve_exact(&mut self, additional: L) -> FlexArrResult<()> {
        let needed = self.capacity_needed(additional)?;
        let cap = self.capacity();
        if cap >= needed {
            return Ok(());
        }

        return self.inner.expand_capacity_to(needed, Self::LAYOUT);
    }

    /// Appends the slice to the end of the `FlexArr`. The type `T` must implement `Copy`.
    /// If your type does not implement `Copy`, use `extend_from_slice_clone`.
    pub fn extend_from_slice(&mut self, _slice: &[T])
    where
        T: Copy,
    {
    }

    pub fn extend_from_slice_clone()
    where
        T: Clone,
    {
    }

    /// Returns the number of elements `FlexArr` can store without needing to reallocate.
    ///
    /// For zero sized types, this function will return the maximum value for the `LengthType`.
    pub const fn capacity(&self) -> L {
        return self.inner.capacity(Self::SIZE);
    }

    /// Returns the number of elements in the `FlexArr`.
    #[inline]
    pub const fn len(&self) -> L {
        return self.inner.length;
    }

    /// Returns a reference to the underlying storage as a slice.
    /// Unfortunately, since a `slice` is a built in type, the indexing operations
    /// on it will be a `usize`.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.inner.length.as_usize()) }
    }

    /// Returns a mutable reference to the underlying storage as a slice.
    /// Unfortunately, since a `slice` is a built in type, the indexing operations
    /// on it will be a `usize`.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.inner.length.as_usize()) }
    }

    /// Returns a raw pointer to the underlying storage. If the type is zero sized
    /// the pointer value will be a dangling pointer. Like one would get with
    /// `NonNull::dangling()` ect...
    ///
    /// # Safety
    /// The caller should ensure the underlying storage outlives this pointer.
    /// Adding/removing items to the `FlexArr` can cause the pointer to become invalid.
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        return self.inner.get_ptr();
    }

    /// Returns a raw mutable pointer to the underlying storage. If the type is zero sized
    /// the pointer value will be a dangling pointer. Like one would get with
    /// `NonNull::dangling()` ect...
    ///
    /// # Safety
    /// The caller should ensure the underlying storage outlives this pointer.
    /// Adding/removing items to the `FlexArr` can cause the pointer to become invalid.
    #[inline]
    pub const fn as_mut_ptr(&self) -> *mut T {
        return self.inner.get_ptr();
    }

    #[inline(always)]
    pub fn capacity_needed(&self, requested: L) -> FlexArrResult<L> {
        let Some(needed) = self.inner.length.checked_add(requested) else {
            return Err(FlexArrErr::new(ErrorReason::CapacityOverflow));
        };
        return Ok(needed);
    }
}

#[cfg(feature = "std_alloc")]
impl<T, L: LengthType> FlexArr<T, Global, L>
where
    usize: TryFrom<L>,
{
    /// Creates a new, empty `FlexArr` using the standard allocator.
    ///
    /// This functions similarly to `FlexArr::new_in()`, but automatically
    /// uses the global allocator. No memory is allocated until elements are added
    ///
    /// This is only available if the `std_alloc` feature is enabled.
    pub const fn new() -> Self {
        return Self::new_in(Global);
    }

    /// Creates a new `FlexArr` with the specified capacity using the standard allocator.
    ///
    /// This functions similarly to `FlexArr::with_capacity_in()`, but automatically
    /// uses the global allocator.
    ///
    /// This is only available if the `std_alloc` feature is enabled.
    pub fn with_capacity(capacity: L) -> FlexArrResult<Self> {
        return Self::with_capacity_in(Global, capacity);
    }
}

impl<T, A: AltAllocator, L: LengthType> Index<L> for FlexArr<T, A, L>
where
    usize: TryFrom<L>,
{
    type Output = T;
    fn index(&self, index: L) -> &Self::Output {
        let i = index.as_usize();
        return &self.as_slice()[i];
    }
}

impl<T, A: AltAllocator, L: LengthType> IndexMut<L> for FlexArr<T, A, L>
where
    usize: TryFrom<L>,
{
    fn index_mut(&mut self, index: L) -> &mut Self::Output {
        let i = index.as_usize();
        return &mut self.as_mut_slice()[i];
    }
}

impl<T, A: AltAllocator, L: LengthType> Drop for FlexArr<T, A, L>
where
    usize: TryFrom<L>,
{
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.as_mut_slice());
            self.inner.deallocate(Self::LAYOUT);
        }
    }
}
