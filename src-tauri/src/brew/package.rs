use std::process::Command;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PackageType {
    Formula,
    Cask,
}

impl PackageType {
    pub fn flag(&self) -> &'static str {
        match self {
            Self::Formula => "--formula",
            Self::Cask => "--cask",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InstalledPackage {
    name: String,
    package_type: PackageType,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PackageCreateError {
    #[error("The package was not found")]
    NotFound,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum InstalledPackageCreateError {
    #[error("The package couldn't create because {0}")]
    CreateError(#[from] PackageCreateError),
    #[error("The package was not installed")]
    NotInstalled,
}

impl InstalledPackage {
    pub fn new_without_check(name: &str, package_type: PackageType) -> Self {
        Self {
            name: name.to_string(),
            package_type,
        }
    }

    // pub fn new(
    //     name: String,
    //     package_type: Option<PackageType>,
    // ) -> Result<Self, InstalledPackageCreateError> {
    //     let installed_list = super::list(package_type);

    //     installed_list.into_iter().find()
    // }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NotInstalledPackage {
    name: String,
    package_type: PackageType,
}

impl NotInstalledPackage {
    pub fn new_without_check(name: &str, package_type: PackageType) -> Self {
        Self {
            name: name.to_string(),
            package_type,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct BrewInfoJson {
    formulae: Vec<serde_json::Map<String, serde_json::Value>>,
    casks: Vec<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Package {
    Installed(InstalledPackage),
    NotInstalled(NotInstalledPackage),
}

impl Package {
    fn new(name: &str, package_type: Option<PackageType>) -> Result<Self, PackageCreateError> {
        let output = Command::new("brew")
            .args(match package_type {
                None => vec!["info", "--json=v2", name],
                Some(package_type) => vec!["info", package_type.flag(), "--json=v2", &name],
            })
            .output()
            .unwrap();

        if !output.status.success() {
            return Err(PackageCreateError::NotFound);
        }

        let s = String::from_utf8(output.stdout).unwrap();

        let json: BrewInfoJson = serde_json::from_str(&s).unwrap();

        let is_installed;
        let package_type;

        if let Some(info) = json.formulae.first() {
            package_type = PackageType::Formula;
            is_installed = !info
                .get("installed")
                .unwrap()
                .as_array()
                .unwrap()
                .is_empty();
        } else if let Some(info) = json.casks.first() {
            package_type = PackageType::Cask;
            is_installed = info.get("installed").unwrap().is_string();
        } else {
            unreachable!()
        }

        if is_installed {
            Ok(Package::Installed(InstalledPackage::new_without_check(
                name,
                package_type,
            )))
        } else {
            Ok(Package::NotInstalled(
                NotInstalledPackage::new_without_check(name, package_type),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    mod package_new {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn installed_formula() {
            assert_eq!(
                Package::new("julia", Some(PackageType::Formula)),
                Ok(Package::Installed(InstalledPackage::new_without_check(
                    "julia",
                    PackageType::Formula
                )))
            );
        }

        #[test]
        fn installed_cask() {
            assert_eq!(
                Package::new("brave-browser", Some(PackageType::Cask)),
                Ok(Package::Installed(InstalledPackage::new_without_check(
                    "brave-browser",
                    PackageType::Cask
                )))
            );
        }

        #[test]
        fn installed_auto_formula() {
            assert_eq!(
                Package::new("pyenv", None),
                Ok(Package::Installed(InstalledPackage::new_without_check(
                    "pyenv",
                    PackageType::Formula
                )))
            )
        }

        #[test]
        fn installed_auto_cask() {
            assert_eq!(
                Package::new("warp", None),
                Ok(Package::Installed(InstalledPackage::new_without_check(
                    "warp",
                    PackageType::Cask
                )))
            )
        }

        #[test]
        fn notinstalled_formula() {
            assert_eq!(
                Package::new("rustls-ffi", Some(PackageType::Formula)),
                Ok(Package::NotInstalled(
                    NotInstalledPackage::new_without_check("rustls-ffi", PackageType::Formula)
                ))
            )
        }

        #[test]
        fn notinstalled_cask() {
            assert_eq!(
                Package::new("julia", Some(PackageType::Cask)),
                Ok(Package::NotInstalled(
                    NotInstalledPackage::new_without_check("julia", PackageType::Cask)
                ))
            );
        }

        #[test]
        fn notinstalled_auto_formula() {
            assert_eq!(
                Package::new("uv", None),
                Ok(Package::NotInstalled(NotInstalledPackage::new_without_check(
                    "uv",
                    PackageType::Formula
                )))
            )
        }

        #[test]
        fn notinstalled_auto_cask() {
            assert_eq!(
                Package::new("mactex", None),
                Ok(Package::NotInstalled(NotInstalledPackage::new_without_check(
                    "mactex",
                    PackageType::Cask
                )))
            )
        }
    }
}
