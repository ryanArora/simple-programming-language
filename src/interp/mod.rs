use std::collections::HashMap;

use crate::ir::{IRStatement, Label, Register, IR};

pub fn interpret(ir: &IR) -> u32 {
    let mut registers: HashMap<Register, u32> = HashMap::new();
    let mut label_locations: HashMap<Label, usize> = HashMap::new();

    for (i, statement) in ir.statements.iter().enumerate() {
        if let IRStatement::Label { label } = statement {
            label_locations.insert(label.clone(), i);
        }
    }

    let mut pc = 0;
    let mut last_rd_val: u32 = 0;

    while pc < ir.statements.len() {
        match &ir.statements[pc] {
            IRStatement::LoadImmediate { rd, imm } => {
                let rd_val = *imm;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::Add { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val + rs2_val;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::Subtract { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val - rs2_val;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::Multiply { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val * rs2_val;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::Divide { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val / rs2_val;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::Modulus { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val % rs2_val;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::Exponentiate { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val.pow(rs2_val as u32);
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::Equal { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = if rs1_val == rs2_val { 1 } else { 0 };
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::NotEqual { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = if rs1_val != rs2_val { 1 } else { 0 };
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::GreaterEqual { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = if rs1_val >= rs2_val { 1 } else { 0 };
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::LessEqual { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = if rs1_val <= rs2_val { 1 } else { 0 };
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::Greater { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = if rs1_val > rs2_val { 1 } else { 0 };
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::Less { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = if rs1_val < rs2_val { 1 } else { 0 };
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::LogicalAnd { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = if rs1_val != 0 && rs2_val != 0 { 1 } else { 0 };
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::LogicalOr { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = if rs1_val != 0 || rs2_val != 0 { 1 } else { 0 };
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::LogicalNot { rd, rs1 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rd_val = if rs1_val == 0 { 1 } else { 0 };
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::BitwiseAnd { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val & rs2_val;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::BitwiseOr { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val | rs2_val;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::BitwiseXor { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val ^ rs2_val;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::LeftShift { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val << rs2_val;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::RightShift { rd, rs1, rs2 } => {
                let rs1_val = *registers.get(rs1).unwrap();
                let rs2_val = *registers.get(rs2).unwrap();
                let rd_val = rs1_val >> rs2_val;
                registers.insert(rd.clone(), rd_val);
                pc += 1;
                last_rd_val = rd_val;
            }
            IRStatement::Branch { label } => {
                pc = label_locations[label];
            }
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

    last_rd_val
}
