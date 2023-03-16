use std::{thread, time::Duration, env, process};
use std::process::Command;

use futures::AsyncReadExt;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use runtime_format::{FormatArgs, FormatKey, FormatKeyError};
use core::fmt;


///<summary>
/// This script will containt the schema for the api and will also be used to retrieve data from the api
///</summary>

const CMS_API_URL_CODE: &str = "CMS_API_URL";

#[derive(Serialize, Deserialize, Clone)]
struct executionData{
    execute_path: String,       //<-The path at which the command should be run           //<-Bash Script to run
    command: String            //<-Command to Run
}

impl executionData{
    fn to_string(&self) -> String{
        format!("Execution File Path: {}, Command: {}", &self.execute_path, &self.command)
    }
}

/*#region unique_identifier_code_formatter */
struct UNIQUE_IDENTIFIER{
    code: String
}

impl UNIQUE_IDENTIFIER{
    fn new(code: String) -> UNIQUE_IDENTIFIER{
        UNIQUE_IDENTIFIER { code: code }
    }
}
impl FormatKey for UNIQUE_IDENTIFIER{
    fn fmt(&self, key: &str, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), FormatKeyError> {
        match key {
            "uniqueidentifier"    => write!(f, "{}", self.code).map_err(FormatKeyError::Fmt),
            _ => Err(FormatKeyError::UnknownKey),
        }
    }
}

/*#endregion */

pub async fn process_command(code: &str) -> std::result::Result<(),  String>{


    let cms_link_result = env::var(CMS_API_URL_CODE);

    if cms_link_result.is_ok() {

        let cms_link_result_string = cms_link_result.unwrap();
        let cms_link_formatter_original: &str = cms_link_result_string.as_str();
        
        let api_format = FormatArgs::new(cms_link_formatter_original, &UNIQUE_IDENTIFIER::new("hi".into())).to_string();
        let api: String = api_format;
        println!("{:?}", api);
        //making connection to the cms server to retrieve the command and process it
        let response_result = reqwest::get(api).await;
        if response_result.is_ok() {

            let response_text = response_result.unwrap().text().await.unwrap();

            let execution_data_result : serde_json::Result<executionData> = serde_json::from_str(&response_text);
            
            if execution_data_result.is_ok() {
                //Now we run command
                let execution_data = execution_data_result.unwrap();

                println!("{:?}", execution_data.to_string());
                let output = Command::new(execution_data.execute_path)
                                    .output();

                if output.is_ok() {
                    return Ok(());
                }else{
                    return Err(output.err().unwrap().to_string());
                }
            }else{
                println!("{}", response_text);
                return Err(execution_data_result.err().unwrap().to_string().to_owned() + " <- Json Parse Error");
            }
        }else{
            return Err("Not online!?".into());
        }

    }else{
        eprintln!("CMS LINK IS NOT SET");
        process::exit(1);
    }

}


