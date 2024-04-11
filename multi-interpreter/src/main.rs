// different number and expression types

#[derive(Debug)]
enum Expression {
    Integer(i32),
    FixedPoint(bool, u32, u32),
    Addition(Vec<Expression>)
}

fn evaluate_addition(expression: &Expression) -> Expression {
    if let Expression::Addition(expressions) = expression {
        match expressions[0] {
            Expression::Integer(_) => evaluate_add_integers(expressions),
            _ => panic!("I don't want to do this")
        }
    } else {
        panic!("not an addition")
    }
}

fn evaluate_integer(expression: &Expression) -> f64 {
    if let Expression::Integer(value) = expression {
        *value as f64
    } else {
        panic!("not an integer, it's something else")
    }
}

fn evaluate_add_integers(expressions: &Vec<Expression>) -> Expression {
    let mut total = 0;
    for each in expressions {
        if let Expression::Integer(value) = each {
            total = total + value;
        } else {
            panic!("we aren't messing around")
        }
    }
    Expression::Integer(total)
}

fn evaluate_fixed_point(expression: &Expression) -> f64 {
    if let Expression::FixedPoint(sign, whole, frac) = expression {
        return (*whole as f64) + ((*frac as f64)/100.0);
    } else {
        panic!("of course we panic")
    }
}

fn add_fixed_point(expressions: &Vec<Expression>) -> Expression {
    let mut total_whole = 0;
    let mut total_frac = 0;
    for each in expressions {
        if let Expression::FixedPoint(sign, whole, frac) = each {
            let mut whole_sum = total_whole + whole;
            let mut frac_sum = total_frac + frac;
        } else {
            panic!("of course we panic")
        }
    }
    Expression::FixedPoint(true, total_whole, total_frac)
}

fn evaluate(expression: &Expression) -> f64 {
    match expression {
        // _ means we don't care what it is
        Expression::Addition(_) => evaluate(&evaluate_addition(expression)),
        Expression::Integer(_) => evaluate_integer(expression),
        Expression::FixedPoint(_,_,_) => evaluate_fixed_point(expression)
    }
}

fn main() {
    
}

#[cfg(test)]
mod test {
    use crate::{add_fixed_point, Expression};

    #[test]
    fn test_anything_works() {
        assert!(true, "it workd")
    }
    #[test]
    fn test_simple_addition() {
        //arrange
        let expr = crate::Expression::Addition(vec![
            crate::Expression::Integer(2),
            crate::Expression::Integer(2)
        ]);
        //assert
        let sum = crate::evaluate(&expr);
        //act
        assert_eq!(sum, 4.0, "expressions not equal or whetever");
    }
    fn test_fixed_point_addition() {
        //arrange
        let expr = crate::Expression::FixedPoint(
            (false), 
            1, 
            55);
        let expr2 = crate::Expression::FixedPoint(
            (false), 
            1, 
            55);   
        let mut numbers = vec![expr, expr2]; 
        //act
        let mut addish = add_fixed_point(&numbers);
        // had a bit of trouble figuring out what to print for the assert
        //assert
        assert_eq!(addish);
    }
}
