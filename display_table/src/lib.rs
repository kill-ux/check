use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.len() == 0 {
            return write!(f, "");
        }

        let mut lens: Vec<_> = self.headers
            .iter()
            .map(|str| str.chars().count())
            .collect();
        for i in 0..self.body.len() {
            for j in 0..self.body[i].len() {
                if lens[j] < self.body[i][j].len() {
                    lens[j] = self.body[i][j].len();
                }
            }
        }

        write!(f, "|")?;

        for (i, header) in self.headers.iter().enumerate() {
            let number = lens[i] - header.chars().count();
            let devide = number / 2;
            let str1 = " ".repeat(devide);
            let str2 = " ".repeat(number - devide);
            write!(f, " {}{header}{} |", str1, str2)?;
        }

        write!(f, "\n|")?;

        for (i, _) in self.headers.iter().enumerate() {
            if i == self.headers.len() - 1 {
                write!(f, "{}|", "-".repeat(lens[i] + 2))?;
            } else {
                write!(f, "{}+", "-".repeat(lens[i] + 2))?;
            }
        }

        for (i, header) in self.body.iter().enumerate() {
            write!(f, "\n|")?;
            for (j, header) in self.body[i].iter().enumerate() {
                let number = lens[j] - header.chars().count();
                let devide = number / 2;
                let str1 = " ".repeat(devide);
                let str2 = " ".repeat(number - devide);
                write!(f, " {}{header}{} |", str1, str2)?;
            }
        }

        write!(f, "\n")
    }
}

impl Table {
    pub fn new() -> Table {
        Self {
            headers: vec![],
            body: vec![],
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}
