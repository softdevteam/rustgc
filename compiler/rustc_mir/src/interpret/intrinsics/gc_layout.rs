use rustc_middle::mir::interpret::Allocation;
use rustc_middle::ty::layout::LayoutCx;
use rustc_middle::ty::{ParamEnv, Ty, TyCtxt};
use rustc_span::symbol::sym;
use rustc_span::DUMMY_SP;
use rustc_target::abi::{Align, Size};
use std::mem::size_of;

fn gc_layout(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, param_env: ParamEnv<'tcx>) -> Vec<u8> {
    let layout = tcx.layout_of(param_env.and(ty)).unwrap();
    let w_size = Size::from_bytes(size_of::<usize>());
    let w_align = Align::from_bytes(w_size.bytes()).unwrap();

    if layout.align.abi.bytes() < (w_size.bytes()) || ty.is_no_trace(tcx.at(DUMMY_SP), param_env) {
        // If an ADT is aligned to a size smaller than 1 word, then it
        // contains no fields large enough to represent a GC pointer.
        //
        // By default, the collector will conservatively scan a struct's
        // memory, treating it as word-sized chunks. However, since rust is
        // free to shuffle field order around, any in-memory words which
        // represent actual pointers would be purely accidental. Such
        // structs should be ignored completely by the collector.
        return vec![0; 16]; // empty (bitmap, size) (u64,u64) as byte stream
    }

    let src_field_iter = ty.ty_adt_def().unwrap().all_fields();
    let mut bitmap: u64 = u64::MAX;

    for i in layout.fields.index_by_increasing_offset() {
        let target_offset = layout.fields.offset(usize::from(i));

        let layout_cx = LayoutCx { tcx, param_env };
        let field = layout.field(&layout_cx, i).unwrap();

        if field.size < w_size {
            // If a field is smaller than a word, it will either be shuffled
            // around in the struct or padded to the min alignment with
            // uninitialized memory. Whatever the case, the word it lives in is
            // never going to be a valid pointer.
            //
            // target_offset is the byte in the struct's layout where this field
            // begins. Diving by w_align gives the offset in words.
            let mask = target_offset.bytes() / w_align.bytes();
            bitmap &= !(1 << mask);
        }

        let no_trace = field.ty.is_no_trace(tcx.at(DUMMY_SP), param_env);

        if no_trace || tcx.has_attr(src_field_iter.clone().nth(i).unwrap().did, sym::no_trace) {
            let start = target_offset.bytes() / w_align.bytes();
            let end = start + (field.size.bytes() / w_align.bytes());
            for n in start..end {
                bitmap &= !(1 << n);
            }
        }
    }

    let ty_size = layout.size.bytes() / w_size.bytes(); // In words
    let mut pair = Vec::with_capacity(16);
    pair.extend_from_slice(&bitmap.to_le_bytes());
    pair.extend_from_slice(&ty_size.to_le_bytes());
    pair
}

/// Directly returns an `Allocation` containing bitmap of a type's GC layout.
crate fn alloc_gc_layout<'tcx>(
    tcx: TyCtxt<'tcx>,
    ty: Ty<'tcx>,
    param_env: ParamEnv<'tcx>,
) -> &'tcx Allocation {
    if ty.ty_adt_def().is_none() {
        unimplemented!("core::intrinsics::gc_layout::<T>() is currently only supported for ADTs");
    }

    let layout = gc_layout(tcx, ty, param_env);
    let alloc = Allocation::from_byte_aligned_bytes(layout);
    tcx.intern_const_alloc(alloc)
}
