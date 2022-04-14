// #[macro_use]
// extern crate gdnative;

use gdnative::{prelude::*, api::TileMap};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

#[methods]
impl HelloWorld {
    fn new(_owner: &Node) -> Self{
        HelloWorld
    }

    #[export]
    fn _ready(&self, _owner: &Node){
        godot_print!("Something");
    }
}

#[derive(NativeClass)]
#[inherit(gdnative::api::TileMap)]
pub struct Map;

#[methods]
impl Map{
    fn new(_owner: &TileMap)->Self{
        Map
    }
}


fn init(handle: InitHandle){
    handle.add_class::<HelloWorld>();
    handle.add_class::<Map>();
}




godot_init!(init);