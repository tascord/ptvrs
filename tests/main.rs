#[cfg(test)]
pub mod test {
    use std::{
        collections::HashMap, future::Future, pin::Pin, sync::Arc,
    };

    use colored::Colorize;
    use dotenv::dotenv;
    use once_cell::sync::Lazy;
    use ptv::*;

    static CLIENT: Lazy<Client> = Lazy::new(|| {

        // Load .env file if DEVID and KEY are not set
        if !std::env::vars().any(|(k, _)| k == "DEVID" || k == "KEY") {
            dotenv().ok();
        }

        Client::new(
            std::env::var("DEVID").unwrap(),
            std::env::var("KEY").unwrap(),
        )
    });

    // TODO: Find sensible constants
    static ROUTE_TYPE: RouteType = RouteType::Train;
    static ROUTE_ID: i32 = 1;
    static STOP_ID: i32 = 1;

    type Task =
        Arc<dyn Fn() -> Pin<Box<dyn Future<Output = anyhow::Result<String>>>> + Send + Sync>;
    pub static TASKS: Lazy<HashMap<&str, Task>> = Lazy::new(|| {
        let mut map = HashMap::<&str, Task>::new();

        // > Departures
        map.insert(
            "departures_stop",
            Arc::new(|| {
                Box::pin(async {
                    let res = CLIENT
                        .departures_stop(ROUTE_TYPE, STOP_ID, DeparturesStopOpts::default())
                        .await?;

                    Ok(format!("{:?}", res))
                })
            }),
        );

        map.insert(
            "departures_stop_route",
            Arc::new(|| {
                Box::pin(async {
                    let res = CLIENT
                        .departures_stop_route(
                            ROUTE_TYPE,
                            ROUTE_ID,
                            STOP_ID,
                            DeparturesStopRouteOpts::default(),
                        )
                        .await?;

                    Ok(format!("{:?}", res))
                })
            }),
        );

        map
    });

    //

    #[tokio::test]
    pub async fn test() {
        let mut failed = 0;
        for (i, (name, task)) in TASKS.iter().enumerate() {
            println!("[{}] Running test: {}", "~".cyan(), name.yellow());
            let start = std::time::Instant::now();
            let res = task().await;
            let elapsed = start.elapsed();
            match res {
                Ok(res) => println!(
                    "[{}] {} {} in {:?}:\n{}",
                    "+".green(),
                    name.yellow(),
                    "passed".green(),
                    elapsed,
                    res.cyan()
                ),
                Err(e) => {
                    failed += 1;
                    println!(
                        "[{}] {} {} in {:?}:\n{}",
                        "-".red(),
                        name.yellow(),
                        "failed".red(),
                        elapsed,
                        e.to_string().cyan()
                    )
                }
            }

            if i < TASKS.len() - 1 {
                println!("\n[{}] Waiting 5 seconds to avoid limiting.\n", "~".cyan());
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }

        if failed > 0 {
            panic!("{} tests failed", failed);
        }

        println!("\n{}", "All tests passed! :3".green());
    }
}
