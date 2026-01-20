# Rust Book + Rustlings - Изучение языка программирования Rust

Этот репозиторий содержит примеры кода и проекты из книги "The Rust Programming Language".

## Структура проекта

- `hello_world/` - Первая программа "Hello, World!"
- `hello_cargo/` - Проект, созданный с помощью Cargo
- `guessing_game/` - Игра "Угадай число"
- `variables/` - Примеры работы с переменными, константами и shadowing
- `first_word/` - Примеры работы со строками и срезами
- `structs/` - Примеры работы со структурами
- `enums/` - Примеры работы со перечислениями

## Как запустить

Для каждого проекта:

```bash
cd <название_проекта>
cargo run
```

Для простых файлов (например, hello_world):

```bash
rustc hello_world/main.rs
./main
```

## Требования

- Rust 1.87.0 или новее
- Cargo (устанавливается вместе с Rust)

## Установка Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```
