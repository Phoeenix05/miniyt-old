tools_cargo := ""

default:
  @echo "MiniYT justfile. Contains commands that are usefull for this project ;) \n"
  @echo "Like 'make' but made with Rust."

bootstrap:
  @echo "This will install all tools that I recommend to use for this project. \n"
  @echo "\033[1;31mThis will not work without Cargo and Rust being installed on your machine\033[0m \n"

  cargo install {{tools_cargo}}
  pnpm i

rm-tools:
  cargo uninstall {{tools_cargo}}

clean:
  /bin/rm -r node_modules src-tauri/target

add-component componentName:
  ng generate component ../components/{{componentName}} --skip-tests
  git add src/components/{{componentName}}/*

add-route routeName:
  ng generate component ../routes/{{routeName}} --type route --skip-tests
  git add src/routes/{{routeName}}/*
  python3.12 scripts/route.py {{routeName}} | pbcopy

cargo-add dep:
  cd src-tauri && cargo add {{dep}}

