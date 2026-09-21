use std::fmt;

/// Output formats supported by Conduit's CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Human,
    Compact,
    Json,
    Toml,
    Csv,
    Plain,
}

impl OutputFormat {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Compact => "compact",
            Self::Json => "json",
            Self::Toml => "toml",
            Self::Csv => "csv",
            Self::Plain => "plain",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "human" | "pretty" => Some(Self::Human),
            "compact" => Some(Self::Compact),
            "json" => Some(Self::Json),
            "toml" => Some(Self::Toml),
            "csv" => Some(Self::Csv),
            "plain" | "text" => Some(Self::Plain),
            _ => None,
        }
    }

    pub fn is_structured(&self) -> bool {
        matches!(self, Self::Json | Self::Toml | Self::Csv)
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Human
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// Output color policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorOutputMode {
    Auto,
    Always,
    Never,
}

impl ColorOutputMode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Always => "always",
            Self::Never => "never",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "auto" => Some(Self::Auto),
            "always" | "on" => Some(Self::Always),
            "never" | "off" => Some(Self::Never),
            _ => None,
        }
    }
}

impl Default for ColorOutputMode {
    fn default() -> Self {
        Self::Auto
    }
}

impl fmt::Display for ColorOutputMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A parsed output configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputOptions {
    format: OutputFormat,
    color: ColorOutputMode,
    headers: bool,
    pretty: bool,
    quiet: bool,
    verbose: bool,
    paging: bool,
    width: Option<usize>,
    output_path: Option<String>,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            format: OutputFormat::Human,
            color: ColorOutputMode::Auto,
            headers: true,
            pretty: true,
            quiet: false,
            verbose: false,
            paging: false,
            width: None,
            output_path: None,
        }
    }
}

impl OutputOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format(&self) -> OutputFormat {
        self.format
    }

    pub fn color(&self) -> ColorOutputMode {
        self.color
    }

    pub fn headers(&self) -> bool {
        self.headers
    }

    pub fn pretty(&self) -> bool {
        self.pretty
    }

    pub fn quiet(&self) -> bool {
        self.quiet
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn paging(&self) -> bool {
        self.paging
    }

    pub fn width(&self) -> Option<usize> {
        self.width
    }

    pub fn output_path(&self) -> Option<&str> {
        self.output_path.as_deref()
    }

    pub fn set_format(&mut self, format: OutputFormat) {
        self.format = format;
    }

    pub fn set_color(&mut self, color: ColorOutputMode) {
        self.color = color;
    }

    pub fn set_headers(&mut self, headers: bool) {
        self.headers = headers;
    }

    pub fn set_pretty(&mut self, pretty: bool) {
        self.pretty = pretty;
    }

    pub fn set_quiet(&mut self, quiet: bool) {
        self.quiet = quiet;
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn set_paging(&mut self, paging: bool) {
        self.paging = paging;
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = Some(width);
    }

    pub fn clear_width(&mut self) {
        self.width = None;
    }

    pub fn set_output_path<S>(&mut self, output_path: S)
    where
        S: Into<String>,
    {
        self.output_path = Some(output_path.into());
    }

    pub fn should_use_color(&self, is_terminal: bool) -> bool {
        match self.color {
            ColorOutputMode::Always => true,
            ColorOutputMode::Never => false,
            ColorOutputMode::Auto => is_terminal,
        }
    }

    pub fn should_print_headers(&self) -> bool {
        self.headers && !self.quiet && !matches!(self.format, OutputFormat::Plain)
    }

    pub fn should_page(&self, is_terminal: bool) -> bool {
        self.paging && is_terminal && !self.quiet
    }
}

/// A single CLI output field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputField {
    name: String,
    value: String,
}

impl OutputField {
    pub fn new<S1, S2>(name: S1, value: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

/// A simple row used by human, CSV, and structured renderers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputRow {
    fields: Vec<OutputField>,
}

impl OutputRow {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
        }
    }

    pub fn with_field<S1, S2>(mut self, name: S1, value: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.add_field(name, value);
        self
    }

    pub fn add_field<S1, S2>(&mut self, name: S1, value: S2)
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.fields.push(OutputField::new(name, value));
    }

    pub fn fields(&self) -> &[OutputField] {
        &self.fields
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl Default for OutputRow {
    fn default() -> Self {
        Self::new()
    }
}

/// A collection of rows ready for formatting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputTable {
    columns: Vec<String>,
    rows: Vec<OutputRow>,
}

impl OutputTable {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn set_columns<I, S>(&mut self, columns: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.columns = columns.into_iter().map(Into::into).collect();
    }

    pub fn add_column<S>(&mut self, column: S)
    where
        S: Into<String>,
    {
        self.columns.push(column.into());
    }

    pub fn add_row(&mut self, row: OutputRow) {
        self.rows.push(row);
    }

    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    pub fn rows(&self) -> &[OutputRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

impl Default for OutputTable {
    fn default() -> Self {
        Self::new()
    }
}

/// A result that can be rendered by the CLI output subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputValue {
    Empty,
    Text(String),
    Table(OutputTable),
    Error(String),
}

impl OutputValue {
    pub fn text<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self::Text(value.into())
    }

    pub fn error<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self::Error(value.into())
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

/// Stateless output formatting helpers.
pub struct OutputFormatter;

impl OutputFormatter {
    pub fn format(value: &OutputValue, options: &OutputOptions) -> String {
        match value {
            OutputValue::Empty => String::new(),
            OutputValue::Text(text) => text.clone(),
            OutputValue::Error(error) => {
                if options.quiet {
                    String::new()
                } else {
                    format!("error: {error}")
                }
            }
            OutputValue::Table(table) => Self::format_table(table, options),
        }
    }

    pub fn format_table(table: &OutputTable, options: &OutputOptions) -> String {
        match options.format() {
            OutputFormat::Plain => Self::format_plain(table),
            OutputFormat::Compact => Self::format_compact(table),
            OutputFormat::Csv => Self::format_csv(table),
            OutputFormat::Json => Self::format_json(table, options.pretty()),
            OutputFormat::Toml => Self::format_toml(table),
            OutputFormat::Human => Self::format_human(table, options),
        }
    }

    fn format_plain(table: &OutputTable) -> String {
        table
            .rows()
            .iter()
            .map(|row| {
                row.fields()
                    .iter()
                    .map(|field| field.value())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_compact(table: &OutputTable) -> String {
        table
            .rows()
            .iter()
            .map(|row| {
                row.fields()
                    .iter()
                    .map(|field| format!("{}={}", field.name(), field.value()))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_csv(table: &OutputTable) -> String {
        let mut lines = Vec::new();

        if !table.columns().is_empty() {
            lines.push(
                table
                    .columns()
                    .iter()
                    .map(|column| Self::csv_escape(column))
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }

        for row in table.rows() {
            lines.push(
                row.fields()
                    .iter()
                    .map(|field| Self::csv_escape(field.value()))
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }

        lines.join("\n")
    }

    fn format_json(table: &OutputTable, pretty: bool) -> String {
        let rows = table
            .rows()
            .iter()
            .map(|row| {
                let fields = row
                    .fields()
                    .iter()
                    .map(|field| {
                        format!(
                            "\"{}\":\"{}\"",
                            Self::json_escape(field.name()),
                            Self::json_escape(field.value())
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",");

                format!("{{{fields}}}")
            })
            .collect::<Vec<_>>()
            .join(",");

        if pretty {
            format!("[\n  {}\n]", rows.replace("},{", "},\n  {"))
        } else {
            format!("[{rows}]")
        }
    }

    fn format_toml(table: &OutputTable) -> String {
        table
            .rows()
            .iter()
            .enumerate()
            .map(|(index, row)| {
                let fields = row
                    .fields()
                    .iter()
                    .map(|field| {
                        format!(
                            "{} = \"{}\"",
                            field.name(),
                            Self::json_escape(field.value())
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                format!("[[row_{index}]]\n{fields}")
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    fn format_human(table: &OutputTable, options: &OutputOptions) -> String {
        if table.is_empty() {
            return String::new();
        }

        let mut lines = Vec::new();

        if options.should_print_headers() && !table.columns().is_empty() {
            lines.push(table.columns().join("  "));
        }

        for row in table.rows() {
            lines.push(
                row.fields()
                    .iter()
                    .map(|field| field.value().to_string())
                    .collect::<Vec<_>>()
                    .join("  "),
            );
        }

        lines.join("\n")
    }

    fn csv_escape(value: &str) -> String {
        if value.contains(',') || value.contains('"') || value.contains('\n') {
            format!("\"{}\"", value.replace('"', "\"\""))
        } else {
            value.to_string()
        }
    }

    fn json_escape(value: &str) -> String {
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }
}
