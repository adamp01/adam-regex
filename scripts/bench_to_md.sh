#!/bin/bash

echo "| Pattern         | Engine        | Time (µs) | Notes |"
echo "|-----------------|---------------|-----------|-------|"

awk '
  BEGIN {
    pattern = ""; engine = ""; value = 0;
  }

  # Case A: pattern - engine and time all on same line
  /^[[:alnum:] _():.-]+ - [[:alnum:]_():.-]+[[:space:]]+time:[[:space:]]+\[[0-9.]+ [a-zµ]+/ {
    line = $0
    sub(/ *time:.*/, "", line)
    split(line, parts, " - ")
    pattern = parts[1]
    engine = parts[2]

    time_part = $0
    sub(/^.*\[/, "", time_part)
    split(time_part, fields, " ")
    value = fields[1] + 0
    unit = fields[2]
    if (unit == "ns") {
      value = value / 1000.0
    }

    printf "| %-15s | %-13s | %9.3f |       |\n", pattern, engine, value
    pattern = ""; engine = ""; value = 0
    next
  }

  # Case B.1: first line has pattern and engine only
  /^[[:alnum:] _():.-]+ - [[:alnum:]_():.-]+$/ {
    split($0, parts, " - ")
    pattern = parts[1]
    engine = parts[2]
    next
  }

  # Case B.2: time line comes after
  /^[[:space:]]*time:[[:space:]]+\[[0-9.]+ [a-zµ]+/ {
    if (pattern != "" && engine != "") {
      line = $0
      sub(/^.*\[/, "", line)
      split(line, fields, " ")
      value = fields[1] + 0
      unit = fields[2]
      if (unit == "ns") {
        value = value / 1000.0
      }

      printf "| %-15s | %-13s | %9.3f |       |\n", pattern, engine, value
      pattern = ""; engine = ""; value = 0
    }
  }
' "$1"