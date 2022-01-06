use gui::message_box as mbox;

use std::io::Write;

fn main() {
    match app() {
        Ok(_) => (),
        Err(e) => {
            if let Some(err) = e.downcast_ref::<mbox::MessageBoxError>() {
                eprintln!("plop: {}", err);
            }
        }
    }
}

type AppError = Box<dyn std::error::Error>;
type AppResult = std::result::Result<(), AppError>;

fn app() -> AppResult {
    let mut quiz = Quiz::new("Current mood");
    quiz.add_question("Did you wake up early? 🌄")
        .add_question("Did you drink your coffee? ☕")
        .add_question("Did you pet the katzie? 😺");

    match quiz.ask_all() {
        Ok(_) => quiz.print_results(),
        Err(_) => quiz.print_cancelled(),
    }?;

    Ok(())
}

struct Quiz {
    title: String,
    questions: Vec<String>,
    responses: Vec<Option<bool>>,
    current: usize,
}

type QuizResult = std::result::Result<(), AppError>;

impl Quiz {
    fn new(title: &str) -> Quiz {
        Quiz {
            title: title.to_owned(),
            questions: vec![],
            responses: vec![],
            current: 0,
        }
    }

    fn print_results(&self) -> std::io::Result<()> {
        println!("—— Quiz: {} ——", self.title.to_string());
        for (i, resp) in self.responses.iter().enumerate() {
            println!("Response {}: {}", i + 1, resp.unwrap());
        }
        std::io::stdout().flush()?;
        Ok(())
    }

    fn print_cancelled(&self) -> std::io::Result<()> {
        let answered = self.responses.iter().fold(1, |acc, x| {
            acc + match x {
                Some(_) => 1,
                None => 0,
            }
        });

        println!("—— Quiz: {} ——", self.title.to_string());
        println!("Only partially answered, the user");
        println!(" cancelled after {} questions.", answered);
        std::io::stdout().flush()?;
        Ok(())
    }

    fn add_question(&mut self, question: &str) -> &mut Self {
        self.questions.push(question.to_owned());
        self.responses.push(None);
        self
    }

    fn ask_all(&mut self) -> QuizResult {
        for _q in 0..self.questions.len() {
            self.ask_next()?;
        }
        Ok(())
    }

    fn ask_next(&mut self) -> QuizResult {
        let res = mbox::new(
            &format!(
                "Question {}: {}",
                self.current + 1,
                self.questions[self.current]
            )
            .to_string(),
            &format!(
                "{}: {}/{}",
                self.title.to_string(),
                self.current + 1,
                self.questions.len()
            )
            .to_string(),
            mbox::style::YesNoCancel | mbox::style::IconQuestion,
        )?;

        match res {
            mbox::Result::Yes => self.responses[self.current] = Some(true),
            mbox::Result::No => self.responses[self.current] = Some(false),
            _ => return Err("unexpected".into()),
        }

        self.current += 1;
        Ok(())
    }
}
