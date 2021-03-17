use std::error::Error;
use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

#[derive(Debug)]
struct Config {
    template: String,
    project: String,
    study: String,
}

fn init_config() -> Result<Option<Config>, Box<dyn Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("\n*********************************************************\n** Welcome to Autotrail Program Generator");

    if !Confirm::with_theme(&theme)
        .with_prompt("Are you sure you want to create a new template program?")
        .interact()?
    {
        return Ok(None);
    }

    let _template = Select::with_theme(&theme)
        .with_prompt("Select template to generate:")
        .default(0)
        .item("Demography listing")
        .item("Adverse Events")
        .item("Patient Profiles")
        .interact()?;

    let template = match _template {
        0 => String::from("%AT_DEMOGRAPHICS"),
        1 => String::from("%AT_AE"),
        2 => String::from("%AT_PATPROFILES"),
        _ => panic!("Invalid option selected!"),
    };


    let project = Input::with_theme(&theme)
        .with_prompt("Project code:")
        .interact()?;

    let study = Input::with_theme(&theme)
        .with_prompt("Project's study code:")
        .interact()?;


    Ok(Some(Config {
        template,
        project,
        study,
    }))
}

fn main() {
    match init_config() {
        Ok(None) => println!("Autotrial template generation aborted."),
        Ok(Some(config)) => println!("Your program has been generated! Metadata follows:\n\n{:#?}", config),
        Err(err) => println!("error: {}", err),
    }
}