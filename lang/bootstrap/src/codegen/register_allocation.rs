use core::fmt;

use crate::codegen::ir::Register;

use super::ir::IR;

#[derive(Debug, Clone)]
struct Interval {
    start: u32,
    end: u32,
}
impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

pub fn spill_extra_virtual_registers(ir: &IR, register_count: u32) {
    let live_intervals: Vec<(Register, Interval)> = get_live_intervals(ir);
    let active_intervals: Vec<(Register, Interval)> = vec![];

    fn expire_old_intervals(i: Interval) {
        todo!();
    }

    todo!();
}

fn get_live_intervals(ir: &IR) -> Vec<(Register, Interval)> {
    let intervals: Vec<(Register, Interval)> = vec![];

    for ir_statement in &ir.statements {
        println!("{}", ir);
    }

    intervals
}
