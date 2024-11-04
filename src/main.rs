use std::io::stdin;

fn main() {
    for line in stdin().lines() {
        // 空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 空白で分割
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // 式の計算
        let left: f64 = tokens[0].parse().unwrap();
        let right: f64 = tokens[2].parse().unwrap();
        let result = match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => unreachable!(),
        };

        // 結果の表示
        print_value(result);
    }
}

fn print_value(value: f64) {
    println!(" -> {}", value);
}
