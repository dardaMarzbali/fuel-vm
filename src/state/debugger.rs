use crate::state::{Breakpoint, DebugEval, ProgramState};

use fuel_types::{ContractId, Word};

use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone)]
/// Debugger implementation for the VM.
///
/// Required features:
/// - `debug`
pub struct Debugger {
    breakpoints: HashMap<ContractId, HashSet<Word>>,
    last_state: Option<ProgramState>,
}

impl Debugger {
    /// Set a new breakpoint in the provided location.
    pub fn set_breakpoint(&mut self, breakpoint: Breakpoint) {
        let contract = *breakpoint.contract();
        let pc = breakpoint.pc();

        self.breakpoints
            .get_mut(&contract)
            .map(|set| set.insert(pc))
            .map(|_| ())
            .unwrap_or_else(|| {
                let mut set = HashSet::new();

                set.insert(pc);

                self.breakpoints.insert(contract, set);
            });
    }

    /// Remove a breakpoint, if existent.
    pub fn remove_breakpoint(&mut self, breakpoint: &Breakpoint) {
        self.breakpoints
            .get_mut(breakpoint.contract())
            .map(|set| set.remove(&breakpoint.pc()));
    }

    /// Evaluate the current state of the interpreter whether or not a
    /// breakpoint was reached.
    pub fn eval_state(&mut self, contract: Option<&ContractId>, pc: Word) -> DebugEval {
        // Default contract address maps to unset contract target
        let contract = contract.copied().unwrap_or_default();
        let last_state = self.last_state.take();

        self.breakpoints
            .get(&contract)
            .map(|set| set.get(&pc))
            .flatten()
            .map(|_| {
                let breakpoint = Breakpoint::raw(contract, pc);

                match last_state {
                    Some(s) if s == breakpoint => DebugEval::Continue,
                    _ => breakpoint.into(),
                }
            })
            .unwrap_or_default()
    }

    /// Overwrite the last known state of the VM.
    pub fn set_last_state(&mut self, state: ProgramState) {
        self.last_state.replace(state);
    }

    /// Retried the last state of execution; return `None` if the VM was never
    /// executed.
    pub const fn last_state(&self) -> &Option<ProgramState> {
        &self.last_state
    }
}