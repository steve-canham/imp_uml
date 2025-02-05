
use imp_uml::error_defs::AppError;
use imp_uml::run;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AppError> {

    let args: Vec<_> = env::args_os().collect();
    run(args).await
}

