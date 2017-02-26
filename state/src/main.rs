extern crate rand;

use std::fmt;
use std::{thread, time};
use std::process;
use rand::{thread_rng, Rng};

trait State : fmt::Display {
    fn do_clock(&self, hour: u32) -> Box<State>;
    fn do_use(&self, context: Box<&Context>);
    fn do_alarm(&self, context: Box<&Context>);
    fn do_phone(&self, context: Box<&Context>);
    fn value(&self) -> String;
}

impl PartialEq<State> for State {
    fn eq(&self, other: &State) -> bool {
        self.value() == other.value()
    }
}

trait Context {
    fn set_clock(&mut self, hour: u32);
    fn change_state(&mut self, state: Box<State>);
    fn call_security_center(&self, msg: String);
    fn record_log(&self, msg: String);
}

#[derive(PartialEq)]
struct DayState {}

impl DayState {
    fn new() -> DayState {
        DayState {}
    }
}

impl State for DayState {
    fn do_clock(&self, hour: u32) -> Box<State> {
        if hour < 9 || 17 <= hour {
            Box::new(NightState::new())
        } else {
            Box::new(DayState::new())
        }
    }

    fn do_use(&self, context: Box<&Context>) {
        context.record_log("金庫使用(昼間)".to_string());
    }

    fn do_alarm(&self, context: Box<&Context>) {
        context.call_security_center("非常ベル(昼間)".to_string());
    }

    fn do_phone(&self, context: Box<&Context>) {
        context.call_security_center("通常の通話(昼間)".to_string());
    }
    
    fn value(&self) -> String {
        "昼間".to_string()
    }
}

impl fmt::Display for DayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[昼間]")
    }
}

#[derive(PartialEq)]
struct NightState {}

impl NightState {
    fn new() -> NightState {
        NightState {}
    }
}

impl State for NightState {
    fn do_clock(&self, hour: u32) -> Box<State> {
        if 9 <= hour && hour < 17 {
            Box::new(DayState::new())
        } else {
            Box::new(NightState::new())
        }
    }

    fn do_use(&self, context: Box<&Context>) {
        context.call_security_center("非常:夜間の金庫使用!".to_string());
    }

    fn do_alarm(&self, context: Box<&Context>) {
        context.call_security_center("非常ベル(夜間)".to_string());
    }

    fn do_phone(&self, context: Box<&Context>) {
        context.record_log("夜間の通話録音".to_string());
    }

    fn value(&self) -> String {
        "夜間".to_string()
    }
}

impl fmt::Display for NightState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[夜間]")
    }
}

struct SafeFrame {
    title: String,
    state: Box<State>,
}

impl SafeFrame {
    fn new(title: String, state: Box<State>) -> SafeFrame {
        SafeFrame {
            title: title,
            state: state,
        }
    }

    fn click_use(&self) {
        self.state.do_use(Box::new(self)); 
    }

    fn click_alarm(&self) {
        self.state.do_alarm(Box::new(self));
    }

    fn click_phone(&self) {
        self.state.do_phone(Box::new(self));
    }

    fn click_exit(&self) {
        process::exit(0);
    }
}

impl Context for SafeFrame {
    fn set_clock(&mut self, hour: u32) {
        println!("現在時刻は{0: >02}:00", hour);
        
        let state = self.state.do_clock(hour);
        if &self.state != &state {
            self.change_state(state);
        }
    }

    fn change_state(&mut self, state: Box<State>) {
        println!("{}から{}へ状態が変化しました。", self.state, state);
        self.state = state;
    }

    fn call_security_center(&self, msg: String) {
        println!("call! {}", msg);
    }

    fn record_log(&self, msg: String) {
        println!("record ... {}", msg);
    }
}

fn main() {
    let mut frame = SafeFrame::new("State Sample".to_string(), Box::new(NightState::new()));
    let mut rng = thread_rng();

    println!("------------");
    println!("{}", frame.title);
    println!("------------\n");
    
    loop {
        for hour in 0..24 {
            frame.set_clock(hour);
            
            match rng.gen_range(0, 3) {
                0 => frame.click_use(),
                1 => frame.click_alarm(),
                2 => frame.click_phone(),
                _ => frame.click_exit(),
            }
            
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}
