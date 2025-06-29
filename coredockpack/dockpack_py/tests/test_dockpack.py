from dockpack_py import unpack_files_from_image
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
