#[macro_export]
macro_rules! when {
    {$($cond: expr => $then: expr,)*} => {
        match () {
            $(_ if $cond => $then,)*
            _ => unreachable!(),
        }
    };
    {$($cond: expr => $then: expr,)* else $(=>)? $else: expr,} => {
        match () {
            $(_ if $cond => $then,)*
            _ => $else,
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_when_without_else() {
        let x = 10;
        let result = when! {
            x % 2 == 0 => "even",
            x % 3 == 0 => "divisible by three",
        };

        assert_eq!(result, "even");
    }

    #[test]
    fn test_when_with_else() {
        let x = 7;
        let result = when! {
            x % 2 == 0 => "even",
            x % 3 == 0 => "divisible by three",
            else => "none",
        };

        assert_eq!(result, "none");
    }
}
