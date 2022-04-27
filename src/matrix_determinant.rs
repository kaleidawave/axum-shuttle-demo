use savage_core::{expression::Expression, functions::function_expression};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct InputNotMatrix;

impl Display for InputNotMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input is not a matrix")
    }
}

impl Error for InputNotMatrix {}

#[derive(Debug)]
pub enum ComputeMatrixDeterminantError {
    ParseError(String),
    CalculationError(String),
}

impl Display for ComputeMatrixDeterminantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputeMatrixDeterminantError::ParseError(err) => write!(f, "Parse error: '{}'", err),
            ComputeMatrixDeterminantError::CalculationError(err) => {
                write!(f, "Calculation error: '{}'", err)
            }
        }
    }
}

impl Error for ComputeMatrixDeterminantError {}

pub fn compute_matrix_determinant(source: &str) -> Result<String, Box<dyn Error>> {
    let expr = source
        .parse::<Expression>()
        .map_err(|err| ComputeMatrixDeterminantError::ParseError(format!("{:?}", err)))?;
    if !matches!(expr, Expression::Matrix(_)) {
        return Err(Box::new(InputNotMatrix));
    }
    let det_function = function_expression("det").unwrap();
    let expr = Expression::FunctionValue(Box::new(det_function.into()), vec![expr]);
    Ok(expr
        .evaluate(Default::default())
        .map_err(|err| ComputeMatrixDeterminantError::CalculationError(format!("{:?}", err)))?
        .to_string())
}
