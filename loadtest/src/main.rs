use goose::prelude::*;
use goose_eggs::{validate_and_load_static_assets, Validate};


async fn loadtest_list_users(user: &mut GooseUser) -> TransactionResult {
    let goose_users = user.get("/users").await?;
    let validate = &Validate::builder()
        .status(200)
        .build();

    validate_and_load_static_assets(user, goose_users, &validate).await?;

    Ok(())
}

async fn loadtest_create_user(user: &mut GooseUser) -> TransactionResult {
    let goose_create_user = user.post_json(
        "/user",
        &serde_json::json!({
            "name": "username",
            "location": "user location",
            "title": "Super Hero"
        })
    ).await?;

    let validate = &Validate::builder()
        .status(201)
        .build();

    validate_and_load_static_assets(user, goose_create_user, &validate).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("List users scenario")
                .set_weight(10)?
                .register_transaction(transaction!(loadtest_list_users))
        )
        .register_scenario(
            scenario!("Create user")
                .set_weight(5)?
                .register_transaction(transaction!(loadtest_create_user))
        )
        .set_default(GooseDefault::Host, "http://localhost:8000")?
        .set_default(GooseDefault::NoPrintMetrics, true)?
        .set_default(GooseDefault::StartupTime, 120)?
        .set_default(GooseDefault::ThrottleRequests, 1000)?
        .execute()
        .await?;

    Ok(())
}