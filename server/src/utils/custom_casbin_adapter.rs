use async_trait::async_trait;
use casbin::{
    Adapter, Filter, Model, Result,
    error::{AdapterError, Error as CasbinError},
};
use entity::casbin_rule::{self, Entity as CasbinRule};
use sea_orm::{ActiveValue, Condition, QueryFilter, prelude::*};

pub struct CustomAdapter {
    pool: DbConn,
}

impl CustomAdapter {
    pub fn new(pool: DbConn) -> Self {
        Self { pool }
    }
}

fn casbin_rule_to_policy_line(rule: &casbin_rule::Model) -> Option<String> {
    let mut line = rule.ptype.clone();
    let fields = [
        rule.v0.as_str(),
        rule.v1.as_str(),
        rule.v2.as_str(),
        rule.v3.as_deref().unwrap_or(""),
        rule.v4.as_deref().unwrap_or(""),
        rule.v5.as_deref().unwrap_or(""),
    ];
    for field in fields {
        if !field.is_empty() {
            line.push_str(", ");
            line.push_str(field);
        }
    }
    Some(line)
}

#[async_trait]
impl Adapter for CustomAdapter {
    async fn load_policy(&mut self, m: &mut Model) -> Result<()> {
        let rules = CasbinRule::find()
            .all(&self.pool)
            .await
            .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;
        for rule in rules {
            if let Some(line) = casbin_rule_to_policy_line(&rule) {
                m.add_policy_from_source_line(&line).await?;
            }
        }
        Ok(())
    }

    async fn save_policy(&mut self, m: &mut Model) -> Result<()> {
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

    async fn add_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<&str>) -> Result<bool> {
        let new_rule = casbin_rule::ActiveModel {
            ptype: ActiveValue::Set(ptype.to_owned()),
            v0: ActiveValue::Set(rule.get(0).map_or("".to_string(), |&s| s.to_string())),
            v1: ActiveValue::Set(rule.get(1).map_or("".to_string(), |&s| s.to_string())),
            v2: ActiveValue::Set(rule.get(2).map_or("".to_string(), |&s| s.to_string())),
            v3: ActiveValue::Set(rule.get(3).map(|&s| s.to_string())),
            v4: ActiveValue::Set(rule.get(4).map(|&s| s.to_string())),
            v5: ActiveValue::Set(rule.get(5).map(|&s| s.to_string())),
            ..Default::default()
        };
        CasbinRule::insert(new_rule)
            .exec(&self.pool)
            .await
            .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;
        Ok(true)
    }

    async fn remove_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<&str>) -> Result<bool> {
        let mut condition = Condition::all().add(casbin_rule::Column::Ptype.eq(ptype));

        if let Some(val) = rule.get(0) {
            condition = condition.add(casbin_rule::Column::V0.eq(*val));
        }
        if let Some(val) = rule.get(1) {
            condition = condition.add(casbin_rule::Column::V1.eq(*val));
        }
        if let Some(val) = rule.get(2) {
            condition = condition.add(casbin_rule::Column::V2.eq(*val));
        }
        if let Some(val) = rule.get(3) {
            condition = condition.add(casbin_rule::Column::V3.eq(*val));
        }
        if let Some(val) = rule.get(4) {
            condition = condition.add(casbin_rule::Column::V4.eq(*val));
        }
        if let Some(val) = rule.get(5) {
            condition = condition.add(casbin_rule::Column::V5.eq(*val));
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
        field_values: Vec<&str>,
    ) -> Result<bool> {
        if field_values.is_empty() {
            return Ok(false);
        }

        let mut condition = Condition::all().add(casbin_rule::Column::Ptype.eq(ptype));

        let mut add_condition = |i: usize, col: casbin_rule::Column| {
            if field_index <= i && i < field_index + field_values.len() {
                let offset = i - field_index;
                if let Some(val) = field_values.get(offset) {
                    return Condition::all().add(col.eq(*val));
                }
            }
            Condition::any()
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
        _m: &mut Model,
        _filter: Filter<'a>,
    ) -> Result<()> {
        Err(CasbinError::from(AdapterError(Box::new(
            "filtered policies are not supported by this adapter",
        ))))
    }

    async fn add_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<&str>>,
    ) -> Result<bool> {
        let new_rules = rules.into_iter().map(|rule| casbin_rule::ActiveModel {
            ptype: ActiveValue::Set(ptype.to_owned()),
            v0: ActiveValue::Set(rule.get(0).map_or("".to_string(), |&s| s.to_string())),
            v1: ActiveValue::Set(rule.get(1).map_or("".to_string(), |&s| s.to_string())),
            v2: ActiveValue::Set(rule.get(2).map_or("".to_string(), |&s| s.to_string())),
            v3: ActiveValue::Set(rule.get(3).map(|&s| s.to_string())),
            v4: ActiveValue::Set(rule.get(4).map(|&s| s.to_string())),
            v5: ActiveValue::Set(rule.get(5).map(|&s| s.to_string())),
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
        rules: Vec<Vec<&str>>,
    ) -> Result<bool> {
        if rules.is_empty() {
            return Ok(true);
        }

        let mut condition = Condition::any();
        for rule in rules {
            let mut rule_condition = Condition::all().add(casbin_rule::Column::Ptype.eq(ptype));
            if let Some(val) = rule.get(0) {
                rule_condition = rule_condition.add(casbin_rule::Column::V0.eq(*val));
            }
            if let Some(val) = rule.get(1) {
                rule_condition = rule_condition.add(casbin_rule::Column::V1.eq(*val));
            }
            if let Some(val) = rule.get(2) {
                rule_condition = rule_condition.add(casbin_rule::Column::V2.eq(*val));
            }
            if let Some(val) = rule.get(3) {
                rule_condition = rule_condition.add(casbin_rule::Column::V3.eq(*val));
            }
            if let Some(val) = rule.get(4) {
                rule_condition = rule_condition.add(casbin_rule::Column::V4.eq(*val));
            }
            if let Some(val) = rule.get(5) {
                rule_condition = rule_condition.add(casbin_rule::Column::V5.eq(*val));
            }
            condition = condition.or(rule_condition);
        }

        let res = CasbinRule::delete_many()
            .filter(condition)
            .exec(&self.pool)
            .await
            .map_err(|e| CasbinError::from(AdapterError(Box::new(e))))?;

        Ok(res.rows_affected > 0)
    }
}
