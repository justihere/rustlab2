use std::io;

fn main() {

    let mut memory: Option<f64> = None; // Змінна для зберігання результату

    loop {
        println!("\n--- Меню ---");
        println!("1. Виконати нову операцію");
        println!("2. Використати запам'ятований результат");
        println!("3. Польська нотація");
        println!("4. Вийти");
        println!("Виберіть опцію:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                let num1 = get_number("Введіть перше число:");
                let num2 = get_number("Введіть друге число:");
                let operator = get_operator();
                let result = calculate(num1, num2, &operator);

                if let Some(res) = result {
                    println!("Результат: {} {} {} = {}", num1, operator, num2, res);
                    memory = Some(res); // Зберігаємо результат
                }
            }
            "2" => {
                if let Some(mem) = memory {
                    println!("Запам'ятований результат: {}", mem);
                    let num2 = get_number("Введіть друге число:");
                    let operator = get_operator();
                    let result = calculate(mem, num2, &operator);

                    if let Some(res) = result {
                        println!("Результат: {} {} {} = {}", mem, operator, num2, res);
                        memory = Some(res); // Оновлюємо результат
                    }
                } else {
                    println!("Немає запам'ятованого результату. Спочатку виконайте операцію.");
                }
            }
            "3" => {
                let expression = get_expression("Введіть вираз в польській нотації (наприклад, '2 3 +'):");
                let result = evaluate_postfix(&expression);

                if let Some(res) = result {
                    println!("Результат: {}", res);
                    memory = Some(res); // Зберігаємо результат
                }
            }
            "4" => {
                println!("До побачення!");
                break;
            }
            _ => println!("Невірний вибір. Спробуйте ще раз."),
        }
    }
}

fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(n) => return n,
            Err(_) => println!("Невірне введення. Спробуйте ще раз."),
        }
    }
}

fn get_operator() -> String {
    loop {
        println!("Введіть оператор (+, -, *, /):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "+" | "-" | "*" | "/" => return input.trim().to_string(),
            _ => println!("Невірний оператор. Спробуйте ще раз."),
        }
    }
}

fn get_expression(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim().to_string();
        if !trimmed.is_empty() {
            return trimmed;
        } else {
            println!("Невірне введення. Спробуйте ще раз.");
        }
    }
}

fn calculate(num1: f64, num2: f64, operator: &str) -> Option<f64> {
    match operator {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                println!("Помилка: ділення на нуль!");
                None
            } else {
                Some(num1 / num2)
            }
        }
        _ => unreachable!(), // Неможливий випадок через перевірку оператора
    }
}

fn evaluate_postfix(expression: &str) -> Option<f64> {
    let mut stack: Vec<f64> = Vec::new();
    let tokens = expression.split_whitespace();

    for token in tokens {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            if stack.len() < 2 {
                println!("Помилка: Недостатньо операндів для оператора '{}'.", token);
                return None;
            }
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            let result = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        println!("Помилка: Ділення на нуль!");
                        return None;
                    }
                    a / b
                }
                _ => {
                    println!("Невідомий оператор '{}'.", token);
                    return None;
                }
            };
            stack.push(result);
        }
    }

    if stack.len() == 1 {
        Some(stack.pop().unwrap())
    } else {
        println!("Помилка: Вираз некоректний.");
        None
    }
}
