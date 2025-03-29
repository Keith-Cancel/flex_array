use core::alloc::Layout;
use core::ptr::NonNull;

use crate::types::AltAllocator;
use crate::types::ErrorKind;
use crate::types::FlexArrErr;
use crate::types::FlexArrResult;
use crate::types::LengthType;

const fn layout_array(layout: Layout, length: usize) -> FlexArrResult<Layout> {
    let lay = layout.pad_to_align();
    let Some(len) = length.checked_mul(lay.size()) else {
        return Err(FlexArrErr::new(ErrorKind::UsizeOverflow));
    };
    let Ok(lay) = Layout::from_size_align(len, layout.align()) else {
        return Err(FlexArrErr::new(ErrorKind::LayoutFailure));
    };
    return Ok(lay);
}

pub(crate) struct Inner<L: LengthType, A: AltAllocator>
where
    usize: TryFrom<L>,
{
    ptr:      NonNull<u8>,
    capacity: L,
    alloc:    A,
}

impl<L: LengthType, A: AltAllocator> Inner<L, A>
where
    usize: TryFrom<L>,
{
    #[inline]
    pub(crate) const fn new_in<T>(alloc: A) -> Self {
        return Self {
            ptr:      NonNull::<T>::dangling().cast(),
            capacity: L::ZERO_VALUE,
            alloc:    alloc,
        };
    }

    pub(crate) fn expand_at_least_by(&mut self, amount: L, layout: Layout) -> FlexArrResult<()> {
        if layout.size() == 0 {
            // Nothing needs allocated for a ZST.
            return Ok(());
        }
        let old_cap = self.capacity;

        let Some(req_cap) = old_cap.checked_add(amount) else {
            return Err(FlexArrErr::new(ErrorKind::CapacityOverflow));
        };
        // Increase the capacity by 50%
        let ext_cap = old_cap + (old_cap >> L::from(1u8));

        // Use the larger of these
        let ext_cap = ext_cap.max(req_cap);
        let ext_cap = ext_cap.max(L::from(8u8));
        let amt = ext_cap - old_cap;

        return self.expand_by(amt, layout);
    }

    pub(crate) fn expand_by(&mut self, amount: L, layout: Layout) -> FlexArrResult<()> {
        if layout.size() == 0 {
            // Nothing needs allocated for a ZST.
            return Ok(());
        }

        let Some(new_cap) = self.capacity.checked_add(amount) else {
            return Err(FlexArrErr::new(ErrorKind::CapacityOverflow));
        };

        let Ok(new_ucap) = usize::try_from(new_cap) else {
            return Err(FlexArrErr::new(ErrorKind::UsizeOverflow));
        };

        let new_layout = layout_array(layout, new_ucap)?;

        // Safety: rust is pretty adamant about sizes not being over isize::MAX
        if new_layout.size() > (isize::MAX as usize) {
            return Err(FlexArrErr::new(ErrorKind::UsizeOverflow));
        }

        // Grow or do a normal allocation.
        let ptr = if let Some(old_layout) = self.current_layout(layout) {
            let Ok(ptr) = (unsafe { self.alloc.grow(self.ptr, old_layout, new_layout) }) else {
                return Err(FlexArrErr::new(ErrorKind::AllocFailure));
            };
            ptr
        } else {
            // There is no old layout so just allocate the new memory.
            let Ok(ptr) = self.alloc.allocate(new_layout) else {
                return Err(FlexArrErr::new(ErrorKind::AllocFailure));
            };
            ptr
        };

        self.ptr = ptr.cast();
        self.capacity = new_cap;
        return Ok(());
    }

    fn current_layout(&self, layout: Layout) -> Option<Layout> {
        // Nothing has ever been allocated so there is no current layout.
        if self.capacity == L::ZERO_VALUE {
            return None;
        }
        let lay = layout.pad_to_align();

        // Safety: none of these need checked since we have already
        // allocated the memory so all of these will be valid.
        let cap = unsafe { usize::try_from(self.capacity).unwrap_unchecked() };
        let size = unsafe { lay.size().unchecked_mul(cap) };
        let layout = unsafe { Layout::from_size_align_unchecked(size, lay.align()) };
        return Some(layout);
    }

    #[inline]
    pub(crate) const fn capacity(&self, item_sz: usize) -> L {
        if item_sz == 0 {
            return L::MAX_VALUE;
        }
        return self.capacity;
    }

    #[inline]
    pub(crate) const fn real_capacity(&self) -> L {
        return self.capacity;
    }

    #[inline]
    pub(crate) const fn get_ptr<T>(&self) -> *mut T {
        let ptr = self.ptr.cast::<T>();
        return ptr.as_ptr();
    }
}
