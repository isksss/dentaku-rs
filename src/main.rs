use std::io::stdin;

fn main() {
    // let mut memory: f64 = 0.0;
    let mut memories: Vec<f64> = vec![0.0; 10];
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        // 空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 空白で分割
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // メモリへの書き込み
        let is_memory = tokens[0].starts_with("mem");

        if is_memory && tokens[0].ends_with("+") {
            add_and_print_memory(&mut memories, tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with("-") {
            add_and_print_memory(&mut memories, tokens[0], -prev_result);
            continue;
        }

        // 式の計算
        let left: f64 = eval_token(tokens[0], &memories);
        let op = tokens[1];
        let right: f64 = eval_token(tokens[2], &memories);

        let result = eval_expression(left, op, right);

        // 結果の表示
        print_output(result);

        prev_result = result;
    }
}

fn print_output(value: f64) {
    println!(" -> {}", value);
}

fn eval_token(token: &str, memories: &[f64]) -> f64 {
    if token.starts_with("mem") {
        let slot_index: usize = token[3..token.len() - 1].parse().unwrap();
        memories[slot_index]
    } else {
        token.parse().unwrap()
    }
}

fn eval_expression(left: f64, op: &str, right: f64) -> f64 {
    match op {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => unreachable!(),
    }
}

fn add_and_print_memory(memories: &mut [f64], token: &str, prev_result: f64) {
    let slot_index: usize = token[3..token.len() - 1].parse().unwrap();
    memories[slot_index] += prev_result;
    print_output(memories[slot_index]);
}
