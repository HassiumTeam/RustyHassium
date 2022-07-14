use core::fmt;
use std::collections::HashMap;

use crate::parser::{BinOpType, UnaryOpType};
use crate::runtime::object::HassiumObject;

#[derive(Debug)]
pub enum VMInstruction {
    BinOp {
        op: BinOpType,
    },
    BuildClass {
        name: String,
        code_obj: CodeObj,
        does_extend: bool,
    },
    BuildFunc {
        name: String,
        code_obj: CodeObj,
        param_names: Vec<String>,
        has_return_type: bool,
    },
    Import,
    Invoke {
        arg_count: u32,
    },
    Iter,
    IterNext {
        jump_if_full: u32,
    },
    Jump {
        to: u32,
    },
    JumpIfFalse {
        to: u32,
    },
    LoadAttrib {
        attrib: String,
    },
    LoadId {
        id: String,
    },
    LoadNumber {
        value: f64,
    },
    LoadString {
        value: String,
    },
    LoadSubscript,
    Pop,
    Raise,
    Return,
    SelfRef,
    StoreAttrib {
        attrib: String,
    },
    StoreId {
        id: String,
    },
    StoreSubscript,
    Super {
        arg_count: u32,
    },
    UnaryOp {
        op: UnaryOpType,
    },
}

impl fmt::Display for VMInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct CodeObj {
    pub is_class: bool,
    pub instructions: Vec<VMInstruction>,
    pub labels: HashMap<u32, u32>,
}

impl CodeObj {
    pub fn new(is_class: bool) -> CodeObj {
        CodeObj {
            is_class,
            instructions: Vec::new(),
            labels: HashMap::new(),
        }
    }
}

pub struct StackFrame {}

pub struct VMContext {
    stack_frame: Vec<StackFrame>,
    pos: u32,
}

impl VMContext {
    pub fn new() -> VMContext {
        VMContext {
            stack_frame: Vec::new(),
            pos: 0,
        }
    }

    pub fn run(&mut self, code: &CodeObj) {
        let mut stack: Vec<HassiumObject> = Vec::new();

        while self.pos < code.instructions.len() as u32 {
            let inst: &VMInstruction = code.instructions.get(self.pos as usize).unwrap();
            println!("{}", inst);

            match inst {
                VMInstruction::BinOp { op } => todo!(),
                VMInstruction::BuildClass {
                    name,
                    code_obj,
                    does_extend,
                } => todo!(),
                VMInstruction::BuildFunc {
                    name,
                    code_obj,
                    param_names,
                    has_return_type,
                } => todo!(),
                VMInstruction::Import => todo!(),
                VMInstruction::Invoke { arg_count } => todo!(),
                VMInstruction::Iter => todo!(),
                VMInstruction::IterNext { jump_if_full } => todo!(),
                VMInstruction::Jump { to } => todo!(),
                VMInstruction::JumpIfFalse { to } => todo!(),
                VMInstruction::LoadAttrib { attrib } => todo!(),
                VMInstruction::LoadId { id } => todo!(),
                VMInstruction::LoadNumber { value } => todo!(),
                VMInstruction::LoadString { value } => todo!(),
                VMInstruction::LoadSubscript => todo!(),
                VMInstruction::Pop => {
                    stack.pop();
                }
                VMInstruction::Raise => todo!(),
                VMInstruction::Return => todo!(),
                VMInstruction::SelfRef => todo!(),
                VMInstruction::StoreAttrib { attrib } => todo!(),
                VMInstruction::StoreId { id } => todo!(),
                VMInstruction::StoreSubscript => todo!(),
                VMInstruction::Super { arg_count } => todo!(),
                VMInstruction::UnaryOp { op } => todo!(),
            }
        }
    }
}
