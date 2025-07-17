use std::{
    fs::File,
    io::{BufReader, Lines},
    iter::Peekable,
};

pub type PeekableFileReader = Peekable<Lines<BufReader<File>>>;

#[derive(Eq, Hash, PartialEq, Debug)]
pub(crate) enum Section {
    VersionInformation,
    WellInformation,
    AsciiLogData,
    OtherInformation,
    CurveInformation,
    ParameterInformation,
}

impl Section {
    pub const COUNT: usize = 6;
}

pub struct Token {}
#[allow(non_snake_case)]
impl Token {
    pub fn Colon() -> String {
        return ":".into();
    }
    pub fn Period() -> String {
        return ".".into();
    }
    pub fn Tilde() -> String {
        return "~".into();
    }
    pub fn Comment() -> String {
        return "#".into();
    }
    pub fn AsciiSection() -> String {
        return "~A".into();
    }
    pub fn VersionInformationSection() -> String {
        return "~V".into();
    }
    pub fn WellInformationSection() -> String {
        return "~W".into();
    }
    pub fn ParameterInformationSection() -> String {
        return "~P".into();
    }
    pub fn OtherSection() -> String {
        return "~O".into();
    }
    pub fn CurveInformationSection() -> String {
        return "~C".into();
    }
}
