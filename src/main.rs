use notify_rust::{Notification, Urgency};
use std::{fs, thread, time};

struct BatteryState {
	capacity: String,
	status: ChargeState,
	new_status: ChargeState,
}

#[derive(PartialEq, Clone)]
enum ChargeState {
	Full,
	Charging,
	Discharging,
}

fn main() {
	let mut battery: BatteryState = BatteryState {
		capacity: "".to_string(),
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
	if battery.status != battery.new_status {
		status(&battery.new_status);
		battery.status.clone_from(&battery.new_status);
	} else if battery.status == ChargeState::Discharging {
		if battery.capacity <= "10\n".to_string() {
			capacity('0', &battery.capacity)
		} else if battery.capacity <= "5\n".to_string() {
			capacity('1', &battery.capacity);
		} else if battery.capacity <= "3\n".to_string() {
		}
	}
	battery
}

fn status(state: &ChargeState) {
	match state {
		ChargeState::Full => notify("Fully Charged", "Battery is fully charged.", Urgency::Low),
		ChargeState::Charging => notify("Charging", "Battery is now plugged in.", Urgency::Low),
		ChargeState::Discharging => notify(
			"Power Unplugged",
			"Your computer has been disconnected from power.",
			Urgency::Low,
		),
	}
}

fn capacity(c: char, capacity: &String) {
	let formated = format!("Less then {}% of battery remaining.", capacity);
	match c {
		'0' => notify("Low Battery", &formated, Urgency::Normal),
		'1' => notify(
			"Low Battery",
			"Your computer will suspend soon unless plugged into a power outlet.",
			Urgency::Critical,
		),
		'2' => power_off(),
		_ => panic!(),
	}
}

fn power_off() {
	/*Command::new("/usr/bin/systemctl")
		.arg("suspend")
		.spawn()
		.expect("failed to execute process"); */
}

fn notify(title: &str, message: &str, priority: Urgency) {
	Notification::new()
		.summary(title)
		.body(message)
		.urgency(priority)
		.show()
		.unwrap();
}

fn read_batterystate(mut battery: BatteryState) -> BatteryState {
	battery.capacity =
		fs::read_to_string("/sys/class/power_supply/BAT1/capacity").expect("Failed to read file");
	battery.new_status = parse_batterystate(
		&fs::read_to_string("/sys/class/power_supply/BAT1/status").expect("Failed to read file"),
	);
	battery
}

fn parse_batterystate(string: &str) -> ChargeState {
	match string {
		"Full\n" => ChargeState::Full,
		"Charging\n" => ChargeState::Charging,
		"Discharging\n" => ChargeState::Discharging,
		_ => panic!(),
	}
}
