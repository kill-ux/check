#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
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

    pub fn filter_col<T: Fn(&str) -> bool>(&self, filter: T) -> Option<Self> {
        let mut table = Table::new();
        for (i, header) in self.headers.iter().enumerate() {
            if filter(header) {
                table.headers.push(header.to_string());
                for e in &self.body {
                    table.add_row(&[e[i].to_string()]);
                }
                return Some(table);
            }
        }
        None
    }

    pub fn filter_row<T: Fn(&str) -> bool>(&self, col_name: &str, filter: T) -> Option<Self> {
        let mut table = Table::new();
        let mut index = 0;
        for (i, header) in self.headers.iter().enumerate() {
            if header == col_name {
                index = i;
            }
        }
        table.headers = self.headers.clone();
        for row in &self.body {
            if filter(&row[index]) {
                table.add_row(row);
            }
        }
        return Some(table);
    }
}
