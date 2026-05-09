# Croute

Croute is a terminal-based command-line interface (CLI) to manage countdowns anywhere on your device

## Installation

Download the binary file and put in a directory which is in your PATH

## Commands

### General

| Command                                           | Action                           |
| :-----------------------------------------------: | :------------------------------: |
| `croute`                                          | Show all info                    |
| `croute version`                                  | Show the current version         |

### Countdown
| Command                                           | Action                           |
| :-----------------------------------------------: | :------------------------------: |
| `croute new "countdown name" YYYY-MM-DD`          | Create a new countdown           |
| `croute new "countdown name" YYYY-MM-DD HH:MM:SS` | Create a new countdown           |
| `croute delete passed`                            | Delete all the passed countdowns |
| `croute delete "countdown name"`                  | Delete a countdown by his name   |

## Save file

The save file is ~/.croute-save.json