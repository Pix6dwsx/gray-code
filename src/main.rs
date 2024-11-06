fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::from("")];
    }
    let mut result = vec![String::from("0"), String::from("1")];
    for _ in 1..n {
        let mut rev = result.clone();
        rev.reverse();
        result = result.iter().map(|s| format!("0{}", s)).collect();
        result.extend(rev.iter().map(|s| format!("1{}", s)));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray() {
        let test_data = [
            (0, vec![""]),
            (1, vec!["0", "1"]),
            (2, vec!["00", "01", "11", "10"]),
            (3, vec!["000", "001", "011", "010", "110", "111", "101", "100"]),
            (4, vec![
                "0000", "0001", "0011", "0010",
                "0110", "0111", "0101", "0100",
                "1100", "1101", "1111", "1110",
                "1010", "1011", "1001", "1000"
            ]),
        ];

        for (n, expected) in test_data.iter() {
            assert_eq!(gray(*n), *expected);
        }
    }
}

fn main() {
    let n = 3; // Пример: генерируем коды Грея для 3 бит
    let gray_codes = gray(n);
    println!("Gray codes for {} bits: {:?}", n, gray_codes);
}
