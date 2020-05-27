#![allow(dead_code)]
#![allow(unused_imports)]
use crate::transform::{MirPass, MirSource};
use crate::util::patch::MirPatch;
use rustc_infer::infer::TyCtxtInferExt;
use rustc_middle::mir::*;
use rustc_middle::traits;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::DUMMY_SP;
use rustc_trait_selection::traits::predicate_for_trait_def;
use rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt;

const VEC_WITH_CAPACITY: &str = "std::vec::Vec::<T>::with_capacity";
const VEC_PUSH: &str = "std::vec::Vec::<T>::push";

pub struct GcPreallocator;

impl<'tcx> MirPass<'tcx> for GcPreallocator {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, source: MirSource<'tcx>, body: &mut BodyAndCache<'tcx>) {
        if !tcx.sess.opts.debugging_opts.gc_destination_propagation {
            return;
        }

        let param_env = tcx.param_env(source.def_id()).with_reveal_all();
        GcPatcher { tcx, source, param_env }.run_pass(body);
    }
}

struct GcPatcher<'tcx> {
    tcx: TyCtxt<'tcx>,
    source: MirSource<'tcx>,
    param_env: ty::ParamEnv<'tcx>,
}

impl GcPatcher<'tcx> {
    fn run_pass(&mut self, body: &mut BodyAndCache<'tcx>) {
        let mut patch = MirPatch::new(body);
        for (bb, bb_data) in body.basic_blocks().iter_enumerated() {
            let terminator = bb_data.terminator();
            if let TerminatorKind::Call {
                ref func,
                ref args,
                ref destination,
                cleanup,
                from_hir_call,
            } = terminator.kind
            {
                if let ty::FnDef(callee_def_id, substs) = func.ty(&**body, self.tcx).kind {
                    let name = self.tcx.def_path_str(callee_def_id);
                    let new_term = match name.as_str() {
                        VEC_WITH_CAPACITY => {
                            if elem_ty_must_drop(substs.type_at(0), self.tcx, self.param_env) {
                                continue;
                            }
                            let new_callee = self.tcx.lang_items().vec_with_capacity_fn().unwrap();
                            let func =
                                Operand::function_handle(self.tcx, new_callee, substs, DUMMY_SP);
                            TerminatorKind::Call {
                                func,
                                args: args.clone(),
                                destination: destination.clone(),
                                cleanup,
                                from_hir_call,
                            }
                        }
                        VEC_PUSH => {
                            if elem_ty_must_drop(substs.type_at(0), self.tcx, self.param_env) {
                                continue;
                            }
                            let new_callee = self.tcx.lang_items().vec_push_gc_fn().unwrap();
                            let func =
                                Operand::function_handle(self.tcx, new_callee, substs, DUMMY_SP);
                            TerminatorKind::Call {
                                func,
                                args: args.clone(),
                                destination: destination.clone(),
                                cleanup,
                                from_hir_call,
                            }
                        }
                        _ => continue,
                    };
                    patch.patch_terminator(bb, new_term);
                }
            }
        }
        patch.apply(body);
    }
}

fn elem_ty_must_drop<'tcx>(ty: Ty<'tcx>, tcx: TyCtxt<'tcx>, param_env: ty::ParamEnv<'tcx>) -> bool {
    if !ty.needs_drop(tcx, param_env) {
        return false;
    } else {
        let prealloc_trait_id = tcx.lang_items().manageable_contents_trait().unwrap();
        let obligation = predicate_for_trait_def(
            tcx,
            param_env,
            traits::ObligationCause::dummy(),
            prealloc_trait_id,
            0,
            ty,
            &[],
        );
        tcx.infer_ctxt().enter(|infcx| infcx.predicate_must_hold_modulo_regions(&obligation))
    }
}
