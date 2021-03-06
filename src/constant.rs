#![allow(dead_code)]

pub enum Modes {
    Development(log::Level),
    Release,
}

pub const ITEMS_PER_PAGE: u64 = 12;
pub const MODE: Modes = Modes::Release;

// the Default value
//pub const SUBPATH: &str = "/";
pub const SUBPATH: &str = "demo/";

// site info
pub const ADMIN: &str = "Bruce Yuan";
pub const SITE_NAME: &str = "Math For Fun";
pub const LOGO_PIC: &str = "/assets/logo.png";
pub const AVATR_PIC: &str = "/assets/github.png";
pub const SITE_DESCRIPTION: &str = "Welcome everybody, glad to see you here!
Here are awesome blogs of my website. 
enjoy yourself here.
  ";

// Note That
// the string is line-separated
// each line is a key-value pair
pub const USER_INFO: &str = "
github: https://github.com/homelyguy/,
email: bruceyuan0@protonmail.com,
social_platform: https://yourlink.here/
";
// or the alternative
pub const OTHER_USER_INFO: &str = "github: https://github.com/homelyguy/,\nemail: bruceyuan0@protonmail.com,\nsocial_platform: https://yourlink.here/,";
