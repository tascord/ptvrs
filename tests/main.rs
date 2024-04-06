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
                .departures_stop(
                    RouteType::Train,
                    1071,
                    DeparturesStopOps {
                        expand: Some(vec![ExpandOptions::All]),
                        ..Default::default()
                    }
                )
                .await
        );
    }
}
