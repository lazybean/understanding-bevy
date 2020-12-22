# Resources

[Resources](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.Resource.html) are global singletons, accessed by their type, which can be used to store global state.
You might want to use resources for storing and configuring settings, handling a complex data structure like a player's inventory that doesn't fit naturally into the ECS model, or tracking game state like the player's score.

You can use virtually any Rust type as a resource, but if possible, you're going to want your resources to be thread-safe: `'static` lifetime and `Send + Sync`.

## Creating Resources

Assuming that we're working with a thread-safe resource that isn't system local, there are two different ways we can add resources to our app.

1. [`init_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.init_resource), which registers the resource type specified in its type parameter.

2. [`.add_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.add_resource), which also sets a starting value for that type.

Use `.init_resource` when you're not sure what data you need in the resource at the time of its creation, and use `.add_resource` when you have a good starting value.

You can use these methods:
    1. Directly on the [AppBuilder](../internals/app-builder.md).
    2. Through [commands](commands.md), which don't take effect until the end of the current stage. This allows you to add or overwrite Resources at runtime, although in most cases you shouldn't need to. If you just need to modify a resource, instead create a system that gets a `ResMut` to the resource in question, then modify it there.
    3. As part of a [plugin](../../organization/plugins.md), to keep your code well-organized, which you then add to the app builder. These are constructed using commands as well, so they also have delayed effect.

Despite the fact that `.init_resource` and `.add_resource` are `AppBuilder` methods, not `Commands` methods, we can still use them on our `commands` object that's accessed by our plugins and systems. Commands accumulates changes, then applies them to the `AppBuilder` at the end of the current stage.

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

If you need to use a not thread safe resource, here's an example of how you might set and access it:

```rust

```

### Thread-local Resources

If you need a resource that is not thread-safe, you first need to create it with: [`.add_thread_local_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.add_thread_local_resource) or [`.init_thread_local_resource`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert_local_resource), whose behavior corresponds to the `add_resource` and `init_resource` methods described above. 

Be aware: thread-local resources created in this way are a completely distinct concept from those created with the `.insert_local_resource` method, which use the `Local` resource smart pointer, which creates a unique instantiation of the resource in the system it is referred to.

Once you have your thread-local resource, you need to use "thread-local systems" (see the corresponding [section](../systems.md) in this book for more information) to manipulate it, which gives you a complete global lock on the entire [app](https://docs.rs/bevy/0.4.0/bevy/app/struct.App.html), with `World` and all of its `Resources`.

Here's an example in action:
```rust

```

### System-Local Resources

System-local resources are mutable, scoped resources that are only available in the system that created them. Their state persists between time steps, but not between distinct systems created using the same function, as they work off of the `SystemId` created at their time of registration.

In typical use, system-local resources are created implicitly, through the use of a `Local` resource smart-pointer type on one of the function arguments in your system. If you had some reason to manually create or overwrite them, you could instead use [`.insert_local_resource`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert_local_resource).


Here's an example showing you how and why you might want to use system-local resources:
```rust

```

## Ensuring Unique Resource Types

When any of the resource creation methods is called on a type that already exists (with the caveat that system-local resources are effectively scoped), Bevy will overwrite any existing data. As a result, you only ever want to have one resource of a given type in your app at once.

Here are a few patterns you can use to ensure that your resources have a unique type:

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

## Using Resources

In order to access resources in a system, wrap the resource type in your function parameters in one of the three smart-pointers.

1. [`Res`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Res.html), for when you want read-only access to the underlying data.
   
2. [`ResMut`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.ResMut.html), for when you want read and write access to the data.
   
3. [`Local`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Local.html), for when you want a system-local resource.

These resource smart pointers all `impl Deref`, ensuring that rather than needing to call `*my_resource` each time, you can usually implicitly skip the dereferencing with `my_resource`. 

When you define a system, you can include resources as one of your function parameter. Bevy's scheduler automatically looks for a  previously added resources with a matching type, and passes in a reference of the appropriate type to your system.

We can see the differences between these different resource types in this simple example:

```rust

```
