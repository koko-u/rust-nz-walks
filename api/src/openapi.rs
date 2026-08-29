use crate::features::students::__path_get_all_students;
use crate::routers::__path_index;

#[derive(utoipa::OpenApi)]
#[openapi(paths(index, get_all_students,))]
pub struct ApiDoc;
