[Outdated] See [buaa-cli](https://github.com/fontlos/buaa-cli)

> Make BUAA Great Again

> This is an example for [buaa-api](https://github.com/fontlos/buaa-api)

A cli can help you login to BUAA-WiFi fast

# Usage

> Example for Windows

1. Download the cli or install from source with `cargo install --git https://github.com/fontlos/buaa-wifi-login-cli`
2. Connect to `BUAA-WiFi` and enable auto-connect without clicking login
3. Put the cli in the right place, and
   ```sh
    buaa-wifi-login-cli -u <username> -p <password>
   ```
4. Create a shortcut for the cli and put it into the boot auto-start folder

In this way, your computer will automatically login to `BUAA-WiFi` after it is automatically connected to `BUAA-WiFi` after booting up, and there will be no browser login window forced to pop up
