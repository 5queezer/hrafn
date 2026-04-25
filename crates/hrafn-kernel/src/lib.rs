#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::collections::{BTreeMap, BTreeSet};

use hrafn_sdk::PluginManifest;

/// Permission-aware registry for plugins known to the Hrafn microkernel.
#[derive(Debug, Default)]
pub struct KernelRegistry {
    granted_permissions: BTreeSet<String>,
    plugins: BTreeMap<String, PluginManifest>,
}

impl KernelRegistry {
    #[must_use]
    pub fn new<I, S>(granted_permissions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            granted_permissions: granted_permissions.into_iter().map(Into::into).collect(),
            plugins: BTreeMap::new(),
        }
    }

    /// Register a plugin manifest after enforcing duplicate-name and permission policy.
    ///
    /// This is intentionally small: it is the seed of the future kernel/plugin boundary.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::DuplicatePlugin`] when a plugin with the same name is
    /// already registered. Returns [`RegistryError::PermissionDenied`] when the plugin
    /// requests a permission that has not been granted to the kernel registry.
    pub fn register(&mut self, manifest: PluginManifest) -> Result<(), RegistryError> {
        if self.plugins.contains_key(&manifest.name) {
            return Err(RegistryError::DuplicatePlugin(manifest.name));
        }

        for permission in &manifest.permissions {
            if !self.granted_permissions.contains(&permission.scope) {
                return Err(RegistryError::PermissionDenied {
                    plugin: manifest.name,
                    permission: permission.scope.clone(),
                });
            }
        }

        self.plugins.insert(manifest.name.clone(), manifest);
        Ok(())
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&PluginManifest> {
        self.plugins.get(name)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    DuplicatePlugin(String),
    PermissionDenied { plugin: String, permission: String },
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicatePlugin(name) => write!(f, "plugin {name} is already registered"),
            Self::PermissionDenied { plugin, permission } => {
                write!(
                    f,
                    "plugin {plugin} requested ungranted permission {permission}"
                )
            }
        }
    }
}

impl std::error::Error for RegistryError {}
