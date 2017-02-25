trait Mediator {
    fn create_colleagues(&self);
    fn colleague_changed(&self);
}

struct LoginFrame {
    title: String,
}

impl LoginFrame {
    fn new(title: String) -> LoginFrame {
        let lf = LoginFrame {
            title: title,
        };

        lf.create_colleagues();
        lf.colleague_changed();

        lf
    }

    fn user_pass_changed(&self) {
    }

    fn print(&self) {
        println!("--------------------");
        println!("{}", self.title);
        println!("Guest:[x] Login:[]");
        println!("Username: \"\"");
        println!("Password: \"\"");
        println!("--------------------");
    }
}

impl Mediator for LoginFrame {
    fn create_colleagues(&self) { 
    }

    fn colleague_changed(&self) {
        // create_collegues
        let s : Box<&Mediator> = Box::new(self);
        let check_guest = Box::new(ColleagueCheckBox::new("Guest".to_string(), true, &s));
        let check_login = Box::new(ColleagueCheckBox::new("Login".to_string(), false, &s));
        let mut text_user = Box::new(ColleagueTextField::new("".to_string(), 10, &s));
        let mut text_pass = Box::new(ColleagueTextField::new("".to_string(), 10, &s));
        let mut button_ok = Box::new(ColleagueButton::new("OK".to_string(), &s));
        let button_cancel = Box::new(ColleagueButton::new("CANCEL".to_string(), &s));

        if check_guest.get_state() {
            text_user.set_colleague_enabled(false);
            text_pass.set_colleague_enabled(false);
            button_ok.set_colleague_enabled(true);
        } else {
            // user_pass_changed
            let user_length = text_user.get_text().len();
            let pass_length = text_pass.get_text().len();
            text_user.set_colleague_enabled(true);
            if user_length > 0 {
                text_pass.set_colleague_enabled(true);
                if pass_length > 0 {
                    button_ok.set_colleague_enabled(true);
                } else {
                    button_ok.set_colleague_enabled(false);
                }
            } else {
                text_pass.set_colleague_enabled(false);
                button_ok.set_colleague_enabled(false);
            }
        }
    }
}

trait Colleague<'a> {
    fn set_mediator(&'a mut self, mediator: &'a Box<&'a Mediator>);
    fn set_colleague_enabled(&'a mut self, enabled: bool);
}

struct ColleagueButton<'a> {
    caption: String,
    mediator: &'a Box<&'a Mediator>,
    enabled: bool,
}

impl<'a> ColleagueButton<'a> {
    fn new(caption: String, mediator: &'a Box<&'a Mediator>) -> ColleagueButton {
        ColleagueButton {
            caption: caption,
            mediator: mediator,
            enabled: true,
        }
    }
}

impl<'a> Colleague<'a> for ColleagueButton<'a> {
    fn set_mediator(&'a mut self, mediator: &'a Box<&'a Mediator>) {
        self.mediator = mediator;
    }

    fn set_colleague_enabled(&'a mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

struct ColleagueTextField<'a> {
    text: String,
    columns: u32,
    mediator: &'a Box<&'a Mediator>,
    enabled: bool,
}

impl<'a> ColleagueTextField<'a> {
    fn new(text: String, columns: u32, mediator: &'a Box<&'a Mediator>) -> ColleagueTextField {
        ColleagueTextField {
            text: text,
            columns: columns,
            mediator: mediator,
            enabled: true,
        }
    }

    fn text_value_changed(&self) {
        self.mediator.colleague_changed();
    }

    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl<'a> Colleague<'a> for ColleagueTextField<'a> {
    fn set_mediator(&'a mut self, mediator: &'a Box<&'a Mediator>) {
        self.mediator = mediator;
    }

    fn set_colleague_enabled(&'a mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

struct ColleagueCheckBox<'a> {
    caption: String,
    mediator: &'a Box<&'a Mediator>,
    enabled: bool,
}

impl<'a> ColleagueCheckBox<'a> {
    fn new(caption: String, enabled: bool, mediator: &'a Box<&'a Mediator>) -> ColleagueCheckBox {
        ColleagueCheckBox {
            caption: caption,
            mediator: mediator,
            enabled: enabled,
        }
    }

    fn item_state_changed(&self) {
        self.mediator.colleague_changed();
    }

    fn get_state(&self) -> bool {
        self.enabled
    }
}

impl<'a> Colleague<'a> for ColleagueCheckBox<'a> {
    fn set_mediator(&'a mut self, mediator: &'a Box<&'a Mediator>) {
        self.mediator = mediator;
    }

    fn set_colleague_enabled(&'a mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

fn main() {
    let mut lf = LoginFrame::new("Mediator Sample".to_string());
    lf.print();
    
    // TODO
    // lf.check_guest();
    // lf.check_login();
    // lf.input_username("yuki".to_string());
    // lf.input_password("pass".to_string());
    // lf.click_ok();
    // lf.click_cancel();
}
