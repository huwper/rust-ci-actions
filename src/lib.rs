pub fn ex(a: usize) -> usize {
    a+1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex() {
        assert_eq!(ex(5), 6);
    }
}

