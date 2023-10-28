use game::app::Application;
use gobs_game as game;

use blobber;
use blobber::app::App;

fn main() {
    blobber::init_logger();

    Application::new().run::<App>();
}
