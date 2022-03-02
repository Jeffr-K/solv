pub mod arrays {
    pub mod insert;
    
    #[cfg(test)]
    mod test {
        use super::insert;

        #[test]
        fn find_max_consecutive_ones_test() {
            let vec = vec![1,2,3,4,5];
            assert_eq!(3, insert::find_max_consecutive_ones(vec));
        }
    }
}

