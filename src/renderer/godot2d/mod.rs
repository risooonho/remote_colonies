pub mod planet;
pub mod ship;
mod input;
mod player;
mod starmap;

use gdnative::*;

use planet::*;

use std::cell::*;
use std::rc::Rc;

use crate::local::starmap::*;
use crate::local::player::*;
use crate::local::GameState;
use crate::local::model::*;
use self::input::InputHandler2D;
use self::starmap::Starmap2D;
use self::player::Player2D;

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassInvalid(String),
}

#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::LocalCellData<Main>)]
pub struct Main {
    #[property]
    planet: PackedScene,

    game_state: Rc<RefCell<GameState<Starmap2D, Player2D>>>
}

#[methods]
impl Main {
    
    fn _init(_owner: Node) -> Self {
        Main {
            planet: PackedScene::new(),
            game_state: Rc::new(RefCell::new(GameState::new())),
        }
    }
    
    #[export]
    unsafe fn _ready(&mut self, mut owner: Node) {
        let input_handler = Rc::new(RefCell::new(InputHandler2D::new()));
        let mut starmap = Starmap2D::new(10)
        .with_generator(|id| {
            let planet_node: Node2D = instance_scene(&self.planet).unwrap();
            owner.add_child(Some(planet_node.to_node()), false);

            Planet::with_mut(planet_node, |planet| {
                planet.set_game_state(self.game_state.clone());
                planet.set_random_features();
                planet.set_id(id);
                planet.set_input_handler(input_handler.clone(), |planet, player_action| {
                    let game_state = planet.get_game_state();
                    match player_action {
                        PlayerAction::AddShip(_) => planet.add_ship(Consts::ADD_SHIP_RESOURCE_COST, game_state.get_current_player()),
                        PlayerAction::MoveShips(from, to) => {
                            let planets = game_state.get_starmap().get_planets();
                            let planet_from = Planet::get_by_id(planets, from.id);
                            let planet_to = Planet::get_by_id(planets, to.id);

                            Planet::with(**planet_from, |planet| {
                                planet.move_ships(Consts::MOVE_SHIP_FLEET_PERCENT, game_state.get_current_player(), planet_to);
                            });
                        },
                        _ => ()
                    }
                });
            });

            planet_node
        })
        .with_validator(|planet1, planet2| {
            let distance = Starmap2D::get_distance_between(planet1, planet2);
            distance > 100.0 && distance < 1000.0
        })
        .with_cleaner(|planet| planet.free())
        .build();

        starmap.get_planets_by_max_distance(2).iter()
        .map(|planet_node| **planet_node)
        .enumerate()
        .for_each(|(index, planet_node)| {
            Planet::with_mut(planet_node, |planet| {
                planet.set_resources(Consts::ADD_PLAYER_RESOURCES_INIT, Consts::ADD_PLAYER_RESOURCES_INC);
                planet.add_player(index > 0);
            });
        });
        
        let mut game_state = self.game_state.borrow_mut();
        game_state.set_starmap(starmap);
    }

    #[export]
    pub unsafe fn _process(&self, _owner: Node, _delta: f64) {
        self.game_state.borrow_mut().update_ai();
    }
}

pub unsafe fn instance_scene<Root>(scene: &PackedScene) -> Result<Root, ManageErrs>
where
    Root: gdnative::GodotObject,
{
    let inst_option = scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED);

    if let Some(instance) = inst_option {
        if let Some(instance_root) = instance.cast::<Root>() {
            Ok(instance_root)
        } else {
            Err(ManageErrs::RootClassInvalid(
                instance.get_name().to_string(),
            ))
        }
    } else {
        Err(ManageErrs::CouldNotMakeInstance)
    }
}