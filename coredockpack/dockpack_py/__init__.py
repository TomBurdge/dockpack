from .dockpack_py import ffi, lib

__all__ = ["ffi", "lib", "unpack_files_from_image"]


def unpack_files_from_image(image_name: str, directory: str) -> str:
    image_name: bytes = image_name.encode("utf-8")
    directory: bytes = directory.encode("utf-8")
    return unpack_files_from_image_c(image_name=image_name, directory=directory)
