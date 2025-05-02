use std::collections::HashMap;

use crate::ir::{IRStatement, Label, Register, IR};

pub fn interpret(ir: &IR) {
    let mut registers: HashMap<Register, u64> = HashMap::new();
    let mut label_locations: HashMap<Label, usize> = HashMap::new();

    for (i, statement) in ir.statements.iter().enumerate() {
        if let IRStatement::Label { label } = statement {
            label_locations.insert(label.clone(), i);
        }
    }

    let mut pc = 0;
    while pc < ir.statements.len() {
        match &ir.statements[pc] {
            IRStatement::LoadImmediate { rd, imm } => {
                registers.insert(rd.clone(), *imm);
                pc += 1;
            }
            IRStatement::Add { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                registers.insert(rd.clone(), rs1_val + rs2_val);
                pc += 1;
            }
            IRStatement::Subtract { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                registers.insert(rd.clone(), rs1_val - rs2_val);
                pc += 1;
            }
            IRStatement::And { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                registers.insert(rd.clone(), rs1_val & rs2_val);
                pc += 1;
            }
            IRStatement::Or { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                registers.insert(rd.clone(), rs1_val | rs2_val);
                pc += 1;
            }
            IRStatement::Xor { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                registers.insert(rd.clone(), rs1_val ^ rs2_val);
                pc += 1;
            }
            IRStatement::LeftShift { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                registers.insert(rd.clone(), rs1_val << rs2_val);
                pc += 1;
            }
            IRStatement::RightShift { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                registers.insert(rd.clone(), rs1_val >> rs2_val);
                pc += 1;
            }
            IRStatement::Branch { label } => pc = label_locations[label],
            IRStatement::BranchNotZero { rs1, label } => {
                let rs1_val = *registers.get(rs1).unwrap();
                if rs1_val != 0 {
                    pc = label_locations.get(label).unwrap().clone();
                } else {
                    pc += 1;
                }
            }
            IRStatement::BranchZero { rs1, label } => {
                let rs1_val = *registers.get(rs1).unwrap();
                if rs1_val == 0 {
                    pc = label_locations.get(label).unwrap().clone();
                } else {
                    pc += 1;
                }
            }
            IRStatement::Label { label: _ } => {
                pc += 1;
            }
            IRStatement::Print { rs1 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                println!("{}", rs1_val);
                pc += 1;
            }
        }
    }
}
