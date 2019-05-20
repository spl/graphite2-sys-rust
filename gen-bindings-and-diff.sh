#!/bin/bash

# Empty internal field separator to distinguish '\n' from '\t', etc.
IFS=

BINDINGS=graphite2-sys/src/bindings.rs

if ! git diff-index --quiet HEAD -- $BINDINGS; then
  echo
  echo -e "\033[31;1mWARNING! Local changes to $BINDINGS.\033[0m"
  echo
  git status -- $BINDINGS
  echo
  echo -en "\033[31;1mOverwrite? [Yn]\033[0m "
  read -n1 answer
  echo
  case ${answer} in
    y|Y|'' )
    ;;
    * )
      echo -e "\033[31;1mCancelled.\033[0m"
      exit 0
    ;;
  esac
fi

echo "Generating $BINDINGS..."
cargo run --manifest-path graphite2-sys-bindgen/Cargo.toml

echo "done."
echo
echo -en "\033[31;1mgit diff -- $BINDINGS? [Yn]\033[0m "
read -n1 -p "" answer
case ${answer} in
  y|Y|'' )
  ;;
  * )
    echo "Cancelled."
    exit 0
  ;;
esac

# After running the above, we should verify that the changes are what we want.
git diff -- $BINDINGS
