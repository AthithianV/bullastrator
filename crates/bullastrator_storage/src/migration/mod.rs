pub struct Migration {
    pub version: &'static str,
    pub sql: &'static str,
}

pub const MIGRATION_TABLE: Migration = Migration {
    version: "m2026_10_20_01_migration_table",
    sql: include_str!("m2026_10_20_01_migration_table.sql"),
};

pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: "m2026_10_04_01_init",
        sql: include_str!("m2026_10_04_01_init.sql"),
    },
    Migration {
        version: "m2026_10_20_02_add_sort_order",
        sql: include_str!("m2026_10_20_02_add_sort_order.sql"),
    },
];
