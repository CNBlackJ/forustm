use crate::AppWebContext;
use log::info;
use sapper::{
  status, Module as SapperModule, Request, Response, Result as SapperResult, Router as SapperRouter,
};
use sapper_std::*;

pub struct SectionPage;

impl SectionPage {
  pub fn index(req: &mut Request) -> SapperResult<Response> {
    let web = get_ext_owned!(req, AppWebContext).unwrap();

    res_html!("theme/pages/section/index.html", web)
  }

  pub fn get_by_id(req: &mut Request) -> SapperResult<Response> {
    let web = get_ext_owned!(req, AppWebContext).unwrap();

    res_html!("theme/pages/section/detail.html", web)
  }
}

impl SapperModule for SectionPage {
  fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
    router.get("/theme/sections", Self::index);
    router.get("/theme/sections/:id", Self::get_by_id);
    Ok(())
  }
}
