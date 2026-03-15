use localhost::{
    debug,
    Loader,
};

fn main() {
    let mut mux = match Loader::load("./config/server.toml") {
        Ok(multiplexer) => multiplexer,
        Err(e) => {
            debug!(e);
            return;
        }
    };

    if let Err(e) = mux.register_listeners() {
        debug!(e);
    };

    mux.run()
}
