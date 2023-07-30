use teloxide::{prelude::*, utils::command::BotCommands};
use std::collections::HashMap;
use std::error::Error;
use std::env;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use dotenv::dotenv;

static BOOKS_LIST: Lazy<Mutex<HashMap<ChatId, Vec<String>>>> = Lazy::new(|| {
    match serde_any::from_file("books_list.json") {
        Ok(hm) => Mutex::new(hm),
        Err(_) => Mutex::new(HashMap::new())
    }
});

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Вот такие комманды есть:")]
enum Command {
    #[command(description = "Выводит этот текст.")]
    Help,
    #[command(description = "Добавить книгу в список")]
    Add(String),
    #[command(description = "Удалить книгу из списка: /remove <номер книги в списке>")]
    Remove(String),
    #[command(description = "Выводит список")]
    List,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let token = dotenv::var("TELOXIDE_TOKEN").unwrap(); //Это не трогать
    let bot = Bot::new(token); //Не трогать

    let bot = Bot::from_env().auto_send();
    println!("РАБОТАЕТ");
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

async fn answer(bot: AutoSend<Bot>, message: Message, command: Command, ) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help           => { bot.send_message(message.chat.id, Command::descriptions().to_string()).await? },
        Command::Add(items)    => { bot.send_message(message.chat.id, add_to_list(&bot, message, items)).await? },
        Command::Remove(items)     => { bot.send_message(message.chat.id, remove_from_list(&bot, message, items)).await? },
        Command::List           => { bot.send_message(message.chat.id, display_list(&bot, message)).await? },
    };
    Ok(())
}

fn display_list(_: &AutoSend<Bot>, message: Message, )  -> String  {
    let mut subs = BOOKS_LIST.lock().unwrap();
    match subs.get_mut(&message.chat.id) {
        Some(items) =>  {
            if items.len() == 0 {
                return format!("Нет книг в списке...")
            }
            let mut out = "".to_string();
            for (i, x) in items.iter().enumerate() {
                out = format!("{}\n{}\t\t{}", out, i, x);
            }
            out
        },
        None => format!("Нет книг в списке..."),
    }
}

fn remove_from_list(_: &AutoSend<Bot>, message: Message, items: String, ) -> String {
    let mut list = BOOKS_LIST.lock().unwrap();
    let resp = if items.eq("all") {
        match list.get_mut(&message.chat.id) {
            Some(v) => v.clear(),
            None => ()
        }
        "Убрать все книги из списка".to_string()
    } else {
        let ids: Vec<usize> = items
            .split_whitespace()
            .map(|id_str| id_str.parse::<usize>())
            .take_while(|x|x.is_ok())
            .map(|x|x.ok().unwrap())
            .collect();

        match list.get_mut(&message.chat.id) {
            Some(v) =>  {
                let mut out = String::new();
                for id in ids {
                    v.remove(id as usize);
                    println!("Убрать книгу из списка: {:?}", id);
                    out = format!("{}Книга была удалена.\n", out);
                }
                out
            },
            None => format!("..."),
        }
    };
    match serde_any::to_file("books_list.json", &*list) {
        Ok(_) => {();},
        Err(e) => {println!("Оибка сохранения: {:?}", e);}
    };
    resp
}

fn add_to_list(_: &AutoSend<Bot>, message: Message, items: String, ) -> String {
    let mut list = BOOKS_LIST.lock().unwrap();
    list.entry(message.chat.id).or_insert(Vec::new());
    let resp = match list.get_mut(&message.chat.id) {
        Some(v) =>  {
            let mut out = String::new();
            for item_slice in items.split(",") {
                let item = item_slice.trim().to_string();
                if v.iter().find(|&x| *x == *item) == None {
                    v.push(item.clone());
                    out = format!("{}Успешно добавлена книга({}) в список.\n", out, item);
                } else {
                    out = format!("{}Вы уже добавили книгу в список.\n", out);
                }
            }
            out
        },
        None => format!("Поломалась"),
    };
    match serde_any::to_file("books_list.json", &*list) {
        Ok(_) => {();},
        Err(e) => {println!("Ошибка в сохранении: {:?}", e);}
    };
    resp
}