pub mod defaults;

use core::fmt;
use std::{collections::HashMap, ops::Add, rc::Rc};

use super::vm::VMContext;

static mut HASSIUM_OBJECT_ID: usize = 0;
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectId(usize);

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub enum HassiumObjectContext {
    Function(fn(&mut VMContext, ObjectId, args: Vec<ObjectId>) -> ObjectId),
    None,
    Number(f64),
    String(String),
    Type(String),
}

#[derive(Clone)]
pub struct HassiumObject {
    pub id: ObjectId,
    pub context: HassiumObjectContext,
    pub self_ref: Box<Option<HassiumObject>>,
    pub attributes: HashMap<String, ObjectId>,
}

unsafe impl Sync for HassiumObject {}

impl HassiumObject {
    pub fn new(
        vm: &mut VMContext,
        context: HassiumObjectContext,
        self_ref: Option<HassiumObject>,
    ) -> HassiumObject {
        unsafe {
            let id = ObjectId(HASSIUM_OBJECT_ID);
            let ret = HassiumObject {
                id,
                context,
                attributes: HashMap::new(),
                self_ref: Box::new(self_ref),
            };
            HASSIUM_OBJECT_ID = HASSIUM_OBJECT_ID.add(1);
            println!("Inserting with id {}", id);
            vm.all_objects.insert(id, Rc::new(Box::new(ret.clone())));
            return ret;
        }
    }

    pub fn getattr(&self, name: &str) -> ObjectId {
        *self.attributes.get(name).unwrap()
    }
}

impl ObjectId {
    pub fn invoke(&self, vm: &mut VMContext, args: Vec<ObjectId>) -> ObjectId {
        match vm.deref(*self).context {
            HassiumObjectContext::Function(func) => func(vm, *self, args),
            _ => panic!(""),
        }
    }
}
