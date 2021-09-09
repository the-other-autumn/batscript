use notify_rust::{Notification, Urgency};
use std::{fs, process::Command, str::FromStr, thread, time};

struct BatteryState {
	capacity: i32,
	status: ChargeState,
	new_status: ChargeState,
}

#[derive(PartialEq, Clone)]
enum ChargeState {
	Full,
	Charging,
	Discharging,
	Unknown,
}

fn main() {
	let mut battery: BatteryState = BatteryState {
		capacity: 100,
		status: ChargeState::Full,
		new_status: ChargeState::Full,
	};
	battery = read_batterystate(battery);
	battery.status.clone_from(&battery.new_status);
	loop {
		thread::sleep(time::Duration::from_secs(10));
		battery = trigger(battery);
	}
}

fn trigger(mut battery: BatteryState) -> BatteryState {
	battery = read_batterystate(battery);
	if battery.status != battery.new_status && battery.new_status != ChargeState::Unknown {
		status(&battery.new_status);
		battery.status.clone_from(&battery.new_status);
	} else if battery.status == ChargeState::Discharging {
		capacity(battery.capacity)
	}
	battery
}

fn status(state: &ChargeState) {
	match state {
		ChargeState::Full => notify("Fully Charged", "Battery is fully charged.", Urgency::Low),
		ChargeState::Charging => notify("Charging", "Battery is now plugged in.", Urgency::Low),
		ChargeState::Discharging => notify("Power Unplugged", "Your computer has been disconnected from power.", Urgency::Low),
		ChargeState::Unknown => (),
	}
}

fn capacity(capacity: i32) {
	if capacity <= 3 {
		power_off()
	} else if capacity <= 5 {
		notify("Low Battery", "Your computer will suspend soon unless plugged into a power outlet.", Urgency::Critical)
	} else if capacity <= 10 {
		let formated = format!("Less then {}% of battery remaining.", capacity);
		notify("Low Battery", &formated, Urgency::Normal)
	}
}

fn power_off() {
	Command::new("/usr/bin/sudo").args(["/usr/bin/systemctl", "suspend"]).spawn().expect("failed to execute process");
}

fn notify(title: &str, message: &str, priority: Urgency) {
	Notification::new().summary(title).body(message).urgency(priority).show().unwrap();
}

fn read_batterystate(mut battery: BatteryState) -> BatteryState {
	battery.capacity = i32::from_str(
		&fs::read_to_string("/sys/class/power_supply/BAT1/capacity")
			.expect("Failed to read file")
			.replace("\n", ""),
	)
	.unwrap();
	battery.new_status = parse_batterystate(
		&fs::read_to_string("/sys/class/power_supply/BAT1/status")
			.expect("Failed to read file")
			.replace("\n", ""),
	);
	battery
}

fn parse_batterystate(string: &str) -> ChargeState {
	match string {
		"Full" => ChargeState::Full,
		"Charging" => ChargeState::Charging,
		"Discharging" => ChargeState::Discharging,
		"Unknown" => ChargeState::Unknown,
		_ => panic!(),
	}
}
