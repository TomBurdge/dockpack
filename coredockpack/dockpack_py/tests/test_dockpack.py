#!/usr/bin/env python3

import dockpack_py
import os
import shutil


def test_execute_push():
    image_name = "maxwellflitton/nan-one".encode("utf-8")
    directory = "./cache/two"
    directory_arg = directory.encode("utf-8")
    if os.path.exists(directory):
        shutil.rmtree(directory)

    result = dockpack_py.lib.unpack_files_from_image_c(image_name, directory_arg)
    assert os.path.exists(directory)
    shutil.rmtree(directory)


#     #[tokio::test]
#     async fn test_unpack_files_from_image() {
#         let image = "maxwellflitton/nan-one";
#         let directory = "./cache/two";
#         let result = unpack_files_from_image(image, directory).await;
#
#         match result {
#             Ok(ref ok) => println!("Ok: {}", ok),
#             Err(ref err) => println!("Err: {}", err),
#         }
#
#         assert!(result.is_ok());
#         let path = result.unwrap();
#         assert!(Path::new(&path).exists());
#         fs::remove_dir_all(directory).unwrap();
#     }
# }
