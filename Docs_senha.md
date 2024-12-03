# Gerenciador de Senhas em Rust

Este projeto é um gerenciador de senhas simples em Rust, que usa criptografia AES-256 em modo CBC para armazenar e recuperar senhas de forma segura. 
O programa permite que o usuário registre, busque, liste e exclua senhas de um banco de dados SQLite.

## Funcionalidades

1. **Cadastro de Senhas**: Permite que o usuário cadastre senhas, associando-as a um aplicativo, usuário e email.
2. **Busca de Senhas**: O usuário pode buscar senhas armazenadas por nome de aplicativo.
3. **Listagem de Senhas**: Lista todas as senhas cadastradas no banco de dados.
4. **Exclusão de Senhas**: Permite excluir senhas armazenadas por nome de aplicativo.
5. **Criptografia e Descriptografia**: Utiliza AES-256 (modo CBC) para criptografar as senhas antes de armazená-las e descriptografá-las quando necessário.

## Requisitos

- **Rust**: O código foi escrito em Rust, portanto, você precisa ter o Rust instalado. 
Se não tiver, pode instalar usando [rustup](https://www.rust-lang.org/learn/get-started).
- **Crates (Bibliotecas)**:
  - `rusqlite`: para interação com o banco de dados SQLite.
  - `aes`: para criptografia AES.
  - `block_modes`: para o modo de operação CBC (Cipher Block Chaining) da criptografia.
  - `block_padding`: para adicionar o padding PKCS7.
  - `base64`: para codificação e decodificação base64 de dados criptografados.

## Estrutura do Projeto

- **src/senha.rs**: Código principal da aplicação que implementa as funcionalidades do gerenciador de senhas.
- **password_manager.db**: Banco de dados SQLite onde as senhas e usuários são armazenados.

## Como Executar

1. **Clone o repositório** (caso necessário):

   ```bash
   git clone <url-do-repositorio>
   cd <diretorio-do-repositorio>
   ```

2. **Instale as dependências**:

   O Cargo, gerenciador de pacotes do Rust, irá automaticamente baixar as dependências necessárias quando você compilar o projeto.

   ```bash
   cargo build
   ```

3. **Execute o programa**:

   Após compilar, você pode executar o gerenciador de senhas com:

   ```bash
   cargo run
   ```

   Isso iniciará o menu do gerenciador de senhas, onde você poderá interagir com o sistema.

## Como Funciona

### 1. Criação das Tabelas no Banco de Dados

O banco de dados SQLite será inicializado com duas tabelas:

- **users**: armazena informações de usuários, como nome de usuário e senha (criptografada).
- **passwords**: armazena senhas associadas a aplicativos, usuários, emails e observações.

As tabelas são criadas automaticamente na primeira execução do programa.

### 2. Criação do Usuário Admin

Ao rodar o programa pela primeira vez, será solicitado que o usuário admin forneça uma senha. Esta senha será criptografada e armazenada no banco de dados.

### 3. Criptografia de Senhas

A criptografia das senhas é feita usando o algoritmo **AES-256** em modo **CBC** (Cipher Block Chaining) com **padding PKCS7**. 
O vetor de inicialização (IV) e a chave de 256 bits (32 bytes) são constantes no código.

- **Chave de Criptografia (KEY)**: `b"32-byte-secret-key-used-for-encr"`
- **Vetor de Inicialização (IV)**: `b"16-byte-iv-vec!!"`

### 4. Funções Principais

- **`encrypt(data: &str) -> String`**: Criptografa uma string usando AES-256 CBC e retorna o resultado em Base64.
  
- **`decrypt(data: &str) -> String`**: Descriptografa uma string em Base64 usando AES-256 CBC e retorna o texto original.

### 5. Operações do Menu

O programa oferece um menu com as seguintes opções:

1. **Cadastrar Senha**: Permite que o usuário adicione uma nova senha ao banco de dados. 
A senha é criptografada antes de ser armazenada.
2. **Buscar Senha**: Permite buscar senhas por nome de aplicativo.
3. **Listar Senhas**: Exibe todas as senhas armazenadas no banco de dados.
4. **Excluir Senha**: Permite excluir uma senha registrada pelo nome do aplicativo.
5. **Sair**: Encerra o programa.

### 6. Estrutura do Banco de Dados

- **Tabela `users`**:
  - `id`: Identificador único do usuário.
  - `username`: Nome de usuário (ex: "admin").
  - `password`: Senha criptografada do usuário.

- **Tabela `passwords`**:
  - `id`: Identificador único da senha.
  - `app`: Nome do aplicativo.
  - `user`: Nome de usuário associado ao aplicativo.
  - `email`: Email do usuário.
  - `password`: Senha criptografada.
  - `obs`: Observações adicionais sobre a senha (opcional).

### 7. Exemplo de Uso

#### Cadastro de Senha

Ao escolher a opção "Cadastrar Senha" no menu, o programa pedirá:

- Nome do aplicativo
- Nome de usuário
- Email
- Senha (a ser criptografada)
- Observações (opcional)

A senha será armazenada de forma segura no banco de dados, criptografada com AES-256.

#### Buscar Senha

Ao escolher "Buscar Senha", você pode procurar senhas armazenadas por nome de aplicativo. 
O programa exibirá as informações correspondentes, incluindo a senha (descriptografada).

#### Listar Senhas

"Listar Senhas" exibirá todas as senhas armazenadas, mostrando detalhes como aplicativo, usuário, email e senha (descriptografada).

#### Excluir Senha

Com "Excluir Senha", você pode remover uma senha específica do banco de dados, identificando-a pelo nome do aplicativo.

## Estrutura do Código

### `src/senha.rs`

1. **Dependências**:
   O código utiliza várias dependências de bibliotecas externas como `aes`, `rusqlite`, `base64`, etc.

2. **Estruturas de Dados**:
   A struct `PasswordEntry` é usada para representar as entradas de senha no banco de dados.

3. **Funções Principais**:
   - `create_tables`: Cria as tabelas no banco de dados.
   - `create_admin_user`: Cria o usuário admin, se necessário.
   - `register_password`: Registra uma nova senha.
   - `search_password`: Busca senhas por aplicativo.
   - `list_passwords`: Lista todas as senhas.
   - `delete_password`: Exclui senhas por nome de aplicativo.
   - `encrypt` e `decrypt`: Funções para criptografar e descriptografar senhas usando AES-256.

## Conclusão

Este gerenciador de senhas simples utiliza técnicas de criptografia para garantir que as senhas armazenadas sejam seguras. 
O banco de dados SQLite facilita o armazenamento e recuperação eficiente dos dados.

---
