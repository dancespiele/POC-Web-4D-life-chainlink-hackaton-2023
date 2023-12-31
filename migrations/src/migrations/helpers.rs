use super::{
    add_request_id_to_prize, change_score_column, create_account, create_prize, create_score,
};
use refinery::{Migration, Runner};

pub enum ExecOpt {
    Up,
    Down,
}

pub struct RollbackScript {
    pub name: String,
    pub sql: String,
}

pub fn get_migrations() -> Runner {
    let v1_migration = Migration::unapplied(
        "V1__create_account",
        &create_account::migration(ExecOpt::Up),
    )
    .unwrap();

    let v2_migration =
        Migration::unapplied("V2__create_score", &create_score::migration(ExecOpt::Up)).unwrap();

    let v3_migration =
        Migration::unapplied("V3__create_prize", &create_prize::migration(ExecOpt::Up)).unwrap();

    let v4_migration = Migration::unapplied(
        "V4__change_score_column",
        &change_score_column::migration(ExecOpt::Up),
    )
    .unwrap();

    let v5_migration = Migration::unapplied(
        "V5__add_request_id_to_prize",
        &add_request_id_to_prize::migration(ExecOpt::Up),
    )
    .unwrap();

    Runner::new(&[
        v1_migration,
        v2_migration,
        v3_migration,
        v4_migration,
        v5_migration,
    ])
}

pub fn get_rollback_migrations() -> Vec<RollbackScript> {
    vec![
        RollbackScript {
            name: "V1__create_account".to_string(),
            sql: create_account::migration(ExecOpt::Down),
        },
        RollbackScript {
            name: "V2__create_score".to_string(),
            sql: create_score::migration(ExecOpt::Down),
        },
        RollbackScript {
            name: "V3__create_prize".to_string(),
            sql: create_prize::migration(ExecOpt::Down),
        },
        RollbackScript {
            name: "V4__change_score_column".to_string(),
            sql: change_score_column::migration(ExecOpt::Down),
        },
        RollbackScript {
            name: "V5__add_request_id_to_prize".to_string(),
            sql: add_request_id_to_prize::migration(ExecOpt::Down),
        },
    ]
}
