mod test_utils;


#[cfg(test)]
mod tests {
    use crate::test_utils;

    use log::info;

    #[tokio::test]
    async fn test_azure_keyvault() -> Result<(), anyhow::Error> {

        test_utils::test_setup().await?;

        let outputs = test_utils::load_env_dryrun_for("azure_keyvault", "dev").await?;

        info!("test_azure_keyvault: Found variables: {:?}", outputs.variables);
        info!("test_azure_keyvault: Found files: {:?}", outputs.files);

        assert_eq!(outputs.variables.get("AZ_KEYVAULT_SECRET_VAR").unwrap().value, "RESULT:novops-test-kv/test-secret/");
        assert_eq!(outputs.files.get("/tmp/AZ_KEYVAULT_SECRET_FILE").unwrap().content, "RESULT:novops-test-kv/test-secret/56ed118a41364a9e8a086e76c43629e4".as_bytes());
        Ok(())
    }

}