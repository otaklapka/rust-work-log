use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
use chrono::{Local, Duration};

#[test]
fn log_current_timestamp() -> Result<(), Box<dyn std::error::Error>> {
    let message = "test message with current timestamp";
    let cur_time = Local::now().naive_local();

    // should successfully insert new message with current timestamp
    let mut log_cmd = Command::cargo_bin("rwl")?;
    log_cmd.arg("log").arg(message);
    log_cmd.assert()
        .success();

    // should list inserted message under current day with current timestamp
    let mut ls_cmd = Command::cargo_bin("rwl")?;
    ls_cmd.arg("ls");
    ls_cmd.assert()
        .success()
        .stdout(predicate::str::is_match(format!("{}\\s*{}", cur_time.format("%d.%m.%Y %H:%M"), message)).unwrap());

    Ok(())
}

#[test]
fn log_custom_timestamp() -> Result<(), Box<dyn std::error::Error>> {
    let message = "test message with custom timestamp";
    let time_input = "10.01.2021 13:02";

    // should insert message with custom time
    let mut log_cmd = Command::cargo_bin("rwl")?;
    log_cmd.arg("log").arg(message).arg("-t").arg(time_input);
    log_cmd.assert()
        .success();

    // should list test message under custom date
    let mut ls_cmd = Command::cargo_bin("rwl")?;
    ls_cmd.arg("ls").arg("-d").arg(&time_input[..10]);
    ls_cmd.assert()
        .success()
        .stdout(predicate::str::is_match(format!("{}\\s*{}", time_input, message)).unwrap());

    Ok(())
}

#[test]
fn list_last_day() -> Result<(), Box<dyn std::error::Error>> {
    let last_day_message = "test message from last day";
    let last_day = (Local::now() - Duration::days(1)).naive_local().format("%d.%m.%Y %H:%M").to_string();

    // should insert message with last day time
    let mut log_cmd = Command::cargo_bin("rwl")?;
    log_cmd.arg("log").arg(last_day_message).arg("-t").arg(&last_day);
    log_cmd.assert()
        .success();

    // should list inserted message under last day flag
    let mut ls_cmd = Command::cargo_bin("rwl")?;
    ls_cmd.arg("ls").arg("-l");
    ls_cmd.assert()
        .success()
        .stdout(predicate::str::is_match(format!("{}\\s*{}", &last_day, last_day_message)).unwrap());

    Ok(())
}
