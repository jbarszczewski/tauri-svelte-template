#[tauri::command]
pub fn my_custom_command(invoke_message: String) -> String {
  format!(
    "Hey! I was invoked from JS, with this message: {}",
    invoke_message
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = my_custom_command(String::from("test"));
    assert_eq!(
      result,
      "Hey! I was invoked from JS, with this message: test"
    );
  }
}
