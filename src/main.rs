use std::{fs, process::Command, thread, time};
use notify_rust::{Notification, Urgency};

struct STATE {
	capacity: String,
	status: String,
	new_capacity: String,
	new_status: String,
}

fn main() {
	let mut st: STATE = STATE { capacity: "".to_string(), status: "".to_string(), new_status: "".to_string(), new_capacity: "".to_string()};
	st = read_capacity(st);
	st.capacity.clone_from(&st.new_capacity);
	st.status.clone_from(&st.new_status);
	loop {
		thread::sleep(time::Duration::from_secs(10));
		st = trigger(st);
	}
}

fn trigger(st: STATE) -> STATE {
	let mut state = read_capacity(st);

	if state.capacity != state.new_capacity {
		state.capacity.clone_from(&state.new_capacity);
	}
	if state.status != state.new_status {
		if state.new_status == "Full\n" {
			status('0');
		} else if state.new_status == "Charging\n" {
			status('1');
		} else if state.new_status == "Discharging\n" {
			status('2');
		}
		state.status.clone_from(&state.new_status);
	} else if state.status == "Discharging" {
		if state.capacity <= "10\n".to_string() {
			capacity('0', &state)
		} else if state.capacity <= "5\n".to_string() {
			capacity('1', &state);
		} else if state.capacity <= "3\n".to_string() {
		}
	}
	state
}

fn status(c: char) {
	match c {
		'0' => notify("Fully Charged", "Battery is fully charged.", Urgency::Low),
		'1' => notify("Charging", "Battery is now plugged in.", Urgency::Low),
		'2' => notify("Power Unplugged", "Your computer has been disconnected from power.", Urgency::Low),
		_ => panic!()
	}
}

fn capacity(c: char, st: &STATE) {
	let formated = format!("Less then {}% of battery remaining.", st.status);
	match c {
		'0' =>  notify("Low Battery", &formated, Urgency::Normal),
		'1' => notify("Low Battery", "Your computer will suspend soon unless plugged into a power outlet.", Urgency::Critical),
		'2' => power_off(),
		_ => panic!()
	}
}

fn power_off() {
	Command::new("/usr/bin/systemctl")
		.arg("suspend")
		.spawn()
		.expect("failed to execute process");
}


fn notify(title: &str, message: &str, priority:	Urgency) {
	Notification::new()
	.summary(title)
	.body(message)
	.urgency(priority)
	.show().unwrap();
}

fn read_capacity(st: STATE) -> STATE {
	let mut state = st;
	let capacity = fs::read_to_string("/sys/class/power_supply/BAT1/capacity")
		.expect("Failed to read file");
	let status = fs::read_to_string("/sys/class/power_supply/BAT1/status")
		.expect("Failed to read file");
		state.new_capacity = capacity;
		state.new_status = status;
		state
}