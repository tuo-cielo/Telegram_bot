use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::Message;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let token = dotenvy::var("TELOXIDE_TOKEN").unwrap(); //Это не трогать
    let bot = Bot::new(token); //Это важно

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Существуют следующие команды:")]
enum Command {
    #[command(description = "Выводит этот список команд")]
    Help,
    #[command(description = "Ссылка на сайт ВУЗа")]
    Mtuci,
    #[command(description = "Рассказывает о ВУЗе если вы только собрались поступить к нам")]
    Абитуриенту,
    #[command(description = "Вы вводите ваши экзамены мы советуем направления")]
    Направление
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            let result = bot.send_message(msg.chat.id, Command::descriptions().to_string()).await;
            if let Err(err) = result {
                log::error!("Failed to send message: {:?}", err);
            }
        }
        Command::Mtuci => {
            let result = bot.send_message(msg.chat.id, format!("https://mtuci.ru/")).await;
            if let Err(err) = result {
                log::error!("Failed to send message: {:?}", err);
            }
        }
        Command::Абитуриенту => {
            let result = bot.send_message(msg.chat.id, format!("Информация о месте нахождения образовательной организации:
Адрес МТУСИ: 111024, г. Москва, ул. Авиамоторная, 8а;
Адрес МТУСИ: 123423, г. Москва, ул. Народного Ополчения, д. 32
Адрес КТ МТУСИ: 125493, г. Москва, ул. Авангардная, д. 5.


Информация о режиме и графике работы образовательной организации:
График работы структурных подразделений:

Пн – чт : 09:00- 18:00 ( обед 13:00-14:00)
Пт : 09:00-16:45 ( обед 13:00-14:00)
Прием ректором студентов проводится по понедельникам с 14:00 до 14:30

Информация о контактных телефонах образовательной организации:
Ректорат +7 (495)957-79-17
Пост охраны +7(495)957-7945

Отдел документационного обеспечения управления +7(495)957-7731, факс +7(495)925-04-35

Информация об адресах электронной почты образовательной организации:
Ректорат: mtuci@mtuci.ru
Отдел документационного обеспечения управления: kanc@mtuci.ru")).await;
            if let Err(err) = result {
                log::error!("Failed to send message: {:?}", err);
            }
        }
        Command::Направление => {
            let result = bot.send_message(msg.chat.id, format!("Введите ваш набор экзаменов следующим образом: математика, русский, физика")).await;
            if let Err(err) = result {
                log::error!("Failed to send message: {:?}", err);

            }
        }
    }

    Ok(())
}

async fn analyze_message(text: &str) -> String {
    if text.contains("привет") {
        "Привет!".to_string()
    } else {
        "Я не понимаю эту команду.".to_string()
        }

}
