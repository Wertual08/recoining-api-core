use std::fs;

use scylla::{Session, IntoTypedRows, transport::errors::QueryError};

use crate::storage::ScyllaContext;

pub async fn migrate(context: &ScyllaContext, migrations: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running migrations...");

    for path in fs::read_dir(migrations).unwrap() {
        if let Ok(entry) = path {
            let file_name = entry.file_name().into_string().unwrap();
            let id_end = file_name.find(|c: char| c != '_' && !c.is_digit(10)).expect("Migration file name must start with id");

            let (id_src, name_src) = file_name.split_at(id_end);

            let id: i64 = id_src.replace("_", "").parse().expect("Migration id must have integer value");
            let name = if name_src.ends_with(".cql") {
                String::from(&name_src[..name_src.len() - 4])
            }
            else {
                String::from(name_src)
            };

            let content = fs::read_to_string(entry.path())?;

            let statements: Vec<String> = content
                .split(";")
                .filter_map(|cql| {
                    let trimmed = cql.trim();
                    if trimmed.is_empty() {
                        None
                    }
                    else {
                        Some(String::from(trimmed))
                    }
                })
                .collect();

            create_migrations_table(
                &context.session, 
                &context.keyspace,
            ).await;

            let mut migration = if let Some(mut some) = select_migration(
                &context.session, 
                &context.keyspace,
                id
            ).await {
                some.name = name;
                some.statements = statements;
                some
            }
            else {
                Migration {
                    id: id,
                    cursor: 0,
                    name: name,
                    statements: statements,
                }
            };
            
            print!("Migrating [{}] {}...", id, &migration.name);
            
            for (cursor, cql) in migration.vec() {
                context.session.query(cql.as_str(), ()).await?;
                
                migration.cursor = cursor;

                if create_migrations_table(
                    &context.session,
                    &context.keyspace,
                ).await {
                    create_migration(
                        &context.session,
                        &context.keyspace,
                        &migration,
                    ).await?;
                }

                print!(" {}", cursor + 1);
            }
            
            migration.cursor = -1;
            create_migration(
                &context.session,
                &context.keyspace,
                &migration,
            ).await?;
            
            println!(" DONE")
        }
    }

    println!("Migrations succeed");

    Ok(())
}

async fn create_migrations_table(session: &Session, keyspace: &str) -> bool {
    session.query(
        format!(
            "create table if not exists {}.migrations (
                id bigint primary key,
                cursor int,
                name text,
                statements list<text>
            )",
            keyspace,
        ), 
        (),
    ).await.is_ok()
}

async fn create_migration(session: &Session, keyspace: &str, migration: &Migration) -> Result<(), QueryError> {
    session.query(
        format!(
            "insert into {}.migrations (
                id,
                cursor,
                name,
                statements
            ) values (?, ?, ?, ?)",
            keyspace,
        ), 
        (migration.id, migration.cursor, &migration.name, &migration.statements),
    ).await?;

    Ok(())
}

async fn select_migration(session: &Session, keyspace: &str, id: i64) -> Option<Migration> {
    let result = session.query(
        format!(
            "select id, cursor, name, statements
            from {}.migrations
            where id = ?",
            keyspace,
        ),
        (id,),
    ).await;

    match result {
        Ok(success) => match success.rows {
            Some(rows) => {
                match rows.into_typed::<(i64, i32, String, Vec<String>)>().next() {
                    Some(row_result) => match row_result {
                        Ok((id, cursor, name, statements)) => Some(Migration { 
                            id, 
                            cursor, 
                            name, 
                            statements, 
                        }),
                        Err(_) => None,
                    }
                    None => None,
                }
            },
            None => None,
        },
        Err(_) => None,
    }
}

struct Migration {
    id: i64,
    cursor: i32,
    name: String,
    statements: Vec<String>,
}

impl Migration {
    fn vec(&self) -> Vec<(i32, String)> {
        if self.cursor < 0 {
            return Vec::new();
        }
        self.statements
            .iter()
            .enumerate()
            .skip(self.cursor as usize)
            .map(|(index, statement)| (index as i32 + 1, statement.clone()))
            .collect()
    }
}