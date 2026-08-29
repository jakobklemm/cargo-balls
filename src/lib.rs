use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

/// Measures how long it takes to run `f`.
pub fn measure(f: impl FnOnce()) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

/// How measured durations map to ball animation speed
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    /// Scale every ball so the fastest measured entry laps the specified time.
    Relative(Duration),
    /// Each ball laps the track in exactly its own measured duration.
    Absolute,
}

impl Default for Mode {
    /// `Relative(1000ms)`.
    fn default() -> Self {
        Mode::Relative(Duration::from_millis(1000))
    }
}

/// Runs each benchmark, times it, and writes an HTML "ball race" visualization
/// to `target/balls/<name>.html`, where `<name>` is the file stem of
/// `source` (pass `file!()`) — so each benchmark file gets its own page
/// instead of all of them overwriting a shared `index.html`. `title` is
/// shown as the page title and heading.
///
/// Prefer the [`balls_main!`] macro over calling this directly.
pub fn render(title: &str, source: &str, mut entries: Vec<(&str, Duration)>, mode: Mode) {
    assert!(!entries.is_empty(), "cargo-balls: no benchmarks to render");

    entries.sort_by_key(|(_, duration)| *duration);

    let scale = match mode {
        // Guard against a zero measured duration (e.g. a no-op benchmark),
        // which would otherwise make the scale factor infinite.
        Mode::Relative(fastest) => {
            let quickest = entries[0].1.max(Duration::from_nanos(1));
            fastest.as_secs_f64() / quickest.as_secs_f64()
        }
        Mode::Absolute => 1.0,
    };

    let subtitle = match mode {
        Mode::Relative(fastest) => {
            format!("fastest run laps the track in {fastest:.2?}, others scaled relative to it")
        }
        Mode::Absolute => "each ball laps the track at its actual measured speed".to_string(),
    };

    let mut styles = String::new();
    let mut balls = String::new();

    for (i, (name, duration)) in entries.iter().enumerate() {
        let color = COLORS[i % COLORS.len()];
        let scaled = duration.mul_f64(scale);
        let lap = scaled.as_millis().max(1);

        styles += &format!(".ball-{i} {{ background: {color}; animation-duration: {lap}ms; }}\n");
        balls += &format!(
            r#"<div class="row">
      <span class="label">{name}</span>
      <div class="track"><div class="ball ball-{i}"></div></div>
      <span class="time">{duration:.2?} ({scaled:.2?})</span>
    </div>
"#
        );
    }

    let output = TEMPLATE
        .replace("__TITLE__", &escape_html(title))
        .replace("/*__STYLES__*/", &styles)
        .replace("<!--__BALLS__-->", &balls)
        .replace("__SUBTITLE__", &subtitle);

    let stem = std::path::Path::new(source)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("index");

    let mut file = path();
    file.push(format!("{stem}.html"));
    std::fs::write(&file, output).unwrap();
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// // Defaults to title "cargo-balls" and relative mode, fastest ball = 1 second lap.
/// balls_main!(first, second);
/// ```
///
/// Both are optional and, when present, go before the function list in the
/// order `title:`, then mode, each terminated by `;`:
///
/// ```ignore
/// balls_main!(title: "my benchmarks", relative: Duration::from_secs(2); first, second);
/// balls_main!(title: "my benchmarks", absolute; first, second);
/// ```
#[macro_export]
macro_rules! balls_main {
    (@main $title:expr, $mode:expr; $($func:ident),+ $(,)?) => {
        fn main() {
            let entries: ::std::vec::Vec<(&str, ::std::time::Duration)> = ::std::vec![
                $((::std::stringify!($func), $crate::measure($func))),+
            ];
            $crate::render($title, ::std::file!(), entries, $mode);
        }
    };
    (title: $title:expr, relative: $fastest:expr; $($func:ident),+ $(,)?) => {
        $crate::balls_main!(@main $title, $crate::Mode::Relative($fastest); $($func),+);
    };
    (title: $title:expr, absolute; $($func:ident),+ $(,)?) => {
        $crate::balls_main!(@main $title, $crate::Mode::Absolute; $($func),+);
    };
    (title: $title:expr; $($func:ident),+ $(,)?) => {
        $crate::balls_main!(@main $title, $crate::Mode::default(); $($func),+);
    };
    (relative: $fastest:expr; $($func:ident),+ $(,)?) => {
        $crate::balls_main!(@main "cargo-balls", $crate::Mode::Relative($fastest); $($func),+);
    };
    (absolute; $($func:ident),+ $(,)?) => {
        $crate::balls_main!(@main "cargo-balls", $crate::Mode::Absolute; $($func),+);
    };
    ($($func:ident),+ $(,)?) => {
        $crate::balls_main!(@main "cargo-balls", $crate::Mode::default(); $($func),+);
    };
}

const COLORS: [&str; 8] = [
    "#2978e7", "#bb1838", "#1bdd7a", "#dda111", "#b87bb4", "#118300", "#4a3aa7", "#e34948",
];

const TEMPLATE: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>__TITLE__</title>
<script>
  try {
    var saved = localStorage.getItem("cargo-balls-theme");
    if (saved === "light" || saved === "dark") document.documentElement.dataset.theme = saved;
  } catch (e) {}
</script>
<style>
  :root {
    color-scheme: light;
    --surface: #fcfcfb;
    --surface-raised: #ffffff;
    --border: #e5e3de;
    --text-primary: #0b0b0b;
    --text-secondary: #52514e;
    --track: #eeece6;
  }
  @media (prefers-color-scheme: dark) {
    :root:not([data-theme="light"]) {
      color-scheme: dark;
      --surface: #1a1a19;
      --surface-raised: #232322;
      --border: #34332f;
      --text-primary: #ffffff;
      --text-secondary: #c3c2b7;
      --track: #2a2a27;
    }
  }
  :root[data-theme="dark"] {
    color-scheme: dark;
    --surface: #1a1a19;
    --surface-raised: #232322;
    --border: #34332f;
    --text-primary: #ffffff;
    --text-secondary: #c3c2b7;
    --track: #2a2a27;
  }
  * { box-sizing: border-box; }
  body {
    margin: 0;
    padding: 40px 20px;
    background: var(--surface);
    color: var(--text-primary);
    font-family: Helvetica
    display: flex;
    justify-content: center;
  }
  .card {
    width: 100%;
    max-width: 720px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 28px 32px;
  }
  .header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
  }
  h1 {
    margin: 0 0 4px;
    font-size: 1.25rem;
  }
  .subtitle {
    margin: 0 0 28px;
    color: var(--text-secondary);
    font-size: 0.875rem;
  }
  .theme-toggle {
    flex-shrink: 0;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-primary);
    border-radius: 8px;
    width: 32px;
    height: 32px;
    font-size: 1rem;
    line-height: 1;
    cursor: pointer;
  }
  .theme-toggle:hover {
    border-color: var(--text-secondary);
  }
  .row {
    display: flex;
    align-items: center;
    gap: 14px;
    margin: 18px 0;
  }
  .label {
    min-width: 110px;
    max-width: 110px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 0.875rem;
    font-weight: 600;
  }
  .track {
    position: relative;
    flex: 1;
    height: 22px;
    background: var(--track);
    border-radius: 999px;
  }
  .ball {
    position: absolute;
    top: 1px;
    left: 0;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.35);
    animation-name: move;
    animation-timing-function: ease-in-out;
    animation-iteration-count: infinite;
    animation-direction: alternate;
  }
  @keyframes move {
    from { left: 0; }
    to { left: calc(100% - 20px); }
  }
  .time {
    min-width: 130px;
    text-align: right;
    white-space: nowrap;
    font-variant-numeric: tabular-nums;
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }
/*__STYLES__*/
</style>
</head>
<body>
  <div class="card">
    <div class="header">
      <div>
        <h1>__TITLE__</h1>
        <p class="subtitle">__SUBTITLE__</p>
      </div>
      <button class="theme-toggle" onclick="cargoBallsToggleTheme()" aria-label="Toggle color theme">&#9680;</button>
    </div>
<!--__BALLS__-->
  </div>
  <script>
    function cargoBallsToggleTheme() {
      var root = document.documentElement;
      var isDark = root.dataset.theme
        ? root.dataset.theme === "dark"
        : window.matchMedia("(prefers-color-scheme: dark)").matches;
      var next = isDark ? "light" : "dark";
      root.dataset.theme = next;
      try { localStorage.setItem("cargo-balls-theme", next); } catch (e) {}
    }
  </script>
</body>
</html>
"##;

/// Returns `target/balls/`, creating it if necessary.
pub fn path() -> PathBuf {
    let mut target = target_path();
    target.push("balls");
    std::fs::create_dir_all(&target).unwrap();
    target
}

fn target_path() -> PathBuf {
    if let Ok(dir) = std::env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(dir);
    }

    let exe = std::env::current_exe().unwrap();
    let mut dir = exe.as_path();
    while let Some(parent) = dir.parent() {
        if dir.file_name().map(|n| n == "target").unwrap_or(false) {
            return dir.to_path_buf();
        }
        dir = parent;
    }

    panic!("cargo-balls: could not locate the `target` directory");
}
