use std::any;

use crate::domain::content::*;

pub fn create_article(model: Article) -> anyhow::Result<Article> {
    let repository = article()?;
    repository.create(model)
}
