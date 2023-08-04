use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Commandline interface that controls turing-pi's BMC. An ethernet
/// connection to the board is required in order for this tool to function. All
/// commands are persisted by the BMC. Please be aware that if no hostname is
/// specified, it will try to resolve the hostname by testing a predefined sequence
/// of options.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(
        help = "Optional turing-pi host to connect to. Host will be determind given the following order:
1. Explicitly passed via the Cli
2. Using hostname 'turing-pi.local'
3. First host to respond to redfish service discovery
"
    )]
    #[arg(default_value = "turingpi.local", long, global = true)]
    pub host: Option<String>,

    #[arg(short, name = "gen completion", exclusive = true)]
    pub gencompletion: Option<clap_complete::shells::Shell>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Power or reset specific nodes.
    Power(PowerArgs),
    /// Change the USB device/host configuration. The USB-bus can only be routed to one
    /// node simultaniously.
    Usb(UsbArgs),
    /// Upgrade the firmware of the BMC
    Firmware(FirmwareArgs),
    /// Flash a given node
    Flash(FlashArgs),
    /// configure the on-board ethernet switch.
    Eth(EthArgs),
    /// [deprecated] forward a uart command or get a line from the serial.
    Uart(UartArgs),
}

#[derive(ValueEnum, Clone, PartialEq)]
pub enum GetSet {
    Get,
    Set,
}

#[derive(ValueEnum, Clone, PartialEq)]
pub enum UsbCmd {
    Device,
    Host,
    Status,
}

#[derive(ValueEnum, Clone, PartialEq)]
pub enum PowerCmd {
    On,
    Off,
    Reset,
    Get,
}

#[derive(Args, Clone)]
pub struct EthArgs {
    /// reset ethernet switch
    #[arg(short, long)]
    pub reset: bool,
}

#[derive(Args)]
pub struct UartArgs {
    pub action: GetSet,
    /// [possible values: 1-4], Not specifying a node
    /// selects all nodes.
    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u8).range(1..5))]
    pub node: u8,
    #[arg(short, long)]
    pub cmd: Option<String>,
}

#[derive(Args, Clone)]
pub struct UsbArgs {
    /// specify which mode to set the given node in.
    #[arg(short, long)]
    pub mode: UsbCmd,
    // /// instead of USB-A, route usb-bus to the BMC chip.
    #[arg(short, long)]
    pub bmc: bool,
    /// Set the boot pin, referred to as 'rpiboot pin' high
    #[arg(short, long)]
    pub usb_boot: bool,
    /// [possible values: 1-4], Not specifying a node
    /// selects all nodes.
    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u8).range(1..5))]
    pub node: u8,
}

#[derive(Args, Clone)]
pub struct FirmwareArgs {
    #[arg(short, long)]
    pub file: PathBuf,
}

#[derive(Args, Clone)]
#[group(required = true)]
pub struct FlashArgs {
    /// Update a node with an image local on the disk.
    #[arg(short, long)]
    pub local: bool,
    /// Update a node with the given image.
    #[arg(short, long)]
    pub image_path: PathBuf,
    /// [possible values: 1-4], Not specifying a node
    /// selects all nodes.
    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u8).range(1..5))]
    pub node: u8,
}

#[derive(Args)]
pub struct PowerArgs {
    // specify command
    pub cmd: PowerCmd,
    /// [possible values: 1-4], Not specifying a node
    /// selects all nodes.
    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u8).range(1..5))]
    pub node: Option<u8>,
}
