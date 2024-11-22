pub enum ConfigProperty {
    // Link Properties
    LinkRedirectHostName,
    LinkRedirectNetworkHost,
    LinkRedirectPort,
    LinkRedirectAddress,

    // Connection Properties
    ConnectionRedirectPort,
    ConnectionRedirectHostName,
    ConnectionRedirectNetworkHost,
}

pub struct Config {
    link_redirect_host_name: Option<String>,
    link_redirect_network_host: Option<String>,
    link_redirect_port: Option<String>,
    link_redirect_address: Option<String>,
    connection_redirect_port: Option<String>,
    connection_redirect_host_name: Option<String>,
    connection_redirect_network: Option<String>,
}

impl Config {
    pub fn set<T: Into<String>>(&mut self, prop: ConfigProperty, value: T) -> &mut Self {
        match prop {
            ConfigProperty::LinkRedirectHostName => {
                self.link_redirect_host_name = Some(value.into())
            }
            ConfigProperty::LinkRedirectNetworkHost => {
                self.link_redirect_network_host = Some(value.into())
            }
            ConfigProperty::LinkRedirectPort => self.link_redirect_port = Some(value.into()),
            ConfigProperty::LinkRedirectAddress => self.link_redirect_address = Some(value.into()),
            ConfigProperty::ConnectionRedirectPort => {
                self.connection_redirect_port = Some(value.into())
            }
            ConfigProperty::ConnectionRedirectHostName => {
                self.connection_redirect_host_name = Some(value.into())
            }
            ConfigProperty::ConnectionRedirectNetworkHost => {
                self.connection_redirect_network = Some(value.into())
            }
        }
        self
    }

    pub fn get(&self, prop: ConfigProperty) -> Option<&String> {
        match prop {
            ConfigProperty::LinkRedirectHostName => &self.link_redirect_host_name,
            ConfigProperty::LinkRedirectNetworkHost => &self.link_redirect_network_host,
            ConfigProperty::LinkRedirectPort => &self.link_redirect_port,
            ConfigProperty::LinkRedirectAddress => &self.link_redirect_address,
            ConfigProperty::ConnectionRedirectPort => &self.connection_redirect_port,
            ConfigProperty::ConnectionRedirectHostName => &self.connection_redirect_host_name,
            ConfigProperty::ConnectionRedirectNetworkHost => &self.connection_redirect_network,
        }
        .as_ref()
    }
}
