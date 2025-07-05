from dockpack_py import (
    unpack_files_from_image,
    build_files_from_image,
    create_dockerfile,
)
import os
import shutil


def test_execute_push():
    image_name = "maxwellflitton/nan-one"
    directory = "./cache/two"
    if os.path.exists(directory):
        shutil.rmtree(directory)

    result = unpack_files_from_image(image_name, directory)
    assert os.path.exists(directory)
    assert result == directory
    shutil.rmtree(directory)


def test_push(test_image_dir, dockerfile_path):
    image_name = "test_image:latest"
    with open(dockerfile_path, "w") as f:
        f.write("FROM scratch\nCOPY . .\n")
    res = build_files_from_image(image_name, "./cache")
    assert res == dir


# #[tokio::test]
# async fn test_execute_push() {
#     let image_name = "test_image:latest";
#     let directory = "./test_image";
#
#     fs::create_dir_all(directory).expect("Failed to create test directory");
#     let dockerfile_path = format!("{}/Dockerfile", directory);
#     let dockerfile_content = "FROM scratch\nCOPY . .\n";
#     fs::write(&dockerfile_path, dockerfile_content).expect("Failed to write Dockerfile");
#
#     let result = execute_docker_build(directory, image_name).await;
#     assert!(result.is_ok());
#
#     // fs::remove_dir_all(directory).expect("Failed to remove test directory");
#


def test_build(test_image_dir):
    create_dockerfile(str(test_image_dir))
    dockerfile_path = os.path.join(test_image_dir, "Dockerfile")
    with open(dockerfile_path, "r") as f:
        file_content = f.read()
    exp = "FROM scratch\nCOPY . .\n"
    assert file_content == exp
    print(file_content)
