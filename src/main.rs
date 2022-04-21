use std;
use std::borrow::Borrow;
use std::thread::LocalKey;

pub struct Fibonacci {
    memoized: Vec<i32>,
}

impl Fibonacci {
    fn thread_local_instance() -> Fibonacci {
        Fibonacci {
            memoized: vec![0, 1],
        }
    }

    fn fib(&mut self, n: i32) -> i32 {
        fn compute_recoursive(fib_instance: &mut Fibonacci, n: i32) -> i32 {
            let fib_n1 = fib_instance
                .memoized
                .get((n - 1) as usize)
                .map_or_else(|| { fib_instance.fib(n - 1) }, |memoized| *memoized);

            // if you can calculate n-1 you surely have n-2
            let fib = fib_n1 + fib_instance.memoized[(n - 2) as usize];
            fib_instance.memoized.insert(n as usize, fib);

            fib
        }

        self.memoized
            .get(n as usize)
            .map_or_else(|| { compute_recoursive(self, n) }, |memoized| *memoized)
    }
}

mod tests {
    use crate::Fibonacci;

    #[test]
    fn fibonacci_0() {
        let mut fibonacci = Fibonacci::thread_local_instance();
        assert_eq!(0, fibonacci.fib(0));
        assert_eq!(1, fibonacci.fib(2));
        assert_eq!(832040, fibonacci.fib(30));
        assert_eq!(3524578, fibonacci.fib(33));
    }
}

fn main() {}
