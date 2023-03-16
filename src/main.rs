/*#region Import */    
    /*#region externing crates */
        extern crate dotenv;                //externing crates
        extern crate querystring;
    /*#endregion */

    /*#region general importing */

        use std::process::Command;          //<-command
        use std::io;

        use serde_json::Error;
        use tokio;

        /*#region environment variables*/
            use dotenv::dotenv;             //<-dot env variables
            use std::env;                   //<-env library
        /*#endregion */

    /*#endregion */

    /*#region modules */
        mod api_schema;
        mod server_schema;
    /*#endregion */

/*endregion */

/*#region CONSTANTS */
    const PORT_CODE: &str = "PORT";                 //<-string code for port (change if necessary)
    const CMS_API_URL_CODE: &str = "CMS_API_URL";
/*#endregion */

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    dotenv().ok();

    //getting the port
    let _port_result = env::var(PORT_CODE);
    let _api_result = env::var(CMS_API_URL_CODE);

    if _port_result.is_ok() && _api_result.is_ok() {
    
    let _port = _port_result.unwrap().parse::<i32>().unwrap();
    //let webserver = server_schema::CommandRunnerHttpServer.start("0.0.0.0:".to_string() + _port.to_string().as_str()).unwrap();
    server_schema::start_server(_port).await

    /*Command::new("")
    .current_dir("C:/Users/WebRTC User/Documents/Freelancing/Gnosis Labs/Gnosis-Lab-Launcher")
    .arg("code .")
    .spawn().expect("Cannot run code command");*/
    }else{
        //eprintln!("PORT IS NOT SET!");
        Err(std::io::Error::new(io::ErrorKind::InvalidData, "PORT OR CMS URL IS NOT SET!"))
    }
}