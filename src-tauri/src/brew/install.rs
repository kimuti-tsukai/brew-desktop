use std::{fmt::Display, process::Command};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::package::{InstalledPackage, NotInstalledPackage};

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct InstallError {
    msg: String,
}

impl Display for InstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl NotInstalledPackage {
    pub fn install(self) -> Result<InstalledPackage, InstallError> {
        let output = Command::new("brew")
            .args(["install", self.package_type.flag(), &self.name])
            .output()
            .unwrap();

        if output.status.success() {
            Ok(InstalledPackage::new_without_check(
                &self.name,
                self.package_type,
            ))
        } else {
            let stderr = String::from_utf8(output.stderr).unwrap();

            Err(InstallError { msg: stderr })
        }
    }
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct UnInstallError {
    msg: String,
}

impl Display for UnInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct ReInstallError {
    msg: String,
}

impl Display for ReInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl InstalledPackage {
    pub fn uninstall(self) -> Result<NotInstalledPackage, UnInstallError> {
        let output = Command::new("brew")
            .args(["uninstall", self.package_type.flag(), &self.name])
            .output()
            .unwrap();

        if output.status.success() {
            Ok(NotInstalledPackage::new_without_check(
                &self.name,
                self.package_type,
            ))
        } else {
            let stderr = String::from_utf8(output.stderr).unwrap();

            Err(UnInstallError { msg: stderr })
        }
    }

    pub fn reinstall(self) -> Result<Self, ReInstallError> {
        let output = Command::new("brew")
            .args(["reinstall", self.package_type.flag(), &self.name])
            .output()
            .unwrap();

        if output.status.success() {
            Ok(self)
        } else {
            let stderr = String::from_utf8(output.stderr).unwrap();

            Err(ReInstallError { msg: stderr })
        }
    }
}
