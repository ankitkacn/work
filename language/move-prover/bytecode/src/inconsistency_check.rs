// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! Instrument `assert false;` in strategic locations in the program such that if proved, signals
//! an inconsistency among the specifications.
//!
//! The presence of inconsistency is a serious issue. If there is an inconsistency in the
//! verification assumptions (perhaps due to a specification mistake or a Prover bug), any false
//! post-condition can be proved vacuously. The `InconsistencyCheckInstrumentationProcessor` adds
//! an `assert false;` before every `return;` instruction such that if this `assert false;` can
//! be proved, it means we have an inconsistency in the specifications.

use crate::{
    function_data_builder::FunctionDataBuilder,
    function_target::FunctionData,
    function_target_pipeline::{
        FunctionTargetProcessor, FunctionTargetsHolder, FunctionVariant, VerificationFlavor,
    },
    stackless_bytecode::{Bytecode, PropKind},
};

use move_model::{exp_generator::ExpGenerator, model::FunctionEnv};

// This message is for the boogie wrapper, and not shown to the users.
const EXPECTED_TO_FAIL: &str = "expected to fail";

pub struct InconsistencyCheckInstrumenter {}

impl InconsistencyCheckInstrumenter {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl FunctionTargetProcessor for InconsistencyCheckInstrumenter {
    fn process(
        &self,
        targets: &mut FunctionTargetsHolder,
        fun_env: &FunctionEnv<'_>,
        data: FunctionData,
    ) -> FunctionData {
        if fun_env.is_native() || fun_env.is_intrinsic() {
            // Nothing to do.
            return data;
        }
        let flavor = match &data.variant {
            FunctionVariant::Baseline
            | FunctionVariant::Verification(VerificationFlavor::Inconsistency(..)) => {
                // instrumentation only applies to non-inconsistency verification variants
                return data;
            }
            FunctionVariant::Verification(flavor) => flavor.clone(),
        };

        // create a clone of the data for inconsistency check
        let new_data = data.fork(FunctionVariant::Verification(
            VerificationFlavor::Inconsistency(Box::new(flavor)),
        ));

        // instrumentation
        let mut builder = FunctionDataBuilder::new(fun_env, new_data);
        let old_code = std::mem::take(&mut builder.data.code);
        for bc in old_code {
            if matches!(bc, Bytecode::Ret(..)) {
                let loc = builder.fun_env.get_spec_loc();
                builder.set_loc_and_vc_info(loc, EXPECTED_TO_FAIL);
                let exp = builder.mk_bool_const(false);
                builder.emit_with(|id| Bytecode::Prop(id, PropKind::Assert, exp));
            }
            builder.emit(bc);
        }

        // add the new variant to targets
        let new_data = builder.data;
        targets.insert_target_data(
            &fun_env.get_qualified_id(),
            new_data.variant.clone(),
            new_data,
        );

        // the original function data is unchanged
        data
    }

    fn name(&self) -> String {
        "inconsistency_check_instrumenter".to_string()
    }
}
