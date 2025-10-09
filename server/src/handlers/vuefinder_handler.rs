use crate::types::common::AppState;
use crate::types::error::AppError;
use salvo::prelude::*;
use entity::resources;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub q: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VfEntry {
    pub type_: String,
    pub path: String,
    pub basename: String,
    pub size: u64,
    pub timestamp: i64,
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VfListResp {
    pub path: String,
    pub dirs: Vec<VfEntry>,
    pub files: Vec<VfEntry>,
}

/// GET /api/vuefinder/list
#[handler]
pub async fn list(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<Json<VfListResp>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let q = req.parse_queries::<ListQuery>()?;
    let base = q.path.unwrap_or_else(|| "/".to_string());
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    let rows = resources::Entity::find()
        .filter(resources::Column::Path.contains(&base))
        .all(&state.db)
        .await?;
    use std::collections::BTreeSet;
    let mut folder_set: BTreeSet<String> = BTreeSet::new();
    for r in rows {
        let p = normalize(&r.path);
        if p == base {
            files.push(VfEntry {
                type_: "file".into(),
                path: p.clone(),
                basename: r.name.clone(),
                size: 0,
                timestamp: r.updated_at.map(|t| t.timestamp()).unwrap_or(0),
                url: Some(r.url),
            });
        } else if let Some(child) = first_child(&base, &p) {
            folder_set.insert(child);
        }
    }
    for d in folder_set {
        let full = join_path(&base, &d);
        dirs.push(VfEntry {
            type_: "dir".into(),
            path: full.clone(),
            basename: d,
            size: 0,
            timestamp: 0,
            url: None,
        });
    }
    Ok(Json(VfListResp {
        path: base,
        dirs,
        files,
    }))
}

fn normalize(p: &str) -> String {
    let mut s = p.trim().to_string();
    if !s.starts_with('/') {
        s = format!("/{}", s);
    }
    if s.len() > 1 && s.ends_with('/') {
        s.pop();
    }
    s
}
fn first_child(base: &str, path: &str) -> Option<String> {
    let b = normalize(base);
    let p = normalize(path);
    if b == "/" {
        let seg = p.trim_start_matches('/').split('/').next().unwrap_or("");
        return if seg.is_empty() {
            None
        } else {
            Some(seg.to_string())
        };
    }
    let prefix = format!("{}/", b.trim_end_matches('/'));
    if p.starts_with(&prefix) {
        let rest = &p[prefix.len()..];
        let seg = rest.split('/').next().unwrap_or("");
        if !seg.is_empty() {
            return Some(seg.to_string());
        }
    }
    None
}
fn join_path(base: &str, name: &str) -> String {
    if base == "/" {
        format!("/{}", name)
    } else {
        format!("{}/{}", base.trim_end_matches('/'), name)
    }
}
