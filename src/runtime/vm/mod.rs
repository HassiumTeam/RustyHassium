use core::fmt;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

use crate::parser::{BinOpType, UnaryOpType};
use crate::runtime::object::defaults::{new_hassium_number, new_hassium_string};
use crate::runtime::object::{HassiumObject, HassiumObjectContext};

use super::object::defaults::get_defaults;
use super::object::ObjectId;

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

#[derive(Clone)]
pub struct VMContext {
    pub all_objects: HashMap<ObjectId, Rc<Box<HassiumObject>>>,
    stack_frame: Vec<HashMap<String, ObjectId>>,
    pos: u32,
}

impl VMContext {
    pub fn new() -> VMContext {
        let mut ret = VMContext {
            all_objects: HashMap::new(),
            stack_frame: Vec::new(),
            pos: 0,
        };
        ret.stack_frame.push(get_defaults(&mut ret.clone()));

        return ret;
    }

    pub fn deref(&self, id: ObjectId) -> &Rc<Box<HassiumObject>> {
        self.all_objects.get(&id).unwrap()
    }

    pub fn run(&mut self, code: &CodeObj) {
        let mut stack: Vec<ObjectId> = Vec::new();

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
                VMInstruction::Invoke { arg_count } => {
                    let obj = stack.pop().unwrap();
                    let mut args: Vec<ObjectId> = Vec::new();
                    for _i in 0..*arg_count {
                        args.push(stack.pop().unwrap());
                    }
                    args.reverse();
                    stack.push(obj.invoke(self, args))
                }
                VMInstruction::Iter => todo!(),
                VMInstruction::IterNext { jump_if_full } => todo!(),
                VMInstruction::Jump { to } => self.pos = *to,
                VMInstruction::JumpIfFalse { to } => todo!(),
                VMInstruction::LoadAttrib { attrib } => todo!(),
                VMInstruction::LoadId { id } => {
                    for frame in &self.stack_frame {
                        let option = frame.get(id);
                        if option.is_some() {
                            stack.push(*option.unwrap());
                            break;
                        }
                    }
                    // panic!("ID {} could not be resolved!", id)
                }
                VMInstruction::LoadNumber { value } => {
                    let id = new_hassium_number(self, *value).id;
                    stack.push(self.all_objects.get(&id).unwrap().id);
                }
                VMInstruction::LoadString { value } => {
                    let id = &new_hassium_string(self, value.to_string()).id;
                    stack.push(self.all_objects.get(id).unwrap().id);
                }
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

            self.pos += 1;
        }
    }
}
