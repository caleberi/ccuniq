#[derive(Debug, PartialEq, Eq)]
pub struct RepeatedOpt<'a> {
    pub condition: bool,
    pub value: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RepeatedTypeOpt<'a> {
    Value(RepeatedOpt<'a>),
}

pub const NONE_REPEATED_TYPE_OPT: RepeatedTypeOpt = RepeatedTypeOpt::Value(RepeatedOpt {
    condition: true,
    value: "none",
});

pub const PREPEND_REPEATED_TYPE_OPT: RepeatedTypeOpt = RepeatedTypeOpt::Value(RepeatedOpt {
    condition: true,
    value: "prepend",
});

pub const SEPERATE_REPEATED_TYPE_OPT: RepeatedTypeOpt = RepeatedTypeOpt::Value(RepeatedOpt {
    condition: true,
    value: "seperate",
});
