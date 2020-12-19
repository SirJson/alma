use super::markers::BlockDevice;
use crate::{process::CommandExt, tool::Tool};
use anyhow::Context;

#[derive(Debug, Clone, Copy)]
pub enum FilesystemType {
    Ext4,
    Vfat,
    F2FS
}

impl FilesystemType {
    pub fn to_mount_type(self) -> &'static str {
        match self {
            FilesystemType::Ext4 => "ext4",
            FilesystemType::Vfat => "vfat",
            FilesystemType::F2FS => "f2fs"
        }
    }
}

impl std::str::FromStr for FilesystemType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let clear_user_arg = s.to_owned().trim().to_lowercase();
        match clear_user_arg.as_str() {
            "ext4" => Ok(FilesystemType::Ext4),
            "f2fs" => Ok(FilesystemType::F2FS),
            _ =>  Err(anyhow::anyhow!("{} is not supported or was not understood.", clear_user_arg))
        }
    }
}

#[derive(Debug)]
pub struct Filesystem<'a> {
    fs_type: FilesystemType,
    block: &'a dyn BlockDevice,
}

impl<'a> Filesystem<'a> {
    pub fn format(
        block: &'a dyn BlockDevice,
        fs_type: FilesystemType,
        mkfs: &Tool,
    ) -> anyhow::Result<Self> {
        let mut command = mkfs.execute();
        match fs_type {
            FilesystemType::Ext4 => command.arg("-F").arg(block.path()),
            FilesystemType::Vfat => command.arg("-F32").arg(block.path()),
            FilesystemType::F2FS => command.arg("-f").arg(block.path())
        };

        command.run().context("Error formatting filesystem")?;

        Ok(Self { fs_type, block })
    }

    pub fn from_partition(block: &'a dyn BlockDevice, fs_type: FilesystemType) -> Self {
        Self { fs_type, block }
    }

    pub fn block(&self) -> &dyn BlockDevice {
        self.block
    }

    pub fn fs_type(&self) -> FilesystemType {
        self.fs_type
    }
}
