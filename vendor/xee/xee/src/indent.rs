use std::path::PathBuf;

use clap::Parser;

use crate::format;

#[derive(Debug, Parser)]
pub(crate) struct Indent {
    /// input xml file (default stdin)
    infile: Option<PathBuf>,
    /// output xml file (default stdout)
    outfile: Option<PathBuf>,
    /// Element name to exclude from indentation (can be repeated)
    /// To specify a namespaced element, use Q{namespace}name
    /// If --indent is not specified, using this option is an error.
    #[arg(long)]
    suppress_indent: Vec<String>,
    /// Element name to output as a CDATA section (can be repeated)
    /// To specify a namespaced element, use Q{namespace}name
    #[arg(long)]
    cdata_element: Vec<String>,
    /// doctype public identifier.
    /// A system identifier has to be specified as well, otherwise this is an
    /// error.
    #[arg(long)]
    doctype_public: Option<String>,
    /// doctype system identifier.
    /// Can be used by itself or with --doctype-public.
    #[arg(long)]
    doctype_system: Option<String>,
    /// Output the XML declaration (without encoding).
    #[arg(long)]
    declaration: bool,
    /// Encoding for the XML declaration
    /// If not specified, the encoding is UTF-8.
    /// Implies --declaration.
    #[arg(long)]
    declaration_encoding: Option<String>,
    /// Standalone declaration for the XML declaration
    /// If not specified, the standalone declaration is omitted.
    /// Implies --declaration. Can be used in combination with
    /// --declaration-encoding.
    #[arg(long)]
    declaration_standalone: Option<bool>,
    /// Escape gt (>) characters in text content. By default this is false.
    #[arg(long)]
    escape_gt: bool,
}

impl Indent {
    pub(crate) fn run(&self) -> anyhow::Result<()> {
        let format = format::Format {
            indent: true,
            infile: self.infile.clone(),
            outfile: self.outfile.clone(),
            suppress_indent: self.suppress_indent.clone(),
            cdata_element: self.cdata_element.clone(),
            doctype_public: self.doctype_public.clone(),
            doctype_system: self.doctype_system.clone(),
            declaration: self.declaration,
            declaration_encoding: self.declaration_encoding.clone(),
            declaration_standalone: self.declaration_standalone,
            escape_gt: self.escape_gt,
        };
        format.run()
    }
}
