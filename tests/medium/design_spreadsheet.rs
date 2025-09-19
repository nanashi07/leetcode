// # 3484. Design Spreadsheet
// https://leetcode.com/problems/design-spreadsheet/

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */
struct Spreadsheet {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    fn new(rows: i32) -> Self {
        todo!()
    }

    fn set_cell(&self, cell: String, value: i32) {
        todo!()
    }

    fn reset_cell(&self, cell: String) {
        todo!()
    }

    fn get_value(&self, formula: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::design_spreadsheet::Spreadsheet;

    #[test]
    fn test_spreadsheet_1() {
        let spreadsheet = Spreadsheet::new(3); // Initializes a spreadsheet with 3 rows and 26 columns
        assert_eq!(12, spreadsheet.get_value("=5+7".to_string())); // returns 12 (5+7)
        spreadsheet.set_cell("A1".to_string(), 10); // sets A1 to 10
        assert_eq!(16, spreadsheet.get_value("=A1+6".to_string())); // returns 16 (10+6)
        spreadsheet.set_cell("B2".to_string(), 15); // sets B2 to 15
        assert_eq!(25, spreadsheet.get_value("=A1+B2".to_string())); // returns 25 (10+15)
        spreadsheet.reset_cell("A1".to_string()); // resets A1 to 0
        assert_eq!(15, spreadsheet.get_value("=A1+B2".to_string())); // returns 15 (0+15)
    }
}
