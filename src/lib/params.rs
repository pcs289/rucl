use clap::Parser;
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Debug, Clone)]
#[clap(version, about = "rucl: RUst CurL!", author = "Pau de la Cuesta")]
pub struct Params {
    #[clap(long, value_name = "file")]
    /// CA certificate to verify peer against
    cacert: Option<String>,

    #[clap(long, value_name = "dir")]
    /// CA directory to verify peer against
    capath: Option<PathBuf>,

    #[clap(short = 'E', long, value_name = "certificate[:password]")]
    /// Client certificate file and password
    cert: Option<String>,

    #[clap(long)]
    /// Verify the status of the server cert via OCSP-staple
    cert_status: bool,

    #[clap(long, value_name = "type")]
    /// Certificate type (DER/PEM/ENG/P12)
    cert_type: Option<String>,

    #[clap(long, value_name = "list of ciphers")]
    /// SSL ciphers to use
    ciphers: Option<String>,

    #[clap(long, value_name = "fractional second")]
    /// Maximum time allowed for connection
    pub connect_timeout: Option<f32>,

    #[clap(short = 'b', long, value_name = "data|filename")]
    /// Send cookies from string/file
    cookie: Option<String>,

    #[clap(short = 'c', long, value_name = "filename")]
    /// Write cookies to <filename> after operation
    cookie_jar: Option<String>,

    #[clap(short, long, value_name = "data")]
    /// HTTP POST data
    pub data: Option<String>,

    #[clap(short = 'F', long, value_name = "name=string")]
    /// Specify multipart MIME data
    pub form: Option<String>,

    #[clap(long = "form-string", value_name = "name=string")]
    /// Specify multipart MIME data
    form_string: Option<String>,

    #[clap(short = 'G', long)]
    /// Put the post data in the URL and use GET
    pub get: bool,

    #[clap(short = 'I', long)]
    /// Show document info only
    pub head: bool,

    #[clap(short = 'H', long)]
    /// Show document info only
    header: Option<String>,

    #[clap(short, long)]
    /// Include protocol response headers in the output
    include: bool,

    #[clap(short, long, value_name = "file")]
    /// Write to file instead of stdout
    output: Option<PathBuf>,

    #[clap(value_name = "url")]
    /// Target URL
    pub url: Url,

    #[clap(short, long, value_name = "user:password")]
    /// Server user and password
    user: Option<String>,

    #[clap(short = 'A', long, value_name = "name")]
    /// Send User-Agent <name> to server
    user_agent: Option<String>,

    #[command(flatten)]
    pub verbose: Verbosity,
}
