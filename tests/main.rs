#[cfg(test)]
pub mod test {
    use dotenv::dotenv;
    use once_cell::sync::Lazy;
    use ptvrs::*;

    static CLIENT: Lazy<Client> = Lazy::new(|| {
        dotenv().ok();
        Client::new(
            std::env::var("DEVID").unwrap(),
            std::env::var("KEY").unwrap(),
        )
    });

    //

    #[tokio::test]
    pub async fn test() {
        println!(
            "{:?}",
            CLIENT
                .runs_id(
                    2,
                    RunsIdOpts {
                        expand: vec![ExpandOptions::All].into(),
                        ..Default::default()
                    }
                )
                .await
        );
    }
}
