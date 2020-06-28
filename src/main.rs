use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;


#[derive(StructOpt)]
struct ConversionCLI {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
}

fn format(content: String) {
    //TODO Error if there this is not found, as it means not BVB bank statement CSV
    // Could also just skip the first 13 lines, but that would be fragile
    let beginning = content.find("\"Buchungstag\";\"Valuta\"").unwrap();
    let transactions = content.get(beginning..).unwrap();
    for line in transactions.lines().skip(1) {
        println!("{}", line);
    }
}

fn main() -> Result<(), ExitFailure> {
    //TODO Help txt with 'cli'
    let args = ConversionCLI::from_args();
    let infilename = args.infile.as_path().display().to_string();

    //TODO Consider BufReader, though month or two of personal finance fits into memory fine
    let content = std::fs::read_to_string(&args.infile)
        .with_context(|_| format!("Could not read file '{}'", infilename))?;
    
    //TODO handle or convert ISO-8859-15, instead of leaving it to the user
    
    format(content);

    Ok(())
}
