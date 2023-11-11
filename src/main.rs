use clap::Parser;
use colored::Colorize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    // Load Environment Variables from .env file
    dotenv::from_filename(String::from(".env")).ok();

    // Perform cleanup actions when the Ctrl+C signal is received
    ctrlc::set_handler(move || {
        // Cleanup
        println!("Graceful shutdown");
        // Terminate the program after cleanup
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Parse Input Parameters
    let params = rucl::Params::parse();
    // Init Tracing
    env_logger::Builder::new()
        .filter_level(params.verbose.log_level_filter())
        .format_level(false)
        .format_indent(Some(0))
        .format_module_path(false)
        .format_target(false)
        .format_timestamp(None)
        .init();

    // Resolve Host Name
    let host = params
        .url
        .host()
        .unwrap_or_else(|| {
            println!("FATAL ParseHost");
            std::process::exit(0);
        })
        .to_string();
    log::warn!("Connecting to {}", &host.blue().bold());
    let ip = match rucl::dns::query(host.clone()).await {
        Ok(i) => i,
        Err(e) => {
            println!("FATAL QueryDns: {:?}", e);
            std::process::exit(0);
        }
    };
    log::warn!("{} {}", "DNS OK:".green(), ip.to_string().blue().bold());

    // Establish TCP Connection to specific port depending on URL scheme
    let mut tcp = match params.connect_timeout {
        Some(t) => rucl::tcp::connect_timeout(ip, &params.url, t)
            .await
            .unwrap_or_else(|e| {
                println!("FATAL EstablishTcpTimeout: {:?}", e);
                std::process::exit(0);
            }),
        None => rucl::tcp::connect(ip, &params.url)
            .await
            .unwrap_or_else(|e| {
                println!("FATAL EstablishTcp: {:?}", e);
                std::process::exit(0);
            }),
    };
    log::info!("{}", "Establish TCP OK".to_string().green());

    let http_request = rucl::http::build_request(params.clone())
        .await
        .unwrap_or_else(|e| {
            println!("FATAL BuildHttpRequest: {:?}", e);
            std::process::exit(0);
        });
    log::info!("{}:\n{}", "HttpReq OK".green(), &http_request.blue());

    if params.url.scheme() == "https" {
        // Start TLS Session
        let mut tls = rucl::tls::start_session(host, tcp)
            .await
            .unwrap_or_else(|e| {
                println!("FATAL StartTls: {:?}", e);
                std::process::exit(0);
            });
        log::info!("{}", "Start TLS OK".to_string().green());

        // Send HTTP Request
        tls.write_all(http_request.as_bytes())
            .await
            .unwrap_or_else(|e| {
                println!("FATAL TxTls: {:?}", e);
                std::process::exit(0);
            });
        log::info!("{}", "Tx TLS OK".to_string().green());

        // Parse HTTP Response
        let mut res = vec![];
        let read_bytes = tls.read_to_end(&mut res).await.unwrap_or_else(|e| {
            println!("FATAL RxTls: {:?}", e);
            std::process::exit(0);
        });
        log::info!("{}", "Rx TLS OK".to_string().green());
        let read_string = format!("Read {} bytes", read_bytes);
        log::warn!("{}: {}", "TLS OK".green(), read_string.blue().bold());

        // Drop TLS connection
        tls.shutdown().await.unwrap_or_else(|e| {
            println!("FATAL DropTls: {:?}", e);
            std::process::exit(0);
        });

        // Display/Store Results
        println!("{}", String::from_utf8_lossy(&res));
    } else {
        // Send HTTP Request
        tcp.write_all(http_request.as_bytes())
            .await
            .unwrap_or_else(|e| {
                println!("FATAL WriteTcp {:?}", e);
                std::process::exit(0);
            });
        log::info!("{}", "Tx TCP OK".green());

        // Parse HTTP Response
        let mut res = vec![];
        let read_bytes = tcp.read_to_end(&mut res)
            .await
            .unwrap_or_else(|e| {
                println!("FATAL ReadTcp {:?}", e);
                std::process::exit(0);
            });
        log::info!("{}", "Rx TCP OK".green());
        let read_string = format!("Read {} bytes", read_bytes);
        log::warn!("{} {}", "TCP OK:".green(), read_string.blue().bold());

        // Drop TCP connection
        tcp.shutdown().await.unwrap_or_else(|_| {
            println!("FATAL DropTcp");
            std::process::exit(0);
        });

        // Display/Store Results
        println!("HTTP: {}", String::from_utf8_lossy(&res));
    }
}
