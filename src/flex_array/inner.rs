use core::alloc::Layout;
use core::ptr::NonNull;

use crate::alloc::AltAllocator;
#[cfg(feature = "std_alloc")]
use crate::alloc::Global;
use crate::types::ErrorReason;
use crate::types::FlexArrErr;
use crate::types::FlexArrResult;
use crate::types::LengthType;

const fn layout_array(layout: Layout, length: usize) -> FlexArrResult<Layout> {
    let lay = layout.pad_to_align();
    let Some(len) = length.checked_mul(lay.size()) else {
        return Err(FlexArrErr::new(ErrorReason::UsizeOverflow));
    };
    let Ok(lay) = Layout::from_size_align(len, layout.align()) else {
        return Err(FlexArrErr::new(ErrorReason::LayoutFailure));
    };
    return Ok(lay);
}

macro_rules! define_inner_struct {
    ($($global:ty)?) => {
        #[derive(Debug)]
        pub(crate) struct Inner<A: AltAllocator $(= $global)?, L: LengthType = u32>
        where
            usize: TryFrom<L>,
        {
            ptr:               NonNull<u8>,
            alloc:             A,
            // The length is put here so rust can pack the structs more tightly.
            // The inner impl does not use this field.
            pub(crate) length: L,
            capacity:          L,
        }
    };
}

#[cfg(feature = "std_alloc")]
define_inner_struct!(Global);

#[cfg(not(feature = "std_alloc"))]
define_inner_struct!();

impl<A: AltAllocator, L: LengthType> Inner<A, L>
where
    usize: TryFrom<L>,
{
    #[inline]
    pub(crate) const fn new_in<T>(alloc: A) -> Self {
        return Self {
            ptr:      NonNull::<T>::dangling().cast(),
            length:   L::ZERO_VALUE,
            capacity: L::ZERO_VALUE,
            alloc:    alloc,
        };
    }

    pub(crate) fn expand_capacity_at_least(&mut self, capacity: L, layout: Layout) -> FlexArrResult<()> {
        // Use the capacity function so this returns the MAX value for ZST.
        let old_cap = self.capacity(layout.size());
        // Increase the capacity by 50%
        // Also don't care if this overflows. Max ensures that will
        // at least get the capacity needed.
        let new_cap = old_cap.wrapping_add(old_cap >> L::ONE_VALUE);
        let new_cap = new_cap.max(capacity);
        let new_cap = new_cap.max(L::from(8u8));

        return self.expand_capacity_to(new_cap, layout);
    }

    pub(crate) fn expand_capacity_to(&mut self, capacity: L, layout: Layout) -> FlexArrResult<()> {
        if layout.size() == 0 {
            // Nothing needs allocated for a ZST.
            return Ok(());
        }

        let Ok(usz_cap) = usize::try_from(capacity) else {
            return Err(FlexArrErr::new(ErrorReason::UsizeOverflow));
        };

        let new_layout = layout_array(layout, usz_cap)?;

        // Safety: rust is pretty adamant about sizes not being over isize::MAX
        if new_layout.size() > (isize::MAX as usize) {
            return Err(FlexArrErr::new(ErrorReason::UsizeOverflow));
        }

        // Grow or do a normal allocation.
        let ptr = if let Some(old_layout) = self.current_layout(layout) {
            let Ok(ptr) = (unsafe { self.alloc.grow(self.ptr, old_layout, new_layout) }) else {
                return Err(FlexArrErr::new(ErrorReason::AllocFailure));
            };
            ptr
        } else {
            // There is no old layout so just allocate the new memory.
            let Ok(ptr) = self.alloc.allocate(new_layout) else {
                return Err(FlexArrErr::new(ErrorReason::AllocFailure));
            };
            ptr
        };

        self.ptr = ptr.cast();
        self.capacity = capacity;
        return Ok(());
    }

    fn current_layout(&self, layout: Layout) -> Option<Layout> {
        // Nothing has ever been allocated so there is no current layout.
        if self.capacity == L::ZERO_VALUE {
            return None;
        }
        let lay = layout.pad_to_align();
        let cap = self.capacity.as_usize();
        let size = lay.size() * cap;

        // Safety: This does not need to be checked since we have already
        // allocated the memory so all of these will be valid.
        let layout = unsafe { Layout::from_size_align_unchecked(size, lay.align()) };
        return Some(layout);
    }

    pub(crate) unsafe fn deallocate(&mut self, layout: Layout) {
        let Some(layout) = self.current_layout(layout) else {
            // Nothing has ever been allocated so there is no current layout.
            // So do not deallocate.
            return;
        };
        unsafe { self.alloc.deallocate(self.ptr, layout) };
    }

    #[inline]
    pub(crate) const fn capacity(&self, item_sz: usize) -> L {
        if item_sz == 0 {
            return L::MAX_VALUE;
        }
        return self.capacity;
    }

    #[inline]
    pub(crate) const fn get_ptr<T>(&self) -> *mut T {
        let ptr = self.ptr.cast::<T>();
        return ptr.as_ptr();
    }
}
