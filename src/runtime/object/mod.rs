use std::{any::Any, cell::RefCell, collections::HashMap, ops::Add, rc::Rc};

pub struct HassiumObject {
    pub content: HassiumObjectContent,
    pub parent: HassiumObjectRef,
    pub attributes: Rc<RefCell<HashMap<String, HassiumObjectRef>>>,
}

impl HassiumObject {
    pub fn new(content: HassiumObjectContent, parent: HassiumObjectRef) -> HassiumObject {
        HassiumObject {
            content,
            parent,
            attributes: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

pub enum HassiumObjectContent {
    None,
    True,
    False,
    Anonymous,
    Number(f64),
    String(String),
    List(Vec<HassiumObjectRef>),
    Type(String),
}

static NEXT_HASSIUM_OBJECT_REF_ID: u64 = 0;
#[derive(Clone)]
pub struct HassiumObjectRef {
    id: u64,
}

impl HassiumObjectRef {
    pub fn new() -> HassiumObjectRef {
        HassiumObjectRef {
            id: NEXT_HASSIUM_OBJECT_REF_ID.add(1),
        }
    }

    pub fn set_attr(
        &self,
        ref_mapper: &mut ObjectRefMapper,
        name: String,
        value: HassiumObjectRef,
    ) {
        ref_mapper
            .get(self)
            .attributes
            .borrow_mut()
            .insert(name, value);
    }
}

pub struct ObjectRefMapper {
    objects: HashMap<u64, HassiumObject>,
}

impl ObjectRefMapper {
    pub fn new() -> ObjectRefMapper {
        ObjectRefMapper {
            objects: HashMap::new(),
        }
    }

    pub fn new_ref(&mut self, obj: HassiumObject) -> HassiumObjectRef {
        let obj_ref: HassiumObjectRef = HassiumObjectRef::new();
        self.objects.insert(obj_ref.id, obj);
        obj_ref
    }

    pub fn set_ref(&mut self, obj_ref: HassiumObjectRef, obj: HassiumObject) {
        self.objects.insert(obj_ref.id, obj);
    }

    pub fn get(&self, obj_ref: &HassiumObjectRef) -> &HassiumObject {
        self.objects.get(&obj_ref.id).unwrap()
    }

    pub fn get_mut(&mut self, obj_ref: &HassiumObjectRef) -> &mut HassiumObject {
        self.objects.get_mut(&obj_ref.id).unwrap()
    }
}

pub struct DefaultTypes {
    pub object: HassiumObjectRef,
    pub type_: HassiumObjectRef,
    pub none: HassiumObjectRef,
    pub string: HassiumObjectRef,
    pub number: HassiumObjectRef,
    pub map: HashMap<String, HassiumObjectRef>,
}

impl DefaultTypes {
    pub fn new(ref_mapper: &mut ObjectRefMapper) -> DefaultTypes {
        let object: HassiumObjectRef = HassiumObjectRef::new();
        let type_: HassiumObjectRef = HassiumObjectRef::new();

        let none: HassiumObjectRef = ref_mapper.new_ref(HassiumObject::new(
            HassiumObjectContent::None,
            type_.clone(),
        ));
        let string: HassiumObjectRef = ref_mapper.new_ref(HassiumObject::new(
            HassiumObjectContent::Type(String::from("string")),
            type_.clone(),
        ));
        let number: HassiumObjectRef = ref_mapper.new_ref(HassiumObject::new(
            HassiumObjectContent::Type(String::from("number")),
            type_.clone(),
        ));

        let mut map: HashMap<String, HassiumObjectRef> = HashMap::new();
        map.insert(String::from("object"), object.clone());
        map.insert(String::from("type"), type_.clone());
        map.insert(String::from("none"), none.clone());
        map.insert(String::from("string"), string.clone());
        map.insert(String::from("number"), number.clone());

        DefaultTypes {
            object,
            type_,
            none,
            string,
            number,
            map,
        }
    }
}
