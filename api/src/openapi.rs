use crate::features::health_check::handlers::*;
use crate::features::regions::handlers::*;
use crate::features::walks::handlers::*;

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    health_check,
    get_all_regions,
    get_single_region,
    create_region,
    update_region,
    delete_region,
    create_walk,
    get_all_walks,
))]
pub struct ApiDoc;
