use anyhow::{Context, Result};
use clap::Parser;
use odbc_api::{Connection, ConnectionOptions, Environment, Cursor, ResultSetMetadata};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ODBC Driver name
    #[arg(short = 'D', long)]
    driver: String,

    /// Server address and port
    #[arg(short, long)]
    server: String,

    /// Database name
    #[arg(short = 'b', long)]
    database: String,

    /// Username
    #[arg(short, long)]
    uid: String,

    /// Password
    #[arg(short, long)]
    pwd: String,

    /// SQL query to execute
    #[arg(short, long)]
    query: String,
}

struct OdbcClient {
    env: Environment,
}

impl OdbcClient {
    fn new() -> Result<Self> {
        let env = Environment::new()?;
        Ok(Self { env })
    }

    fn connect(&self, connection_string: &str) -> Result<Connection> {
        self.env
            .connect_with_connection_string(connection_string, ConnectionOptions::default())
            .context("Failed to connect to database")
    }

    fn execute_query(&self, connection: &Connection, query: &str) -> Result<()> {
        let mut statement = connection
            .prepare(query)
            .context("Failed to prepare statement")?;
        
        let cursor = statement
            .execute(())
            .context("Failed to execute query")?;

        if let Some(mut cursor) = cursor {
            let num_cols = cursor.num_result_cols()?;
            let mut buffer = Vec::new();
            while let Some(mut row) = cursor.next_row().context("Failed to fetch row")? {
                // Print column values
                for i in 0..num_cols {
                    buffer.clear();
                    let value = row.get_text(i as u16 + 1, &mut buffer)?;
                    print!("{} ", if value { String::from_utf8_lossy(&buffer) } else { "NULL".into() });
                }
                println!();
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let client = OdbcClient::new()?;
    
    let connection_string = format!(
        "Driver={{{}}};Server={};Database={};UID={};PWD={}",
        args.driver, args.server, args.database, args.uid, args.pwd
    );
    
    let connection = client.connect(&connection_string)?;
    client.execute_query(&connection, &args.query)?;

    Ok(())
}
