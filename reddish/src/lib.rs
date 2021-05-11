

#[derive(Eq,PartialEq,Debug)]  
pub enum Command { 
    Publish(String),
    Retrieve, 
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error { 
    UnknownVerb,
    UnexpectedPayload,
    MissingPayload,
    EmptyMessage,
    IncompleteMessage,
}

pub fn parse(input: &str) -> Result<Command, Error> {
    let mut iter = input.splitn(2, ' ');

    match iter.next() {
        Some(verb) if verb.trim() == "PUBLISH"=> {
            if let Some(message) = iter.next() {
                Ok(Command::Publish(message.trim().into()))
            } else {
                Err(Error::IncompleteMessage)
            }
        }
        Some(verb) if verb.trim() == "RETRIEVE"=> {
            Ok(Command::Retrieve)
        }
        Some(_) => {
            Err(Error::UnknownVerb)
        }
        _ => Err(Error::EmptyMessage)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_works() -> Result<(), Error> {
        assert_eq!(parse("PUBLISH message\n")?, Command::Publish("message".into()));
        Ok(())
    }

    #[test]
    fn retrieve_works() -> Result<(), Error> {
        assert_eq!(parse("RETRIEVE\n")?, Command::Retrieve);
        Ok(())
    }
}
