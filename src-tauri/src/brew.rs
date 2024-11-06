use std::process::Command;

use package::{InstalledPackage, PackageType};

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
        let formula_list = list(Some(PackageType::Formula));

        let cask_list = list(Some(PackageType::Cask));

        [formula_list, cask_list].concat()
    }
}
