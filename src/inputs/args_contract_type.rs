use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::inputs::args_contract_type::CodeTool::{GITHUB, GITLAB};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GoldenPath {
    pub url: String,
    pub path: String,
    pub branch: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CodeTool {
    GITHUB,
    GITLAB,
    GITEA,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Code {

    #[serde(rename = "type")]
    pub c_type: CodeTool,
    pub url: String,
    pub branch: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Contract {
    pub action: String,
    #[serde(rename = "golden-path")]
    pub golden_path: GoldenPath,
    pub code: Code,
}

impl FromStr for CodeTool {

    type Err = CodeToolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s {
            "github" => Ok(GITHUB),
            "gitlab" => Ok(GITLAB),
            "gitea" => Ok(Self::GITEA),
            _ => Err(CodeToolError)
        }
    }
}

pub struct CodeToolError;