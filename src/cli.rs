use argh::FromArgs;

#[derive(Debug, Clone, FromArgs, Default)]
#[argh(description = "ysoserial-rs")]
pub struct ConfigArgs {
    /// select a payload
    #[argh(option, short = 'p')]
    pub payload: Option<String>,
    /// command to execute
    #[argh(option, short = 'c')]
    pub command: Option<String>,
    /// url to request dns
    #[argh(option)]
    pub url: Option<String>,
    /// tomcat echo request header name
    #[argh(option)]
    pub echo_name: Option<String>,
    /// tomcat command request header name
    #[argh(option)]
    pub command_name: Option<String>,
    /// save payload to file
    #[argh(option, short = 'o')]
    pub output: Option<String>,
    /// format to hex or base64
    #[argh(option, short = 'f', default = "String::from(\"\")")]
    pub format: String,
    /// list all payload
    #[argh(switch, short = 'l')]
    pub list: bool,
}

impl ConfigArgs {
    pub fn new() -> Self {
        let default: ConfigArgs = argh::from_env();
        default
    }
}
