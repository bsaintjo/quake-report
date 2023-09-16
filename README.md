# Quake Report

Generate game reports from Quake 3 Arena log files.

## Notes

- ! Line 97, seems like the logger overwrote the ShutDown line
  - Can't count on a ShutDown line to terminate the game
  - Line can contain several times, so we need to handle having multiple times
- Start of game
  - {time} InitGame: {info} {version} {platform} {month} {day} {year}{other_info}
- kills
  - {time} Kill: {x} {y} {z}: {killer} killed {victim} by {weapon}
