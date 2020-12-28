#![allow(warnings)]
// In this example, we need many different Resources that use an u32 to store its data
use bevy::prelude::*;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

// Creating a type alias for u32
type Score = u32;

// Creating a simple tuple struct using the 'newtype' pattern
// You can do the exact same thing with an ordinary struct if you want the field name
struct FallingThreshold(u32);

// Adding the Deref trait makes these simple structures much more pleasant to work with
// You always want the inner data anyways
impl Deref for FallingThreshold {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FallingThreshold {
    fn deref_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

// Creating a marker struct to combine with our data as a tuple type
// You could easily reuse these as marker components as well
struct Friendly;
struct Hostile;

// Generic types can be a great solution for when you want to have similar copies of the same data
// and need to disambiguate
struct MaxUnits<T> {
    n: u32,
    // PhantomData allows us to embed T without using it
    // but holds no semantic value
    phantom: PhantomData<T>,
}

impl<T> MaxUnits<T> {
    // This is just for convenience to elide the strange phantom field
    fn new(n: u32) -> Self {
        MaxUnits::<T> {
            n,
            phantom: PhantomData,
        }
    }
}

fn main() {
    App::build()
        // Don't do this: it's really hard to debug and read
        // And also hard to extend with new behavior or traits
        .add_resource(1 as u32)
        // This overwrites our previous u32, since type aliasing doesn't create a new type
        .add_resource(2 as Score)
        // FallingThreshold is its own type, despite being used like a raw u32
        .add_resource(FallingThreshold(3))
        // (Friendly, u32) and (Hostile, u32) are unique types, disambiguating properly
        .add_resource((Friendly, 4))
        .add_resource((Hostile, 5))
        // Generic data types let us do the same thing, but in clearer fashion
        .add_resource(MaxUnits::<Friendly>::new(4))
        .add_resource(MaxUnits::<Hostile>::new(5))
        .run();
}
