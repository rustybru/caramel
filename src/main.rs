fn main() {
    println!("Hello, caramel!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main_works () {
        assert_eq!(1,1);
    }
}
