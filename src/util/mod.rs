use serenity::utils::parse_username;

pub fn parse_id<S: AsRef<str>>(s: S) -> Option<u64> {
    parse_username(s.as_ref()).or_else(|| s.as_ref().parse::<u64>().ok())
}
