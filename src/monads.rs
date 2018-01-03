pub trait Monad<T>: Sized {
    fn ret(v: T) -> Self;
    fn bind<F>(self: Self, f: F) -> Self where F: FnOnce(T) -> Self;
}

#[cfg(test)]
mod monad {
    use super::*;

    impl<T> Monad<T> for Option<T> {
        fn ret(v: T) -> Self {
            Some(v)
        }

        fn bind<F>(self: Self, f: F) -> Self
            where F: FnOnce(T) -> Self
        {
            match self {
                None => None,
                Some(v) => f(v)
            }
        }
    }

    #[test]
    fn option_ret() {
        assert_eq!(5, Option::ret(5).unwrap());
    }

    #[test]
    fn option_bind() {
        assert_eq!(5, Some(2).bind(|x| { Option::ret(x + 3) }).unwrap());
    }
}
