// # 3484. Design Spreadsheet
// https://leetcode.com/problems/design-spreadsheet/
use std::cell::RefCell;
use std::collections::HashMap;

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */
struct Spreadsheet {
    cells: RefCell<HashMap<String, i32>>,
    rows: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Spreadsheet {
            cells: RefCell::new(HashMap::new()),
            rows,
        }
    }

    fn set_cell(&self, cell: String, value: i32) {
        self.cells.borrow_mut().insert(cell, value);
    }

    fn reset_cell(&self, cell: String) {
        self.cells.borrow_mut().remove(&cell);
    }

    fn get_value(&self, formula: String) -> i32 {
        if !formula.starts_with('=') {
            return formula.parse().unwrap_or(0);
        }

        let expression = &formula[1..]; // Remove the '=' prefix
        self.evaluate_expression(expression)
    }

    fn evaluate_expression(&self, expr: &str) -> i32 {
        // Parse and evaluate the expression
        let tokens = self.tokenize(expr);
        self.evaluate_tokens(&tokens)
    }

    fn tokenize(&self, expr: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();

        for ch in expr.chars() {
            match ch {
                '+' | '-' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token);
                        current_token = String::new();
                    }
                    tokens.push(ch.to_string());
                }
                ' ' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token);
                        current_token = String::new();
                    }
                }
                _ => {
                    current_token.push(ch);
                }
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        tokens
    }

    fn evaluate_tokens(&self, tokens: &[String]) -> i32 {
        if tokens.is_empty() {
            return 0;
        }

        let mut result = self.get_token_value(&tokens[0]);
        let mut i = 1;

        while i < tokens.len() {
            if i + 1 >= tokens.len() {
                break;
            }

            let operator = &tokens[i];
            let operand = self.get_token_value(&tokens[i + 1]);

            match operator.as_str() {
                "+" => result += operand,
                "-" => result -= operand,
                _ => {}
            }

            i += 2;
        }

        result
    }

    fn get_token_value(&self, token: &str) -> i32 {
        // Check if it's a number
        if let Ok(num) = token.parse::<i32>() {
            return num;
        }

        // Check if it's a cell reference
        if self.is_valid_cell_reference(token) {
            return self.cells.borrow().get(token).copied().unwrap_or(0);
        }

        0
    }

    fn is_valid_cell_reference(&self, cell: &str) -> bool {
        if cell.len() < 2 {
            return false;
        }

        let mut chars = cell.chars();
        let col_char = chars.next().unwrap();

        // Check if first character is A-Z
        if !col_char.is_ascii_uppercase() {
            return false;
        }

        // Check if remaining characters form a valid row number
        let row_str: String = chars.collect();
        if let Ok(row_num) = row_str.parse::<i32>() {
            return row_num >= 1 && row_num <= self.rows;
        }

        false
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
