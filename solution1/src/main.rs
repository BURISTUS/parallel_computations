use parallel_computations::errors::GeneralErrors;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

const TRESHOLD: usize = 5;

fn parallel_computations<T, R, F>(input: Vec<T>, compute_fn: F) -> Result<Vec<R>, GeneralErrors>
where
    T: Send + Sync + 'static + Clone,
    R: Send + 'static + Clone,
    F: Fn(&T) -> R + Send + Sync + 'static,
{
    if input.len() <= 1 {
        return Err(GeneralErrors::LengthError);
    }

    if TRESHOLD > input.len() {
        println!("Program execution was in single thread");
        return Ok(input.iter().map(compute_fn).collect());
    }

    let mid = input.len() / 2;
    let (left_input, right_input) = input.split_at(mid);

    let left_fn = Arc::new(compute_fn);
    let right_fn = left_fn.clone();

    let left_mutex = Arc::new(Mutex::new(Vec::new())); //
    let right_mutex = Arc::new(Mutex::new(Vec::new())); //

    let handle_left = handle(left_input, &left_mutex, left_fn);
    let handle_right = handle(right_input, &right_mutex, right_fn);

    if handle_left.join().is_err() {
        return Err(GeneralErrors::JoinError);
    }

    if handle_right.join().is_err() {
        return Err(GeneralErrors::JoinError);
    }

    let left_output = left_mutex.lock().map_err(|_| GeneralErrors::LockError)?;
    let right_output = right_mutex.lock().map_err(|_| GeneralErrors::LockError)?;

    println!("Program execution was divided into threads");
    Ok(left_output
        .iter()
        .cloned()
        .chain(right_output.iter().cloned())
        .collect())
}

fn handle<T, R, F>(
    input: &[T],
    mutex: &Arc<Mutex<Vec<R>>>,
    compute_fn: Arc<F>,
) -> JoinHandle<Result<(), GeneralErrors>>
where
    T: Send + 'static + Clone,
    R: Send + 'static + Clone,
    F: Fn(&T) -> R + Send + Sync + 'static,
{
    thread::spawn({
        let input = input.to_vec();
        let mutex = mutex.clone();
        move || -> Result<(), GeneralErrors> {
            let mut result = Vec::with_capacity(input.len());
            for item in input {
                let r = compute_fn(&item);
                result.push(r);
            }
            let mut output = mutex.lock().map_err(|_| GeneralErrors::LockError)?;
            output.extend(result);
            Ok(())
        }
    })
}

fn main() {
    let input = vec![1, 2, 3, 4, 5];
    let output = parallel_computations(input, |x| x * x + 25);
    println!("{:?}", output);
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
