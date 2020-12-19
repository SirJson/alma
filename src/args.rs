use crate::storage::FilesystemType;
use super::aur::AurHelper;
use byte_unit::Byte;
use std::path::PathBuf;
use structopt::StructOpt;

/// Parse size argument as bytes
/// e.g. 10GB, 10GiB, etc.
fn parse_bytes(src: &str) -> Result<Byte, &'static str> {
    Byte::from_str(src).map_err(|_| "Invalid image size")
}

#[derive(StructOpt)]
#[structopt(name = "alma", about = "Arch Linux Mobile Appliance")]
pub struct App {
    /// Verbose output
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "create", about = "Create a new Arch Linux USB")]
    Create(CreateCommand),

    #[structopt(name = "chroot", about = "Chroot into exiting Live USB")]
    Chroot(ChrootCommand),

    #[structopt(name = "qemu", about = "Boot the USB with Qemu")]
    Qemu(QemuCommand),
}

#[derive(StructOpt)]
pub struct CreateCommand {
    /// Either a path to a removable block device or a nonexiting file if --image is specified
    #[structopt(parse(from_os_str))]
    pub path: Option<PathBuf>,

    /// Additional packages to install
    #[structopt(short = "p", long = "extra-packages", value_name = "package")]
    pub extra_packages: Vec<String>,

    /// Additional packages to install
    #[structopt(long = "aur-packages", value_name = "aurpackage")]
    pub aur_packages: Vec<String>,

    /// Enter interactive chroot before unmounting the drive
    #[structopt(short = "i", long = "interactive")]
    pub interactive: bool,

    /// Encrypt the root partition
    #[structopt(short = "e", long = "encrypted-root")]
    pub encrypted_root: bool,

    /// Request a different FilesystemType for the new root filesystem. Supported are: 'ext4', 'f2fs'. 
    /// You can choose only one format and your input will be parsed case insensitive.
    /// If not set or the argument failed to parse 'ext4' will be used as fallback
    #[structopt(short = "f", long="rootfs")]
    pub rootfs: Option<FilesystemType>,

    /// Path to preset files
    #[structopt(long = "presets", value_name = "preset")]
    pub presets: Vec<PathBuf>,

    /// Create an image with a certain size in the given path instead of using an actual block device
    #[structopt(
        long = "image",
        parse(try_from_str = parse_bytes),
        value_name = "size",
        requires = "path"
    )]
    pub image: Option<Byte>,

    /// Overwrite existing image files. Use with caution!
    #[structopt(long = "overwrite")]
    pub overwrite: bool,

    /// Allow installation on non-removable devices. Use with extreme caution!
    ///
    /// If no device is specified in the command line, the device selection menu will
    /// show non-removable devices
    #[structopt(long = "allow-non-removable")]
    pub allow_non_removable: bool,

    #[structopt(long = "aur-helper", possible_values=&["yay"], default_value="yay")]
    pub aur_helper: AurHelper,
}

#[derive(StructOpt)]
pub struct ChrootCommand {
    /// Path starting with /dev/disk/by-id for the USB drive
    #[structopt(parse(from_os_str))]
    pub block_device: PathBuf,

    /// Allow installation on non-removable devices. Use with extreme caution!
    #[structopt(long = "allow-non-removable")]
    pub allow_non_removable: bool,

    /// If you created an appliance that doesn't use the default rootfs format you need specify your fs type here.
    /// Without using this argument 'ext4' is assumed for your rootfs.
    #[structopt(short = "f", long="rootfs")]
    pub rootfs: FilesystemType,

    /// Optional command to run
    #[structopt()]
    pub command: Vec<String>,
}

#[derive(StructOpt)]
pub struct QemuCommand {
    /// Path starting with /dev/disk/by-id for the USB drive
    #[structopt(parse(from_os_str))]
    pub block_device: PathBuf,

    /// Arguments to pass to qemu
    #[structopt()]
    pub args: Vec<String>,
}
