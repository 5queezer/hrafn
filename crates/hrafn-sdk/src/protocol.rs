use crate::prelude::{String, Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Stable protocol version implemented by this SDK crate.
pub const SDK_PROTOCOL_VERSION: &str = "hrafn-plugin-jsonrpc-0.1";

/// The broad kind of extension registered with the Hrafn kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum ExtensionKind {
    Provider,
    Channel,
    Tool,
    Memory,
    Observer,
    Runtime,
    Peripheral,
    Frontend,
}

/// A capability advertised by a plugin or granted by the kernel.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Capability {
    pub name: String,
}

impl Capability {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// A permission requested by a plugin and granted or denied by policy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Permission {
    pub scope: String,
}

impl Permission {
    #[must_use]
    pub fn new(scope: impl Into<String>) -> Self {
        Self {
            scope: scope.into(),
        }
    }
}

/// Minimal manifest returned during plugin handshake.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub kind: ExtensionKind,
    pub protocol: String,
    pub capabilities: Vec<Capability>,
    pub permissions: Vec<Permission>,
}

impl PluginManifest {
    #[must_use]
    pub fn new(name: impl Into<String>, version: impl Into<String>, kind: ExtensionKind) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            kind,
            protocol: SDK_PROTOCOL_VERSION.into(),
            capabilities: Vec::new(),
            permissions: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(Capability::new(capability));
        self
    }

    #[must_use]
    pub fn with_permission(mut self, permission: impl Into<String>) -> Self {
        self.permissions.push(Permission::new(permission));
        self
    }
}

/// Kernel-to-plugin handshake request.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HandshakeRequest {
    pub protocol_version: String,
    pub kernel_version: String,
}

/// Plugin-to-kernel handshake response.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HandshakeResponse {
    pub manifest: PluginManifest,
}
