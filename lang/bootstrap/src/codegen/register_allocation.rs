use core::fmt;
use std::collections::HashMap;

use crate::codegen::ir::Register;

use super::ir::{IRStatement, IR};

#[derive(Debug, Clone, PartialEq)]
struct Interval {
    start: usize,
    end: usize,
}
impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

pub fn spill_extra_virtual_registers(ir: &IR, register_count: u32) {
    let live_intervals = get_live_intervals(ir);
    let active_intervals: Vec<(Register, Interval)> = vec![];

    fn expire_old_intervals(i: Interval) {
        todo!();
    }

    todo!();
}

fn get_live_intervals(ir: &IR) -> HashMap<Register, Interval> {
    let mut intervals: HashMap<Register, Interval> = HashMap::new();

    for (i, ir_statement) in ir.statements.iter().enumerate() {
        let updated_registers = match ir_statement {
            IRStatement::LoadImmediate { rd, imm: _ } => vec![rd],
            IRStatement::Add { rd, rs1, rs2 } => vec![rd, rs1, rs2],
            IRStatement::Subtract { rd, rs1, rs2 } => vec![rd, rs1, rs2],
            IRStatement::And { rd, rs1, rs2 } => vec![rd, rs1, rs2],
            IRStatement::Or { rd, rs1, rs2 } => vec![rd, rs1, rs2],
            IRStatement::Xor { rd, rs1, rs2 } => vec![rd, rs1, rs2],
            IRStatement::LeftShift { rd, rs1, rs2 } => vec![rd, rs1, rs2],
            IRStatement::RightShift { rd, rs1, rs2 } => vec![rd, rs1, rs2],
            IRStatement::Branch { label: _ } => vec![],
            IRStatement::BranchNotZero { rs1, label: _ } => vec![rs1],
            IRStatement::BranchZero { rs1, label: _ } => vec![rs1],
            IRStatement::Label { label: _ } => vec![],
            IRStatement::Push { rs1 } => vec![rs1],
            IRStatement::Pop { rs1 } => vec![rs1],
        };

        for register in updated_registers {
            match intervals.get_mut(register) {
                None => {
                    intervals.insert(register.clone(), Interval { start: i, end: i });
                }
                Some(interval) => {
                    interval.end = i;
                }
            };
        }
    }

    intervals
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        codegen::{self, ir::Register, register_allocation::Interval},
        parser::Parser,
    };

    #[test]
    fn test_get_live_intervals() {
        let mut parser = Parser::new("let a = 1; let b = a + 1;");
        let program = parser.get_ast().unwrap().unwrap();
        let ir = codegen::ir::get_ir(&program).unwrap();
        let live_intervals = codegen::register_allocation::get_live_intervals(&ir);

        let mut expected_live_intervals: HashMap<Register, Interval> = HashMap::new();
        expected_live_intervals.insert(Register(1), Interval { start: 0, end: 2 });
        expected_live_intervals.insert(Register(2), Interval { start: 1, end: 2 });
        expected_live_intervals.insert(Register(3), Interval { start: 2, end: 2 });

        assert_eq!(live_intervals, expected_live_intervals);
    }
}
