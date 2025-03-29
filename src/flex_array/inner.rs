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
    pub(crate) const fn new_in(alloc: A, align: usize) -> Self {
        let ptr = align as *mut u8;
        return Self {
            ptr:      unsafe { NonNull::new_unchecked(ptr) },
            capacity: L::ZERO_VALUE,
            alloc:    alloc,
        };
    }

    pub(crate) fn initial_alloc(alloc: A, capacity: L, layout: Layout) -> FlexArrResult<Self> {
        let Ok(cap) = usize::try_from(capacity) else {
            return Err(FlexArrErr::new(ErrorKind::UsizeOverflow));
        };
        let layout = layout_array(layout, cap)?;

        // Don't allocate anything in this case.
        if layout.size() == 0 {
            return Ok(Self::new_in(alloc, layout.align()));
        }

        // Safety: rust is pretty adamant about sizes not being over isize::MAX
        if layout.size() > (isize::MAX as usize) {
            return Err(FlexArrErr::new(ErrorKind::UsizeOverflow));
        }

        let Ok(ptr) = alloc.allocate(layout) else {
            return Err(FlexArrErr::new(ErrorKind::AllocFailure));
        };

        return Ok(Self {
            ptr:      ptr.cast(),
            capacity: capacity,
            alloc:    alloc,
        });
    }
}
