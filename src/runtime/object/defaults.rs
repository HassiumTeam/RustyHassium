use std::{borrow::Borrow, collections::HashMap};

use crate::runtime::object::ObjectId;
use crate::runtime::vm::VMContext;

use super::{HassiumObject, HassiumObjectContext};

pub fn get_defaults(vm: &mut VMContext) -> HashMap<String, ObjectId> {
    let mut ret: HashMap<String, ObjectId> = HashMap::new();
    ret.insert("println".to_string(), new_hassium_function(vm, println).id);
    return ret;
}

pub fn new_hassium_function(
    vm: &mut VMContext,
    func: fn(&mut VMContext, ObjectId, Vec<ObjectId>) -> ObjectId,
) -> HassiumObject {
    HassiumObject::new(vm, HassiumObjectContext::Function(func), None)
}

pub fn new_hassium_number(vm: &mut VMContext, value: f64) -> HassiumObject {
    let mut obj = HassiumObject::new(vm, HassiumObjectContext::Number(value), None);
    bind_common_attributes(vm, &mut obj);
    return obj;
}

pub fn new_hassium_string(vm: &mut VMContext, str: String) -> HassiumObject {
    let mut obj = HassiumObject::new(vm, HassiumObjectContext::String(str), None);
    bind_common_attributes(vm, &mut obj);
    return obj;
}

fn bind_common_attributes(vm: &mut VMContext, obj: &mut HassiumObject) {
    obj.attributes.insert(
        "toNumber".to_string(),
        new_hassium_function(vm, to_number).id,
    );
    obj.attributes.insert(
        "toString".to_string(),
        new_hassium_function(vm, to_string).id,
    );
}

// Global default methods
fn println(vm: &mut VMContext, _obj_id: ObjectId, args: Vec<ObjectId>) -> ObjectId {
    println!("Rust is not broken");
    for arg_id in args {
        let arg = vm.deref(arg_id);
        let hassium_str = match vm.deref(arg.getattr("to_string")).context {
            HassiumObjectContext::Function(func) => func(vm, arg.id, Vec::new()),
            _ => panic!(),
        };

        match &vm.deref(hassium_str).context {
            HassiumObjectContext::String(value) => println!("{}", value),
            _ => panic!(),
        }
    }

    ObjectId(0)
}

// Common instance methods on types
fn to_number(vm: &mut VMContext, obj_id: ObjectId, _args: Vec<ObjectId>) -> ObjectId {
    let obj = vm.deref(obj_id);
    match &obj.context {
        HassiumObjectContext::Function(_) => panic!(),
        HassiumObjectContext::None => panic!(),
        HassiumObjectContext::Number(value) => new_hassium_number(vm, *value).id,
        HassiumObjectContext::String(string) => new_hassium_number(vm, string.parse().unwrap()).id,
        HassiumObjectContext::Type(_) => panic!(),
    }
}

fn to_string(vm: &mut VMContext, obj_id: ObjectId, _args: Vec<ObjectId>) -> ObjectId {
    let obj = vm.deref(obj_id);
    match &obj.context {
        HassiumObjectContext::Function(_) => new_hassium_string(vm, "function()".to_string()).id,
        HassiumObjectContext::None => new_hassium_string(vm, "None".to_string()).id,
        HassiumObjectContext::Number(value) => new_hassium_string(vm, value.to_string()).id,
        HassiumObjectContext::String(string) => new_hassium_string(vm, string.clone()).id,
        HassiumObjectContext::Type(name) => new_hassium_string(vm, name.clone()).id,
    }
}
