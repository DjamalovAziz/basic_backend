use _actix_web::server::run_actix_web;
use _clap::commands::admin::run_admin_commands;
use tokio::fs;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    flexi_logger::Logger::try_with_env_or_str("info")?
        .log_to_file(
            flexi_logger::FileSpec::default()
                .directory("basic_backend_logfiles")
                .basename("daily-report")
                .suffix("log"),
        )
        .print_message()
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(30),
        )
        .duplicate_to_stderr(flexi_logger::Duplicate::Info)
        .write_mode(flexi_logger::WriteMode::Async)
        .start()?;

    fs::create_dir_all("./media/images/avatars").await?;

    run_admin_commands().await?;

    run_actix_web().await?;

    Ok(())
}
