use crate::AppWebContext;
use sapper::{
  Module as SapperModule, Request, Response, Result as SapperResult, Router as SapperRouter,
};
use sapper_std::*;

use crate::dataservice::article::Article;
use crate::dataservice::section::Section;
use crate::envconfig;
use crate::AppUser;

pub struct HomePage;

impl HomePage {
  pub fn index(req: &mut Request) -> SapperResult<Response> {
    let mut web = get_ext_owned!(req, AppWebContext).unwrap();

    let napp = envconfig::get_int_item("NUMBER_ARTICLE_PER_PAGE");
    let articles = Article::get_latest_articles(napp);

    let reply_articles = Article::get_latest_reply_articles(napp);

    let blog_articles = Article::get_latest_blog_articles(napp);

    // get all configured index displaying sections
    // and latest commented three articles
    let sections = Section::forum_sections();

    web.insert("articles", &articles);
    web.insert("reply_articles", &reply_articles);
    web.insert("blog_articles", &blog_articles);
    web.insert("sections", &sections);

    match get_ext!(req, AppUser) {
      Some(user) => {
        web.insert("user", &user);
      }
      None => {}
    }

    res_html!("theme/pages/home/index.html", web)
  }

  pub fn get_by_id(req: &mut Request) -> SapperResult<Response> {
    let web = get_ext_owned!(req, AppWebContext).unwrap();

    res_html!("theme/pages/home/detail.html", web)
  }
}

impl SapperModule for HomePage {
  fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
    router.get("/theme", Self::index);
    router.get("/theme/details/:id", Self::get_by_id);
    Ok(())
  }
}
