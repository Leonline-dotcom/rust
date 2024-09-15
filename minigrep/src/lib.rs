use std::{fs,env, error::Error};


 pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
   

    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive{
        search(config.query, &contents)
    }else{
        search_case_insensitive(config.query, &contents)
    };

    for line in results {
        println!("{}",line)
    }

    Ok(())
}

pub struct Config<'a>{
    pub query: &'a str,
    pub filename: & 'a str,
    pub case_sensitive: bool,
}

impl<'a> Config <'a>{

    pub fn new(args: &'a[String]) -> Result<Config<'a>, String>{

        if args.len() <3{
            return Err (format!("You added {} arguments instead of 2 goofus", args.len()-1));
        }
        let query = &args[1];
        let filename = &args[2];
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config{ query, filename, case_sensitive})
    }
}

pub fn search <'a>(query:&str, content: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    
    for line in content.lines(){
        if line.contains(query){
           result.push(line)
        }
    }
    return result
}


pub fn search_case_insensitive <'a> (query:&str, contents:& 'a str) -> Vec<& 'a str>{
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line)
        }
    }
 
    return result;
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "as";
        let contents = "\
what up handsome
ASSSS
safe, fast, productive, learning-curve";

        assert_eq!(vec!["safe, fast, productive, learning-curve"], search(query,contents))
    }

    #[test]
    fn case_insensitive(){
        let query = "aS";
        let contents = "\
what up handsome
haSte
safe, fast, productive, learning-curve";

        assert_eq!(vec!["haSte", "safe, fast, productive, learning-curve"], search_case_insensitive(query,contents))
    }


}