use crate::features::health_check::handlers::*;
use crate::features::regions::handlers::*;

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    health_check,
    get_all_regions,
    get_single_region,
    create_region,
    update_region,
    delete_region
))]
pub struct ApiDoc;
