```markdown
# DeepCool AK620 Digital Temperature Display for Ubuntu (Rust Implementation)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This project provides a Rust-based solution to display your CPU temperature on the DeepCool AK620 Digital Air Cooler when running Ubuntu.

## Features

- **Accurate Temperature Monitoring:** Reads your CPU temperature directly from system sensors.
- **Customizable Display:** Displays the temperature directly on the AK620's screen using Celsius and a visual temperature bar graph for easy monitoring.
- **Systemd Integration:** Runs as a background service for continuous monitoring.
- **Rust Power:** Leverages Rust's safety and performance for a reliable solution.

## Inspiration and Alternatives

This project was heavily inspired by the Python-based solution found at [raghulkrishna/deepcool-ak620-digital-linux](https://github.com/raghulkrishna/deepcool-ak620-digital-linux). While that solution is excellent, it did not work for me, hence this Rust implementation offers a slightly different approach for users who prefer Rust or seek its specific benefits.

## Prerequisites

- **Ubuntu:** This project is designed specifically for Ubuntu Linux.
- **DeepCool AK620:** Make sure you have the DeepCool AK620 Digital Air Cooler installed.
- **Rust:** Install the Rust toolchain using `rustup` ([rust-lang.org](https://www.rust-lang.org/tools/install)).

## Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/hannody/deepcool_ak620_digital_linux_driver.git
   ```

2. **Build the Project:**

   ```bash
   cd deepcool_ak620_digital_linux_driver
   cargo build --release
   ```

3. **Configure the Service:**

   - Update the `.service` file based on your system info/config.

   - Copy the provided `deepcool-ak620.service` file to `/etc/systemd/system/`:

     ```bash
     sudo cp deepcool-ak620.service /etc/systemd/system/
     ```

   - Edit the file:

     Open the file with a text editor (e.g., `sudo nano /etc/systemd/system/deepcool-ak620.service`) and make the following changes:
     - Replace `<your_username>` with your actual username.
     - Adjust the `WorkingDirectory` and `ExecStart` paths if you cloned the repository to a different location.
     - (Optional) Modify the Environment variable if you are using a different Rust toolchain.

4. **Enable and Start the Service:**

   ```bash
   sudo systemctl enable deepcool-ak620.service
   sudo systemctl start deepcool-ak620.service
   ```

## Tested Environments

This project has been successfully tested on the following system:

- CPU: Ryzen 9 7800X3D (AM5)
- Operating System: Ubuntu 24.04 LTS
```