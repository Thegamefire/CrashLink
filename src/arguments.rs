use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "CrashLink", about = "A CLI utility that kills a process when a deathlink is sent in archipelago.")]
pub enum Command {
    /// Connect to Archipelago and monitor a process
    Run {
        /// Matches processes based on name
        #[structopt(short = "n", long)]
        use_process_name: bool,

        target_process: String,
        ap_host: String,
        ap_slot: String,

        /// Password for the archipelago server
        #[structopt(short = "p", long, default_value = "")]
        ap_pass: String,
    },

    /// Lists running processes and exits
    #[structopt(name = "list-processes")]
    ListProcesses,
}



pub struct Config {
    pub use_process_name: bool,
    pub target_process: String,
    pub ap_host: String,
    pub ap_slot: String,
    pub ap_pass: String,
}