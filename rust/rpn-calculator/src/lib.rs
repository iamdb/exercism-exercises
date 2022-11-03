#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for i in inputs {
        match i {
            CalculatorInput::Add => {
                if let (Some(first), Some(second)) = (stack.pop(), stack.pop()) {
                    stack.push(second + first);
                } else {
                    return None;
                }
            }
            CalculatorInput::Subtract => {
                if let (Some(first), Some(second)) = (stack.pop(), stack.pop()) {
                    stack.push(second - first);
                } else {
                    return None;
                }
            }
            CalculatorInput::Multiply => {
                if let (Some(first), Some(second)) = (stack.pop(), stack.pop()) {
                    stack.push(second * first);
                } else {
                    return None;
                }
            }
            CalculatorInput::Divide => {
                if let (Some(first), Some(second)) = (stack.pop(), stack.pop()) {
                    stack.push(second / first);
                } else {
                    return None;
                }
            }
            CalculatorInput::Value(val) => stack.push(*val),
        }
    }

    if stack.len() > 1 {
        None
    } else {
        stack.pop()
    }
}
