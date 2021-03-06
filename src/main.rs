use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod common;
mod components;
mod entities;
mod resources;
mod state;
mod systems;

use crate::systems::{
    CupboardSystem, DestinationSystem, DishSystem, IngredientSystem, MoveFeelingSystem,
    MovePatronSystem, MoveThoughtBubbleSystem, PatronTaskSystem, PlateSystem, RegisterSystem,
    SimpleAnimationSystem, StoveSystem, WorkerSystem, WorkerTaskSystem,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(MovePatronSystem, "move_patron_system", &[])
        .with(PatronTaskSystem, "patron_task_system", &[])
        .with(
            SimpleAnimationSystem,
            "simple_animation_system",
            &["move_patron_system"],
        )
        .with(
            MoveThoughtBubbleSystem,
            "thought_bubble_system",
            &["move_patron_system"],
        )
        .with(
            MoveFeelingSystem,
            "move_food_system",
            &["thought_bubble_system"],
        )
        .with(RegisterSystem, "register_system", &["move_food_system"])
        .with(
            DestinationSystem,
            "destination_system",
            &["register_system"],
        )
        .with(DishSystem, "dish_system", &["destination_system"])
        .with(WorkerSystem, "worker_system", &["dish_system"])
        .with(WorkerTaskSystem, "task_system", &["worker_system"])
        .with(CupboardSystem, "cupboard_system", &["worker_system"])
        .with(IngredientSystem, "ingredient_system", &["worker_system"])
        .with(StoveSystem, "stove_system", &["worker_system"])
        .with(PlateSystem, "plate_system", &[]);

    let mut game = Application::new(resources, state::MyState, game_data)?;
    game.run();

    Ok(())
}
