use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "{{ project-name }}", version, about)]
pub struct Cli {
    /// Path to config file
    #[arg(long)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show effective configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
{% if persistence ~= 'None' %}    /// Database migration commands
    Migrate {
        #[command(subcommand)]
        action: MigrateAction,
    },
{% endif %}
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Print default configuration
    Defaults,
    /// Print merged configuration
    Show,
}
{% if persistence ~= 'None' %}
#[derive(Subcommand, Debug)]
pub enum MigrateAction {
    /// Run pending migrations
    Up,
    /// Revert the last migration
    Down,
}
{% endif %}
