use std::{thread, time::Duration, env, process};
use std::process::Command;

use futures::AsyncReadExt;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result;


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

pub async fn process_command(code: &str) -> std::result::Result<(),  String>{

    let cms_link_result = env::var(CMS_API_URL_CODE);

    if cms_link_result.is_ok() {

        let api_link = cms_link_result.unwrap() + code;
        println!("{:?}", api_link);
        //making connection to the cms server to retrieve the command and process it
        let response_result = reqwest::get(api_link).await;
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


