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
