use {
    crate::{
        debug,
        mux::Multiplexer,
        server::Server,
        utils::AppResult,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    std::{
        fs::File,
        io::Read,
    },
};

#[derive(Serialize, Deserialize)]
/// Represents the main configuration structure
/// holding a collection of server configurations.
pub struct Config {
    size_limit: Option<u64>,
    servers:    Vec<Server>,
}

impl Config {
    /// Returns the ownership of the
    /// server configurations as an unwraping.
    pub fn servers(self) -> Option<Vec<Server>> {
        if self.servers.is_empty() {
            None
        }
        else {
            Some(self.servers)
        }
    }

    pub fn size_limit(&self) -> Option<u64> { debug!(self.size_limit) }

    /// Removes any invalid server configuration from the list.
    ///
    /// Iterates through the list of servers and remove any server
    /// whose configuration is not valid.
    ///
    /// If a server's configuration is invalid, a warning message
    /// will be printed to the console.
    pub fn clean(&mut self) {
        let mut idx = 1;
        self.servers.retain(|server| {
            if !server.has_valid_config() {
                println!("Invalid Config: Server number {}", idx);
            }
            idx += 1;
            server.has_valid_config()
        })
    }
}

/// Represents the loader structure
/// with no field, only its method.
pub struct Loader;

impl Loader {
    /// Loads the server configuration from a TOML file
    /// and initializes a multiplexer through these steps:
    ///
    /// 1. Reading a configuration from a specified file path.
    /// 2. Parsing the file using the `toml` crate.
    /// 3. Validating the configuration.
    /// 4. Creating a new `Multiplexer` instance based on the parsed
    ///    configuration.
    ///
    /// # Arguments
    ///
    /// * `path`: The path of the TOML configuration file.
    ///
    /// # Returns
    ///
    /// * `Ok(Multiplexer)`: Returns a initialized `Multiplexer` instance
    ///   on success.
    /// * `Err(String)`: Returns an error message string on failure.
    ///
    /// # Errors
    ///
    /// May return an error if:
    /// * The configuration file cannot be opened or read.
    /// * The configuration file contains invalid TOML syntax.
    /// * An error occurs during the creation of the `Multiplexer`.
    pub fn load(path: &str) -> AppResult<Multiplexer> {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let mut config: Config = toml::from_str(&contents)?;
        config.clean();

        let mux = Multiplexer::new(config)?;
        Ok(mux)
    }
}
