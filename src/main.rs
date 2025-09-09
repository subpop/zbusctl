use clap::{Args, Parser, Subcommand};
use zbus::{Connection, Result};
use zbusctl::build_body;
use zvariant::Structure;

#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A command-line utility for interacting with D-Bus")]
struct ZBusCtl {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Call a D-Bus method
    Call(CallArgs),
}

#[derive(Args)]
struct CallArgs {
    #[arg(long, help = "Use system bus instead of session bus")]
    system: bool,

    #[arg(short, long, help = "D-Bus service name")]
    service: String,

    #[arg(short, long, help = "D-Bus object path")]
    object: String,

    #[arg(short, long, help = "D-Bus interface name")]
    interface: String,

    #[arg(short, long, help = "D-Bus method name")]
    method: String,

    #[arg(help = "D-Bus method arguments")]
    args: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = ZBusCtl::parse();

    match args.command {
        Commands::Call(call) => run_call_command(call).await?,
    }

    Ok(())
}

async fn run_call_command(args: CallArgs) -> Result<()> {
    // Establish D-Bus connection
    let connection = if args.system {
        Connection::system().await?
    } else {
        Connection::session().await?
    };

    let body = if let Some(args) = args.args {
        Some(build_body(args.iter().map(|s| s.as_str()).collect())?)
    } else {
        None
    };

    // Make the D-Bus method call
    let result = match body {
        Some(ref body) => {
            connection
                .call_method(
                    Some(args.service.as_str()),
                    args.object.as_str(),
                    Some(args.interface.as_str()),
                    args.method.as_str(),
                    body,
                )
                .await?
        }
        None => {
            connection
                .call_method(
                    Some(args.service.as_str()),
                    args.object.as_str(),
                    Some(args.interface.as_str()),
                    args.method.as_str(),
                    &(),
                )
                .await?
        }
    };

    // Unpack the result body.
    let result_body = result.body().clone();
    let response = result_body.deserialize::<Structure>()?;

    // Convert the response to a JSON object.
    let response_json = serde_json::to_value(&response.fields()[0])
        .map_err(|e| zbus::Error::Failure(format!("Failed to convert response to JSON: {}", e)))?;

    // Display the result
    println!("{}", response_json);

    Ok(())
}
