pub fn doubtful(s: &mut String ) {
    s.push('?');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case0() {
        let mut s = String::from("Hello");
        doubtful(&mut s);
        assert_eq!(s, "Hello?");
    }
}
