extern crate rusqlite;
extern crate aes;
extern crate block_modes;
extern crate block_padding;
extern crate base64;

use rusqlite::{params, Connection, Result};
use aes::{Aes256};
use block_modes::{BlockMode, Cbc};
use block_padding::Pkcs7;
use base64::{Engine as _, engine::general_purpose::STANDARD}; // Importando o trait Engine para permitir o uso de encode e decode
use std::io::{self, Write};

#[derive(Debug)]
struct PasswordEntry {
    app: String,
    user: String,
    email: String,
    password: String,
    obs: Option<String>,
}

// A chave foi ajustada para ter exatamente 32 bytes
const KEY: &[u8; 32] = b"32-byte-secret-key-used-for-encr"; // Chave de 32 bytes
const IV: &[u8; 16] = b"16-byte-iv-vec!!"; // Vetor de inicialização de 16 bytes


type Aes256Cbc = Cbc<Aes256, Pkcs7>; // Criação do tipo AES-256 com CBC e padding PKCS7

fn main() -> Result<()> {
    let conn = Connection::open("password_manager.db")?;
    
    // Criar as tabelas
    create_tables(&conn)?;

    // Cadastrar admin
    if !is_admin_exists(&conn)? {
        create_admin_user(&conn)?;
    }

    // Menu principal
    loop {
        println!("\nGerenciador de Senhas:");
        println!("1 - Cadastrar Senha");
        println!("2 - Buscar Senha");
        println!("3 - Listar Senhas");
        println!("4 - Excluir Senha");
        println!("5 - Sair");

        let mut choice = String::new();
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u8 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => register_password(&conn)?,
            2 => search_password(&conn)?,
            3 => list_passwords(&conn)?,
            4 => delete_password(&conn)?,
            5 => break,
            _ => println!("Opção inválida!"),
        }
    }

    Ok(())
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            app TEXT NOT NULL,
            user TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL,
            obs TEXT
        )",
        [],
    )?;

    Ok(())
}

fn is_admin_exists(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = 'admin'")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count > 0)
}

fn create_admin_user(conn: &Connection) -> Result<()> {
    println!("Cadastro do usuário admin");

    print!("Digite a senha do admin: ");
    io::stdout().flush().unwrap();
    let mut admin_password = String::new();
    io::stdin().read_line(&mut admin_password).unwrap();
    let admin_password = admin_password.trim();

    // Criptografar a senha do admin antes de armazenar
    let encrypted_admin_password = encrypt(admin_password);

    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        params!["admin", encrypted_admin_password],
    )?;

    println!("Usuário admin cadastrado com sucesso!");
    Ok(())
}

fn register_password(conn: &Connection) -> Result<()> {
    println!("\nCadastro de Senha:");

    let mut app = String::new();
    print!("App: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut app).unwrap();

    let mut user = String::new();
    print!("Usuário: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user).unwrap();

    let mut email = String::new();
    print!("Email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();

    let mut password = String::new();
    print!("Senha: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();

    let mut obs = String::new();
    print!("Observações (opcional): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut obs).unwrap();

    let entry = PasswordEntry {
        app: app.trim().to_string(),
        user: user.trim().to_string(),
        email: email.trim().to_string(),
        password: encrypt(password.trim()),
        obs: if obs.trim().is_empty() {
            None
        } else {
            Some(obs.trim().to_string())
        },
    };

    conn.execute(
        "INSERT INTO passwords (app, user, email, password, obs) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![entry.app, entry.user, entry.email, entry.password, entry.obs],
    )?;

    println!("Senha cadastrada com sucesso!");
    Ok(())
}

fn list_passwords(conn: &Connection) -> Result<()> {
    println!("\nListar Senhas:");

    let mut stmt = conn.prepare("SELECT app, user, email, password, obs FROM passwords")?;
    let password_iter = stmt.query_map([], |row| {
        Ok(PasswordEntry {
            app: row.get(0)?,
            user: row.get(1)?,
            email: row.get(2)?,
            password: decrypt(&row.get::<_, String>(3)?),
            obs: row.get(4)?,
        })
    })?;

    let mut found = false;

    for entry in password_iter {
        match entry {
            Ok(password_entry) => {
                found = true;
                println!("App: {}", password_entry.app);
                println!("Usuário: {}", password_entry.user);
                println!("Email: {}", password_entry.email);
                println!("Senha: {}", password_entry.password); // Exibindo a senha em texto claro
                if let Some(obs) = password_entry.obs {
                    println!("Observações: {}", obs);
                } else {
                    println!("Observações: Nenhuma");
                }
            },
            Err(e) => {
                eprintln!("Erro ao listar senhas: {}", e);
            }
        }
    }

    if !found {
        println!("Nenhuma senha cadastrada.");
    }

    Ok(())
}

fn search_password(conn: &Connection) -> Result<()> {
    println!("\nBuscar Senha:");

    let mut app = String::new();
    print!("Digite o nome do aplicativo: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut app).unwrap();

    let mut stmt = conn.prepare("SELECT app, user, email, password, obs FROM passwords WHERE app = ?1")?;
    let password_iter = stmt.query_map([app.trim()], |row| {
        Ok(PasswordEntry {
            app: row.get(0)?,
            user: row.get(1)?,
            email: row.get(2)?,
            password: decrypt(&row.get::<_, String>(3)?),
            obs: row.get(4)?,
        })
    })?;

    let mut found = false;

    for entry in password_iter {
        match entry {
            Ok(password_entry) => {
                found = true;
                println!("App: {}", password_entry.app);
                println!("Usuário: {}", password_entry.user);
                println!("Email: {}", password_entry.email);
                println!("Senha: {}", password_entry.password);
                if let Some(obs) = password_entry.obs {
                    println!("Observações: {}", obs);
                } else {
                    println!("Observações: Nenhuma");
                }
            },
            Err(e) => {
                eprintln!("Erro ao buscar senha: {}", e);
            }
        }
    }

    if !found {
        println!("Senha não encontrada.");
    }

    Ok(())
}

fn delete_password(conn: &Connection) -> Result<()> {
    println!("\nExcluir Senha:");

    let mut app = String::new();
    print!("Digite o nome do aplicativo para excluir: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut app).unwrap();

    conn.execute(
        "DELETE FROM passwords WHERE app = ?1",
        params![app.trim()],
    )?;

    println!("Senha excluída com sucesso!");
    Ok(())
}

fn encrypt(data: &str) -> String {
    let cipher = Aes256Cbc::new_from_slices(KEY, IV).unwrap();
    let mut buffer = data.as_bytes().to_vec();
    let ciphertext = cipher.encrypt_vec(&mut buffer);
    STANDARD.encode(&ciphertext)  // Codificando a saída em base64
}

fn decrypt(data: &str) -> String {
    let cipher = Aes256Cbc::new_from_slices(KEY, IV).unwrap();
    let decoded_data = STANDARD.decode(data).unwrap(); // Decodificando a string base64
    let decrypted_data = cipher.decrypt_vec(&decoded_data).unwrap();
    String::from_utf8(decrypted_data).unwrap()
}
