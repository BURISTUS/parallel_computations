use parallel_computations::errors::GeneralErrors;
use rayon::prelude::*;

const TRESHOLD: usize = 5;

fn parallel_computations<T, R, F>(input: Vec<T>, compute_fn: F) -> Result<Vec<R>, GeneralErrors>
where
    T: Send + Sync,
    R: Send,
    F: Fn(&T) -> R + Send + Sync,
{
    if input.len() <= 1 {
        return Err(GeneralErrors::LengthError);
    }

    if TRESHOLD > input.len() {
        println!("Program execution was in single thread");
        return Ok(input.iter().map(compute_fn).collect());
    }

    println!("Program execution was divided into threads");
    Ok(input.par_iter().map(compute_fn).collect())
}

fn main() -> Result<(), GeneralErrors> {
    let input = vec![1, 2, 3, 4, 5];
    let output = parallel_computations(input, |x| x * x + 25)?;
    println!("{:?}", output);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_map_with_small_input() {
        let input = vec![1, 2];
        let output = parallel_computations(input, |x| x * x).unwrap();
        assert_eq!(output, vec![1, 4]);
    }

    #[test]
    fn test_parallel_map_with_large_input() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let output = parallel_computations(input, |x| x * x).unwrap();
        assert_eq!(output, vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]);
    }

    #[test]
    fn test_parallel_map_with_another_expression() {
        let input = vec![1, 2, 3, 4, 5];
        let output = parallel_computations(input, |x| x * x * x).unwrap();
        assert_eq!(output, vec![1, 8, 27, 64, 125]);
    }

    #[test]
    fn test_parallel_map_with_invalid_length() {
        let input = vec![1];
        let actual_err = parallel_computations(input, |x| x * x).unwrap_err();
        let expected_err = GeneralErrors::LengthError;
        assert_eq!(actual_err, expected_err);
    }
}
