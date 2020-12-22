# Entities, Components and Systems

Bevy supported version:
```
{{#include _index_code/Cargo.toml:10}}
```

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
{{#include _index_code/examples/boilerplate.rs}}
```

If you're looking for some useful functionality out of the box (windowing, graphics, input, a scheduler that automatically loops etc.), you'll want to add [`DefaultPlugins`](https://docs.rs/bevy/0.4.0/bevy/struct.DefaultPlugins.html) as well.

```rust
{{#include _index_code/examples/default_plugins.rs}}
```

By chaining methods together on the [`AppBuilder`](internals/app-builder.md) that we created with `App::build`, we can carefully build up our game's functionality in a modular fashion with the [Builder pattern](https://refactoring.guru/design-patterns/builder).

To give you a taste of the syntax for Bevy's ECS, here's a not-quite-minimal Hello World example: 

```rust
{{#include _index_code/examples/hello_world.rs}}
```

If you want to dive right into more realistic code-bases, check out the extensive [examples](https://github.com/bevyengine/bevy/tree/master/examples) section of the official repo, or the examples and tutorials found at [awesome-bevy](https://github.com/bevyengine/awesome-bevy#games).