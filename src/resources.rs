use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Resources {
    objects: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add<T: 'static>(&mut self, resource: T) {
        let type_id = TypeId::of::<T>();
        self.objects.insert(type_id, Box::new(resource));
    }

    pub fn get<T: 'static>(&self) -> &T {
        let type_id = TypeId::of::<T>();

        self.objects
            .get(&type_id)
            .and_then(|boxed_resource| boxed_resource.downcast_ref::<T>())
            .expect(&format!(
                "Failed to get resource of type {}",
                std::any::type_name::<T>()
            ))
    }
}
