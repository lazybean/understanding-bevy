# Entities, Components and Systems

At its heart, Bevy operates using an [Entity-Component-System](https://en.wikipedia.org/wiki/Entity_component_system) model. Rather than operating in a traditional object-oriented fashion, where your game is composed of individual objects with properties and methods, ECS games work in a fashion that is much closer to a relational database: grouping similar data together in a memory-dense fashion, with the `Entity` serving as a primary key to identify the data, then processing the game data in a batch-wise fashion.

Games in Bevy are largely composed of five core building blocks, that come up again and again:
- **Entity:** [Entities](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Entity.html) are simple, unique identifiers, and nothing more. Given a component, the entity signifies which in-game object the stored data belongs to, thus "grouping" different components together.
- **Components:** [Components](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.Component.html) refer to data that is associated with an entity. You can use any thread-safe Rust type as a component, and they are stored in a performant fashion using [archetypes](internals/archetypes.md).
- **Systems:** [Systems](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.System.html) are largely-ordinary Rust functions that handle the game logic. They access components through [queries](../../book/ecs/queries.html) and can directly read and write resources (including [Commands](communication/commands.md) and [Events](communication/events.md)). When added to your game, they automatically fetch data of the appropriate type, and are run in parallel by the [scheduler](timing/scheduling.md) based on the resources and components that they read and write.
- **Resources:** [Resources](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.Resource.html) are well-behaved global singletons that are used to store cross-system game state that isn't associated with a particular entity. Like with components, you can use any thread-safe Rust type as a Resource. 
- **Assets:** [Assets](https://docs.rs/bevy/0.4.0/bevy/asset/struct.Assets.html) are blocks of data that are used to create content for your game. Game models, raster graphics for your UI, sound effects and so on are all assets. Unlike resources, these are saved on the disc, rather than created by the engine.

## Getting Started

To get started, [Install Rust](https://www.rust-lang.org/tools/install), then set up a new Bevy project ("Add Bevy as a Dependency") as described in the [official book](https://bevyengine.org/learn/book/getting-started/).

The boilerplate needed to make your own app is simple:

```rust
use bevy::prelude::*;

fn main() {
    App::build().run();
}
```

If you're looking for some useful functionality out of the box (windowing, graphics, input, a scheduler that automatically loops etc.), you'll want to add [`DefaultPlugins`](https://docs.rs/bevy/0.4.0/bevy/struct.DefaultPlugins.html) as well.

```rust
use bevy::prelude::*;

fn main() {
	App::build()
        .add_plugins(DefaultPlugins)
		.run();
}
```

By chaining methods together on the [`AppBuilder`](internals/app-builder.md) that we created with `App::build`, we can carefully build up our game's functionality in a modular fashion with the [Builder pattern](https://refactoring.guru/design-patterns/builder).

To give you a taste of the syntax for Bevy's ECS, here's a not-quite-minimal Hello World example: 

```rust
use bevy::prelude::*;
use std::collections::HashMap;
use std::fmt;

// Denizen is a "marker component" here, to allow us to filter for denizens in our queries
// In general, marker components do not store data, and are just used for queries
// Marker components are talked about in more detail in section 2.2: Components
#[derive(Debug)]
struct Denizen;

// We use the owned form String, rather than &str in this struct 
// Because resources and components must be thread-safe with a 'static lifetime
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Name(String);

// A custom impl of Display to ensure we can print these names nicely
impl fmt::Display for Name{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
enum World{
	Venus,
	Earth,
	Mars
}

// Creating a unique timer type for each system that needs one allows us to be sure we're grabbing the right one
struct HelloTimer(Timer);

fn main() {
	App::build()
	// We add the default plugins so our game will loop
	// This also adds the Time resource that we use in `say_hello`
	.add_plugins(DefaultPlugins)
	// We're creating a HashMap to record which planet entities a name are on
	.add_resource(HashMap::<Name, World>::new())
	// Startup systems only run once, before normal systems take place
	.add_startup_system(place_denizens.system())
	// When systems cannot be run in parallel, priority is based on insertion order
	.add_startup_system(spawn_denizens.system())
	// The `say_hello` system will be run every frame, but only prints when the timer is complete
	// (See `say_hello` logic)
	.add_resource(HelloTimer(Timer::from_seconds(2.0, true)))
	.add_system(say_hello.system())
	.run();
}

// Because we're modifying the `directory` argument, we need to get the mutable version of it with `ResMut`
// Bevy's ECS finds a Resource with the matching type; we want to be sure we have exactly one resource of each type that we need
fn place_denizens(mut directory: ResMut<HashMap::<Name, World>>){
	// .into() converts our string literal from &str to the required String
	directory.insert(Name("Alice".into()), World::Venus);
	directory.insert(Name("Bevy".into()), World::Earth);
	directory.insert(Name("Cart".into()), World::Mars);
}

// The special `Commands` resource queues up actions that should be performed to modify the World
// We only need to read from the directory resource, so we can call it with `Res` instead of `ResMut`
fn spawn_denizens(commands: &mut Commands, directory: Res<HashMap::<Name, World>>){
	// We need to use .clone and .into_iter rather than .iter here 
	// to satisfy the lifetime requirements of .spawn()
	for (name, world) in directory.clone().into_iter(){
		if name == Name("Bevy".into()){
			// .spawn creates new entities with the specified components
			// each tuple entry is a different component
			commands.spawn((name, world));
		} else {
			// Only Alice and Cart are Denizens, so Bevy will be excluded from our query filter in `say_hello`
			commands.spawn((name, world, Denizen));
		}
	}
}

// Queries extract each entity that have all of the components specified in their first type argument
// They only return the components specified in the query, not any other components that may be associated with the entities
// The second type argument is a query filter, which restricts which entities are actually provided 
fn say_hello(query: Query<(&Name, &World), With<Denizen>>, mut timer: ResMut<HelloTimer>, time: Res<Time>){
	// Only run this system when the timer has elapsed
	if timer.0.tick(time.delta_seconds()).just_finished(){
		// Iterating over and then unpacking the query gives us access to the components for each of its entities
		for (name, world) in query.iter(){
			// Because we're querying for &Query and &World, we need to dereference them before we work with them
			println!("Hello {:?}, my name is {}!", *world, *name);
		}
	}
}
```

If you want to dive right into more realistic code-bases, check out the extensive [examples](https://github.com/bevyengine/bevy/tree/master/examples) section of the official repo, or the examples and tutorials found at [awesome-bevy](https://github.com/bevyengine/awesome-bevy#games).