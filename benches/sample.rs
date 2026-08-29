use cargo_balls::*;
use std::{thread, time::Duration};

fn some_function(n: u64) {
    println!("running benchmark: {}", n);
    thread::sleep(Duration::from_millis(n));
}

const TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html>
<head>
  <style>
    .container {
      font-family: Arial, sans-serif;
      margin: 20px;
    }
    .ball-container {
      display: flex;
      align-items: center;
      margin: 10px 0;
      width: 400px;
    }
    .ball {
      width: 20px;
      height: 20px;
      border-radius: 50%;
      margin-right: 10px;
    }
    STYLES
    @keyframes move {
      from { transform: translateX(0); }
      to { transform: translateX(350px); }
    }
    .label {
      min-width: 120px;
      text-align: right;
      margin-right: 10px;
    }
  </style>
</head>
<body>
  <div class="container">
    BALLS
  </div>
</body>
</html>
"#;

const COLORS: [&'static str; 16] = [
    "aqua", "fuchsia", "gray", "green", "maroon", "navy", "olive", "purple", "red", "silver",
    "teal", "white", "lime", "yellow", "black", "blue",
];

fn main() {
    let first = ("first", measure(|| some_function(17)));
    let second = ("second", measure(|| some_function(42)));
    let third = ("third", measure(|| some_function(10)));

    let mut all = vec![first, second, third];
    assert!(all.len() > 0);
    assert!(all.len() <= COLORS.len());

    all.sort_by_key(|(_, dur)| dur.clone());

    let (_, fastest) = all[0];
    let inverse = 1000 / fastest.as_millis();
    assert!(inverse < (u32::MAX as u128));
    let inverse = inverse as u32;

    let actuals = all.iter().map(|(name, dur)| (name, *dur * inverse));

    let mut styles = String::new();
    let mut balls = String::new();

    for (i, (name, time)) in actuals.enumerate() {
        let color = COLORS[i];

        let style = format!(
            r#"
            .ball-{i} {{
                background-color: {color};
                animation: move {}ms infinite alternate ease-in-out;
            }}
            "#,
            time.as_millis()
        );
        let ball = format!(
            r#"
            <div class="ball-container">
              <span class="label">{name}</span>
              <div class="ball ball-{i}"></div>
            </div>
            "#
        );

        styles += &style;
        balls += &ball;
    }

    let output = TEMPLATE;
    let output = output.replace("STYLES", &styles);
    let output = output.replace("BALLS", &balls);

    let mut path = path();
    path.push("index.html");

    std::fs::write(&path, output).unwrap();
}
