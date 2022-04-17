// Enums are types which have a few definite values

enum Movement {
  // Variants
  Up,
  Down,
  Left,
  Right
}

fn move_avatar(m : Movement) {
  // Perform action depending on info

  match m {
    Movement::Up => println!("Avatar moing up"),
    Movement::Down => println!("Avatar moing down"),
    Movement::Left => println!("Avatar moing left"),
    Movement::Right => println!("Avatar moing right"),
  }
}

pub fn run() {
  let avatar1 = Movement::Left;
  let avatar2 = Movement::Right;
  let avatar3 = Movement::Up;
  let avatar4 = Movement::Down;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}