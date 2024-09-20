default:
  @just --list

new folder:
  mkdir -p rust/{{folder}}
  echo "/target" > rust/{{folder}}/.gitignore
  cargo init --lib rust/{{folder}}

open:
  nvim $(ls -d -1 "$PWD/"rust/** | fzf)
