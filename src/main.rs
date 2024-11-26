use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Sets a custom config file
    #[arg(global = true, short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Enable colored log messages
    #[arg(global = true, long)]
    color: Option<bool>,

    /// Set log verbosity. Valid values: fatal,
    /// error, warn, notice, info, debug
    #[arg(global = true, short, long)]
    verbose: Option<bool>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Puppet dynamic environment deployment
    Deploy(DeployArgs),
    /// Perform operations on a Puppetfile
    Puppetfile(PuppetfileArgs),
    /// Print the version of r11k
    Version,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct DeployArgs {
    #[command(subcommand)]
    command: DeployCommands,

    /// Specify a cachedir, overriding the value in config
    #[arg(global = true, long)]
    cache_dir: Option<String>,
    /// Exclude the module's spec dir for deployment
    #[arg(global = true, long)]
    exclude_spec: Option<String>,
    /// Run `puppet generate types` after updating an environment
    #[arg(global = true, long)]
    generate_types: Option<bool>,
    /// Prevent the overwriting of local module modifications
    #[arg(global = true, long)]
    no_force: Option<bool>,
    /// Path to puppet.conf
    #[arg(global = true, long)]
    puppet_conf: Option<PathBuf>,
    /// Path to puppet executable
    #[arg(global = true, long)]
    puppet_path: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum DeployCommands {
    /// Display environments and modules in the deployment
    Display(DeployDisplayArgs),
    /// Deploy environments and their dependent modules
    Environment(DeployEnvironmentArgs),
    /// Deploy modules in all environments
    ///
    /// `r11k deploy module` Deploys and updates modules inside of Puppet
    /// environments. It will load the Puppetfile configurations out of all
    /// environments, and will try to deploy the given module names in all
    /// environments.
    Module(DeployModuleArgs),
}

#[derive(Debug, Args)]
struct DeployDisplayArgs {
    /// Display detailed information
    #[arg(long)]
    detail: Option<bool>,
    /// Update available environment lists from all remote sources
    #[arg(long)]
    fetch: Option<bool>,
    /// Display output in a specific format. Valid values: json, yaml.
    #[arg(long)]
    format: Option<String>,
    /// Display modules
    #[arg(short, long)]
    modules: Option<bool>,
}

#[derive(Debug, Args)]
struct DeployEnvironmentArgs {
    /// Specify a branchname to override the default branch in the puppetfile
    #[arg(long)]
    default_branch_override: Option<String>,
    /// Used with the --modules flag, only update those modules whose definition has changed or whose definition allows the version to float
    #[arg(long)]
    incremental: Option<bool>,
    /// Deploy modules
    #[arg(short, long)]
    modules: Option<bool>,
}

#[derive(Debug, Args)]
struct DeployModuleArgs {
    /// Update the modules in the given environment
    #[arg(short, long)]
    environment: Option<String>,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct PuppetfileArgs {
    #[command(subcommand)]
    command: PuppetfileCommands,

    ///  Path to puppetfile
    #[arg(global = true, long)]
    puppetfile: Option<String>,
}

#[derive(Debug, Subcommand)]
enum PuppetfileCommands {
    /// Try and load the Puppetfile to verify the syntax is correct
    Check(PuppetfileCheckArgs),
    /// Install all modules from a Puppetfile
    Install(PuppetfileInstallArgs),
    /// Purge unmanaged modules from a Puppetfile managed directory
    Purge(PuppetfilePurgeArgs),
}

#[derive(Debug, Args)]
struct PuppetfileCheckArgs {}

#[derive(Debug, Args)]
struct PuppetfileInstallArgs {
    /// Force locally changed files to be overwitten
    #[arg(long)]
    force: Option<bool>,
    /// A regex to exclude modules from installation. Helpful in CI environments
    #[arg(long, value_name = "REGEX")]
    module_exclude_regex: Option<String>,
    /// Path to install modules to
    #[arg(long, value_name = "PATH")]
    module_dir: Option<String>,
}

#[derive(Debug, Args)]
struct PuppetfilePurgeArgs {}

fn main() {
    let cli = Cli::parse();
    // match &cli.command {
    //     Commands::Deploy => {
    //         println!("deploy")
    //     }
    //     Commands::Puppetfile => {
    //         println!("puppetfile")
    //     }
    //     Commands::Version => {}
    // }
}
