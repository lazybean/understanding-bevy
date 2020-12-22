# Resources

[Resources](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.Resource.html) are global singletons, accessed by their type, which can be used to store global state.
You might want to use resources for storing and configuring settings, handling a complex data structure like a player's inventory that doesn't fit naturally into the ECS model, or tracking game state like the player's score.

Structs, enums and tuple structs all work fine as resources: the only requirement is that your type be thread-safe: `'static` lifetime and `Send + Sync`.

## Creating Resources

Resources are added to your app (either directly to the [AppBuilder](../internals/app-builder.md), to a [plugin](../../organization/plugins.md) or to [commands](commands.md)) in two different ways:

1. [`.add_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.add_resource), which creates an uninitialized resource of the type specified in its type parameter.

2. [`insert_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.insert_resource) which creates a resource of the type of its argument, with an initial value of the method's argument.

Use `.add_resource` when you're not sure what data you need in the resource at the time of its creation, and use `.init_resource` when you have a good starting value.

Calling either method on a resource type that already exists (unless it's `Local` to another system) will overwrite any existing data.

Here's how you might add resources of various types for a mock RTS game:
```rust
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

	struct InitialPosition(f32, f32)

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
```

If you're frequently using `init_resource()` for a specific type (such as in `Local` resources), you should consider setting up `impl Default` or a `new` method for that type.

## Using Resources

When you define a system, you can include resources as one of your function parameter. Once it's being used as a system, Bevy's scheduler automatically looks for a  previously added resources with a matching type, and  

There are three different forms that you might encounter Resources in:

1. [`Res`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Res.html), for when you want an immutable smart-pointer to the underlying data.
   
2. [`ResMut`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.ResMut.html), for when you want a *mutable* smart-pointer instead.
   
3. [`Local`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Local.html), for when you want a smart-pointer to a resource that's contained within the system. These are mutable, automatically created when you first call the system and persist between calls of the system.

These resource smart pointers all `impl Deref`, ensuring that rather than needing to call `*my_resource` each time, you can usually implicitly skip the dereferencing with `my_resource`. 

As a result of the lookup-by-type behavior, you only ever want to have one resource of a given type in your app at once, to make sure you're not overwriting data by accident. Because `Local` resources are unique to the system, they don't follow this rule.

If you need distinct resources with the same underlying data types, there are a few patterns you can use to ensure they have a unique type:

```rust
	// In this example, we need many different Resources that use an f32 to store its data

	// Creating a type alias for f32
	let Score = f32;

	// Creating a simple tuple struct
	// You can do the exact same thing with an ordinary struct if you want the field name
	struct FallingThreshold(f32);
	
	// Adding the Deref trait makes these simple structures much more pleasant to work with
	// You always want the inner data anyways
	impl Deref for FallingThreshold{
		type Target = Timer;

		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}

	impl DerefMut for FallingThreshold{
		fn deref_mut(&mut self) -> &mut Timer {
			&mut self.0
		}
	}

	// Creating a marker struct to combine with our data as a tuple type
	// You could easily reuse these as marker components as well
	struct Friendly;
	struct Hostile

	fn main() {
		App::build()
		// Don't do this: it's really hard to debug and read
		// And also hard to extend with new behavior or traits
		.add_resource::<f32>()
		// This overwrites our previous f32, since type aliasing doesn't create a new type
		.add_resource::<Score>()
		// FallingThreshold is its own type, despite being used like a raw f32
		.add_resource::<FallingThreshold>()
		// (Friendly, f32) and (Hostile, f32) are unique types, disambiguating properly
		.add_resource::<(Friendly, f32)>()
		.add_resource::<(Hostile, f32)>()
		.run();
	}
```