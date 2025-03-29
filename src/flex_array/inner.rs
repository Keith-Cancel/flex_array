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

#[derive(Debug)]
pub(crate) struct Inner<L: LengthType, A: AltAllocator>
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

impl<L: LengthType, A: AltAllocator> Inner<L, A>
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

    pub(crate) fn expand_at_least_by(&mut self, amount: L, layout: Layout) -> FlexArrResult<()> {
        // Use the capacity function so this returns the MAX value for ZST.
        let old_cap = self.capacity(layout.size());

        // Do this first since if this called for a ZST it means we
        // have hit the maximum length so we need to return an error.
        let Some(req_cap) = old_cap.checked_add(amount) else {
            return Err(FlexArrErr::new(ErrorKind::CapacityOverflow));
        };

        if layout.size() == 0 {
            // Nothing needs allocated for a ZST.
            return Ok(());
        }
        // Increase the capacity by 50%
        let ext_cap = old_cap + (old_cap >> L::ONE_VALUE);

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
        let cap = self.capacity.as_usize();
        let size = lay.size() * cap;

        // Safety: This does not need to bechecked since we have already
        // allocated the memory so all of these will be valid.
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
    pub(crate) const fn get_ptr<T>(&self) -> *mut T {
        let ptr = self.ptr.cast::<T>();
        return ptr.as_ptr();
    }
}
