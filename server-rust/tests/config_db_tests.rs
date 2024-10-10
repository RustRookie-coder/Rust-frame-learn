mod tests {

    #[test]
    pub fn test_db_connection() {
        dotenv::dotenv().expect("Cant find env file.");
    }

    #[test]
    pub fn test_collection_func() {
        let tar = vec![1, 2, 3];

        let res = tar.iter().map(|e| e + 1).collect::<Vec<i32>>();

        println!("res: {:?}", res)
    }
}
