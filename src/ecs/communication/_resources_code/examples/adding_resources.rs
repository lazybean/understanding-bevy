use bevy::prelude::*;

	struct InfantryStats{
		hp: f32,
		damage: f32,
		speed: f32,
	}

	struct PlayerResources{
		gold: usize,
		wood: usize,
	}

	struct InitialPosition(f32, f32);

	enum PlayerColor{
		Red,
		Blue,
		Pink
	}

	fn main() {
		App::build()
		.add_resource::<InfantryStats>()
		.init_resource(PlayerResources{gold: 1000, wood: 500})
		.init_resource(InitialPosition(0,0))
		.init_resource(PlayerColor::Pink)
		.run();
	}
