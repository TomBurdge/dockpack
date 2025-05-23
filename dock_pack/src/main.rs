//! This tool is for unpacking the files from a Docker image into a directory, essentially
//! enabling you to use Docker to distribute files. Because Docker is integrated into
//! every major cloud provider and CI/CD tool, and has caching and version control built-in,
//! you can use Dockpack to package and distribute private code libraries of any language.
//! You can also combine multiple projects and tools in different language into a single Docker
//! image for distribution. I've personally used this tool todistribute private Rust code
//! libraries, trained ML models, and private python packages. I've also used it to bundle
//! multiple tools and scripts together to setup a build package for servers.
//!
//! ## Installation
//! I plan on supporting `brew` and `apt get` in the future but for now you can
//! install the tool with cargo by using the following command:
//! ```bash
//! cargo install dockpack
//! ```
//!
//! ## Unpacking files from a Docker image
//! To unpack the files from a Docker image into a directory, you can the following `pull` command:
//! ```bash
//! dockpack pull -i <image> -d <directory>
//! ```
//! For a toy example, you can unpack the `maxwellflitton/nan-one` image into a directory called `cache`
//! with the command below:
//! ```bash
//! dockpack pull -i maxwellflitton/nan-one -d cache
//! ```
//! This will give you the following file structure:
//! ```plaintext
//! ├── cache
//! │   ├── Cargo.toml
//! │   ├── src
//! │   │   └── lib.rs
//! │   └── tar
//! │       ├── <Various tar files from the Docker image>
//! ```
//!
//! ## Packing files into a Docker image
//! I am working on a `push` command for later versions. However, for now, just use Docker and the `scratch` image.
//! For instance, you can have the following `Dockerfile`:
//! ```Dockerfile
//! FROM scratch
//!
//! COPY ./some_dir .
//! ```
//! Then build the image with the following command:
//! ```bash
//! docker build . \
//! --platform linux/amd64,linux/arm64,linux/arm/v7,linux/arm/v6,linux/s390x,linux/ppc64le \
//! -t <IMAGE_REPO> \
//! --push
//! ```
//! We must add all the platforms to ensure that the image can be run on any architecture as we don't have anything to
//! run in the image, just files to unpack.
//!
//! ## Future features
//! [] Add a `push` command to pack files into a Docker image
//! [] Add a `ls` command to list all the unpacked images
//! [] Add a `rm` command to remove unpacked images
//! [] Add data store for tracking unpacked images and their locations
//! [] Add an undate command to update downloaded images in their existing directories
//! [] Add buckets for bundling multiple images together for distribution
//! [] Dynamic C library so other languages can directly interact with core functionalities to build on top of it
use clap::{Arg, Command};

use core_dockpack::cmd_processes::build::build_dockerfile;
use core_dockpack::cmd_processes::pull::unpack_files;
use core_dockpack::cmd_processes::push::execute_push;

#[tokio::main]
async fn main() {
    // Create the Clap command line app
    let matches = Command::new("Docker Unpacker")
        .version("0.1.1")
        .author("Maxwell Flitton maxwellflitton@gmail.com")
        .about("Unpacks Docker images into a specified directory")
        .arg(
            Arg::new("command")
                .help("The command to execute (e.g., pull, push, ls)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("image")
                .short('i')
                .long("image")
                .value_name("IMAGE")
                .help("The name of the Docker image to unpack"),
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .value_name("DIRECTORY")
                .help("The directory to unpack the Docker image into"),
        )
        .get_matches();

    let command = matches
        .get_one::<String>("command")
        .expect("Command argument is required");

    // Match the command to perform the corresponding action
    match command.as_str() {
        "pull" => {
            let image = match matches.get_one::<String>("image") {
                Some(image) => image,
                None => {
                    eprintln!("Image argument is required for pull");
                    return;
                }
            };
            let directory = match matches.get_one::<String>("directory") {
                Some(directory) => directory,
                None => {
                    eprintln!("Directory argument is required for pull");
                    return;
                }
            };

            match unpack_files::unpack_files_from_image(image, directory).await {
                Ok(path) => println!("Successfully unpacked to: {}", path),
                Err(e) => eprintln!("Error unpacking image: {}", e),
            }
        }
        "build" => {
            let directory = match matches.get_one::<String>("directory") {
                Some(directory) => directory,
                None => {
                    eprintln!("Directory argument is required for pull");
                    return;
                }
            };

            match build_dockerfile::create_dockerfile(directory) {
                Ok(()) => println!("Successfully built to: {}", directory),
                Err(e) => eprintln!("Error unpacking image: {}", e),
            }
        }

        "push" => {
            let image = match matches.get_one::<String>("image") {
                Some(image) => image,
                None => {
                    eprintln!("Image argument is required for push");
                    return;
                }
            };
            let directory = match matches.get_one::<String>("directory") {
                Some(directory) => directory,
                None => {
                    eprintln!("Directory argument is required for push");
                    return;
                }
            };

            match execute_push::execute_docker_build(directory, image).await {
                Ok(()) => println!("Successfully created docker image"),
                Err(e) => eprintln!("Error creating docker image: {}", e),
            }
        }
        "ls" => {
            // Placeholder for the ls command implementation
            println!("listing unpacked images will come when a data store for tracking unpacked images is implemented");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
