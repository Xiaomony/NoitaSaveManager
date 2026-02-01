# NoitaSaveManager

[简体中文Readme](./docs/Readme_Simplified_Chinese.md) [繁體中文Readme](/docs/Readme_Traditional_Chinese.md)

The application currently support:

- Simplified Chinese
- Traditional Chinese(AI translated)
- English(en_US)
- English(en_GB, AI translated)
- Japanese(AI translated)

1. [How to use & Notes](#how-to-use-notes)
2. [Command Line Usage](#command-line-usage)
3. [Save Info File](#save-info-file)
4. [Command List](#command-list)

## How to use & Notes

1. The program will create a `Saves` folder in the same directory as the program when it's running.
   The folder is used to store logs and saved game data.
   **Don't delete it.** Unless you want reset the state of the program and it will behaviour as the first time you launch it.

2. It’s recommended to place the program in a normal folder on your disk and then create a desktop shortcut.
   This helps avoid accidentally deleting the `Saves` folder if the program is placed directly on the desktop.

3. Please load your save when there's an "**Continue**" option on your noita menu.
   if it doesn't, please click "**New Game**", then quit the game and load your save.
   After loading your save, "**Continue**" option should appear.

4. It’s recommended to **disable Steam Cloud saves**.

5. Only save after Noita has been **properly saved and closed**.
   If you save while the game is still running, the program will back up Noita’s **auto-save**, which may be from a few minutes earlier.

6. **Don't load saves while game is running**

7. As you progress in the game, both the **save time** and **disk usage** will increase.
   Please be patient.

8. You can pass command-line arguments to the command-line version of the program

9. Avoid clicking buttons too fast when using GUI version.

10. The command-line version and GUI version share the same `Saves` folder,
    but **never run both of them at the same time**

11. You can set the path of `noita.exe` and launch noita from the program,
    which is faster than launching noita via steam.
    But launch the game without steam running will cause the mods downloaded from steam not loaded.

12. How to check the path of `noita.exe`:
    Steam Library → Noita → Gear icon → Manage → Browse local files

## Command Line Usage

1. Commands can be entered in full or using their abbreviations (shown in parentheses).

2. Commands can be used in two ways: **command-argument mode** and **interactive mode**.
    - **Command-argument mode**
      Use commands like a typical CLI:

        ```bash
        save SaveName SaveNote
        ```

        Arguments are separated by spaces.
        If any argument contains spaces, wrap it in **quotes**:

        ```bash
        save "Save 1" "This is a note"
        ```

        Each command has its own argument format.
        Use `help + command` to see detailed usage.

    - **Interactive mode**  
      Enter the command first, then follow the prompts:

        ```bash
        >>>save
        Please enter save name (press Enter to cancel): Save 1
        Please enter save note (press Enter to skip): Save note
        Save successful

        ```

3. Saves locked using the `lock` command **cannot be modified**.
   Use `unlock` first if you want to change them.

4. Use `help + command` to view detailed help for a specific command.

5. The `delete`, `lock`, and `unlock` commands support **batch operations**.
   Use `help delete` to see how they work.

## Save Info File

This program uses `./Saves/info.json` to store save metadata such as names, notes, and timestamps.
If you’re not familiar with JSON files, it’s best **not to edit this file manually**.

```json
{
  "noita_exe_path": "",
  "saves": [
    {
      "m_date": "2025-09-20",
      "m_time": "20:58:40",
      "m_name": "as_zjuu6O",
      "m_note": "",
      "m_islocked": true
    },
    ...
    {
      "m_date": "2025-09-20-",
      "m_time": "20:58:40",
      "m_name": "as_zjuu6O",
      "m_note": "",
      "m_islocked": true
    }
  ]
}
```

## Command List

|  Command  |   Meaning    |      Alias      |                      Description                       |
| :-------: | :----------: | :-------------: | :----------------------------------------------------: |
|   help    |              |        h        |                  Show help and usage                   |
|   clear   |              |       cls       |                      Clear screen                      |
|   quit    |              |    q / exit     |                    Exit the program                    |
| startgame |              |       sg        |                      Start Noita                       |
|  setpath  |              |       sp        |               Set the path of noita.exe                |
|   save    |              |        s        |                          Save                          |
|   qsave   |  quick save  |       qs        |       Quick save (auto-generated name, no note)        |
| overwrite |              | ow / rsave / rs | Overwrite the latest save (name and note won't change) |
|   asave   |  auto save   |       as        |                       Auto save                        |
|   load    |              |        l        |                      Load a save                       |
|   qload   |  quick load  |       ql        |                  Load the latest save                  |
|   list    |              |  ls / log / lg  |                     Show save list                     |
|   slist   |  short list  |    sl / slog    |                   Show recent saves                    |
|  modify   | modify save  |       mo        |                     Edit save info                     |
|  delete   |              |        d        |                         Delete                         |
|  qdelete  | quick delete |       qd        |                 Delete the latest save                 |
|   lock    |              |     lc / f      |                      Lock a save                       |
|  unlock   |              |     ul / uf     |                     Unlock a save                      |
|   usage   |              |       use       |                    Check disk usage                    |
