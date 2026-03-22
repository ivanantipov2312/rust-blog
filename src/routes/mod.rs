mod index;
mod post;
mod login;
mod register;
mod add_post;

pub use index::index;
pub use post::{get_post, get_posts};
pub use login::{login_get, login_post};
pub use register::{register_get, register_post};
pub use add_post::{add_post_get, add_post_post};
