// IMPORTANT NOTE:
// This example does not currently work properly and skips events

use bevy::prelude::*;
use rand::Rng;
use std::thread::sleep;
// We're using bevy's re-exported time types
// because std::time isn't supported on wasm
use bevy::ecs::bevy_utils::{Duration, Instant};

struct ImportantMessage {
	pub message_number: u32,
	pub time_stamp: f64,
}

struct TimeBudget {
	duration: Duration,
}

// This example demonstrates pattern with Events
// because there's any easy way to track and save progress
// But you could do this with any system that you can safely pause and defer
fn main() {
	App::build()
		.add_plugins(MinimalPlugins)
		.init_resource::<Events<ImportantMessage>>()
		.add_system(send_events.system())
		// We could use FrameTimeDiagnostics instead to calibrate this
		// and slowly decrease our budget when our frame times start to increase
		.add_resource(TimeBudget {
			duration: Duration::new(0, 3 * 10 ^ 8),
		})
		.add_system(do_work.system())
		.run();
}

fn send_events(
	time: Res<Time>,
	mut events: ResMut<Events<ImportantMessage>>,
	mut message_number: Local<u32>,
) {
	let n = rand::thread_rng().gen_range(0..5);
	for _ in 0..n {
		*message_number += 1;
		events.send(ImportantMessage {
			message_number: *message_number,
			time_stamp: time.seconds_since_startup(),
		});

		println! {"Message {} sent!", *message_number};
	}
}

fn do_work(
	time: Res<Time>,
	events: Res<Events<ImportantMessage>>,
	mut event_reader: Local<EventReader<ImportantMessage>>,
	time_budget: Res<TimeBudget>,
) {
	// We can't use Res<Time> here, since it only updates at the start of each tick
	let system_start = Instant::now();

	for event in event_reader.iter(&events) {
		// Sleeping for 0.1 seconds
		// Processing these events sure does take a while!
		sleep(Duration::new(0, 10 ^ 8));

		println!(
			"Message {:?} sent at {:?} was processed at {:?}",
			event.message_number,
			event.time_stamp,
			time.seconds_since_startup()
		);

		// We have to check whether to break the loop here, rather than the beginning
		// to avoid dropping events
		// The event counter is updated upon iteration
		if system_start.elapsed() >= time_budget.duration {
			break;
		}
	}
}
