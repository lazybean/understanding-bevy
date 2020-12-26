// In this example, we need many different Resources that use an f32 to store its data
use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

// Creating a type alias for f32
type Score = f32;

// Creating a simple tuple struct
// You can do the exact same thing with an ordinary struct if you want the field name
struct FallingThreshold(f32);

// Adding the Deref trait makes these simple structures much more pleasant to work with
// You always want the inner data anyways
impl Deref for FallingThreshold {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FallingThreshold {
    fn deref_mut(&mut self) -> &mut f32 {
        &mut self.0
    }
}

// Creating a marker struct to combine with our data as a tuple type
// You could easily reuse these as marker components as well
struct Friendly;
struct Hostile;

fn main() {
    App::build()
        // Don't do this: it's really hard to debug and read
        // And also hard to extend with new behavior or traits
        .add_resource(1.0 as f32)
        // This overwrites our previous f32, since type aliasing doesn't create a new type
        .add_resource(2.0 as Score)
        // FallingThreshold is its own type, despite being used like a raw f32
        .add_resource(FallingThreshold(3.0))
        // (Friendly, f32) and (Hostile, f32) are unique types, disambiguating properly
        .add_resource((Friendly, 4.0))
        .add_resource((Hostile, 5.0))
        .run();
}
