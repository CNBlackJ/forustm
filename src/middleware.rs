use crate::envconfig;
use crate::AppUser;
use sapper::{Error as SapperError, Request};
use sapper_std::*;

pub fn permission_need_login(req: &mut Request) -> Result<(), SapperError> {
    let (path, _) = req.uri();
    if path.starts_with("/s/") || path.starts_with("/p/") {
        match get_ext!(req, AppUser) {
            Some(ref _user) => {
                // pass, nothing need to do here
                return Ok(());
            }
            None => {
                return res_400!("No permissions: need login.".to_string());
            }
        }
    } else {
        Ok(())
    }
}

pub fn permission_need_be_admin(req: &mut Request) -> Result<(), SapperError> {
    let (path, _) = req.uri();
    if path.starts_with("/s/") || path.starts_with("/p/") {
        match get_ext!(req, AppUser) {
            Some(user) => {
                if user.role >= 9 {
                    // pass, nothing need to do here
                    return Ok(());
                } else {
                    return res_400!("No permissions: need be admin.".to_string());
                }
            }
            None => {
                return res_400!("No permissions: need login.".to_string());
            }
        }
    } else {
        Ok(())
    }
}

pub fn is_admin(req: &mut Request) -> bool {
    match get_ext!(req, AppUser) {
        Some(user) => {
            if user.role >= 9 {
                true
            } else {
                false
            }
        }
        None => false,
    }
}

pub fn check_cache_switch(req: &mut Request) -> bool {
    if envconfig::get_int_item("CACHE") == 1 && !is_admin(req) {
        true
    } else {
        false
    }
}
