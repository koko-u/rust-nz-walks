use crate::features::regions::models as r_models;
use crate::features::regions::repositories as r_repositories;
use crate::features::walks::commands;
use crate::features::walks::validators;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, utoipa::IntoParams, garde::Validate)]
#[into_params(parameter_in = Query)]
pub struct GetWalksQuery {
    /// pagination: page number
    #[serde(default = "default_page")]
    #[garde(range(min = 1))]
    pub page: u32,
    /// pagination: number of items per page
    #[serde(default = "default_per_page")]
    #[garde(range(min = 1))]
    pub per_page: u32,

    /// Filter by name containing the given string.
    #[garde(alphanumeric, length(min = 1))]
    pub name_contains: Option<String>,

    /// Filter by the difficulty
    #[garde(custom(validators::is_optional_difficulty))]
    pub difficulty: Option<String>,

    /// Filter by the region code
    #[garde(length(min = 1))]
    pub region_code: Option<String>,

    /// sort order, comma-separated list of fields (e.g. name,-length), direction is indicated by '-' prefix
    #[garde(length(min = 1))]
    pub sort: Option<String>,
}

fn default_page() -> u32 {
    1
}
fn default_per_page() -> u32 {
    10
}

macro_rules! merge {
    ($($arg:expr),* $(,)?) => {{
        let mut report = garde::Report::new();
        $(
            for (p, e) in $arg.into_inner() {
                report.append(p, e);
            }
        )*
        report
    }};
}
impl GetWalksQuery {
    pub async fn validate_into(self, conn: &sqlx::PgPool) -> Result<commands::QueryCommand, garde::Report> {
        use garde::Validate as _;
        let validated = self.validate();

        let region_id = check_region(self.region_code.as_ref(), conn).await;

        let sort_order = parse_from(self.sort.as_ref());

        let difficulty = self.difficulty.as_ref().map(|d| {
            serde_json::from_value(serde_json::Value::String(d.to_string()))
                .expect("difficulty is a valid string")
        });

        match (validated, region_id, sort_order) {
            (Ok(_), Ok(region_id), Ok(sort_order)) => Ok(commands::QueryCommand {
                page: self.page,
                per_page: self.per_page,
                name_contains: self.name_contains,
                difficulty,
                region_id,
                sort_order,
            }),
            (Err(r1), Ok(_), Ok(_)) => Err(r1),
            (Ok(_), Err(r2), Ok(_)) => Err(r2),
            (Ok(_), Ok(_), Err(r3)) => Err(r3),
            (Err(r1), Err(r2), Ok(_)) => Err(merge!(r1, r2)),
            (Err(r1), Ok(_), Err(r3)) => Err(merge!(r1, r3)),
            (Ok(_), Err(r2), Err(r3)) => Err(merge!(r2, r3)),
            (Err(r1), Err(r2), Err(r3)) => Err(merge!(r1, r2, r3)),
        }
    }
}

async fn check_region(
    region_code: Option<&String>,
    conn: &sqlx::PgPool,
) -> Result<Option<r_models::RegionId>, garde::Report> {
    let Some(region_code) = region_code else {
        return Ok(None);
    };

    let row = r_repositories::get_region_by_code(conn, region_code).await;

    match row {
        Ok(Some(region)) => Ok(Some(r_models::RegionId::from(region.id))),
        Ok(None) => {
            let mut report = garde::Report::new();
            report.append(
                garde::Path::new("region_code"),
                garde::error::Error::new(format!("Region {region_code} not found")),
            );
            Err(report)
        }
        Err(e) => {
            tracing::error!(error = ?e, "sqlx query call failed. validate GetWalksQuery");
            let mut report = garde::Report::new();
            report.append(garde::Path::empty(), garde::error::Error::new("DB error"));
            Err(report)
        }
    }
}

fn parse_from(sort: Option<&String>) -> Result<Vec<commands::Sort>, garde::Report> {
    let Some(sort) = sort else {
        return Ok(vec![]);
    };

    let mut sort_order = vec![];
    for term in sort.split(',') {
        // 前後の空白除去
        let term = term.trim();
        // - で始まる名前は降順、そうでなければ昇順
        let (term, direction) = match term.strip_prefix('-') {
            Some(term) => (term, commands::SortDirection::Desc),
            None => (term, commands::SortDirection::Asc),
        };

        // 名前が SortField に適合するかを確認
        let Ok(field) = term.parse::<commands::SortField>() else {
            let mut report = garde::Report::new();
            report.append(
                garde::Path::empty(),
                garde::error::Error::new(format!("invalid sort field: {term}")),
            );
            return Err(report);
        };

        sort_order.push(commands::Sort { field, direction });
    }

    Ok(sort_order)
}
