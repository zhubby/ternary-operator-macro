#[macro_export]
macro_rules! ternary {
    ($condition: expr => $true_expr: expr , $false_expr: expr) => {
        if $condition {
            $true_expr
        } else {
            $false_expr
        }
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ternary() {
        assert_eq!(ternary!(true => "a","b"),"a");
        assert_eq!(ternary!(false => "a","b"),"b");
    }
}
