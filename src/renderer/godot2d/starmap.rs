use gdnative::*;

use std::rc::Rc;


use super::planet::Planet;
use crate::local::starmap::*;
use crate::local::model::*;
use crate::local::starmap::builder::StarmapBuilder;


pub struct Starmap2D {   
    planets: Vec<Rc<Node2D>>
}

impl Starmap for Starmap2D {
    type CelestialType = Node2D;

    fn get_planets(&self) -> &Vec<Rc<Node2D>> {
        &self.planets
    }
    
    unsafe fn get_planet_properties(&self, planet_id: usize) -> CelestialProperties {
        let planet_node = **self.planets.get(planet_id).unwrap();
        Planet::with(planet_node, |planet| {
            planet.properties()
        })
    }

    fn set_planets(&mut self, planets: Vec<Rc<Node2D>>) {
        self.planets = planets;
    }

    fn new<F, G, H>(count: usize) -> StarmapBuilder<Node2D, Starmap2D, F, G, H> 
    where 
        F: FnMut(usize) -> Node2D,
        G: Fn(&Node2D, &Node2D) -> bool,
        H: Fn(&Node2D) -> (),
        Self: Sized {
        StarmapBuilder::new(count, Starmap2D {
            planets: vec!()
        })
    }

    unsafe fn destroy(&self) -> () {
        self.planets.iter()
            .for_each(|p| p.free());
    }

    unsafe fn get_distance_between(planet1: &Node2D, planet2: &Node2D) -> f32 {
        let p1pos = Point2::new(planet1.get_position().x, planet1.get_position().y);
        let p2pos = Point2::new(planet2.get_position().x, planet2.get_position().y);
        p1pos.distance_to(p2pos)
    }
}