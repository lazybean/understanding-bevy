# Entities and Components

The ECS architecture, at its heart, is about efficiently laying out data, and then accessing it in principled ways.
Drawing on the metaphors of databases, we can think of each **component** as representing a unique column of data, with a homogenous type, and each row determines which entity that component belongs to.

Every entity in Bevy has a special component called `Entity`, which serves as our primary key into the database, allowing us to fetch the appropriate component efficiently. In [archetypal](internals/archetypes.md) ECS, like Bevy, we can extend this metaphor further. An archetype is a collection of entities with a specific set of components: creating an efficient sub-table where every cell, regardless of its entity or component, has a valid entry.

When we want to read or modify the data stored in our components, we can use a [`Query`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Query.html): allowing us to extract the columns we care about. We can further filter within this `Query` by using a [`QueryFilter`](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.QueryFilter.html), allowing us to limit ourselves to only entities that have or lack a certain component, or have recently changed.

## Designing Components

In Bevy, you can make nearly anything a component: merely give it a unique type (see [*Ensuring Unique Resource Types*](resources.html#ensuring-unique-resource-types) for tips) add it your entities, and then query away. 
The only caveat is that it must fulfill the [`Component`](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.Component.html) trait: it must be `Send + Sync + 'static`, allowing us to safely pass components between threads.

As your codebase grows, you're going to want to `impl Default` (or create a `new` method) for most of your component types or component bundles. 
This helps ensure consistent behavior (rather than having to track down 17 different constants) and can make spawning new entities less onerous.

Because Bevy's [scheduler](timing/scheduling.md) allows us to automatically parallelize systems that don't have conflicting read/write demands on our components or systems, it's good practice to make your components small. 
You should continue to break down your components until you're reasonably sure that hypothetical future systems will never want to consume one of its fields without the context of at least one other piece. 
Even if they're commonly used together, you should consider using a component bundle of small components, rather than one large component.

This practice also makes it much easier to extend your systems to support new, possibly unplanned behavior. 
From a design perspective, components in ECS fill a very similar role to Rust's traits. 
Rather than needing to think about precisely which [types / archetypes] our [functions / systems] are running on, [traits / components] guarantee the presence of certain data, and the desirability of certain behavior.

This allows us to skip verifying that the entity that we're operating on has certain properties; instead constructing more expressive type signatures for your queries (corresponding to more exact trait bounds). 
This is harder to accidentally break, easier to read, and should permit more of your systems to run in parallel as you only request the exact data you'll be operating on.

### Marker Components

When translating a game design to an ECS paradigm, you'll often want to be able to easily toggle behavior for different entities based on one of their properties.
 The idiomatic way to do so is to create a **marker component**: a data-less [unit struct](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html) that can be used to filter the entities returned by your queries using `With` or `Without` (see below). Here's some guidance on when you might want to use them:

 - Marker components are great for controlling whether a behavior (or set of behaviors) occurs at all. for things like enabling collisions or handling buffs.
 - If you always want to perform a behavior, but with some variants, control it using data stored in your component (as a field or as an enum), rather than proliferating marker components.
 - Adding and removing components has a substantial performance cost in archetype-based ECS like Bevy. If your game is performance-limited, marker components may be the wrong tool for handling things like short-lived buffs.

### Option Components

Sometimes, you don't know the value of a component at the time of its creation (or it may sometimes cease to exist during the course of gameplay). 
This might commonly be observed for components that relate one entity to another by storing an `Entity` as part of their data: defining a parent-child relationship, designating a target or so on. 
A useful pattern here is to wrap your component type `T` in an `Option<T>`, allowing you to safely set its value to `None`.
This ensuring that it shows up in the appropriate queries, its archetype is stable (helping performance) and you can take advantage of Rust's enum matching tools to ensure that you're always handling the possibility of missing data appropriately.

### Component Bundles

In certain complex subsystems of our game, we may want large quantities of related data to be created and consumed at the same time.
Bevy offers **component bundles** for this purpose, allowing us to stick to small, modular components while providing some nice sugar to ensure that components aren't carelessly forgotten.

Bevy itself uses this pattern for [Sprites](../graphics/assets-sprites.md), [UI](../ui/_index.md), [Cameras](../graphics/cameras.md) and so on. Here's a quick demo of how you might work with bundles (both [static](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.Bundle.html) and [dynamic](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.DynamicBundle.html)):

```rust ```

## Spawning and Despawning Entities

In order for your nicely-architected components to do much of anything, you're going to want to create some entities with those components.
Pass in a tuple of components or a component bundle to `commands.spawn` and you'll get a new entity with those components:

```rust ```

Despawning entities is easy, so long as you know which entity to grab:

```rust ```

Because entity creation and deletion are wide-reaching operations that involve altering archetypes, they can only be done via [`Commands`](communication/commands.md) or in a thread-local system by modifying [World](). 
As a result, they will not take effect until the end of the current [stage](timing/stages.md).

This has the convenient effect of ensuring that the set of entities (and components) that exist are constant within any given stage. always stick around until the end of the current stage. 
If, for example, you want to target an enemy in one system then attack it in the next, you can guarantee that it won't have been despawned in the mean time.
When using causal chains of systems like this across stages though, be mindful of the possibility that the entity or component you're pointing to may no longer exist.

When you're creating many of the same archetype of entity at once, it's somewhat more efficient to [spawn them in a batch](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.spawn_batch), allowing Bevy to allocate memory a single time. To do so, you need to create an appropriate iterator:

```rust```

## Modifying Components After Creation

While changing the archetype of an entity has a performance cost, it can often be the clearest way to handle a change in an entity's behavior.
You can add and remove components to entities with `insert`, `remove`, `insert_one` and `remove_one` ([see the methods on `Commands`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert)):

```rust```

Like entity creation and deletion, modifying components can only be done via `Commands` or in a thread-local system and will not take effect until the end of the current stage.
