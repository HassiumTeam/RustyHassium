use std::collections::HashMap;

use crate::parser::{BinOpType, UnaryOpType};

pub enum VMInstruction {
    BinOp {
        op: BinOpType,
    },
    Break,
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
    Call {
        arg_count: u32,
    },
    Continue,
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
