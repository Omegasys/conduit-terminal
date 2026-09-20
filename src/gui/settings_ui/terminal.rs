#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorBlinkMode {
    Never,
    Focused,
    Always,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BellStyle {
    None,
    Visual,
    Audible,
    VisualAndAudible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollbarMode {
    Hidden,
    Overlay,
    AlwaysVisible,
}

#[derive(Debug, Clone)]
pub struct TerminalSettings {
    scrollback_lines: usize,
    unlimited_scrollback: bool,
    cursor_blink: CursorBlinkMode,
    cursor_blink_interval_ms: u64,
    bell_style: BellStyle,
    audible_bell_volume: f32,
    scrollbar_mode: ScrollbarMode,
    mouse_reporting: bool,
    bracketed_paste: bool,
    focus_reporting: bool,
    synchronized_output: bool,
    sixel_enabled: bool,
    kitty_graphics_enabled: bool,
    hyperlinks_enabled: bool,
    clipboard_integration: bool,
    copy_on_select: bool,
    paste_on_middle_click: bool,
    confirm_paste_multiline: bool,
    erase_keystroke: bool,
}

impl TerminalSettings {
    pub fn new() -> Self {
        Self {
            scrollback_lines: 10_000,
            unlimited_scrollback: false,
            cursor_blink: CursorBlinkMode::Focused,
            cursor_blink_interval_ms: 500,
            bell_style: BellStyle::Visual,
            audible_bell_volume: 0.5,
            scrollbar_mode: ScrollbarMode::Overlay,
            mouse_reporting: true,
            bracketed_paste: true,
            focus_reporting: false,
            synchronized_output: true,
            sixel_enabled: true,
            kitty_graphics_enabled: true,
            hyperlinks_enabled: true,
            clipboard_integration: true,
            copy_on_select: false,
            paste_on_middle_click: true,
            confirm_paste_multiline: true,
            erase_keystroke: true,
        }
    }

    pub fn scrollback_lines(&self) -> usize {
        self.scrollback_lines
    }

    pub fn unlimited_scrollback(&self) -> bool {
        self.unlimited_scrollback
    }

    pub fn cursor_blink(&self) -> CursorBlinkMode {
        self.cursor_blink
    }

    pub fn cursor_blink_interval_ms(&self) -> u64 {
        self.cursor_blink_interval_ms
    }

    pub fn bell_style(&self) -> BellStyle {
        self.bell_style
    }

    pub fn audible_bell_volume(&self) -> f32 {
        self.audible_bell_volume
    }

    pub fn scrollbar_mode(&self) -> ScrollbarMode {
        self.scrollbar_mode
    }

    pub fn mouse_reporting(&self) -> bool {
        self.mouse_reporting
    }

    pub fn bracketed_paste(&self) -> bool {
        self.bracketed_paste
    }

    pub fn focus_reporting(&self) -> bool {
        self.focus_reporting
    }

    pub fn synchronized_output(&self) -> bool {
        self.synchronized_output
    }

    pub fn sixel_enabled(&self) -> bool {
        self.sixel_enabled
    }

    pub fn kitty_graphics_enabled(&self) -> bool {
        self.kitty_graphics_enabled
    }

    pub fn hyperlinks_enabled(&self) -> bool {
        self.hyperlinks_enabled
    }

    pub fn clipboard_integration(&self) -> bool {
        self.clipboard_integration
    }

    pub fn copy_on_select(&self) -> bool {
        self.copy_on_select
    }

    pub fn paste_on_middle_click(&self) -> bool {
        self.paste_on_middle_click
    }

    pub fn confirm_paste_multiline(&self) -> bool {
        self.confirm_paste_multiline
    }

    pub fn erase_keystroke(&self) -> bool {
        self.erase_keystroke
    }

    pub fn set_scrollback_lines(&mut self, value: usize) {
        self.scrollback_lines = value.max(100);
    }

    pub fn set_unlimited_scrollback(&mut self, value: bool) {
        self.unlimited_scrollback = value;
    }

    pub fn set_cursor_blink(&mut self, value: CursorBlinkMode) {
        self.cursor_blink = value;
    }

    pub fn set_cursor_blink_interval_ms(&mut self, value: u64) {
        self.cursor_blink_interval_ms = value.clamp(100, 5000);
    }

    pub fn set_bell_style(&mut self, value: BellStyle) {
        self.bell_style = value;
    }

    pub fn set_audible_bell_volume(&mut self, value: f32) {
        self.audible_bell_volume = value.clamp(0.0, 1.0);
    }

    pub fn set_scrollbar_mode(&mut self, value: ScrollbarMode) {
        self.scrollbar_mode = value;
    }

    pub fn set_mouse_reporting(&mut self, value: bool) {
        self.mouse_reporting = value;
    }

    pub fn set_bracketed_paste(&mut self, value: bool) {
        self.bracketed_paste = value;
    }

    pub fn set_focus_reporting(&mut self, value: bool) {
        self.focus_reporting = value;
    }

    pub fn set_synchronized_output(&mut self, value: bool) {
        self.synchronized_output = value;
    }

    pub fn set_sixel_enabled(&mut self, value: bool) {
        self.sixel_enabled = value;
    }

    pub fn set_kitty_graphics_enabled(&mut self, value: bool) {
        self.kitty_graphics_enabled = value;
    }

    pub fn set_hyperlinks_enabled(&mut self, value: bool) {
        self.hyperlinks_enabled = value;
    }

    pub fn set_clipboard_integration(&mut self, value: bool) {
        self.clipboard_integration = value;
    }

    pub fn set_copy_on_select(&mut self, value: bool) {
        self.copy_on_select = value;
    }

    pub fn set_paste_on_middle_click(&mut self, value: bool) {
        self.paste_on_middle_click = value;
    }

    pub fn set_confirm_paste_multiline(&mut self, value: bool) {
        self.confirm_paste_multiline = value;
    }

    pub fn set_erase_keystroke(&mut self, value: bool) {
        self.erase_keystroke = value;
    }
}

impl Default for TerminalSettings {
    fn default() -> Self {
        Self::new()
    }
}
