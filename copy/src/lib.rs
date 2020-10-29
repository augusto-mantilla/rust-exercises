/*
## copy

### Instructions

Your objective is to fix the program so that all functions work.

- `nbr_function`, returns a tuple with the original value, the exponential and logarithm of that value
- `str_function`, returns a tuple with the original value and the exponential of that value as a string
- `vec_function`, returns a tuple with the original value and the logarithm of each value

### Notions

- https://doc.rust-lang.org/rust-by-example/scope/move.html

*/

// (original value, exponential, logarithm)
fn nbr_function(c: u32) -> (u32, f64, f64){
    (c.clone(), ,)
}
fn logarithm(n: u32) -> f64 {
    (n as f64).ln().abs()
}
fn exponential(n: u32) -> f64 {
    (n as f64).exp()
}

fn str_function(a: String) -> (String, String){
    (a, exp(a).join(" "))
}
fn exp(s: String) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for token in s.split_whitespace() {
        v.push(logarithm(token.parse::<u32>().unwrap()).to_string());
    }
    v
}

fn vec_function(b: Vec<u32>) -> (Vec<u32>, Vec<f64>){
    (b, b.iter().map(|v| exponential(*v)).collect())
}

/*
// Example:
fn main() {
    let a = String::from("1 2 4 5 6");
    let b = vec![1, 2, 4, 5];
    let c: u32 = 0;

    println!("{:?}", nbr_function(c));
    // output: (12, 162754.79141900392, 2.4849066497880004)
    println!("{:?}", vec_function(b));
    // output: ([1, 2, 4], [0.0, 0.6931471805599453, 1.3862943611198906])
    println!("{:?}", str_function(a));
    // output: ("1 2 4", "2.718281828459045 7.38905609893065 54.598150033144236")
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ownership_nbr_test() {
        assert_eq!(nbr_function(12), (12, 162754.79141900392, 2.4849066497880004));
        assert_eq!(nbr_function(1), (1, 2.718281828459045, 0.0));
        assert_eq!(nbr_function(0), (0, 1.0, std::f64::INFINITY));
    }
    #[test]
    fn ownership_vec_test() {
        assert_eq!(vec_function(vec![1, 2, 4]), (vec![1, 2, 4], vec![0.0, 0.6931471805599453, 1.3862943611198906]));
        assert_eq!(vec_function(vec![0, 1]), (vec![0, 1], vec![ std::f64::INFINITY, 0.0]));
        assert_eq!(vec_function(vec![1, 2, 4, 5]), (vec![1, 2, 4, 5], vec![0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003]));
    }
    #[test]
    fn ownership_str_test() {
        assert_eq!(str_function(String::from("1 2 4")), ("1 2 4".to_string(), "2.718281828459045 7.38905609893065 54.598150033144236".to_string()));
        assert_eq!(str_function(String::from("1 0")), (("1 0".to_string(), "2.718281828459045 1".to_string())));
        assert_eq!(str_function(
            String::from("1 2 4 5 6")),
            (("1 2 4 5 6".to_string(), "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_string())));
    }
}
