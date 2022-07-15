pub mod movable;
pub mod ezshape;
pub mod collision;

// ECS itself:

use std::cell::{RefCell, RefMut};

use crate::ecs::movable::Movable;


pub trait Component{
    fn start(&mut self, ecs: &mut ECS, entity_id: usize){}
    fn update(&mut self, ecs: &mut ECS, entity_id: usize, dt: f32){}
}

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
    fn update_all(&mut self, ecs: &mut ECS, dt: f32);
}


pub struct ECS {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    pub fn add_component_to_entity<ComponentType: 'static + Component>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<Box<ComponentType>>>>>()
            {
                component_vec.get_mut()[entity] = Some(Box::new(component));
                return;
            }
        }

        let mut new_component_vec: Vec<Option<Box<ComponentType>>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(Box::new(component));

        self.component_vecs
            .push(Box::new(new_component_vec));
    }

    pub fn borrow_component_vec<ComponentType: 'static + Component>(
        &mut self,
    ) -> Option<&mut Vec<Option<Box<ComponentType>>>> {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<Box<ComponentType>>>>()
            {
                return Some(component_vec);
            }
        }
        None
    }

    pub fn borrow_component<ComponentType: 'static + Component>(
        &mut self,
        entity_id: usize,
    ) -> Option<&mut Box<ComponentType>> {
        if entity_id < 0 || entity_id >= self.entities_count {
            return None;
        }
        if let Some(mut component_vec) = self.borrow_component_vec::<ComponentType>(){
            return component_vec[entity_id].as_mut();
        }
        None
    }

    pub fn update_all(&mut self, dt: f32){
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.update_all(self, dt);
        }
    }
}

impl<T: 'static + Component> ComponentVec for Vec<Option<Box<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None)
    }

    fn update_all(&mut self, ecs: &mut ECS, dt: f32) {
        let mut id: usize = 0;
        for component in self{
            if let Some(component) = component {
                component.update(ecs, id, dt);
            }
            id += 1;
        }
    }
}