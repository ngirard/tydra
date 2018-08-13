# tydra

> Terminal Hydra

Customizable shortcut program for your terminal.

**Note:** This readme is currently a specification of the intended use of the
program. It has not yet been implemented!

## Basic idea

The idea is that when running this program you get presented with a menu of
shortcuts. A shortcut can lead to another page of shortcuts and/or run a
command.

The menu will be hidden as long as the command is running, and if the command
is not supposed to return to a page then tydra will quit.

## Use

Define your action file and run tydra with it as your first argument.

```yaml
# Global settings are applied to each page. Each page can override them
# individually if they so wish.
global:
  layout: columns # Render entries in columns
  shortcut_color: red # Show shortcut letter in red
pages:
  # tydra always start on the "root" page by default:
  root:
    title: Welcome
    header: This is the default page.
    footer: "You can always quit using {fg=blue Esc}."
    groups:
      - title: Desktop
        entries:
          - shortcut: h
            title: Home
            command: "xdg-open ~"
            mode: background # Run command in background; ignore output and
                             # return immediately after starting it.
          - shortcut: d
            title: Downloads
            command: "xdg-open ~/Downloads"
            mode: background
          - shortcut: D
            title: Desktop
            command: "xdg-open ~/Desktop"
            mode: background

      - title: Web
        entries:
          - shortcut: g
            title: Google
            # Commands can also be given in a structured form instead of as a
            # shell script. No shell-features (like $ENV subsititution,
            # redirects, pipes, globbing, ~ expansion, etc.) work in here, but
            # that also means that there is no need to escape arguments.
            # It also means that when you need none of these shell features,
            # then the command should have a faster startup.
            command:
              name: xdg-open
              args:
                - https://www.google.com
            mode: background
          - shortcut: G
            title: Github
            command:
              name: xdg-open
              args:
                - https://www.github.com
            mode: background
          - shortcut: l
            title: Gitlab
            command:
              name: xdg-open
              args:
                - https://www.gitlab.com
            mode: background

      - title: Misc
        entries:
          - shortcut: "?"
            title: Show tydra help
            command: "tydra --help | less"
            return: true # Return to the same page after the command has finished.
          - shortcut: p
            shortcut_color: blue
            title: Packages
            # command: /bin/true # Default when no command is given.
            return: packages # Go to the packages page
          - shortcut: q
            title: Quit
            return: false # This is default when not specified
  packages:
    title: Packages
    header: "Perform package operations."
    settings:
      layout: list
    groups:
      - entries:
        - shortcut: r
          title: Refresh package repos
          command: "clear; sudo pacman -Sy"
          return: true
        - shortcut: u
          title: Show packages that can be upgraded
          command: "clear; pacman -Qu | less -+F"
          return: true
        - shortcut: U
          title: Install upgrades
          command: sudo pacman -Su
          mode: wait # Wait for user to press enter before returning to menu
          return: true
      - settings: # Individual groups can also have other default settings
          shortcut_color: blue
        entries:
        - shortcut: q
          title: Go back
          return: root
```

```
tydra /path/to/actions.yml
```

## Still TODO

- A way to dynamically generate pages.
  - Run a command that generates JSON or YAML?

- Configure keybindings.
