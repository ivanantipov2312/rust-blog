mod index;
mod post;
mod login;
mod register;
mod add_post;
mod fallback;

pub use index::index;
pub use post::{get_post_by_id, get_post_by_slug, get_posts};
pub use login::{login_get, login_post};
pub use register::{register_get, register_post};
pub use add_post::{add_post_get, add_post_post};
pub use fallback::fallback;
