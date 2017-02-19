trait Display {
    fn get_columns(&self) -> usize;
    fn get_rows(&self) -> usize;
    fn get_row_text(&self, row: usize) -> String;
    fn show(&self) {
        for i in 0..self.get_rows() {
            println!("{}", self.get_row_text(i));
        }
    }
}

struct StringDisplay {
    string: String,
}

impl StringDisplay {
    fn new(string: String) -> StringDisplay {
        StringDisplay {
            string: string,
        }
    }
}

impl Display for StringDisplay {
    fn get_columns(&self) -> usize {
        let bytes_len = self.string.len();
        let chars_len = self.string.chars().count();
        if bytes_len == chars_len {
            self.string.len()
        } else {
            2 * self.string.chars().count()
        }
    }

    fn get_rows(&self) -> usize {
        1
    }

    fn get_row_text(&self, row: usize) -> String {
        if row == 0 {
            self.string.clone()
        } else {
            "".to_string()
        }
    }
}

trait Border : Display {}

struct SideBorder {
    display: Box<Display>,
    border_char: char,
}

impl SideBorder {
    fn new(display: Box<Display>, border_char: char) -> SideBorder {
            SideBorder {
                display: display,
                border_char: border_char,
            } 
    }
}

impl Display for SideBorder {
    fn get_columns(&self) -> usize {
        1 + self.display.get_columns() + 1
    }

    fn get_rows(&self) -> usize {
        self.display.get_rows()  
    }

    fn get_row_text(&self, row: usize) -> String {
        format!("{}{}{}", self.border_char, self.display.get_row_text(row), self.border_char)
    }
}

impl Border for SideBorder {}

struct FullBorder {
    display: Box<Display>,    
}

impl FullBorder {
    fn new(display: Box<Display>) -> FullBorder {
        FullBorder {
            display: display,
        }
    }

    fn make_line(&self, ch: char,count: usize) -> String {
        let mut buffer = "".to_string();
        for _ in 0..count {
            buffer.push(ch);
        }
        buffer
    }
}


impl Display for FullBorder {
    fn get_columns(&self) -> usize {
        1 + self.display.get_columns() + 1
    }

    fn get_rows(&self) -> usize {
        1 + self.display.get_rows() + 1
    }

    fn get_row_text(&self, row: usize) -> String {
        if row == 0 {
            format!("+{}+", self.make_line('-', self.display.get_columns()))
        } else if row == self.display.get_rows() + 1 {

            format!("+{}+", self.make_line('-', self.display.get_columns()))
        } else {
            format!("|{}|", self.display.get_row_text(row-1))

        }
    }
}

impl Border for FullBorder {}

fn main() {
    let b1 = StringDisplay::new("Hello, world.".to_string());
    b1.show();
    
    let b2 = SideBorder::new(Box::new(b1), '#');
    b2.show();
    
    let b3 = FullBorder::new(Box::new(b2));
    b3.show();

    let b4 = SideBorder::new(
        Box::new(FullBorder::new(
                Box::new(FullBorder::new(
                        Box::new(SideBorder::new(
                                Box::new(FullBorder::new(
                                        Box::new(StringDisplay::new(
                                                "こんにちは。".to_string()
                                                )
                                            )
                                        )
                                    ),
                                    '*'
                                )
                            )
                        )
                    )
                )
            ),
            '/'
        );
    b4.show();
}
