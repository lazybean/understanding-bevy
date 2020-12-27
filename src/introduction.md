# Introduction

[Bevy](https://bevyengine.org/) is a fantastic, free, open source, ECS-style game engine written in Rust. This book is [my](https://github.com/alice-i-cecile) attempt to collect, summarize and contextualize the engine and its ecosystem. This book was written for Bevy 0.4, and assumes familiarity with core Rust concepts (like lifetimes, traits, objects and iterators), but no previous experience with either Bevy or game programming as a whole. If you have any suggestions, questions, or spot an error, please feel free to open an issue on this site's [Github](https://github.com/alice-i-cecile/understanding-bevy.github.io).

This book is deliberately opinionated: rather than simply reiterating the API that Bevy offers you, it attempts to provide context on the options available and when you should reach for a particular tool. 
Ideally, this book will teach how you should write elegant, performant and idiomatic game code in Bevy + Rust and Bevy and steer you away from obvious pitfalls that may seem tempting at first glance.

Right now, many of these sections of this book are incomplete, but the sections that are complete should be quite helpful and usable. This book, and the advice in it, is likely to remain very unstable until Bevy hits 1.0 at some point in the distant future.

<h2> Why You Should Use Bevy </h2>

- blazing fast ECS system with exceptional ergonomics: minimal boilerplate, repeated use of a few simple and powerful patterns

- fully configurable: strip out the bits you don't need, replace them, or hack the core code yourself

- written in Rust, offering fantastic speed, modern ergonomics and superior memory safety
  
- actively developed and carefully curated, with a commitment to code quality

- helpful, friendly, and expert community

<h2> Why You Shouldn't Use Bevy </h2>

- immature: many core functionalities (e.g. asset pipelines, scripting pipelines, advanced scenes, networking) are still incomplete

- no visual editor, limiting its accessibility

- unstable API, likely to break repeatedly

- poorly documented (hence this book!)

<h2> Resources </h2>

As you read along, there are a few key resources you should be aware of:

- The [Bevy Book](https://bevyengine.org/learn/book/introduction/): a great example-driven introduction to the library

- The [Bevy Rust API docs](https://docs.rs/bevy/0.3.0/bevy/): for looking up the details of a particular bit of code

- The [official Github](https://github.com/bevyengine/bevy): the source code is quite legible, and the issues, pull requests and discussions are where the serious engine development takes place

- The official [Bevy examples](https://github.com/bevyengine/bevy/tree/master/examples): a fantastic collection of idiomatic snippets that demonstrate core functionality

- [Awesome Bevy](https://github.com/bevyengine/awesome-bevy): a collection of examples, tutorials and resources for learning and using Bevy

- The [Bevy Discord community](https://discord.com/invite/gMUk5Ph): a friendly, active group for troubleshooting help and casual engine development discussion

Now, let me show you the `World` of Bevy.
