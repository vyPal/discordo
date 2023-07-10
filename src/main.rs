mod application;

use application::Application;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new();
    app.run()
}
