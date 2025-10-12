use ctor::ctor;
use w_macro::{repository, repository_factory};

use crate::domain::content::{Article, ArticleRepository};

repository!(Article);

impl ArticleRepository for DefaultArticleRepository {}

repository_factory!(Article);

#[ctor]
fn init_article_repositoty() {
    unsafe {
        let r = &raw const crate::domain::content::ARTICLE_REPOSITORY
            as *mut Option<std::sync::LazyLock<anyhow::Result<Box<dyn ArticleRepository>>>>;
        r.replace(Some(std::sync::LazyLock::new(article)));
    }
}
