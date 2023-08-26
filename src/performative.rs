
#[derive(Debug, PartialEq)]
pub enum Performative {
    Open,
    Begin,
    Attach,
    Flow,
    Transfer,
    Disposition,
    Detach,
    End,
    Close,
}
