use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Resources {
    objects: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add<T: 'static + Send + Sync>(&mut self, resource: T) {
        let type_id = TypeId::of::<T>();
        self.objects.insert(type_id, Box::new(resource));
    }

    pub fn get<T: 'static>(&self) -> &T {
        let type_id = TypeId::of::<T>();

        self.objects
            .get(&type_id)
            .and_then(|boxed_resource| {
                let r: &dyn Any = boxed_resource.as_ref();
                r.downcast_ref::<T>()
            })
            .unwrap_or_else(|| {
                panic!(
                    "Failed to get resource of type {}",
                    std::any::type_name::<T>()
                )
            })
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}
