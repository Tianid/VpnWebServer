/// Raw (unvalidated) CLI arguments parsed from tokens.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RawArgs {
    pub address: Option<String>,
    pub port: Option<String>,
}

/// Read arguments from any iterator of Strings.
/// This function performs only syntactic parsing and does NOT validate values.
/// Supported forms:
/// - --key=value and -k=value
/// - --key value and -k value
/// Unknown flags are ignored. If using std::env::args(), the binary name is skipped.
pub fn read_args<T>(args: T) -> RawArgs
where
    T: IntoIterator<Item = String>,
{
    let mut iterator = args.into_iter().peekable();
    let mut address: Option<String> = None;
    let mut port: Option<String> = None;

    // Skip binary name if present (first token without leading '-' and without '=')
    if let Some(first) = iterator.peek() {
        if !first.starts_with('-') && !first.contains('=') {
            iterator.next();
        }
    }

    while let Some(arg) = iterator.next() {
        if let Some((k, v)) = arg.split_once('=') {
            let v = v.trim();
            match k {
                "--address" | "-a" => address = Some(v.to_string()),
                "--port" | "-p" => port = Some(v.to_string()),
                _ => {}
            }
            continue;
        }

        match arg.as_str() {
            "--address" | "-a" => {
                if let Some(next) = iterator.peek() {
                    if !next.starts_with('-') {
                        address = iterator.next().map(|s| s.trim().to_string());
                    }
                }
            }
            "--port" | "-p" => {
                if let Some(next) = iterator.peek() {
                    // Accept value if it doesn't look like a flag OR if it's a negative number (e.g. "-1")
                    if !next.starts_with('-') || looks_like_negative_number(next) {
                        port = iterator.next().map(|s| s.trim().to_string());
                    }
                }
            }
            _ => {}
        }
    }

    RawArgs { address, port }
}

fn looks_like_negative_number(s: &str) -> bool {
    let rest = s.strip_prefix('-').unwrap_or("");
    !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
}
