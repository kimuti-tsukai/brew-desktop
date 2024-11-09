use std::process::Command;

use install::{InstallError, ReInstallError, UnInstallError};
use package::{InstalledPackage, NotInstalledPackage, Package, PackageType};

pub mod package;

pub mod install;

#[tauri::command]
pub fn list(package_type: Option<PackageType>) -> Vec<InstalledPackage> {
    if let Some(package_type) = package_type {
        let result = Command::new("brew")
            .args(["list", package_type.flag()])
            .output()
            .unwrap();

        let stdout = String::from_utf8(result.stdout).unwrap();

        stdout
            .lines()
            .map(|v| InstalledPackage::new_without_check(v, package_type))
            .collect()
    } else {
        let mut formula_list = list(Some(PackageType::Formula));

        let mut cask_list = list(Some(PackageType::Cask));

        formula_list.append(&mut cask_list);

        formula_list
    }
}

#[tauri::command]
pub fn search(package_name: &str, package_type: Option<PackageType>) -> Vec<Package> {
    if let Some(package_type) = package_type {
        let result = Command::new("brew")
            .args(["search", package_type.flag(), package_name])
            .output()
            .unwrap();

        let stdout = String::from_utf8(result.stdout).unwrap();

        stdout
            .lines()
            .map(|v| Package::new(v, Some(package_type)).unwrap())
            .collect()
    } else {
        let mut formula_list = search(package_name, Some(PackageType::Formula));

        let mut cask_list = search(package_name, Some(PackageType::Cask));

        formula_list.append(&mut cask_list);

        formula_list
    }
}

#[tauri::command]
pub fn install(package: NotInstalledPackage) -> Result<InstalledPackage, InstallError> {
    package.install()
}

#[tauri::command]
pub fn uninstall(package: InstalledPackage) -> Result<NotInstalledPackage, UnInstallError> {
    package.uninstall()
}

#[tauri::command]
pub fn reinstall(package: InstalledPackage) -> Result<InstalledPackage, ReInstallError> {
    package.reinstall()
}
