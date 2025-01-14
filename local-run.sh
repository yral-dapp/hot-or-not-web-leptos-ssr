#!/bin/bash

LEPTOS_SITE_ROOT="target/site" LEPTOS_HASH_FILES=true cargo leptos build --bin-features local-bin --lib-features local-lib

