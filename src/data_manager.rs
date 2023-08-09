pub mod manager {
    use sqlx::postgres::PgPool;

    pub struct DbTool {
        pub endpoint: String,
        password: String,
        pool: PgPool,
    }
    impl DbTool {
        pub async fn new(args: &[String]) -> Result<DbTool, sqlx::Error> {
            let endpoint = args[0].clone();
            let password = args[1].clone();
            let connection_string = endpoint
                .replace("[YOUR-PASSWORD]", &password)
                .replace('"', "");

            println!("Connecting to {connection_string}");
            let pool = PgPool::connect(&connection_string).await?;
            Ok(DbTool {
                endpoint,
                password,
                pool,
            })
        }
    }
}
