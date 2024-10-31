// Ensures all 4 CRUD functions under lib.rs run successfully
#[cfg(test)]
mod tests {
    use sqlite_crud::*;

    #[test]
    fn test_crud() {
        let url = "https://raw.githubusercontent.com/nogibjj/ag825_individual_project_2/refs/heads/main/sqlite_crud/Cancer_Data.csv";
        let file_path = "Cancer_Data.csv";
        let database = "CancerDB.db";
        
        let result = crud(database);

        assert!(
            result.is_ok(),
            "FAILED");
    }

}
