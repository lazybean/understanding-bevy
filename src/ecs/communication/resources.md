# Resources

[Resources](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.Resource.html) are global singletons, accessed by their type, which can be used to store global state.
You might want to use resources for storing and configuring settings, handling a complex data structure like a player's inventory that doesn't fit naturally into the ECS model, or tracking game state like the player's score.

You can use virtually any Rust type as a resource, but if possible, you're going to want your resources to be thread-safe: `'static` lifetime and `Send + Sync`.

## Creating Resources

Assuming that we're working with a thread-safe resource that isn't system local, there are two different ways we can add resources to our app.

When you're working with your [`AppBuilder`](../internals/app-builder.md) (including through a [plugin](../../organization/plugins.md)), there are two ways to add resources:

1. [`init_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.init_resource), which adds a resource of the type specified in its type parameter, with a starting value given by its `Default` trait.

2. [`.add_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.add_resource), which sets a custom starting value for that type.

Use `.init_resource` when you're not sure what data you need in the resource at the time of its creation, and use `.add_resource` when you have a good starting value.

If you need to add or overwrite Resources at run-time, consider using [commands](commands.md) with the [.insert_resource](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert_resource) method, which works the same as `.add_resource` above. Be mindful though: commands don't take effect until the end of each stage. Most of the time, you shouldn't need to do this: to modify a resource, instead create a system that gets a `ResMut` to the resource in question, then modify it within that system.

Here's how you might add resources of various types for a mock RTS game:
```rust
{{#include _resources_code/examples/adding_resources.rs}}
```

### Thread-local Resources

If you need a resource that is not thread-safe, you first need to create it with: [`.add_thread_local_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.add_thread_local_resource) or [`.init_thread_local_resource`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert_local_resource), whose behavior corresponds to the `add_resource` and `init_resource` methods described above. 

Be aware: thread-local resources created in this way are a completely distinct concept from those created with the `.insert_local_resource` method, which use the `Local` resource smart pointer, which creates a unique instantiation of the resource in the system it is referred to.

Once you have your thread-local resource, you need to use "thread-local systems" (see the corresponding [section](../systems.md) in this book for more information) to manipulate it, which gives you a complete global lock on the entire [app](https://docs.rs/bevy/0.4.0/bevy/app/struct.App.html), with `World` and all of its `Resources`.

Here's how you might set and access such a resource:
```rust
{{#include _resources_code/examples/thread_local_resources.rs}}
```

### System-Local Resources

System-local resources are scoped, mutable resources that are only available in the system that created them. Their state persists between time steps, but not between distinct systems created using the same function, as they work off of the `SystemId` created at their time of registration.

In typical use, system-local resources are created implicitly, through the use of a `Local` resource smart-pointer type on one of the function arguments in your system. If you had some reason to manually create or overwrite them, you could instead use [`.insert_local_resource`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert_local_resource).


Here's an example showing the power of system-local resources when combined with generic systems:
```rust
{{#include _resources_code/examples/system_local_resources.rs}}
```

## Ensuring Unique Resource Types

When any of the resource creation methods is called on a type that already exists (with the caveat that system-local resources are effectively scoped), Bevy will overwrite any existing data. As a result, you only ever want to have one resource of a given type in your app at once.

Here are a few patterns you can use to ensure that your resources have a unique type:

```rust
{{#include _resources_code/examples/unique_resource_types.rs}}
```

## Using Resources in Your Systems

In order to access resources in a system, wrap the resource type in your function parameters in one of the three smart-pointers.

1. [`Res`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Res.html), for when you want read-only access to the underlying data.
   
2. [`ResMut`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.ResMut.html), for when you want read and write access to the data.
   
3. [`Local`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Local.html), for when you want a system-local resource.

These resource smart pointers all `impl Deref`, ensuring that rather than needing to call `*my_resource` each time, you can usually implicitly skip the dereferencing with `my_resource`. 

When you define a system, you can include resources as one of your function parameter. Bevy's scheduler automatically looks for a  previously added resources with a matching type, and passes in a reference of the appropriate type to your system.

We can see the differences between these different resource types in this simple example:

```rust
{{#include _resources_code/examples/resource_smart_pointers.rs}}
```
