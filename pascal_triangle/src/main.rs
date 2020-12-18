// Create an associated function "new" that compute Pascal's triangle up to a given number of rows.
// In Pascal's Triangle each number is computed by adding the numbers to the right
// and left of the current position in the previous row.

//    1
//   1 1
//  1 2 1
// 1 3 3 1
//1 4 6 4 1
// # ... etc

#[derive(Debug)]
pub struct PascalsTriangle {
    data: Vec<Vec<u32>>,
}

impl PascalsTriangle {
}

fn main() {
    let pt = PascalsTriangle::new(5);
    println!("{:?}", pt.rows());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_rows() {
        let pt = PascalsTriangle::new(0);
        let expected: Vec<Vec<u32>> = Vec::new();
        assert_eq!(expected, pt.rows());
    }

    #[test]
    fn one_row() {
        let pt = PascalsTriangle::new(1);
        let expected: Vec<Vec<u32>> = vec![vec![1]];
        assert_eq!(expected, pt.rows());
    }

    #[test]
    fn two_rows() {
        let pt = PascalsTriangle::new(2);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
        assert_eq!(expected, pt.rows());
    }

    #[test]
    fn three_rows() {
        let pt = PascalsTriangle::new(3);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];
        assert_eq!(expected, pt.rows());
    }

    #[test]
    fn last_of_four_rows() {
        let pt = PascalsTriangle::new(4);
        let expected: Vec<u32> = vec![1, 3, 3, 1];
        assert_eq!(Some(expected), pt.rows().pop());
    }

    #[test]
    fn five_rows() {
        let pt = PascalsTriangle::new(5);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(expected, pt.rows());
    }

    #[test]
    fn six_rows() {
        let pt = PascalsTriangle::new(6);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
        ];
        assert_eq!(expected, pt.rows());
    }

    #[test]
    fn seven_rows() {
        let pt = PascalsTriangle::new(7);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
        ];
        assert_eq!(expected, pt.rows());
    }

    #[test]
    fn ten_rows() {
        let pt = PascalsTriangle::new(10);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
            vec![1, 7, 21, 35, 35, 21, 7, 1],
            vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
            vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
        ];
        assert_eq!(expected, pt.rows());
    }
}
