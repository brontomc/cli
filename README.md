# Amethyst CLI
This Command Line Interface (CLI) is designed to provide easy management of Amethyst Core. With this CLI, you can install, update, uninstall, change configuration of Amethyst effortlessly.

## System Requirements
The Amethyst CLI supports Windows and Linux operating systems.

## Installation
### Windows:
Download the latest executable from the [Releases](https://github.com/amethyst-core/cli/releases) page. \
Once downloaded, double-click the executable file to install the Amethyst CLI.

### Linux:
Download the latest executable from the [Releases](https://github.com/amethyst-core/cli/releases) page. \
Open a terminal and navigate to the directory containing the downloaded executable.
Make the file executable with the following command:

```bash
chmod +x amethyst-cli
```

Move the executable to a directory in your system's PATH to make it accessible globally. 

```bash
sudo mv amethyst-cli /usr/local/bin/amethyst
```

## Usage
Once installed, you can start using the Amethyst CLI by running the `amethyst` command followed by the desired action. Here are some examples:

### Install Amethyst:
```bash
amethyst install
```

**Additional Flags:**
- Specify Installation Path: Use the `-p` or `--path` flag followed by the desired directory path to specify the installation directory.
- Specify Version: Use the `-v` or `--version` flag followed by the desired version to specify the version to be installed.

### Update Amethyst:
```bash
amethyst update
```

### Config Amethyst:
```bash
amethyst update
```

**Atleast One Flag Required:**
- Change Core Port: Use the `-p` or `--port` flag followed by the desired port to run the core at. You will need to restart the core after changing the port.

### Start Amethyst Core:
```bash
amethyst start
```

### Stop Amethyst Core:
```bash
amethyst stop
```

### Restart Amethyst Core:
```bash
amethyst restart
```

### Amethyst Core Status:
```bash
amethyst status
```

For more information on each command's available flags and options, refer to help documentation using the --help flag.

## Support and Contributions

For any issues, bugs, or feature requests, please visit the [issues](https://github.com/amethyst-core/cli/issues) tab repository and open an issue.

Contributions are welcomed via pull requests.

## License
The Amethyst CLI is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Credits
The Amethyst project is maintained by a community of developers. See the [CONTRIBUTORS](CONTRIBUTORS) file for a list of contributors.

Thank you for choosing Amethyst! Happy Minecrafting! ✨
