# vhdrs

A lightweight library that provides an ergonomic interface for managing Virtual Hard Disks (VHD/VHDX) on Windows systems. It leverages the Windows API to facilitate operations such as opening, attaching, detaching, and retrieving information from VHD files.

## Features

- Open VHD/VHDX Files: Supports opening VHD/VHDX files in both ReadOnly and ReadWrite modes.
- Mounting and Unmounting: Attach and detach virtual disks to and from the system with options for persistent and temporary mounts.
- Disk Information Retrieval: Obtain detailed information about the virtual disk, including its size and unique identifier.
- Automatic Resource Management: Handles cleanup operations, ensuring that resources like file handles are correctly released.

## Usage

### Opening a VHD/VHDX File

You can open a VHD/VHDX file by specifying the file path and the desired access mode. The file type is inferred from the extension unless explicitly specified.

```rust
let vhd = vhdrs::Vhd::new("file.vhd", vhdrs::OpenMode::ReadOnly, None).unwrap();
```

### Attaching a VHD

To mount a VHD to a system drive, use the attach method. You can choose to make the mount persistent across system reboots.

```rust
let mut vhd = vhdrs::Vhd::new("file.vhd", vhdrs::OpenMode::ReadOnly, None).unwrap();
let drive_letter = vhd.attach(false).unwrap();
println!("VHD mounted at drive: {}", drive_letter);
```

### Detaching a VHD

To manually unmount a VHD, use the detach method. Manual detachment is only necessary for persistent mounts; temporary mounts are automatically detached when the VHD instance is dropped.

```rust
vhdrs::Vhd::detach("file.vhd").unwrap();
```

### Retrieving Disk Information

You can retrieve detailed information about the VHD, including its virtual size, physical size, block size, and sector size.

```rust
let mut vhd = vhdrs::Vhd::new("file.vhd", vhdrs::OpenMode::ReadOnly, None).unwrap();
let disk_info = vhd.get_size().unwrap();
println!("Disk Info: {:?}", disk_info);
```

### Getting the VHD Identifier

This function retrieves a unique identifier for the attached virtual disk, useful for tracking and managing multiple VHDs.

```rust
let mut vhd = vhdrs::Vhd::new("file.vhd", vhdrs::OpenMode::ReadOnly, None).unwrap();
let identifier = vhd.get_identifier().unwrap();
println!("VHD Identifier: {}", identifier);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for more details.

## Contributing

Contributions are welcome! If you have suggestions for improvements or want to report issues, feel free to open an issue or a pull request on the project's [GitHub repository](https://github.com/samvdst/vhdrs).
