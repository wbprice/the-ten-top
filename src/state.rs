use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use crate::{
    common::Point,
    components::Patron,
    entities::{init_cupboard, init_patron, init_plate, init_register, init_stove, init_worker},
    resources::{Cookbook, GameState, Ingredients, SpriteResource},
};

use log::info;

pub struct MyState;

impl SimpleState for MyState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        let sprite_sheet_handle = load_sprite_sheet(world);
        // Register sprite sheet handle with the world
        world.insert(SpriteResource {
            sprite_sheet: sprite_sheet_handle.clone(),
        });
        world.insert(GameState::default());
        world.insert(Cookbook::new());

        init_patron(world, sprite_sheet_handle.clone(), Patron::generate());
        init_register(world, sprite_sheet_handle.clone());
        init_worker(world, sprite_sheet_handle.clone());
        init_cupboard(
            world,
            sprite_sheet_handle.clone(),
            Ingredients::HotDogBun,
            Point { x: 24.0, y: 120.0 },
        );
        init_cupboard(
            world,
            sprite_sheet_handle.clone(),
            Ingredients::HotDogWeiner,
            Point { x: 40.0, y: 120.0 },
        );
        init_stove(
            world,
            sprite_sheet_handle.clone(),
            Point { x: 60.0, y: 120.0 },
        );
        init_plate(
            world,
            sprite_sheet_handle.clone(),
            Point { x: 72.0, y: 120.0 },
        );
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/spritesheet.ron",         // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}
