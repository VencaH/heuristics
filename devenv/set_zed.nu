#!/usr/bin/env nu

mut test = (cat ~/.config/zed/settings.json | from json)
let rust = which rust-analyzer

$test.lsp.rust-analyzer.binary.path = $rust.path.0 

$test | to json | save ~/.config/zed/settings.json -f
