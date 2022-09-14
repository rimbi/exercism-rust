use std::vec;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for input in inputs {
        let val = match input {
            CalculatorInput::Add => {
                stack.pop()? + stack.pop()?
            }
            CalculatorInput::Subtract => {
                let pop = stack.pop()?;
                stack.pop()? - pop
            }
            CalculatorInput::Multiply => {
                stack.pop()? * stack.pop()?
            }
            CalculatorInput::Divide => {
                let pop = stack.pop()?;
                stack.pop()? / pop
            }
            CalculatorInput::Value(val) => *val,
        };
        stack.push(val);
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
