# eelu-login

eelu-login is a command-line tool that allows users to log in to the EELU Moodle platform quickly and easily through the command-line interface.

### Installation For Users
You can install the latest stable version of eelu-login via Cargo:
`cargo install eelu-login`

Or you can get the latest git version from the repository:
`cargo install --git https://github.com/anaselgarhy/eelu-login.git`

Or check [Releases page](https://github.com/Crypt00o/eelu-login/releases) for pre-built binaries (not updated frequently).

### Installation and Building For Developers
To install eelu-login, you'll need to have Rust and Cargo installed on your machine. Once you have those installed, you can run the following command:

build:
```sh
cargo build --release
```

run :
```sh
cargo run
```

### Usage
`eelu-login --help` will show you the usage of the tool:
```
[+] Usage : eelu-login [--user <username>] [--pass <password>] [--type <staff| sys-user | student>]
Args:
[-user | --user | --username | -username |  -u]   <username>  :  username to login with
[-pass | --pass | --password | -p]   <password>  :  password to login with
[-type | --type | --usertype | -usertype | -t]  : <usertype>

Flags:
[-o | --open | -open] : open browser after login
[-v | --verbose | -verbose] : verbose mode
[-V | --version | -version] : print version
[-h | --help | -help] : print this help message

usertype can be :
    [ staff | 3 ] for staff privilege
    [ sys-user | 1] for system user privilege
    [ student | 2] for student privilege"#
```

Replace `<username>` and `<password>` with your EELU Moodle login credentials, and `< staff | sys-user | student>` with your user type.

If you don't want to enter your credentials every time you run the tool, you can set the `SIS_EELU_USERNAME` and `SIS_EELU_PASSWORD` environment variables to your username and password respectively.

You can don't need to specify the user, and the tool will be try to login as a student and if it fails it will try to login as a staff and if it fails it will try to login as a system user.

### Support
If you have any questions or need help using eelu-login, you can contact the creator, Eslam Muhammad [0xCrypt00o], at 0xCrypt00o@protonmail.com. You can also support the creator by sending Bitcoin to the following address: bc1qdp3f6u3puwkyu6ztxz7hzcurd7jnjyw6hzjcas.


### Contributors 🦀❤
- [Anas Elgarhy](https://github.com/anaselgarhy)

### Contributing
If you want to contribute to this project, please read the [contributing guidelines](./CONTRIBUTING.md) first.

### Useful Links
- [sis-login crate](https://crates.io/crates/sis-login)
- [EELU SIS](https://sis.eelu.edu.eg/)
- [EELU Moodle](https://moodle.eelu.edu.eg/)

License: MIT OR Apache-2.0
