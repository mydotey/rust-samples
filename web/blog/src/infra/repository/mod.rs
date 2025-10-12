use ctor::ctor;
use w_macro::*;

use crate::domain::content::*;

repository!(Article);
impl_repository_trait!(Article);
