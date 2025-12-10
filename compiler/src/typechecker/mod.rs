//! Type Checker Module
//! 
//! Performs type checking and inference on HPM-DL AST.

use crate::parser::ast::*;
use anyhow::Result;

pub struct TypeChecker {
    // TODO: Add type environment
}

#[derive(Debug, Clone, Default)]
pub struct TypeInfo {
    // TODO: Add type information
}

#[derive(Debug, Clone)]
pub struct TypeCheckedProgram {
    pub program: Program,
    pub type_info: TypeInfo,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn typecheck(&mut self, program: &Program) -> Result<TypeCheckedProgram> {
        // 简单校验：矩阵尺寸与数据长度匹配；状态维度非空。
        for op in &program.operators {
            if let Some(m) = &op.spec.matrix_representation {
                if let Some(data) = &m.data {
                    if m.rows * m.cols != data.len() {
                        anyhow::bail!(
                            "operator {}: matrix size {}x{} but data len {}",
                            op.name,
                            m.rows,
                            m.cols,
                            data.len()
                        );
                    }
                }
            }
        }

        for st in program.initial_state.iter() {
            if st.psi_0.dimensions.is_empty() {
                anyhow::bail!("initial state dimensions missing");
            }
        }

        Ok(TypeCheckedProgram {
            program: program.clone(),
            type_info: TypeInfo::default(),
        })
    }
}

