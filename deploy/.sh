#!/usr/bin/env bash

set -e

cd "$(dirname "$0")/.."
TF="infra"

# Since Lambda uses Amazon Linux,
# the executable targets x86_64-unknown-linux-musl platform.
rustup target add x86_64-unknown-linux-musl

cargo build --release --target x86_64-unknown-linux-musl

cp ./target/release/examples/bootstrap ./bootstrap

zip lambda.zip bootstrap && rm bootstrap

pushd $TF
  if [ ! -d ".terraform" ]; then
    mkdir .terraform
    echo "${DEPLOY_ENV}" > .terraform/environment
  fi
  terraform init
  terraform workspace select "${DEPLOY_ENV}"

  TF_VAR_FILE="env-${DEPLOY_ENV}.tfvars"

  if [[ "$TF_OPERATION" == 'apply' ]]; then
    TF_ARGS=("apply" "-auto-approve")
  elif [[ "$TF_OPERATION" == 'plan' ]]; then
    TF_ARGS=("plan")
  else
    TF_ARGS=("apply")
  fi

  echo "[workspace: ${DEPLOY_ENV}] running ${TF_APPLY} terraform ${TF_ARGS[@]}"
  terraform ${TF_ARGS[@]} -var-file="${TF_VAR_FILE}" -var "CI_COMMIT_SHA=${CI_COMMIT_SHA}"

popd

pushd server
  if [[ "$TF_OPERATION" != 'plan' ]]; then
    export STAGE=$ZAPPA_ENV # work around to deal with zappa collectstatic issue
    zappa manage "$DEPLOY_ENV" migrate
    zappa manage "$DEPLOY_ENV" "collectstatic --noinput"
  fi
popd
