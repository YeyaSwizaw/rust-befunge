use std::fmt::{Show, Formatter, Result};

#[deriving(Copy, Clone)]
pub enum ParserError {
    CmdError,
    FileReadError(String),
    FileEmptyError(String),
    UnexpectedChar(int, int, char),
    VarsDisabled,
    OutputError,
    OutputFileError(String)
}

impl Show for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &CmdError => write!(f, "Error parsing command line args"),
            &FileReadError(ref file) => write!(f, "Unable to read file: {}", file),
            &FileEmptyError(ref file) => write!(f, "File is empty: {}", file),
            &UnexpectedChar(x, y, c) => write!(f, "Unexpected char at ({}, {}): {}", x, y, c),

            &VarsDisabled => {
                try!(write!(f, "Using 'p' or 'g' is disabled by default.\n"));
                try!(write!(f, "Pass '--enable-vars' to enable using them for variables.\n"));
                write!(f, "Disabled by default as it potentially allows invalid code.")
            }

            &OutputError => write!(f, "Unable to write output"),
            &OutputFileError(ref file) => write!(f, "Unable to open output file for writing: {}", file)
        }
    }
}
