use clap::{Arg, Command};
use common::enums::AdminRoleType;
use container::Container;
use domain::dto::admin::{CreateAdminCLIDTO, PatchAdminCLIDTO, SignUpAdminCLIDTO};

pub async fn run_admin_commands() -> Result<(), Box<dyn std::error::Error>> {
    let service = Container::new().await.admin_service.clone();

    let arg_matches = Command::new("admin")
        .version("0.0.1")
        .author("<djamalov.muhammad.ali7@gmail.com>")
        .about("Administrator menegment!")
        .subcommand(
            Command::new("signup-superadmin")
                .about("Signup superadmin")
                .arg(
                    Arg::new("password")
                        .value_name("password")
                        .required(true)
                        .help("password"),
                )
                .arg(
                    Arg::new("confirm-password")
                        .value_name("confirm-password")
                        .required(true)
                        .help("confirm-password"),
                )
                .arg(
                    Arg::new("phone-number")
                        .value_name("phone-number")
                        .required(true)
                        .help("phone-number"),
                ),
        )
        .subcommand(
            Command::new("create")
                .about("Create an administrator")
                .arg(
                    Arg::new("password")
                        .value_name("password")
                        .required(true)
                        .help("password"),
                )
                .arg(
                    Arg::new("confirm-password")
                        .value_name("confirm-password")
                        .required(true)
                        .help("confirm-password"),
                )
                .arg(Arg::new("role").value_name("role").required(true).help("role"))
                .arg(
                    Arg::new("phone-number")
                        .value_name("phone-number")
                        .required(true)
                        .help("phone-number"),
                ),
        )
        .subcommand(
            Command::new("merge")
                .about("Merge an existing administrator")
                .arg(
                    Arg::new("currenat-phone-number")
                        .value_name("currenat-phone-number")
                        .required(true)
                        .help("currenat-phone-number"),
                )
                .arg(
                    Arg::new("adminname")
                        .value_name("adminname")
                        .required(false)
                        .help("adminname"),
                )
                .arg(Arg::new("role").value_name("role").required(false).help("role")),
        )
        .get_matches();

    if let Some((subcommand_name, subcommand_arg_matches)) = arg_matches.subcommand() {
        match subcommand_name {
            "signup-superadmin" => {
                let admin = SignUpAdminCLIDTO {
                    password: subcommand_arg_matches
                        .get_one::<String>("password")
                        .cloned()
                        .ok_or("password not found")?,
                    confirm_password: subcommand_arg_matches
                        .get_one::<String>("confirm-password")
                        .cloned()
                        .ok_or("confirm_password not found")?,
                    phone_number: subcommand_arg_matches
                        .get_one::<String>("phone-number")
                        .cloned()
                        .ok_or("phone_number not found")?,
                };

                println!("Admin signupped successfully: {:#?}", service.signup_cli(admin).await);
            }
            "create" => {
                let admin = CreateAdminCLIDTO {
                    password: subcommand_arg_matches
                        .get_one::<String>("password")
                        .cloned()
                        .ok_or("password not found")?,
                    confirm_password: subcommand_arg_matches
                        .get_one::<String>("confirm-password")
                        .cloned()
                        .ok_or("confirm-password not found")?,
                    role: subcommand_arg_matches
                        .get_one::<String>("role")
                        .cloned()
                        .map(AdminRoleType::from)
                        .ok_or("role not found")?,
                    phone_number: subcommand_arg_matches
                        .get_one::<String>("phone-number")
                        .cloned()
                        .ok_or("phone-number not found")?,
                };
                println!("Admin created successfully: {:#?}", service.create_cli(admin).await);
            }
            "merge" => {
                let current_phone_number = subcommand_arg_matches
                    .get_one::<String>("currenat-phone-number")
                    .cloned()
                    .ok_or("currenat-phone-number not found")?;
                let admin = PatchAdminCLIDTO {
                    role: Some(AdminRoleType::from(
                        subcommand_arg_matches
                            .get_one::<String>("role")
                            .cloned()
                            .ok_or("role not found")?,
                    )),
                    phone_number: subcommand_arg_matches.get_one::<String>("phone-number").cloned(),
                };
                println!(
                    "Admin merged successfully: {:#?}",
                    service.merge_cli(current_phone_number, admin).await
                );
            }
            _ => println!("Unknown command!"),
        }
    } else {
        println!("Please specify a subcommand!");
    }

    Ok(())
}
