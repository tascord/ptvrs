#[allow(dead_code)]
#[cfg(test)]
pub mod test {
    use std::{collections::{BTreeMap, HashMap}, future::Future, pin::Pin, sync::Arc};

    use colored::Colorize;
    use dotenv::dotenv;
    use once_cell::sync::Lazy;
    use ptv::*;

    macro_rules! make_test {
        ($m:expr, $name:literal, {$e:expr}) => {
            $m.insert($name, Arc::new(|| {
                Box::pin(async {
                    let res = $e.await?;
                    Ok(format!("{:?}", res))
                })
            }));
        };
    }

    static CLIENT: Lazy<Client> = Lazy::new(|| {
        // Load .env file if DEVID and KEY are not set
        if std::env::var("DEVID").is_err() || std::env::var("KEY").is_err() {
            dotenv().ok();
        }

        Client::new(
            std::env::var("DEVID").unwrap(),
            std::env::var("KEY").unwrap(),
        )
    });

    // TODO: Find sensible constants
    static ROUTE_TYPE: RouteType = RouteType::Train; // Train
    static ROUTE_ID: i32 = 1; // Alamein (Line)
    static STOP_ID: i32 = 1002; // Alamein (Station)
    static DIRECTION_ID: i32 = 1; // Towards Flinders Street

    type Task =
        Arc<dyn Fn() -> Pin<Box<dyn Future<Output = anyhow::Result<String>>>> + Send + Sync>;
    pub static TASKS: Lazy<BTreeMap<&str, Task>> = Lazy::new(|| {
        let mut map = BTreeMap::<&str, Task>::new();

        // > Departures
        make_test!(map, "departures_stop", {
            CLIENT.departures_stop(ROUTE_TYPE, STOP_ID, DeparturesStopOpts::default())
        });

        make_test!(map, "departures_stop_route", {
            CLIENT.departures_stop_route(
                ROUTE_TYPE,
                ROUTE_ID,
                STOP_ID,
                DeparturesStopRouteOpts::default()
            )
        });

        // > Directions

        make_test!(map, "directions_id", {
            CLIENT.directions_id(DIRECTION_ID)
        });

        make_test!(map, "directions_route", {
            CLIENT.directions_route(ROUTE_ID)
        });

        make_test!(map, "directions_id_route", {
            CLIENT.directions_id_route(DIRECTION_ID, ROUTE_TYPE)
        });

        // > Disruptions

        make_test!(map, "disruptions", {
            CLIENT.disruptions(DisruptionsOpts::default())
        });

        make_test!(map, "disruptions_route", {
            CLIENT.disruptions_route(ROUTE_ID, DisruptionsSpecificOpts::default())
        });

        make_test!(map, "disruptions_route_stop", {
            CLIENT.disruptions_route_stop(ROUTE_ID, STOP_ID, DisruptionsSpecificOpts::default())
        });

        make_test!(map, "disruptions_stop", {
            CLIENT.disruptions_stop(STOP_ID, DisruptionsSpecificOpts::default())
        });

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
                    "[{}] {} {} in {:?}:{}",
                    "+".green(),
                    name.yellow(),
                    "passed".green(),
                    elapsed,
                    {
                        if std::env::var("quiet").is_ok() {
                            format!("\n{}", res.cyan())
                        } else {
                            " ...".cyan().to_string()
                        }
                    }
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
