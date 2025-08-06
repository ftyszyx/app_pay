use async_trait::async_trait;
use casbin::{
    Adapter, Filter, Model, Result,
    error::{AdapterError, Error as CasbinError},
};
use entity::casbin_rule::{self, Entity as CasbinRule};
use sea_orm::{ActiveValue, ColumnTrait, Condition as SeaOrmCondition, QueryFilter, prelude::*};
use std::io::{Error, ErrorKind};

pub struct CasbinAdapter {
    pool: DbConn,
}

impl CasbinAdapter {
    pub fn new(pool: DbConn) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Adapter for CasbinAdapter {
    async fn load_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        let rules = CasbinRule::find()
            .all(&self.pool)
            .await
            .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;
        for r in rules {
            let rule = vec![r.v0, r.v1, r.v2]
                .into_iter()
                .chain(r.v3.into_iter())
                .chain(r.v4.into_iter())
                .chain(r.v5.into_iter())
                .collect::<Vec<String>>();
            if !m.add_policy("p", &r.ptype, rule) {
                return Err(CasbinError::from(AdapterError(Box::new(Error::new(
                    ErrorKind::Other,
                    "Failed to add policy",
                )))));
            }
        }
        Ok(())
    }

    async fn save_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        self.clear_policy().await?;
        let mut policies_to_add = Vec::new();

        if let Some(ast_map) = m.get_model().get("p") {
            for (ptype, ast) in ast_map {
                for rule in ast.get_policy() {
                    let mut new_rule = casbin_rule::ActiveModel {
                        ptype: ActiveValue::Set(ptype.clone()),
                        ..Default::default()
                    };
                    for (i, val) in rule.iter().enumerate() {
                        match i {
                            0 => new_rule.v0 = ActiveValue::Set(val.clone()),
                            1 => new_rule.v1 = ActiveValue::Set(val.clone()),
                            2 => new_rule.v2 = ActiveValue::Set(val.clone()),
                            3 => new_rule.v3 = ActiveValue::Set(Some(val.clone())),
                            4 => new_rule.v4 = ActiveValue::Set(Some(val.clone())),
                            5 => new_rule.v5 = ActiveValue::Set(Some(val.clone())),
                            _ => {}
                        }
                    }
                    policies_to_add.push(new_rule);
                }
            }
        }
        if let Some(ast_map) = m.get_model().get("g") {
            for (ptype, ast) in ast_map {
                for rule in ast.get_policy() {
                    let mut new_rule = casbin_rule::ActiveModel {
                        ptype: ActiveValue::Set(ptype.clone()),
                        ..Default::default()
                    };
                    for (i, val) in rule.iter().enumerate() {
                        match i {
                            0 => new_rule.v0 = ActiveValue::Set(val.clone()),
                            1 => new_rule.v1 = ActiveValue::Set(val.clone()),
                            _ => {}
                        }
                    }
                    policies_to_add.push(new_rule);
                }
            }
        }

        if !policies_to_add.is_empty() {
            CasbinRule::insert_many(policies_to_add)
                .exec(&self.pool)
                .await
                .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;
        }
        Ok(())
    }

    async fn add_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        let new_rule = casbin_rule::ActiveModel {
            ptype: ActiveValue::Set(ptype.to_owned()),
            v0: ActiveValue::Set(rule.get(0).cloned().unwrap_or_default()),
            v1: ActiveValue::Set(rule.get(1).cloned().unwrap_or_default()),
            v2: ActiveValue::Set(rule.get(2).cloned().unwrap_or_default()),
            v3: ActiveValue::Set(rule.get(3).cloned()),
            v4: ActiveValue::Set(rule.get(4).cloned()),
            v5: ActiveValue::Set(rule.get(5).cloned()),
            ..Default::default()
        };
        CasbinRule::insert(new_rule)
            .exec(&self.pool)
            .await
            .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;
        Ok(true)
    }

    async fn remove_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        let mut condition = SeaOrmCondition::all().add(casbin_rule::Column::Ptype.eq(ptype));
        if let Some(val) = rule.get(0) {
            condition = condition.add(casbin_rule::Column::V0.eq(val));
        }
        if let Some(val) = rule.get(1) {
            condition = condition.add(casbin_rule::Column::V1.eq(val));
        }
        if let Some(val) = rule.get(2) {
            condition = condition.add(casbin_rule::Column::V2.eq(val));
        }
        if let Some(val) = rule.get(3) {
            condition = condition.add(casbin_rule::Column::V3.eq(val));
        }
        if let Some(val) = rule.get(4) {
            condition = condition.add(casbin_rule::Column::V4.eq(val));
        }
        if let Some(val) = rule.get(5) {
            condition = condition.add(casbin_rule::Column::V5.eq(val));
        }

        let res = CasbinRule::delete_many()
            .filter(condition)
            .exec(&self.pool)
            .await
            .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;
        Ok(res.rows_affected > 0)
    }

    async fn remove_filtered_policy(
        &mut self,
        _sec: &str,
        ptype: &str,
        field_index: usize,
        field_values: Vec<String>,
    ) -> Result<bool> {
        if field_values.is_empty() {
            return Ok(false);
        }
        let mut condition = SeaOrmCondition::all().add(casbin_rule::Column::Ptype.eq(ptype));
        let add_condition = |i: usize, col: casbin_rule::Column| {
            if field_index <= i && i < field_index + field_values.len() {
                let offset = i - field_index;
                if let Some(val) = field_values.get(offset) {
                    return SeaOrmCondition::all().add(col.eq(val));
                }
            }
            SeaOrmCondition::any()
        };
        condition = condition.add(add_condition(0, casbin_rule::Column::V0));
        condition = condition.add(add_condition(1, casbin_rule::Column::V1));
        condition = condition.add(add_condition(2, casbin_rule::Column::V2));
        condition = condition.add(add_condition(3, casbin_rule::Column::V3));
        condition = condition.add(add_condition(4, casbin_rule::Column::V4));
        condition = condition.add(add_condition(5, casbin_rule::Column::V5));

        let res = CasbinRule::delete_many()
            .filter(condition)
            .exec(&self.pool)
            .await
            .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;
        Ok(res.rows_affected > 0)
    }

    async fn clear_policy(&mut self) -> Result<()> {
        CasbinRule::delete_many()
            .exec(&self.pool)
            .await
            .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;
        Ok(())
    }

    fn is_filtered(&self) -> bool {
        false
    }

    async fn load_filtered_policy<'a>(
        &mut self,
        _m: &mut dyn Model,
        _filter: Filter<'a>,
    ) -> Result<()> {
        Err(CasbinError::from(AdapterError(Box::new(Error::new(
            ErrorKind::Other,
            "filtered policies are not supported by this adapter",
        )))))
    }

    async fn add_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        let new_rules = rules.into_iter().map(|rule| casbin_rule::ActiveModel {
            ptype: ActiveValue::Set(ptype.to_owned()),
            v0: ActiveValue::Set(rule.get(0).cloned().unwrap_or_default()),
            v1: ActiveValue::Set(rule.get(1).cloned().unwrap_or_default()),
            v2: ActiveValue::Set(rule.get(2).cloned().unwrap_or_default()),
            v3: ActiveValue::Set(rule.get(3).cloned()),
            v4: ActiveValue::Set(rule.get(4).cloned()),
            v5: ActiveValue::Set(rule.get(5).cloned()),
            ..Default::default()
        });

        CasbinRule::insert_many(new_rules)
            .exec(&self.pool)
            .await
            .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;
        Ok(true)
    }

    async fn remove_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        if rules.is_empty() {
            return Ok(true);
        }
        let mut condition = SeaOrmCondition::any();
        for rule in rules {
            let mut rule_condition =
                SeaOrmCondition::all().add(casbin_rule::Column::Ptype.eq(ptype));
            if let Some(val) = rule.get(0) {
                rule_condition = rule_condition.add(casbin_rule::Column::V0.eq(val));
            }
            if let Some(val) = rule.get(1) {
                rule_condition = rule_condition.add(casbin_rule::Column::V1.eq(val));
            }
            if let Some(val) = rule.get(2) {
                rule_condition = rule_condition.add(casbin_rule::Column::V2.eq(val));
            }
            if let Some(val) = rule.get(3) {
                rule_condition = rule_condition.add(casbin_rule::Column::V3.eq(val));
            }
            if let Some(val) = rule.get(4) {
                rule_condition = rule_condition.add(casbin_rule::Column::V4.eq(val));
            }
            if let Some(val) = rule.get(5) {
                rule_condition = rule_condition.add(casbin_rule::Column::V5.eq(val));
            }
            condition = SeaOrmCondition::any().add(condition).add(rule_condition);
        }

        let res = CasbinRule::delete_many()
            .filter(condition)
            .exec(&self.pool)
            .await
            .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;
        Ok(res.rows_affected > 0)
    }
}
