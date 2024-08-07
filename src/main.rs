use clap::{Parser, Subcommand};

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Limit package lookup to a specific provider
    #[arg(short, long)]
    provider: Option<String>,

    /// Limit package lookup to a specific distribution
    #[arg(short, long)]
    distribution: Option<String>,

    /// Limit package lookup to a specific component
    #[arg(short, long)]
    component: Option<String>,

    /// The package name, DEB file path or DEB URL to install
    #[arg(name = "PACKAGE NAME | FILE | URL")]
    package: Option<String>,

    /// Specify values to use as single value for XDEB_OPTS when executing xdeb
    #[arg(short, long, allow_hyphen_values = true, num_args = 0..)]
    option: Vec<String>,

    /// Specify values to use as environment variables when executing xdeb
    #[arg(name = "KEY=VALUE", short = 'e', long = "environment", allow_hyphen_values = true, num_args = 0..)]
    environment: Vec<String>,

    /// Set the temporary xdeb context root path
    #[arg(name = "DIRECTORY", long = "temp-directory")]
    temp_directory: Option<String>,

    /// Pass any following arguments to the xdeb binary
    #[arg(name = "XDEB ARGS", raw = true)]
    pass_on: Vec<String>,

    #[command(subcommand)]
    command: Option<MainCommands>,
}

#[derive(Clone, Debug, Subcommand)]
enum MainCommands {
    /// Install the xdeb utility to the system along with its dependencies
    #[clap(visible_alias("x"), visible_alias("X"))]
    Xdeb {
        /// Release version
        release: Option<String>,
    },

    /// Synchronize remote repositories
    #[clap(visible_alias("S"))]
    Sync {
        /// Limit synchronization to specific providers
        #[arg(short, long, num_args = 0..)]
        provider: Vec<String>,

        /// Limit synchronization to specific distributions
        #[arg(short, long)]
        #[arg(short, long, num_args = 0..)]
        distribution: Vec<String>,

        /// Limit synchronization to specific components
        #[arg(short, long)]
        #[arg(short, long, num_args = 0..)]
        component: Vec<String>,
    },

    /// List available providers
    #[clap(visible_alias("p"), visible_alias("P"))]
    Providers {
        /// Display provider details
        #[arg(short, long, action)]
        details: bool,
    },

    /// List available distributions
    #[clap(visible_alias("d"), visible_alias("D"))]
    Distributions {
        /// Display distribution details
        #[arg(short, long, action)]
        details: bool,
    },

    /// List available components
    #[clap(visible_alias("c"))]
    Components {
        /// Display component details
        #[arg(short, long, action)]
        details: bool,
    },

    /// Search synchronized repositories for a package
    #[clap(visible_alias("s"))]
    Search {
        /// The package to look for.
        package: String,

        /// Perform an exact match of the package name provided
        #[arg(short, long, action)]
        exact_match: bool,

        /// Limit package lookup to a specific provider
        #[arg(short, long)]
        provider: Option<String>,

        /// Limit package lookup to a specific distribution
        #[arg(short, long)]
        distribution: Option<String>,

        /// Limit package lookup to a specific component
        #[arg(short, long)]
        component: Option<String>,
    },

    /// Clean up temporary xdeb context root path
    #[clap(visible_alias("C"))]
    Clean {
        /// Clean up repository lists as well
        #[arg(short, long, action)]
        lists: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);
}
