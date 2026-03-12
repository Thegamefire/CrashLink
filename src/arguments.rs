use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "CrashLink", about = "A CLI utility that kills a process when a deathlink is sent in archipelago.")]
pub enum Command {
    /// Connect to Archipelago and monitor a process
    Run {
        /// Match processes based on name
        #[structopt(short = "n", long)]
        use_process_name: bool,
        /// The process to kill when receiving a deathlink
        target_process: String,
        /// The host and port of the archipelago server (e.g. archipelago.gg:38281)
        ap_host: String,
        /// The slot to connect to on the archipelago server (e.g. Player1)
        ap_slot: String,

        /// Password for the archipelago server
        #[structopt(short = "p", long, default_value = "")]
        ap_pass: String,

        /// Don't play a sound on death
        #[structopt(short = "s", long)]
        disable_sound: bool
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
    pub disable_sound: bool,
}