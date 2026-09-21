use std::sync::{Arc, Mutex};

use super::history::ClipboardHistory;
use super::large_paste::{LargePasteConfig, LargePasteHandler};
use super::paste_protection::{PasteDecision, PasteProtection};
use super::security::{ClipboardPermission, ClipboardSecurityPolicy};
use super::sensitive_data::SensitiveDataDetector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClipboardType {
    Clipboard,
    Primary,
}

pub trait ClipboardProvider: Send + Sync {
    fn read(&self, clipboard_type: ClipboardType) -> Result<String, String>;
    fn write(&self, clipboard_type: ClipboardType, text: &str) -> Result<(), String>;
    fn clear(&self, clipboard_type: ClipboardType) -> Result<(), String>;
    fn available(&self, clipboard_type: ClipboardType) -> bool;
}

#[derive(Clone)]
pub struct ClipboardManager {
    provider: Arc<dyn ClipboardProvider>,
    history: Arc<Mutex<ClipboardHistory>>,
    paste_protection: PasteProtection,
    sensitive_detector: SensitiveDataDetector,
    security_policy: ClipboardSecurityPolicy,
    large_paste: LargePasteHandler,
}

impl ClipboardManager {
    pub fn new(provider: Arc<dyn ClipboardProvider>) -> Self {
        Self {
            provider,
            history: Arc::new(Mutex::new(ClipboardHistory::new())),
            paste_protection: PasteProtection::default(),
            sensitive_detector: SensitiveDataDetector::default(),
            security_policy: ClipboardSecurityPolicy::default(),
            large_paste: LargePasteHandler::new(LargePasteConfig::default()),
        }
    }

    pub fn read(&self, clipboard_type: ClipboardType) -> Result<String, String> {
        self.check_permission(ClipboardPermission::Read)?;

        self.provider.read(clipboard_type)
    }

    pub fn write(
        &self,
        clipboard_type: ClipboardType,
        text: impl Into<String>,
    ) -> Result<(), String> {
        self.check_permission(ClipboardPermission::Write)?;

        let text = text.into();

        if self.sensitive_detector.detect(&text).is_sensitive() {
            self.check_permission(ClipboardPermission::WriteSensitive)?;
        }

        self.provider.write(clipboard_type, &text)?;

        if let Ok(mut history) = self.history.lock() {
            history.push(text);
        }

        Ok(())
    }

    pub fn paste(
        &self,
        clipboard_type: ClipboardType,
    ) -> Result<PasteDecision, String> {
        self.check_permission(ClipboardPermission::Read)?;

        let text = self.provider.read(clipboard_type)?;

        let sensitive = self.sensitive_detector.detect(&text);

        if sensitive.is_sensitive() {
            self.check_permission(ClipboardPermission::PasteSensitive)?;
        }

        let decision = self.paste_protection.check(&text);

        if !decision.is_allowed() {
            return Ok(decision);
        }

        let large_paste = self.large_paste.prepare(&text);

        Ok(large_paste)
    }

    pub fn clear(&self, clipboard_type: ClipboardType) -> Result<(), String> {
        self.check_permission(ClipboardPermission::Clear)?;

        self.provider.clear(clipboard_type)
    }

    pub fn history(&self) -> Arc<Mutex<ClipboardHistory>> {
        Arc::clone(&self.history)
    }

    pub fn security_policy(&self) -> &ClipboardSecurityPolicy {
        &self.security_policy
    }

    pub fn security_policy_mut(&mut self) -> &mut ClipboardSecurityPolicy {
        &mut self.security_policy
    }

    pub fn paste_protection(&self) -> &PasteProtection {
        &self.paste_protection
    }

    pub fn paste_protection_mut(&mut self) -> &mut PasteProtection {
        &mut self.paste_protection
    }

    pub fn large_paste(&self) -> &LargePasteHandler {
        &self.large_paste
    }

    pub fn set_large_paste_config(&mut self, config: LargePasteConfig) {
        self.large_paste = LargePasteHandler::new(config);
    }

    fn check_permission(&self, permission: ClipboardPermission) -> Result<(), String> {
        if self.security_policy.allows(permission) {
            Ok(())
        } else {
            Err(format!(
                "clipboard operation denied by security policy: {:?}",
                permission
            ))
        }
    }
}
